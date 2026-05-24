# Phase 4 Scheduler/Preemption Contract Consolidation

Task: `phase4-scheduler-preemption-contract-consolidation-20260524`

## Goal

Consolidate the accepted QEMU and Pi 5 timer-preemption proofs into a durable
scheduler/preemption contract before Phase 4 closeout.

## Evidence Reconciliation

- QEMU/substitute proof: commit `2cf0e64` accepted
  `scripts/qemu-timer-preemption-smoke.sh`. The log reported two kernel
  threads, six timer ticks, six handled preemption requests, six timer
  preemptions, six dispatch switches, zero voluntary yields, INTID 26, and
  PASS.
- Serial hardware boot/output proof: commit `9e53676` accepted the serialized
  Pi 5 run in
  `tasks/evidence/2026-05-24-pi5-timer-preemption-hardware-proof/`. The Pi
  fetched the 103,152-byte candidate `kernel_2712.img` and reported task1=3,
  task2=3, ticks=6, requests=6, handled=6, timer-preemptions=6,
  dispatch-switches=6, voluntary-yields=0, INTID 26, unexpected=0, and
  `rpi5-timer-preemption-smoke: PASS`.

Both proofs validate the same boundary: INTID 26 records tick and preemption
request evidence in IRQ context, then kernel-thread code performs scheduler
dispatch and the cooperative context switch after IRQ return.

## Contract Boundary

Production scheduler contract:

- scheduler-local task IDs, task states, per-task kernel stacks, saved
  cooperative `ContextFrame` values, a single boot-CPU runnable queue, and
  scheduler counters;
- short `single_core_irq_mask_save()` / `single_core_irq_restore()` windows
  around current/runnable task mutation, dispatch counters, and context-frame
  handoff;
- EL2 kernel-thread context switching only through
  `talos_aarch64_context_switch`.

Timer IRQ contract:

- acknowledge with `GICC_IAR`;
- classify INTID 26;
- record vector/IAR/INTID evidence, monotonic ticks, and the bounded
  preemption-request counter when the timer-preemption diagnostic is enabled;
- reprogram `CNTHP_CVAL_EL2`;
- write `GICC_EOIR` and return through the current-EL IRQ frame.

The IRQ hot path must not allocate, format, print to serial, block, sleep, call
the scheduler, mutate runnable queues, or cross the context-switch primitive.

## Diagnostic Surfaces

Retained:

- `TALOS_QEMU_CONTEXT_SWITCH_SMOKE` /
  `scripts/qemu-context-switch-smoke.sh`: validates the raw cooperative
  switch primitive until a regular kernel-thread launcher supersedes it.
- `TALOS_QEMU_SCHEDULER_YIELD_SMOKE` /
  `scripts/qemu-scheduler-yield-smoke.sh`: validates voluntary-yield dispatch
  until a regular in-kernel yield path exists.
- `TALOS_QEMU_TIMER_PREEMPTION_SMOKE` /
  `scripts/qemu-timer-preemption-smoke.sh`: fast regression gate for
  timer-driven dispatch through Phase 4 closeout.
- `TALOS_RPI5_TIMER_IRQ_DIAGNOSTIC` and
  `TALOS_RPI5_TIMER_PREEMPTION_DIAGNOSTIC` with their image scripts:
  serialized hardware evidence gates for Pi 5 EL2 timer IRQ delivery and
  timer-driven handoff.

No diagnostic surface is reclassified as a supported kernel interface. Revisit
or remove the retained Pi 5 boot-image diagnostics after Phase 4 closeout, or
when a Phase 5 local diagnostic command channel can report equivalent counters
without special boot images.

## Deferrals

Real quantum policy, preemption-disable counters, async exception-frame
switching, sleep/wakeup queues, SMP run-queue locking, task migration, lower-EL
state, process resources, descriptors, filesystem, console/TTY, networking, and
SSH remain deferred.

## Validation

- static inspection: audited `src/target/qemu_virt.rs`,
  `src/target/rpi5.rs`, `src/scheduler.rs`, and the accepted QEMU/Pi 5 task
  records for IRQ-context and scheduler critical-section boundaries.
- fmt/lint/typecheck: `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container, so the mdBook
  build was not run.
- Rust tests, QEMU smokes, Pi 5 image builds, and hardware runs were not rerun
  because this task changed only docs and task records.
