# Phase 6 CPU-Local Scheduler Service Boundary Source Inventory

Status: accepted as a Phase 6.3 source inventory and contract for the
CPU-local scheduler service boundary. No Rust implementation, boot image,
hardware run, shared run queue, remote enqueue queue, task migration, load
balancing, work stealing, multi-core preemption, Phase 7, filesystem,
networking, SSH, shell, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy is added by this inventory.

This inventory reconciles the accepted timer-preemption, remote wake,
target-owned local runnable, production secondary dispatch, and shared
scheduler metadata slices into one normal-control-flow service order. It is the
boundary that should be implemented before Talos starts broader shared
scheduler topology work.

## Source Inventory

- src/scheduler.rs owns the target-independent scheduler data model:
  SingleCoreScheduler, PerCoreScheduler, RemoteWakeQueue, target-owned
  blocked-to-runnable wake consumption, production diagnostic secondary
  dispatch, and SharedSchedulerMetadata.
- docs/src/architecture/scheduler.md records the accepted rule that local
  runnable queues remain CPU-local. Remote CPUs may publish bounded wake
  requests or inspect owner-published metadata, but they may not mutate another
  CPU's PerCoreScheduler or RunnableQueue.
- src/smp.rs owns secondary CPU lifecycle, stack ownership, logical identity,
  and controlled secondary handoff. It does not provide task migration, load
  balancing, IPIs, or scheduler queues.
- src/smp_sync.rs owns the accepted SpinLock&lt;T&gt;, IRQ-save lock guard, and
  dmb ish barrier primitives. The scheduler uses those primitives only at
  named shared boundaries; CPU-local runnable queues do not become shared by
  virtue of the lock existing.
- The GIC/target code and focused QEMU/Pi 5 proofs retain SGI/IPI delivery as
  a signal path. IPI context acknowledges, classifies, records bounded
  evidence, EOIs, and returns. It must not run the scheduler service.
- The accepted shared metadata table is an owner-published read surface. It is
  not a global mutable task registry, remote enqueue authority, or scheduler
  topology.

## Service Ordering

The production CPU-local scheduler service should run from normal kernel
control flow on the owning logical CPU. It may be entered after a voluntary
yield, after a timer interrupt records a local preemption request, after an
IPI records remote-wake pending state, or from a secondary diagnostic
production dispatch loop. The service order is:

1. Establish the requester as the current logical CPU and operate only on that
   CPU's PerCoreScheduler, local task table, and target-owned RemoteWakeQueue.
2. Drain target-owned remote wake requests outside IPI context. Each consumed
   request must be for this CPU and must identify a local task owned by this
   CPU.
3. Apply local runnable transitions for consumed wake requests. The only
   accepted remote-wake scheduler effect is a target-owned Blocked to Runnable
   transition through local scheduler rules, with mismatched task IDs,
   duplicate local runnable entries, non-blocked tasks, wrong owners, and full
   local queues reported as explicit errors.
4. Handle the pending local timer-preemption request, if one exists. The timer
   IRQ may record the request, but switching remains outside asynchronous
   exception context. Draining wake requests first lets a just-woken local task
   participate in the preemption decision.
5. Enter CPU-local dispatch only through the owner scheduler. The boot CPU may
   use the accepted production path; secondaries remain limited to the accepted
   SecondaryProductionDiagnostic role until a later task replaces the
   diagnostic entry with a general production secondary service.
6. Refresh owner-published scheduler metadata after local task state,
   current-task, runnable-queue, or dispatch-counter mutations. Only the owner
   CPU may publish or refresh snapshots for its local scheduler state.
7. Return without holding scheduler locks across talos_aarch64_context_switch,
   printing, UART polling, diagnostic command dispatch, allocation, blocking,
   sleeping, migration, or arbitrary callbacks.

This order intentionally drains remote wake requests before servicing a local
timer-preemption dispatch. The invariant is that wake delivery becomes local
runnable state before the scheduler decides whether the current task should
continue or switch to another local runnable task.

## Diagnostic Gates Versus Runtime Behavior

Retained validation gates for this boundary:

- cargo fmt --all -- --check and cargo -Zjson-target-spec test when Rust
  implementation changes the scheduler service or supporting data structures.
- scripts/qemu-timer-preemption-smoke.sh for the accepted local
  timer-preemption request/dispatch evidence.
- scripts/qemu-remote-wake-to-local-runnable-smoke.sh for target-owned remote
  wake drains and local blocked-to-runnable transitions.
- scripts/qemu-production-secondary-dispatch-smoke.sh for CPU-local secondary
  diagnostic dispatch.
- scripts/qemu-shared-scheduler-metadata-smoke.sh for owner-published metadata
  refresh/lookup invariants.
- The focused Pi 5 remote wake, production secondary dispatch, and shared
  scheduler metadata proof scripts remain physical validation surfaces when a
  later task makes physical scheduler claims.

Those gates prove bounded slices. Productized runtime behavior still requires
a single service implementation that orders the slices without relying on test
flags, proof-only entry points, or ad hoc diagnostic sequencing.

## Recommended Follow-Up

The next bounded task should be
phase6-cpu-local-scheduler-service-core-20260526.

That task should implement a target-independent CPU-local scheduler service
adapter that sequences target-owned remote wake drains, local runnable
transitions, pending timer-preemption handling, CPU-local dispatch, and
owner-published metadata refresh for one owning logical CPU. It should include
unit tests and a QEMU-only smoke that proves the service order with local
diagnostic tasks.

Non-goals for that follow-up: shared run queues, remote enqueue queues, task
migration, load balancing, work stealing, multi-core preemption, Phase 7,
filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt
ownership, and DMA/cache-driver policy.

## Deferred Work

The following remain explicitly deferred:

- global task registry mutation authority, remote enqueue queues, shared run
  queues, migration, load balancing, work stealing, and remote reschedule;
- multi-core preemption policy, secondary timer-preemption policy, sleep
  queues, wait queues, blocking I/O readiness, and production task movement
  beyond CPU-local service order;
- descriptor-facing TTY behavior, userspace, EL0, syscalls, descriptor tables,
  file descriptors, user/kernel copy policy, process address spaces, and
  process lifetime rules;
- filesystem behavior, program loading, libc/Rust std support, portable
  userland, and local shell behavior;
- RP1/PCIe ownership, DMA, cache-coherent DMA driver policy, networking, SSH,
  Ethernet, and shell access.

## Validation

- static inspection: git status --short was clean before edits.
- static source/doc review: inspected scheduler, SMP, SMP sync, wake/IPI,
  shared metadata, roadmap, decision log, accepted closeouts, and accepted task
  records.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests and hardware runs were not required because this task changes
  only Markdown documentation and durable task state.
