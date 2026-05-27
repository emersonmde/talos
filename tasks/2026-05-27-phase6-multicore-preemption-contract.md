# Phase 6 Multi-Core Preemption Contract

## Task

- Title: Phase 6 multi-core preemption contract
- Owner: worker
- Date: 2026-05-27
- Milestone: Phase 6.3, Multi-Core Preemptive Scheduler
- Scope: documentation and contract only

## Goal

Define the first multi-core preemption contract after source inventory while
keeping scheduler mutation out of asynchronous interrupt and IPI context.

## Acceptance Criteria

- Contract states the first allowed multi-core preemption invariant and all
  deferred behaviors.
- Failure modes for stale metadata, wrong-owner scheduler access,
  nested/preemption-disabled sections, pending remote wake, and full/shared
  queues are explicit.
- QEMU and Pi 5 proof obligations are named but not implemented.
- Accepted contract is committed before multi-core preemption code starts.

## Context

The accepted source inventory is
docs/src/project/phase6-multicore-preemption-source-inventory.md. It showed
that accepted timer IRQ paths record bounded local state, owner-local service
cycles perform scheduler mutation after IRQ return, IPI/wake paths remain
notification or target-owned wake mailboxes, and shared metadata,
SharedRunQueue, and load-balancing surfaces do not authorize remote switching
of another CPU's current task.

## Work Performed

- Added docs/src/project/phase6-multicore-preemption-contract.md.
- Updated docs/src/architecture/scheduler.md with the accepted contract
  summary and implementation boundary.
- Updated docs/src/SUMMARY.md so mdBook includes the new project record.
- Updated docs/src/roadmap.md to reflect the accepted contract frontier.
- Updated docs/src/decisions/README.md with the accepted decision entry.
- Updated this durable task record.

## Evidence

- static inspection: reviewed the accepted multi-core preemption source
  inventory, scheduler architecture, load-balancing closeout and contract,
  shared run-queue/migration contract, src/scheduler.rs, src/smp.rs,
  src/smp_sync.rs, src/arch/aarch64/generic_timer.rs,
  src/arch/aarch64/exceptions.rs, src/arch/aarch64/gicv2.rs, roadmap, and
  decision log.
- contract summary: documented timer/IPI bounded recorder rules,
  owner-local normal-control-flow scheduler mutation, current-task authority,
  lock ordering, metadata freshness, preemption-disable expectations,
  deterministic failure outcomes, and proof plan.
- deferral list: preserved direct IRQ/IPI-context scheduling, remote current
  switching, running-task migration, work stealing, general remote reschedule,
  non-diagnostic secondary runtime, Phase 7, filesystem, networking, SSH,
  shell, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver deferrals.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.

## Review

- Pre-hardware review findings: hardware is not required; this task makes no
  physical claim.
- Hardware test evidence, if required: not required.
- Post-hardware review findings: not applicable.

## Result

Accepted as the Phase 6.3 multi-core preemption contract. The next bounded
task should be phase6-multicore-preemption-core-20260527. Multi-core
preemption implementation may begin only inside this contract; direct
IRQ/IPI-context scheduling, remote current switching, running-task migration,
work stealing, general remote reschedule, Phase 7, filesystem, networking,
SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
policy remain deferred.
