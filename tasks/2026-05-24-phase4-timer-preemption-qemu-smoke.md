# Phase 4 Timer Preemption QEMU Smoke

Task: `phase4-timer-preemption-qemu-smoke-20260524`

## Boundary

This slice proves the first timer-driven single-core scheduler handoff on QEMU
virt. The EL2 physical timer IRQ still follows the accepted order: acknowledge
INTID 26, record bounded tick accounting, record a preemption request,
reprogram CNTHP_CVAL_EL2, write GICC_EOIR, and return through the current-EL IRQ
frame.

The IRQ handler does not allocate, format, print, block, sleep, walk scheduler
queues, or call the scheduler. The QEMU diagnostic kernel threads observe the
pending preemption request after IRQ return, mask IRQs for the short
single-core scheduler mutation window, call
`SingleCoreScheduler::timer_preempt()`, restore the previous IRQ-mask state,
and cross the existing AArch64 context-switch primitive.

## QEMU Evidence

Focused QEMU/substitute evidence from `scripts/qemu-timer-preemption-smoke.sh`:

```text
qemu-timer-preemption-smoke: start current=1 runnable=2 preempted=0 requests=0
qemu-timer-preemption-smoke: progress task1=3 task2=3 ticks=6 requests=6 handled=6 timer-preemptions=6 dispatch-switches=6 voluntary-yields=0 transitions=7 current=1 runnable=2 preempted=2
qemu-timer-preemption-smoke: irq vector=5 iar=0x0000001a intid=26 unexpected=0 ctl=0x1 daif=0x3c0
qemu-timer-preemption-smoke: PASS
```

The log is captured at `target/qemu-timer-preemption-smoke.log`. It shows both
kernel threads made progress from timer-driven preemption while the
voluntary-yield counter remained zero.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed 70 no_std tests, including
  timer-preempt scheduler counter coverage.
- QEMU/substitute: `scripts/qemu-timer-preemption-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed the default EL1 smoke.
- image/archive inspection: `scripts/rpi5-image.sh` built
  `target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img`.
- fmt/lint/typecheck: `scripts/rpi5-format-guard-check.sh` and
  `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container, so the mdBook
  build was not run.

## Deferrals

Pi 5 timer-preemption hardware proof, real quantum policy, preemption-disable
counters, async exception-frame switching, sleep queues, wait queues, SMP,
EL0/userspace, process resources, descriptors, filesystem, console/TTY,
networking, and SSH remain deferred.
