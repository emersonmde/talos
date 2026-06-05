# Phase 10 Pi 5 Generated-Root Boot Transport Proof

Task: phase10-pi5-generated-root-boot-transport-proof-20260605

Status: completed with a source-backed blocker, not accepted.

## Goal

Run the serialized Pi 5 proof for the accepted generated-root boot-transport
candidate and retain decisive hardware evidence or a source-backed blocker.

## Outcome

The task published only the accepted candidate archive, power-cycled the Pi 5,
captured fresh serial/TFTP evidence, and restored the prior boot tree. The run
does not satisfy acceptance because Talos did not consume the external
generated-root artifact as source firmware-initramfs.

The retained hardware evidence is decisive enough to block this candidate
without claiming acceptance: firmware fetched the candidate initramfs_2712
artifact and supplied /chosen initramfs bounds, but the reported range
overlapped Talos' early bootstrap page-table reservation. By the time the
generated-root installer parsed the firmware range, it fell back to the
compiled generated root with reason=missing-artifact.

## Evidence

Evidence directory:

tasks/evidence/2026-06-05-phase10-pi5-generated-root-boot-transport-proof/

Primary run:

tasks/evidence/2026-06-05-phase10-pi5-generated-root-boot-transport-proof/local2-candidate/

Key files:

- candidate-identity.txt
- status-before.json
- snapshot-create.json
- serial-peek-before.json
- tftp-cursor-before.txt
- publish-candidate.json
- status-after-publish.json
- power-cycle-candidate.json
- tftp-delta-candidate-before-restore.json
- serial-transcript.txt
- proof-summary.txt
- restore.json
- status-after-restore.json
- blocker-analysis.txt

Candidate identity:

- archive SHA-256:
  8cb1d731e55f35d13328cf4f618c9dac2bf673311535ddd36038680d8a4ef60e
- kernel SHA-256:
  c44e5a55eb600a09a217c6ad23f665a43d1092a8e982423f5162099c34a42169
- generated-root artifact SHA-256:
  0341f5393502f54489acb1951633bf2773fb846a82bde89b3e4a2e82000724c6

TFTP evidence shows the Pi 5 fetched:

- da591740/kernel_2712.img, 204888 bytes
- da591740/initramfs_2712, 662 bytes
- da591740/bcm2712-rpi-5-b.dtb, 78679 bytes
- expected config/cmdline/overlay files from the da591740/ mirror

Serial evidence records:

- fresh serial cursor: 4066159
- firmware initramfs range:
  start=0x000000002efff000 end=0x000000002efff296 len=0x0000000000000296
- fallback:
  source=compiled-fallback reason=missing-artifact
- command-loop readiness:
  rpi5-generated-root-boot-transport-proof: ready command=0
- generated manifest fallback content:
  Talos generated-root manifest fixture

Restore evidence:

- pre-run boot tree hash:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
- candidate boot tree hash:
  5c24d009220334ebaff9dacc037ef2833c1ea21ecee1296818953449c602c7e6
- restored boot tree hash:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10

## Findings

- fixed: Acquired the hardware lock in supervisor state, published only the
  accepted candidate archive, captured fresh serial and TFTP cursors, captured
  candidate TFTP fetch evidence before restore, restored the prior boot tree,
  and released the task with a source-backed blocker.
- fixed: Identified that the firmware initramfs range begins at 0x2efff000,
  the same address used by the early page-frame seed, bootstrap reserve, and
  translation table layout.
- deferred: Remediation should reserve/copy the firmware initramfs range before
  early page-table/bootstrap allocation or move generated-root installation
  earlier. That is outside this serialized proof task and needs a separately
  planned implementation task.
- not-an-issue: Candidate archive publication and TFTP placement worked; the
  Pi fetched the selected kernel and initramfs_2712 artifact.

## Validation

- lab-controller status: /status before publish, after publish, before restore,
  and after restore retained. GET / returned 404 in this deployed lab API, so
  /status is the recorded equivalent for configured/effective kernel and boot
  tree identity.
- serialized Pi 5 hardware boot/output: captured in local2-candidate.
- lab-controller TFTP evidence: captured in
  tftp-delta-candidate-before-restore.json.
- restore proof: passed; restored tree hash matches the pre-run tree hash.
- git diff hygiene: git diff --check passed before docs edits; final
  validation is recorded in the commit state.

## Next Action

The queued closeout task may reconcile this blocked proof record. Do not claim
Milestone 10.3 Pi 5 generated-root boot transport acceptance until a later
implementation reserves or copies the firmware initramfs range before it can
overlap early memory setup and then passes a fresh serialized Pi 5 proof.
