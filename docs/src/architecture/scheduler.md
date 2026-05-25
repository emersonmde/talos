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
- `SchedulerCoreRole` separates `BootCpuProduction` from
  `SecondaryDeferred`; only the boot-CPU role can enter production dispatch in
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
