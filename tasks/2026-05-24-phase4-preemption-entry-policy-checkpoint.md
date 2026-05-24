# Phase 4 Preemption-Entry Policy Checkpoint

Task: phase4-preemption-entry-policy-checkpoint-20260524

Status: accepted as a documentation and supervisor-state checkpoint.

## Evidence Reconciliation

- Timer IRQ delivery: QEMU virt and Raspberry Pi 5 both have accepted EL2
  physical timer evidence through GICv2/GIC-400 PPI 10 / INTID 26.
- Tick accounting: periodic tick diagnostics reprogram CNTHP_CVAL_EL2 before
  GICC_EOIR and report outside IRQ context.
- Critical section policy: boot-CPU scheduler state may use short
  single_core_irq_mask_save() / single_core_irq_restore() windows, with no
  allocation, formatting, printing, blocking, sleeping, or callbacks.
- Cooperative switching: the accepted EL2 primitive switches per-task
  ContextFrame state and kernel stacks from normal kernel control flow.
- Voluntary dispatch: SingleCoreScheduler::voluntary_yield() updates running,
  runnable, yielded, and counter state under an explicit short IRQ-masked
  scheduler mutation window.

## Decision

Go for phase4-timer-preemption-qemu-smoke-20260524 as the next bounded task. The
first proof is QEMU-only. It may let the EL2 physical timer IRQ request a
single-core scheduler dispatch, but any context switch and diagnostics must stay
outside the IRQ hot path after acknowledge, bounded accounting, timer
reprogramming, and EOI.

Pi 5 hardware is not required for the next task unless the implementation
changes normal hardware-facing boot or timer behavior. Hardware preemption proof
should be a separate serialized task after QEMU evidence.

## Next Task Acceptance Criteria

- QEMU evidence shows at least two kernel threads make progress because of
  timer-driven preemption, not explicit voluntary yield calls.
- Tick, preemption-request, and context-switch counters are reported outside IRQ
  context and remain bounded/stable for the smoke.
- The timer IRQ path preserves acknowledge/reprogram/EOI ordering and performs
  no allocation, formatting, printing, blocking, sleeping, arbitrary callbacks,
  or broad queue walking.
- No SMP, userspace, process resources, descriptors, filesystem, console/TTY,
  networking, or SSH behavior is introduced.
- Accepted work is committed before the next task starts.

## Rollback Criteria

- If QEMU cannot prove timer-driven progress separately from voluntary-yield
  progress, narrow the smoke rather than broadening scope.
- If scheduler mutation must happen inside the IRQ hot path to make the proof
  work, roll back or split the design.
- If hardware behavior becomes relevant, stop at QEMU evidence and require a
  serialized hardware follow-up with hardwareTestLock.

## Validation

- static inspection: git status --short was clean before checkpoint edits.
- fmt/lint/typecheck: git diff --check passed.
- static inspection: mdbook was unavailable in the container.
