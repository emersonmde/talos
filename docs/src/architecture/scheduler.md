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

The next bounded task may implement the first scheduler structs and local unit
tests for task state, kernel-stack descriptors, scheduler-local IDs, and a
single-core runnable queue. It should not add context switching, assembly
switch code, preemptive time slicing, sleep queues, SMP locks, userspace,
syscalls, file descriptors, filesystem, console/TTY, networking, or SSH.

## Implemented Struct Boundary

The first implementation lives in `src/scheduler.rs` and keeps the accepted
shape intentionally narrow:

- `TaskId` is scheduler-local and rejects zero; it is not a process ID.
- `TaskState` currently records `Running`, `Runnable`, and `Blocked` states.
  No blocking, wakeup, sleep queue, or exit policy exists yet.
- `KernelStack` records per-task stack bounds, and `ContextFrame` records the
  stack pointer and program counter placeholder that a later assembly context
  switch can save or restore.
- `Task::kernel_thread` creates a kernel-thread task with no process owner.
  `ProcessOwnerId` is an optional future extension point only; it does not add
  address spaces, descriptors, credentials, wait state, or other process
  resources.
- `RunnableQueue` is a fixed-capacity FIFO over task IDs for the single boot
  CPU. It is a pure data structure and does not hide interrupt masking or
  preemption policy.
- `SingleCoreScheduler` wraps the runnable queue with a small state-transition
  counter for diagnostics.

Because this slice has no global scheduler instance or interrupt-time mutation
path, it does not call `single_core_irq_mask_save()` internally. Future code
that mutates scheduler-owned global state from an interruptible path must place
the accepted short single-core IRQ mask/restore boundary explicitly around that
call-site invariant.
