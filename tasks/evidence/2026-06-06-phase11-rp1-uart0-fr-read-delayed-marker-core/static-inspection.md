# Static Inspection

Task: phase11-rp1-uart0-fr-read-delayed-marker-core-20260606

## Source

- Added boot scenario rpi5_rp1_uart0_fr_read_delayed_marker.
- rust_entry branches directly to
  target::rpi5::run_rp1_uart0_fr_read_delayed_marker_diagnostic() for that
  scenario and excludes it from the normal BootInfo/target initialization path.
- The selected path emits the accepted FR-shaped
  rpi5-rp1-uart0-fr-read: start and pre-mmio-read UART10 lines, then emits
  classification=before-rp1-read.
- The path emits 32 bounded TALOS: fr-delayed-preload-loop markers and one
  explicit final pre-load marker before the RP1 load.
- The path calls read_rp1_reg_u32(RP1_UART0_FR) exactly once. RP1_UART0_FR is
  asserted elsewhere as 0x1f_0003_0018.
- Post-load contract, raw-value, mapped/read-value, and PASS output occur only
  after the volatile load returns.

## Image And Archive

- archive: target/talos-rpi5-rp1-uart0-fr-read-delayed-marker-core.tar.gz
- archive SHA-256:
  90452242f872eb085c9fe7963c02ad67556694326daebd7d199caf4ed5f597f4
- boot-tree identity:
  bc72d011494343727ebce2a37e4f2d3b14079065f5990100f7c7769f4313fbc6
- root kernel SHA-256:
  0fd1e68e50da725b416079ae168a89a4b44a6a68aab40c4fff8cabf210db1b6f
- prefixed kernel SHA-256:
  0fd1e68e50da725b416079ae168a89a4b44a6a68aab40c4fff8cabf210db1b6f
- kernel size: 46,152 bytes
- arm64 Image header: text_offset=0, header_image_size=46152, flags=12,
  magic=ARMd

## Disassembly

Retained disassembly:

- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-core/disassembly-start.txt
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-core/disassembly-rust-entry.txt
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-core/disassembly-delayed-marker.txt
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-core/disassembly-review.txt

Key observations:

- rust_entry contains a direct branch to
  run_rp1_uart0_fr_read_delayed_marker_diagnostic.
- The delayed-marker function writes the start, pre-MMIO, and classification
  strings before entering the bounded repeated marker loop.
- The loop marker writes happen before the final pre-load marker.
- The final pre-load marker write and UART10 flush complete before the RP1
  address is constructed.
- The contracted address is constructed in x20 as 0x18, 0x3 << 16, and
  0x1f << 32, then exactly one ldr w19, [x20] executes.
- The post-load contract, address, raw-value, mapped/read-value, and PASS
  writes follow the ldr w19, [x20].
- The retained ldr w10, [x9, #0x18] instructions are UART10 PL011 FR polling
  with x9 = 0x10_7d00_1000.
- The candidate path reaches no BootInfo::from_aarch64_x0, target::init, boot
  reporting, allocator setup, scheduler work, GPIO, interrupts, DMA/cache,
  storage, generated-root, networking, SSH, broader PCIe, or Milestone 11.2
  behavior before the contracted load.

## Findings

- fixed: added the delayed-marker source/static candidate while preserving the
  accepted FR-shaped UART10 reporting path.
- fixed: the candidate emits bounded repeated and final pre-load markers before
  the contracted volatile load.
- fixed: produced a non-published candidate archive and retained archive
  SHA-256, boot-tree identity, kernel SHA-256, kernel size, and arm64 Image
  header fields.
- fixed: task-owned archive review confirms the marker, contract, mapped-value
  classification, and PASS strings are present.
- fixed: static disassembly proves exactly one contracted RP1 UART0 FR load and
  post-load output ordering.
- deferred: visible final pre-load marker output, post-load output,
  trap/no-return behavior, and restore hygiene remain hardware questions for
  the queued Pi 5 discriminator.
- not-an-issue: no hardware run, boot archive publication, GPIO, interrupts,
  DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
  Milestone 11.2, or phase transition is accepted by this source/static task.

## Non-Acceptance

This inspection accepts only the source/static candidate and non-published
archive. It does not accept Pi 5 final marker visibility, RP1 mapped/read-value
behavior, RP1 trap/no-return behavior, firmware-state behavior, GPIO ownership,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, or phase transition behavior.
