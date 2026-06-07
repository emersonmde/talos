# Static Inspection

Task: phase11-rp1-interrupt-routing-diagnostic-core-20260607

## Source

- Added boot scenarios rpi5_rp1_interrupt_routing_msix_cfg_read and
  rpi5_rp1_interrupt_routing_no_mmio_control.
- rust_entry routes both scenarios directly to their Pi 5 diagnostic functions
  before BootInfo parsing, target initialization, boot reports, memory planning,
  allocator setup, scheduler work, command-loop work, or broader Phase 11
  behavior.
- The real candidate emits a before-load marker, flushes UART10, performs one
  read_rp1_reg_u32(RP1_IO_BANK0_MSIX_CFG), and then loops forever emitting the
  retained raw MSI-X config value, decoded enable/test/iack/iack-en bits, and
  classification=routing-msix-cfg-visible.
- The control candidate emits a no-rp1-msix-pcie-gic-mmio marker and loops
  forever with the same report-field shape, address=not-constructed,
  simulated raw value 0, decoded bit fields, and classification=simulated/control.

## Image And Archive

RP1 interrupt-routing candidate:

- archive: target/talos-rpi5-rp1-interrupt-routing-msix-cfg-read-core.tar.gz
- archive SHA-256:
  09c5f12dcf9a6d52d4ba265e2a6cfcb1c2797a278e342e59f08edba8a27de00b
- kernel SHA-256:
  f076867595f93b71632c09b058d55177e9814f195bc6e102e63725c26d153748
- kernel size: 46,648 bytes
- arm64 Image header: text_offset=0, header_image_size=46648, flags=12,
  magic=ARMd.
- repeated marker: TALOS: rp1-interrupt-routing-result

No-MMIO/no-enable control:

- archive: target/talos-rpi5-rp1-interrupt-routing-no-mmio-control-core.tar.gz
- archive SHA-256:
  df030dd8c696e9cb5f3bd8a28abaa3fc6584d2d540811e29b2c37b3bf156668c
- kernel SHA-256:
  114c1e76a54f30c6f49f39d2418c0974b52a8af63b74ccf99f034b1f7df6a154
- kernel size: 46,520 bytes
- arm64 Image header: text_offset=0, header_image_size=46520, flags=12,
  magic=ARMd.
- repeated marker: TALOS: rp1-interrupt-routing-control

## Assembly Review

Retained assembly:

- tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-diagnostic-core/rp1-interrupt-routing-msix-cfg-read-asm.txt
- tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-diagnostic-core/control-interrupt-routing-asm.txt

Key observations:

- The real candidate constructs x19 = 0x1f00108008 and performs exactly one
  contracted load, ldr w22, [x19], before entering the repeated report loop.
- The real candidate's later 0x1f00108008 construction is for printing the
  accepted target address in the terminal marker, not for an additional MMIO
  access.
- The no-MMIO control function does not construct 0x1f00108008,
  0x1f000d0070, 0x1f000e0008, 0x1f000f003c, 0x107fff9000, or 0x107fffa000.
- The no-MMIO control has no read_rp1_reg_u32 call and no RP1 interrupt, GPIO,
  pads, RIO, clock/reset, MSI-X, PCIe config, MIP, or GIC MMIO load/store.
- The retained control-path ldr w10, [x9, #0x18] instructions use
  x9 = 0x107d001000 and are UART10 FR polling in wait_uart10_empty_early_phase.

## Findings

- fixed: added the exact read-only/no-enable RP1 IO_BANK0 MSIX_CFG(0)
  diagnostic selected by phase11-rp1-interrupt-routing-source-contract-v1.
- fixed: added a paired no-MMIO/no-enable control preserving the serial output
  field shape without constructing the contracted RP1/MSI-X/PCIe/GIC address.
- fixed: added task-owned image, boot-tree, archive, and archive-review helpers
  for both artifacts.
- fixed: retained archive identity, image/header metadata, strings, and
  assembly evidence for both candidates.
- deferred: Pi 5 visibility of the no-MMIO/no-enable control and real
  RP1 interrupt-routing candidate remains queued hardware work.
- not-an-issue: source-predicted hwirq/MSI-X/GIC fields are report metadata for
  this read-only diagnostic; this task does not accept interrupt delivery.

## Non-Acceptance

This inspection accepts only local source/static/archive evidence. The
serialized no-MMIO/no-enable Pi 5 control must pass before any real
interrupt-routing hardware proof is attempted.
