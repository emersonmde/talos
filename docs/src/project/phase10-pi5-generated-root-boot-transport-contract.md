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

## Deferred

Writable persistence, SD/USB/block drivers, networking, SSH, and Phase 11 remain
deferred. This contract accepts only the Pi 5 boot-transport candidate shape for
the already accepted generated-root artifact format.
