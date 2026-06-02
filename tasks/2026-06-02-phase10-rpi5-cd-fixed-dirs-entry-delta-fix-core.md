# Phase 10 RPi5 Cd Fixed Dirs Entry Delta Fix Core

Task: phase10-rpi5-cd-fixed-dirs-entry-delta-fix-core-20260602
Status: accepted-fix-candidate-produced
Commit: recorded in talos-supervisor-state.json after the accepted commit is
created.

## Goal

Find and correct the smallest target-specific cd fixed-directories delta that
could explain the Pi 5 cd candidate being fetched over TFTP but failing before
Talos kernel-entry while the non-invasive literal-echo control remains
prompt-capable.

## Delta Hypothesis

The blocked non-invasive cd candidate proved TFTP fetch of a 110008-byte
kernel but retained only firmware/RP1 serial output. The accepted non-invasive
literal-echo control proved prompt-capable serial behavior with a 108896-byte
kernel. The concrete cd-only delta selected here is the RPi5 hardware proof
prelude: the cd proof harness was still running help/status/stdio before the
feature transcript, even though the original acceptance only requires the
pwd/cd sequence, cwd outputs, rejected missing directory with cwd unchanged,
ready prompt, final classification, and PASS.

This task changes only the RPi5 cd proof harness command plan from twelve
commands to nine commands:

- pwd
- cd /etc
- pwd
- cd /bin
- pwd
- cd /
- pwd
- cd /missing
- pwd

QEMU/substitute cd fixed-directories remains unchanged and still exercises the
broader local regression script.

## Candidate

Fresh fix candidate archive:

- archive: target/talos-rpi5-local-cd-fixed-dirs-entry-delta-fix-core.tar.gz
- archive sha256:
  52eb5d54aef19044ae9af0689786f49bc573b46bbff0d71956c3fbe6b45011a3
- kernel sha256:
  c9b174c3fe087ac6c887c102c9b2a8fe143ea265027d3bab498eb8c581e7464f
- kernel size: 109800 bytes

The selected candidate is 208 bytes smaller than the prior blocked
non-invasive cd candidate and 904 bytes larger than the accepted non-invasive
literal-echo control. Archive review passed, the arm64 Image header size
matches the file size, flags remain 12, and the image retains
rpi5-local-cd-fixed-dirs-proof, pi5-local-cd-fixed-dirs-complete, and
TALOS: command loop proof entered while omitting TALOS: asm_start and
TALOS: asm_pre_rust_entry.

## Evidence

Retained evidence directory:

tasks/evidence/2026-06-02-rpi5-cd-fixed-dirs-entry-delta-fix-core/.

Key artifacts:

- static-delta-analysis.txt
- static-image-map-comparison.txt
- qemu-local-cd-fixed-dirs-smoke.log
- qemu-local-literal-echo-smoke.log
- qemu-local-serial-command-loop-smoke.log
- cd-fixed-dirs-archive-review.txt
- cd-fixed-dirs-boot-tree-files.txt
- cd-fixed-dirs-key-strings.txt
- archive-sha256.txt
- kernel-sha256.txt
- kernel-size.txt

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed with existing
  dead_code warnings.
- QEMU/substitute cd fixed dirs:
  scripts/qemu-local-cd-fixed-dirs-smoke.sh --quiet passed, with final
  qemu-local-cd-fixed-dirs-complete classification and
  qemu-local-cd-fixed-dirs: PASS.
- QEMU/substitute literal echo:
  scripts/qemu-local-literal-echo-smoke.sh --quiet passed, with final
  qemu-local-literal-echo-complete classification and
  qemu-local-literal-echo: PASS.
- QEMU/substitute command-loop regression:
  scripts/qemu-local-serial-command-loop-smoke.sh --quiet passed, with final
  qemu-local-serial-command-loop-complete classification and
  qemu-local-serial-command-loop: PASS.
- RPi5 archive/image inspection:
  scripts/rpi5-archive-review.sh passed for the fresh cd entry-delta fix
  candidate archive without publishing.
- Static image/string/map inspection retained the expected proof strings,
  omitted quarantined raw assembly marker strings, and recorded ELF section
  layout and kernel size deltas.
- Static diff hygiene: git diff --check passed.
- Docs validation: mdbook build passed.
- Staged static diff hygiene: git diff --cached --check passed before commit.

## Hardware

No boot archive was published, no Pi 5 power-cycle was performed, and
hardwareTestLock remained unlocked/restored and unused.

## Classification

Final classification: accepted-fix-candidate-produced.

## Next Action

The next mechanically unblocked task is
phase10-pi5-local-cd-fixed-dirs-entry-delta-proof-20260602 if hardwareTestLock
remains unlocked/restored and supervisorIntervention remains inactive. It must
publish only the fresh
target/talos-rpi5-local-cd-fixed-dirs-entry-delta-fix-core.tar.gz archive and
may accept the original cd proof only with the full retained pwd/cd feature
transcript, ready prompt, pi5-local-cd-fixed-dirs-complete, and
rpi5-local-cd-fixed-dirs-proof: PASS.
