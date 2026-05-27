# Phase 6 QEMU Shared Run-Queue Migration Smoke

## Task

- Title: Phase 6 QEMU shared run-queue/migration smoke
- Owner: worker
- Date: 2026-05-26
- Milestone: Phase 6.3, Multi-Core Preemptive Scheduler
- Scope: deterministic QEMU substitute proof for the accepted shared
  run-queue/migration core

## Goal

Prove the accepted target-independent shared run-queue/migration core with a
focused QEMU diagnostic before any Pi 5 physical claim.

## Acceptance Criteria

- QEMU transcript proves the named shared run-queue/migration invariant through
  the implemented core, not a bypass.
- Existing scheduler smoke tests still pass.
- Diagnostic surface retention or cleanup expectations are documented.
- Accepted QEMU proof is committed before any Pi 5 physical proof.

## Context

The target-independent core was accepted in
`tasks/2026-05-26-phase6-shared-runqueue-core.md`. This task adds only a QEMU
diagnostic scenario and runner that drive `SharedRunQueue::publish_migration`
and `SharedRunQueue::consume_for_destination` through the same scheduler and
metadata APIs used by the unit-tested implementation.

## Work Performed

- Added the `qemu_shared_runqueue_migration` boot scenario.
- Added `scripts/qemu-shared-runqueue-migration-smoke.sh`.
- Added `target::qemu_virt::run_shared_runqueue_migration_smoke()`.
- The diagnostic builds a source owner, destination owner, shared metadata
  table, shared run queue, and runnable task, then proves the source-owner
  publish and destination-owner consume sequence:
  `MigrationReserved -> SharedQueued -> DestinationEnqueued`.
- Updated scheduler architecture, roadmap status, and decision log entries for
  the accepted QEMU substitute proof.

## Evidence

- static inspection: changes are bounded to `build.rs`, `src/main.rs`,
  `src/target/qemu_virt.rs`, `scripts/qemu-shared-runqueue-migration-smoke.sh`,
  scheduler/roadmap/decision docs, and this task record.
- QEMU/substitute: `scripts/qemu-shared-runqueue-migration-smoke.sh` passed and
  emitted
  `classification=qemu-shared-runqueue-migration-complete`.
- QEMU transcript: `target/qemu-shared-runqueue-migration-smoke.log` shows
  source owner 0 publishing task 107 to destination owner 1, source local queue
  length changing 1 -> 0, shared queue length changing 1 -> 0, destination
  queue length becoming 1 with front task 107, metadata owner changing to 1,
  and PASS.
- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed with 142 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- whitespace inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.

## Diagnostic Surface Notes

`qemu_shared_runqueue_migration` and
`scripts/qemu-shared-runqueue-migration-smoke.sh` are retained Phase 6.3 QEMU
substitute proof surfaces until a later shared run-queue closeout either keeps
them as regression gates or explicitly retires/replaces them. They do not add
Pi 5 staging, load balancing, work stealing, multi-core preemption, secondary
runtime policy, Phase 7, filesystem, networking, SSH, shell behavior,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver behavior.

## Review

- Pre-hardware review findings: hardware is not required; this task makes no
  physical claim.
- Hardware test evidence, if required: not required.
- Post-hardware review findings: not applicable.

## Result

Accepted as QEMU substitute evidence for the shared run-queue/migration core.
The next bounded task may be the serialized Pi 5 shared run-queue/migration
proof only after supervisor ready-marking and hardware lock availability.
