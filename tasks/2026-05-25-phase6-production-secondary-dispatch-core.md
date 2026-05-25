# Phase 6 Production Secondary Dispatch Core

## Task

Implement the first production secondary dispatch slice for CPU-local
diagnostic kernel threads without shared run queues or task migration.

## Scope

- Added the scheduler-core representation for a production-enabled secondary
  diagnostic role.
- Added CPU-local production dispatch for already seeded diagnostic kernel
  threads.
- Preserved target-owned local runnable mutations and kept remote wake
  publication separate from local queue mutation.
- Kept the implementation at the scheduler data-structure boundary; no QEMU
  proof, Pi 5 hardware run, shared queue, migration, or broad scheduler work
  was added.

## Non-Goals

No shared run queue, task migration, load balancing, cross-core stealing,
multi-core preemption, userspace, descriptors, filesystem, networking, SSH,
shell behavior, RP1/PCIe, UART interrupt ownership, DMA/cache driver policy,
or hardware run.

## Implementation Summary

- `SchedulerCoreRole::SecondaryProductionDiagnostic` is the only secondary
  role that enables production dispatch.
- `PerCoreScheduler::dispatch_cpu_local_diagnostic_task()` requires local
  ownership, rejects deferred secondary schedulers, checks that the selected
  task is the front local runnable task, checks that the task is still
  `Runnable`, records the per-core current task, and increments local dispatch
  counters.
- `ProductionDispatchError` names wrong-owner, deferred-role, empty-queue,
  mismatched-task, and non-runnable-task failures without mutating local
  scheduler state.
- Remote wake behavior remains unchanged: remote CPUs may publish bounded wake
  requests, but only the target CPU may consume them and mutate its local queue.

## Changed Files

- `src/scheduler.rs`
- `src/target/qemu_virt.rs`
- `docs/src/architecture/scheduler.md`
- `tasks/2026-05-25-phase6-production-secondary-dispatch-core.md`

## Validation

- Static inspection: `git status --short` before edits showed a clean Talos
  worktree.
- Unit tests: `cargo -Zjson-target-spec test` passed after adding the local
  QEMU 9.2.0 tool path; 119 no_std tests passed.
- Initial unit-test environment check: the same command failed with exit 127
  before the documented QEMU path was exported because `qemu-system-aarch64`
  was not on `PATH`.
- Formatting: `cargo fmt --all -- --check` passed.
- QEMU/substitute baseline: `scripts/qemu-smoke.sh` passed.
- QEMU/substitute retained gate: `scripts/qemu-per-core-scheduler-ownership-smoke.sh`
  produced `qemu-per-core-scheduler-ownership: PASS` and
  `classification=qemu-per-core-scheduler-ownership-complete`.
- QEMU/substitute retained gate:
  `scripts/qemu-remote-wake-to-local-runnable-smoke.sh` produced
  `qemu-remote-wakeup-request: PASS` and
  `classification=qemu-remote-wake-to-local-runnable-complete`.
- Image/archive inspection substitute: `scripts/rpi5-image.sh` built
  `target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img`.
- Documentation: `mdbook build` passed.
- Whitespace inspection: `git diff --check` passed.

## Acceptance

Accepted as the bounded scheduler-core implementation task. Talos is ready for
the queued focused QEMU production secondary dispatch smoke. The core is not
accepted as QEMU or Pi 5 behavior until those proof tasks pass.
