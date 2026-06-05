# Phase 11 Known-Good Runtime Readiness Pi 5 Discriminator

Task id: phase11-known-good-runtime-readiness-pi5-discriminator-20260605

Status: completed with blocker evidence

## Goal

Run one serialized Pi 5 known-good runtime readiness discriminator using the
repaired fetch/readiness evidence rule.

## Scope

- Acquired hardwareTestLock for this task only and released it after restoring
  the pre-run snapshot.
- Used the restored known-good boot tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective_kernel=kernel_2712.img.
- Captured pre-run health, status, boot files, snapshots, serial cursor, and
  TFTP cursor before a single power cycle.
- Observed a bounded serial readiness window using
  scripts/rpi5-observe-runtime-readiness.sh with required marker
  rpi5-production-timer-preemption: PASS.
- Restored the pre-run snapshot
  phase11-known-good-runtime-readiness-pre-20260605T2215Z before completion.

## Non-Goals Honored

No boot archive publication, RP1 candidate/source change, candidate rerun,
GPIO ownership, interrupts, DMA/cache, storage, generated-root, networking,
SSH, broader PCIe, Milestone 11.2 work, or phase transition was performed. No
RP1 candidate fetch, Rust entry, entry-control reachability, RP1
mapped/read-value, RP1 unmapped/trap, or firmware-state behavior is accepted.

## Findings And Disposition

- fixed: hardware lock acquisition, pre-run snapshot, boot identity, fresh
  serial cursor, power-cycle response, bounded serial output, final restore,
  and post-restore status evidence were retained.
- fixed: pre-run, pre-restore, and post-restore status all reported the same
  known-good tree hash
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 and
  effective_kernel=kernel_2712.img.
- fixed: stable replay from the retained fresh TFTP cursor 4095602 returned 13
  events on both checks, including two served da591740/kernel_2712.img fetches
  of 104,136 bytes.
- deferred: the serial readiness observation reached Raspberry Pi firmware/RP1
  boot output only. It did not contain TALOS: kernel_main, talos>, or
  rpi5-production-timer-preemption: PASS.
- deferred: this task is completed with blocker evidence rather than accepted
  runtime readiness because the initial pre-restore cursor-delta files were
  collected with a blank cursor due to using the wrong top-level TFTP cursor
  field. The final pre-restore TFTP tail is retained, and the fresh cursor was
  recovered from tftp-tail-before.json and replayed after restore.
- removed: no same-shaped hardware rerun, alternate capture path, open-ended
  wait stack, candidate publication, or source change was added to mask the
  evidence-capture mistake.
- not-an-issue: the helper's non-zero exit is expected for this classification;
  it means the accepted known-good Talos readiness markers were absent.

## Evidence

- Summary:
  tasks/evidence/2026-06-05-phase11-known-good-runtime-readiness-pi5-discriminator/observed-summary.json.
- Classification:
  tasks/evidence/2026-06-05-phase11-known-good-runtime-readiness-pi5-discriminator/classification.json.
- Pre-run identity: health-before.json, lab-status-before.json,
  boot-files-before.json, boot-snapshots-before.json, and
  pre-run-snapshot.json.
- Cursors and power cycle: serial-peek-before.json,
  known-good-serial-cursor.txt, tftp-tail-before.json, tftp-cursor-before.txt,
  and known-good-power-cycle.json.
- Serial hardware output:
  known-good-runtime-readiness-observe.json.
- TFTP evidence: known-good-tftp-delta-replay-after-restore.json and
  known-good-tftp-delta-replay-after-restore-rerun.json.
- Pre-restore and restore: lab-status-pre-restore.json,
  boot-files-pre-restore.json, tftp-tail-pre-restore.json,
  final-restore.json, lab-status-after-restore.json,
  boot-files-after-restore.json, and boot-snapshots-after-restore.json.
- Static evidence inspection:
  tasks/evidence/2026-06-05-phase11-known-good-runtime-readiness-pi5-discriminator/static-evidence-inspection.md.

## Validation

- serialized Pi 5 hardware evidence through lab-controller endpoints:
  completed.
- lab-controller API: pre-run, pre-restore, and post-restore status all showed
  the same restored known-good boot tree and effective kernel.
- serial hardware boot/output: firmware/RP1 boot output appeared from the
  fresh cursor, but Talos readiness did not.
- TFTP hardware evidence: stable cursor replay showed two known-good
  kernel_2712.img fetches, but this was replayed after restore because the
  first pre-restore delta request used a blank cursor.
- restore evidence: the pre-run snapshot restored successfully.
- static evidence inspection: passed.
- git diff --check: passed.
- git diff --cached --check before commit: passed.

## Result

Completed with classification known-good-fetch-without-readiness and blocker
boot-runtime-readiness-after-known-good-fetch.

This does not accept valid known-good Talos runtime readiness because the
serial readiness markers were absent. It also does not accept RP1
candidate/source behavior. The next closeout must decide whether the stable
post-restore replay of the retained fresh TFTP cursor plus final pre-restore
tail is sufficient to close this discriminator, or whether the evidence-capture
mistake requires supervisor-planned repair before any RP1 candidate rerun.
