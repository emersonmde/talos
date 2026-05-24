# Phase 4 Preemption-Entry Checkpoint

Status: accepted for the first QEMU-only timer-driven preemption smoke entry
policy.

## Scope

This checkpoint reconciles the accepted Phase 4 timer, interrupt, critical
section, cooperative context-switch, and voluntary dispatch evidence before
Talos lets a timer IRQ request scheduler dispatch. It does not implement
timer-driven preemption, sleeping, blocking waits, IRQ wake queues, SMP,
userspace, process resources, descriptors, filesystem, console/TTY, networking,
or SSH.

## Accepted Evidence

- The current-EL IRQ frame path saves the interrupted x0..x30 register frame,
  passes vector, ELR, SPSR, and frame context to rust_irq_handler, restores the
  frame, and returns with ERET.
- QEMU virt and Raspberry Pi 5 have accepted EL2 physical timer delivery through
  GICv2/GIC-400 PPI 10 / INTID 26.
- The periodic tick path reprograms CNTHP_CVAL_EL2 from the architectural
  counter before GICC_EOIR, keeps interrupt-time accounting bounded, and reports
  diagnostics outside the IRQ path.
- single_core_irq_mask_save() / single_core_irq_restore() snapshot and restore
  DAIF.I for short boot-CPU-only critical sections.
- The cooperative switch primitive saves x19..x30 and SP_EL2 in per-task
  ContextFrame state and switches between EL2 kernel-thread stacks.
- SingleCoreScheduler::voluntary_yield() can requeue the running task, select
  the next runnable task, and update voluntary-yield plus dispatch-switch
  counters under an explicit short IRQ-masked mutation window.

## Preemption Entry Policy

The next implementation may add a focused QEMU timer-preemption smoke. The timer
IRQ may request a scheduler handoff only after preserving the accepted timer
ordering:

1. acknowledge with GICC_IAR;
2. classify INTID 26 as the EL2 physical timer;
3. update bounded tick/preemption-request accounting;
4. program the next CNTHP_CVAL_EL2 deadline;
5. write GICC_EOIR;
6. perform any scheduler dispatch or diagnostic reporting outside the IRQ hot
   path.

The IRQ hot path must not allocate, format, print, block, sleep, walk arbitrary
queues, call into process or file-descriptor code, or hold interrupts masked
across diagnostics. The first smoke may use a small target-local pending
preemption flag or equivalent bounded handoff state, but it must keep global
scheduler mutation in the same short single-core critical-section style already
accepted for voluntary dispatch.

The first timer-preemption dispatch remains single-core, EL2, kernel-thread
only. It reuses scheduler-local TaskId, ContextFrame, per-task kernel stacks,
and runnable queue state. It does not add quantum accounting beyond the bounded
diagnostic proof, task sleeping, wake queues, process ownership, EL0 state, SMP
run queues, task migration, or interrupt-safe lock hierarchy.

## Go/No-Go Decision

Go for a QEMU-only timer-preemption smoke. The accepted QEMU timer, IRQ frame,
critical-section, cooperative switch, and voluntary dispatch evidence is enough
to attempt the next bounded substitute proof without a new Pi 5 hardware run.

Pi 5 hardware should remain a separate serialized follow-up after QEMU proves
the shape, unless the QEMU task changes normal hardware-facing boot or timer
behavior. A future Pi 5 run must take hardwareTestLock, publish an archive with
digest, capture TFTP and serial evidence, classify the result, restore or leave
safe lab state intentionally, and record post-hardware review.

## Next Task Contract

The next task is phase4-timer-preemption-qemu-smoke-20260524.

Acceptance criteria:

- QEMU evidence shows at least two kernel threads make progress due to
  timer-driven preemption rather than explicit voluntary yield calls.
- Tick, preemption-request, and context-switch counters are reported outside IRQ
  context and remain bounded for the smoke.
- The timer IRQ path preserves acknowledge/reprogram/EOI ordering and performs
  no allocation, formatting, printing, blocking, sleeping, or long interrupt-off
  work in IRQ context.
- The implementation remains single-core, EL2, kernel-thread only, with no SMP,
  userspace, descriptors, filesystem, console/TTY, networking, or SSH behavior.

Validation gates:

- cargo fmt --all -- --check
- cargo -Zjson-target-spec test
- a named QEMU timer-preemption smoke with captured log
- scripts/rpi5-image.sh
- scripts/rpi5-format-guard-check.sh
- git diff --check
- mdbook build if mdbook is available; otherwise record unavailable

Rollback criteria:

- Roll back or narrow the task if QEMU cannot distinguish timer-driven progress
  from voluntary-yield progress.
- Roll back or narrow the task if scheduler mutation leaks into the IRQ hot path
  in a way that requires allocation, formatting, printing, blocking, sleeping, or
  broad queue walking.
- Do not expand into Pi 5 hardware execution, SMP, userspace, or blocking
  scheduler features to rescue the smoke.

## Deferred Work

- Real quantum policy, sleeping, blocking waits, timer-driven wakeups, wait
  queues, and scheduler fairness beyond the first smoke.
- SMP routing, per-core timer state, run-queue locking, task migration, memory
  ordering for secondary cores, and interrupt-safe lock hierarchy.
- EL0/user timer routing, syscall ABI, process IDs, file descriptors, VFS,
  console/TTY, filesystem, local shell, networking, and SSH.
- Pi 5 timer-preemption hardware proof until after QEMU evidence or a later
  supervisor-planned hardware task.

## Validation

- static inspection: git status --short was clean before checkpoint edits.
- fmt/lint/typecheck: git diff --check passed for this checkpoint.
- static inspection: mdbook was unavailable in the container.
