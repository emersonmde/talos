# Phase 6 Target-Owned Wake Consumption Contract

Status: accepted as the Milestone 6.3 contract for converting consumed remote
wake requests into local scheduler wake actions.

This checkpoint follows the accepted QEMU and Pi 5 remote wake-request proofs.
Those proofs establish request publication, duplicate coalescing, SGI INTID 1
signaling, target-side observation/EOI, target-owned request consumption,
drained queues, and rejected cross-owner scheduler mutation. They do not yet
accept a local runnable transition from a remote request.

## Source Inventory

- `src/scheduler.rs`: `RemoteWakeQueue` is a bounded target-owned mailbox
  keyed by scheduler-local `TaskId`. `publish()` rejects wrong-target and
  self-target requests, coalesces duplicate task IDs, and rejects overflow.
  `consume_next()` rejects non-owner consumers. `PerCoreScheduler` keeps
  local scheduler state owned by one `LogicalCpuId`; cross-owner local
  mutation is rejected, and secondary production dispatch remains deferred.
- `src/smp_sync.rs`: `SpinLock<T>`, `lock_irqsave()`, and
  `smp_full_barrier()` provide the accepted SMP synchronization pieces used
  by remote wake-request diagnostics.
- `src/arch/aarch64/gicv2.rs` and target SGI diagnostics: SGI INTID 1 is the
  accepted raw IPI signal. QEMU and Pi 5 proofs both route request notification
  through the same target-list signal class.
- Accepted records:
  `tasks/2026-05-25-phase6-qemu-remote-wakeup-request-smoke.md`,
  `tasks/2026-05-25-phase6-pi5-remote-wakeup-request-proof.md`, and
  `tasks/evidence/2026-05-25-pi5-remote-wakeup-request-proof/summary.md`.

## Ownership Contract

A remote CPU may publish a bounded wake request for a scheduler-local task ID
into the target CPU's `RemoteWakeQueue` and then signal that target with SGI
INTID 1. The remote CPU must not mutate the target CPU's `RunnableQueue`,
`current_task`, or task state directly.

The target CPU owns request consumption and local scheduler effects. The target
IPI handler may only acknowledge/classify the SGI, record bounded wake-pending
evidence, EOI, and return. A later target-owned drain service runs outside IPI
context, consumes or snapshots requests from the target's queue, and then
applies local scheduler rules.

The first accepted wake action is intentionally narrow: if the consumed request
names a target-owned diagnostic task that is currently `Blocked`, the target
may transition that one task to `Runnable` on its local scheduler. Requests
for running tasks, already-runnable tasks, unknown tasks, nonlocal tasks,
wrong-owner queues, full queues, invalid task IDs, and self-targeted wakeups
must be explicit outcomes and must not enqueue a task silently.

Duplicate pending remote requests remain coalesced by `RemoteWakeQueue`.
After one request is consumed, duplicate local enqueue must still be rejected
unless the target-owned task has returned to a blocked state through a separate
accepted scheduler path.

## Lock And Context Ordering

- Sender side: mask local IRQs, acquire the target wake-request lock, insert
  or coalesce the request, release the lock, restore local IRQ state, publish
  an ordering barrier if needed, then send SGI INTID 1.
- Target IPI side: acknowledge/classify/record/EOI only; no scheduler queue
  mutation and no wake-request draining.
- Target drain side: run outside IPI context, mask local IRQs, acquire the
  owned wake-request lock, drain or snapshot bounded requests, release the
  lock, restore local IRQ state, then enter the target-owned local scheduler
  mutation boundary for any blocked-to-runnable transition.
- Wake-request locks and local scheduler mutation windows must not be held
  across `talos_aarch64_context_switch`, printing, UART polling, diagnostic
  command dispatch, allocation, blocking, sleeping, migration, or arbitrary
  callbacks.

## Next Implementation Boundary

The next bounded implementation should be QEMU-only. It should prove:

- CPU 0 publishes remote wake requests for diagnostic tasks owned by logical
  CPUs 1, 2, and 3 and coalesces a duplicate request for one target.
- Targets observe and EOI SGI INTID 1, then drain their request queues outside
  IPI context.
- Each target transitions exactly one local blocked task to runnable and
  reports the local runnable queue state.
- Cross-owner scheduler mutation remains rejected, queue length is zero after
  drain, duplicate local enqueue is rejected, and production secondary
  dispatch remains deferred.

Pi 5 hardware proof, shared run queues, global task lookup, remote enqueue
queues, task migration, load balancing, work stealing, production secondary
scheduler dispatch, multi-core preemption, Phase 7, filesystem, networking,
SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and
DMA/cache-coherent driver policy remain deferred.

## Validation

- static inspection: `git status --short` was clean before documentation
  edits.
- static review: inspected scheduler, remote wake queue, per-core scheduler
  ownership, SMP synchronization, GICv2 SGI paths, accepted QEMU/Pi 5 remote
  wake-request task records and evidence, architecture docs, roadmap, and
  decision log.
- whitespace inspection: `git diff --check` passed after edits.
- static inspection: `mdbook` was unavailable in the container, so mdBook
  build was not run.
- Rust fmt/tests and hardware runs were not required because this checkpoint
  changes only Markdown documentation and durable task state.
