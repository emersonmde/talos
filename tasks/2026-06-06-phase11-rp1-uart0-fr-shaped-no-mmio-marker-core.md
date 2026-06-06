# Phase 11 RP1 UART0 FR-Shaped No-MMIO Marker Core

Task id: phase11-rp1-uart0-fr-shaped-no-mmio-marker-core-20260606

Status: accepted

## Goal

Add the smallest no-RP1-MMIO RP1 UART0 FR-shaped marker candidate to determine
whether the selected FR-read scenario reaches its pre-MMIO reporting path when
the volatile RP1 load is absent.

## Scope

- Added boot scenario `rpi5_rp1_uart0_fr_shaped_no_mmio_marker`.
- Routed the selected Pi 5 scenario directly from `rust_entry` into a compact
  repeated UART10 marker after the FR-shaped start/pre-MMIO report lines.
- Stopped before BootInfo parsing, target initialization, boot reports, memory
  planning, allocator setup, scheduler work, PSCI reset, RP1 MMIO, GPIO,
  interrupts, DMA/cache, storage, generated-root, networking, SSH, broader
  PCIe, Milestone 11.2, or phase transition behavior.
- Added task-owned image, boot-tree, archive, and archive-review helper
  scripts.
- Produced a non-published candidate archive at
  `target/talos-rpi5-rp1-uart0-fr-shaped-no-mmio-marker-core.tar.gz`.
- Did not acquire hardwareTestLock, publish a boot archive, power cycle the Pi
  5, or run hardware.

## Candidate

- Archive: `target/talos-rpi5-rp1-uart0-fr-shaped-no-mmio-marker-core.tar.gz`
- Archive SHA-256:
  `05a6801471ffd5cb3ae61f450734728f7980d8a2c4db20b3a6280d83b470a484`
- Boot-tree identity:
  `05f68072e4f1653c10eadfefbe099c92cefdde024b7f7d985b7c785c48011e45`
- Kernel image:
  `target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-rp1-uart0-fr-shaped-no-mmio-marker.img`
- Kernel SHA-256:
  `e8b7d507a0b14e5f18270e65c48b1f4c629923c93bd0eeaeb4b24670b70daabb`
- Kernel size: 45,600 bytes
- Arm64 Image fields: text_offset=0, header_image_size=45600, flags=12,
  magic=ARMd
- Marker: `TALOS: fr-no-mmio-loop`

## Static Path

Static disassembly proves `_start` reaches `rust_entry`, and this scenario's
`rust_entry` contains only a direct branch to
`run_rp1_uart0_fr_shaped_no_mmio_marker`. The marker function writes the
FR-shaped `rpi5-rp1-uart0-fr-read: start` and `pre-mmio-read` lines,
reports `classification=no-mmio-marker-before-rp1-read`, flushes UART10, and
then loops forever writing `TALOS: fr-no-mmio-loop`.

The retained marker disassembly does not construct `0x1f_0003_0018`, does not
call `read_rp1_reg_u32`, and has no RP1 UART0 FR volatile load before the
marker loop. The only retained `ldr w10, [x9, #0x18]` instructions are UART10
PL011 FR polling with `x9 = 0x10_7d00_1000`.

## Findings And Disposition

- fixed: added a no-MMIO discriminator that removes the volatile RP1 UART0 FR
  load while preserving the FR-read-shaped entry and pre-MMIO reporting path.
- fixed: kept the repeated marker compact and unique:
  `TALOS: fr-no-mmio-loop`.
- fixed: produced a non-published candidate archive and retained archive
  SHA-256, boot-tree identity, kernel SHA-256, kernel size, and arm64 Image
  header fields.
- fixed: task-owned archive review confirms the marker is present and forbidden
  FR-read success strings are absent.
- fixed: static disassembly shows the selected path reaches the marker loop
  without RP1 MMIO.
- deferred: visible UART10 marker observability remains a hardware question for
  the queued Pi 5 discriminator.
- not-an-issue: no hardware run, boot publication, RP1 mapped/read-value, RP1
  unmapped/trap, GPIO, interrupts, DMA/cache, storage, generated-root,
  networking, SSH, broader PCIe, Milestone 11.2, or phase transition behavior
  is accepted by this source/static task.

## Evidence

- Evidence map:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-core/evidence-map.json`.
- Static inspection:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-core/static-inspection.md`.
- Archive review:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-core/archive-review.txt`.
- Disassembly:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-core/disassembly-start.txt`,
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-core/disassembly-rust-entry.txt`,
  and
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-core/disassembly-marker.txt`.
- Candidate identity:
  `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-shaped-no-mmio-marker-core/candidate-identity.txt`.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- image/archive inspection:
  `scripts/rpi5-rp1-uart0-fr-shaped-no-mmio-marker-review.sh` passed.
- static image/header/symbol/disassembly inspection: completed and retained.
- git diff --check: passed.
- mdbook build: passed because docs/src files changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as ready-for-fr-shaped-no-mmio-marker-pi5-discriminator.

This task accepts only the source/static candidate and its non-published
archive. It does not accept visible marker serial output, RP1 mapped/read-value
behavior, RP1 unmapped/trap behavior, firmware-state behavior, GPIO,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, or phase transition.
