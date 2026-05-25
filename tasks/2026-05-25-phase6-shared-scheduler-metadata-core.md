# Phase 6 Shared Scheduler Metadata Core

Task ID: phase6-shared-scheduler-metadata-core-20260525

## Goal

Implement the smallest shared scheduler metadata core needed to name CPU-local
diagnostic tasks across cores without enabling shared run queues or migration.

## Scope

- Added scheduler metadata records and owner-only APIs in src/scheduler.rs.
- Tracked owning logical CPU, scheduler-local task ID, task state, optional
  process owner, kernel-stack bounds, owner-local current/runnable membership,
  and generation-based stale snapshot rejection.
- Named SharedSchedulerMetadataLock as the accepted SpinLock boundary for
  future shared metadata use while keeping local runnable queue mutation inside
  PerCoreScheduler ownership.
- Added focused no_std unit tests for registration, lookup, duplicate
  registration, invalid owner/task ID handling, stale snapshots, and rejected
  cross-owner publication without local queue mutation.

## Non-Goals

No shared run queue, task migration, load balancing, work stealing, remote
enqueue queue, production task movement, multi-core preemption, timer policy
change, IPI hot-path scheduler mutation, userspace, descriptors, filesystem,
networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or DMA
behavior was added.

## Implementation Summary

- src/scheduler.rs: added SchedulerTaskSnapshot,
  SharedSchedulerMetadataError, SharedSchedulerMetadata, and
  SharedSchedulerMetadataLock.
- docs/src/architecture/scheduler.md: documented the implemented metadata
  core and its CPU-local ownership boundary.
- docs/src/roadmap.md and docs/src/decisions/README.md: recorded acceptance
  and deferred broader scheduler topology work.

## Evidence

- Static inspection: implementation matches the accepted metadata contract and
  does not alter existing CPU-local dispatch behavior.
- Unit tests: cargo -Zjson-target-spec test passed with 125 no_std tests,
  including shared scheduler metadata registration, refresh, duplicate/unknown
  lookup, invalid owner/task ID, stale snapshot, lock boundary, and cross-owner
  rejection coverage.
- QEMU/substitute: scripts/qemu-smoke.sh passed.
- QEMU/substitute: scripts/qemu-per-core-scheduler-ownership-smoke.sh passed.
- QEMU/substitute: scripts/qemu-remote-wake-to-local-runnable-smoke.sh passed.
- QEMU/substitute: scripts/qemu-production-secondary-dispatch-smoke.sh passed.
- Formatting/docs: cargo fmt --all -- --check, git diff --check, and mdbook
  build passed.

## Acceptance

Accepted and committed as the bounded shared scheduler metadata core. The next
queued task is phase6-qemu-shared-scheduler-metadata-smoke-20260525, not Pi 5
hardware proof, shared run queues, migration, load balancing, multi-core
preemption, or later roadmap work.
