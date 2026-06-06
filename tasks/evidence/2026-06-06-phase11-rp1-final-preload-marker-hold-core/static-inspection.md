# Static Inspection

Task: phase11-rp1-final-preload-marker-hold-core-20260606

## Source

- Added boot scenario rpi5_rp1_final_preload_marker_hold.
- rust_entry branches directly to
  target::rpi5::run_rp1_final_preload_marker_hold() for that scenario and
  excludes it from the normal BootInfo/target initialization path.
- The selected path emits the delayed-marker FR-read-shaped start,
  pre-mmio-read, and classification=before-rp1-read UART10 lines.
- The path emits 32 bounded TALOS: fr-delayed-preload-loop markers and one
  explicit final pre-load marker.
- After the final pre-load marker, the path loops on
  TALOS: fr-final-preload-hold-loop instead of executing RP1 MMIO.
- The task path does not call read_rp1_reg_u32, does not reference
  RP1_UART0_FR, and does not execute the RP1 UART0 FR volatile load.

## Image And Archive

- archive: target/talos-rpi5-rp1-final-preload-marker-hold-core.tar.gz
- archive SHA-256:
  07af64b86908f36c63d368589d79c76aebd492a81906a39586a2c5902d8b9287
- boot-tree identity:
  ed111afd660d233f95e78a2703c6fd17f12419771e34141ea2dbe3f15ffed3e8
- root kernel SHA-256:
  03e26bc9821a9de07d8314bea9d4eabf963224866cd6d64a94390a6ae8b1b8a8
- prefixed kernel SHA-256:
  03e26bc9821a9de07d8314bea9d4eabf963224866cd6d64a94390a6ae8b1b8a8
- kernel size: 45,816 bytes
- arm64 Image header: text_offset=0, header_image_size=45816, flags=12,
  magic=ARMd

## Disassembly

Retained disassembly:

- tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-core/disassembly-start.txt
- tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-core/disassembly-rust-entry.txt
- tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-core/disassembly-hold.txt
- tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-core/disassembly-review.txt

Key observations:

- rust_entry contains a direct branch to run_rp1_final_preload_marker_hold.
- The hold function writes the FR-shaped start/pre-MMIO/classification lines,
  flushes UART10, enters the bounded repeated preload marker loop, emits the
  final pre-load marker, flushes UART10 again, and then loops on the unique hold
  marker.
- The only ldr w10, [x9, #0x18] instructions in the retained hold disassembly
  are UART10 PL011 FR polling: x9 is built as 0x10_7d00_1000 before each
  polling loop.
- The retained symbol table contains run_rp1_final_preload_marker_hold and does
  not contain read_rp1_reg_u32.
- The retained hold disassembly does not construct 0x1f_0003_0018, does not
  call read_rp1_reg_u32, and has no ldr from the RP1 UART0 FR address before
  the hold loop.
- The candidate path reaches no BootInfo::from_aarch64_x0, target::init, boot
  reporting, allocator setup, scheduler work, PSCI reset, RP1 MMIO, GPIO,
  interrupt, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
  or Milestone 11.2 behavior before the hold loop.

## Findings

- fixed: added a new source/static discriminator that keeps the delayed-marker
  FR-read reporting path through the final pre-load marker while removing the
  volatile RP1 UART0 FR load.
- fixed: the hold marker is compact, unique, and repeated:
  TALOS: fr-final-preload-hold-loop.
- fixed: produced a non-published candidate archive and retained archive
  SHA-256, boot-tree identity, kernel SHA-256, kernel size, and arm64 Image
  header fields.
- fixed: task-owned archive review confirms required marker strings are present
  and forbidden RP1 FR-read success strings are absent.
- fixed: static symbol and disassembly review proves the candidate reaches the
  hold loop without RP1 MMIO.
- deferred: visible Pi 5 final pre-load marker and hold marker output remain
  hardware questions for the queued discriminator.
- not-an-issue: no hardware run, boot archive publication, RP1 mapped/read-value
  claim, RP1 unmapped/trap claim, GPIO, interrupts, DMA/cache, storage,
  generated-root, networking, SSH, broader PCIe, Milestone 11.2, or phase
  transition is accepted by this source/static task.

## Non-Acceptance

This inspection accepts only the source/static candidate and non-published
archive. It does not accept Pi 5 final marker visibility, Pi 5 hold marker
visibility, RP1 mapped/read-value behavior, RP1 unmapped/trap behavior,
firmware-state behavior, GPIO ownership, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.2, or phase
transition behavior.
