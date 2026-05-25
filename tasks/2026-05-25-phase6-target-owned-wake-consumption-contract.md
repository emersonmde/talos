# Phase 6 Target-Owned Wake Consumption Contract

Task ID: phase6-target-owned-wake-consumption-contract-20260525
Status: accepted

## Goal

Define the target-owned path from a consumed remote wake request to a local
scheduler wake action before any implementation mutates runnable state from
cross-core signals.

## Scope

- Inventoried accepted scheduler, `RemoteWakeQueue`, per-core ownership,
  `SpinLock`, IRQ-masking, GICv2/IPI, and Pi 5 remote wake-request evidence.
- Specified that a target CPU may consume remote requests outside IPI context
  and transition only its own local blocked task to runnable state.
- Named lock ordering, bounded queue behavior, duplicate handling, task-state
  preconditions, and diagnostic evidence required for the first implementation.
- Selected the next QEMU-only implementation proof boundary.

## Evidence

- Static source review: `src/scheduler.rs`, `src/smp_sync.rs`,
  `src/arch/aarch64/gicv2.rs`, `src/target/qemu_virt.rs`, and
  `src/target/rpi5.rs`.
- Accepted evidence review:
  `tasks/2026-05-25-phase6-qemu-remote-wakeup-request-smoke.md`,
  `tasks/2026-05-25-phase6-pi5-remote-wakeup-request-proof.md`, and
  `tasks/evidence/2026-05-25-pi5-remote-wakeup-request-proof/summary.md`.
- Documentation updates:
  `docs/src/project/phase6-target-owned-wake-consumption-contract.md`,
  `docs/src/architecture/scheduler.md`, `docs/src/roadmap.md`,
  `docs/src/decisions/README.md`, and `docs/src/SUMMARY.md`.

## Contract Summary

A remote CPU may publish or coalesce a bounded request and signal with SGI
INTID 1, but it must not mutate another CPU's runnable queue. The target IPI
handler acknowledges/classifies/records/EOIs only. Outside IPI context, the
target CPU drains its owned request queue and may transition exactly one
target-owned diagnostic task from `Blocked` to `Runnable` under local
scheduler rules.

The first implementation proof must make wrong-owner, unknown-task,
already-runnable, duplicate-local-enqueue, queue-full, self-target, and
production-secondary-dispatch-deferred outcomes explicit instead of silently
mutating scheduler state.

## Validation

- git status --short before edits: clean.
- static review: completed over scheduler, synchronization, SGI/IPI, accepted
  remote wake-request proof records, architecture docs, roadmap, and decision
  log.
- git diff --check: passed.
- mdbook build: not run because `mdbook` is unavailable in the container.
- Rust fmt/tests and hardware runs were not required because this task changed
  only Markdown documentation and durable task state.

## Acceptance

Accepted as the target-owned wake-consumption contract. The next bounded task
should be a QEMU-only target-owned wake-consumption proof with
blocked-to-runnable local transition evidence, duplicate coalescing, cross-owner
rejection, drained queues, and no production secondary dispatch. Pi 5 hardware
proof, shared run queues, global task lookup, remote enqueue queues, task
migration, load balancing, production secondary scheduler dispatch, multi-core
preemption, Phase 7, filesystem, networking, SSH, shell behavior, RP1/PCIe,
UART interrupt ownership, and DMA/cache-coherent driver policy remain deferred.
