# Phase 4 Scheduler Shape And POSIX Alignment

Task: phase4-scheduler-shape-posix-alignment-20260524

## Goal

Define the first single-core scheduler task/process shape before implementing
scheduler structs.

## Early POSIX Reconciliation

`docs/src/project/early-posix-shape.md` distinguishes tasks, kernel threads,
processes, and user threads. The accepted Phase 4.3 shape keeps that split:

- the scheduler schedules tasks, not processes;
- the first concrete tasks are kernel threads running only in kernel address
  space;
- process-owned resources such as address spaces, descriptor tables, current
  working directory, credentials, exit/wait state, and child state remain
  deferred;
- the first task structs should leave an extension point for a later process
  owner without inventing Phase 7 process semantics.

## Accepted Shape

The first scheduler implementation should be boot-CPU-only and kernel-thread
only. It may add scheduler-local task IDs, saved context state, per-task kernel
stack descriptors, a minimal task state enum, a single-core runnable queue, and
diagnostic counters for state transitions or switches.

The model should not treat a task ID as a POSIX PID, should not store file
descriptors or path state on each task, and should not assume a schedulable
context owns global resources directly. Wakeups should be shaped around tasks
so later blocking I/O can sleep one task without blocking the future process
model.

## Critical-Section Boundary

The accepted single-core IRQ mask/restore primitive may protect short
runnable-queue or task-state invariants on the boot CPU. It is not a spinlock,
blocking lock, sleepable lock, preemption-disable counter, SMP memory-ordering
policy, or lower-EL interrupt policy.

The next implementation task must keep any IRQ-masked scheduler section short
and explicit. It must not hide preemption policy inside the runnable-queue data
structure.

## Deferred Work

- context switching, assembly switch code, yielding, sleeping, and preemptive
  time slicing;
- preemption-disable accounting beyond explicit short single-core IRQ-masked
  sections;
- SMP run queues, spinlocks, interrupt-safe locks, memory barriers, secondary
  core startup, and task migration;
- EL0, process address spaces, syscalls, descriptor tables, VFS, current
  working directory, spawn, exec, exit, wait, and user threads;
- console/TTY, filesystem, networking, and SSH.

## Next Task Scope

Queue the next bounded implementation task as
`phase4-scheduler-structs-runnable-queue-20260524`.

That task should own only the first scheduler structs and local tests for
task IDs, task states, kernel-stack descriptors, and a single-core runnable
queue. It should not implement context switching or preemption.

## Local Validation

- static inspection: `git status --short` was clean before documentation
  edits.
- fmt/lint/typecheck: `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container.
