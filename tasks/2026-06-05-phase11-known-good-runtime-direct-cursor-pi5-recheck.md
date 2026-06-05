# Phase 11 Known-Good Runtime Direct-Cursor Pi 5 Recheck

Task id: phase11-known-good-runtime-direct-cursor-pi5-recheck-20260605

Status: completed with blocker evidence

## Goal

Run one serialized Pi 5 known-good runtime-readiness recheck using direct fresh-cursor TFTP evidence before restore.

## Scope

- Acquired hardwareTestLock for this task only and restored the pre-run boot snapshot before completion.
- Used the restored known-good boot tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with effective_kernel=kernel_2712.img.
- Captured pre-run health, status, boot files, snapshots, serial cursor, and authoritative TFTP cursor before one power cycle.
- Retained direct stable pre-restore TFTP evidence from the fresh cursor, final pre-restore status/boot-files/TFTP tail, restore evidence, and post-restore status.
- Classified runtime readiness only against the accepted markers: TALOS: kernel_main plus rpi5-production-timer-preemption: PASS.

## Non-Goals Honored

No RP1 candidate archive publication, RP1 source/runtime change, new RP1 constants, RP1 MMIO read, GPIO ownership, interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone 11.2 work, or phase transition was performed. This task does not accept RP1 candidate fetch, Rust entry, entry-control reachability, RP1 mapped/read-value, RP1 unmapped/trap, or firmware-state behavior.

## Findings And Disposition

- fixed: hardware lock acquisition, pre-run snapshot, boot identity, fresh serial cursor, fresh authoritative TFTP cursor, power-cycle response, serial output, direct stable pre-restore TFTP delta, final pre-restore state, restore response, and post-restore state were retained.
- fixed: pre-run, pre-restore, and post-restore status all reported tree hash a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 and effective_kernel=kernel_2712.img.
- fixed: direct stable pre-restore TFTP evidence from cursor 4096953 retained 13 events, including two served 104,136-byte da591740/kernel_2712.img fetches.
- deferred: the 75-second serial readiness observation from fresh cursor 4096040 did not contain TALOS: kernel_main, talos>, or rpi5-production-timer-preemption: PASS.
- deferred: classification is boot-runtime-readiness-after-known-good-fetch; valid-known-good-talos-readiness remains unaccepted and RP1 candidate/source work remains blocked pending closeout.
- fixed: the first combined capture command stopped after serial observation before TFTP/restore files were written. The recovery path used the retained fresh TFTP cursor, captured direct stable pre-restore TFTP evidence, recorded pre-restore status/files/tail, restored the pre-run snapshot, and retained post-restore evidence before classification.
- removed: no alternate capture path, extra wait stack, source change, RP1 candidate rerun, boot publication, or phase transition was added.
- not-an-issue: the readiness helper's nonzero exit is expected for this classification because the accepted known-good Talos readiness markers were absent.

## Evidence

- Summary: tasks/evidence/2026-06-05-phase11-known-good-runtime-direct-cursor-pi5-recheck/observed-summary.json.
- Classification: tasks/evidence/2026-06-05-phase11-known-good-runtime-direct-cursor-pi5-recheck/classification.json.
- Pre-run identity: health-before.json, lab-status-before.json, boot-files-before.json, boot-snapshots-before.json, and pre-run-snapshot.json.
- Cursors and power cycle: serial-peek-before.json, known-good-serial-cursor.txt, tftp-tail-before.json, tftp-cursor-before.txt, and known-good-power-cycle.json.
- Serial hardware output: known-good-runtime-readiness-observe.json.
- Direct stable TFTP evidence: known-good-tftp-delta-stable-pre-restore.json and known-good-tftp-delta-stable-pre-restore-rerun.json.
- Pre-restore and restore: lab-status-pre-restore.json, boot-files-pre-restore.json, tftp-tail-pre-restore.json, final-restore.json, lab-status-after-restore.json, boot-files-after-restore.json, and boot-snapshots-after-restore.json.
- Static inspection: tasks/evidence/2026-06-05-phase11-known-good-runtime-direct-cursor-pi5-recheck/static-evidence-inspection.md.

## Validation

- serialized Pi 5 hardware evidence through lab-controller endpoints: completed.
- lab-controller API: pre-run, pre-restore, and post-restore status all showed the same restored known-good boot tree and effective kernel.
- serial hardware boot/output: firmware/RP1 boot output appeared from the fresh cursor, but Talos readiness did not.
- TFTP hardware evidence: direct stable pre-restore evidence showed two 104,136-byte da591740/kernel_2712.img fetches.
- restore evidence: the pre-run snapshot restored successfully.
- static evidence inspection: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: not run; docs/src files were not touched and the accepted readiness boundary did not change.
- git diff --cached --check before commit: passed.

## Result

Completed with classification boot-runtime-readiness-after-known-good-fetch.

This task accepts known-good fetch evidence through the direct-cursor path, but it does not accept valid known-good Talos runtime readiness because the serial readiness markers were absent. The next queued closeout is mechanically unblocked after this completed blocker evidence and must keep RP1 entry-control candidate rerun blocked unless it explicitly accepts valid-known-good-talos-readiness.
