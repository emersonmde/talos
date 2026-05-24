# Phase 4 Scheduler Structs And Runnable Queue

Task: phase4-scheduler-structs-runnable-queue-20260524

## Goal

Implement the first single-core scheduler structs and runnable queue without
context switching or preemption.

## Implementation Shape

- `src/scheduler.rs` owns scheduler-local `TaskId` values. These are not POSIX
  process IDs.
- `TaskState` names the first task states: `Running`, `Runnable`, and
  `Blocked`. Blocking and waking policy are not implemented in this task.
- `KernelStack` records per-task kernel stack bounds. `ContextFrame` records a
  saved stack pointer and program counter placeholder for later assembly
  context switching.
- `Task::kernel_thread` creates a kernel-thread task with no process owner.
  `ProcessOwnerId` is only an explicit future extension point for Phase 7
  process ownership.
- `RunnableQueue` is a fixed-capacity, single-core FIFO over task IDs.
- `SingleCoreScheduler` wraps the queue and a state-transition counter for the
  first scheduler diagnostics.
- `src/main.rs` keeps the module behind a local `dead_code` allowance outside
  tests because this task accepts the scheduler data structures before wiring a
  boot-time scheduler instance.

## Boundaries

The runnable queue is a pure data structure in this slice. It does not mask
interrupts internally, because this task does not install a global scheduler or
interrupt-time entry point. Later code that mutates scheduler-owned global state
from interruptible paths must use the accepted short
`single_core_irq_mask_save()` / `single_core_irq_restore()` boundary explicitly
at the call site.

This task does not add assembly context switching, yielding, sleeping,
preemptive time slicing, timer-driven scheduler callbacks, SMP run queues,
spinlocks, secondary-core support, userspace, syscalls, descriptors,
filesystem, console/TTY, networking, or SSH.

## Local Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- Unit tests: `cargo -Zjson-target-spec test` passed 65 no_std tests,
  including eight scheduler tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed the default EL1 smoke.
- Image/archive inspection: `scripts/rpi5-image.sh` built
  `target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img`.
- fmt/lint/typecheck: `scripts/rpi5-format-guard-check.sh` and
  `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container.
