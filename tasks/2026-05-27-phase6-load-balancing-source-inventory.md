# Phase 6 Load-Balancing Source Inventory

## Task

- Title: Phase 6 load-balancing source inventory
- Owner: worker
- Date: 2026-05-27
- Milestone: Phase 6.3, Multi-Core Preemptive Scheduler
- Scope: documentation and source inventory only

## Goal

Inventory the scheduler, metadata, run-queue, wake, timer, and diagnostic
surfaces needed before Talos designs load-balancing policy.

## Acceptance Criteria

- Inventory cites concrete source/doc/evidence surfaces and distinguishes
  accepted capability from diagnostic-only proof scaffolding.
- Required load-balancing policy inputs and stale/invalid input failure modes
  are listed before implementation.
- The next contract task is explicit and the worker does not need to infer it
  from the roadmap.
- Accepted work is committed before any load-balancing implementation starts.

## Context

The accepted shared run-queue/migration closeout is
docs/src/project/phase6-shared-runqueue-migration-closeout-checkpoint.md.
It accepted the target-independent owner-transfer core plus QEMU and Pi 5
diagnostic evidence, but explicitly deferred target selection, fairness,
affinity, work stealing, remote reschedule, and multi-core preemption.

## Work Performed

- Added docs/src/project/phase6-load-balancing-source-inventory.md.
- Updated docs/src/SUMMARY.md so mdBook includes the new project record.
- Updated docs/src/roadmap.md to reflect that the next bounded task is a
  load-balancing contract, not implementation.
- Updated docs/src/decisions/README.md with the accepted inventory decision.
- Updated this durable task record.

## Evidence

- static inspection: reviewed accepted shared run-queue closeout, scheduler
  architecture, src/scheduler.rs, src/smp.rs, src/smp_sync.rs, retained Phase
  6 QEMU/Pi 5 diagnostic surfaces, roadmap, decision log, task records, and
  evidence summaries.
- source/doc inventory summary: recorded scheduler ownership, owner-local
  queues, remote wake, CPU-local service, secondary service loop, shared
  metadata, shared run-queue, SMP lock, SMP lifecycle, and diagnostic proof
  surfaces.
- policy inputs and failure modes: recorded CPU role, runnable pressure,
  current-task state, task membership, metadata generation, shared queue
  capacity, wake state, timer-preemption state, stale metadata, running/blocked
  candidates, full queues, deferred roles, cross-owner mutation, remote wake
  confusion, timer confusion, and diagnostic overreach.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.

## Review

- Pre-hardware review findings: hardware was not required; this task makes no
  physical claim.
- Hardware test evidence, if required: not required.
- Post-hardware review findings: not applicable.

## Result

Accepted as the Phase 6.3 load-balancing source inventory. The next bounded
task should be phase6-load-balancing-contract-20260527. Load-balancing
implementation, work stealing, running-task migration, remote reschedule,
multi-core preemption, Phase 7, filesystem, networking, SSH, shell behavior,
RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy remain
deferred.
