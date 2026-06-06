# Phase 11 RP1 Entry-Control Handoff Discriminator Core

Task id: phase11-rp1-entry-control-handoff-discriminator-core-20260606

Status: accepted

## Goal

Create the smallest no-hardware source/handoff discriminator for the fetched RP1
entry-control image so the next Pi 5 run can distinguish Rust-entry reachability
from a pre-entry/handoff failure without relying only on UART serial markers.

## Work Performed

- Added the rpi5_rp1_handoff_reset boot scenario.
- Routed that scenario at the first Pi 5 rust_entry branch before
  BootInfo::from_aarch64_x0, target::init, boot reports, memory planning,
  allocator setup, or the RP1 UART0 FR read path.
- Added run_rp1_handoff_reset_diagnostic, which loops on PSCI SYSTEM_RESET
  (0x84000009, smc #0) and performs no RP1 MMIO.
- Added scripts/rpi5-rp1-handoff-reset-image.sh and
  scripts/rpi5-rp1-handoff-reset-boot-tree.sh for the task-owned candidate
  image and serial-prefixed boot-tree mirror.
- Produced the non-published candidate archive
  target/talos-rpi5-rp1-handoff-reset-discriminator-core.tar.gz.

## Findings And Disposition

- fixed: the first-principles blocker is candidate fetch without any fresh
  Talos kernel_main or entry-control marker/PASS; a UART-only Rust marker can
  leave Rust-entry reachability ambiguous when serial is incomplete.
- fixed: the new candidate makes the hardware-visible side effect a repeated
  TFTP boot/fetch sequence from PSCI reset after reaching rust_entry, while
  preserving serial as separate evidence.
- fixed: static disassembly shows _start -> rust_entry ->
  run_rp1_handoff_reset_diagnostic, and the reset diagnostic contains
  mov w0, #0x0009, movk w0, #0x8400, lsl #16, and smc #0.
- fixed: the archive review retains the accepted Pi 5 boot-tree shape, root and
  da591740/ mirrored kernels, text_offset=0, header_image_size=45248, flags=12,
  and ARMd magic.
- not-an-issue: the previous rpi5_rp1_entry_control source candidate remains
  retained as historical accepted evidence, but it is not the selected next
  discriminator because its acceptance signal was serial-only.
- not-an-issue: the handoff-reset image does not contain old RP1 register-read
  or entry-control PASS/classification strings.
- deferred: only the serialized Pi 5 discriminator can prove whether the
  fetched candidate reaches the PSCI reset side effect.

## Evidence

- Source/script/image comparison:
  tasks/evidence/2026-06-06-phase11-rp1-entry-control-handoff-discriminator-core/source-script-image-comparison.md.
- Candidate identity:
  tasks/evidence/2026-06-06-phase11-rp1-entry-control-handoff-discriminator-core/candidate-identity.txt.
- Archive review:
  tasks/evidence/2026-06-06-phase11-rp1-entry-control-handoff-discriminator-core/archive-review.log.
- Boot-tree review:
  tasks/evidence/2026-06-06-phase11-rp1-entry-control-handoff-discriminator-core/rpi5-rp1-handoff-reset-boot-tree.log.
- Symbol/header/section review:
  tasks/evidence/2026-06-06-phase11-rp1-entry-control-handoff-discriminator-core/symbol-header-section-review.txt.
- Side-effect provenance:
  tasks/evidence/2026-06-06-phase11-rp1-entry-control-handoff-discriminator-core/side-effect-provenance.txt.
- Validation summary:
  tasks/evidence/2026-06-06-phase11-rp1-entry-control-handoff-discriminator-core/validation-summary.txt.
- Classification:
  tasks/evidence/2026-06-06-phase11-rp1-entry-control-handoff-discriminator-core/classification.json.

## Candidate Identity

- archive: target/talos-rpi5-rp1-handoff-reset-discriminator-core.tar.gz
- archive SHA-256:
  ee251a145b88df55fd162b0150a82d62a9671906f401948524d27d45929516c6
- kernel SHA-256:
  38170a7fe229b37bfb358479f09d45a14a342af86b16c51d36b3c33023255594
- kernel size: 45,248 bytes

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- image/archive review:
  scripts/rpi5-rp1-handoff-reset-boot-tree.sh target/rpi5-local-cat-banner-boot-tree-local1 target/rpi5-rp1-handoff-reset-discriminator-core-boot-tree
  and
  scripts/rpi5-archive-review.sh target/talos-rpi5-rp1-handoff-reset-discriminator-core.tar.gz
  passed.
- static symbol/header/section inspection: passed.
- static side-effect provenance inspection: passed.
- git diff --check: passed.
- mdbook build: passed because docs/src files changed.
- git diff --cached --check: passed.

## Result

Accepted as ready-for-rp1-handoff-pi5-discriminator.

The next serialized Pi 5 discriminator may publish only
target/talos-rpi5-rp1-handoff-reset-discriminator-core.tar.gz from this task and
classify candidate fetch, repeated TFTP reset side effect, serial marker
visibility, restore state, and any source/handoff/runtime-readiness blocker
separately. This accepts no RP1 mapped/unmapped behavior, firmware-state
behavior, GPIO, interrupts, DMA/cache, storage, networking, SSH, broader PCIe,
Milestone 11.2, or phase transition.
