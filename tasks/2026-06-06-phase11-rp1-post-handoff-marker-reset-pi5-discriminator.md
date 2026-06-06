# Phase 11 RP1 Post-Handoff Marker Reset Pi 5 Discriminator

Task id: phase11-rp1-post-handoff-marker-reset-pi5-discriminator-20260606

Status: completed

## Goal

Publish the accepted post-handoff marker/reset candidate on Pi 5 and classify
whether hardware evidence shows marker visibility, reset side effects without a
visible marker, marker-path failure before reset, or a staging/capture blocker.

## Scope

- Published only target/talos-rpi5-post-handoff-marker-reset-core.tar.gz
  (SHA-256 73a74db1d08d89a3aa371d5329bc6158553bef172a82f0b479598bc29f15acaa).
- Captured pre-run boot identity, snapshots, fresh serial and TFTP cursors,
  candidate publication state, power-cycle evidence, serial observations,
  stable same-cursor TFTP samples, rerun evidence, restored known-good control
  evidence, and restore proofs.
- Kept RP1 UART0 FR reads, mapped/unmapped behavior, GPIO, interrupts,
  DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
  11.2, and phase transition out of scope.

## Findings And Disposition

- fixed: candidate publication changed the boot tree from
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 to
  37995c483190ddcfaef70c9cf5be04244f75c4fcd9cf25fdd90f941ccc48c4f2 with
  51,736-byte da591740/kernel_2712.img and kernel_2712.img files.
- fixed: first candidate power cycle retained fresh serial from cursor 4110717
  and fresh TFTP cursor 4106410. Serial reached Raspberry Pi firmware/RP1 boot
  output, but no TALOS or post-handoff marker text appeared.
- deferred: the first stable same-cursor TFTP sample before restore reported
  zero events; a late same-cursor query later showed 26 events and four
  da591740/kernel_2712.img lines, but by then status showed the restored tree,
  so the endpoint's current-file byte annotation could not be used as candidate
  file-size proof.
- fixed: candidate rerun retained fresh TFTP cursor 4109112, fresh firmware
  serial, stable zero TFTP events, final zero TFTP events, and post-restore
  proof.
- fixed: restored known-good control retained fresh TFTP cursor 4110463, fresh
  firmware serial, stable zero TFTP events, final zero TFTP events, and
  post-restore proof.
- fixed: every run restored the boot tree to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 before
  completion.
- not-an-issue: no RP1 mapped/read-value, unmapped/trap, firmware-state, GPIO,
  interrupt, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
  or Milestone 11.2 behavior was accepted.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-pi5-discriminator/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-pi5-discriminator/classification.json.
- Archive identity: archive-sha256.txt.
- Candidate run: pre-status.json, post-publish-status.json,
  post-publish-boot-files.json, serial-peek-before-power.json,
  tftp-cursor-before-power.json, power-cycle.json, serial-observe-1.json,
  serial-observe-2.json, serial-observe-3.json,
  tftp-delta-stable-pre-restore.json, tftp-delta-late-before-restore.json,
  final-pre-restore-status.json, restore-snapshot.json, and
  post-restore-status.json.
- Candidate rerun: rerun-pre-status.json, rerun-post-publish-status.json,
  rerun-serial-observe-90s.json, rerun-tftp-delta-stable-pre-restore.json,
  rerun-tftp-delta-final-pre-restore.json, rerun-final-pre-restore-status.json,
  rerun-restore-snapshot.json, and rerun-post-restore-status.json.
- Restored known-good control: control-pre-status.json,
  control-serial-observe-90s.json,
  control-tftp-delta-stable-pre-restore.json,
  control-tftp-delta-final-pre-restore.json, control-final-pre-restore-status.json,
  control-restore-snapshot.json, and control-post-restore-status.json.

## Validation

- serialized Pi 5 hardware evidence through lab-controller endpoints:
  completed.
- serial hardware boot/output: candidate, rerun, and restored control each
  produced fresh Raspberry Pi firmware/RP1 boot output, but no Talos marker or
  runtime prompt.
- TFTP hardware evidence: candidate stable same-cursor sample, candidate rerun,
  and restored control all recorded stable zero TFTP deltas in their bounded
  windows; late first-run TFTP lines are retained as capture-timing evidence,
  not candidate identity proof.
- restore proof: every post-restore status returned tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- classification: staging-capture-blocker.
- git diff --check: passed.
- mdbook build: not run because no docs/src files were touched.
- git diff --cached --check before commit: pending at commit time.

## Result

Completed as staging-capture-blocker.

This run does not accept visible post-handoff serial observability, reset
side-effect evidence, marker-path failure, or RP1 UART0 FR-read readiness. The
next bounded task is the already queued marker/reset closeout, which must close
out this blocker without inferring RP1 mapped/unmapped behavior or promoting
the RP1 UART0 FR-read path.
