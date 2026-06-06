# Static Inspection

Task: phase11-rp1-uart0-fr-shaped-no-mmio-marker-core-20260606

## Source

- Added boot scenario `rpi5_rp1_uart0_fr_shaped_no_mmio_marker`.
- `rust_entry` branches directly to
  `target::rpi5::run_rp1_uart0_fr_shaped_no_mmio_marker()` for that
  scenario and excludes it from the normal BootInfo/target initialization path.
- The selected path emits the same FR-read-shaped `start` and
  `pre-mmio-read` UART10 lines used by the RP1 UART0 FR-read candidate, then
  emits
  `classification=no-mmio-marker-before-rp1-read`, flushes UART10, and loops
  on the repeated marker `TALOS: fr-no-mmio-loop`.
- The task path does not call `read_rp1_reg_u32`, does not reference
  `RP1_UART0_FR`, and does not execute the RP1 UART0 FR volatile load.

## Image And Archive

- archive: `target/talos-rpi5-rp1-uart0-fr-shaped-no-mmio-marker-core.tar.gz`
- archive SHA-256:
  `05a6801471ffd5cb3ae61f450734728f7980d8a2c4db20b3a6280d83b470a484`
- boot-tree identity:
  `05f68072e4f1653c10eadfefbe099c92cefdde024b7f7d985b7c785c48011e45`
- root kernel SHA-256:
  `e8b7d507a0b14e5f18270e65c48b1f4c629923c93bd0eeaeb4b24670b70daabb`
- prefixed kernel SHA-256:
  `e8b7d507a0b14e5f18270e65c48b1f4c629923c93bd0eeaeb4b24670b70daabb`
- kernel size: 45,600 bytes
- arm64 Image header: `text_offset=0`, `header_image_size=45600`,
  `flags=12`, `magic=ARMd`

## Disassembly

Retained disassembly:

- `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-core/disassembly-start.txt`
- `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-core/disassembly-rust-entry.txt`
- `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-core/disassembly-marker.txt`

Key observations:

- `rust_entry` contains a direct branch to
  `run_rp1_uart0_fr_shaped_no_mmio_marker`.
- The marker function performs three `write_early_static` calls for the
  FR-shaped start/pre-MMIO/classification lines, then flushes UART10.
- The only `ldr w10, [x9, #0x18]` instructions in the retained marker
  disassembly are UART10 PL011 FR polling: `x9` is built as
  `0x10_7d00_1000` before each polling loop.
- The retained marker disassembly does not construct `0x1f_0003_0018`, does
  not call `read_rp1_reg_u32`, and has no `ldr` from the RP1 UART0 FR
  address before the repeated marker loop.
- The candidate path reaches no `BootInfo::from_aarch64_x0`, `target::init`,
  boot reporting, allocator setup, scheduler work, PSCI reset, GPIO, interrupt,
  DMA/cache, storage, generated-root, networking, SSH, broader PCIe, or
  Milestone 11.2 behavior before the marker loop.

## Findings

- fixed: added a new source/static discriminator that removes the volatile RP1
  UART0 FR load while preserving the selected FR-read-shaped entry and
  pre-MMIO reporting path.
- fixed: the marker is compact, unique, and repeated:
  `TALOS: fr-no-mmio-loop`.
- fixed: produced a non-published candidate archive and retained archive
  SHA-256, boot-tree identity, kernel SHA-256, kernel size, and arm64 Image
  header fields.
- fixed: task-owned archive review confirms the marker is present and forbidden
  FR-read success strings are absent.
- fixed: static disassembly proves the candidate reaches the repeated marker
  loop without RP1 MMIO.
- deferred: visible Pi 5 serial output from this marker remains a hardware
  question for the queued discriminator.
- not-an-issue: no hardware run, boot archive publication, RP1 mapped/read-value
  claim, RP1 unmapped/trap claim, GPIO, interrupts, DMA/cache, storage,
  generated-root, networking, SSH, broader PCIe, Milestone 11.2, or phase
  transition is accepted by this source/static task.

## Non-Acceptance

This inspection accepts only the source/static candidate and non-published
archive. It does not accept Pi 5 marker visibility, RP1 mapped/read-value
behavior, RP1 unmapped/trap behavior, firmware-state behavior, GPIO ownership,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, or phase transition behavior.
