# Phase 11 RP1 UART0 FR Read Delayed-Marker Core

Task id: phase11-rp1-uart0-fr-read-delayed-marker-core-20260606

Status: accepted

## Goal

Create the smallest no-hardware RP1 UART0 FR volatile-read candidate that uses
the accepted FR-shaped UART10 marker visibility, then attempts exactly one
contracted flag-register load.

## Scope

- Added boot scenario rpi5_rp1_uart0_fr_read_delayed_marker.
- Preserved the accepted FR-shaped rpi5-rp1-uart0-fr-read: start and
  pre-mmio-read UART10 reporting path.
- Added 32 bounded TALOS: fr-delayed-preload-loop markers and one final
  rpi5-rp1-uart0-fr-read-delayed-marker: final-preload-marker before the RP1
  load.
- Executed exactly one 32-bit volatile load from contracted address
  0x1f_0003_0018 on the selected path.
- Kept post-load contract/value/classification/PASS output after, and therefore
  control-dependent on, the volatile load returning.
- Added task-owned image, boot-tree, archive, and archive-review helper
  scripts.
- Produced a non-published candidate archive at
  target/talos-rpi5-rp1-uart0-fr-read-delayed-marker-core.tar.gz.
- Did not acquire hardwareTestLock, publish a boot archive, power cycle the Pi
  5, or run hardware.

## Candidate

- Archive: target/talos-rpi5-rp1-uart0-fr-read-delayed-marker-core.tar.gz
- Archive SHA-256:
  90452242f872eb085c9fe7963c02ad67556694326daebd7d199caf4ed5f597f4
- Boot-tree identity:
  bc72d011494343727ebce2a37e4f2d3b14079065f5990100f7c7769f4313fbc6
- Kernel image:
  target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-rp1-uart0-fr-read-delayed-marker.img
- Kernel SHA-256:
  0fd1e68e50da725b416079ae168a89a4b44a6a68aab40c4fff8cabf210db1b6f
- Kernel size: 46,152 bytes
- Arm64 Image fields: text_offset=0, header_image_size=46152, flags=12,
  magic=ARMd
- Repeated marker: TALOS: fr-delayed-preload-loop
- Final pre-load marker:
  rpi5-rp1-uart0-fr-read-delayed-marker: final-preload-marker
- Contracted read address: 0x1f_0003_0018

## Static Path

Static disassembly proves _start reaches rust_entry, and this scenario's
rust_entry contains only a direct branch to
run_rp1_uart0_fr_read_delayed_marker_diagnostic. The delayed-marker function
writes the FR-shaped start/pre-MMIO/classification lines, flushes UART10,
emits the bounded repeated preload marker, emits the final pre-load marker,
flushes UART10 again, and then constructs 0x1f_0003_0018 in x20.

The selected path contains exactly one contracted RP1 load: ldr w19, [x20]
after x20 is built from 0x18, 0x3 << 16, and 0x1f << 32. Post-load contract,
address, raw-value, mapped/read-value, and PASS output appear only after that
load. The other retained ldr w10, [x9, #0x18] instructions are UART10 PL011 FR
polling with x9 = 0x10_7d00_1000.

The selected path reaches no BootInfo::from_aarch64_x0, target::init, boot
reporting, allocator setup, scheduler work, GPIO, interrupts, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe, or Milestone 11.2
behavior before the contracted load.

## Findings And Disposition

- fixed: added the delayed-marker source/static candidate that preserves the
  accepted FR-shaped UART10 reporting path and performs the contracted RP1 UART0
  FR load only after explicit repeated and final pre-load markers.
- fixed: retained source/static, archive, string, symbol, and disassembly
  evidence for the selected path.
- fixed: task-owned archive review confirms required marker, contract,
  classification, and PASS strings are present.
- fixed: static disassembly proves exactly one contracted 32-bit load from
  0x1f_0003_0018 on the selected path, with post-load output after the load.
- deferred: visible final pre-load marker output, post-load output,
  trap/no-return behavior, and restore hygiene remain hardware questions for
  the queued Pi 5 discriminator.
- not-an-issue: no hardware run, boot archive publication, GPIO, interrupts,
  DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
  Milestone 11.2, or phase transition behavior is accepted by this source/static
  task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-core/evidence-map.json.
- Static inspection:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-core/static-inspection.md.
- Archive review:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-core/archive-review.txt.
- Disassembly:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-core/disassembly-start.txt,
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-core/disassembly-rust-entry.txt,
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-core/disassembly-delayed-marker.txt, and
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-core/disassembly-review.txt.
- Candidate identity:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-delayed-marker-core/candidate-identity.txt.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- normal rpi5 image build: scripts/rpi5-image.sh passed.
- diagnostic candidate image/archive build: passed.
- archive/string/header review:
  scripts/rpi5-rp1-uart0-fr-read-delayed-marker-review.sh passed.
- targeted disassembly review: completed and retained.
- git diff --check: passed.
- mdbook build: passed because docs/src files changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as ready-for-delayed-marker-rp1-fr-read-pi5-discriminator.

This task accepts only the source/static candidate and its non-published
archive. It does not accept visible final pre-load marker output,
mapped/read-value behavior, trap/no-return behavior, firmware-state behavior,
GPIO, interrupts, DMA/cache, storage, generated-root, networking, SSH, broader
PCIe, Milestone 11.2, or phase transition.
