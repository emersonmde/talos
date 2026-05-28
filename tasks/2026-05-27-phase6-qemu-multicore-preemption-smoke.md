# Phase 6 QEMU Multi-Core Preemption Smoke

## Task

- Title: Phase 6 QEMU multi-core preemption smoke
- Owner: worker
- Date: 2026-05-27
- Milestone: Phase 6.3, Multi-Core Preemptive Scheduler
- Scope: focused QEMU substitute proof for the accepted multi-core preemption
  core

## Goal

Prove the accepted target-independent multi-core preemption core in QEMU
without making a physical Pi 5 claim.

## Acceptance Criteria

- A focused QEMU smoke reaches a stable PASS/classification for the accepted
  multi-core preemption invariant.
- Evidence shows each participating owner records and services only its own
  local pending timer-preemption request.
- The proof does not bypass owner-local service flow or mutate scheduler state
  from IRQ/IPI context.
- Retained scheduler regression gates remain passing.

## Context

The target-independent multi-core preemption core was accepted in
tasks/2026-05-27-phase6-multicore-preemption-core.md. This task adds only a
focused QEMU diagnostic scenario and runner that drive
PerCorePreemptionState::record_local_timer_irq and
CpuLocalSchedulerService::run_preemption_cycle on logical CPUs 1, 2, and 3.

## Work Performed

- Added the qemu_multicore_preemption_smoke boot scenario.
- Added scripts/qemu-multicore-preemption-smoke.sh.
- Added target::qemu_virt::run_multicore_preemption_smoke().
- The diagnostic starts three QEMU secondary CPUs through the accepted PSCI
  path. Each secondary owns a production-diagnostic scheduler, records a local
  pending timer-preemption request, coalesces a duplicate local record, rejects
  cross-owner recording, proves no current-task/runnable-queue/metadata
  mutation occurred during the record-only step, then services the request
  through owner-local normal scheduler control flow.
- Updated scheduler architecture, roadmap status, and decision log entries for
  the retained QEMU substitute proof.

## Evidence

- static inspection: changes are bounded to build.rs, src/main.rs,
  src/target/qemu_virt.rs, scripts/qemu-multicore-preemption-smoke.sh,
  scheduler/roadmap/decision docs, and this task record.
- git status: git status --short was clean before edits.
- QEMU/substitute: scripts/qemu-multicore-preemption-smoke.sh passed and
  emitted classification=qemu-multicore-preemption-smoke-complete.
- QEMU transcript: target/qemu-multicore-preemption-smoke.log shows logical
  CPUs 1, 2, and 3 each reporting record-outcome=inserted,
  duplicate-outcome=coalesced, cross-owner-rejected=true,
  irq-record-scheduler-mutated=false, pending-after-record=true, owner-local
  service-timer-preemption to the next task, previous task state returning to
  runnable, selected task state becoming running, pending-after-service=false,
  recorded=1, coalesced=1, serviced=1, per-owner metadata refresh, errors=0,
  and PASS.
- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed.
- QEMU/substitute: scripts/qemu-smoke.sh passed.
- QEMU/substitute: scripts/qemu-timer-preemption-smoke.sh passed.
- QEMU/substitute: scripts/qemu-secondary-scheduler-service-loop-smoke.sh
  passed.
- QEMU/substitute: scripts/qemu-shared-runqueue-migration-smoke.sh passed.
- QEMU/substitute: scripts/qemu-load-balancing-smoke.sh passed.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.

## Diagnostic Surface Notes

qemu_multicore_preemption_smoke and
scripts/qemu-multicore-preemption-smoke.sh are retained Phase 6.3 QEMU
substitute proof surfaces until the multi-core preemption closeout either keeps
them as regression gates or explicitly retires/replaces them. They do not add
Pi 5 staging, direct IRQ/IPI-context scheduling, remote current-task switching,
running-task migration, autonomous work stealing, general remote reschedule,
Phase 7, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver behavior.

## Review

- Pre-hardware review findings: hardware is not required; this task makes no
  physical claim.
- Hardware test evidence, if required: not required.
- Post-hardware review findings: not applicable.

## Result

Accepted as QEMU substitute evidence for the multi-core preemption core. The
next bounded task may be the serialized Pi 5 multi-core preemption proof only
after supervisor ready-marking and hardware lock availability.
