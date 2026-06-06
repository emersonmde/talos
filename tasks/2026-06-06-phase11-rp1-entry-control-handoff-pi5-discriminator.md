# Phase 11 RP1 Entry-Control Handoff Pi 5 Discriminator

Task id: phase11-rp1-entry-control-handoff-pi5-discriminator-20260606

Status: accepted

## Goal

Publish the accepted no-RP1-MMIO handoff-reset candidate on Pi 5 and determine
whether the fetched image reaches the first rust_entry routing point before
BootInfo parsing, target::init, boot reports, memory planning, allocator setup,
or the RP1 UART0 FR read path.

## Scope

- Published only target/talos-rpi5-rp1-handoff-reset-discriminator-core.tar.gz.
- Acquired the hardware lock, retained pre-run boot identity, created a named
  pre-run snapshot, captured fresh serial and TFTP cursors, power-cycled the Pi
  5, retained serial/TFTP/status/boot-file evidence before restore, restored the
  pre-run boot tree, and retained post-restore evidence.
- Classified only candidate fetch and the PSCI reset side effect as evidence of
  pre-BootInfo handoff reachability.
- Kept RP1 MMIO reads, mapped/unmapped behavior, firmware-state behavior, GPIO
  ownership, interrupts, DMA/cache, storage, generated-root, networking, SSH,
  broader PCIe, Milestone 11.2, and phase transition out of scope.

## Findings And Disposition

- fixed: publication staged candidate boot tree
  760e7e3c59c3d6d6da4f465c9f67fc53a445bfa18850c6a76f2a3972af680d2d with
  effective kernel kernel_2712.img and 45,248-byte root and
  da591740/kernel_2712.img files from archive
  ee251a145b88df55fd162b0150a82d62a9671906f401948524d27d45929516c6.
- fixed: the stable same-cursor pre-restore TFTP follow-up retained 26 events
  and four 45,248-byte da591740/kernel_2712.img fetches across two boot
  sequences at 05:51:46/05:51:47 and 05:52:04/05:52:05 UTC.
- fixed: repeated candidate boot/fetch sequences from one power cycle match the
  accepted source side effect: rust_entry routes to PSCI SYSTEM_RESET before
  BootInfo, target::init, or RP1 MMIO.
- fixed: an initial stable helper sample returned zero events because it ran
  before the late TFTP log entries were visible; the retained follow-up
  same-cursor stability sample supersedes it before restore.
- deferred: fresh serial from cursor 4107969 contains Raspberry Pi firmware
  network boot output and candidate config fetch text, but no TALOS:
  kernel_main or source-level marker; serial visibility remains separate from
  the accepted TFTP reset side effect.
- not-an-issue: no RP1 mapped/read-value, unmapped/trap, firmware-state, GPIO
  ownership, interrupt, DMA/cache, storage, generated-root, networking, or SSH
  behavior was accepted.
- not-an-issue: restore returned the lab to pre-run tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective kernel kernel_2712.img.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-entry-control-handoff-pi5-discriminator/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-06-phase11-rp1-entry-control-handoff-pi5-discriminator/classification.json.
- Archive identity: archive-sha256.txt.
- Hardware lock and boot identity: pre-status.json, pre-boot-files.json,
  pre-snapshots.json, pre-run-snapshot-name.txt, pre-run-snapshot.json,
  post-publish-status.json, post-publish-boot-files.json, and
  post-publish-snapshots.json.
- Candidate run: publish.json, serial-peek-before-power.json,
  tftp-cursor-before-power.json, power-cycle.json, serial-observe-1.json,
  serial-observe-2.json, serial-observe-3.json,
  tftp-delta-stable-pre-restore.json,
  tftp-delta-stable-followup-pre-restore.json,
  tftp-delta-final-pre-restore.json, tftp-delta-final2-pre-restore.json,
  final-pre-restore-status.json, final-pre-restore-boot-files.json,
  final2-pre-restore-status.json, and final2-pre-restore-boot-files.json.
- Restore evidence: restore-snapshot.json, post-restore-status.json,
  post-restore-boot-files.json, and post-restore-snapshots.json.

## Validation

- serialized Pi 5 hardware evidence through lab-controller endpoints:
  completed.
- serial hardware boot/output: fresh cursor output reached Raspberry Pi firmware
  network boot and candidate config fetch logs.
- TFTP hardware evidence: stable same-cursor pre-restore follow-up observed four
  45,248-byte da591740/kernel_2712.img candidate fetches across two boot
  sequences after one power cycle.
- restore proof: post-restore tree hash matched pre-run tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- classification: pre-bootinfo-handoff-reachability-accepted.
- git diff --check: passed.
- mdbook build: passed because docs/src files changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as pre-bootinfo-handoff-reachability-accepted.

This run proves that the fetched handoff-reset candidate reached the rust_entry
handoff branch far enough to trigger the PSCI reset side effect before BootInfo,
target::init, or RP1 MMIO. It does not prove any RP1 mapped/read-value,
unmapped/trap, firmware-state, GPIO, interrupt, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, or Milestone 11.2 behavior. The
next bounded task is the already queued handoff closeout.
