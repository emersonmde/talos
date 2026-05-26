# Phase 6 Shared Run-Queue and Migration Contract

## Task

- Title: Phase 6 shared run-queue and migration contract
- Owner: worker
- Date: 2026-05-26
- Milestone: Phase 6.3, Multi-Core Preemptive Scheduler
- Scope: documentation and contract only

## Goal

Define the shared run-queue and migration ownership contract before any
implementation begins.

## Acceptance Criteria

- Contract describes lock ownership, IRQ masking interaction, memory-order
  requirements, state transitions, and failure/diagnostic reporting
  boundaries.
- Contract explicitly separates shared run-queue/migration from later load
  balancing and multi-core preemption.
- Scheduler architecture docs and project checkpoint references remain
  consistent.
- Accepted contract is committed before shared run-queue core implementation
  starts.

## Context

The accepted source inventory is
docs/src/project/phase6-shared-runqueue-migration-source-inventory.md. It
showed that Talos has owner-local runnable queues, target-owned remote wake
mailboxes, owner-published metadata, accepted SMP lock primitives, and
diagnostic proof routing, but no shared scheduler topology.

The current implementation sources reviewed for this contract are:

- src/scheduler.rs
- src/smp_sync.rs
- docs/src/architecture/scheduler.md
- docs/src/roadmap.md
- docs/src/decisions/README.md

## Work Performed

- Added docs/src/project/phase6-shared-runqueue-migration-contract.md.
- Updated docs/src/architecture/scheduler.md with the accepted contract
  summary and implementation boundary.
- Updated docs/src/SUMMARY.md so the contract appears with the Phase 6
  project records.
- Updated docs/src/roadmap.md to reflect the accepted contract frontier.
- Updated docs/src/decisions/README.md with the accepted decision entry.

## Evidence

- static inspection: reviewed the accepted source inventory, scheduler
  architecture, scheduler source symbols, SMP synchronization primitives,
  roadmap, and decision log.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed and wrote the HTML book under book/.

## Review

- Pre-hardware review findings: hardware was not required; this task makes no
  physical claim.
- Hardware test evidence, if required: not required.
- Post-hardware review findings: not applicable.

## Result

Accepted as a documentation-only Phase 6.3 contract. The next bounded
implementation may add a shared run-queue core only if it preserves this
contract and remains separate from load balancing, work stealing, multi-core
preemption, Phase 7, filesystem, networking, SSH, shell behavior, RP1/PCIe,
UART interrupt ownership, and DMA/cache-driver policy.

## Follow-Up

Recommended next task after supervisor planning: phase6-shared-runqueue-core-20260526.
