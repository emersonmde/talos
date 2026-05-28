# Phase 6 Multi-Core Preemption Core

## Task

- Title: Phase 6 multi-core preemption core
- Owner: worker
- Date: 2026-05-27
- Milestone: Phase 6.3, Multi-Core Preemptive Scheduler
- Scope: target-independent multi-core preemption state and owner-local
  service integration

## Goal

Implement the first bounded multi-core preemption core according to the
accepted contract without adding proof routing or hardware claims.

## Acceptance Criteria

- Implementation matches the accepted contract and keeps scheduler mutation
  owner-local.
- Unit tests cover success and deterministic defer/reject cases.
- Existing Phase 4 timer-preemption, Phase 6 secondary service-loop, shared
  run-queue, and load-balancing regression gates remain passing.
- Accepted core is committed before QEMU or Pi 5 proof tasks claim it.

## Context

The accepted contract is
`docs/src/project/phase6-multicore-preemption-contract.md`. It allows timer
and IPI paths to record bounded state only, while owner-local normal control
flow performs scheduler mutation after interrupt return. It requires
deterministic behavior for no pending request, no runnable peer, wrong-owner
access, current-task mismatch, nested/preemption-disabled sections, pending
remote wake, stale metadata, and full queues.

## Work Performed

- Added `PerCorePreemptionState`, `PerCorePreemptionCounters`,
  `PreemptionRecordOutcome`, and `PerCorePreemptionStateError` in
  `src/scheduler.rs`.
- Implemented `record_local_timer_irq` as the bounded IRQ-side recording hook
  that only sets/coalesces local pending timer-preemption state.
- Added explicit nested preemption-disable enter/exit state with deterministic
  underflow, overflow, and service-defer outcomes.
- Added `CpuLocalSchedulerService::run_preemption_cycle` as the owner-local
  service entry that preflights preemption-state owner, scheduler owner,
  production role, and current-task identity before draining wake queues or
  mutating scheduler state.
- Preserved the existing service order once preflight succeeds: target-owned
  remote wake consumption, local timer preemption, optional local dispatch only
  when no timer preemption was serviced, and owner-published metadata refresh.
- Added unit tests for local request recording/coalescing, nested disable
  defer, owner-local successful service, wrong-owner rejection before queue
  mutation, current-task mismatch rejection before service, and pending
  request retention after no-runnable failure.
- Updated scheduler architecture, roadmap, and decision docs for the accepted
  core boundary.

## Evidence

- static inspection: changes are bounded to `src/scheduler.rs`, scheduler
  architecture/roadmap/decision docs, this task record, and durable worker
  state. No scripts, boot scenarios, target routing, QEMU proof routing, or Pi
  5 hardware staging surfaces were changed.
- git status: `git status --short` was clean before edits.
- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed with 153 no_std tests.
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
- whitespace inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.

## Diagnostic Surface Notes

No proof-only scripts, boot scenarios, cfg aliases, target routing, QEMU
diagnostics, or Pi 5 staging surfaces were added or modified. The accepted
QEMU and Pi 5 multi-core preemption proof tasks remain separate queued work.

## Review

- Pre-hardware review findings: hardware was not required; this task makes no
  physical claim.
- Hardware test evidence, if required: not required.
- Post-hardware review findings: not applicable.

## Result

Accepted as the target-independent multi-core preemption core. The next
bounded task should be a focused QEMU multi-core preemption smoke that proves
multiple logical CPUs can record local timer-preemption requests and service
them from owner-local normal control flow. Pi 5 proof, direct IRQ/IPI-context
scheduling, remote current-task switching, running-task migration, autonomous
work stealing, general remote reschedule, Phase 7, filesystem, networking,
SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
policy remain deferred.
