# Phase 6 Secondary-Core Bring-Up Closeout Checkpoint

Status: accepted as the Milestone 6.1 secondary-core bring-up closeout.

This checkpoint reconciles the accepted Phase 6.1 evidence before Talos starts
SMP-safe primitives, shared scheduler data structures, multi-core scheduling,
EL0, syscalls, descriptors, filesystem, networking, SSH, shell behavior, UART
interrupts, RP1/PCIe/DMA, or scheduler blocking I/O.

## Accepted Work

- Source inventory and contract: commit `50e2bbf`; checkpoint
  `docs/src/project/phase6-secondary-core-bringup-source-inventory.md`; task
  record `tasks/2026-05-24-phase6-secondary-core-bringup-source-inventory.md`.
- QEMU secondary-core discriminator: commit `80ffca0`; task record
  `tasks/2026-05-24-phase6-qemu-secondary-core-bringup-discriminator.md`.
- Per-core state and stacks: commit `78db923`; task record
  `tasks/2026-05-24-phase6-per-core-state-and-stacks.md`.
- Pi 5 PSCI secondary-core alive proof: commit `4f5f1a9`; task record
  `tasks/2026-05-24-phase6-pi5-psci-secondary-core-alive-proof.md`;
  evidence directory
  `tasks/evidence/2026-05-24-pi5-psci-secondary-core-alive-proof/`.
- Controlled secondary-core workload proof: commit `19cd241`; task record
  `tasks/2026-05-24-phase6-secondary-core-controlled-kthread-workload.md`;
  evidence directory
  `tasks/evidence/2026-05-24-pi5-secondary-core-workload-proof/`.

## Accepted Boundary

Milestone 6.1 accepts that Talos can bring up secondary cores through the
PSCI SMC path, identify them, assign reserved secondary stacks, publish
per-core state, park them in a controlled handoff state, and run a bounded
diagnostic-only workload on secondary cores.

The production scheduler remains single-core. Secondary cores do not enter the
production run queue, migrate tasks, share runnable state, balance load, handle
cross-core preemption, block on I/O, or participate in userspace execution.

The accepted Pi 5 MPIDR affinities for secondary cores are `0x100`, `0x200`,
and `0x300`. The accepted QEMU substitute affinities remain `0x1`, `0x2`,
and `0x3`; QEMU evidence is useful for target-independent regressions but is
not Pi 5 hardware proof.

## Evidence Reconciliation

The QEMU discriminator proves the shared trampoline, PSCI SMC call shape,
logical CPU mapping, stack-slot ownership, and `handoff-ready` lifecycle under
QEMU virt with EL2 virtualization, GICv2, Cortex-A76, and four CPUs.

The Pi 5 alive proof proves the hardware path after earlier rejected
classifications. The accepted run served the 90,784-byte candidate image, then
serial evidence showed cores 1, 2, and 3 reaching Rust, publishing state,
reporting MPIDR affinities `0x100`, `0x200`, and `0x300`, owning distinct
stack slots, and reaching `handoff-ready` with classification
`pi5-psci-smc-secondary-cores-alive`. The hardware test lock was held for
publish, TFTP inspection, power cycle, serial capture, and restore.

The controlled workload proof then extended the same boundary with
`workload-running`, `workload-complete`, and `workload_progress`. QEMU and
serialized Pi 5 hardware evidence both show cores 1, 2, and 3 reaching
`workload-complete` with `progress=64 target=64 ok=true`. The accepted Pi 5
archive was `target/talos-rpi5-secondary-core-workload-boot.tar.gz` with
SHA256
`73e7419eef2ddc0e5ba6a4ac3756d5c0b1d0c2f5b6888b7759b9b921f6621fa7`; the
kernel SHA256 was
`a0ecfe8fef7ad4d144ed68ceefeadf325c4a5fa3ca9cb7b703f7c6e6927d8092`, size
91,288 bytes. TFTP served `da591740/kernel_2712.img` twice before the pre-run
snapshot was restored.

## Retained and Retired Surfaces

Retained as regression gates:

- `scripts/qemu-secondary-core-discriminator.sh` for QEMU/substitute
  secondary-core bring-up behavior.
- `scripts/qemu-secondary-core-workload-smoke.sh` for the diagnostic
  workload path under QEMU.
- `scripts/rpi5-psci-secondary-core-alive-image.sh` and the associated
  serialized Pi 5 proof path when a future task explicitly needs hardware
  alive/park regression evidence.
- `scripts/rpi5-secondary-core-workload-image.sh`,
  `scripts/rpi5-secondary-core-workload-boot-tree.sh`, and the associated
  serialized Pi 5 proof path when a future task explicitly needs workload
  regression evidence.
- Focused no_std tests in `src/smp.rs` for MPIDR mapping, stack-slot
  ownership, lifecycle transitions, and controlled workload progress.

Retained as kernel bring-up surfaces:

- the AArch64 secondary entry trampoline;
- `src/smp.rs` per-core identity, stack, state, and progress records;
- target-specific QEMU and Pi 5 PSCI bring-up diagnostic entry points.

Retired or kept historical only:

- stale Pi 5 probe/proof surfaces removed during the maintainability
  remediation sequence;
- rejected PSCI discriminator classifications unless needed as historical
  evidence for future debugging.

Deferred:

- SMP-safe spin locks, interrupt-safe locks, and per-core critical-section
  policy;
- scheduler migration, shared run queues, per-core run queues, load balancing,
  cross-core wakeups, IPIs, and cross-core preemption;
- concurrent runtime-console ownership, UART interrupts, descriptor tables,
  syscall ABI, user/kernel copy, EL0, filesystem behavior, networking, SSH,
  and shell behavior;
- RP1/PCIe/DMA ownership and cache-coherent DMA policy.

## Remaining Risks

Secondary-core state publication required explicit cache maintenance for the
accepted Pi 5 proof. Milestone 6.2 must define the synchronization and barrier
policy before any shared scheduler or kernel data structure is touched from
multiple cores.

The accepted workload is bounded and boot-time diagnostic-only. It does not
prove long-running multi-core scheduling, fairness, preemption across cores,
interrupt routing to secondary CPUs, IPI delivery, or safe concurrent console
output.

## Next Recommendation

Milestone 6.1 is closed for the current secondary-core bring-up boundary. The
next supervisor-planned slice should be a documentation/source-inventory task
for Milestone 6.2 SMP-safe primitives, tentatively
`phase6-smp-safe-primitives-source-inventory-and-contract-20260524`. That
task should reconcile the accepted cache-maintenance lesson before implementing
locks, shared queues, or multi-core scheduler behavior.

The worker must not start that slice until the supervisor creates or promotes
an explicit durable task.

## Validation

- static inspection: `git status --short` was clean before checkpoint edits.
- fmt/lint/typecheck: `git diff --check` passed after checkpoint edits.
- static inspection: `mdbook` was unavailable in the container, so mdBook
  build was not run.
- Rust fmt/tests were not required because this checkpoint changes only
  Markdown documentation and durable task state.
