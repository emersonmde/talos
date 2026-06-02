# Phase 10 RPi5 Accepted Control Entry Provenance Core

Task: phase10-rpi5-accepted-control-entry-provenance-core-20260602
Status: accepted-control-marker-core

## Goal

Add a narrow entry-provenance marker path to one already accepted
prompt-capable Pi 5 control so the next hardware proof can distinguish entry
and serial-capture health from the blocked cd fixed-directories candidate.

## Selected Control

The selected control is rpi5_local_literal_echo.

The prior accepted Pi 5 proof is recorded in
tasks/2026-05-31-phase10-pi5-local-literal-echo-proof.md, with retained PASS
evidence at
tasks/evidence/2026-05-31-pi5-local-literal-echo-proof/local3-candidate-summary/proof-result-local3.txt.

Literal echo is the smallest useful prompt-capable control for this
discriminator: it has one typed command, descriptor-backed stdio markers,
visible output, ready-next evidence, final classification, and PASS. It avoids
the larger cd cwd sequence while still exercising the prompt-capable local
command-loop path.

## Implementation

- build.rs now emits TALOS_RPI5_EARLY_ENTRY_PROVENANCE_SCENARIO for
  rpi5_local_literal_echo.
- src/arch/aarch64/boot.S is unchanged. The existing provenance path emits
  TALOS: asm_start just after the arm64 Image header branch and
  TALOS: asm_pre_rust_entry after BSS clear and stack setup, before
  rust_entry.

No shell-visible literal echo behavior changed. This task does not change cd
semantics, descriptor-backed stdio behavior, filesystem behavior, userspace
execution, process lifecycle, networking, RP1/PCIe, UART interrupt ownership,
DMA, or cache policy.

## Static Review

Detailed evidence is retained at
tasks/evidence/2026-06-02-rpi5-accepted-control-entry-provenance-core/static-review.txt.

The selected literal echo control archive passed scripts/rpi5-archive-review.sh
with kernel_size=108896, header_image_size=108896, text_offset=0, and flags=12.

The comparison against the blocked cd entry-provenance candidate found:

- both images use text_offset=0, flags=12, and magic=ARMd.
- both ELFs enter at 0x200000, with __kernel_start and _start at 0x200000.
- both keep .text.boot at 0x200000, place .text at 0x201000, and keep
  rust_entry at 0x201a48.
- both disassemblies show _start writing asm_start, clearing BSS, setting the
  stack, writing asm_pre_rust_entry, and branching to rust_entry.
- the differences are expected scenario payload differences: literal echo is a
  one-command proof and cd is a twelve-command cwd proof.

No concrete Image/header/linker/.text.boot/build-routing defect was found
beyond adding the same earliest entry marker path to the selected accepted
control.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- QEMU/substitute selected control:
  scripts/qemu-local-literal-echo-smoke.sh --quiet passed.
- RPi5 archive/image inspection:
  scripts/rpi5-local-literal-echo-boot-tree.sh rebuilt
  target/talos-rpi5-local-literal-echo-entry-provenance-control.tar.gz
  locally without publishing, and scripts/rpi5-archive-review.sh passed.
- Static string/disassembly inspection retained TALOS: asm_start and
  TALOS: asm_pre_rust_entry before rust_entry in the selected control.
- Static comparison against the blocked cd entry-provenance candidate retained
  header, linker, .text.boot, rust_entry, proof-main selection, and boot-tree
  routing evidence.
- Static diff hygiene: git diff --check passed.
- mdBook was not run because docs/src was not touched.
- hardwareTestLock remained unlocked/restored and unused.

## Classification

Final classification: accepted-control-marker-core.

## Next Action

The next queued Pi 5 accepted-control entry-provenance proof can publish only
this selected control archive and classify whether earliest entry marker capture
is healthy on an already prompt-capable control before any cd candidate rerun.
