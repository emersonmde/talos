# Phase 6 Multi-Core Preemption Contract

Status: accepted as the Phase 6.3 contract for the first multi-core
preemption slice. This document changes architecture and validation policy
only. It does not add Rust behavior, boot images, QEMU claims, Pi 5 hardware
claims, direct IRQ/IPI-context scheduling, work stealing, running-task
migration, general remote reschedule, Phase 7, filesystem, networking, SSH,
shell behavior, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

This contract follows the accepted multi-core preemption source inventory. It
defines the first allowed invariant: timer and IPI paths may record bounded
state, but scheduler mutation remains owner-local and runs from normal control
flow after interrupt return.

## Allowed Preemption Model

The first multi-core preemption implementation must preserve the existing
CPU-local scheduler authority while allowing each owner CPU to notice and
service a local timer-preemption request:

- A local timer IRQ may acknowledge/classify the interrupt, record a bounded
  pending preemption request for its own LogicalCpuId, reprogram the timer,
  and EOI the interrupt.
- IRQ context must not call SingleCoreScheduler::timer_preempt,
  PerCoreScheduler::set_current_task,
  CpuLocalSchedulerService::run_cycle,
  SecondarySchedulerServiceLoop::run_once, SharedRunQueue consumption, or
  load-balancing publication.
- Owner-local normal control flow is the only place that may mutate the
  owner's current task, RunnableQueue, dispatch counters, saved context, or
  owner-published scheduler metadata.
- A secondary owner may service a pending local timer-preemption request only
  through the accepted SecondaryProductionDiagnostic service-loop role until a
  later task accepts a non-diagnostic secondary runtime role.
- Cross-core IPI paths remain notification-only. They may record delivery,
  wake intent, or a future owner-local service hint, but they may not run the
  scheduler or consume shared queue entries in interrupt context.
- Remote wake remains separate from preemption. RemoteWakeQueue carries wake
  intent for already target-owned blocked tasks, not migration,
  preemption-completion acknowledgement, or remote enqueue authority.

This model allows QEMU and Pi 5 proofs to show each participating CPU records
and services its own timer-preemption request without granting any CPU
permission to switch another CPU's current task.

## Ownership And Current Task

Current-task authority is per owner:

- Each PerCoreScheduler owns exactly one local current-task slot plus its
  local RunnableQueue.
- The current task may be switched only by the owning CPU's scheduler service
  path after it has observed a pending local preemption request.
- Cross-owner calls to production_scheduler_mut, set_current_task, and
  dispatch_cpu_local_diagnostic_task remain deterministic errors.
- SharedSchedulerMetadata may publish current-on-owner and runnable-on-owner
  snapshots, but those snapshots do not grant mutation authority.
- SharedRunQueue and LoadBalancingPolicy may move only runnable, non-current
  tasks through the accepted owner-transfer mechanism. Running-task migration
  remains rejected.

The implementation may add a small per-owner pending-preemption state if
needed, but it must not introduce a global current-task registry or a remote
CPU current-task write path.

## Lock Ordering And Critical Sections

All scheduler topology mutation must preserve the accepted lock order:

- save/mask local IRQ state before taking any SMP scheduler lock;
- release the SMP scheduler lock before restoring local IRQ state;
- do not hold scheduler locks across talos_aarch64_context_switch, printing,
  allocation, blocking, sleeping, diagnostic command dispatch, timer
  programming loops, IPI send loops, or lab waits;
- keep IRQ and IPI handlers bounded to acknowledgement, accounting, request
  recording, and EOI.

If the first implementation needs to sample shared metadata or SharedRunQueue
state before servicing local preemption, stale or contended observations must
produce deterministic defer/retry outcomes instead of widening the critical
section.

## Memory Ordering And Metadata Refresh

The first multi-core preemption implementation must keep metadata advisory and
owner-published:

- Owner-local scheduler mutation happens before that owner refreshes
  SharedSchedulerMetadata.
- Metadata generation numbers remain freshness checks, not synchronization
  proof that a remote CPU can mutate local scheduler state.
- Stale generation, owner mismatch, unknown task, duplicate task, invalid CPU,
  and invalid role outcomes must be deterministic rejection or defer results.
- A destination CPU may learn that work exists only through accepted polling,
  wake, shared queue, or future notification-only hints; it still consumes and
  mutates from owner-local normal control flow.

The first implementation should prefer polling-compatible service-cycle
integration. Notification-only remote reschedule may be specified later, but
it is not required here.

## Preemption Disable And Nested Sections

The first implementation may introduce a bounded preemption-disable counter or
equivalent per-owner state only if it has clear deterministic behavior:

- A pending timer-preemption request recorded while preemption is disabled is
  deferred, not dropped.
- Nested disabled sections must balance before the owner-local service path
  can switch tasks.
- Underflow, overflow, wrong-owner updates, and service attempts while
  disabled must produce deterministic errors or defer results in tests.
- IRQ masking by itself is not a general preemption-disable contract. The
  implementation must document any separate counter or state it adds.

If the core task can preserve the invariant without adding an explicit counter,
it must still make the deferred nested/preemption-disabled outcome explicit in
tests or task evidence.

## Failure Outcomes

The implementation must expose deterministic outcomes for:

- no pending local preemption request;
- pending local request but no local runnable peer;
- current task missing, not running, or owned by the wrong CPU;
- scheduler service called from a wrong owner or deferred role;
- nested or preemption-disabled state requires deferral;
- stale metadata generation or metadata owner mismatch during any optional
  shared observation;
- pending remote wake that must be drained before or after local preemption
  according to the accepted service-cycle order;
- full local, shared, or destination queues if a proof combines preemption with
  accepted migration/load-balancing surfaces;
- attempt to run scheduler work directly in IRQ/IPI context;
- attempt to migrate or remotely switch a running task.

Every failure must leave task ownership, local queue membership, current-task
state, and owner-published metadata in a single-owner state.

## Proof Plan

The next implementation task should add only the target-independent or narrowly
target-abstracted state needed for this contract, with unit tests before any
new QEMU or hardware proof.

The QEMU proof task should add one focused boot scenario and script that show:

- multiple logical CPUs can record local timer-preemption requests;
- each owner services its own request from normal scheduler control flow;
- IRQ/IPI context does not execute scheduler mutation;
- remote wake, secondary service-loop, shared run-queue, and load-balancing
  retained gates still pass;
- output reaches a stable PASS/classification for the implemented invariant,
  not a marker-only shim.

The Pi 5 proof task remains separate and serialized under hardwareTestLock. It
must prove the same named invariant as QEMU with candidate identity, fresh
serial cursor, TFTP evidence, archive/kernel digests, classification/PASS,
participant counts, restore proof, and required inconclusive-run triage before
any post-hardware code changes.

The closeout checkpoint remains separate. It must reconcile source inventory,
contract, implementation, QEMU proof, Pi 5 proof or explicit physical defer
decision, retained diagnostics, risks, and the next task recommendation before
Phase 7 or later subsystem work.

## Deferrals

This contract does not accept:

- direct scheduler execution from IRQ or IPI context;
- remote switching of another CPU's current task;
- running-task migration or asynchronous cross-core context capture;
- autonomous work stealing or a continuously running load-balancer loop;
- general remote reschedule beyond possible notification-only future hints;
- a non-diagnostic secondary idle/wake runtime beyond accepted service-loop
  proof boundaries;
- lower-EL userspace, syscalls, descriptor tables, POSIX process semantics, or
  portable userland;
- filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt, or
  DMA/cache-driver behavior.

The next bounded task may implement the first multi-core preemption core only
if it stays inside this contract and preserves owner-local scheduler mutation.

## Validation

- static inspection: git status --short was clean before edits.
- static inspection: reviewed the accepted multi-core preemption source
  inventory, scheduler architecture, load-balancing closeout and contract,
  shared run-queue/migration contract, src/scheduler.rs, src/smp.rs,
  src/smp_sync.rs, src/arch/aarch64/generic_timer.rs,
  src/arch/aarch64/exceptions.rs, src/arch/aarch64/gicv2.rs, roadmap, and
  decision log.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU reruns, and Pi 5 hardware runs are not required because
  this task changes only Markdown documentation and durable task state.
