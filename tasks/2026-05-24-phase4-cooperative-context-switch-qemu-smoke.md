# Phase 4 Cooperative Context Switch QEMU Smoke

Task: `phase4-cooperative-context-switch-qemu-smoke-20260524`

## Scope

Implement the first cooperative AArch64 EL2 kernel-thread context-switch
primitive and prove it with a QEMU-only smoke. This task does not add timer
preemption, voluntary scheduler dispatch, sleeping, SMP, EL0, process
resources, descriptors, filesystem, console/TTY, networking, or SSH.

## Source Boundary

- `ContextFrame` is now the saved cooperative frame for `x19..x29`, `x30`, and
  `SP_EL2`.
- `talos_aarch64_context_switch` saves the outgoing callee-saved frame and
  stack pointer, loads the incoming frame, and returns through the restored
  `x30`.
- `talos_aarch64_kernel_thread_trampoline` starts fresh kernel-thread contexts
  by passing saved `x19` as the thread argument and branching through saved
  `x20` as the entry function.
- `scripts/qemu-context-switch-smoke.sh` builds with
  `TALOS_QEMU_CONTEXT_SWITCH_SMOKE=1` and runs the focused EL2 QEMU proof.

## QEMU Evidence

Final QEMU/substitute evidence from `scripts/qemu-context-switch-smoke.sh`:

```text
qemu-context-switch-smoke: start current=0 runnable=1
qemu-context-switch-smoke: progress task1=2 task2=2 switches=5 current=2 runnable=0
qemu-context-switch-smoke: PASS
```

This proves two separate static kernel-thread stacks and contexts both made
bounded progress after cooperative switches, then returned to the main kernel
context for reporting.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed with 67 no_std tests.
- QEMU/substitute: `scripts/qemu-context-switch-smoke.sh` passed as the focused
  EL2 cooperative context-switch smoke.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed the default EL1 smoke.
- image/archive inspection: `scripts/rpi5-image.sh` built
  `target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img`.
- image/archive inspection: `scripts/rpi5-format-guard-check.sh` passed.
- fmt/lint/typecheck: `git diff --check` passed.
- static inspection: `mdbook` is unavailable in this container, so the mdBook
  build was not run.

## Deferrals

Voluntary yield/dispatch, round-robin scheduler selection, timer-driven
preemption, sleep/blocking state, SMP, userspace/EL0, process resources,
descriptors, filesystem, console/TTY, networking, and SSH remain deferred.
