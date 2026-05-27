# Phase 6 Load-Balancing Policy Contract

Status: accepted as the Phase 6.3 contract for the first load-balancing
policy slice. This document changes architecture and validation policy only.
It does not add Rust behavior, boot images, QEMU claims, Pi 5 hardware claims,
load-balancing implementation, work stealing, running-task migration,
multi-core preemption, Phase 7, filesystem, networking, SSH, shell behavior,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy.

This contract follows the accepted load-balancing source inventory. It defines
how a later policy may choose a runnable task and destination CPU before using
the accepted SharedRunQueue owner-transfer mechanism.

## Policy Authority

The first load-balancing policy is an owner-local decision layer, not a global
scheduler:

- A source CPU may consider only tasks it owns and observes as locally
  runnable. It may not select its current running task, a blocked task, an
  unknown task, or a task already queued for shared migration.
- A destination CPU is eligible only if its SchedulerCoreRole can consume
  production scheduler work under the accepted owner-local service-loop model.
  Deferred secondary roles remain ineligible.
- Policy selection may read owner-published SharedSchedulerMetadata snapshots,
  local RunnableQueue pressure, SharedRunQueue capacity, and CPU roles. Those
  inputs are advisory until the migration mechanism re-checks freshness and
  ownership.
- Policy may request a migration by calling the accepted SharedRunQueue
  publish/consume mechanism. It must not directly mutate another CPU's
  RunnableQueue, current task, counters, saved context, or metadata owner.
- SharedSchedulerMetadata remains observational. It can reject stale or
  unknown observations, but it does not grant repair authority to a remote CPU.

The first implementation should keep the policy API small: select one
source-owned runnable candidate, select one eligible destination, and report a
deterministic decision or defer reason.

## Target Selection Inputs

The first policy may use only inputs that are accepted today:

- LogicalCpuId for source and candidate destination identity.
- SchedulerCoreRole for destination eligibility.
- RunnableQueue::len, is_empty, is_full, contains, and front for owner-local
  queue pressure and candidate selection.
- PerCoreScheduler current-task state to reject running-task migration.
- SharedSchedulerMetadata task owner, state, current-on-owner,
  runnable-on-owner, and generation for cross-core freshness checks.
- SharedRunQueue capacity, len, duplicate membership, and full-queue errors
  for bounded handoff backpressure.
- RemoteWakeQueue pressure only as a separate wake-path signal. It is not a
  migration queue, completion acknowledgement, or remote-reschedule channel.
- CPU-local timer-preemption pending state only as local context. It is not an
  accepted cross-core balancing interrupt.

The first policy must not invent per-task affinity, priority, virtual runtime,
queue age, CPU load averages, cache locality, NUMA topology, or fairness
history. Those fields remain unavailable until a later task adds and validates
them.

## Deterministic Policy Shape

The first accepted implementation should be deliberately conservative:

- Candidate source task: choose a locally runnable, non-current task from the
  source owner's RunnableQueue, using deterministic FIFO/front-first behavior
  unless the implementation task documents a narrower test-only selector.
- Destination: choose an eligible production-capable destination with local
  queue capacity and a valid LogicalCpuId. A destination that cannot consume
  scheduler work must produce a defer result.
- Fairness: initial fairness is limited to avoiding duplicate migration and
  preserving single-owner queue membership. There is no accepted fairness
  accounting beyond the deterministic queue order.
- Affinity: no hard or soft affinity exists yet. The first policy must treat
  all eligible production-capable destinations equally unless later metadata
  adds affinity.
- Backpressure: full destination queues, full shared queues, duplicate shared
  entries, and stale metadata produce deterministic deferral, not unbounded
  retry loops or forced migration.

This policy is enough to exercise target selection while keeping the accepted
SharedRunQueue mechanism as the only owner-transfer path.

## Freshness And Rollback Rules

Every policy decision is provisional until the migration mechanism accepts it:

- The policy must carry the metadata generation it used for the candidate
  task. SharedRunQueue::publish_migration must re-check that generation before
  removing the task from the source-local runnable queue.
- Stale metadata, unknown tasks, metadata owner mismatch, and task-state
  mismatch must return deterministic rejection or defer outcomes.
- If publication fails, the task remains source-owned and the policy may not
  synthesize a remote repair.
- If destination consumption fails, the task must return to one accepted state
  through the SharedRunQueue failure path; it may not appear on two local
  runnable queues.
- Running-task migration remains rejected. Multi-core preemption and
  asynchronous context capture must be separately accepted before policy can
  move a current task.

The policy layer may classify failures for diagnostics, but the state machine
and rollback authority remain in the accepted migration mechanism.

## Remote Reschedule Contract

Remote reschedule is not required for the first implementation. The first
load-balancing core may be polling-only: destination owners observe shared
work from normal owner-local scheduler service cycles or focused diagnostic
proof code.

If a later task adds remote reschedule, it must be notification only:

- an IPI or wake-like signal may record that normal scheduler control flow
  should run soon;
- the interrupt handler must not run the scheduler, consume SharedRunQueue
  entries, mutate another CPU's local queue, print diagnostics, or hold
  scheduler topology locks for unbounded work;
- RemoteWakeQueue must remain dedicated to already target-owned blocked-task
  wake requests.

This keeps remote reschedule separate from remote enqueue, migration, and
remote wake semantics.

## Failure Outcomes

The implementation must expose deterministic outcomes for:

- no locally runnable non-current source candidate;
- source candidate is running, blocked, unknown, not source-owned, or not in
  the source-local runnable queue;
- stale metadata generation or metadata owner mismatch;
- invalid source or destination LogicalCpuId;
- destination role is deferred or otherwise not production-capable;
- destination local queue is full;
- shared run queue is full;
- duplicate local or shared queue membership;
- remote wake requested for migration or migration acknowledgement;
- timer/preemption state requested as a cross-core balancing mechanism;
- running-task migration requested before multi-core preemption is accepted.

Each failure should leave task ownership, local queue membership, and
owner-published metadata in a single-owner state.

## Validation Strategy

The next implementation task should validate the policy core without QEMU or
hardware claims first:

- cargo fmt --all -- --check;
- cargo -Zjson-target-spec test;
- focused unit tests for source selection, destination rejection,
  stale-generation rejection, full-queue backpressure, duplicate rejection,
  invalid-role rejection, and preservation of single-owner membership;
- scripts/qemu-smoke.sh plus retained secondary scheduler service-loop and
  shared run-queue migration QEMU gates if the implementation touches runtime
  scheduler paths.

The QEMU proof task remains separate. It should prove that the implemented
policy selects a destination, publishes through SharedRunQueue, lets the
destination consume from normal control flow or a focused diagnostic adapter,
and reports a deterministic PASS/classification.

The Pi 5 proof task remains separate and serialized under hardwareTestLock. It
must prove the same named invariant as QEMU with candidate identity, fresh
serial cursor, TFTP evidence, classification/PASS, and restore proof.

The load-balancing closeout checkpoint remains separate. It must reconcile
inventory, contract, implementation, QEMU proof, Pi 5 proof or explicit
physical defer decision, retained diagnostics, risks, and next task
recommendations before broader Phase 6.3 work.

## Deferrals

This contract does not accept:

- work stealing loops or autonomous background balancing;
- migration of currently running tasks;
- multi-core timer preemption or asynchronous cross-core context switching;
- a non-diagnostic secondary idle/wake runtime beyond accepted service-loop
  proof boundaries;
- stable fairness, priority, affinity, virtual runtime, queue age, or
  CPU-load-average policy;
- remote scheduler execution in IPI context;
- Phase 7 process, EL0, syscall, descriptor, or POSIX behavior;
- filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt, or
  DMA/cache-driver behavior.

The next bounded task may implement the first target-independent
load-balancing core only if it stays inside this contract and preserves the
accepted SharedRunQueue ownership contract.

## Validation

- static inspection: reviewed the accepted load-balancing source inventory,
  shared run-queue/migration contract and closeout, docs/src/architecture/scheduler.md,
  src/scheduler.rs, src/smp.rs, src/smp_sync.rs, roadmap, and decision log.
- whitespace inspection: git diff --check must pass before acceptance.
- documentation: mdbook build must pass before acceptance.
- Rust fmt/tests, QEMU reruns, and hardware runs are not required because this
  task changes only Markdown documentation and durable task state.
