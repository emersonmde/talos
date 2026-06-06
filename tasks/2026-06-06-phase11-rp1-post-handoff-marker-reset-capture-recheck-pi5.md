# Phase 11 RP1 Post-Handoff Marker Reset Capture Recheck Pi 5

Task id: phase11-rp1-post-handoff-marker-reset-capture-recheck-pi5-20260606

Status: completed

## Goal

Rerun the accepted no-RP1-MMIO post-handoff marker/reset candidate on the Pi 5
under the repaired capture invariant.

## Scope

- Published only target/talos-rpi5-post-handoff-marker-reset-core.tar.gz
  (SHA-256 73a74db1d08d89a3aa371d5329bc6158553bef172a82f0b479598bc29f15acaa).
- Captured the selected boot tree identity, effective kernel, boot file sizes,
  fresh serial cursor, fresh TFTP cursor, bounded serial observation, stable
  same-cursor TFTP evidence before restore, final pre-restore status/files,
  restore proof, and post-restore status/files through the capture-invariant
  proof bundle.
- Kept RP1 UART0 FR reads, RP1 mapped/unmapped classification, GPIO,
  interrupts, DMA/cache, storage, generated-root, networking, SSH, broader
  PCIe, Milestone 11.2, phase transition, and kernel source changes out of
  scope.

## Findings And Disposition

- fixed: candidate publication selected tree
  37995c483190ddcfaef70c9cf5be04244f75c4fcd9cf25fdd90f941ccc48c4f2 with
  effective kernel kernel_2712.img and 51,736-byte
  da591740/kernel_2712.img before power cycle.
- fixed: stable same-cursor pre-restore TFTP evidence from cursor 4111814
  reached cursor_end 4118569, stabilized for three samples, and retained 65
  events including 10 served da591740/kernel_2712.img fetches at 51,736 bytes.
- fixed: fresh serial evidence from cursor 4113931 retained 19,625 bytes over
  the 90-second bounded window. The window shows repeated Raspberry Pi firmware
  NETWORK boot/fetch cycles, but no TALOS: kernel_main and no
  rpi5-rp1-post-handoff-marker-reset text.
- fixed: the proof restored snapshot
  phase11-post-handoff-marker-reset-capture-recheck-pre-20260606T0852Z and
  post-restore status returned tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: scripts/rpi5-capture-invariant-proof-bundle.sh had a jq object-value
  syntax error in the final suggested_classification expression. The hardware
  capture and restore had already completed, and the script fix plus direct
  regeneration produced capture-invariant-summary.json from the retained
  candidate-run files.
- not-an-issue: known-good control and candidate rerun were not required,
  because the first repaired capture-invariant run was not a capture/staging
  inconclusive result; candidate fetches and reset-loop side effects were
  observed before restore.
- not-an-issue: no RP1 mapped/read-value, unmapped/trap, firmware-state, GPIO,
  interrupt, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
  or Milestone 11.2 behavior was accepted.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-capture-recheck-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-capture-recheck-pi5/classification.json.
- Candidate identity: archive-sha256.txt, archive-listing.txt,
  candidate-post-publish-status.json, candidate-post-publish-boot-files.json,
  candidate-post-publish-tree-hash.txt, and candidate-kernel-bytes.txt.
- Candidate run bundle: candidate-run/pre-status.json,
  candidate-run/pre-boot-files.json, candidate-run/pre-snapshots.json,
  candidate-run/preflight-identity.json,
  candidate-run/serial-peek-before-power.json,
  candidate-run/serial-cursor-before-power.txt,
  candidate-run/tftp-cursor-before-power.json,
  candidate-run/tftp-cursor-before-power.txt, candidate-run/power-cycle.json,
  candidate-run/serial-observe-window.json,
  candidate-run/tftp-delta-stable-pre-restore.json,
  candidate-run/final-pre-restore-status.json,
  candidate-run/final-pre-restore-boot-files.json,
  candidate-run/restore-snapshot.json, candidate-run/post-restore-status.json,
  candidate-run/post-restore-boot-files.json, and
  candidate-run/capture-invariant-summary.json.
- Validation summary:
  tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-capture-recheck-pi5/validation-summary.txt.

## Validation

- serialized Pi 5 hardware proof through lab-controller endpoints: completed.
- candidate identity check via GET /status and GET /boot/files before power
  cycle: passed.
- serial hardware boot/output: fresh 90-second window retained firmware NETWORK
  boot/fetch cycles and did not contain TALOS: kernel_main or the
  post-handoff marker.
- TFTP hardware evidence: stable same-cursor pre-restore delta observed
  candidate kernel fetches before restore.
- restore proof and hardware lock release evidence: restore proof retained;
  supervisor hardwareTestLock release is recorded in durable state.
- bash -n scripts/rpi5-capture-invariant-proof-bundle.sh: passed.
- scripts/rpi5-capture-invariant-proof-bundle.sh --dry-run: passed.
- git diff --check: passed.
- mdbook build: not run because no docs/src files were touched.
- git diff --cached --check before commit: passed.

## Result

Completed as reset-side-effect-without-visible-marker.

The repaired capture invariant proves the candidate was selected and fetched
before restore, and the serial window shows repeated firmware reboot/fetch
cycles without the post-handoff marker. This accepts reset side-effect evidence
only; it does not accept visible post-handoff serial observability, RP1
mapped/unmapped behavior, RP1 UART0 FR-read readiness, GPIO, interrupts,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
11.2, or phase transition.
