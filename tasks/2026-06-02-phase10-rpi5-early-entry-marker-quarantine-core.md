# Phase 10 RPi5 Early Entry Marker Quarantine Core

Task: phase10-rpi5-early-entry-marker-quarantine-core-20260602
Status: accepted-noninvasive-marker-quarantine-core

## Goal

Quarantine the invasive raw early-entry UART marker path from prompt-capable
RPi5 local command-loop feature/control archives so accepted controls can be
used again as reliable blockers for cd fixed-directories recovery.

## Analysis

The accepted literal-echo control with entry-provenance markers was fetched from
TFTP on Pi 5 hardware but retained no TALOS: asm_start,
TALOS: asm_pre_rust_entry, Rust marker, prompt, classification, or PASS. That
matched the blocked cd entry-provenance candidate. The shared failure after
adding the same raw pre-Rust assembly marker path reframes the blocker from
cd-specific to instrumentation/control-boundary.

The invariant for prompt-capable local feature archives is now: do not require
or include raw UART writes before BSS clear, stack setup, Rust entry, target
initialization, and runtime-console setup unless that raw path is separately
proven non-invasive.

Detailed analysis and evidence are retained at
tasks/evidence/2026-06-02-rpi5-early-entry-marker-quarantine-core/static-analysis.txt.

## Implementation

build.rs no longer defines TALOS_RPI5_EARLY_ENTRY_PROVENANCE_SCENARIO for:

- rpi5_local_literal_echo
- rpi5_local_cd_fixed_dirs

The boot.S raw marker implementation remains present for RPi5 SMP scenarios
that already use early UART diagnostics. This task does not add a new raw early
UART write and does not change cd semantics, literal echo behavior,
descriptor-backed stdio behavior, command-loop vocabulary, userspace execution,
process lifecycle, networking, RP1/PCIe, UART interrupt ownership, DMA, or cache
policy.

## Evidence

Retained evidence directory:

tasks/evidence/2026-06-02-rpi5-early-entry-marker-quarantine-core/.

Key artifacts:

- static-analysis.txt
- qemu-local-literal-echo-smoke.log
- qemu-local-cd-fixed-dirs-smoke.log
- qemu-local-serial-command-loop-smoke.log
- literal-echo-archive-review.txt
- cd-fixed-dirs-archive-review.txt
- literal-echo-strings.txt
- cd-fixed-dirs-strings.txt
- image-sizes.txt
- archive-paths.txt

Fresh archive/string inspection showed:

- literal-echo kernel size 108896 bytes.
- cd fixed-dirs kernel size 110008 bytes.
- both images retain their proof strings and TALOS: command loop proof entered.
- neither image contains TALOS: asm_start or TALOS: asm_pre_rust_entry.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed with existing
  dead_code warnings.
- QEMU/substitute literal echo: scripts/qemu-local-literal-echo-smoke.sh --quiet
  passed.
- QEMU/substitute cd fixed dirs: scripts/qemu-local-cd-fixed-dirs-smoke.sh --quiet
  passed.
- QEMU/substitute command loop regression:
  scripts/qemu-local-serial-command-loop-smoke.sh --quiet passed.
- RPi5 archive/image inspection:
  scripts/rpi5-archive-review.sh passed for the rebuilt literal-echo control
  archive and cd fixed-dirs candidate archive without publishing.
- Static string/image inspection confirmed the raw assembly marker strings are
  absent from both rebuilt images.
- Static diff hygiene: git diff --check and git diff --cached --check passed.
- Docs validation: mdbook build passed.

## Hardware

No boot tree was published, no Pi 5 power-cycle was performed, and
hardwareTestLock remained unlocked/restored and unused.

## Classification

Final classification: accepted-noninvasive-marker-quarantine-core.

## Next Action

The next mechanically unblocked task is the serialized accepted-control
non-invasive regression proof. Only a retained literal-echo prompt/PASS result
from that proof should unblock cd recovery work. The superseded cd
entry-regression path remains blocked.
