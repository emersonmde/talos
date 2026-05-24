# Phase 4 Single-Core Critical Section Policy

Task: phase4-interrupt-mask-critical-section-policy-20260524

## Goal

Define and validate the first explicit interrupt masking and critical-section
contract for single-core kernel code.

## Implementation Shape

- `src/arch/aarch64/mod.rs` owns `SingleCoreIrqMaskState`,
  `single_core_irq_mask_save()`, and `single_core_irq_restore()`.
- The API snapshots `DAIF`, masks `PSTATE.I`, and restores the previous
  IRQ-mask state from the saved snapshot.
- Nested masked scopes remain masked after inner and outer restore.
- Scopes entered with IRQs unmasked restore unmasked delivery on exit.
- The name is intentionally single-core-only. It is not an SMP lock, blocking
  lock, sleepable lock, preemption counter, lower-EL policy, or scheduler API.

## QEMU Evidence Shape

`scripts/qemu-timer-irq-smoke.sh` now requires the timer smoke log to show:

```text
qemu-timer-irq-smoke: irq-mask nested-start=true inner-restored=true outer-restored=true unmasked-start=true saved-mask=true restored-unmasked=true
```

The same diagnostic wraps each bounded timer-smoke workload iteration in a
short save/restore critical section. The periodic EL2 physical timer still
reaches the four-tick proof target with INTID 26, unexpected=0, and PASS.

## Local Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- Unit tests: `cargo -Zjson-target-spec test` passed 57 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed the default EL1 smoke.
- QEMU/substitute: `scripts/qemu-timer-irq-smoke.sh` passed with the
  irq-mask line above, `tick-count=4 target=4`, INTID 26, unexpected=0,
  bounded critical-section workload progress, and PASS.
- Image/archive inspection: `scripts/rpi5-image.sh` built the normal Pi 5
  image.
- fmt/lint/typecheck: `scripts/rpi5-format-guard-check.sh` and
  `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container.

## Hardware Scope

No Pi 5 hardware run was required. This task did not change the accepted Pi 5
physical timer IRQ behavior or publish a candidate hardware archive; it added a
shared single-core IRQ mask/restore primitive and proved the critical-section
workload against the QEMU timer diagnostic.
