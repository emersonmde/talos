# Phase 10 Pi 5 Generated-Root Boot Transport Contract

Status: accepted as the contract for the Milestone 10.3 Pi 5 generated-root
boot-transport candidate.

This contract adds no runtime behavior, publishes no archive, runs no Pi 5
hardware, and acquires no hardware lock. It selects the boot-archive and
evidence shape for the next candidate task.

## Selected Path

The Pi 5 candidate transport uses the Raspberry Pi firmware initramfs envelope
to load the existing talos-generated-root-v1 artifact bytes outside the kernel
image:

initramfs initramfs_2712 followkernel

The candidate archive must include identical artifact files at both paths:

initramfs_2712
da591740/initramfs_2712

Both root and da591740/ config.txt files must contain the same initramfs line.
The proof task must record which path the firmware actually fetched after a
fresh TFTP cursor.

## Runtime Source Contract

QEMU's accepted generated-root transport uses a fixed loader-device address at
0x47000000. That address is not a Pi 5 hardware placement contract.

The Pi 5 candidate must derive the artifact range from firmware-provided FDT
/chosen initramfs bounds, conventionally linux,initrd-start and
linux,initrd-end. Serial evidence must report the range, length, digest, and
source before claiming that generated-root reads or execs used the external
artifact.

Invalid or missing firmware initramfs evidence falls back to the compiled
generated-root image. Malformed external artifact bytes must never partially
merge into the VFS.

## Candidate Archive Contract

The non-published candidate archive should be named:

target/talos-rpi5-generated-root-boot-transport-candidate-20260605.tar.gz

Required static evidence:

- source commit and clean/conflict-free status;
- archive SHA-256;
- kernel image SHA-256 and size;
- external generated-root artifact SHA-256 and size;
- root and serial-prefix file equality for kernel images, config, and artifact;
- boot tree listing;
- config proof for initramfs initramfs_2712 followkernel;
- proof strings for rootinfo, /generated/manifest.txt, /generated/status7, and
  the final classification/PASS marker.

The candidate task must not publish the archive or touch hardware.

## Candidate Implementation

`phase10-pi5-generated-root-boot-archive-candidate-core-20260605` accepted the
non-published candidate archive at the contract path. The candidate is built by
`scripts/rpi5-generated-root-boot-transport-boot-tree.sh` and reviewed by
`scripts/rpi5-generated-root-boot-transport-candidate-review.sh`.

The candidate kernel includes the `rpi5_generated_root_boot_transport` scenario.
At Pi 5 startup it reads `/chosen` `linux,initrd-start` and `linux,initrd-end`,
installs the selected artifact as generated-root source `firmware-initramfs`,
and falls back to the compiled generated-root image for missing or oversized
firmware bounds or malformed artifact bytes. The proof harness is limited to
`rootinfo`, `cat /generated/manifest.txt`, `exec /generated/status7 alpha`,
`waitpid`, and `laststatus`.

Accepted candidate identity:

- archive SHA-256:
  `8cb1d731e55f35d13328cf4f618c9dac2bf673311535ddd36038680d8a4ef60e`
- kernel SHA-256:
  `c44e5a55eb600a09a217c6ad23f665a43d1092a8e982423f5162099c34a42169`
- generated-root artifact SHA-256:
  `0341f5393502f54489acb1951633bf2773fb846a82bde89b3e4a2e82000724c6`
- retained evidence:
  `tasks/evidence/2026-06-05-phase10-pi5-generated-root-boot-archive-candidate-core/`

## Hardware Proof Contract

The later hardware proof must own hardwareTestLock, publication, power-cycle,
TFTP delta capture, serial capture, and restore. Acceptance requires:

- candidate identity;
- fresh serial cursor;
- fresh TFTP cursor and delta showing candidate kernel plus generated-root
  artifact fetch;
- serial proof that Talos consumed the firmware-loaded artifact and observed the
  generated file content and generated executable status;
- prompt/readiness and PASS/classification markers;
- post-run restore evidence.

Any inconclusive run must follow the standard triage sequence before code
changes: candidate identity, fresh serial cursor, TFTP delta, known-good
control, then candidate rerun.

## Hardware Blocker

phase10-pi5-generated-root-boot-transport-proof-20260605 completed the first
serialized Pi 5 candidate run with a source-backed blocker, not acceptance. The
Pi fetched the selected candidate files, including da591740/kernel_2712.img at
204888 bytes and da591740/initramfs_2712 at 662 bytes, and Talos received FDT
/chosen initramfs bounds:

start=0x000000002efff000 end=0x000000002efff296 len=0x0000000000000296

Talos then reported source=compiled-fallback reason=missing-artifact. The same
serial transcript shows the initramfs range starts at the early page-frame seed
and bootstrap reservation address, and overlaps the initial translation table
layout at 0x2efff000. The next implementation must reserve or copy the firmware
initramfs range before early page-table/bootstrap allocation can overwrite it.
The accepted archive/TFTP placement is therefore not enough by itself to claim
Pi 5 generated-root transport acceptance.

## Deferred

Writable persistence, SD/USB/block drivers, networking, SSH, and Phase 11 remain
deferred. This contract accepts only the Pi 5 boot-transport candidate shape for
the already accepted generated-root artifact format.
