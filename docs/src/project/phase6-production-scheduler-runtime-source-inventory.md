# Phase 6 Production Scheduler Runtime Source Inventory

Status: accepted as a Phase 6.3 source inventory before any production
timer/preemption runtime integration. This document changes documentation
only. It does not add Rust behavior, boot scenarios, QEMU runs, Pi 5 hardware
runs, direct IRQ/IPI-context scheduling, remote current-task switching,
running-task migration, autonomous work stealing, Phase 7, filesystem,
networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

This inventory follows the accepted multi-core preemption closeout. It maps
the accepted diagnostic scheduler surfaces against the normal boot, timer, and
owner-local runtime paths so the next task can define a small production
timer/preemption contract without drifting into a broad scheduler rewrite.

## Accepted Diagnostic Surfaces

- `src/scheduler.rs` owns the accepted target-independent scheduler core.
  `PerCoreScheduler` keeps the current-task slot and local `RunnableQueue`
  owner-local. Cross-owner `production_scheduler_mut`, `set_current_task`, and
  CPU-local diagnostic dispatch calls remain deterministic errors.
- `PerCorePreemptionState::record_local_timer_irq` is the accepted bounded
  timer-side recording hook. It records or coalesces only local pending state
  for the owning `LogicalCpuId`; it does not inspect runnable queues, choose a
  task, refresh metadata, mutate `PerCoreScheduler`, or touch another CPU's
  scheduler state.
- `CpuLocalSchedulerService::run_preemption_cycle` is the accepted
  owner-local service entry for pending preemption state. It preflights the
  preemption-state owner, scheduler owner, production-capable role, and
  current-task identity before calling the normal service cycle.
- `CpuLocalSchedulerService::run_cycle` is the accepted normal-control-flow
  scheduler service order: consume at most one target-owned `RemoteWakeQueue`
  request, service local timer preemption when requested, optionally dispatch
  one local diagnostic task only when timer preemption did not run, then
  refresh owner-published `SharedSchedulerMetadata`.
- `SecondarySchedulerServiceLoop::run_once` adapts one service cycle for
  secondary owners in `SecondaryProductionDiagnostic`. It rejects boot-CPU
  use, wrong owners, and deferred secondary roles. It is a diagnostic
  secondary service-loop proof surface, not a continuous production idle loop.
- `RemoteWakeQueue` is still a bounded target-owned signal mailbox. IPI paths
  may record delivery, EOI, or wake intent, but only the target owner may
  consume the request and mutate local scheduler state from normal flow.
- `SharedSchedulerMetadata` is advisory and owner-published. It exposes owner,
  task-state, current-on-owner, runnable-on-owner, process-owner placeholder,
  stack bounds, and generation snapshots. It is not a global task registry or
  remote mutation authority.
- `SharedRunQueue` and `LoadBalancingPolicy` are accepted owner-transfer and
  deterministic front-runnable policy surfaces. They move only runnable,
  non-current tasks through source-owner publication and destination-owner
  consumption. Running-task migration remains rejected.
- `scripts/qemu-timer-preemption-smoke.sh`,
  `scripts/qemu-secondary-scheduler-service-loop-smoke.sh`,
  `scripts/qemu-shared-runqueue-migration-smoke.sh`,
  `scripts/qemu-load-balancing-smoke.sh`, and
  `scripts/qemu-multicore-preemption-smoke.sh` are retained QEMU proof gates.
  The matching Pi 5 image and boot-tree helpers for accepted timer,
  service-loop, shared-runqueue, load-balancing, and multi-core preemption
  proofs are retained serialized hardware reproduction surfaces.

## Normal Runtime And Timer Paths

- `src/arch/aarch64/generic_timer.rs` owns EL2 physical timer helpers.
  `record_el2_physical_tick_and_rearm` increments the monotonic tick counter
  and rearms the timer. It has no scheduler-owner, current-task,
  preemption-state, metadata, or remote-wake authority.
- `src/arch/aarch64/exceptions.rs` routes IRQs to target handlers. It does
  not own scheduler state and does not decide whether a timer IRQ should
  switch tasks.
- `src/target/qemu_virt.rs::handle_irq` and `src/target/rpi5.rs::handle_irq`
  acknowledge/classify GIC interrupts, call the generic timer rearm helper for
  timer INTIDs, record retained diagnostic counters under old
  `*_timer_preemption` boot scenarios, EOI the interrupt, and return. These
  handlers currently do not call `PerCorePreemptionState::record_local_timer_irq`
  in the normal timer path.
- The accepted single-core timer-preemption diagnostics still hand off through
  scenario-local request counters consumed by diagnostic kernel threads that
  call `SingleCoreScheduler::timer_preempt` after IRQ return. They are useful
  gates for timer-driven progress and IRQ ordering, but they are not the new
  multi-core per-owner preemption-state path.
- The accepted QEMU and Pi 5 multi-core preemption proofs construct local
  diagnostic scheduler, preemption-state, remote-wake, and metadata objects
  inside focused proof routines, then call
  `PerCorePreemptionState::record_local_timer_irq` and
  `CpuLocalSchedulerService::run_preemption_cycle` directly from diagnostic
  owner-local flow. They prove the invariant, but they do not install durable
  per-CPU runtime scheduler objects or wire the real timer IRQ to those
  objects.
- Boot-scenario routing in `build.rs`, `src/main.rs`, `src/target/qemu_virt.rs`,
  and `src/target/rpi5.rs` selects focused diagnostics. The default runtime
  still lacks a production owner loop that continuously waits for local work,
  observes pending preemption state, drains target-owned wake requests, and
  refreshes metadata for every production-capable CPU.
- `src/smp.rs` owns logical CPU identity, secondary stacks, secondary entry,
  cache maintenance helpers, and diagnostic progress. It does not own
  scheduler policy or a non-diagnostic secondary runtime role.

## Retained Gates Versus Proof-Only Surfaces

Retained gates for the next production contract and implementation are:

- `cargo -Zjson-target-spec test` for target-independent scheduler behavior.
- `scripts/qemu-timer-preemption-smoke.sh` for timer IRQ ordering and the
  older single-core post-IRQ dispatch boundary.
- `scripts/qemu-secondary-scheduler-service-loop-smoke.sh` for secondary
  owner-local service ordering in the diagnostic role.
- `scripts/qemu-shared-runqueue-migration-smoke.sh` and
  `scripts/qemu-load-balancing-smoke.sh` for owner-transfer and polling policy
  invariants that production preemption must not bypass.
- `scripts/qemu-multicore-preemption-smoke.sh` for the accepted per-owner
  record/service invariant.

Proof-only surfaces that must not become production APIs are:

- `qemu_multicore_preemption_smoke` and `rpi5_multicore_preemption_proof`
  scenario-local construction of schedulers, preemption state, metadata, and
  report structures.
- Direct diagnostic calls to `record_local_timer_irq` from proof routines that
  bypass the real timer IRQ handler.
- The old single-core timer-preemption scenario-local request counters as a
  multi-core runtime mechanism.
- `SecondaryProductionDiagnostic` as a general idle-loop or work-stealing
  role.
- Pi 5 image/boot-tree helpers as implicit production enablement; they remain
  reproduction tools for explicit hardware-proof tasks only.

## Production Runtime Gaps

The next contract must keep these gaps explicit before code changes:

- Durable per-CPU runtime ownership: Talos needs an agreed storage and access
  boundary for each production-capable CPU's `PerCoreScheduler`,
  `PerCorePreemptionState`, `RemoteWakeQueue`, and metadata handle.
- Timer IRQ recording: the normal QEMU and Pi 5 timer handlers need a bounded
  way to map the current logical CPU to its local preemption state and record a
  pending request without scheduler mutation, printing, allocation, blocking,
  or unbounded locking.
- Owner-local service point: after IRQ return, production normal flow needs a
  named place to call `CpuLocalSchedulerService::run_preemption_cycle` with the
  current task and local objects. Today that path exists in diagnostic proof
  routines, not in a durable runtime loop.
- Current-task source of truth: the contract must say how owner-local normal
  flow obtains the mutable current task that matches the `PerCoreScheduler`
  current slot before timer preemption can service.
- Secondary runtime role: secondaries can prove a single diagnostic service
  cycle, but Talos has not accepted a non-diagnostic continuous secondary idle
  loop or a wake protocol that makes remote work serviceable outside a proof.
- Preemption-disable scope: the target-independent counter exists, but the
  normal runtime has not yet specified which critical sections enter/exit it,
  or how a timer IRQ observes a disabled owner without widening IRQ work.
- Remote wake and metadata interaction: production service must preserve the
  accepted order of remote wake consumption before timer preemption and owner
  metadata refresh after local mutation. Stale metadata, pending remote wake,
  full queues, and wrong-owner access must stay deterministic defer/reject
  cases.
- Proof routing: the next QEMU proof should exercise the production timer IRQ
  and owner-local runtime entry, not only direct diagnostic helper calls.

## Recommended Follow-Up

The next bounded task should be
`phase6-production-timer-preemption-contract-20260528`.

That contract should name the exact production entry points that an
implementation may change: the normal per-owner timer IRQ recording path, the
owner-local post-IRQ service point in primary and secondary runtime flow, and
the state objects those paths may access. It should preserve record-only
IRQ/IPI behavior, owner-local scheduler mutation, remote-wake-first service
ordering, metadata refresh rules, deterministic disabled/stale/wrong-owner
outcomes, and retained QEMU/Pi 5 proof expectations.

## Validation

- static inspection: git status --short was clean before edits.
- static review: inspected accepted multi-core preemption closeout,
  architecture, roadmap, decision log, scheduler core, target IRQ handlers,
  boot-scenario routing, retained QEMU/Pi 5 proof scripts, and accepted task
  records.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU reruns, and Pi 5 hardware runs were not required
  because this task changes only Markdown documentation and durable worker
  state.
