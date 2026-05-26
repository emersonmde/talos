# Phase 6 Shared Run-Queue and Migration Source Inventory

Task ID: phase6-shared-runqueue-migration-source-inventory-20260526
Status: accepted
Owner: worker

## Goal

Inventory the scheduler sources and accepted Phase 6.3 checkpoints needed
before designing shared run queues, remote enqueue, migration, load balancing,
or multi-core preemption.

## Scope

- Reviewed scheduler, CPU-local service, secondary service-loop, remote wake,
  shared metadata, SMP lock, target proof-routing, docs, and accepted task
  records.
- Identified owner-local runnable queue assumptions, lock/memory-order
  boundaries, metadata publication points, and migration blockers.
- Classified reusable pieces versus pieces that require a new shared
  run-queue/migration contract.
- Recommended the next bounded contract task without implementation.

## Non-Goals

- No Rust implementation.
- No shared run queue, remote enqueue queue, task migration, load balancing,
  work stealing, remote reschedule, or multi-core preemption.
- No QEMU or Pi 5 boot image.
- No Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt
  ownership, or DMA/cache-driver policy.

## Artifacts

- Project inventory:
  docs/src/project/phase6-shared-runqueue-migration-source-inventory.md.
- Roadmap update: docs/src/roadmap.md.
- Decision log update: docs/src/decisions/README.md.
- Summary entry: docs/src/SUMMARY.md.

## Evidence

- static inspection: `git status --short` was clean before edits.
- rg/static source inventory: inspected `src/scheduler.rs`, `src/smp.rs`,
  `src/smp_sync.rs`, `src/target/qemu_virt.rs`, `src/target/rpi5.rs`,
  retained scheduler proof scripts, scheduler architecture docs, roadmap,
  decision log, accepted task records, and evidence summaries.
- whitespace inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.

Rust fmt/tests, QEMU reruns, and hardware runs were not required because this
task changed only Markdown documentation and durable task state.

## Result

Accepted. The inventory names the concrete owners for local runnable queues,
target-owned wake mailboxes, CPU-local service sequencing, secondary
service-loop entry, owner-published metadata, SMP locks, diagnostic proof
routing, and retained scripts. It records that the current design remains
owner-local: remote wake is not remote enqueue, metadata is observational, and
there is no global task registry, shared run queue, migration state machine,
remote reschedule policy, lock hierarchy, load-balancing policy, or general
secondary production runtime role.

Recommended next task:
`phase6-shared-runqueue-migration-contract-20260526`.
