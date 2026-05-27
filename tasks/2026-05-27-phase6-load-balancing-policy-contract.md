# Phase 6 Load-Balancing Policy Contract

## Task

- Title: Phase 6 load-balancing policy contract
- Owner: worker
- Date: 2026-05-27
- Milestone: Phase 6.3, Multi-Core Preemptive Scheduler
- Scope: documentation and contract only

## Goal

Define the first load-balancing policy contract before code selects targets or
migrates work automatically.

## Acceptance Criteria

- Contract cleanly separates policy from mechanism and does not weaken the
  accepted shared run-queue/migration ownership contract.
- Remote reschedule, if required, is specified as notification only and not
  scheduler execution in IPI context.
- Validation strategy for core, QEMU substitute proof, Pi 5 proof, and
  closeout is named.
- Accepted contract is committed before implementation starts.

## Context

The accepted source inventory is
docs/src/project/phase6-load-balancing-source-inventory.md. It showed that
Talos has enough scheduler metadata, owner-local queue pressure, CPU role, and
SharedRunQueue capacity signals to define a minimal policy, but still lacks
fairness accounting, affinity, production secondary idle/wake behavior, remote
reschedule semantics, and multi-core preemption.

## Work Performed

- Added docs/src/project/phase6-load-balancing-policy-contract.md.
- Updated docs/src/architecture/scheduler.md with the accepted contract
  summary and implementation boundary.
- Updated docs/src/SUMMARY.md so mdBook includes the new project record.
- Updated docs/src/roadmap.md to reflect the accepted contract frontier.
- Updated docs/src/decisions/README.md with the accepted decision entry.
- Updated this durable task record.

## Evidence

- static inspection: reviewed the accepted load-balancing source inventory,
  shared run-queue/migration contract and closeout, scheduler architecture,
  src/scheduler.rs, src/smp.rs, src/smp_sync.rs, roadmap, and decision log.
- contract summary: recorded policy authority, accepted target-selection
  inputs, freshness and rollback rules, conservative fairness/affinity limits,
  polling-only first remote-reschedule behavior, and deterministic failure
  outcomes.
- alternatives: selected a minimal deterministic policy over immediate work
  stealing, affinity/fairness design, or interrupt-driven remote reschedule.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.

## Review

- Pre-hardware review findings: hardware was not required; this task makes no
  physical claim.
- Hardware test evidence, if required: not required.
- Post-hardware review findings: not applicable.

## Result

Accepted as the Phase 6.3 load-balancing policy contract. The next bounded
task should be phase6-load-balancing-core-20260527. Load-balancing
implementation may begin only inside this contract; work stealing,
running-task migration, interrupt-driven remote reschedule, multi-core
preemption, Phase 7, filesystem, networking, SSH, shell behavior, RP1/PCIe,
UART interrupt ownership, and DMA/cache-driver policy remain deferred.
