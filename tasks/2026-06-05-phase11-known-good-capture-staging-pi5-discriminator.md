# Phase 11 Known-Good Capture/Staging Pi 5 Discriminator

Task id: phase11-known-good-capture-staging-pi5-discriminator-20260605

Status: accepted

## Goal

Run one serialized Pi 5 discriminator on the restored known-good boot tree and
classify capture/staging separately from Talos runtime readiness before any RP1
candidate rerun.

## Scope

- Acquired hardwareTestLock for this task only.
- Used the restored known-good boot tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
  with effective_kernel=kernel_2712.img.
- Captured pre-run health, status, boot files, snapshots, serial cursor, and
  TFTP cursor before a single power cycle.
- Queried stable TFTP evidence and retained final pre-restore status, boot
  files, and TFTP tail before restoring the pre-run snapshot.
- Restored the pre-run boot snapshot before completion.

## Non-Goals Honored

No boot archive publication, RP1 candidate rerun, kernel/RP1 runtime change,
GPIO ownership, interrupts, DMA/cache work, storage, generated-root work,
networking, SSH, broader PCIe work, Milestone 11.2 work, or phase transition
was performed.

## Findings And Disposition

- fixed: hardware lock, pre-run snapshot, status, boot files, snapshots, fresh
  serial cursor, fresh TFTP cursor, power-cycle response, serial output, stable
  TFTP evidence, restore response, and post-restore status were retained.
- fixed: final pre-restore TFTP evidence from cursor 4094251 is stable and
  includes 13 events, including two served da591740/kernel_2712.img fetches of
  104,136 bytes.
- fixed: pre-run, pre-restore, and post-restore status all report restored tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective_kernel=kernel_2712.img.
- deferred: the first stable TFTP query from the same fresh cursor observed
  zero events before the final pre-restore query observed the served files; the
  closeout must reconcile this capture-latency evidence before authorizing RP1
  candidate reuse.
- deferred: serial output reached Raspberry Pi firmware/RP1 boot output but did
  not reach TALOS: kernel_main, command-loop readiness, or PASS.
- not-an-issue: the pre-run snapshot restored successfully and post-restore
  boot identity matches the known-good tree.
- removed: no workaround capture path, candidate publication, source change, or
  extra hardware rerun was added.

## Evidence

- Summary:
  tasks/evidence/2026-06-05-phase11-known-good-capture-staging-pi5-discriminator/observed-summary.json.
- Classification:
  tasks/evidence/2026-06-05-phase11-known-good-capture-staging-pi5-discriminator/classification.json.
- Pre-run identity: health-before.json, lab-status-before.json,
  boot-files-before.json, boot-snapshots-before.json, and pre-run-snapshot.json.
- Cursors and power cycle: serial-peek-before.json,
  known-good-serial-cursor.txt, tftp-tail-before.json,
  tftp-cursor-before.txt, and known-good-power-cycle.json.
- Serial hardware output: known-good-serial-observe.json.
- TFTP hardware evidence: known-good-tftp-delta-stable-pre-restore.json and
  known-good-tftp-delta-stable-pre-restore-rerun.json.
- Pre-restore and restore: lab-status-pre-restore.json,
  boot-files-pre-restore.json, tftp-tail-pre-restore.json,
  final-restore.json, lab-status-after-restore.json,
  boot-files-after-restore.json, and boot-snapshots-after-restore.json.
- Static inspection:
  tasks/evidence/2026-06-05-phase11-known-good-capture-staging-pi5-discriminator/static-evidence-inspection.md.

## Validation

- serialized Pi 5 hardware evidence through lab-controller endpoints: passed.
- lab-controller API: pre-run, pre-restore, and post-restore status all showed
  the same restored known-good boot tree and effective kernel.
- TFTP hardware evidence: final stable pre-restore evidence showed known-good
  config/kernel/DTB/overlay fetches, including two served
  da591740/kernel_2712.img entries.
- serial hardware boot/output: firmware/RP1 boot output appeared from the fresh
  cursor, but Talos readiness did not.
- restore evidence: the pre-run snapshot restored successfully.
- static evidence inspection: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed with existing warnings only.
- git diff --cached --check before commit: passed.

## Result

Accepted discriminator with classification
known-good-fetch-observed-without-talos-readiness.

This accepts only known-good capture/staging evidence through the repaired lab
path. It does not accept RP1 candidate fetch, Rust entry, entry-control
reachability, RP1 mapped/read-value, RP1 unmapped/trap, GPIO, interrupts,
DMA/cache, storage, generated-root work, networking, SSH, broader PCIe, or
Milestone 11.2 behavior. The next queued closeout must decide whether the
capture-latency and no-Talos-readiness evidence blocks or permits a bounded RP1
candidate proof.
