# Phase 6 Load-Balancing Core

## Task

- Title: Phase 6 load-balancing core
- Owner: worker
- Date: 2026-05-27
- Milestone: Phase 6.3, Multi-Core Preemptive Scheduler
- Scope: target-independent load-balancing policy core and tests

## Goal

Implement the first target-independent load-balancing policy core inside the
accepted contract without adding QEMU or Pi 5 proof claims.

## Acceptance Criteria

- Implementation follows the accepted policy contract without expanding
  diagnostic-only roles into general runtime behavior.
- Unit tests prove selected success and deterministic rejection paths.
- Existing shared run-queue and secondary service-loop gates still pass.
- Accepted implementation is committed before QEMU proof work starts.

## Context

The accepted policy contract is
`docs/src/project/phase6-load-balancing-policy-contract.md`. It permits a
small owner-local policy that selects one source-owned runnable, non-current
task and one production-capable destination, then uses the accepted
`SharedRunQueue` owner-transfer mechanism. It keeps work stealing,
running-task migration, interrupt-driven remote reschedule, and multi-core
preemption deferred.

## Work Performed

- Added `LoadBalancingPolicy`, `LoadBalancingPlan`,
  `LoadBalancingPublishReport`, and `LoadBalancingPolicyError` to
  `src/scheduler.rs`.
- Implemented front-runnable source planning from owner-local queue state and
  owner-published `SharedSchedulerMetadata`.
- Added destination role and capacity checks plus shared queue full/duplicate
  backpressure checks before publication.
- Routed publication through `SharedRunQueue::publish_migration` so stale
  metadata, source-local removal, duplicate shared membership, and migration
  rejection stay under the accepted owner-transfer mechanism.
- Added unit tests for successful front-runnable publication, stale-plan
  rejection, destination queue backpressure, invalid/deferred destination
  rejection, shared queue backpressure, and single-owner queue membership.
- Updated scheduler architecture, roadmap, and decision docs for the accepted
  implementation boundary.

## Evidence

- static inspection: changes are bounded to `src/scheduler.rs`, scheduler
  architecture/roadmap/decision docs, and this task record. No scripts, boot
  scenarios, target routing, QEMU proof, or Pi 5 hardware surfaces were
  changed.
- git status: `git status --short` was clean before edits.
- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed with 147 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-secondary-scheduler-service-loop-smoke.sh`
  passed, preserving the accepted owner-local secondary service-loop gate.
- QEMU/substitute: `scripts/qemu-shared-runqueue-migration-smoke.sh` passed,
  preserving the accepted SharedRunQueue migration regression gate.
- whitespace inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.

## Diagnostic Surface Notes

No proof-only scripts, boot scenarios, cfg aliases, target routing, QEMU
diagnostics, or Pi 5 staging surfaces were added or modified. The accepted
QEMU and Pi 5 load-balancing proof tasks remain separate queued work.

## Review

- Pre-hardware review findings: hardware was not required; this task makes no
  physical claim.
- Hardware test evidence, if required: not required.
- Post-hardware review findings: not applicable.

## Result

Accepted as the target-independent load-balancing core. The next bounded task
should be a focused QEMU load-balancing smoke that proves the implemented
policy can select a destination and publish through `SharedRunQueue` without
bypassing the owner-transfer mechanism. Pi 5 proof, work stealing,
running-task migration, interrupt-driven remote reschedule, multi-core
preemption, Phase 7, filesystem, networking, SSH, shell behavior, RP1/PCIe,
UART interrupt ownership, and DMA/cache-driver policy remain deferred.
