# Phase 6 Load-Balancing Source Inventory

Status: accepted as a Phase 6.3 source inventory for load-balancing policy.
No Rust implementation, QEMU run, Pi 5 hardware run, load balancer, work
stealing, running-task migration, remote reschedule, multi-core preemption,
Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt, or
DMA/cache-driver behavior is added by this inventory.

This inventory follows the accepted shared run-queue/migration closeout. It
identifies the scheduler, metadata, wake, timer, and diagnostic surfaces Talos
can use before it designs policy for choosing a migration target.

## Source Inventory

- src/scheduler.rs owns the current scheduler vocabulary:
  LogicalCpuId, TaskId, TaskState, Task, RunnableQueue,
  SingleCoreScheduler, PerCoreScheduler, SchedulerCoreRole,
  RemoteWakeQueue, CpuLocalSchedulerService,
  SecondarySchedulerServiceLoop, SharedSchedulerMetadata, and
  SharedRunQueue.
- RunnableQueue is still fixed-capacity and owner-local. It can enqueue,
  dequeue, check membership, and remove a runnable task for accepted migration,
  but it has no remote producer, no steal API, no per-task affinity, and no
  age/fairness accounting.
- PerCoreScheduler is the mutation authority for a CPU's local current task,
  local runnable queue, local blocked-to-runnable wake consumption, and
  diagnostic dispatch. Cross-owner local mutation is rejected.
- SchedulerCoreRole separates production boot CPU dispatch, deferred
  secondaries, and the explicit SecondaryProductionDiagnostic role used by
  proof surfaces. There is no non-diagnostic secondary runtime role yet.
- RemoteWakeQueue is a bounded target-owned wake mailbox. It can signal that
  a target should wake an already target-owned blocked task; it is not remote
  enqueue, migration acknowledgement, load-balancer notification, or remote
  reschedule.
- CpuLocalSchedulerService::run_cycle sequences one owner-local cycle:
  consume at most one remote wake request, handle a pending timer-preemption
  request, optionally dispatch one local diagnostic task, and refresh metadata.
  It is the current normal-control-flow point where a later owner-local policy
  could observe pending local work.
- SecondarySchedulerServiceLoop::run_once is a single-cycle adapter for
  explicit diagnostic secondary owners. It proves the service shape but does
  not define a continuous idle loop, parking protocol, or production wake path.
- SharedSchedulerMetadata publishes owner, task state, process-owner
  placeholder, stack bounds, current-on-owner, runnable-on-owner, and
  generation snapshots. It can reject unknown or stale observations; it is not
  a global mutable task registry and cannot by itself move work.
- SharedRunQueue is the accepted owner-transfer surface. A source owner can
  publish a runnable task to a destination owner after fresh metadata checks;
  the destination owner can consume that entry into its local runnable queue
  and transfer the metadata owner. It does not choose a destination.
- src/smp_sync.rs provides SpinLock, AArch64 lock_irqsave(), and the
  full barrier primitive. The accepted scheduler rule remains local IRQ save
  before SMP scheduler lock acquisition, with no lock held across context
  switching, printing, allocation, blocking, timer loops, IPI loops, or lab
  waits.
- src/smp.rs publishes secondary-core lifecycle, identity, stack ownership,
  and workload progress. It does not own scheduler policy, queue length, or
  balancing state.
- src/target/qemu_virt.rs, src/target/rpi5.rs, and the retained Phase 6
  scripts provide diagnostic gates for raw SGIs, remote wake, production
  secondary dispatch, shared metadata, secondary service-loop execution, and
  shared run-queue/migration. They are proof surfaces, not production policy.

## Available Policy Inputs

- CPU identity and role: LogicalCpuId and SchedulerCoreRole show which CPU
  owns a scheduler and whether production dispatch is currently accepted.
- Local runnable pressure: RunnableQueue::len, is_empty, is_full, and front
  expose small fixed-capacity queue state for an owner, but only while
  respecting owner-local mutation rules.
- Local current-task state: PerCoreScheduler::current_task and metadata
  current_on_owner identify running work that must not be migrated by the
  current shared run-queue core.
- Task state and membership: TaskState, metadata runnable_on_owner, and local
  queue membership can distinguish runnable migration candidates from running,
  blocked, unknown, duplicate, or stale tasks.
- Metadata freshness: SharedSchedulerMetadata generations and stale-snapshot
  errors are the only accepted staleness signal for cross-core observation.
- Shared run-queue capacity and backpressure: SharedRunQueue::capacity, len,
  is_full, duplicate checks, and deterministic errors identify when a policy
  must defer rather than publish more migration work.
- Wake state: RemoteWakeQueue::len, duplicate count, and consume errors show
  target-owned wake pressure for already target-owned tasks, but not a remote
  enqueue or migration-completion signal.
- Timer-preemption state: pending_timer_preemption in
  CpuLocalSchedulerService::run_cycle is CPU-local. It does not yet provide a
  cross-core balancing trigger or remote preemption mechanism.
- Hardware participation evidence: accepted QEMU and Pi 5 diagnostics prove
  that all logical/physical cores can participate in seeded invariants, but
  they do not provide production load metrics or fairness history.

## Failure Modes To Handle Before Implementation

- Stale metadata: a policy may choose from an old snapshot and must re-check
  generation before publish. Stale input should produce a deterministic defer
  or retry, not a forced migration.
- Running-task candidate: the accepted shared queue rejects running/current
  tasks. Load balancing must begin with runnable-only movement unless a later
  multi-core preemption task designs async capture.
- Blocked or unknown task: blocked tasks are handled by wake paths, not
  migration; unknown tasks indicate missing registration or stale policy data.
- Full destination queue: a destination with no local queue capacity must not
  receive another migrated task.
- Full shared queue or duplicate shared entry: policy must preserve bounded
  backpressure and avoid unbounded retries.
- Deferred secondary role: SecondaryDeferred cannot consume production work. A
  policy must not target a CPU until its runtime role is explicitly accepted.
- Cross-owner mutation attempt: source selection, publication, and destination
  consumption must keep local scheduler mutation on the owning CPU.
- Remote wake confusion: wake requests for target-owned blocked tasks must not
  be reused as load-balancing enqueue, migration acknowledgement, or remote
  reschedule.
- Timer/preemption confusion: local pending preemption is not an accepted
  balancing interrupt, and no secondary exception-frame switching exists yet.
- Diagnostic evidence overreach: retained scripts prove named invariants with
  seeded work. They do not prove fairness, affinity, continuous balancing, or
  production secondary idle/wake behavior.

## Boundary Split For The Next Contract

Target selection should be a policy layer above SharedRunQueue. It may read
fresh owner-published metadata, local queue pressure, CPU role, and shared
queue capacity, then choose one source-owned runnable task and one eligible
destination. It must not mutate another CPU's local scheduler directly.

Fairness and affinity are currently under-specified. Talos has no per-task CPU
affinity, priority, virtual runtime, queue age, NUMA/cache locality, or
per-CPU load average. The first contract should either define a deliberately
minimal deterministic policy or explicitly defer those fields.

Remote reschedule is a separate notification problem. After migration
publication, the destination owner needs a way to notice pending shared work,
but the accepted remote wake queue is not that mechanism. The next contract
must decide whether the first implementation is polling-only from owner-local
service cycles or introduces a new remote-reschedule signal.

Migration mechanism already has a core: source-owner publish and
destination-owner consume through SharedRunQueue. The contract should reuse
that core rather than adding a second remote enqueue path.

## Recommended Follow-Up

The next bounded task should be
phase6-load-balancing-contract-20260527.

That task should define the first load-balancing contract: policy inputs,
freshness checks, eligible source and destination rules, failure/defer
outcomes, polling versus remote-reschedule behavior, diagnostic gate
selection, and retained deferrals. It should remain contract-first unless the
supervisor creates a later implementation task. It must not implement load
balancing, work stealing, running-task migration, multi-core preemption,
Phase 7, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

## Validation

- static inspection: git status --short was clean before edits.
- static review: inspected accepted shared run-queue closeout, scheduler
  architecture, src/scheduler.rs, src/smp.rs, src/smp_sync.rs, retained
  Phase 6 QEMU/Pi 5 diagnostic surfaces, roadmap, decision log, accepted task
  records, and accepted evidence summaries.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU reruns, and Pi 5 hardware runs were not required
  because this task changes only Markdown documentation and durable task state.
