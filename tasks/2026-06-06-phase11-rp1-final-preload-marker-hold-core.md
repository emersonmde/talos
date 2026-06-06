# Phase 11 RP1 Final Preload Marker Hold Core

Task id: phase11-rp1-final-preload-marker-hold-core-20260606

Status: accepted

## Goal

Create the smallest no-hardware final-preload-marker hold candidate that proves
whether the delayed FR-read marker path is source/static-visible when the RP1
UART0 FR load is absent.

## Scope

- Added boot scenario rpi5_rp1_final_preload_marker_hold.
- Routed the selected Pi 5 scenario directly from rust_entry into the same
  delayed-marker FR-read-shaped UART10 reporting path through the final
  pre-load marker.
- Preserved the accepted rpi5-rp1-uart0-fr-read: start, pre-mmio-read,
  classification=before-rp1-read, TALOS: fr-delayed-preload-loop, and final
  pre-load marker strings.
- Replaced the RP1 UART0 FR volatile load with a compact repeated UART10 hold
  marker loop: TALOS: fr-final-preload-hold-loop.
- Added task-owned image, boot-tree, archive, and archive-review helper
  scripts.
- Produced a non-published candidate archive at
  target/talos-rpi5-rp1-final-preload-marker-hold-core.tar.gz.
- Did not acquire hardwareTestLock, publish a boot archive, power cycle the Pi
  5, or run hardware.

## Candidate

- Archive: target/talos-rpi5-rp1-final-preload-marker-hold-core.tar.gz
- Archive SHA-256:
  07af64b86908f36c63d368589d79c76aebd492a81906a39586a2c5902d8b9287
- Boot-tree identity:
  ed111afd660d233f95e78a2703c6fd17f12419771e34141ea2dbe3f15ffed3e8
- Kernel image:
  target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-rp1-final-preload-marker-hold.img
- Kernel SHA-256:
  03e26bc9821a9de07d8314bea9d4eabf963224866cd6d64a94390a6ae8b1b8a8
- Kernel size: 45,816 bytes
- Arm64 Image fields: text_offset=0, header_image_size=45816, flags=12,
  magic=ARMd
- Repeated pre-load marker: TALOS: fr-delayed-preload-loop
- Final pre-load marker:
  rpi5-rp1-uart0-fr-read-delayed-marker: final-preload-marker
- Hold marker: TALOS: fr-final-preload-hold-loop

## Static Path

Static disassembly proves _start reaches rust_entry, and this scenario's
rust_entry contains only a direct branch to run_rp1_final_preload_marker_hold.
The hold function writes the FR-read-shaped start, pre-MMIO, and
before-RP1-read classification lines, flushes UART10, emits 32 bounded
TALOS: fr-delayed-preload-loop markers, emits the final pre-load marker,
flushes UART10 again, and then loops forever writing
TALOS: fr-final-preload-hold-loop.

The selected path does not call read_rp1_reg_u32, does not include the
read_rp1_reg_u32 symbol, and does not construct or load from 0x1f_0003_0018.
The only retained ldr w10, [x9, #0x18] instructions are UART10 PL011 FR polling
with x9 = 0x10_7d00_1000; the visible 0x1f immediate in the retained hold
disassembly is the byte length for the repeated pre-load marker string, not an
address component.

The selected path reaches no BootInfo::from_aarch64_x0, target::init, boot
reporting, allocator setup, scheduler work, PSCI reset, RP1 MMIO, GPIO,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, or phase transition behavior before the hold loop.

## Findings And Disposition

- fixed: added a no-RP1-MMIO final-preload-marker hold candidate that preserves
  the delayed-marker FR-read reporting path through the final pre-load marker.
- fixed: replaced the contracted volatile RP1 UART0 FR load with the unique
  repeated hold marker TALOS: fr-final-preload-hold-loop.
- fixed: produced a non-published candidate archive and retained archive
  SHA-256, boot-tree identity, kernel SHA-256, kernel size, and arm64 Image
  header fields.
- fixed: task-owned archive review confirms required start/pre-MMIO,
  before-RP1-read, repeated pre-load, final pre-load, and hold marker strings
  are present.
- fixed: task-owned archive review confirms forbidden RP1 FR-read success
  strings are absent.
- fixed: static symbol and disassembly review proves no read_rp1_reg_u32 call,
  no selected-path construction/use of 0x1f_0003_0018, and no RP1 UART0 FR
  volatile load before the hold loop.
- deferred: visible final pre-load marker output, visible hold marker output,
  candidate fetch, capture/staging, and restore hygiene remain hardware
  questions for the queued Pi 5 discriminator.
- not-an-issue: no hardware run, boot archive publication, RP1 mapped/read-value
  claim, RP1 unmapped/trap claim, firmware-state claim, GPIO, interrupts,
  DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
  Milestone 11.2, or phase transition is accepted by this source/static task.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-core/evidence-map.json.
- Static inspection:
  tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-core/static-inspection.md.
- Archive review:
  tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-core/archive-review.txt.
- Disassembly:
  tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-core/disassembly-start.txt,
  tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-core/disassembly-rust-entry.txt,
  tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-core/disassembly-hold.txt, and
  tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-core/disassembly-review.txt.
- Candidate identity:
  tasks/evidence/2026-06-06-phase11-rp1-final-preload-marker-hold-core/candidate-identity.txt.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed with 423 tests.
- normal rpi5 image build: scripts/rpi5-image.sh passed.
- image/archive inspection:
  scripts/rpi5-rp1-final-preload-marker-hold-review.sh passed.
- static image/header/string/symbol/disassembly inspection: completed and
  retained.
- git diff --check: passed.
- mdbook build: passed because docs/src files changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as ready-for-final-preload-marker-hold-pi5-discriminator.

This task accepts only the source/static candidate and its non-published
archive. It does not accept visible final pre-load marker output, visible hold
marker output, candidate fetch, RP1 mapped/read-value behavior, RP1
unmapped/trap behavior, firmware-state behavior, GPIO, interrupts, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe, Milestone 11.2, or
phase transition.
