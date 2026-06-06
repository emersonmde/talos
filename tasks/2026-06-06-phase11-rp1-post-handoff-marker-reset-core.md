# Phase 11 RP1 Post-Handoff Marker Reset Core

Task id: phase11-rp1-post-handoff-marker-reset-core-20260606

Status: accepted

## Goal

Add the smallest no-hardware post-handoff observability discriminator so the
next Pi 5 task can distinguish visible post-handoff UART output from reset-only
handoff reachability before returning to the RP1 UART0 flag-register read.

## Work Performed

- Added the rpi5_rp1_post_handoff_marker_reset boot scenario.
- Routed that scenario at the first Pi 5 rust_entry branch before
  BootInfo::from_aarch64_x0, target::init, boot reports, memory planning,
  allocator setup, scheduler work, or the RP1 UART0 FR read path.
- Emitted the normal TALOS: rust_entry early-phase line, then emitted a unique
  post-handoff marker and classification string through the existing UART10
  early-serial helper before PSCI SYSTEM_RESET.
- Added scripts/rpi5-rp1-post-handoff-marker-reset-image.sh and
  scripts/rpi5-rp1-post-handoff-marker-reset-boot-tree.sh for the task-owned
  candidate image and serial-prefixed boot-tree mirror.
- Produced the non-published candidate archive
  target/talos-rpi5-post-handoff-marker-reset-core.tar.gz.

## Findings And Disposition

- fixed: the accepted handoff-reset proof showed rust_entry reachability only
  by reset side effect; the new candidate makes post-handoff serial visibility a
  direct hardware-observable question.
- fixed: rust_entry branches to the marker/reset candidate before BootInfo
  parsing, target initialization, boot reports, memory planning, allocator
  setup, scheduler work, or the RP1 UART0 FR read.
- fixed: static disassembly shows _start -> rust_entry ->
  run_rp1_post_handoff_marker_reset_diagnostic, the marker writes happen before
  wait_uart10_empty_early_phase, and PSCI SYSTEM_RESET follows the marker path.
- fixed: the archive review retains the accepted Pi 5 boot-tree shape, root and
  da591740/ mirrored kernels, text_offset=0, header_image_size=51736, flags=12,
  and ARMd magic.
- not-an-issue: the image includes the generic exception vector section, but the
  selected scenario's rust_entry path does not parse BootInfo or enter
  target::init before the marker/reset discriminator.
- not-an-issue: static symbol/string checks found no RP1 UART0 FR diagnostic
  symbol or rpi5-rp1-uart0-fr-read strings in the selected image.
- deferred: only the serialized Pi 5 discriminator can classify marker-visible
  plus reset, reset side effect without visible marker, marker path hang/fault,
  or staging/capture blocker.

## Evidence

- Static inspection:
  tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-core/static-inspection.md.
- Candidate identity:
  tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-core/candidate-identity.txt.
- Archive review:
  tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-core/archive-review.log.
- Boot-tree review:
  tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-core/rpi5-rp1-post-handoff-marker-reset-boot-tree.log.
- Source/script/image comparison:
  tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-core/source-script-image-comparison.md.
- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-core/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-core/classification.json.
- Validation summary:
  tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-core/validation-summary.txt.

## Candidate Identity

- archive: target/talos-rpi5-post-handoff-marker-reset-core.tar.gz
- archive SHA-256:
  73a74db1d08d89a3aa371d5329bc6158553bef172a82f0b479598bc29f15acaa
- kernel SHA-256:
  42367beda5de1d0564417e6267a59bd5ae5b770798fa4a3cbb3c0ce101554350
- kernel size: 51,736 bytes

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- image/archive review:
  scripts/rpi5-rp1-post-handoff-marker-reset-boot-tree.sh target/rpi5-local-cat-banner-boot-tree-local1 target/rpi5-rp1-post-handoff-marker-reset-core-boot-tree
  and scripts/rpi5-archive-review.sh
  target/talos-rpi5-post-handoff-marker-reset-core.tar.gz passed.
- static image/header/symbol inspection: passed.
- git diff --check: passed.
- mdbook build: passed because docs/src files changed.
- git diff --cached --check: passed.

## Result

Accepted as ready-for-post-handoff-marker-reset-pi5-discriminator.

The next serialized Pi 5 discriminator may publish only
target/talos-rpi5-post-handoff-marker-reset-core.tar.gz from this task and
classify post-handoff marker visibility, reset side effect, marker-path
hang/fault, staging/capture blocker, and restore state separately. This accepts
no RP1 mapped/unmapped behavior, firmware-state behavior, GPIO, interrupts,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
11.2, or phase transition.
