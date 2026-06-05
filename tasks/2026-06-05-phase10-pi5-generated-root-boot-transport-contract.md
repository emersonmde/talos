# Phase 10 Pi 5 Generated-Root Boot Transport Contract

Task: phase10-pi5-generated-root-boot-transport-contract-20260605

Status: accepted

## Scope

- Inspect accepted local/QEMU generated-root no-rebuild transport evidence, Pi 5
  lab docs, boot archive scripts, firmware config options, linker/artifact
  windows, and roadmap Milestone 10.3 notes.
- Select one concrete Pi 5 candidate transport path or record a blocker.
- Record artifact placement/address, archive contents, kernel/artifact identity
  requirements, fallback behavior, lab evidence requirements, and restore
  policy.

## Non-Goals

- No runtime code changes.
- No boot archive publication, Pi 5 power-cycle, or hardwareTestLock
  acquisition.
- No writable filesystem, SD/USB/block driver, networking, SSH, or phase
  transition.

## Selected Transport

Selected path: Raspberry Pi firmware-loaded initramfs envelope carrying the
existing talos-generated-root-v1 artifact bytes.

The future candidate archive must add initramfs_2712 to the existing Pi 5 local
serial command-loop boot tree and add this line to both root and serial-prefixed
config.txt files:

initramfs initramfs_2712 followkernel

The archive must include identical root and serial-prefix copies:

initramfs_2712
da591740/initramfs_2712

This mirrors the existing root and da591740/ kernel/config/DTB pattern because
the lab has observed both root and serial-prefixed firmware fetch paths over
time. The hardware proof must record which artifact path was actually served
after a fresh TFTP cursor.

## Candidate Identity Contract

The next candidate task must use a stable non-published archive name:

target/talos-rpi5-generated-root-boot-transport-candidate-20260605.tar.gz

The candidate boot tree must be:

target/rpi5-generated-root-boot-transport-candidate-20260605-boot-tree

The candidate must record:

- source commit and git status --short;
- archive SHA-256;
- kernel_2712.img SHA-256 and size;
- kernel8.img equality with kernel_2712.img;
- generated-root artifact SHA-256 and size;
- root and da591740/ artifact copy equality;
- config line initramfs initramfs_2712 followkernel in both config files;
- boot tree listing;
- proof strings for rootinfo, /generated/manifest.txt, /generated/status7, and
  the expected classification/PASS line.

The expected runtime source for Pi 5 is the firmware-provided FDT /chosen
initramfs range, conventionally exposed as linux,initrd-start and
linux,initrd-end. The candidate implementation may not reuse QEMU's fixed
0x47000000 loader address as a Pi 5 hardware claim. It must report the observed
initramfs start/end range, length, digest, and source in serial output before
shell-visible generated-root reads or execs claim success.

## Hardware Proof Contract

The later serialized proof task must acquire hardwareTestLock before archive
publication or power control and must retain:

- lab API GET / status with configured/effective kernel and boot tree hash
  before publication;
- named boot snapshot or restore handle for the prior accepted boot tree;
- fresh serial cursor before candidate power-cycle;
- fresh TFTP cursor before candidate power-cycle;
- TFTP delta after the run showing the expected candidate kernel fetch and the
  generated-root artifact fetch (initramfs_2712 or da591740/initramfs_2712);
- serial transcript showing generated-root source/digest/range, generated file
  content, generated executable status, shell readiness, and PASS/classification
  markers;
- post-run restore through the lab snapshot/restore endpoints and post-restore
  boot identity.

Acceptance requires proof that Talos consumed the external firmware-loaded
artifact on Pi 5. A TFTP fetch alone, a kernel-only boot, or old serial
scrollback is not enough.

## Fallback And Boundaries

The Pi 5 runtime must preserve the accepted local fallback policy:

- missing FDT initramfs bounds -> compiled generated-root fallback;
- invalid bounds, oversize range, bad magic/version/length/digest/order/path, or
  executable-size violation -> compiled generated-root fallback;
- no partial merge from malformed artifacts into the VFS.

This contract accepts only the candidate transport shape. It does not accept
writable persistence, SD/USB/block storage, networking, SSH, or a phase
transition. QEMU/local no-rebuild evidence remains accepted separately at the
loader-device 0x47000000 boundary.

## Inconclusive-Run Triage

Before changing code after any inconclusive Pi 5 hardware run, the proof task
must record this sequence:

1. Candidate identity: commit, git status, archive SHA-256, kernel SHA-256,
   kernel size, artifact SHA-256, artifact size, and archive-review output.
2. Fresh serial cursor: prove the observe window starts after candidate
   publish/power-cycle.
3. TFTP delta: prove the Pi fetched the candidate kernel and generated-root
   artifact after candidate publish.
4. Known-good control: restore and run an accepted known-good control unless
   the failure is already a clear Talos proof failure with complete candidate
   fetch and serial evidence.
5. Candidate rerun: rerun the same candidate with fresh serial and TFTP cursors
   before code changes.

## Findings

- fixed: Selected the Pi 5 firmware initramfs initramfs_2712 followkernel path
  as the concrete generated-root boot transport candidate.
- fixed: Required root and serial-prefix artifact copies so the archive is
  compatible with both observed lab firmware fetch patterns.
- fixed: Required runtime source evidence from FDT /chosen initramfs start/end
  bounds instead of QEMU's loader-device address.
- fixed: Recorded exact candidate archive/config/artifact identity and
  hardware-proof evidence gates.
- removed: Treating QEMU's 0x47000000 address as a Pi 5 placement assumption.
- deferred: Writable persistence, SD/USB/block drivers, networking, SSH, and
  phase transition.
- not-an-issue: No hardware lock, archive publication, power-cycle, QEMU rerun,
  or runtime behavior change was required for this contract-only task.

## Result

The Pi 5 generated-root boot-transport contract is accepted. The next queued
candidate archive core is mechanically unblocked only for a non-published
candidate archive/static review that follows this contract. The serialized Pi 5
proof remains separate and must own hardware publication, power-cycle, evidence,
and restore.

## Validation

- static inspection:
  tasks/evidence/2026-06-05-phase10-pi5-generated-root-boot-transport-contract/static-inspection.md
- diff hygiene: git diff --check passed.
- documentation: /home/node/.cargo/bin/mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before commit.
