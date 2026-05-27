# Phase 6 Multi-Core Preemption Source Inventory

Status: accepted as a Phase 6.3 source inventory before any multi-core
preemption implementation. No Rust behavior change, boot scenario, QEMU run,
Pi 5 hardware run, remote reschedule implementation, work stealing,
running-task migration, Phase 7, filesystem, networking, SSH, shell,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver behavior is added by
this inventory.

This inventory follows the accepted load-balancing closeout. It maps the
existing timer, scheduler, SMP, IPI/wake, shared metadata, SharedRunQueue, and
load-balancing boundaries that the next contract must preserve.

## Source Inventory

- src/arch/aarch64/generic_timer.rs owns the EL2 physical timer helpers.
  record_el2_physical_tick_and_rearm increments the relaxed monotonic tick
  counter and rearms the timer; it does not know about scheduler queues,
  current tasks, preemption-disable state, or CPU ownership.
- src/arch/aarch64/exceptions.rs routes IRQs through rust_irq_handler. QEMU
  IRQs are delegated to target::qemu_virt::handle_irq; the retained Pi 5
  timer/IPI diagnostics are delegated to target::rpi5::handle_irq. Unhandled
  IRQs are recorded as unexpected snapshots only.
- src/target/qemu_virt.rs and src/target/rpi5.rs implement the existing
  single-CPU timer-preemption diagnostics. Their IRQ handlers acknowledge the
  GIC interrupt, record a timer-preemption request counter, rearm the timer,
  and return. The scheduler switch is performed later by the running
  diagnostic kernel thread through dispatch_timer_preemption_from, not directly
  from IRQ context.
- src/scheduler.rs keeps timer preemption target-independent in
  SingleCoreScheduler::timer_preempt. The current task must already be
  Running, a local runnable peer must exist, and the current task is requeued
  before the next task is selected.
- src/scheduler.rs wraps per-CPU ownership in PerCoreScheduler.
  production_scheduler_mut, set_current_task, and
  dispatch_cpu_local_diagnostic_task require the requester to be the owner and
  require a production-enabled role. Cross-owner mutation is rejected.
- CpuLocalSchedulerService::run_cycle is the current normal-control-flow
  service point. It consumes at most one target-owned remote wake request,
  handles a pending local timer-preemption request, optionally dispatches one
  local diagnostic task, and refreshes owner-published metadata.
- SecondarySchedulerServiceLoop::run_once adapts one service cycle for
  secondary owners in the explicit SecondaryProductionDiagnostic role. It
  rejects boot-CPU use, wrong owners, and deferred secondary roles.
- RemoteWakeQueue is a bounded target-owned signal mailbox. Remote CPUs may
  publish wake intent for already target-owned blocked tasks; only the target
  owner may consume and mutate its local scheduler.
- src/arch/aarch64/gicv2.rs provides SGI construction and GIC acknowledge/EOI
  helpers. The retained cross-core IPI and remote-wake proof handlers record
  receive/EOI counters and publish bounded mailbox state; they do not execute
  scheduler mutation in IPI context.
- SharedSchedulerMetadata publishes owner, task state, process-owner
  placeholder, stack bounds, current-on-owner, runnable-on-owner, and
  generation snapshots. It rejects wrong-owner, unknown-task, invalid-owner,
  duplicate, and stale-generation cases; it is not a global mutable task
  registry.
- SharedRunQueue is the accepted owner-transfer surface for runnable tasks.
  Source owners publish after fresh metadata checks; destination owners
  consume locally and refresh metadata. Running-task and blocked-task
  migration are explicitly rejected.
- LoadBalancingPolicy selects only one source-local front runnable task and
  one eligible destination, then publishes through SharedRunQueue. It is
  deterministic, polling-compatible, and does not introduce remote reschedule,
  fairness, affinity, or asynchronous context capture.
- src/smp.rs owns secondary-core lifecycle, logical CPU identity, stack
  ownership, cache maintenance helpers, and diagnostic progress. It has no
  scheduler policy, preemption-disable state, or queue mutation authority.
- src/smp_sync.rs owns SpinLock, the AArch64 IRQ-save lock path, and the full
  barrier primitive. The accepted scheduler rule remains local IRQ save before
  SMP scheduler lock acquisition, with no scheduler lock held across context
  switches, printing, allocation, blocking, timer loops, IPI loops, or lab
  waits.
- The retained scripts scripts/qemu-timer-preemption-smoke.sh,
  scripts/qemu-secondary-scheduler-service-loop-smoke.sh,
  scripts/qemu-shared-runqueue-migration-smoke.sh, and
  scripts/qemu-load-balancing-smoke.sh are proof gates for named invariants.
  The Pi 5 timer, service-loop, shared-runqueue, and load-balancing scripts
  are serialized hardware proof surfaces, not a general runtime.

## CPU-Local And Cross-Core Boundary

Today, accepted timer preemption is CPU-local. A timer IRQ can record a pending
request, but scheduler mutation happens from owner-local normal control flow
after IRQ return. SingleCoreScheduler::timer_preempt and
PerCoreScheduler::set_current_task both require the owner to supply the current
task and update the owner-local scheduler.

Cross-core paths are notification or transfer surfaces only. SGI/IPI handlers
record delivery and EOI. RemoteWakeQueue carries target-owned wake intent.
SharedSchedulerMetadata exposes snapshots with generations. SharedRunQueue
transfers runnable ownership only when both source and destination owners act
through their local schedulers. None of those paths currently permits a remote
CPU to switch another CPU's current task.

The accepted secondary service-loop proof shows that a secondary owner can run
one CPU-local service cycle, including pending timer-preemption handling, for
seeded diagnostic tasks. It does not define a continuous idle loop, production
secondary wake protocol, remote reschedule, or asynchronous exception-frame
switching.

## Blockers For Implementation

- Current-task authority: the contract must define where the current task is
  stored per CPU and how normal control flow obtains a mutable current task
  after a timer IRQ records a pending preemption request.
- Preemption-disable policy: Talos does not yet have a nesting counter or
  critical-section contract that tells timer/IPI paths when a preemption
  request must be deferred.
- IRQ/IPI context boundary: the next contract must keep GIC/timer handlers
  bounded to acknowledgement, accounting, and request recording unless it
  deliberately proves a stricter exception-frame switch contract.
- Lock ordering: shared metadata and SharedRunQueue access must preserve the
  local-IRQ-save before SMP-lock rule and must not hold scheduler locks across
  context switching or lab/diagnostic waits.
- Metadata freshness: any cross-core decision must tolerate stale generations
  and produce deterministic defer/retry outcomes rather than forcing a switch.
- Remote reschedule semantics: accepted remote wake is not remote enqueue or
  remote reschedule. A destination CPU noticing pending work must be specified
  separately, likely polling-only for the first contract.
- Secondary runtime role: SecondaryProductionDiagnostic is still a proof role
  for seeded tasks, not a general production scheduler loop.
- Running-task migration: accepted SharedRunQueue rejects running tasks.
  Asynchronous capture or migration of a currently running task remains out of
  scope until a later explicit task.
- Diagnostic proof routing: new proof scenarios must exercise the real
  owner-local preemption path and not introduce marker-only PASS output.

## Recommended Follow-Up

The next bounded task should be
phase6-multicore-preemption-contract-20260527.

That task should define the first allowed multi-core preemption invariant:
timer/IPI handlers record bounded state, owner-local normal control flow
performs scheduler mutation, metadata refresh remains owner-published, and
remote reschedule remains deferred or notification-only. It should name stale
metadata, wrong-owner access, nested or preemption-disabled sections, pending
remote wake, and full-queue outcomes before any implementation starts.

## Validation

- static inspection: git status --short was clean before edits.
- static review: inspected src/scheduler.rs, src/smp.rs, src/smp_sync.rs,
  src/arch/aarch64/generic_timer.rs, src/arch/aarch64/exceptions.rs,
  src/arch/aarch64/gicv2.rs, src/target/qemu_virt.rs, src/target/rpi5.rs,
  retained Phase 6 scripts, scheduler architecture docs, accepted task
  records, roadmap, and decision log.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU reruns, and Pi 5 hardware runs were not required
  because this task changes only Markdown documentation and durable task
  state.
