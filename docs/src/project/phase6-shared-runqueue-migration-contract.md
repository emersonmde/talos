# Phase 6 Shared Run-Queue and Migration Contract

Status: accepted as the Phase 6.3 contract for the first shared run-queue and
task-migration implementation slice. This document changes architecture and
validation policy only. It does not add Rust behavior, boot images, QEMU
claims, Pi 5 hardware claims, load balancing, work stealing, multi-core timer
preemption, Phase 7, filesystem, networking, SSH, shell behavior, RP1/PCIe,
UART interrupt ownership, or DMA/cache-driver policy.

This contract follows the accepted shared run-queue and migration source
inventory. It preserves the current owner-local scheduler model while defining
the minimum authority, locking, state transition, and diagnostic boundaries
needed before implementation can introduce a shared run queue.

## Ownership Model

The first shared topology remains CPU-owned, not globally scheduled by an
anonymous singleton:

- A task has one owner CPU at a time. The owner is the only CPU allowed to
  mutate the task's saved context, running/current state, blocked state, and
  owner-local diagnostic counters.
- A shared run queue is a named transfer surface for runnable tasks whose
  owner is changing or whose target owner has been selected. It is not a
  replacement for each CPU's local scheduler invariants.
- A remote CPU may publish enqueue or migration intent only through an
  accepted shared structure protected by the shared scheduler lock hierarchy.
  It may not directly mutate another CPU's RunnableQueue or current task.
- The destination owner consumes a shared entry from normal scheduler control
  flow and performs the destination-local enqueue. That local enqueue is the
  moment the destination becomes the task's mutation authority.
- RemoteWakeQueue remains a wake-request mailbox for already target-owned
  blocked tasks. It is not reused as the remote enqueue or migration queue.
- SharedSchedulerMetadata remains observational. Metadata may help choose a
  target or reject stale decisions, but it does not itself move ownership or
  grant mutation authority.

This keeps the first implementation compatible with later load balancing:
policy may choose candidates later, but the mechanism must already prevent
cross-owner queue mutation and stale ownership handoff.

## Lock And IRQ Contract

The lock order for the first shared scheduler topology is:

1. save and mask local IRQ state for the short scheduler mutation window;
2. acquire the shared scheduler lock that protects the shared run queue or
   migration handoff state;
3. release the shared scheduler lock;
4. restore the saved local IRQ state.

The accepted SpinLock lock_irqsave shape in src/smp_sync.rs is the model for
this ordering on AArch64. Code may use separate explicit calls only if it
preserves the same order. Acquiring an SMP scheduler lock and then masking IRQs
is not accepted for scheduler topology work because it can invert the local
interrupt exclusion rule.

No scheduler lock may be held across:

- context switch or trampoline entry;
- UART polling, printing, formatting, or diagnostic command dispatch;
- allocation, blocking, sleeping, filesystem work, descriptor work, syscall
  work, or arbitrary callbacks;
- IPI send loops or timer reprogramming loops;
- hardware lab controller calls or proof-script waits.

Local RunnableQueue mutation remains owner-local. The first shared queue
implementation must not hide local queue mutation inside a global lock if the
target owner has not yet accepted the task. When both shared and local state
must change, the implementation should split the operation into a shared
handoff phase and a destination-local consume phase rather than holding a
shared lock while mutating another CPU's local queue.

## Memory Ordering

The accepted spinlock acquire/release ordering is the primary publication
boundary for shared scheduler structures:

- Producers publish a complete shared run-queue or migration entry before
  releasing the shared lock.
- Consumers acquire the same lock before reading and removing an entry.
- smp_full_barrier is reserved for explicit handoff points where the contract
  needs a named full-system ordering edge outside the lock's ordinary
  acquire/release pair.
- Owner metadata refresh happens after the owner-local mutation it describes.
  Metadata readers must treat generation mismatches or unknown tasks as stale
  observations, not as permission to repair state remotely.

The first implementation should prefer one shared lock around the bounded
handoff queue over ad hoc atomics. If later work splits locks for scalability,
that task must update this contract with a new lock order before code changes.

## Migration State Machine

The first migration mechanism must be explicit and rollback-friendly. The
accepted states are:

- OwnerLocal: the task is owned by one PerCoreScheduler; only that owner may
  mutate task state and local queue membership.
- MigrationReserved: the source owner has selected the task and recorded a
  destination while the task is not running. The task must not be on two local
  runnable queues.
- SharedQueued: a complete handoff entry is visible in the shared queue. The
  source no longer re-dispatches the task as locally runnable.
- DestinationEnqueued: the destination owner consumed the entry from normal
  scheduler control flow, installed the task into its local runnable queue,
  and became the owner.
- MigrationRejected: the destination could not accept the entry because of
  stale generation, duplicate ownership, invalid target, full local queue, or
  unsupported state. The task returns to a single owner-local state through an
  explicit failure path.

The first implementation may encode these states as concrete Rust enums or as
documented results around existing types, but the behavior must be observable
in tests. A task cannot be Running while entering MigrationReserved; running
task migration remains deferred until multi-core preemption and asynchronous
context capture are separately accepted.

## Enqueue And Wake Boundaries

Remote enqueue and remote wake are separate operations:

- Remote wake targets a blocked task already owned by the target CPU. It uses
  RemoteWakeQueue, then the target owner turns that blocked task into local
  runnable state.
- Remote enqueue publishes runnable work for destination-owned consumption
  through the shared run queue. It requires ownership handoff or an already
  destination-owned task.
- Migration is owner transfer. It must remove the task from source-local
  runnable membership before destination-local enqueue becomes visible.
- Remote reschedule is only a notification that a CPU should run its normal
  scheduler service loop. It must not run the scheduler in IPI context.

IPI handlers remain bounded recorders: acknowledge, classify, record pending
state, EOI, and return. They may not acquire scheduler topology locks for
unbounded queue work, walk task tables, dispatch tasks, or print diagnostics.

## Failure And Diagnostics

The shared run-queue implementation must report deterministic outcomes for:

- accepted enqueue or migration handoff;
- duplicate local or shared queue membership;
- stale metadata generation;
- unknown task ID;
- wrong source owner or wrong destination owner;
- invalid logical CPU;
- full shared queue;
- full destination local queue;
- running, blocked, or otherwise unsupported migration source state;
- deferred secondary role if a proof tries to use a non-accepted runtime role.

Diagnostic proof surfaces may expose counters, final queue snapshots, owner
metadata, and classifications, but they remain validation gates. They are not
stable kernel interfaces and must not become the only way production scheduler
state can move.

## Deferrals

This contract does not accept:

- load-balancing policy, work stealing, fairness, affinity, or CPU selection
  heuristics;
- migration of currently running tasks;
- multi-core timer preemption or asynchronous cross-core context switching;
- a general non-diagnostic secondary runtime role beyond accepted service-loop
  proof entry;
- Phase 7 process, EL0, syscall, descriptor, or POSIX behavior;
- filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt, or
  DMA/cache-driver behavior.

The next bounded implementation may add the shared run-queue core and tests if
it stays inside this contract. QEMU and Pi 5 proof tasks remain separate
supervisor-planned slices.

## Validation

- static inspection: reviewed the accepted shared run-queue/migration source
  inventory, docs/src/architecture/scheduler.md, src/scheduler.rs,
  src/smp_sync.rs, docs/src/roadmap.md, and docs/src/decisions/README.md.
- whitespace inspection: git diff --check must pass before acceptance.
- documentation: mdbook build must pass before acceptance.
- Rust fmt/tests, QEMU reruns, and hardware runs are not required because this
  task changes only Markdown documentation and durable task state.
