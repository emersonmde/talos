# Pi 5 Generated-Root Boot Transport Closeout Evidence Inspection

Task: phase10-pi5-generated-root-boot-transport-closeout-20260605

Inspection level: static evidence inspection. No hardware work, archive
publication, QEMU rerun, or runtime behavior change was performed.

## Inputs

- Contract task:
  tasks/2026-06-05-phase10-pi5-generated-root-boot-transport-contract.md
- Contract evidence:
  tasks/evidence/2026-06-05-phase10-pi5-generated-root-boot-transport-contract/static-inspection.md
- Contract commit:
  7f915dd4f5d168f0fbe1ca93b0821187d0c9b719
- Candidate task:
  tasks/2026-06-05-phase10-pi5-generated-root-boot-archive-candidate-core.md
- Candidate evidence directory:
  tasks/evidence/2026-06-05-phase10-pi5-generated-root-boot-archive-candidate-core/
- Candidate commit:
  3616d310dd224bd8c4c6c34b161be053205bd793
- Proof/blocker task:
  tasks/2026-06-05-phase10-pi5-generated-root-boot-transport-proof.md
- Proof/blocker evidence directory:
  tasks/evidence/2026-06-05-phase10-pi5-generated-root-boot-transport-proof/
- Proof/blocker commit:
  63d212d047e3e6a6647b0a0f1b7149d8518f0c7a

## Evidence Map

- Contract selected the firmware-loaded initramfs_2712 envelope, root and
  da591740/ artifact copies, and FDT /chosen linux,initrd-start/end as the Pi 5
  runtime source. It did not accept hardware consumption.
- Candidate archive/static review accepted a non-published candidate archive at
  target/talos-rpi5-generated-root-boot-transport-candidate-20260605.tar.gz with
  archive SHA-256
  8cb1d731e55f35d13328cf4f618c9dac2bf673311535ddd36038680d8a4ef60e, kernel
  SHA-256 c44e5a55eb600a09a217c6ad23f665a43d1092a8e982423f5162099c34a42169,
  and generated-root artifact SHA-256
  0341f5393502f54489acb1951633bf2773fb846a82bde89b3e4a2e82000724c6.
- Serialized Pi 5 proof published only that retained candidate, captured fresh
  serial and TFTP cursors, observed da591740/kernel_2712.img at 204888 bytes and
  da591740/initramfs_2712 at 662 bytes, reached the command-loop proof harness,
  and restored the prior boot tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- The same proof blocked acceptance because firmware initramfs bounds
  0x2efff000..0x2efff296 overlapped the early page-frame seed/bootstrap
  reservation/translation-table range, so Talos later reported
  source=compiled-fallback reason=missing-artifact instead of consuming the
  external generated-root artifact.

## Boundary

Accepted:

- Local/QEMU generated-root no-kernel-rebuild transport remains accepted at the
  loader-device 0x47000000 boundary from
  phase10-generated-root-no-rebuild-transport-closeout-20260605.
- The Pi 5 candidate archive shape and static placement are accepted as a
  non-published archive/candidate boundary.
- Candidate publication, TFTP fetch, fresh serial/TFTP capture, and restore
  procedure were demonstrated as retained hardware evidence for the blocked
  proof.

Not accepted:

- Pi 5 consumption of the firmware-loaded external generated-root artifact.
- Writable persistence, SD/USB/block storage, networking, SSH, or any phase
  transition.
- Milestone 10.3 hardware transport closeout as accepted; the milestone remains
  open for an explicit remediation task and fresh serialized Pi 5 proof.

## Findings

- fixed: Reconciled contract, candidate, hardware proof/blocker, restore, and
  retained local/QEMU control evidence into one closeout boundary.
- fixed: Recorded that the hardware blocker is not archive publication, TFTP
  placement, serial capture, or restore; it is source-backed memory-range
  overlap before generated-root installation.
- deferred: A planned implementation must reserve or copy the firmware
  initramfs range before early page-table/bootstrap allocation can overwrite it,
  then run a fresh serialized Pi 5 proof.
- not-an-issue: No new hardware run was required for this closeout because the
  proof task retained decisive blocker evidence and released/restored the
  hardware lock.
