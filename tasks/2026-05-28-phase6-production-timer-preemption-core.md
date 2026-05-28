# Phase 6 Production Timer/Preemption Core

## Task

- Title: Phase 6 production timer/preemption core
- Owner: worker
- Date: 2026-05-28
- Milestone: Phase 6.3, Production Scheduler Runtime Integration
- Scope: production timer IRQ recording, durable per-CPU runtime boundary, and
  owner-local service adapter

## Goal

Implement the first bounded production timer/preemption runtime integration
according to the accepted contract without making a new QEMU production-proof
or Pi 5 hardware claim.

## Acceptance Criteria

- Implementation changes stay limited to the accepted production
  timer/preemption contract surface.
- IRQ hot paths record bounded local state only and do not mutate scheduler
  queues or current tasks.
- Owner-local service order remains remote wake consumption, local timer
  preemption, optional dispatch, then metadata refresh.
- Existing retained Phase 4/6 scheduler QEMU gates and unit tests remain
  passing.
- Accepted work is committed before any QEMU or Pi 5 proof task claims the
  production integration.

## Work Performed

- Added `ProductionSchedulerRuntime` and
  `ProductionTimerPreemptionRecordError` in `src/scheduler.rs`.
- The runtime owns the per-CPU production boundary for
  `PerCoreScheduler`, `PerCorePreemptionState`, target-owned
  `RemoteWakeQueue`, role/capability, and owner-local service of pending
  preemption through `CpuLocalSchedulerService::run_preemption_cycle`.
- Added target-independent unit coverage for IRQ-side record/coalesce without
  scheduler mutation, missing/invalid/wrong-owner/deferred-role reject cases,
  and owner-local service ordering with remote wake before timer preemption.
- Wired the normal QEMU timer IRQ path in `src/target/qemu_virt.rs` to record a
  bounded local production preemption request after the generic timer rearm
  helper and before EOI.
- Wired the normal Pi 5 timer IRQ path in `src/target/rpi5.rs` to perform the
  same bounded local recording while preserving existing acknowledge, rearm,
  timer-preemption diagnostic counter, and EOI order.

## Evidence

- static inspection: touched files are `src/scheduler.rs`,
  `src/target/qemu_virt.rs`, `src/target/rpi5.rs`, scheduler architecture,
  roadmap, decision log, this task record, and durable worker state.
- static inspection: IRQ paths record through production runtime preemption
  state only; scheduler queues, current task, remote wake queues, shared run
  queues, and metadata are not mutated in IRQ context.
- unit tests: `cargo -Zjson-target-spec test` passed with 156 no_std tests.
- fmt/lint: `cargo fmt --all -- --check` passed.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed with `talos: qemu smoke
  PASS`.
- QEMU/substitute: `scripts/qemu-timer-preemption-smoke.sh` passed with
  `qemu-timer-preemption-smoke: PASS`.
- QEMU/substitute: `scripts/qemu-secondary-scheduler-service-loop-smoke.sh`
  passed with `classification=qemu-secondary-scheduler-service-loop-complete`
  and PASS.
- QEMU/substitute: `scripts/qemu-shared-runqueue-migration-smoke.sh` passed
  with `classification=qemu-shared-runqueue-migration-complete` and PASS.
- QEMU/substitute: `scripts/qemu-load-balancing-smoke.sh` passed with
  `classification=qemu-load-balancing-smoke-complete` and PASS.
- QEMU/substitute: `scripts/qemu-multicore-preemption-smoke.sh` passed with
  `classification=qemu-multicore-preemption-smoke-complete` and PASS.
- whitespace inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.

## Diagnostic Surface Notes

No new boot scenario, proof script, image/archive helper, or Pi 5 hardware
claim was added. The next focused QEMU production proof must exercise the
normal timer IRQ recording path and owner-local post-IRQ service point before
any serialized Pi 5 production proof claims this integration physically.

## Result

Accepted as the first production timer/preemption core. The next bounded task
should be the focused QEMU production timer/preemption smoke. Pi 5 hardware,
remote current-task switching, running-task migration, autonomous work
stealing, general secondary idle loops, Phase 7, filesystem, networking, SSH,
shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
policy remain deferred.
