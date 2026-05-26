# Phase 6 Shared Run-Queue Core

## Task

- Title: Phase 6 shared run-queue core
- Owner: worker
- Date: 2026-05-26
- Milestone: Phase 6.3, Multi-Core Preemptive Scheduler
- Scope: target-independent shared run-queue/migration core and tests

## Goal

Implement the smallest target-independent shared run-queue/migration core that
satisfies the accepted contract without load balancing or multi-core
preemption.

## Acceptance Criteria

- Core implementation compiles and preserves existing accepted scheduler gates.
- Unit tests cover the new queue/migration invariants, including ownership,
  empty/full/error cases, and state transitions named by the contract.
- Diagnostic/proof-only surfaces remain quarantined and documented.
- Accepted implementation is committed before QEMU or Pi 5 proof tasks.

## Context

The accepted contract is
`docs/src/project/phase6-shared-runqueue-migration-contract.md`. It requires a
shared owner-transfer surface that keeps task mutation single-owner, separates
remote wake from remote enqueue/migration, preserves local-IRQ-then-SMP-lock
ordering at callers, and reports deterministic failure boundaries.

This task intentionally follows the accepted cfg-routing cleanup precursor at
commit `489b557` and makes no boot-scenario or cfg-routing changes.

## Work Performed

- Added `MigrationState`, `SharedRunQueueEntry`, `SharedRunQueue`,
  `SharedRunQueueLock`, publish/consume reports, and deterministic
  `SharedRunQueueError` outcomes in `src/scheduler.rs`.
- Added `RunnableQueue::remove` so migration publication can remove a task
  from the source-local queue before publishing a shared entry.
- Added source-owner `publish_migration` and destination-owner
  `consume_for_destination` paths with metadata generation checks, destination
  role checks, duplicate membership checks, full queue checks, and explicit
  `MigrationReserved -> SharedQueued -> DestinationEnqueued` reporting.
- Added unit coverage for FIFO removal, wrapped removal, successful handoff
  publication, destination consumption and metadata owner transfer, stale
  metadata rejection, running/blocked/full rejection, deferred destination
  rejection, duplicate destination-local enqueue rejection, and shared queue
  lock wrapping.
- Updated `docs/src/architecture/scheduler.md`, `docs/src/roadmap.md`, and
  `docs/src/decisions/README.md` for the accepted implementation boundary.

## Evidence

- static inspection: code changes are bounded to `src/scheduler.rs` plus
  architecture, roadmap, decision, and task documentation. No scripts, boot
  scenarios, target routing, or hardware proof surfaces were changed.
- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed with 142 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-secondary-scheduler-service-loop-smoke.sh`
  passed as the focused existing Phase 6 scheduler gate. Rationale: it is the
  retained owner-local service-loop proof that exercises scheduler metadata,
  remote wake drain, CPU-local dispatch, deferred-role rejection, and lock
  availability without using the new shared run queue.
- whitespace inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.

## Diagnostic Surface Notes

No proof-only scripts, boot scenarios, cfg aliases, target routing, QEMU
diagnostics, or Pi 5 staging surfaces were added or modified. The accepted
QEMU and Pi 5 shared run-queue proof tasks remain separate queued work.

## Review

- Pre-hardware review findings: hardware was not required; this task makes no
  physical claim.
- Hardware test evidence, if required: not required.
- Post-hardware review findings: not applicable.

## Result

Accepted as the target-independent shared run-queue/migration core. The next
bounded task should be a QEMU shared run-queue/migration smoke that proves the
implemented core without bypassing it. Pi 5 proof, load balancing, work
stealing, multi-core preemption, Phase 7, filesystem, networking, SSH, shell
behavior, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy
remain deferred.
