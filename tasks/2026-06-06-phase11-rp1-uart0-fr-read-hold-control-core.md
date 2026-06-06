# Phase 11 RP1 UART0 FR Read Hold-Control Core

Task id: phase11-rp1-uart0-fr-read-hold-control-core-20260606

Status: accepted

## Goal

Create the smallest source/static RP1 UART0 FR-read candidate that uses the
accepted no-RP1-MMIO hold-marker visibility boundary as its pre-read control
point, then performs exactly one contracted FR load.

## Scope

- Added boot scenario rpi5_rp1_uart0_fr_read_hold_control.
- Routed the selected Pi 5 scenario directly from rust_entry into the
  hold-control diagnostic before BootInfo parsing, target initialization, boot
  reports, memory planning, allocator setup, scheduler work, or command-loop
  work.
- Emitted the unique pre-read control marker
  rpi5-rp1-uart0-fr-read-hold-control: pre-read-control-marker through the
  accepted UART10 early-serial path before the RP1 read.
- Performed exactly one contracted 32-bit volatile load from RP1 UART0 PL011 FR
  at 0x1f00030018 on the selected path.
- Reported contract id phase11-rp1-pcie-map-contract-v1, target
  rp1-uart0-fr-read, address, width, raw value, mapped/read-value
  classification, and a unique post-read terminal hold marker after a returned
  read.
- Added task-owned image, boot-tree, archive, and archive-review helper
  scripts.
- Produced a non-published candidate archive at
  target/talos-rpi5-rp1-uart0-fr-read-hold-control-core.tar.gz.
- Did not acquire hardwareTestLock, publish a boot archive, power cycle the Pi
  5, or run hardware.

## Candidate

- Archive: target/talos-rpi5-rp1-uart0-fr-read-hold-control-core.tar.gz
- Archive SHA-256:
  e9ab45b6dd15e4e80395302a116fb8aa751d699c5b679e5b9cee22077059a9b2
- Boot-tree identity:
  a479f225f68424f228635fcc796b7e5707f6e5b16c3b35ad53f09496786992cf
- Kernel image:
  target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-rp1-uart0-fr-read-hold-control.img
- Kernel SHA-256:
  2e10735026f6abd3d9d919d79f95ebc80caf9ba065d317b5c99b45f75d656eb6
- Kernel size: 46,320 bytes
- Arm64 Image fields: text_offset=0, header_image_size=46320, flags=12,
  magic=ARMd
- Pre-read loop marker: TALOS: fr-hold-control-pre-read-loop
- Pre-read control marker:
  rpi5-rp1-uart0-fr-read-hold-control: pre-read-control-marker
- Post-read marker:
  rpi5-rp1-uart0-fr-read-hold-control: post-read-terminal-hold-marker
- Post-read hold loop marker: TALOS: fr-hold-control-post-read-loop

## Static Path

Static source and disassembly prove _start reaches rust_entry, and this
scenario's rust_entry contains only a direct branch to
run_rp1_uart0_fr_read_hold_control. The selected path writes the FR-read start
and pre-MMIO lines, writes a pre-read-control-before-RP1-read classification,
flushes UART10, emits 32 bounded TALOS: fr-hold-control-pre-read-loop markers,
emits the unique pre-read control marker, flushes UART10 again, then constructs
0x1f00030018 and executes one ldr w19, [x20] 32-bit load from that address.

After a returned read, the selected path writes the contract id, target,
address, width, raw value, mapped/read-value classification, and unique
post-read terminal hold marker, then loops forever writing
TALOS: fr-hold-control-post-read-loop.

The retained ldr w10, [x9, #24] instructions are UART10 PL011 FR polling for
the early-serial output path with x9 = 0x107d001000. The only RP1 contracted
load in the selected diagnostic is the ldr w19, [x20] after x20 is built from
0x18, 0x3 << 16, and 0x1f << 32. No GPIO, interrupt, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.2, or phase
transition behavior is added by this source/static candidate.

## Findings And Disposition

- fixed: added the source/static hold-control FR-read candidate that starts
  from the accepted visible hold-marker boundary instead of another no-MMIO
  marker-only or delayed-marker-only experiment.
- fixed: added the unique pre-read control marker and repeated pre-read loop
  markers through the accepted UART10 early-serial path.
- fixed: retained exactly one contracted 32-bit RP1 UART0 FR volatile load from
  0x1f00030018 on the selected path.
- fixed: retained post-read contract reporting with contract id, target,
  address, width, raw value, mapped/read-value classification, and terminal
  hold marker.
- fixed: produced a non-published candidate archive and retained archive
  SHA-256, boot-tree identity, kernel SHA-256, kernel size, header fields,
  selected marker strings, symbols, and disassembly evidence.
- deferred: candidate fetch, pre-read marker visibility, RP1 read return,
  bus-fault/trap behavior, and restore hygiene remain hardware questions for
  the queued Pi 5 discriminator.
- not-an-issue: local/static acceptance does not claim RP1 mapped/read-value,
  unmapped/trap, firmware-state behavior, Pi 5 marker visibility for this
  candidate, GPIO, interrupts, DMA/cache, storage, generated-root, networking,
  SSH, broader PCIe, Milestone 11.2, or phase transition.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-core/evidence-map.json.
- Static inspection:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-core/static-inspection.md.
- Archive review:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-core/archive-review.txt.
- Candidate identity:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-core/candidate-identity.txt.
- Disassembly:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-core/disassembly-start.txt,
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-core/disassembly-rust-entry.txt,
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-core/disassembly-hold-control.txt, and
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-core/disassembly-review.txt.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed with 423 tests.
- normal rpi5 image build: scripts/rpi5-image.sh passed.
- image/archive inspection:
  scripts/rpi5-rp1-uart0-fr-read-hold-control-review.sh passed.
- static image/header/string/symbol/disassembly inspection: completed and
  retained.
- git diff --check: passed.
- mdbook build: passed because docs/src files changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as local-static-hold-control-rp1-uart0-fr-read-candidate-accepted.

This task accepts only the source/static candidate and its non-published
archive. It does not accept Pi 5 marker visibility for this candidate, RP1
mapped/read-value hardware behavior, RP1 unmapped/trap behavior, firmware-state
behavior, GPIO, interrupts, DMA/cache, storage, generated-root, networking,
SSH, broader PCIe, Milestone 11.2, or phase transition.
