# Phase 4 Monotonic Tick Accounting

Task: phase4-monotonic-tick-accounting-20260524

## Goal

Convert the accepted EL2 physical timer interrupt into a minimal single-core
monotonic kernel tick counter with explicit interrupt-time constraints.

## Implementation Shape

- Shared tick state lives in `src/arch/aarch64/generic_timer.rs`.
- The diagnostic tick cadence is one centisecond of `CNTFRQ_EL0`, with a
  1,000-counter floor.
- QEMU virt and Pi 5 keep their target-local GIC base addresses and
  acknowledgement state, but both consume the shared counter and reprogramming
  helper.
- The IRQ handler increments the relaxed atomic tick count and reprograms
  `CNTHP_CVAL_EL2` before writing `GICC_EOIR`.
- Diagnostic printing happens after IRQs are masked again, outside the IRQ hot
  path.

The hot path does not allocate, format, print, sleep, call scheduler policy, or
touch process/user state.

## Local Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- Unit tests: `cargo -Zjson-target-spec test` passed 54 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed the default EL1 smoke.
- QEMU/substitute: `scripts/qemu-timer-irq-smoke.sh` passed with `tick-count=4
  target=4`, INTID 26, unexpected=0, continued workload progress, and PASS.
- Image/archive inspection: `scripts/rpi5-image.sh` built the normal Pi 5
  image.
- Image/archive inspection: `scripts/rpi5-timer-irq-diagnostic-image.sh` built
  the focused diagnostic image.
- Image/archive inspection: `scripts/rpi5-archive-review.sh
  target/talos-rpi5-monotonic-tick-boot.tar.gz` passed with archive sha256
  `8faded95158d043b0270684dc208ad1e4e4652ced6d39b19229642bcefa3c022`,
  kernel_size=86661, header_image_size=86661, text_offset=0, and flags=12.
- fmt/lint/typecheck: `scripts/rpi5-format-guard-check.sh` and
  `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container.

## Hardware Evidence

Evidence directory:
`tasks/evidence/2026-05-24-pi5-monotonic-tick-accounting/`.

- Archive path: `target/talos-rpi5-monotonic-tick-boot.tar.gz`.
- Archive sha256:
  `8faded95158d043b0270684dc208ad1e4e4652ced6d39b19229642bcefa3c022`.
- Kernel image sha256:
  `809b0f110a34209dae762a87e1769490eadb89895b38c923e54b0c537e123890`.
- Kernel image size: 86,661 bytes.
- TFTP/archive proof: `tftp-delta-third.json` shows 10.42.1.4 was served
  `kernel_2712.img` at 86,661 bytes from the candidate boot tree.
- Serial hardware boot/output: `serial-observe-third.json` shows Talos reached
  the diagnostic after MMU/cache/allocator setup, printed
  `target-ticks=4`, handled `tick-count=4 target=4 vector=5
  iar=0x0000001a intid=26 unexpected=0 ctl=0x1`, reported post-tick workload
  progress, and printed `rpi5-timer-irq-smoke: PASS`.
- Safe-state note: pre-run boot snapshot
  `pre-phase4-monotonic-tick-20260524T051820Z` was restored after capture;
  `post-final-restore-status.json` reports `ok=true`, PoE `UP`, and tree hash
  `6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef`.

Classification: accepted hardware evidence for periodic EL2 physical timer
tick accounting on the Pi 5 boot CPU. This is not scheduler, preemption, SMP,
UART IRQ, lower-EL, DMA, RP1/PCIe, filesystem, userspace, or networking
evidence.
