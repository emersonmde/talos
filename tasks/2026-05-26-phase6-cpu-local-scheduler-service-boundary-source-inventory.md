# Phase 6 CPU-Local Scheduler Service Boundary Source Inventory

Status: accepted.

Task id: phase6-cpu-local-scheduler-service-boundary-source-inventory-20260526

## Goal

Define the production CPU-local scheduler service boundary that orders accepted
timer, remote wake, local runnable, secondary dispatch, and shared metadata
slices before shared queues or migration are implemented.

## Scope

- Reviewed scheduler/preemption, PerCoreScheduler, RemoteWakeQueue,
  SharedSchedulerMetadata, SMP sync, IPI, and retained diagnostic proof
  boundaries.
- Defined normal-control-flow service ordering for timer-preemption request
  handling, target-owned remote wake drains, local runnable transitions,
  production secondary dispatch entry, and owner metadata refresh.
- Named accepted QEMU/Pi 5 gates that remain validation surfaces for this
  boundary.
- Recommended the smallest follow-up implementation task.

## Non-Goals

- No Rust implementation in this inventory task.
- No shared run queues, remote enqueue queues, task migration, load balancing,
  work stealing, or multi-core preemption.
- No Phase 7, userspace, filesystem, networking, SSH, shell, RP1/PCIe, UART
  interrupt ownership, or DMA/cache-driver policy.

## Evidence

- Service-ordering inventory:
  docs/src/project/phase6-cpu-local-scheduler-service-boundary-source-inventory.md.
- Architecture update: docs/src/architecture/scheduler.md.
- Roadmap update: docs/src/roadmap.md.
- Decision-log update: docs/src/decisions/README.md.

## Service-Ordering Result

The accepted service order is:

1. Enter the service from normal kernel control flow on the owning logical CPU.
2. Drain target-owned remote wake requests outside IPI context.
3. Convert matching local blocked tasks to local runnable state through
   target-owned scheduler rules.
4. Handle pending local timer-preemption requests after remote wake drains so
   newly woken local tasks can participate in the dispatch decision.
5. Dispatch only through the owner scheduler, with secondary CPUs still limited
   to the accepted SecondaryProductionDiagnostic role.
6. Refresh owner-published metadata after local task state, current-task,
   queue, or dispatch-counter mutations.
7. Return without holding scheduler locks across context switch, printing,
   UART polling, diagnostic command dispatch, allocation, blocking, sleeping,
   migration, or arbitrary callbacks.

## Recommended Follow-Up

Queue phase6-cpu-local-scheduler-service-core-20260526 as the next bounded
implementation/refactor task. It should implement a target-independent
CPU-local scheduler service adapter and QEMU-only smoke for the accepted order.
It must not implement shared run queues, migration, load balancing,
multi-core preemption, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe,
UART interrupt ownership, or DMA/cache policy.

## Validation

- static inspection: git status --short was clean before edits.
- static source/doc review: inspected scheduler, SMP sync, IPI/wakeup,
  metadata, roadmap, decisions, accepted closeouts, and accepted task records.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests and hardware runs were not required because the task changed
  only Markdown documentation and durable task state.
