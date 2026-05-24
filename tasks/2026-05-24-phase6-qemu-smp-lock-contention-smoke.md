# Phase 6 QEMU SMP Lock Contention Smoke

Status: accepted.

Task: `phase6-qemu-smp-lock-contention-smoke-20260524`

## Scope

This task proves the accepted Phase 6.2 `SpinLock<T>` primitive under bounded
QEMU multi-core contention. It adds:

- `TALOS_QEMU_SMP_LOCK_CONTENTION_SMOKE`;
- `scripts/qemu-smp-lock-contention-smoke.sh`;
- a QEMU virt diagnostic path that starts secondary cores through the accepted
  PSCI/trampoline path and has cores 1, 2, and 3 contend on the shared
  `SpinLock<T>`;
- a deterministic shared counter invariant outside the production scheduler.

The diagnostic remains a validation surface. It does not add Pi 5 hardware
claims, scheduler migration, shared run queues, cross-core wakeups, IPIs,
userspace, descriptors, filesystem, networking, SSH, shell behavior,
RP1/PCIe, UART interrupts, or DMA ownership.

## QEMU Evidence

`scripts/qemu-smp-lock-contention-smoke.sh` passed and captured the transcript
at `target/qemu-smp-lock-contention-smoke.log`.

The transcript showed:

- QEMU virt with EL2 virtualization, GICv2, Cortex-A76, and `-smp 4`;
- PSCI `CPU_ON` through SMC returned `0` for logical CPUs 1, 2, and 3;
- each secondary core reached `workload-complete` with its expected MPIDR
  affinity, owned stack slot, `lock-count=64`, `progress=64`, and `ok=true`;
- the final shared invariant reported `counter=192 expected=192
  participants=3 errors=0 lock-available=true`;
- classification `qemu-smp-lock-contention-complete`;
- `qemu-smp-lock-contention: PASS`.

## Validation

- static inspection: `git status --short` was clean before implementation
  edits.
- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed with 102 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-smp-lock-contention-smoke.sh` passed.
- static inspection: `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container, so mdBook
  build was not run.

## Result

Accepted as the QEMU contention proof for the first SMP-safe primitive. The
next queued task is
`phase6-pi5-smp-lock-cache-coherence-proof-20260524`, which must not shrink to
QEMU-only evidence if physical Pi 5 behavior is inconclusive.
