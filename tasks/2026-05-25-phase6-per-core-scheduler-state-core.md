# Phase 6 Per-Core Scheduler State Core

Task: phase6-per-core-scheduler-state-core-20260525

Status: accepted.

## Scope

Implement the first CPU-local scheduler ownership data boundary after the
accepted Phase 6.3 readiness inventory. This task is limited to scheduler data
structures, focused no_std tests, scheduler architecture documentation, and
retained QEMU/image validation gates.

## Output

- Added `LogicalCpuId`, `SchedulerCoreRole`,
  `PerCoreSchedulerAccessError`, and `PerCoreScheduler` to
  `src/scheduler.rs`.
- Kept `SingleCoreScheduler` behavior unchanged for the existing runnable
  queue, voluntary-yield, and timer-preemption paths.
- Updated `docs/src/architecture/scheduler.md` with the implemented
  CPU-local state boundary.

## Evidence

- Changed scheduler/SMP files: `src/scheduler.rs` only.
- Invariant tests added for logical CPU identity, boot-CPU current-task
  ownership, rejected cross-owner queue mutation, deferred secondary production
  dispatch, and retained local queue state/counter behavior.
- Explicit deferrals retained: task migration, shared run queues, load
  balancing, global task lookup, cross-core wakeups, IPIs, secondary-core
  production scheduling, userspace, descriptors, filesystem, networking, SSH,
  shell, RP1/PCIe, and DMA policy.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed with 108 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-context-switch-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-timer-preemption-smoke.sh` passed.
- image/archive inspection: `scripts/rpi5-image.sh` built
  `target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img` and passed the
  arm64 Image header size check.
- whitespace inspection: `git diff --check` passed.
- static inspection: `mdbook` is unavailable in the container.

## Acceptance

Accepted as the first Milestone 6.3 per-core scheduler ownership data
boundary. The next bounded task is
`phase6-qemu-per-core-scheduler-ownership-smoke-20260525`, which may add
QEMU substitute evidence without enabling shared run queues, migration, IPIs,
cross-core wakeups, or secondary-core production scheduling.
