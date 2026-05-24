# Phase 4 Pi 5 EL2 Timer IRQ Smoke

Task: phase4-pi5-gic400-el2-timer-smoke-20260524

## Goal

Carry the accepted QEMU EL2 timer-interrupt shape to Pi 5 GIC-400 hardware with
serialized lab evidence.

## Implementation Shape

- Focused diagnostic cfg: TALOS_RPI5_TIMER_IRQ_DIAGNOSTIC.
- GIC-400 distributor: 0x10_7fff_9000.
- GIC-400 CPU interface: 0x10_7fff_a000.
- Timer: EL2 hypervisor physical timer via CNTHP_*_EL2.
- Interrupt: PPI 10 / INTID 26.

The IRQ path acknowledges with GICC_IAR, records bounded atomic counters,
masks CNTHP_CTL_EL2, EOIs with GICC_EOIR, and returns through the existing
saved exception frame. Diagnostic reporting happens after IRQs are masked again,
outside the IRQ path.

## Local Validation

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- Unit tests: cargo -Zjson-target-spec test passed 51 no_std tests.
- QEMU/substitute: scripts/qemu-smoke.sh passed.
- QEMU/substitute: scripts/qemu-timer-irq-smoke.sh passed with INTID 26,
  irq-count=1, EOI, and qemu-timer-irq-smoke: PASS.
- Image/archive inspection: scripts/rpi5-image.sh built the normal Pi 5 image.
- Image/archive inspection: scripts/rpi5-timer-irq-diagnostic-image.sh built
  the focused diagnostic image.
- fmt/lint/typecheck: scripts/rpi5-format-guard-check.sh passed.
- Image/archive inspection: scripts/rpi5-archive-review.sh
  target/talos-rpi5-timer-irq-boot.tar.gz passed with archive sha256
  1861b6978b505381fd28ffb21320f1db9434405c4ce44af69354d6e1e82f5bb2,
  kernel_size=86429, header_image_size=86429, text_offset=0, and flags=12.
- fmt/lint/typecheck: git diff --check passed.

## Hardware Evidence

Evidence directory:
tasks/evidence/2026-05-24-pi5-el2-timer-irq-smoke/.

- Archive path: target/talos-rpi5-timer-irq-boot.tar.gz.
- Archive sha256:
  1861b6978b505381fd28ffb21320f1db9434405c4ce44af69354d6e1e82f5bb2.
- Kernel image sha256:
  850902110e96af341e595f1493c0802f742e6618ad57546f0f37dc06236d3e0a.
- Kernel image size: 86,429 bytes.
- TFTP/archive proof: tftp-delta.json shows 10.42.1.4 was served
  kernel_2712.img at 86,429 bytes from the candidate boot tree.
- Serial hardware boot/output: serial-observe.json shows Talos reached the
  diagnostic after MMU/cache/allocator setup, printed the GIC/timer setup,
  handled irq-count=1 vector=5 iar=0x0000001a intid=26 unexpected=0 ctl=0x2,
  reported post-IRQ workload progress, and printed
  rpi5-timer-irq-smoke: PASS.
- Safe-state note: pre-run boot snapshot
  pre-phase4-pi5-timer-irq-20260524T044757Z was restored after capture;
  restore-pre-snapshot.json reports ok=true and tree hash
  6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef.

Classification: accepted hardware delivery of EL2 physical timer PPI 10 /
INTID 26 through Pi 5 GIC-400. This is not scheduler, periodic tick, UART IRQ,
SMP, lower-EL, DMA, RP1/PCIe, or networking evidence.
