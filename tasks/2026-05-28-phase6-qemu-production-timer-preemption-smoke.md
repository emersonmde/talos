# Phase 6 QEMU Production Timer/Preemption Smoke

## Task

- Title: Phase 6 QEMU production timer/preemption smoke
- Owner: worker
- Date: 2026-05-28
- Milestone: Phase 6.3, Production Scheduler Runtime Integration
- Scope: focused QEMU substitute proof for the accepted production
  timer/preemption runtime integration

## Goal

Prove in QEMU that the accepted production timer/preemption runtime boundary
records through the target-owned production timer IRQ adapter and services
pending preemption only through owner-local normal scheduler control flow.

## Acceptance Criteria

- A focused QEMU smoke reaches stable PASS/classification for the production
  timer/preemption invariant.
- Evidence distinguishes the production runtime entry path from direct
  diagnostic helper calls.
- Retained scheduler regression gates remain passing.
- No Pi 5 hardware claim is made by this task.

## Work Performed

- Added the `qemu_production_timer_preemption_smoke` boot scenario.
- Added `scripts/qemu-production-timer-preemption-smoke.sh`.
- Added `target::qemu_virt::run_production_timer_preemption_smoke()`.
- The diagnostic starts QEMU secondary logical CPUs 1, 2, and 3 through the
  accepted PSCI path. Each secondary initializes a production-capable
  `ProductionSchedulerRuntime`, records a pending timer-preemption request
  through the target-owned production timer IRQ adapter, coalesces a duplicate
  adapter record, rejects a wrong-owner record, proves current-task,
  runnable-queue, task-state, and metadata state did not mutate during the
  record-only step, then services pending preemption through
  `ProductionSchedulerRuntime::service_pending_preemption`.
- Updated scheduler architecture, roadmap status, and decision log entries for
  the retained QEMU substitute proof.

## Evidence

- static inspection: changes are bounded to `build.rs`, `src/main.rs`,
  `src/target/qemu_virt.rs`,
  `scripts/qemu-production-timer-preemption-smoke.sh`, scheduler
  architecture/roadmap/decision docs, this task record, and durable worker
  state.
- git status: `git status --short` was clean before edits.
- QEMU/substitute: `scripts/qemu-production-timer-preemption-smoke.sh`
  passed and emitted
  `classification=qemu-production-timer-preemption-smoke-complete`.
- QEMU transcript: `target/qemu-production-timer-preemption-smoke.log`
  shows logical CPUs 1, 2, and 3 each reporting
  `entry-path=production-timer-irq-adapter`,
  `record-outcome=inserted`, `duplicate-outcome=coalesced`,
  `cross-owner-rejected=true`, `record-misses=0`,
  `timer-record-rejections=1`, `irq-record-scheduler-mutated=false`,
  `pending-after-record=true`, owner-local service to the next task,
  previous task state returning to runnable, selected task state becoming
  running, `pending-after-service=false`, `recorded=1`,
  `coalesced=1`, `serviced=1`, per-owner metadata refresh,
  `errors=0`, and PASS.
- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-timer-preemption-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-secondary-scheduler-service-loop-smoke.sh`
  passed.
- QEMU/substitute: `scripts/qemu-shared-runqueue-migration-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-load-balancing-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-multicore-preemption-smoke.sh` passed.
- whitespace inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.
- hardware: no Pi 5 hardware claim was made and hardwareTestLock was not
  acquired.

## Diagnostic Surface Notes

`qemu_production_timer_preemption_smoke` and
`scripts/qemu-production-timer-preemption-smoke.sh` are retained Phase 6.3
QEMU substitute proof surfaces until the production scheduler runtime closeout
keeps them as regression gates or explicitly retires/replaces them. They do
not add Pi 5 staging, direct IRQ/IPI-context scheduler mutation, remote
current-task switching, running-task migration, autonomous work stealing,
Phase 7, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver behavior.

## Review

- Pre-hardware review findings: hardware is not required; this task makes no
  physical claim.
- Hardware test evidence, if required: not required.
- Post-hardware review findings: not applicable.

## Result

Accepted as QEMU substitute evidence for the production timer/preemption
runtime integration. The next bounded task may be the serialized Pi 5
production timer/preemption proof only after hardware lock availability.
