# Phase 4 Pre-Scheduler Closeout

Task: phase4-pre-scheduler-closeout-20260524

## Goal

Checkpoint interrupt controller, timer, tick, and critical-section evidence
before kernel thread and scheduler structures begin.

## Accepted Boundary

- Phase 4 source-backed interrupt/timer inventory is accepted at `0fb6260`.
- The inert current-EL IRQ frame foundation is accepted at `de40482`.
- QEMU EL2 physical timer IRQ delivery is accepted at `bce215d`.
- Pi 5 EL2 physical timer IRQ delivery is accepted at `966d453` with
  serialized lab evidence under
  `tasks/evidence/2026-05-24-pi5-el2-timer-irq-smoke/`.
- The timer-smoke checkpoint is accepted at `957bbc8`.
- Monotonic tick accounting is accepted at `54d2075` with QEMU evidence and
  serialized Pi 5 evidence under
  `tasks/evidence/2026-05-24-pi5-monotonic-tick-accounting/`.
- The single-core IRQ mask/restore critical-section policy is accepted at
  `1bbfec6` with QEMU nested mask/restore and timer-stability evidence.

## Deferred Work

- Scheduler task structures, runnable queues, context switching, yielding,
  sleeping, and preemptive time slicing.
- Scheduler-owned preemption-disable policy beyond the accepted single-core
  IRQ save/restore primitive.
- SMP routing, secondary-core timer setup, spinlocks, interrupt-safe locks,
  atomics policy beyond the current bounded counters, and memory barriers.
- UART interrupts, secondary interrupt controllers, RP1/PCIe, DMA, IOMMU, and
  cache-coherent driver policy.
- Lower-EL timer access, POSIX clocks, user processes, filesystems, local
  shell, networking, and SSH.

## Decision

Phase 4.3 may start after this checkpoint. The next queued task is a bounded
scheduler-shape task that must check task/process terminology and ownership
against `docs/src/project/early-posix-shape.md` before scheduler structs are
committed.

## Local Validation

- static inspection: `git status --short` was clean before checkpoint edits.
- fmt/lint/typecheck: `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container.
