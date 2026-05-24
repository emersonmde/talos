# Phase 4 Pre-Scheduler Closeout

Status: accepted for the Phase 4.1/4.2 interrupt-controller, timer, tick, and
single-core critical-section boundary before Milestone 4.3 scheduler work.

## Scope

This checkpoint reconciles the accepted Phase 4 interrupt/timer evidence before
Talos starts kernel thread and scheduler structures. It does not add scheduler
data structures, runnable queues, context switching, preemption time slicing,
sleep queues, SMP, userspace, UART interrupts, DMA, RP1/PCIe routing, or
networking.

## Accepted Behavior

- The current-EL IRQ frame path preserves the interrupted `x0..x30` register
  frame, calls `rust_irq_handler(vector, elr, spsr, frame)`, restores the
  frame, and returns with `ERET`.
- QEMU virt and Raspberry Pi 5 both deliver the EL2 hypervisor physical timer
  through GICv2/GIC-400 PPI 10 / INTID 26.
- The timer IRQ hot path acknowledges with `GICC_IAR`, recognizes INTID 26,
  reprograms `CNTHP_CVAL_EL2` from the current architectural counter before
  `GICC_EOIR`, uses bounded relaxed atomic accounting, and keeps allocation,
  formatting, serial output, scheduler callbacks, and sleeping out of IRQ
  context.
- The periodic tick diagnostic reaches `tick-count=4 target=4` on both QEMU and
  Pi 5, with continued post-tick workload progress.
- The single-core critical-section API snapshots `DAIF`, masks `PSTATE.I`, and
  restores the previous IRQ-mask state. It is explicitly a boot-CPU primitive,
  not an SMP lock, blocking lock, sleepable lock, preemption counter,
  lower-EL policy, or scheduler API.

## Evidence

- Phase 4 source inventory: commit `0fb6260`; architecture note
  `docs/src/architecture/interrupts-timers.md`.
- IRQ frame foundation: commit `de40482`.
- QEMU EL2 timer IRQ smoke: commit `bce215d`; task record
  `tasks/2026-05-24-phase4-qemu-el2-timer-irq-smoke.md`; QEMU log
  `target/qemu-timer-irq-smoke.log`.
- Pi 5 EL2 timer IRQ smoke: commit `966d453`; task record
  `tasks/2026-05-24-phase4-pi5-el2-timer-irq-smoke.md`; serialized hardware
  evidence in `tasks/evidence/2026-05-24-pi5-el2-timer-irq-smoke/`.
- Timer-smoke checkpoint: commit `957bbc8`; checkpoint
  `docs/src/project/phase4-timer-smoke-checkpoint.md`.
- Monotonic tick accounting: commit `54d2075`; task record
  `tasks/2026-05-24-phase4-monotonic-tick-accounting.md`; serialized Pi 5
  hardware evidence in
  `tasks/evidence/2026-05-24-pi5-monotonic-tick-accounting/`.
- Single-core IRQ mask policy: commit `1bbfec6`; task record
  `tasks/2026-05-24-phase4-interrupt-mask-critical-section-policy.md`; QEMU
  evidence includes the nested mask/restore proof and four-tick timer smoke.

## Deferred Work

- Scheduler task structures, runnable queues, context switching, yielding,
  sleeping, and preemptive time slicing.
- Preemption-disable policy and any scheduler-owned interrupt masking rules
  beyond the accepted single-core save/restore primitive.
- SMP routing, per-core timer state, secondary-core startup, spinlocks,
  interrupt-safe locks, memory barriers, and cross-core scheduler ownership.
- UART interrupts, BCM2712 secondary interrupt controllers, BCM2835 system timer
  SPIs, RP1/PCIe interrupts, MSI, DMA, IOMMU, and cache-coherent driver policy.
- Lower-EL timer routing, EL0/user timer access, POSIX clocks, user processes,
  filesystems, local shell, networking, and SSH.

## Milestone 4.3 Readiness

Phase 4.3 may start after this checkpoint. The first scheduler task must remain
bounded to the early scheduler shape: task/process terminology, ownership, and
single-core scheduler invariants should be checked against
`docs/src/project/early-posix-shape.md` before committing scheduler structs.

The next task should not infer SMP, userspace, time slicing, or blocking I/O
from the timer evidence. It should define the minimal kernel-thread scheduler
shape that can later grow toward POSIX processes and file descriptors without
moving those later phases forward.

## Validation

- static inspection: `git status --short` was clean before checkpoint edits.
- fmt/lint/typecheck: `git diff --check` passed for this checkpoint.
- static inspection: `mdbook` was unavailable in the container.
