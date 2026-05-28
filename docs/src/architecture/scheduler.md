# Scheduler Shape

This note defines the first Phase 4.3 scheduler shape before Talos adds
scheduler structs, runnable queues, context switching, sleeping, yielding, or
preemptive time slicing. It reconciles the accepted timer and single-core
critical-section evidence with the early POSIX guardrail in
`docs/src/project/early-posix-shape.md`.

## Naming

Talos should keep these terms separate from the first scheduler data model:

- Task: a schedulable execution context known to the scheduler.
- Kernel thread: a task that runs only in kernel address space.
- Process: a later resource-owning container for address space, descriptor
  table, current working directory, credentials, exit/wait state, and one or
  more tasks.
- User thread: a later task executing in a process address space at EL0.

The first implementation may create only kernel threads, but the scheduler must
schedule tasks. It should not make process-owned resources global task fields or
assume one schedulable context is always one Unix process.

## First Single-Core Shape

The first scheduler structures should be a boot-CPU-only kernel-thread model:

- a task identifier that is scheduler-local and not a POSIX process ID;
- per-task saved register or context-switch state;
- per-task kernel stack ownership and stack bounds;
- a small task state enum for at least running, runnable, and blocked or
  sleeping placeholders if those states are not implemented immediately;
- a runnable queue owned by the single boot CPU;
- counters or diagnostics for switches and task state transitions.

The first kernel thread may have no owning process. The struct should leave a
clear extension point for an optional process pointer or handle when Phase 7
introduces process address spaces and descriptor tables. That extension point
does not create processes, descriptors, syscalls, or EL0 in Phase 4.

## Lifetime And Ownership

Per-task kernel stack and saved register state belong to the task, not to a
future process. That keeps task lifetime separable from process lifetime later:
a process may eventually contain multiple tasks, and a task may block, wake, or
exit independently of process resource ownership.

The first scheduler task should avoid embedding future process fields such as
descriptor tables, current working directory, root namespace, credentials,
children, signals, or exit status. Those belong in a later process structure.
If a diagnostic needs names, it should use scheduler-local labels rather than
process IDs or shell command names.

## Critical Sections

The accepted `single_core_irq_mask_save()` and
`single_core_irq_restore()` primitive may protect very short boot-CPU
scheduler invariants while the runnable queue is single-core only. Suitable
uses include taking a runnable-queue snapshot, changing a task state, or
choosing the next runnable task around a context-switch boundary.

This is not a hidden preemption-disable policy. Phase 4.3 still must document
where interrupts are masked around context switching, and it must keep those
sections bounded enough that the periodic timer is not starved. The primitive
also does not provide SMP mutual exclusion, interrupt-safe locks, blocking
locks, sleepable locks, memory-ordering policy for secondary cores, or lower-EL
interrupt masking.

## POSIX Deferrals

The following early POSIX concepts remain intentionally deferred:

- process IDs, parent process IDs, exit status, and wait;
- process address spaces and user stacks;
- descriptor tables, open file descriptions, pipes, sockets, and console
  descriptors;
- current working directory, root directory, and path normalization;
- syscall ABI and errno mapping;
- spawn, exec, user-thread creation, and shell command launching.

The scheduler shape should make those additions possible without implementing
them now. In particular, wakeups should target tasks, and blocking I/O should
eventually sleep a task without implying that an entire future process model is
blocked by a global singleton.

## Next Implementation Boundary

The first scheduler structs and runnable queue now exist. The next bounded
implementation task may add the first cooperative EL2 kernel-thread context
switch only after it follows the contract below. It should not add timer-driven
preemption, sleep queues, SMP locks, userspace, syscalls, file descriptors,
filesystem, console/TTY, networking, or SSH.

## Implemented Struct Boundary

The first implementation lives in `src/scheduler.rs` and keeps the accepted
shape intentionally narrow:

- `TaskId` is scheduler-local and rejects zero; it is not a process ID.
- `TaskState` currently records `Running`, `Runnable`, and `Blocked` states.
  No blocking, wakeup, sleep queue, or exit policy exists yet.
- `KernelStack` records per-task stack bounds, and `ContextFrame` records the
  saved cooperative switch frame for `x19..x29`, `x30`, and `SP_EL2`.
- `Task::kernel_thread` creates a kernel-thread task with no process owner.
  `ProcessOwnerId` is an optional future extension point only; it does not add
  address spaces, descriptors, credentials, wait state, or other process
  resources.
- `RunnableQueue` is a fixed-capacity FIFO over task IDs for the single boot
  CPU. It is a pure data structure and does not hide interrupt masking or
  preemption policy.
- `SingleCoreScheduler` wraps the runnable queue with state-transition,
  voluntary-yield, and dispatch-switch counters for diagnostics.

The pure scheduler data structures do not call `single_core_irq_mask_save()`
internally. Code that mutates scheduler-owned global state from an
interruptible path must place the accepted short single-core IRQ mask/restore
boundary explicitly around that call-site invariant.

## Cooperative Context-Switch Contract

The first context switch is a current-EL2 cooperative switch between kernel
threads on the boot CPU. It is entered from normal kernel control flow, not from
an IRQ exception frame, and it returns as if a regular function call resumed in
the selected task.

The minimal saved context for this cooperative boundary is the AArch64
callee-saved call state plus the stack and resume address:

- `x19` through `x29`;
- `x30` as the resumed link register, or an equivalent saved program counter
  for a freshly bootstrapped task;
- `SP_EL2` for the task's kernel stack pointer.

The switch primitive may use `x0` through `x18` as caller-saved scratch in
the normal AArch64 procedure-call sense. Kernel code that calls the cooperative
yield boundary must not expect those registers to survive except through the
compiler's ordinary call-preservation rules. The primitive must not change the
exception level, install an EL0 context, or use `ERET` for this first
cooperative switch.

`ContextFrame` now stores the cooperative switch frame directly: `x19..x29`,
`x30`, and `SP_EL2`. Fresh kernel-thread contexts use `x30` as the trampoline
resume address, `x19` as the first bootstrap argument, and `x20` as the
kernel-thread entry function. `KernelStack` continues to own the stack bounds
for that saved frame. A newly created kernel thread starts from a trampoline
whose initial frame uses a 16-byte-aligned `SP_EL2` inside the task's
`KernelStack`.

The switch boundary assumes:

- every switched task is a kernel thread running at EL2 in the shared kernel
  address space;
- `SP_EL2` is 16-byte aligned at every public call boundary;
- the saved stack pointer remains within the task's `KernelStack` bounds;
- no process address space, user stack, descriptor table, or EL0 state belongs
  to the task yet;
- the first implementation is single-core only and cannot migrate tasks.

Scheduler-owned global state must be coherent while the current task, runnable
queue, and saved context pointers are changed. The accepted
`single_core_irq_mask_save()` / `single_core_irq_restore()` primitive protects
only that short boot-CPU invariant: choose the next runnable task, mark the old
and new task states, install or read the two context-frame pointers, and cross
the assembly switch boundary. It is not a general spinlock or SMP policy, and
the masked section must not allocate, format, print, block, or run arbitrary
callbacks.

For the cooperative switch, `PSTATE` is not a schedulable lower-EL user state.
The first switch runs with IRQs masked by the scheduler boundary and resumes
kernel code at EL2. Timer-driven preemption later needs a separate exception
frame contract that captures asynchronous caller-saved state, `ELR_EL2`,
`SPSR_EL2`, interrupt acknowledge/reprogram/EOI ordering, and the rules for
leaving IRQ context before diagnostics or blocking work. Those pieces are
intentionally not part of this cooperative contract.

The first validation should be QEMU-first: create two kernel-thread contexts
with separate stacks and bounded progress counters, cooperatively switch between
them, and print or otherwise record the counters after returning outside the
switch hot path. Pi 5 hardware is not required for this contract because no
board-specific timer, interrupt-controller, UART, or boot behavior changes.

## Cooperative Context-Switch Implementation

The first implementation keeps the boundary QEMU-only behind
`TALOS_QEMU_CONTEXT_SWITCH_SMOKE=1` and `scripts/qemu-context-switch-smoke.sh`.
The AArch64 primitive lives at `talos_aarch64_context_switch`: it saves
`x19..x30` and `SP_EL2` into the outgoing `ContextFrame`, loads the same state
from the incoming `ContextFrame`, and returns through the restored `x30`.

`talos_aarch64_kernel_thread_trampoline` is the bootstrap entry for fresh
kernel-thread contexts. It passes the saved `x19` value as the thread argument
and branches through the saved `x20` entry function. The QEMU smoke uses two
static 16-byte-aligned stacks, two saved contexts, bounded per-task progress
counters, and prints the switch count plus current/runnable task IDs only after
returning to the main kernel context.

This is still direct cooperative switching, not scheduler dispatch. Voluntary
yield, round-robin queue selection, timer preemption, sleeping, SMP, EL0, and
process resources remain deferred to later tasks.

## Voluntary Yield Dispatch

The first scheduler-owned dispatch boundary is
`SingleCoreScheduler::voluntary_yield()`. It is still cooperative: a running
kernel thread explicitly yields from normal kernel control flow, the scheduler
places that task at the back of the single-core runnable queue, dequeues the
next runnable task ID, and increments voluntary-yield and dispatch-switch
counters. The method returns only the next scheduler-local `TaskId`; the
architecture-specific caller still owns the actual `ContextFrame` pointers and
crosses `talos_aarch64_context_switch`.

The dispatch boundary is intentionally narrow:

- the current task must be `Running`;
- at least one peer must already be runnable;
- the runnable queue must have capacity to requeue the yielding task;
- the yielded task becomes `Runnable`, and the caller marks the selected task
  `Running` before switching;
- process-owner metadata remains only an optional future hook and does not add
  address spaces, descriptors, wait state, or process lifetime rules.

The QEMU diagnostic call site wraps only the scheduler-owned mutation in
`single_core_irq_mask_save()` / `single_core_irq_restore()`: current/yielded
task state, queue contents, selected next task, and counters. That masked window
does not allocate, format, print, block, or run callbacks. Diagnostic output is
emitted after switching returns to the main context.

This is not timer-driven preemption. The timer IRQ path still does not call into
the scheduler, no task is switched from asynchronous exception context, and no
sleeping, blocking, wakeup, SMP, EL0, descriptor, filesystem, console/TTY,
networking, or SSH behavior is introduced.

## Timer-Preemption Entry Policy

The accepted preemption-entry checkpoint permits the next bounded task to try a
QEMU-only timer-driven scheduler smoke. The entry policy is intentionally
smaller than a general preemptive scheduler: the EL2 physical timer IRQ may
record a bounded preemption request only after preserving the accepted timer
ordering of acknowledge, INTID classification, tick accounting, next-deadline
programming, and EOI. Context switching and diagnostics must happen outside the
IRQ hot path.

The IRQ handler must not allocate, format, print, block, sleep, walk arbitrary
queues, or run callbacks. Scheduler-owned global state remains protected by
short boot-CPU single_core_irq_mask_save() / single_core_irq_restore() windows
around current-task, runnable-queue, selected-task, context-frame, and counter
updates. Those windows are still not SMP locks, blocking locks, lower-EL policy,
or process-resource policy.

The first smoke remains EL2 kernel-thread only. It may prove that timer ticks
drive progress between runnable kernel threads, but it must not add sleep
queues, wakeups, SMP run queues, task migration, EL0 state, process resources,
descriptors, filesystem, console/TTY, networking, or SSH. Pi 5 hardware
preemption evidence is deferred until after QEMU proves the shape or a separate
serialized hardware task is planned.

## Timer-Driven QEMU Preemption Smoke

The first timer-driven scheduler smoke is accepted for QEMU virt only. It keeps
the same EL2 physical timer and GICv2 path as the earlier timer smokes, but the
IRQ handler only records the bounded tick and a preemption-request counter after
INTID 26 classification. It then reprograms CNTHP_CVAL_EL2 and writes
GICC_EOIR before returning through the saved IRQ frame. The IRQ hot path still
does not allocate, format, print, block, sleep, walk scheduler queues, or call
the scheduler.

The diagnostic kernel threads observe the pending request after returning from
IRQ context. The call site then masks IRQs for a short boot-CPU critical
section, calls `SingleCoreScheduler::timer_preempt()`, updates current/runnable
task state and counters, restores the previous IRQ mask state, and crosses the
existing cooperative AArch64 context-switch primitive. This proves timer-driven
progress without explicit voluntary-yield calls while keeping the actual
scheduler mutation and diagnostic reporting outside the IRQ hot path.

This remains a single-core EL2 kernel-thread proof. It does not define a real
quantum policy, preemption-disable counters, async exception-frame switching,
sleep queues, wait queues, SMP run-queue locking, lower-EL state, process
resources, descriptors, filesystem, console/TTY, networking, or SSH.

The Pi 5 hardware diagnostic uses the same accepted boundary behind
`TALOS_RPI5_TIMER_PREEMPTION_DIAGNOSTIC`. The physical run reached two
kernel threads, six timer ticks, six preemption requests, six handled
preemptions, six dispatch switches, zero voluntary yields, INTID 26, and
`rpi5-timer-preemption-smoke: PASS`. Its IRQ handler still only
acknowledges/classifies INTID 26, records tick and preemption-request counters,
reprograms `CNTHP_CVAL_EL2`, writes `GICC_EOIR`, and returns; scheduler
mutation and diagnostic output stay after IRQ return.

## Consolidated Scheduler/Preemption Contract

The accepted production contract after the QEMU and Pi 5 timer-preemption
proofs is deliberately smaller than a complete preemptive scheduler:

- the scheduler owns task IDs, task state, per-task kernel stacks, saved
  cooperative context frames, the single boot-CPU runnable queue, and dispatch
  counters;
- the EL2 physical timer IRQ path owns only acknowledge/classification,
  monotonic tick accounting, bounded preemption-request accounting,
  `CNTHP_CVAL_EL2` reprogramming, and `GICC_EOIR`;
- timer-driven switching is performed by kernel-thread code after IRQ return
  when it observes a pending request;
- scheduler mutation remains inside short
  `single_core_irq_mask_save()` / `single_core_irq_restore()` windows that
  update current/runnable task state, dispatch counters, and context-frame
  pointers before crossing `talos_aarch64_context_switch`.

Those short masked windows are boot-CPU critical sections, not general locks.
They do not provide SMP mutual exclusion, blocking or sleepable locking,
preemption-disable nesting, lower-EL interrupt policy, or cross-core memory
ordering. They must stay bounded and must not allocate, format, print, block,
sleep, run callbacks, or walk unrelated queues.

The retained diagnostic surfaces are owned by Phase 4 validation:

- `TALOS_QEMU_CONTEXT_SWITCH_SMOKE` and
  `scripts/qemu-context-switch-smoke.sh` keep the raw cooperative switch
  primitive covered until a non-diagnostic kernel-thread launcher replaces the
  smoke.
- `TALOS_QEMU_SCHEDULER_YIELD_SMOKE` and
  `scripts/qemu-scheduler-yield-smoke.sh` keep voluntary-yield dispatch
  covered until the scheduler has a regular in-kernel yield path.
- `TALOS_QEMU_TIMER_PREEMPTION_SMOKE` and
  `scripts/qemu-timer-preemption-smoke.sh` keep the fast substitute proof for
  timer-driven dispatch and should remain a regression gate through Phase 4
  closeout.
- `TALOS_RPI5_TIMER_IRQ_DIAGNOSTIC`,
  `TALOS_RPI5_TIMER_PREEMPTION_DIAGNOSTIC`,
  `scripts/rpi5-timer-irq-diagnostic-image.sh`, and
  `scripts/rpi5-timer-preemption-diagnostic-image.sh` are serialized hardware
  evidence surfaces. Revisit or remove them after Phase 4 closeout once their
  contracts are covered by ordinary boot diagnostics or by a later local
  console diagnostic command.

The following remain explicit deferrals: real quantum policy,
preemption-disable counters, switching directly from an asynchronous exception
frame, sleep and wakeup queues, SMP run-queue locking, task migration, lower-EL
state, process address spaces, descriptor tables, POSIX process lifetime, and
all filesystem, console/TTY, networking, and SSH behavior.

## Phase 6.1 Per-Core Boundary

Phase 6.1 now has a separate `src/smp.rs` per-core ownership boundary for
secondary-core bring-up. That boundary records possible core count, secondary
stack slot ownership, MPIDR/logical identity, and the boot-time lifecycle
through `handoff-ready`.

The scheduler remains single-core. `src/smp.rs` does not provide scheduler
locks, runnable queues, migration, load balancing, cross-core preemption,
sleep/wakeup behavior, IPIs, or process resources. Secondary cores that reach
`handoff-ready` must still park or run a separately planned bounded workload
until a later supervisor task accepts SMP-safe scheduler primitives.

## Phase 6.2 Primitive Boundary

`src/smp_sync.rs` now owns the first narrow SMP-safe primitive core:
`SpinLock<T>`, `SpinLockGuard`, AArch64 `lock_irqsave()` composition, and
`smp_full_barrier()`. The lock uses acquire ordering to enter and release
ordering to unlock. The IRQ-save wrapper keeps the accepted ordering rule
explicit: save/mask local IRQ state first, acquire the SMP lock second, release
the lock first, then restore the saved IRQ state.

This does not make `src/scheduler.rs` SMP-safe. Scheduler runnable queues,
current-task state, task migration, load balancing, IPIs, cross-core wakeups,
and multi-core preemption remain deferred until a later task wires shared
scheduler state to accepted synchronization and proves it under contention.

The primitive also keeps cache maintenance out of the generic lock. Early
boot-time secondary-core state that needs explicit clean/invalidate operations
must keep using a named cache-sharing boundary rather than assuming the lock
solves non-coherent publication.

Milestone 6.2 is closed with both QEMU substitute evidence and serialized Pi 5
hardware proof for the generic lock/cache-coherence diagnostic. That proof
accepts `SpinLock<T>` as a primitive, not the scheduler's data structures:
the scheduler still needs a separate source inventory for shared run-queue
ownership, lock placement, cross-core wakeups/IPIs, and per-core
timer/preemption interactions before any multi-core scheduling implementation.

## Phase 6.3 Scheduler Migration Readiness

The first Milestone 6.3 migration boundary is documented in
`docs/src/project/phase6-scheduler-migration-readiness-source-inventory.md`.
The selected first slice is CPU-local scheduler ownership: keep each runnable
queue owned by one logical CPU, introduce an explicit per-core scheduler state
wrapper, and leave CPU 0 as the only production scheduler owner until a later
task accepts secondary scheduler participation.

This boundary deliberately does not add shared run queues, migration, IPIs,
cross-core wakeups, or secondary-core production scheduling. Local timer
preemption may mutate only the current CPU's local scheduler state, under the
existing short local IRQ-mask boundary and outside the IRQ hot path. A purely
CPU-local run queue should not take an SMP lock in this first slice; the
accepted `SpinLock<T>` is reserved for later shared scheduler metadata,
cross-core wake lists, migration queues, or global task state that a future
task names and proves explicitly.

The lock-ordering rule for later shared scheduler state is local IRQ mask
first, then SMP lock acquisition; release the SMP lock before restoring local
IRQ state. Scheduler locks must not be held across
`talos_aarch64_context_switch`, and code must not allocate, format, print,
poll UART input, dispatch diagnostic commands, block, sleep, or run arbitrary
callbacks while holding scheduler locks.

## Phase 6.3 Per-Core Scheduler State

`src/scheduler.rs` now exposes the first CPU-local scheduler ownership data
boundary:

- `LogicalCpuId` records the logical CPU that owns a local scheduler state.
- `SchedulerCoreRole` separates `BootCpuProduction`,
  `SecondaryProductionDiagnostic`, and `SecondaryDeferred`. Only the boot CPU
  and the explicit secondary diagnostic role can enter production dispatch in
  this slice.
- `PerCoreScheduler` wraps a local `SingleCoreScheduler`, its owning logical
  CPU, role, and current-task slot. `local_scheduler_mut()` rejects callers
  from another CPU, and `production_scheduler_mut()` additionally rejects
  deferred secondary owners.

This is a data boundary, not scheduler migration. `SingleCoreScheduler`
continues to own the same fixed FIFO runnable queue and dispatch counters, and
existing cooperative and timer-preemption diagnostics remain boot-CPU evidence.
Purely CPU-local queues still do not take `SpinLock<T>`; future shared
scheduler structures must name their lock, IRQ-mask ordering, and validation
evidence before implementation.

The QEMU substitute diagnostic behind
`TALOS_QEMU_PER_CORE_SCHEDULER_OWNERSHIP_SMOKE` starts four QEMU virt CPUs
through the accepted PSCI path and has each logical core publish a bounded
per-core scheduler ownership report. It proves distinct owner IDs for logical
CPUs 0 through 3, keeps secondary roles deferred from production dispatch, and
exercises local runnable/progress accounting without shared run queues,
migration, IPIs, or cross-core wakeups. Diagnostic output is emitted after the
bounded local work and outside hot lock/IRQ paths.

The QEMU substitute diagnostic behind
`TALOS_QEMU_PRODUCTION_SECONDARY_DISPATCH_SMOKE` extends that evidence to the
first production secondary-dispatch slice. Logical CPUs 1, 2, and 3 use
`SecondaryProductionDiagnostic` to seed only CPU-local diagnostic tasks,
dispatch three bounded local tasks each, publish stable current-task and
dispatch counters, and reject cross-owner local queue/dispatch attempts. This
does not add shared run queues, global task lookup, task migration, load
balancing, secondary timer preemption, or a Pi 5 hardware claim.

## Phase 6.3 Cross-Core Wakeup and IPI Readiness

The cross-core wakeup/IPI source inventory accepts a raw signal-delivery plan,
not scheduler migration. The first implementation must prove SGI delivery
before any scheduler task is remotely enqueued or migrated.

The scheduler-facing wakeup contract remains:

- wakeups target scheduler tasks, not POSIX processes;
- a local wake may eventually enqueue onto the current CPU's local runnable
  queue under that CPU's local scheduler rules;
- a remote wake may only publish a bounded wake request and send an IPI after a
  later task accepts the remote wake-list or remote-enqueue ownership model;
- an IPI handler may acknowledge/classify the SGI, record bounded per-core
  wake-pending evidence, EOI, and return;
- remote wake handling must not mutate another CPU's local runnable queue
  directly until a separate task names the shared lock, memory ordering, and
  validation evidence.

If later shared scheduler wake state is introduced, the lock-ordering rule is
local IRQ mask first, then SMP lock acquisition; release the SMP lock before
restoring local IRQ state. Scheduler locks must not be held across
`talos_aarch64_context_switch`, and neither IPI context nor scheduler-locked
sections may allocate, format, print, poll UART input, dispatch diagnostic
commands, block, sleep, migrate tasks, or run arbitrary callbacks.

The QEMU SGI/IPI delivery smoke is accepted as raw signal delivery only. It
uses SGI INTID 1, maps logical CPUs 1, 2, and 3 to GICD_SGIR target-list bits
0x02, 0x04, and 0x08, and proves each secondary receives and EOIs exactly one
diagnostic SGI. The next proof in the split sequence is a serialized Pi 5 raw
SGI/IPI hardware proof before any production scheduler wakeup uses SGIs on
hardware. Shared run queues, global task lookup, task migration, load
balancing, work stealing, sleep/wakeup queues, userspace, descriptors,
filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt
ownership, and DMA policy remain deferred.

The Pi 5 raw SGI/IPI hardware proof is now accepted as interrupt-delivery
evidence. It proves that SGI INTID 1 can be delivered and EOI'd by logical
CPUs 1, 2, and 3 on the physical GIC-400 path after the Pi 5 IRQ dispatcher
includes the cross-core IPI proof handler. It still does not authorize direct
remote enqueue, shared run queues, production secondary scheduler dispatch, or
task migration.

## Phase 6.3 Remote Wake-Request Ownership

The remote wakeup ownership inventory selects a bounded per-target remote
wake-request list as the first scheduler-facing IPI model. A remote sender may
publish a request for a scheduler `TaskId` into the target CPU's wake-request
list and then signal the target with SGI INTID 1. The target CPU owns request
consumption and any future local scheduler effect.

This is intentionally not direct remote enqueue. Another CPU must not mutate a
target's `RunnableQueue` or `current_task` slot from the outside. CPU 0 remains
the only production scheduler owner in this slice; QEMU remote-wakeup evidence
may use secondary CPUs as diagnostic owners for request-consumption counters,
not as production dispatch owners.

The publication path must mask local IRQs, acquire the target wake-request
lock, insert or coalesce the bounded request, release the lock, restore the
saved IRQ state, publish the request before signaling when a barrier is needed,
and only then send the SGI. The target drain path runs outside IPI context,
masks local IRQs, acquires its own wake-request lock, drains or snapshots
bounded requests, releases the lock, and restores local IRQ state.

IPI context remains a hot path: acknowledge, classify, record bounded
wake-pending evidence, EOI, and return. It must not take scheduler locks, walk
runnable queues, allocate, format, print, poll UART input, dispatch diagnostic
commands, block, sleep, migrate tasks, or cross
`talos_aarch64_context_switch`.

Duplicate pending wakes for the same target `TaskId` are coalesced: the first
request remains pending, the duplicate does not consume another slot, and the
implementation may count the duplicate for evidence. Queue-full, invalid
target CPU, invalid task ID, and self-targeted remote requests must be explicit
outcomes rather than silent scheduler mutations.

The QEMU remote wake-request smoke and the serialized Pi 5 proof are now
accepted as scheduler-facing IPI evidence. `RemoteWakeQueue` is a bounded
target-owned request list over scheduler-local `TaskId` values: remote
publication may insert or coalesce a request, but target-owned consumption
happens after IPI context and does not mutate a remote `RunnableQueue`. The
diagnostics prove CPU 0 can publish requests for logical CPUs 1, 2, and 3,
coalesce a duplicate CPU 1 wake, signal each target with SGI INTID 1, and let
each target observe, EOI, and consume its own request with zero errors.
Cross-owner local queue mutation and secondary production dispatch remain
rejected. Shared run queues, local runnable transitions from remote requests,
task migration, production secondary scheduler dispatch, multi-core
preemption, Phase 7, filesystem, networking, SSH, shell behavior, RP1/PCIe,
and DMA behavior remain deferred.

## Phase 6.3 Target-Owned Wake Consumption

The next accepted boundary converts a consumed remote wake request into a local
wake action without breaking CPU-local scheduler ownership. The rule is
target-owned throughout: a remote CPU may only publish or coalesce a bounded
request and signal the target; only the target CPU may consume that request and
mutate its own local scheduler state.

The local wake action runs outside IPI context. IPI context remains limited to
acknowledge, classify, record bounded wake-pending state, EOI, and return. A
target drain/wake service may then run from normal kernel control flow on the
target CPU. That service first drains or snapshots its owned
`RemoteWakeQueue`, then applies local scheduler wake rules to target-owned
task state. It must not hold the wake-request lock while walking or mutating a
local scheduler queue.

The first local wake precondition is intentionally narrow: the request names a
scheduler-local `TaskId` that the target CPU's local diagnostic task table
already owns, and that task is `Blocked`. The only accepted transition is
`Blocked -> Runnable` on the target's local scheduler. A request for a
running, already-runnable, unknown, wrong-owner, or nonlocal task must be an
explicit diagnostic outcome and must not enqueue anything. Duplicate pending
remote requests for the same task remain coalesced by `RemoteWakeQueue`; once
one request is consumed, the local wake service must still reject duplicate
local enqueue of a task that is no longer blocked.

Lock and context ordering for the first implementation is:

- sender side: mask local IRQs, acquire the target wake-request lock, publish
  or coalesce the bounded request, release the lock, restore local IRQ state,
  publish a barrier if needed, then send SGI INTID 1;
- target IPI side: acknowledge/classify/record/EOI only;
- target drain side: outside IPI context, mask local IRQs, acquire the owned
  wake-request lock, drain or snapshot bounded requests, release the lock,
  restore local IRQ state, then enter the target-owned local scheduler mutation
  boundary for any `Blocked -> Runnable` transition;
- no scheduler lock is held across `talos_aarch64_context_switch`, printing,
  UART polling, diagnostic command dispatch, allocation, blocking, sleeping,
  migration, or arbitrary callbacks.

The QEMU implementation proof is now accepted. `PerCoreScheduler` exposes the
target-owned
`wake_blocked_local_task_from_remote_request()` boundary: after a target has
drained its owned `RemoteWakeQueue` outside IPI context, the target may use
that consumed request to transition exactly one matching local `Blocked` task
to `Runnable`. The method rejects wrong-owner callers, wrong-target requests,
task-ID mismatches, non-blocked tasks, duplicate local runnable entries, and
full local queues without enabling production secondary dispatch.

`scripts/qemu-remote-wake-to-local-runnable-smoke.sh` extends the accepted
remote wake-request diagnostic. It proves CPU 0 publication, duplicate
coalescing, SGI INTID 1 observation/EOI, target-owned request consumption,
zero queue length after drain, one local blocked-to-runnable transition on
each target scheduler, duplicate-local-enqueue rejection, cross-owner
scheduler mutation rejection, and deferred production dispatch for logical
CPUs 1, 2, and 3.

The serialized Pi 5 proof carries the same invariant to hardware. The accepted
run proves request publication for logical CPUs 1, 2, and 3, duplicate request
coalescing, SGI INTID 1 observation/EOI, target-owned request drain, local
Blocked -> Runnable transitions for target-owned diagnostic tasks, duplicate
local enqueue rejection, drained queues, cross-owner rejection, and deferred
production secondary dispatch.

Shared run queues, global task lookup, task migration, load balancing,
production secondary scheduler dispatch, multi-core preemption, Phase 7,
filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt
ownership, and DMA policy remain deferred.

## Phase 6.3 Remote Wakeup Scheduler Integration Closeout

The remote wakeup scheduler-integration closeout accepts the current diagnostic
bridge from raw SGI delivery to a target-owned local runnable transition. It
does not make the scheduler topology shared. Remote CPUs may publish bounded
requests and signal with SGI INTID 1, but local runnable state remains owned by
the target CPU.

Talos is ready for a production secondary scheduler dispatch source inventory
and contract. That next task must define how secondary cores can leave
diagnostic dispatch and run production scheduler work while preserving the
existing IPI, wake-drain, context-switch, timer/preemption, console/output, and
failure-diagnostic boundaries. It should not implement production secondary
dispatch.

## Phase 6.3 Production Secondary Dispatch Contract

The production secondary scheduler dispatch source inventory is accepted as a
contract only. The first acceptable implementation slice may enable secondary
CPUs to dispatch explicitly seeded CPU-local diagnostic kernel threads from
normal secondary control flow. Each participating logical CPU owns its local
PerCoreScheduler, local current-task slot, local runnable queue, diagnostic
task state, and dispatch counters.

This is still a CPU-local topology. A remote CPU may publish or coalesce a
bounded wake request and signal with SGI INTID 1, but only the target CPU may
drain its own request queue and mutate its own local scheduler state. The IPI
handler remains limited to acknowledge/classify/record/EOI and must not run a
dispatch loop, walk runnable queues, format output, allocate, poll UART input,
dispatch diagnostic commands, block, sleep, migrate tasks, or cross
talos_aarch64_context_switch.

Secondary production dispatch may start only after the accepted secondary-core
bring-up, stack ownership, cacheable-MMU handoff, raw SGI delivery, and
target-owned remote wake evidence remain intact. The boot CPU keeps its
existing production scheduler behavior. The accepted QEMU and Pi 5 proof
surfaces keep secondary dispatch behind explicit diagnostic validation flags;
normal boot behavior is not yet a shared multi-core scheduler topology.

Local timer/preemption state remains CPU-local in this slice. A timer IRQ may
record local state according to the accepted preemption-entry policy, but this
contract does not accept switching directly from asynchronous exception
context or multi-core preemption. No scheduler lock may be held across
talos_aarch64_context_switch, printing, UART polling, diagnostic command
dispatch, allocation, blocking, sleeping, migration, or arbitrary callbacks.

The production secondary dispatch core is accepted at QEMU substitute and
serialized Pi 5 hardware evidence levels for CPU-local diagnostic kernel
threads. Shared run queues, global task lookup, remote enqueue queues, task
migration, load balancing, work stealing, multi-core preemption,
lower-EL/userspace, descriptors, filesystem, networking, SSH, shell behavior,
RP1/PCIe, UART interrupt ownership, and DMA/cache driver policy remain
deferred.

`SchedulerCoreRole::SecondaryProductionDiagnostic` is the only secondary role
that may pass the production-dispatch gate; `SecondaryDeferred` still rejects
production dispatch. `PerCoreScheduler::dispatch_cpu_local_diagnostic_task()`
requires the requesting logical CPU to own the scheduler, requires the selected
task to be the front of that CPU's local runnable queue, requires that task to
still be `Runnable`, then records it as the per-core current task and increments
the local production-dispatch counter. Wrong-owner, deferred-role,
empty-queue, mismatched-task, and non-runnable-task cases are explicit errors
that leave local scheduler state intact.

This accepted core does not add any remote mutation path. Remote wake requests
remain bounded signals only, and target-owned wake consumption remains the only
path from remote request to local runnable state. The accepted QEMU production
secondary dispatch smoke and serialized Pi 5 proof show logical CPUs 1, 2, and
3 entering the diagnostic production role from secondary normal control flow,
dispatching three CPU-local diagnostic tasks each, publishing current-task and
counter snapshots, and rejecting cross-owner local queue and production
dispatch attempts.

## Phase 6.3 Shared Scheduler Metadata Contract

The shared scheduler metadata source inventory is accepted as a contract only.
The first metadata slice may name scheduler tasks across cores, but it must not
turn the scheduler into a shared run-queue topology. The minimal record is a
TaskId, owning LogicalCpuId, TaskState, optional ProcessOwnerId, kernel-stack
bounds, owner-local current/runnable membership, and a generation or counter
field sufficient to reject stale diagnostic snapshots.

Ownership remains CPU-local. The owning CPU is the only writer for its Task,
PerCoreScheduler, current_task, RunnableQueue, and target-owned remote wake
consumption state. A remote CPU may inspect metadata or publish a bounded wake
request, but it must not mutate another CPU's local scheduler state, force a
remote enqueue, steal work, migrate a task, or dispatch a remote runnable task.

IPI and timer IRQ context remain bounded observation paths. They may
acknowledge, classify, record bounded state, EOI, and return; they must not
allocate, format, print, poll UART input, dispatch diagnostic commands, walk
unbounded metadata, mutate runnable queues, migrate tasks, or cross
talos_aarch64_context_switch.

The first implementation task should add only local-owner metadata types and
APIs for task identity/owner/state snapshots. Shared run queues, global task
lookup with mutation authority, remote enqueue queues, task migration, load
balancing, work stealing, multi-core preemption, Phase 7, filesystem,
networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, and
DMA/cache-coherent driver policy remain deferred.

The shared scheduler metadata core now implements that first slice in
src/scheduler.rs. SchedulerTaskSnapshot records the scheduler-local task ID,
owning logical CPU, task state, optional process owner, kernel-stack bounds,
owner-local current/runnable membership, and a generation number.
SharedSchedulerMetadata is a bounded table for owner-published snapshots, and
SharedSchedulerMetadataLock names the accepted SpinLock boundary for callers
that share that table across CPUs. The table does not contain a runnable queue
and does not grant mutation authority over another CPU's PerCoreScheduler.

Only the owner CPU can register or refresh a task snapshot for its local
PerCoreScheduler; wrong-owner publication is rejected before the metadata table
changes. Lookup is read-only, duplicate task registration and unknown task IDs
are explicit outcomes, invalid logical CPU owners are rejected by the table's
CPU-capacity boundary, and generation-qualified lookups reject stale snapshots.
This is enough to name CPU-local diagnostic tasks across cores for later
proofs, but it is not shared dispatch, remote enqueue, task migration, load
balancing, or multi-core preemption.

The shared scheduler metadata invariant is now accepted at both QEMU substitute
and serialized Pi 5 hardware evidence levels. The QEMU smoke and Pi 5 proof
both show logical CPUs 0 through 3 publishing task IDs 101, 201, 301, and 401,
owner-task lookup and boot-task lookup succeeding, cross-owner local scheduler
mutation and cross-owner metadata publication rejected, local runnable queues
preserved, final-metadata-len=4, errors=0, and PASS classification. This
proves the bounded owner-published metadata table across physical cores, not a
shared run queue, remote enqueue queue, task migration, load balancing, or
multi-core preemption.

## Phase 6.3 CPU-Local Scheduler Service Boundary

The CPU-local scheduler service boundary is accepted as the next
productionization contract after the shared metadata closeout. It orders the
accepted diagnostic slices into one normal-control-flow service, but it does
not add shared run queues, remote enqueue queues, task migration, load
balancing, work stealing, multi-core preemption, Phase 7, filesystem,
networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

The service must run on the owning logical CPU and mutate only that CPU's
PerCoreScheduler, local task table, target-owned RemoteWakeQueue, and
owner-published scheduler metadata. IPI context remains limited to bounded
acknowledge/classify/record/EOI work; timer IRQ context may record local
preemption state, but scheduler switching still happens outside asynchronous
exception context.

The accepted order is:

1. establish the requester as the current logical CPU and enter from normal
   kernel control flow;
2. drain target-owned remote wake requests outside IPI context;
3. convert matching local blocked tasks to local runnable state through local
   scheduler rules;
4. handle pending local timer-preemption requests after the wake drain so a
   just-woken local task can participate in dispatch;
5. enter CPU-local dispatch only through the owner scheduler, with secondary
   CPUs still limited to SecondaryProductionDiagnostic;
6. refresh owner-published scheduler metadata after local state, current-task,
   runnable-queue, or dispatch-counter mutations;
7. return without holding scheduler locks across context switch, printing,
   UART polling, diagnostic command dispatch, allocation, blocking, sleeping,
   migration, or arbitrary callbacks.

The target-independent implementation is `CpuLocalSchedulerService` in
`src/scheduler.rs`. Its `run_cycle` entry point consumes one target-owned
remote wake request, applies the matching local wake transition, handles an
optional pending timer-preemption request, dispatches through the owner
`PerCoreScheduler` when timer preemption did not already select the next task,
and refreshes owner-published metadata after the local mutations. The accepted
unit/QEMU evidence covers service order and explicit remote-wake, dispatch,
timer, and metadata error boundaries.

This implementation does not create shared scheduler topology or broaden the
secondary diagnostic dispatch role into general multi-core scheduling.

## Phase 6.3 Secondary Scheduler Service Loop Boundary

The secondary scheduler service-loop source inventory is accepted as the next
CPU-local productionization contract after the CPU-local scheduler service
closeout. It defines where an already-started secondary CPU may run the
accepted `CpuLocalSchedulerService` from normal kernel control flow without
creating shared run queues, remote enqueue queues, task migration, load
balancing, work stealing, or multi-core preemption.

The loop entry precondition is the accepted secondary handoff state:
`src/smp.rs` has established logical CPU identity, stack ownership,
`CoreLifecycle::HandoffReady`, and normal Rust control flow. Pi 5 proof paths
also rely on the accepted secondary cacheable-MMU handoff before physical
scheduler claims. The service loop does not own secondary bring-up itself.

One secondary loop iteration is owner-local:

1. identify the requester as the owning `LogicalCpuId`;
2. observe bounded pending work recorded by IPI or timer paths;
3. call `CpuLocalSchedulerService::run_cycle` on that CPU's local scheduler,
   target-owned remote wake queue, local task state, and metadata table;
4. dispatch only through the owner `PerCoreScheduler` and the accepted
   `SecondaryProductionDiagnostic` role;
5. refresh owner-published metadata after local mutations;
6. return to the loop or an explicit idle/no-work point without holding
   scheduler locks across context switch, printing, UART polling, diagnostic
   command dispatch, allocation, blocking, sleeping, migration, or arbitrary
   callbacks.

IPI context remains limited to acknowledge, classify, record bounded pending
state, EOI, and return. Timer IRQ context remains limited to recording a local
preemption request. Scheduler mutation, remote wake drain, dispatch, and
metadata refresh belong in the normal secondary service loop, not in interrupt
context.

The existing QEMU and Pi 5 secondary workload, remote wake, production
secondary dispatch, and shared metadata scripts remain diagnostic proof entry
points. The service-loop boundary names the productionization owner for those
accepted behaviors but deliberately keeps `SecondaryProductionDiagnostic` as
the only accepted secondary production role until a later task defines a
general non-diagnostic runtime role.

The target-independent implementation is `SecondarySchedulerServiceLoop` in
`src/scheduler.rs`. Its `run_once` entry point is a normal-control-flow
secondary adapter around `CpuLocalSchedulerService::run_cycle`: it rejects
boot-CPU use, cross-owner requests, and deferred secondary roles before
consuming target-owned wake state, then runs one owner-local cycle and reports
whether the cycle observed remote wake state, pending timer-preemption state,
dispatch intent, and actual local work. The adapter does not create a target
idle primitive, shared scheduler topology, remote enqueue queue, migration,
load balancing, work stealing, or multi-core preemption policy.

The retained QEMU substitute gate is
scripts/qemu-secondary-scheduler-service-loop-smoke.sh. It starts secondary
cores through the accepted PSCI/QEMU path and runs one service-loop cycle per
secondary with owner-local metadata, remote wake drain, local diagnostic
dispatch, no-work refresh, cross-owner rejection, deferred-role rejection, and
explicit no shared-queue or migration behavior. The gate uses an optimized
QEMU build because the current 4 KiB diagnostic secondary stacks are too small
for the debug build of this proof workload; this is a gate property, not a
runtime stack-size claim.

The retained serialized Pi 5 gate is
scripts/rpi5-secondary-scheduler-service-loop-boot-tree.sh with
TALOS_RPI5_SECONDARY_SCHEDULER_SERVICE_LOOP_PROOF. It uses the accepted
secondary cacheable-MMU handoff before running the same owner-local
service-loop invariant on physical logical CPUs 1, 2, and 3. Accepted lab
evidence reports remote wake drain, local dispatch, no-work metadata refresh,
cross-owner rejection, deferred-role rejection, local-queue preservation,
classification=pi5-secondary-scheduler-service-loop-complete, and PASS. This
remains a diagnostic proof surface, not a general secondary scheduler role or
shared scheduler topology.

## Phase 6.3 Shared Run-Queue And Migration Contract

The shared run-queue and migration contract is accepted in
docs/src/project/phase6-shared-runqueue-migration-contract.md. It is the
implementation boundary that follows the accepted source inventory before any
shared scheduler topology code may start.

The contract keeps task mutation single-owner at every instant. A task has one
owning PerCoreScheduler; a remote CPU may publish enqueue or migration intent
only through an accepted shared structure, and the destination owner consumes
that handoff from normal scheduler control flow before installing the task into
its local RunnableQueue. Remote wake remains separate: RemoteWakeQueue still
carries requests for already target-owned blocked tasks, not arbitrary remote
enqueue or migration work.

The first shared scheduler lock order is local IRQ save/mask, then SMP
scheduler lock acquisition, then SMP lock release, then local IRQ restore. The
accepted SpinLock lock_irqsave shape is the model for this ordering. No
scheduler lock may be held across context switch, printing, UART polling,
diagnostic command dispatch, allocation, blocking, sleeping, IPI send loops,
timer reprogramming loops, migration callbacks, or hardware lab waits.

Acquire/release ordering on the shared scheduler lock is the primary
publication boundary for shared run-queue or migration entries. Producers must
publish complete entries before releasing the lock; consumers must acquire the
same lock before removing entries. smp_full_barrier is reserved for named
handoff points that need an explicit full-system ordering edge outside the
ordinary lock pair.

The accepted migration states are owner-local, migration-reserved,
shared-queued, destination-enqueued, and migration-rejected. A task must never
be on two local runnable queues, and running-task migration remains deferred
until multi-core preemption and asynchronous context capture are separately
accepted. Failure reporting must distinguish stale generation, duplicate
membership, unknown task, wrong owner, invalid CPU, full shared queue, full
destination queue, unsupported task state, and deferred secondary role.

This contract does not accept load balancing, work stealing, fairness or
affinity policy, migration of currently running tasks, multi-core timer
preemption, a general non-diagnostic secondary runtime role, Phase 7,
filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver behavior. The next bounded implementation may
add only the shared run-queue core and tests if it stays inside this contract.

## Phase 6.3 Shared Run-Queue Core

The first target-independent shared run-queue core now lives in
`src/scheduler.rs`. It adds `SharedRunQueue`, `SharedRunQueueEntry`,
`MigrationState`, `SharedRunQueueError`, and `SharedRunQueueLock` as the
bounded owner-transfer surface between existing `PerCoreScheduler` owners.
The implementation does not choose targets, balance load, steal work, send
IPIs, dispatch from interrupt context, or add a general secondary runtime
role.

`SharedRunQueue::publish_migration` is the source-owner handoff boundary. It
requires a requester that matches the source scheduler owner, a valid
destination CPU, a fresh owner-published metadata generation, a runnable task
that is present on the source-local queue, and available shared-queue capacity.
On success it removes the task from the source-local `RunnableQueue`, records
the explicit `MigrationReserved -> SharedQueued` transition in the returned
report, and leaves metadata source-owned until the destination consumes the
entry.

`SharedRunQueue::consume_for_destination` is the destination-owner boundary.
It requires a requester that matches the destination owner, an accepted
production-capable scheduler role, matching task identity, fresh source
metadata, no duplicate destination-local runnable membership, and available
destination-local queue capacity. On success it enqueues the task into the
destination-local `RunnableQueue`, updates shared scheduler metadata to the
destination owner, removes the shared entry, and reports the
`DestinationEnqueued` transition.

The core reports deterministic errors for invalid CPU IDs, wrong source or
destination owners, source/destination owner equality, unknown task metadata,
stale metadata generations, metadata owner mismatch, duplicate shared entries,
duplicate destination-local runnable membership, full shared or local queues,
source tasks that are not queued, running-task migration deferral,
blocked-task migration rejection, deferred secondary roles, and task mismatch.
These are unit-tested target-independent invariants; QEMU and Pi 5 proof
surfaces remain separate scheduler-validation tasks.

## Phase 6.3 QEMU Shared Run-Queue Migration Proof

The QEMU substitute proof lives behind `qemu_shared_runqueue_migration` and
`scripts/qemu-shared-runqueue-migration-smoke.sh`. It is deliberately narrower
than the SMP metadata and service-loop diagnostics: it runs on the QEMU boot
CPU as a deterministic substitute for the target-independent shared
run-queue/migration core.

The diagnostic constructs a source `PerCoreScheduler`, a destination
production-diagnostic `PerCoreScheduler`, one shared metadata table, one
`SharedRunQueue`, and task 107. It then publishes the task from source owner 0
to destination owner 1 through `SharedRunQueue::publish_migration`, proving
that the source-local runnable queue loses the task and the shared queue holds
the `SharedQueued` entry. It consumes the entry through
`SharedRunQueue::consume_for_destination`, proving destination-local enqueue,
shared queue removal, and metadata owner transfer to owner 1.

The accepted transcript reports
classification=qemu-shared-runqueue-migration-complete and PASS. This is QEMU
substitute evidence only; it does not claim physical Pi 5 behavior, target
selection, load balancing, work stealing, running-task migration, multi-core
timer preemption, a general secondary runtime role, Phase 7, filesystem,
networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver behavior.

## Phase 6.3 Pi 5 Shared Run-Queue Migration Proof

The serialized Pi 5 proof lives behind
TALOS_RPI5_SHARED_RUNQUEUE_MIGRATION_PROOF and the focused
scripts/rpi5-shared-runqueue-migration-image.sh plus
scripts/rpi5-shared-runqueue-migration-boot-tree.sh staging helpers. It is a
diagnostic validation surface for the accepted SharedRunQueue core, not a
general multi-core scheduler role.

The accepted local1 evidence used archive SHA256
4d5c8e2666d64ddcc5df7b49c8d3a541b01634800917616cbdb88404a54630d5, kernel
SHA256 98a9cb87bcb89c38b19a097a05695a136aaf6b0eb911ec03c3b0c17eeab6a394, and
kernel size 102,952 bytes. TFTP evidence tied the boot to
da591740/kernel_2712.img fetched from 10.42.1.4 at 102,952 bytes before
restore. Cursor-valid serial reported all four physical-core participants
completing the implemented shared run-queue/migration invariant with
participants=4, expected=4, errors=0, lock-available=true,
classification=pi5-shared-runqueue-migration-complete, and PASS.

The proof does not accept target selection, load balancing, work stealing,
remote reschedule, running-task migration, multi-core timer preemption, a
general secondary runtime role, Phase 7, filesystem, networking, SSH, shell
behavior, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver behavior.

## Phase 6.3 Shared Run-Queue Migration Closeout

The shared run-queue/migration closeout is accepted in
docs/src/project/phase6-shared-runqueue-migration-closeout-checkpoint.md. It
reconciles the source inventory, contract, cfg-routing precursor, core
implementation, QEMU substitute proof, Pi 5 hardware proof, retained
diagnostics, and remaining risks.

The accepted productized boundary is the target-independent owner-transfer
core: source-owner publish, destination-owner consume, deterministic migration
states, deterministic error reporting, and metadata owner transfer. The QEMU
and Pi 5 proof scripts remain retained diagnostic gates.

The load-balancing source inventory and policy contract are now accepted in
docs/src/project/phase6-load-balancing-source-inventory.md and
docs/src/project/phase6-load-balancing-policy-contract.md. Implementation of
load balancing, work stealing, multi-core preemption, and later roadmap phases
remains deferred until a bounded implementation task starts from that
contract.

## Phase 6.3 Load-Balancing Policy Contract

The accepted load-balancing policy boundary is deliberately smaller than a
general multi-core scheduler. A later implementation may choose one
source-owned runnable, non-current task and one eligible production-capable
destination CPU, then call the accepted SharedRunQueue owner-transfer
mechanism. It may not directly mutate another CPU's local RunnableQueue,
current task, saved context, counters, or owner-published metadata.

Accepted policy inputs are limited to current scheduler surfaces:
LogicalCpuId, SchedulerCoreRole, owner-local RunnableQueue pressure,
PerCoreScheduler current-task state, SharedSchedulerMetadata owner/state and
generation snapshots, SharedRunQueue capacity/backpressure, and wake/timer
state only as non-migration context. Per-task affinity, priority, queue age,
virtual runtime, CPU load averages, and cache-locality policy remain
unimplemented and unavailable to the first policy.

Every policy decision is provisional. SharedRunQueue::publish_migration must
re-check metadata generation, ownership, source-local runnable membership, and
queue capacity before source-local removal. Destination consumption remains an
owner-local scheduler action. Stale metadata, invalid CPU roles, full queues,
duplicates, running or blocked candidates, and remote-wake or timer/preemption
confusion produce deterministic deferral or rejection while preserving
single-owner task state.

The first load-balancing implementation may be polling-only. Remote reschedule
is not required by the contract. If a later task adds it, an IPI or similar
signal may only record that normal scheduler control flow should run soon; it
must not execute scheduler work, consume SharedRunQueue entries, print
diagnostics, or hold scheduler topology locks in interrupt context.

## Phase 6.3 Load-Balancing Core

The first target-independent load-balancing core now lives in
`src/scheduler.rs`. It adds `LoadBalancingPolicy`, `LoadBalancingPlan`,
`LoadBalancingPublishReport`, and `LoadBalancingPolicyError` as a policy layer
above the accepted `SharedRunQueue` mechanism.

The policy is deliberately conservative. A source owner may plan only the
front task from its local runnable queue. The selected task must have
owner-published metadata that still says it is runnable, source-owned,
current-task-free, and locally queued. The destination must be a valid
production-capable scheduler owner with local queue capacity. The shared run
queue must have capacity and must not already contain the selected task.

Planning records the metadata generation in `LoadBalancingPlan`, but planning
does not move ownership. Publication must call `SharedRunQueue::publish_migration`
with that recorded generation. Stale metadata, source/destination mismatch,
running or blocked tasks, duplicate shared entries, full queues, and invalid
roles therefore remain deterministic rejection paths, and the source task stays
single-owner unless the accepted shared owner-transfer mechanism removes it
from the source-local queue.

This core is a static/unit-testable policy primitive only. It does not add
QEMU or Pi 5 proof claims, autonomous work stealing, running-task migration,
remote scheduler execution in IPI context, multi-core timer preemption,
userspace, filesystem, networking, SSH, shell behavior, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

## Phase 6.3 QEMU Load-Balancing Smoke

The retained QEMU load-balancing proof is
`scripts/qemu-load-balancing-smoke.sh`. It builds the
`qemu_load_balancing_smoke` boot scenario and exercises the accepted
`LoadBalancingPolicy` core above `SharedRunQueue`.

The diagnostic creates source owner 0, destination owner 1, one runnable task,
shared scheduler metadata, and a bounded shared run queue. It proves
`LoadBalancingPolicy::plan_front_runnable` selects the source-local front
runnable task with the current metadata generation, then
`LoadBalancingPolicy::publish_front_runnable` removes the task from the source
queue and publishes a `MigrationReserved -> SharedQueued` handoff through the
accepted shared queue. The destination then consumes the entry through
`SharedRunQueue::consume_for_destination`, enqueues task 109 locally, and
refreshes metadata owner/generation.

Passing output includes
`classification=qemu-load-balancing-smoke-complete` and
`qemu-load-balancing-smoke: PASS`. This is QEMU substitute evidence only. It
does not claim Pi 5 behavior, autonomous work stealing, running-task
migration, remote scheduler execution in IPI context, multi-core timer
preemption, userspace, filesystem, networking, SSH, shell behavior, RP1/PCIe,
UART interrupt ownership, or DMA/cache-driver policy.

## Phase 6.3 Pi 5 Load-Balancing Proof

The retained Pi 5 load-balancing proof is staged by
scripts/rpi5-load-balancing-boot-tree.sh and built with the
rpi5_load_balancing_proof boot scenario. It carries the accepted QEMU
invariant to serialized physical Pi 5 evidence without adding autonomous
balancing loops or a production secondary runtime.

The diagnostic runs on the boot CPU and creates the same source owner 0,
destination owner 1, runnable task, shared scheduler metadata, and bounded
shared run queue used by the QEMU smoke. It proves
LoadBalancingPolicy::plan_front_runnable selects task 109 with the current
metadata generation, LoadBalancingPolicy::publish_front_runnable removes the
task from the source queue and publishes the shared handoff, and
SharedRunQueue::consume_for_destination enqueues the task for destination
owner 1 while refreshing metadata owner/generation.

Passing output includes classification=pi5-load-balancing-complete and
rpi5-load-balancing: PASS. This is a bounded hardware diagnostic proof only.
It does not accept autonomous work stealing, running-task migration, remote
reschedule, multi-core timer preemption, userspace, filesystem, networking,
SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

## Phase 6.3 Load-Balancing Closeout

The load-balancing closeout is accepted in
docs/src/project/phase6-load-balancing-closeout-checkpoint.md. It reconciles
the source inventory, policy contract, target-independent core, QEMU
substitute proof, serialized Pi 5 proof, retained gates, and remaining
deferrals.

The accepted boundary is a deterministic policy primitive over already
accepted scheduler surfaces: a source owner can select one source-local front
runnable task, publish the handoff through SharedRunQueue, and a destination
owner can consume it locally. The retained proof gates are
scripts/qemu-shared-runqueue-migration-smoke.sh,
scripts/qemu-load-balancing-smoke.sh, and the Pi 5 load-balancing image and
boot-tree scripts for explicit future hardware reproduction.

This closeout does not accept an autonomous balancing loop, work stealing,
running-task migration, interrupt-driven remote reschedule, multi-core timer
preemption, Phase 7, filesystem, networking, SSH, shell behavior, RP1/PCIe,
UART interrupt ownership, or DMA/cache-driver policy. The next bounded
scheduler task should be a multi-core preemption source inventory before any
preemption implementation starts.

## Phase 6.3 Multi-Core Preemption Contract

The multi-core preemption source inventory and contract are accepted in
docs/src/project/phase6-multicore-preemption-source-inventory.md and
docs/src/project/phase6-multicore-preemption-contract.md. The accepted
invariant is deliberately owner-local: timer and IPI paths may record bounded
state, but scheduler mutation runs only from normal control flow on the owning
CPU after interrupt return.

A local timer IRQ may acknowledge/classify the interrupt, record a pending
preemption request for its own LogicalCpuId, reprogram the timer, and EOI.
IRQ and IPI context must not call scheduler dispatch, switch the current task,
consume SharedRunQueue entries, publish load-balancing migrations, print
diagnostics, or hold scheduler topology locks for unbounded work.

Current-task authority remains inside each PerCoreScheduler owner. Shared
scheduler metadata is advisory and owner-published; SharedRunQueue and
LoadBalancingPolicy continue to move only runnable, non-current tasks through
the accepted owner-transfer path. Running-task migration and remote switching
of another CPU's current task remain rejected.

The first implementation may add only the target-independent or narrowly
target-abstracted state required to let each owner service its own local
preemption request. It must preserve the lock order of local IRQ save/mask
before SMP scheduler lock acquisition, deterministic stale metadata and
wrong-owner failures, and explicit defer behavior for nested or
preemption-disabled sections. Remote reschedule remains deferred or
notification-only; it is not required for the first core.

The retained proof plan is implementation first, then QEMU, then serialized Pi
5 hardware. QEMU must show multiple logical CPUs record and service their own
local preemption requests through owner-local normal control flow. Pi 5 proof
must use hardwareTestLock and record candidate identity, TFTP evidence, fresh
serial output, participant counts, classification/PASS, and restore proof. The
contract does not accept direct IRQ/IPI-context scheduling, autonomous work
stealing, running-task migration, general remote reschedule, Phase 7,
filesystem, networking, SSH, shell behavior, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver policy.

## Phase 6.3 Multi-Core Preemption Core

The first target-independent multi-core preemption core lives in
`src/scheduler.rs`. It adds `PerCorePreemptionState` as the bounded per-owner
state that a future local timer IRQ may update before owner-local normal
control flow services the request.

`PerCorePreemptionState::record_local_timer_irq` records only local pending
state for the owning `LogicalCpuId`. It coalesces duplicate timer requests and
updates counters, but it does not inspect runnable queues, choose a next task,
refresh metadata, mutate `PerCoreScheduler`, or touch another CPU's scheduler
state.

`CpuLocalSchedulerService::run_preemption_cycle` is the owner-local service
entry for that state. Before draining wake queues or mutating scheduler state,
it verifies that the requester owns both the preemption state and
`PerCoreScheduler`, that the owner has a production-capable role, and that the
provided current task matches the scheduler's current-task slot. When those
checks pass, the service cycle preserves the accepted order: target-owned
remote wake consumption, local timer preemption through
`SingleCoreScheduler::timer_preempt`, optional local dispatch only when no
timer preemption was serviced, and finally owner-published metadata refresh.

The core also adds an explicit nested preemption-disable counter. A timer
request recorded while disabled remains pending and service returns a
deterministic defer result until the owner balances the nested section.
Wrong-owner recording/service, state/scheduler owner mismatch, current-task
mismatch, underflow/overflow, no runnable peer, missing current task, and
deferred secondary roles are deterministic errors that leave the pending
request and owner-local queue state intact.

This implementation is unit-testable core behavior only. It does not add a new
QEMU boot scenario, Pi 5 hardware proof, direct IRQ/IPI-context scheduling,
remote current-task switching, running-task migration, autonomous work
stealing, general remote reschedule, Phase 7, filesystem, networking, SSH,
shell behavior, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

## Phase 6.3 QEMU Multi-Core Preemption Proof

The retained QEMU proof surface is qemu_multicore_preemption_smoke, run by
scripts/qemu-multicore-preemption-smoke.sh. It starts logical CPUs 1, 2, and 3
through the accepted QEMU SMP path and keeps the proof at the accepted
diagnostic scheduler boundary.

Each participating owner records a local timer-preemption request through
PerCorePreemptionState::record_local_timer_irq, coalesces a duplicate local
record, and rejects a cross-owner record attempt. The diagnostic compares the
owner's current task, runnable queue length, task states, and metadata
generation before and after the record-only step; accepted PASS output requires
irq-record-scheduler-mutated=false, proving the IRQ/IPI-side hook did not
execute scheduler mutation.

After the record-only step, each owner services its own pending request through
CpuLocalSchedulerService::run_preemption_cycle from normal owner-local control
flow. PASS requires the current task to return to runnable, the selected local
runnable task to become running, the pending request to clear only after
service, and owner-published metadata to refresh for that same logical CPU.
This is QEMU substitute evidence. The serialized Pi 5 proof now carries the
same invariant to physical hardware.

## Phase 6.3 Pi 5 Multi-Core Preemption Proof

The retained Pi 5 proof surface is rpi5_multicore_preemption_proof, staged by
scripts/rpi5-multicore-preemption-image.sh and
scripts/rpi5-multicore-preemption-boot-tree.sh. It uses the same PSCI
secondary entry, per-core state, stack ownership checks, and secondary
cacheable-MMU handoff path as the accepted secondary scheduler service-loop
proof.

The physical proof starts logical CPUs 1, 2, and 3 on Raspberry Pi 5 hardware.
Each participant records and coalesces local timer-preemption state, rejects a
cross-owner record, proves the record-only step did not mutate scheduler
state, and then services the pending request through
CpuLocalSchedulerService::run_preemption_cycle from owner-local normal control
flow.

Accepted serial evidence reports participants=3, expected=3, errors=0,
classification=pi5-multicore-preemption-complete, and PASS. The proof remains
diagnostic evidence only; it does not accept direct IRQ/IPI-context scheduling,
autonomous work stealing, running-task migration, remote current-task
switching, general remote reschedule, userspace, descriptors, filesystem,
networking, SSH, shell behavior, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

## Phase 6.3 Multi-Core Preemption Closeout

The multi-core preemption closeout is accepted in
docs/src/project/phase6-multicore-preemption-closeout-checkpoint.md. It
reconciles the source inventory, contract, target-independent core, QEMU
substitute proof, serialized Pi 5 proof, retained diagnostics, and remaining
deferrals.

The accepted scheduler boundary is still diagnostic and owner-local. Timer and
IPI paths may record bounded local state only; scheduler mutation remains in
normal control flow on the owning CPU. The retained proof gates are the
scheduler unit tests, the earlier timer-preemption and secondary service-loop
QEMU gates, the shared run-queue and load-balancing QEMU gates, the focused
QEMU multi-core preemption smoke, and the Pi 5 multi-core preemption image and
boot-tree scripts for explicit future hardware reproduction.

This closeout does not accept production timer integration, direct IRQ/IPI
scheduler dispatch, remote current-task switching, running-task migration,
autonomous work stealing, general remote reschedule, non-diagnostic secondary
runtime roles, Phase 7, filesystem, networking, SSH, shell behavior,
RP1/PCIe, UART interrupt ownership, or DMA/cache-driver policy. Further work
requires a new supervisor-planned bounded task.

## Phase 6.3 Production Scheduler Runtime Inventory

The production scheduler runtime source inventory is accepted in
docs/src/project/phase6-production-scheduler-runtime-source-inventory.md. It
does not add runtime behavior; it maps the accepted diagnostic scheduler
surfaces to the normal boot, timer, and owner-local runtime paths that a
future production timer/preemption contract may touch.

The accepted diagnostic surfaces remain useful but narrow. The multi-core
preemption proofs construct scenario-local `PerCoreScheduler`,
`PerCorePreemptionState`, `RemoteWakeQueue`, and metadata objects, call
`PerCorePreemptionState::record_local_timer_irq` directly from proof code, and
then call `CpuLocalSchedulerService::run_preemption_cycle` from owner-local
diagnostic flow. That proves the record/service invariant but does not install
durable per-CPU scheduler objects or route normal timer IRQs into pending
preemption state.

The normal timer path still runs through the target IRQ handlers. QEMU and Pi
5 handlers acknowledge/classify the GIC interrupt, call the generic timer
rearm helper for timer INTIDs, record older single-core diagnostic counters
only under retained timer-preemption scenarios, EOI, and return. They do not
yet map the current logical CPU to a production `PerCorePreemptionState`, and
they do not invoke scheduler mutation in IRQ context.

The production gaps are deliberately explicit: durable per-CPU storage for
`PerCoreScheduler`, preemption state, remote-wake queues, and metadata access;
a bounded timer IRQ recording path; a post-IRQ owner-local service point for
primary and secondary runtime flow; current-task source-of-truth rules;
preemption-disable critical-section ownership; and a non-diagnostic secondary
runtime role. The next contract must preserve the accepted service order of
target-owned remote wake consumption, local timer preemption, optional
dispatch, and owner-published metadata refresh.

Retained proof gates are the scheduler unit tests, the earlier QEMU timer and
secondary service-loop smokes, the shared run-queue and load-balancing QEMU
smokes, and the QEMU multi-core preemption smoke. The Pi 5 proof scripts
remain reproduction surfaces for explicit later hardware tasks only. The next
bounded task is the production timer/preemption contract; production
implementation remains deferred until that contract is accepted.

## Phase 6.3 Production Timer/Preemption Contract

The production timer/preemption contract is accepted in
docs/src/project/phase6-production-timer-preemption-contract.md. It defines the
first production runtime boundary for carrying the accepted owner-local
preemption primitive into normal timer and post-IRQ paths without accepting a
broad scheduler redesign.

The implementation boundary is narrow. The only production entry points it may
touch are `src/target/qemu_virt.rs::handle_irq`,
`src/target/rpi5.rs::handle_irq`,
`src/arch/aarch64/generic_timer.rs::record_el2_physical_tick_and_rearm` as a
timer tick/rearm helper, one owner-local primary post-IRQ service point, one
owner-local secondary service point if the role is already production-capable,
and the minimal durable per-CPU runtime state boundary for local scheduler,
preemption state, remote-wake queue, metadata access, current-task source, and
role/capability.

The accepted invariant remains record-only in IRQ/IPI context and
owner-local in scheduler mutation. Timer handlers may map the current logical
CPU to its local `PerCorePreemptionState` and call
`record_local_timer_irq`; they must still rearm and EOI through the target
interrupt path and must not inspect queues, choose tasks, refresh metadata,
consume remote wake requests, publish or consume shared-runqueue entries,
dispatch, print, allocate, block, or take unbounded scheduler locks.

Owner-local normal control flow is the only place that may service pending
timer preemption through `CpuLocalSchedulerService::run_preemption_cycle`.
The service order remains remote wake consumption first, local timer
preemption second, optional local dispatch only when timer preemption did not
run, and owner-published metadata refresh last. Disabled preemption, stale
metadata, wrong owner, missing current task, current-task mismatch, and
non-production-capable roles are deterministic defer/reject cases that must
not clear pending preemption or mutate another owner's scheduler.

The retained implementation gates are fmt, no_std tests, the base QEMU smoke,
QEMU timer-preemption, secondary service-loop, shared-runqueue, load-balancing,
and multi-core-preemption smokes, plus whitespace inspection and docs build if
docs change.

The first production timer/preemption core is accepted in
tasks/2026-05-28-phase6-production-timer-preemption-core.md. It adds
`ProductionSchedulerRuntime` as the durable per-CPU runtime boundary for the
local scheduler, local preemption state, target-owned remote-wake queue, and
role/capability. Normal QEMU and Pi 5 timer IRQ handlers now record bounded
local production preemption state after the generic timer rearm helper and
before EOI, while leaving scheduler queues, current tasks, remote wake
consumption, shared run queues, and metadata untouched in IRQ context.

The target-independent owner-local service adapter is available through
`ProductionSchedulerRuntime::service_pending_preemption`, which delegates to
`CpuLocalSchedulerService::run_preemption_cycle` and preserves the accepted
remote-wake, timer-preemption, optional-dispatch, metadata-refresh order. The
next QEMU proof must exercise the production timer IRQ recording path and
owner-local post-IRQ service point together. Later Pi 5 proof remains
serialized under hardwareTestLock and must record candidate identity, TFTP,
fresh serial, classification/PASS or blocker classification, and restore
evidence. No physical claim is made by this core task.
