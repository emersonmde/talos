# Phase 4 Timer-Smoke Checkpoint

Task: `phase4-timer-smoke-checkpoint-20260524`

Status: accepted as a documentation and supervisor-state checkpoint.

## Finding-To-Evidence Summary

- IRQ entry/return frame: accepted at `de40482`; the vector path saves and
  restores `x0..x30` and calls the Rust IRQ handler with vector, ELR, SPSR, and
  frame context.
- QEMU EL2 timer smoke: accepted at `bce215d`; `scripts/qemu-timer-irq-smoke.sh`
  shows GICv2 INTID 26 delivery, acknowledge, EOI, and post-IRQ workload
  progress.
- Pi 5 EL2 timer smoke: accepted at `966d453`; serialized lab evidence under
  `tasks/evidence/2026-05-24-pi5-el2-timer-irq-smoke/` shows the candidate
  image was served and the GIC-400 handler processed INTID 26.

## Decision

The next worker task may start monotonic tick accounting after this checkpoint
is accepted. UART interrupts, SMP, lower ELs, DMA, RP1/PCIe interrupt routing,
scheduler policy, filesystem/userland, networking, and SSH remain non-goals.

## Validation

- `git diff --check`: passed.
- `mdbook build`: unavailable in the container.

## Supervisor State

Durable state was updated to accept this checkpoint and mark
`phase4-monotonic-tick-accounting-20260524` ready because its declared
dependency is now satisfied.
