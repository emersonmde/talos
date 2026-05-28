# Phase 6 Production Timer/Preemption Contract

Status: accepted as the Phase 6.3 production timer/preemption integration
contract. This document changes documentation only. It does not add Rust
behavior, QEMU or Pi 5 proof, direct IRQ/IPI-context scheduling, remote
current-task switching, running-task migration, autonomous work stealing,
Phase 7, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

This contract follows the accepted production scheduler runtime source
inventory. It defines the only production runtime surfaces that the next
implementation task may change to carry the accepted owner-local preemption
primitive into normal timer and runtime paths.

## Accepted Invariants

- Scheduler mutation remains owner-local. Only the owning CPU may mutate its
  `PerCoreScheduler`, current-task slot, local runnable queue, local
  `RemoteWakeQueue` consumption, and owner-published metadata.
- IRQ and IPI hot paths are record-only. They may acknowledge/classify an
  interrupt, map the current logical CPU to its local bounded state, record a
  pending local event, rearm/EOI as required, and return. They must not
  inspect runnable queues, choose a task, refresh metadata, consume remote
  wake requests, publish or consume `SharedRunQueue` entries, dispatch, switch
  current tasks, print diagnostics, allocate, block, or take unbounded
  scheduler topology locks.
- Owner-local normal control flow is the only place that may call
  `CpuLocalSchedulerService::run_preemption_cycle` for production
  timer-preemption work.
- The service order remains target-owned remote wake consumption, local timer
  preemption, optional local dispatch only when timer preemption did not run,
  and owner-published metadata refresh.
- `SharedSchedulerMetadata` remains advisory and owner-published. It cannot be
  used as remote current-task authority or as permission to mutate another
  CPU's local scheduler.
- `SharedRunQueue` and `LoadBalancingPolicy` remain runnable, non-current
  owner-transfer mechanisms. They are not part of the first production timer
  preemption implementation.

## Production Entry Points

The next implementation task may change only these production runtime entry
points and the minimal state plumbing needed by them:

- `src/target/qemu_virt.rs::handle_irq`: on the normal QEMU timer INTID path,
  after interrupt classification and before return, record a local pending
  timer-preemption request for the current logical CPU through its production
  `PerCorePreemptionState`. The handler must continue to rearm the generic
  timer and EOI the interrupt through the existing target path.
- `src/target/rpi5.rs::handle_irq`: on the normal Pi 5 timer INTID path, do
  the same bounded local preemption recording while preserving the accepted
  GIC acknowledge/rearm/EOI ordering.
- `src/arch/aarch64/generic_timer.rs::record_el2_physical_tick_and_rearm`:
  remains the timer tick and rearm helper. It must not gain scheduler
  mutation authority. If the implementation needs to return a narrow
  "local timer tick observed" signal to the target handler, that return value
  is allowed only as input to local record-only state.
- The primary owner-local post-IRQ runtime service point: the implementation
  must add or name one normal-flow location for logical CPU 0 to service its
  pending local preemption request through `CpuLocalSchedulerService`.
- The secondary owner-local runtime service point: the implementation must add
  or name one normal-flow location for production-capable secondary CPUs to
  service pending local preemption. Existing
  `SecondarySchedulerServiceLoop::run_once` remains a diagnostic adapter until
  a non-diagnostic secondary runtime role is explicitly accepted.
- The durable per-CPU scheduler runtime state boundary: the implementation may
  introduce storage/accessors for each production-capable CPU's
  `PerCoreScheduler`, `PerCorePreemptionState`, target-owned
  `RemoteWakeQueue`, owner-published metadata handle, current-task pointer or
  handle, and role/capability bit. That boundary must be CPU-local for
  mutation and must reject cross-owner mutable access.

No other target, scheduler policy, Phase 7, filesystem, networking, shell,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver surface is in scope
for the implementation task.

## Timer IRQ Recording Rules

The normal timer handler may record a pending preemption request only when all
of the following are true:

- the handler can resolve the current logical CPU;
- the resolved CPU has production scheduler runtime state;
- that runtime state has a `PerCorePreemptionState` owned by the same logical
  CPU;
- the CPU role is production-capable for local preemption recording.

If those checks pass, the handler records with
`PerCorePreemptionState::record_local_timer_irq(requester)`. `Inserted` and
`Coalesced` are both successful record-only outcomes. The handler must preserve
the existing timer rearm and interrupt EOI behavior regardless of whether a
request is inserted, coalesced, or rejected.

If those checks fail, the handler must defer or reject deterministically:

- no logical CPU identity: rearm/EOI and record a bounded diagnostic counter
  if one exists; do not touch scheduler state;
- missing production runtime state: rearm/EOI and leave no pending
  preemption;
- wrong owner: rearm/EOI and report/record a deterministic wrong-owner
  outcome outside the hot path;
- non-production-capable role: rearm/EOI and leave the request unrecorded;
- pending request already present: coalesce through the preemption state and
  do not try to service in IRQ context.

The IRQ path does not inspect the preemption-disable depth to decide whether
to run the scheduler. Disabled preemption is handled by the owner-local service
point.

## Owner-Local Service Rules

Each production-capable CPU needs a normal-flow service point that observes
its local pending state after IRQ return and calls
`CpuLocalSchedulerService::run_preemption_cycle` with only local objects:

- the requester equal to the owner logical CPU;
- the owner's `PerCoreScheduler`;
- the owner's `PerCorePreemptionState`;
- the owner's target-owned `RemoteWakeQueue`;
- the accepted metadata table or owner-published metadata handle;
- the local task object for remote wake/dispatch interactions;
- the mutable current task that matches `PerCoreScheduler::current_task()`;
- the accepted dispatch flag for that runtime role.

The service point must preserve the accepted `run_cycle` order: consume at
most one target-owned remote wake request first, service local timer
preemption only if pending and enabled, optionally dispatch one local task
only when timer preemption did not run, then refresh metadata.

The service point must handle these outcomes deterministically:

- disabled preemption: `ServiceDeferred` leaves the pending request set and
  keeps scheduler state unchanged until the owner exits the nested disabled
  section;
- stale metadata: reject the metadata-dependent operation and keep scheduler
  ownership unchanged;
- wrong owner or state/scheduler owner mismatch: reject and do not mutate
  scheduler queues or current task;
- missing current task for pending preemption: reject and leave the pending
  request set;
- current-task mismatch: reject and leave the pending request set;
- non-production-capable role: reject local dispatch/preemption service
  without converting the role into a production secondary loop;
- no runnable peer for timer preemption: preserve the existing
  `SingleCoreScheduler::timer_preempt` failure semantics and leave ownership
  local.

The implementation may add small counters or reports for these outcomes, but
only outside the IRQ/IPI hot path and only when they do not widen the runtime
surface.

## Current Task And Role Rules

The current task source of truth remains the owner-local scheduler current
slot plus the local mutable task object provided to the service point. The
implementation must not infer current-task authority from
`SharedSchedulerMetadata`, from a remote wake request, from `SharedRunQueue`,
or from another CPU's published state.

The boot CPU may be the first production-capable owner. Secondary CPUs remain
production-capable only for roles explicitly accepted by prior Phase 6.3 work.
`SecondaryProductionDiagnostic` remains a proof role, not a general idle loop
or work-stealing role. If the implementation cannot name a non-diagnostic
secondary service point without broadening scope, it must keep secondary
production preemption deferred and document that defer in the task evidence.

## Retained Gates And Later Proof Expectations

The implementation task must retain these validation gates unless an accepted
contract update replaces them:

- `cargo fmt --all -- --check`;
- `cargo -Zjson-target-spec test`;
- `scripts/qemu-smoke.sh`;
- `scripts/qemu-timer-preemption-smoke.sh`;
- `scripts/qemu-secondary-scheduler-service-loop-smoke.sh`;
- `scripts/qemu-shared-runqueue-migration-smoke.sh`;
- `scripts/qemu-load-balancing-smoke.sh`;
- `scripts/qemu-multicore-preemption-smoke.sh`;
- `git diff --check`;
- `mdbook build` if documentation changes.

The focused QEMU production proof that follows implementation must exercise
the production timer IRQ recording path and owner-local post-IRQ service
point, not only direct diagnostic calls to
`PerCorePreemptionState::record_local_timer_irq`. It should report the timer
recording outcome, service outcome, current-task transition, remote-wake-first
ordering, metadata refresh, and deterministic disabled/stale/wrong-owner
outcomes.

The serialized Pi 5 proof remains a separate later task. It must use
`hardwareTestLock`, record candidate commit and image/archive identity, TFTP
fetch evidence, fresh serial output, participant counts, classification/PASS
or blocker classification, and restore proof. No physical claim is made by
this contract.

## Next Implementation Boundary

The next bounded task is
`phase6-production-timer-preemption-core-20260528`. It may implement only the
entry points and state plumbing named above. It must not add a new phase,
syscall or EL0 behavior, file descriptors, filesystem, networking, SSH, shell,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, remote
current-task switching, running-task migration, autonomous work stealing, or
direct IRQ/IPI-context scheduler mutation.

## Validation

- static inspection: git status --short was clean before edits.
- documentation: this contract names the exact production entry points,
  owner-local invariants, deterministic defer/reject behavior, retained gates,
  and later QEMU/Pi 5 proof expectations.
- whitespace inspection: git diff --check passed.
- documentation build: mdbook build passed.
- Rust fmt/tests, QEMU, and Pi 5 hardware runs were not required because this
  task changes only Markdown documentation and durable worker state.
