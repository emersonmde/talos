# Phase 6 CPU-Local Scheduler Service Core

Status: accepted.

Task id: phase6-cpu-local-scheduler-service-core-20260526

## Goal

Implement the target-independent CPU-local scheduler service core that
sequences accepted remote wake, local runnable, timer-preemption, CPU-local
dispatch, and owner metadata refresh behavior for one owning logical CPU.

## Scope

- Added `CpuLocalSchedulerService` as a target-independent adapter around
  `PerCoreScheduler`, `RemoteWakeQueue`, `SingleCoreScheduler`, and
  `SharedSchedulerMetadata`.
- Sequenced target-owned remote wake drains, local blocked-to-runnable
  transitions, pending timer-preemption handling, owner dispatch, and metadata
  refresh in normal control flow.
- Added focused unit/QEMU tests for service ordering and explicit error
  boundaries.
- Kept QEMU evidence to existing no_std unit tests and `scripts/qemu-smoke.sh`;
  no new proof script was needed for this target-independent core.

## Non-Goals

- No shared run queues, remote enqueue queues, task migration, load balancing,
  work stealing, or multi-core preemption.
- No Phase 7, userspace, filesystem, networking, SSH, shell, RP1/PCIe, UART
  interrupt ownership, or DMA/cache-driver policy.
- No Pi 5 hardware claim.

## Evidence

- Implementation: `src/scheduler.rs`.
- Unit/QEMU evidence: `cargo -Zjson-target-spec test` with the QEMU 9.2.0 path
  configured passed 128 no_std tests, including:
  - `cpu_local_scheduler_service_drains_wakes_dispatches_and_refreshes_metadata`
  - `cpu_local_scheduler_service_handles_timer_preemption_before_metadata_refresh`
  - `cpu_local_scheduler_service_preserves_explicit_error_boundaries`
- QEMU/substitute smoke: `scripts/qemu-smoke.sh` passed and printed
  `talos: qemu smoke PASS`.

## Service-Core Result

`CpuLocalSchedulerService::run_cycle` executes the accepted CPU-local order:

1. consume one target-owned remote wake request outside IPI context;
2. convert the matching local blocked task to runnable under local ownership;
3. handle a pending local timer-preemption request, letting a just-woken task
   participate in the dispatch decision;
4. dispatch through the owning `PerCoreScheduler` when no timer-preemption
   dispatch already selected the next task;
5. refresh owner-published scheduler metadata after local mutations.

The service returns explicit errors for remote wake queue ownership, wake
consumption, missing timer-current task, timer preemption, production dispatch,
and metadata refresh failures. Existing wrong-owner, wrong-target,
duplicate-runnable, non-blocked task, no-runnable, deferred secondary-role,
unknown metadata, and stale metadata outcomes remain named boundaries.

## Validation

- static inspection: `git status --short` was clean before edits.
- formatting: `cargo fmt --all -- --check` initially identified formatting
  deltas after implementation; `cargo fmt --all` was applied.
- unit/QEMU tests: `cargo -Zjson-target-spec test` initially failed because
  `qemu-system-aarch64` was not on `PATH`; rerun with the documented QEMU 9.2.0
  path passed 128 no_std tests.
- QEMU/substitute smoke: `scripts/qemu-smoke.sh` passed.
- whitespace inspection: `git diff --check` and `git diff --cached --check`
  passed.
- documentation: `mdbook build` passed.
