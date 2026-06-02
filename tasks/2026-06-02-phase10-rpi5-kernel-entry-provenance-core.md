# Phase 10 RPi5 Kernel Entry Provenance Core

Task: phase10-rpi5-kernel-entry-provenance-core-20260602
Status: accepted

## Goal

Add the smallest static/implementation discriminator needed to explain why the
freshly fetched cd fixed-directories candidate kernel can fail before the first
retained Talos entry marker on Pi 5.

## Review

The current invariant comes from
phase10-pi5-local-cd-fixed-dirs-dtb-scan-progress-proof-20260602 local5:
settled same-cursor TFTP evidence served the candidate kernel twice at 110008
bytes, then fresh post-TFTP serial retained no TALOS: rust_entry, DTB memory
scan marker, prompt, cd transcript, classification, or PASS.

Static review covered the arm64 Image header, linker entry, .text.boot
preservation, _start to rust_entry handoff, boot-tree script routing, and
candidate archive identity. No concrete Image/header/linker/boot-tree defect
was found locally.

Detailed evidence:
tasks/evidence/2026-06-02-rpi5-kernel-entry-provenance-core/static-review.txt.

## Implementation

- build.rs now emits TALOS_RPI5_EARLY_ENTRY_PROVENANCE_SCENARIO only for
  rpi5_local_cd_fixed_dirs.
- src/arch/aarch64/boot.S now emits the existing raw UART assembly markers
  TALOS: asm_start and TALOS: asm_pre_rust_entry for either RPi5 SMP scenarios
  or this cd fixed-directories provenance scenario.

This is a deterministic earliest-practical discriminator. A later Pi 5 run can
classify firmware-fetched-no-entry if neither marker appears, entry-before-rust
if the assembly markers appear without TALOS: rust_entry, or Rust/DTB progress
if later markers appear. It does not change cd command semantics, cwd behavior,
descriptor-backed filesystem behavior, userspace shell execution, process
lifecycle, networking, RP1/PCIe, UART interrupt ownership, DMA, or cache policy.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed, 352 tests.
- QEMU/substitute cd feature:
  scripts/qemu-local-cd-fixed-dirs-smoke.sh --quiet passed.
- QEMU/substitute prompt/readiness regression:
  scripts/qemu-local-serial-command-loop-smoke.sh --quiet passed.
- RPi5 archive/image inspection:
  scripts/rpi5-local-cd-fixed-dirs-boot-tree.sh rebuilt
  target/talos-rpi5-local-cd-fixed-dirs-entry-provenance-core.tar.gz locally
  without publishing, and scripts/rpi5-archive-review.sh passed with
  kernel_size=110008, header_image_size=110008, text_offset=0, flags=12.
- Static entry provenance inspection retained _start at the Image entry,
  rust_entry at 0x201a48, .text.boot at 0x200000, and disassembly showing the
  assembly markers before the rust_entry branch.
- Image marker inspection retained TALOS: asm_start,
  TALOS: asm_pre_rust_entry, TALOS: command loop proof entered, and
  rpi5-local-cd-fixed-dirs-proof.
- Static diff hygiene: git diff --check passed.
- mdBook was not run because mdBook docs were not touched.
- hardwareTestLock remained unlocked/restored and unused.

## Next Action

The original Pi 5 cd fixed-directories proof remains blocked. The next
mechanically queued task is the serialized Pi 5 entry-provenance proof; it can
use these markers to classify the fresh candidate or accept the original cd
feature only if the full pwd/cd transcript and PASS are retained.
