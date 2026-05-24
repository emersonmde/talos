# Phase 6 Per-Core State and Stacks

Task: `phase6-per-core-state-and-stacks-20260524`

Status: accepted.

This task adds the minimal per-core state and stack ownership boundary needed
before secondary cores run Talos code. It does not start Pi 5 hardware proof,
add SMP-safe locks, migrate scheduler state across cores, add task migration,
load balancing, cross-core preemption, blocking I/O, EL0, syscalls,
descriptors, filesystem, networking, SSH, or shell behavior.

## Implementation

- Added `src/smp.rs` with `MAX_CORES = 4`, 4 KiB secondary kernel stack slot
  sizing, a bounded `CoreLifecycle` state machine, per-core atomic identity and
  lifecycle records, stack layout validation, and the Pi 5 MPIDR affinity map
  `0x000`, `0x100`, `0x200`, `0x300`.
- Renamed the QEMU discriminator linked stack symbols to the target-neutral
  `talos_secondary_core_stacks` and `talos_secondary_core_stacks_end` while
  preserving the focused QEMU diagnostic gate.
- Moved the QEMU secondary-core alive report to the shared per-core state API:
  secondary CPUs publish MPIDR, affinity, logical context, stack pointer,
  `entered -> stack-ready -> registered -> handoff-ready`, and then park.
- Added no_std tests for lifecycle naming/order, per-core registration,
  distinct stack slot ownership for all four possible cores, and the Pi 5
  Cortex-A76 MPIDR affinity map.

The linked stack reservation remains used by the focused QEMU discriminator in
this slice. Pi 5 hardware proof is still a separate queued task and must capture
its own archive, serial, TFTP, and post-run evidence before making hardware
claims.

## Evidence

- static code inspection: `src/smp.rs`, `src/target/qemu_virt.rs`,
  `src/arch/aarch64/boot.S`, and `src/main.rs` now express per-core
  state/stack ownership without scheduler migration or SMP locking.
- unit tests: `cargo -Zjson-target-spec test` passed with 96 no_std tests.
- QEMU/substitute: `scripts/qemu-secondary-core-discriminator.sh` passed. QEMU
  virt `-smp 4` reported PSCI SMC `CPU_ON` success for logical CPUs 1, 2, and
  3; each secondary reached `handoff-ready`, reported MPIDR affinities `0x1`,
  `0x2`, and `0x3`, used distinct stack slots under
  `0x40220000..0x40224000`, and classified
  `qemu-psci-smc-secondary-cores-alive`.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed the normal boot smoke.
- image/archive inspection: `scripts/rpi5-image.sh` produced
  `target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img`.
- image/archive inspection: `scripts/rpi5-format-guard-check.sh` passed.
- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- fmt/lint/typecheck: `git diff --check` passed.
- static inspection: `mdbook` is unavailable in this container, so mdBook build
  was not run.

## Deferred

Scheduler migration, SMP-safe primitives, cross-core wakeups, IPIs, multi-core
preemption, load balancing, concurrent console output policy, Pi 5 hardware
proof, EL0/syscalls/descriptors, filesystem, networking, SSH, and shell work
remain deferred to explicitly queued supervisor tasks.
