# Phase 11 Known-Good Runtime Serial-Window Pi 5 Discriminator

Task id: phase11-known-good-runtime-serial-window-pi5-discriminator-20260606

Status: completed

## Goal

Run one serialized known-good Pi 5 runtime-readiness discriminator under the repaired serial-window contract.

## Scope

- Acquired the hardware lock for this task only.
- Preserved the restored known-good boot tree identity before the run.
- Captured fresh serial and TFTP cursors, power-cycled the Pi 5, retained serial/TFTP evidence before restore, restored the pre-run boot tree snapshot, and released the proof boundary back to the supervisor state.
- Kept RP1 candidate publication, RP1 source/runtime changes, RP1 MMIO, GPIO ownership, interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone 11.2, and phase transition out of scope.

## Findings And Disposition

- fixed: retained a new pre-run snapshot, pre-run status, pre-run boot-file listing, serial cursor, TFTP cursor, power-cycle response, stable pre-restore TFTP delta, final pre-restore status/boot files, restore response, and post-restore status/boot files under the task evidence directory.
- fixed: confirmed the known-good TFTP path is visible under the stable pre-restore rule. The delta from cursor `4098304` to `4099655` was stable with 13 events and two `da591740/kernel_2712.img` fetches for the 104,136-byte known-good image.
- fixed: retained serial output from the fresh cursor. The captured serial reaches `rpi5-production-timer-preemption: PASS`, but neither the helper summary nor the direct large observe from the same fresh cursor contains `TALOS: kernel_main`.
- not-an-issue: the boot tree restored successfully after the proof; post-restore status still reports effective kernel `kernel_2712.img` and `da591740/kernel_2712.img` at 104,136 bytes.
- deferred: `valid-known-good-talos-readiness` remains unaccepted because the accepted success criteria require both `TALOS: kernel_main` and `rpi5-production-timer-preemption: PASS` in the fresh serial window.
- deferred: the smallest next discriminator is a serial-log completeness/marker-boundary review: explain why the retained fresh serial window includes the later production-timer PASS but omits the earlier `TALOS: kernel_main` marker before any RP1 candidate/source work resumes.

## Evidence

- Evidence map: `tasks/evidence/2026-06-06-phase11-known-good-runtime-serial-window-pi5-discriminator/evidence-map.json`.
- Classification: `tasks/evidence/2026-06-06-phase11-known-good-runtime-serial-window-pi5-discriminator/classification.json`.
- Lock and pre-run identity: `hardware-lock-acquired-state.json`, `hardware-lock-released-state.json`, `pre-run-status.json`, `pre-run-boot-files.json`, `pre-run-snapshots.json`, `pre-run-snapshot-create.json`, and `pre-run-snapshot-name.txt`.
- Hardware run: `serial-cursor.json`, `tftp-cursor-before.json`, `power-cycle.json`, `serial-readiness-observe.json`, `serial-observe-direct-large-after-manual.json`, and `tftp-delta-stable-pre-restore.json`.
- Restore and final state: `final-pre-restore-status.json`, `final-pre-restore-boot-files.json`, `tftp-tail-pre-restore.json`, `restore.json`, `post-restore-status.json`, and `post-restore-boot-files.json`.

## Validation

- serialized Pi 5 hardware evidence through lab-controller endpoints: completed.
- lab-controller API evidence: stable TFTP delta observed two `da591740/kernel_2712.img` fetches; serial hardware boot/output reached the production-timer PASS marker but not the required `TALOS: kernel_main` marker in retained output.
- mandatory inconclusive-run triage before code changes: no code changes were made after the inconclusive marker mismatch; the recorded next discriminator is serial-log completeness/marker-boundary review.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: not run; no `docs/src` files were touched.
- git diff --cached --check before commit: passed.

## Result

Completed with classification `boot-runtime-readiness-blocked`.

This accepts known-good TFTP fetch visibility under the repaired stable TFTP rule, but it does not accept valid known-good Talos runtime readiness. RP1 candidate rerun and RP1 source work remain blocked until a follow-up discriminator explains the missing `TALOS: kernel_main` marker or otherwise updates the accepted readiness boundary through supervisor-planned work.
