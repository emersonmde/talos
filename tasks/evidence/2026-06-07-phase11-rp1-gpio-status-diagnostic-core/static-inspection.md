# Static Inspection

Task: phase11-rp1-gpio-status-diagnostic-core-20260607

## Source

- Added boot scenarios rpi5_rp1_gpio14_status_read and
  rpi5_rp1_gpio14_status_no_mmio_control.
- rust_entry routes both scenarios directly to their Pi 5 diagnostic functions
  before BootInfo parsing, target initialization, boot reports, memory planning,
  allocator setup, scheduler work, command-loop work, or broader Phase 11
  behavior.
- The real candidate emits a before-load marker, flushes UART10, performs one
  read_rp1_reg_u32(RP1_GPIO14_STATUS), and then loops forever emitting the
  retained raw value, interpreted raw/filtered status bits, and
  classification=diagnostic-result-visible.
- The control candidate emits a no-rp1-mmio marker and loops forever with the
  same field shape, address=not-constructed, simulated raw value 0, interpreted
  bit fields, and classification=simulated/control.

## Image And Archive

RP1 GPIO14 status candidate:

- archive: target/talos-rpi5-rp1-gpio14-status-read-core.tar.gz
- archive SHA-256:
  7bc21b39a5d0150221a244701285d733c8faef4e153085a49a34b5069c1fecea
- boot-tree identity:
  2f9b2d308af2b7db1b0099e8910457be008ed98a4abfc75ec5b6b06432e49e6a
- kernel SHA-256:
  e95dbb1cf6e3296bc29832c69115afadf78622c0995c4734638dd249f8867efc
- kernel size: 46,336 bytes
- arm64 Image header: text_offset=0, header_image_size=46336, flags=12,
  magic=ARMd

No-MMIO control:

- archive: target/talos-rpi5-rp1-gpio14-status-no-mmio-control-core.tar.gz
- archive SHA-256:
  0f3b084d686a12d101b5f43d6f8b264a57eee7241790e6ab87577454dd6ac611
- boot-tree identity:
  6bfc9797b0efe254d4a02fe4569064f565281f77191ac56b87351a92c2b7160b
- kernel SHA-256:
  148fd0e7dada9646be1c4c74796e13bea535bc4f03ec11330aa2cab237600b52
- kernel size: 46,160 bytes
- arm64 Image header: text_offset=0, header_image_size=46160, flags=12,
  magic=ARMd

## Assembly Review

Retained assembly:

- tasks/evidence/2026-06-07-phase11-rp1-gpio-status-diagnostic-core/rp1-gpio14-status-read-asm.txt
- tasks/evidence/2026-06-07-phase11-rp1-gpio-status-diagnostic-core/control-gpio14-status-asm.txt
- tasks/evidence/2026-06-07-phase11-rp1-gpio-status-diagnostic-core/assembly-review.txt

Key observations:

- The real candidate constructs x19 = 0x1f000d0070 and performs exactly one
  ldr w8, [x19], the contracted 32-bit volatile load from RP1 GPIO14 STATUS.
- The real candidate's later 0x1f000d0070 construction is address reporting in
  the terminal marker, not a second MMIO access.
- The no-MMIO control has no contracted RP1 GPIO14 STATUS construction, no
  read_rp1_reg_u32 call, and no RP1 GPIO/RIO/PADS/clock/reset/interrupt load or
  store.
- The retained control-path ldr w10, [x9, #0x18] instructions are UART10 FR
  polling in wait_uart10_empty_early_phase.

## Findings

- fixed: added the real source/static GPIO14 STATUS read candidate selected by
  phase11-rp1-irq-clock-gpio-contract-v1.
- fixed: added a matching no-MMIO control with the same serial field shape and
  explicit address=not-constructed output.
- fixed: added task-owned image, boot-tree, archive, and archive-review helpers
  for both artifacts.
- fixed: retained archive identity, boot-tree identity, image/header metadata,
  strings, and assembly evidence for both candidates.
- deferred: Pi 5 no-MMIO control visibility and real RP1 GPIO14 STATUS proof
  remain queued hardware work.
- not-an-issue: source/static evidence alone accepts no GPIO ownership, pinmux
  change, pad write, interrupt enablement/delivery, clock/reset programming,
  DMA/cache, storage, generated-root, networking, SSH, broader PCIe, or phase
  transition claim.

## Non-Acceptance

This inspection accepts only local source/static/archive evidence. The
serialized no-MMIO Pi 5 control must pass before any RP1 GPIO14 status hardware
proof is attempted.
