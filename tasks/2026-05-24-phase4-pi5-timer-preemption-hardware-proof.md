# Phase 4 Pi 5 Timer-Preemption Hardware Proof

Task: phase4-pi5-timer-preemption-hardware-proof-20260524

## Goal

Prove the accepted QEMU timer-driven single-core kernel-thread preemption shape
on physical Raspberry Pi 5 hardware.

## Implementation Shape

- Focused diagnostic cfg: `TALOS_RPI5_TIMER_PREEMPTION_DIAGNOSTIC`.
- Image helper: `scripts/rpi5-timer-preemption-diagnostic-image.sh`.
- Timer and IRQ target: EL2 hypervisor physical timer, PPI 10 / INTID 26.
- GIC-400 distributor: `0x10_7fff_9000`.
- GIC-400 CPU interface: `0x10_7fff_a000`.

The IRQ hot path acknowledges with `GICC_IAR`, classifies INTID 26, records a
monotonic tick and bounded preemption-request counter, reprograms
`CNTHP_CVAL_EL2`, writes `GICC_EOIR`, and returns. It does not allocate,
format, print, block, sleep, or mutate scheduler queues.

Two static EL2 kernel-thread contexts observe requests after IRQ return. The
thread-side handoff masks IRQs only around the short scheduler mutation,
dispatches with `SingleCoreScheduler::timer_preempt()`, restores the previous
IRQ mask state, and crosses the accepted cooperative context-switch primitive.

## Local Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- Unit tests: `cargo -Zjson-target-spec test` passed 70 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-timer-preemption-smoke.sh` passed with two
  kernel threads, six timer preemptions, zero voluntary yields, INTID 26, and
  PASS.
- Image/archive inspection: `scripts/rpi5-image.sh` built the normal Pi 5
  image.
- Image/archive inspection:
  `scripts/rpi5-timer-preemption-diagnostic-image.sh` built the focused
  diagnostic image.
- Image/archive inspection: `scripts/rpi5-archive-review.sh
  target/talos-rpi5-timer-preemption-boot.tar.gz` passed with archive sha256
  `950763917580e17aadfacd0f4e1ba3bba9e2b6960e800285e85db83cfaaa5f07`,
  kernel_size=103152, header_image_size=103152, text_offset=0, and flags=12.
- fmt/lint/typecheck: `scripts/rpi5-format-guard-check.sh` and
  `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container.

## Hardware Evidence

Evidence directory:
`tasks/evidence/2026-05-24-pi5-timer-preemption-hardware-proof/`.

- Archive path: `target/talos-rpi5-timer-preemption-boot.tar.gz`.
- Archive sha256:
  `950763917580e17aadfacd0f4e1ba3bba9e2b6960e800285e85db83cfaaa5f07`.
- Kernel image sha256:
  `417fd5f589b851c1fc1b2b1c77d7640fedc2abad32d0573effe8bc9606e550cb`.
- Kernel image size: 103,152 bytes.
- TFTP/archive proof: `tftp-delta.json` shows 10.42.1.4 was served
  `kernel_2712.img` at 103,152 bytes from the candidate boot tree.
- Serial hardware boot/output: `serial-observe-after-tftp.json` shows Talos
  reached the diagnostic after MMU/cache/allocator setup and printed
  `task1=3 task2=3 ticks=6 requests=6 handled=6 timer-preemptions=6
  dispatch-switches=6 voluntary-yields=0`, `vector=5`,
  `iar=0x0000001a`, `intid=26`, `unexpected=0`, and
  `rpi5-timer-preemption-smoke: PASS`.
- Hardware lock: acquired before publish/power-cycle and released after serial
  and TFTP capture.
- Safe-state note: pre-run boot snapshot
  `pre-phase4-pi5-timer-preemption-20260524T090536Z` was restored after
  capture; `post-restore-status.json` reports ok=true and tree hash
  `6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef`.

Classification: accepted hardware delivery of timer-driven single-core
kernel-thread preemption on the Pi 5 boot CPU. This is not scheduler quantum
policy, async exception-frame switching, SMP, userspace, descriptors,
filesystem, console/TTY, networking, or SSH evidence.
