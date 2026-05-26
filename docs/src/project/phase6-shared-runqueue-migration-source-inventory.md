# Phase 6 Shared Run-Queue and Migration Source Inventory

Status: accepted as a Phase 6.3 source inventory for the shared run-queue and
task-migration boundary. No Rust implementation, boot image, hardware run,
shared run queue, remote enqueue queue, task migration, load balancing,
multi-core preemption, Phase 7, filesystem, networking, SSH, shell,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy is added by
this inventory.

This inventory follows the accepted secondary scheduler service-loop closeout.
It reconciles the current owner-local scheduler model, target-owned wake
mailboxes, owner-published metadata, SMP locks, and retained diagnostic proof
surfaces before Talos designs shared run queues or task migration.

## Source Inventory

- `src/scheduler.rs` owns scheduler-local `TaskId`, `TaskState`,
  `KernelStack`, `ContextFrame`, `Task`, `RunnableQueue`,
  `SingleCoreScheduler`, `PerCoreScheduler`, `SchedulerCoreRole`,
  `RemoteWakeQueue`, `CpuLocalSchedulerService`,
  `SecondarySchedulerServiceLoop`, `SharedSchedulerMetadata`, and
  `SharedSchedulerMetadataLock`.
- `RunnableQueue` is a fixed-capacity FIFO embedded inside
  `SingleCoreScheduler`. It has no lock, no remote producer, no task-stealing
  API, and no notion of a CPU other than its owning `PerCoreScheduler`.
- `PerCoreScheduler` is the local mutation authority for current-task state,
  runnable queue state, local blocked-to-runnable transitions, and diagnostic
  production dispatch. `ensure_local_owner` and `ensure_production_owner`
  reject cross-owner mutation.
- `SchedulerCoreRole::BootCpuProduction` and
  `SchedulerCoreRole::SecondaryProductionDiagnostic` are the only roles that
  pass production dispatch. `SecondaryDeferred` is still an explicit rejection
  boundary, not an idle production role.
- `RemoteWakeQueue` is a target-owned bounded mailbox. A remote CPU may publish
  wake intent for the target owner, but only the target owner may consume it
  and turn a matching local blocked task into local runnable state through
  `PerCoreScheduler::wake_blocked_local_task_from_remote_request`.
- `CpuLocalSchedulerService::run_cycle` sequences one owner CPU's local work:
  consume at most one target-owned wake request, handle an already-recorded
  timer-preemption request, run owner-local diagnostic dispatch when requested,
  and refresh owner-published metadata.
- `SecondarySchedulerServiceLoop::run_once` is the accepted normal-control-flow
  secondary adapter around the CPU-local service. It rejects boot-CPU use,
  cross-owner service, and deferred secondary roles before one owner-local
  cycle.
- `SharedSchedulerMetadata` is a bounded read-oriented table of owner-published
  snapshots. Its registration and refresh paths require the requester to be the
  owner. It is not a global mutable task registry and cannot move task
  ownership.
- `src/smp_sync.rs` owns the accepted `SpinLock<T>`, `lock_irqsave()` ordering,
  and `smp_full_barrier()` primitive. These can protect named shared
  structures, but today only shared metadata and diagnostic/proof state use
  that boundary. Local runnable queues remain outside any shared scheduler
  lock.
- `src/smp.rs` owns secondary-core lifecycle, logical identity, stack slots,
  and handoff readiness. It does not own scheduler queues or migration policy.
- `src/target/qemu_virt.rs` and `src/target/rpi5.rs` retain proof-only routing
  for raw SGI delivery, remote wake publication/consumption, production
  secondary dispatch, shared metadata, and secondary service-loop execution.
  These are diagnostic gates, not production scheduler topology.
- `scripts/qemu-remote-wake-to-local-runnable-smoke.sh`,
  `scripts/qemu-production-secondary-dispatch-smoke.sh`,
  `scripts/qemu-shared-scheduler-metadata-smoke.sh`,
  `scripts/qemu-secondary-scheduler-service-loop-smoke.sh`, and the matching
  Pi 5 image/boot-tree scripts remain validation surfaces for bounded slices.

## Owner-Local Assumptions

The current scheduler assumes that each runnable queue has exactly one owner.
Task state changes, current-task updates, dispatch counters, local runnable
membership, and metadata refreshes are performed by that owner from normal
control flow. Interrupt paths may record bounded pending work and return, but
they do not run the scheduler, walk queues, dispatch tasks, migrate work, or
refresh metadata.

Remote wake is not remote enqueue. The accepted path lets a remote CPU request
that the target owner wake a specific blocked local task. The target owner
consumes the request and mutates its own local scheduler. There is no accepted
path for a remote CPU to place arbitrary work directly onto another CPU's
runnable queue.

Owner-published metadata is observational. It lets later code see task owner,
state, current-on-owner, runnable-on-owner, process-owner placeholder, kernel
stack, and generation snapshots. It does not authorize task ownership changes,
select victims for migration, or provide a consistency model for load
balancing decisions.

The accepted secondary service loop proves one seeded owner-local cycle per
secondary. It does not yet define continuous idle behavior, production
secondary roles beyond the diagnostic gate, multi-core timer-preemption
policy, or cross-CPU task movement.

## Migration Blockers

- There is no global task registry that can locate and mutate a task by
  `TaskId` across CPU owners. `SharedSchedulerMetadata` can detect stale
  snapshots, but it cannot acquire the task object or change its owner.
- There is no shared run-queue data structure, lock ordering, or memory-order
  contract for concurrent producers and consumers.
- `RunnableQueue` membership is local to `SingleCoreScheduler`; removing a
  task from one CPU and installing it on another has no accepted API.
- `Task` ownership is implicit in the containing `PerCoreScheduler` and
  metadata snapshot. There is no two-phase owner transfer, generation bump, or
  rollback path for failed migration.
- Remote wake queues carry wake requests for already target-owned tasks. They
  cannot represent remote enqueue, work stealing, migration acknowledgement, or
  remote reschedule.
- Secondary production dispatch is still gated by
  `SecondaryProductionDiagnostic`; there is no general secondary production
  runtime role or durable idle/wake loop.
- Timer-preemption policy is CPU-local. There is no accepted cross-core
  preemption, remote reschedule IPI, or load-balancing trigger.
- Locks are available as primitives, but the scheduler has no accepted
  hierarchy for local queue locks, global task registry locks, metadata locks,
  remote wake locks, and interrupt masking.
- Diagnostic proof scripts seed local tasks directly and report classifications.
  They do not prove a production path for arbitrary migrated tasks, fairness,
  backpressure, or failure recovery.

## Reusable Pieces

Reusable for the next contract:

- `TaskId`, `LogicalCpuId`, `TaskState`, `KernelStack`, and `ProcessOwnerId`
  placeholders provide the vocabulary for task ownership and later process
  integration.
- `PerCoreScheduler` gives a clear owner-local authority boundary that a
  migration contract can preserve or wrap.
- `RemoteWakeQueue` separates remote notification from target-local scheduler
  mutation; that distinction should be kept when remote enqueue is introduced.
- `SharedSchedulerMetadata` and generation checks provide an initial stale
  observation model for planning migration decisions.
- `SpinLock<T>` plus `lock_irqsave()` give the primitive needed for named
  shared structures once their lock order is documented.
- The accepted QEMU/Pi 5 proof scripts can remain regression gates for local
  wake, metadata, and service-loop invariants while shared topology is added
  in smaller tasks.

Requires new contract before implementation:

- global task registry identity, lookup, and mutation authority;
- shared run-queue structure and lock placement;
- migration state machine, including source-owner removal, destination enqueue,
  metadata update, and failure rollback;
- remote enqueue and remote reschedule semantics distinct from remote wake;
- lock-ordering rules across local queues, global task registry, metadata,
  remote wake queues, and interrupt masking;
- load-balancing policy inputs and stale-metadata handling;
- secondary production role and idle/wake behavior;
- validation plan for QEMU substitute and serialized Pi 5 physical evidence.

## Recommended Follow-Up

The next bounded task should be
`phase6-shared-runqueue-migration-contract-20260526`.

That task should write the contract for shared run queues, global task registry
authority, task migration, remote enqueue/reschedule boundaries, and lock
ordering. It should remain documentation/contract-first unless the supervisor
explicitly narrows an implementation slice. It must not implement a shared run
queue, migrate tasks, add load balancing, add multi-core preemption, start
Phase 7, or touch filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver policy.

## Validation

- static inspection: `git status --short` was clean before edits.
- rg/static source inventory: inspected scheduler, SMP, SMP sync, QEMU/Pi 5
  proof routing, retained scripts, scheduler architecture docs, roadmap,
  decision log, accepted task records, and accepted evidence summaries.
- whitespace inspection: `git diff --check` passed.
- documentation: `mdbook build` passed.
- Rust fmt/tests, QEMU reruns, and hardware runs were not required because
  this task changes only Markdown documentation and durable task state.
