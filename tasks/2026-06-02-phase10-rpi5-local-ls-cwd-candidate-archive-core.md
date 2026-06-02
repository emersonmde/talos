# Phase 10 RPi5 Local Ls Cwd Candidate Archive Core

Task: phase10-rpi5-local-ls-cwd-candidate-archive-core-20260602
Status: accepted
Commit: recorded in talos-supervisor-state.json after the accepted commit is created.

## Goal

Create the smallest RPi5 candidate/archive path for the accepted bare `ls` current-directory feature so the serialized Pi 5 proof can publish an accepted archive without changing code or guessing at an older scenario.

## Implementation

- Added the `rpi5_local_ls_cwd` boot scenario in `build.rs`, implied by the existing `rpi5_local_serial_command_loop` route.
- Added RPi5 proof-harness labels, final classification, nine-command expected dispatch plan, and visible `ls-cwd-observed` markers in `src/target/rpi5.rs`.
- Added `scripts/rpi5-local-ls-cwd-image.sh` and `scripts/rpi5-local-ls-cwd-boot-tree.sh` for target-specific image and boot-tree/archive staging.

The selected RPi5 scenario identity is `rpi5_local_ls_cwd`. It corresponds to the accepted bare `ls` cwd transcript rather than older command-loop, cd, ls-root, or ls-bin scenarios because its proof strings are `rpi5-local-ls-cwd-proof`, `pi5-local-ls-cwd-complete`, and `ls-cwd-observed`, and its expected command plan is `pwd`, bare `ls` at `/`, `cd /etc`, bare `ls` at `/etc`, `cd /bin`, bare `ls` at `/bin`, `cd /`, bare `ls` at `/`, and `bogus`.

## Candidate

Fresh candidate archive:

- archive: `target/talos-rpi5-local-ls-cwd-candidate-archive-core.tar.gz`
- archive sha256: `1f986f73b793b269e5b7aa0cf34cfc4cbf3b58358b0d9b409181e762b986919e`
- kernel sha256: `da6bb65ad8529912e1feca037d6f1e3cfbc46c5ea052ee32a1ab669b000bfd3e`
- kernel size: 110624 bytes

Archive review passed with `kernel_2712.img` and `kernel8.img` mirrored at the root and `da591740/` prefixed boot tree, arm64 Image header size equal to file size, text offset 0, flags 12, and no loader diagnostic mode.

Static proof-string inspection found `rpi5-local-ls-cwd-proof`, `pi5-local-ls-cwd-complete`, `ls-cwd-observed`, and `TALOS: command loop proof entered`. The quarantined raw assembly markers `TALOS: asm_start` and `TALOS: asm_pre_rust_entry` were absent.

## Evidence

Retained evidence directory: `tasks/evidence/2026-06-02-rpi5-local-ls-cwd-candidate-archive-core/`.

Key artifacts: `archive-review.txt`, `archive-sha256.txt`, `boot-tree-files.txt`, `kernel-sha256.txt`, `kernel-size.txt`, `qemu-local-ls-cwd-smoke.log`, `qemu-local-cd-fixed-dirs-smoke.log`, `qemu-local-literal-echo-smoke.log`, `qemu-local-serial-command-loop-smoke.log`, and `static-proof-strings.txt`.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 353 tests.
- QEMU/substitute bare `ls` cwd: `scripts/qemu-local-ls-cwd-smoke.sh` passed with final classification `qemu-local-ls-cwd-complete` and exact `qemu-local-ls-cwd: PASS`.
- QEMU/substitute cd fixed-dirs: `scripts/qemu-local-cd-fixed-dirs-smoke.sh --quiet` passed.
- QEMU/substitute literal echo: `scripts/qemu-local-literal-echo-smoke.sh --quiet` passed.
- QEMU/substitute command-loop regression: `scripts/qemu-local-serial-command-loop-smoke.sh --quiet` passed.
- RPi5 archive/image inspection: `scripts/rpi5-archive-review.sh target/talos-rpi5-local-ls-cwd-candidate-archive-core.tar.gz` passed.
- Static image inspection retained the expected scenario/proof strings and omitted quarantined raw assembly entry markers.
- Static diff hygiene: `git diff --check` passed.

## Hardware

No boot archive was published, no Pi 5 power-cycle was performed, and hardwareTestLock remained unlocked/restored and unused.

## Non-Goals

This task does not change accepted bare `ls` cwd semantics; accept `ls /etc` as a new explicit command form; broaden relative paths, `.` or `..`; implement arbitrary path listing, POSIX `chdir`, descriptor-backed filesystem syscalls, userspace shell execution, process cwd inheritance, globbing, quoting, pipes, redirection, networking, SSH, RP1/PCIe, UART interrupt ownership, DMA, or cache-driver policy; or revisit the older blocked `ls /bin` proof strategy.

## Classification

Final classification: accepted.

## Next Action

The next mechanically unblocked task is phase10-pi5-local-ls-cwd-proof-20260602 if hardwareTestLock remains unlocked/restored and supervisorIntervention remains inactive. It must publish only `target/talos-rpi5-local-ls-cwd-candidate-archive-core.tar.gz` and may accept the feature only with the retained Pi 5 serial transcript, settled TFTP evidence, restore proof, `pi5-local-ls-cwd-complete`, and `rpi5-local-ls-cwd-proof: PASS`.
