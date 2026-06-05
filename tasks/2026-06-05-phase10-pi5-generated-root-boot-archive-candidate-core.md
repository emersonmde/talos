# Phase 10 Pi 5 Generated-Root Boot Archive Candidate Core

Task: phase10-pi5-generated-root-boot-archive-candidate-core-20260605
Status: accepted
Commit: recorded in talos-supervisor-state.json after the accepted commit is created.

## Goal

Build the non-published Pi 5 candidate boot archive that carries the external
generated-root artifact according to the accepted Pi 5 firmware initramfs
transport contract.

## Implementation

- Added the `rpi5_generated_root_boot_transport` boot scenario, implied by the
  accepted `rpi5_local_serial_command_loop` path.
- Added FDT `/chosen` `linux,initrd-start` and `linux,initrd-end` parsing and a
  Pi 5 startup installer that selects a firmware-loaded generated-root artifact
  as source `firmware-initramfs`, with compiled-fallback behavior for missing or
  oversized bounds and the existing all-or-nothing artifact parser for malformed
  bytes.
- Added Pi 5 proof harness labels, classification, and the five-command
  expected transcript: `rootinfo`, `cat /generated/manifest.txt`,
  `exec /generated/status7 alpha`, `waitpid`, and `laststatus`.
- Added `scripts/rpi5-generated-root-boot-transport-image.sh`,
  `scripts/rpi5-generated-root-boot-transport-boot-tree.sh`, and
  `scripts/rpi5-generated-root-boot-transport-candidate-review.sh`.

## Candidate

Fresh non-published candidate archive:

- archive:
  `target/talos-rpi5-generated-root-boot-transport-candidate-20260605.tar.gz`
- boot tree:
  `target/rpi5-generated-root-boot-transport-candidate-20260605-boot-tree`
- archive sha256:
  `8cb1d731e55f35d13328cf4f618c9dac2bf673311535ddd36038680d8a4ef60e`
- kernel sha256:
  `c44e5a55eb600a09a217c6ad23f665a43d1092a8e982423f5162099c34a42169`
- kernel size: `204888` bytes
- generated-root artifact sha256:
  `0341f5393502f54489acb1951633bf2773fb846a82bde89b3e4a2e82000724c6`
- generated-root artifact size: `662` bytes

The boot tree includes root and `da591740/` copies of `kernel_2712.img`,
`kernel8.img`, `config.txt`, `cmdline.txt`, `bcm2712-rpi-5-b.dtb`, overlays, and
`initramfs_2712`. Both config files contain
`initramfs initramfs_2712 followkernel`. `kernel8.img` matches
`kernel_2712.img`, and `da591740/initramfs_2712` matches the root artifact.

## Evidence

Retained evidence directory:
`tasks/evidence/2026-06-05-phase10-pi5-generated-root-boot-archive-candidate-core/`.

Key artifacts: `source-status.txt`, `archive-review.txt`,
`candidate-review.txt`, `boot-tree-files.txt`, `archive-sha256.txt`,
`kernel-sha256.txt`, `kernel-size.txt`, `generated-root-artifact-sha256.txt`,
`generated-root-artifact-size.txt`, `static-proof-strings.txt`, and
`artifact-proof-strings.txt`.

Static proof-string inspection found the candidate kernel markers
`rpi5-generated-root-boot-transport-proof`,
`pi5-generated-root-boot-transport-complete`, `firmware-initramfs`, `rootinfo`,
and `exec /generated/status7 alpha`. Artifact inspection found
`Talos generated-root external artifact A`, `/generated/manifest.txt`, and
`/generated/status7`.

## Findings

- fixed: Added candidate archive plumbing for root and serial-prefixed
  `initramfs_2712` with the exact `followkernel` config line selected by the
  contract.
- fixed: Added the Pi 5 runtime source path for FDT `/chosen` initramfs bounds
  so the serialized hardware proof can show firmware-provided range/source
  evidence instead of relying on QEMU's fixed loader address.
- fixed: Added a task-owned archive review script that verifies artifact/config
  placement, mirrors, hashes, sizes, and proof strings.
- deferred: Hardware publication, power-cycle, TFTP/serial proof, restore,
  writable persistence, SD/USB/block drivers, networking, SSH, and phase
  transition.
- not-an-issue: The candidate archive was built under `target/` only; the lab
  boot tree was not published and `hardwareTestLock` was not acquired.

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- RPi5 candidate build/static review:
  `scripts/rpi5-generated-root-boot-transport-candidate-review.sh
  target/talos-rpi5-generated-root-boot-transport-candidate-20260605.tar.gz`
  passed.
- RPi5 archive/image inspection:
  `scripts/rpi5-archive-review.sh
  target/talos-rpi5-generated-root-boot-transport-candidate-20260605.tar.gz`
  passed.
- diff hygiene: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

## Hardware

No boot archive was published, no Pi 5 power-cycle was performed, and
hardwareTestLock remained unlocked/restored and unused.

## Non-Goals

This task does not accept hardware proof, writable persistence, SD/USB/block
storage, networking, SSH, unrelated shell feature expansion, or a phase
transition.

## Next Action

The next mechanically unblocked task is
`phase10-pi5-generated-root-boot-transport-proof-20260605` if hardwareTestLock
remains unlocked/restored. It must publish only the retained candidate archive,
capture fresh candidate identity/serial/TFTP evidence, prove Talos consumed the
firmware-loaded generated-root artifact, and restore the prior accepted boot
tree afterward.
