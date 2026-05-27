# Phase 6 Multi-Core Preemption Source Inventory

## Task

- Title: Phase 6 multi-core preemption source inventory
- Owner: worker
- Date: 2026-05-27
- Milestone: Phase 6.3, Multi-Core Preemptive Scheduler
- Scope: documentation and source inventory only

## Goal

Inventory accepted timer, scheduler, SMP, IPI/wake, metadata, SharedRunQueue,
load-balancing, and diagnostic proof boundaries before Talos defines a
multi-core preemption contract.

## Acceptance Criteria

- Inventory cites concrete source files/functions/docs for timer IRQ
  recording, preemption request handling, CPU-local scheduler service,
  secondary service-loop dispatch, IPI/remote wake handling, metadata
  publication, SharedRunQueue ownership transfer, and load-balancing policy.
- Current CPU-local versus cross-core preemption assumptions are explicit.
- Implementation blockers and required contracts are listed before code
  changes.
- Next multi-core preemption contract task is recommended without requiring
  roadmap inference.
- Accepted inventory is committed before any multi-core preemption
  implementation begins.

## Context

The accepted load-balancing closeout is
docs/src/project/phase6-load-balancing-closeout-checkpoint.md. It accepted a
deterministic policy primitive and retained gates, then recommended multi-core
preemption source inventory as the next bounded Phase 6.3 task.

## Work Performed

- Added docs/src/project/phase6-multicore-preemption-source-inventory.md.
- Updated docs/src/SUMMARY.md so mdBook includes the new project record.
- Updated docs/src/roadmap.md to record the accepted inventory and next
  bounded contract recommendation.
- Updated docs/src/decisions/README.md with the accepted inventory decision.
- Updated this durable task record.

## Evidence

- static inspection: git status --short was clean before edits.
- source inventory: reviewed src/scheduler.rs, src/smp.rs, src/smp_sync.rs,
  src/arch/aarch64/generic_timer.rs, src/arch/aarch64/exceptions.rs,
  src/arch/aarch64/gicv2.rs, src/target/qemu_virt.rs, src/target/rpi5.rs,
  retained Phase 6 scripts, scheduler architecture docs, accepted
  load-balancing task records, roadmap, and decision log.
- boundary summary: documented CPU-local timer-preemption handling,
  owner-local scheduler mutation, target-owned remote wake consumption,
  notification-only IPI handling, owner-published metadata, SharedRunQueue
  owner transfer, and deterministic load-balancing policy.
- blocker list: documented current-task authority, preemption-disable policy,
  IRQ/IPI context boundary, lock ordering, metadata freshness, remote
  reschedule semantics, secondary runtime role, running-task migration, and
  proof routing gaps.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.

## Review

- Pre-hardware review findings: hardware was not required; this task makes no
  physical claim.
- Hardware test evidence, if required: not required.
- Post-hardware review findings: not applicable.

## Result

Accepted as the Phase 6.3 multi-core preemption source inventory. The next
bounded task should be phase6-multicore-preemption-contract-20260527.
Multi-core preemption implementation, remote reschedule implementation, work
stealing, running-task migration, Phase 7, filesystem, networking, SSH, shell
behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy
remain deferred.
