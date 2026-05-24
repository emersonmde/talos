# Phase 4 Voluntary Yield Dispatch

Task: `phase4-voluntary-yield-dispatch-20260524`

## Boundary

This slice attaches the accepted cooperative context-switch primitive to the
single-core scheduler dispatch path. `SingleCoreScheduler::voluntary_yield()`
requires a running current task, a non-empty runnable queue, and queue capacity
to place the yielding task at the tail before selecting the next runnable task.
It increments voluntary-yield and dispatch-switch counters and returns the next
scheduler-local `TaskId`; the architecture call site still owns the saved
`ContextFrame` pointers and the actual AArch64 switch.

The QEMU smoke keeps the critical section explicit at the call site. It masks
IRQs only while mutating scheduler-owned current/yielded task state, queue
contents, selected next task, and counters. The masked window performs no
allocation, formatting, printing, blocking, or callbacks.

## QEMU Evidence

Focused QEMU/substitute evidence from `scripts/qemu-scheduler-yield-smoke.sh`:

```text
qemu-scheduler-yield-smoke: start current=1 runnable=2 yielded=0
qemu-scheduler-yield-smoke: progress task1=3 task2=3 yields=5 dispatch-switches=5 transitions=6 current=2 runnable=1 yielded=1
qemu-scheduler-yield-smoke: PASS
```

The log is captured at `target/qemu-scheduler-yield-smoke.log`.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed 69 no_std tests, including voluntary-yield dispatch tests.
- QEMU/substitute: `scripts/qemu-scheduler-yield-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- image/archive inspection: `scripts/rpi5-image.sh` built the normal Pi 5 image.
- fmt/lint/typecheck: `scripts/rpi5-format-guard-check.sh` and `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container.

## Deferrals

Timer-driven preemption, quantum accounting, sleep queues, blocking waits, IRQ
wakeups, SMP, EL0, process resources, descriptors, filesystem, console/TTY,
networking, and SSH remain deferred.
