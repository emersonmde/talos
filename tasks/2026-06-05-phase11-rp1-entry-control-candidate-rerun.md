# Phase 11 RP1 Entry-Control Candidate Rerun

Task id: phase11-rp1-entry-control-candidate-rerun-20260605

Status: completed with blocker

## Goal

Rerun the accepted RP1 entry-control candidate on Pi 5 after known-good Talos readiness was accepted under the repaired marker-boundary rule.

## Scope

- Published only the accepted target/talos-rpi5-rp1-entry-control-source-core.tar.gz archive.
- Acquired the hardware lock, snapshotted the restored pre-run boot tree, captured candidate archive identity, fresh serial and TFTP cursors, serial output, stable pre-restore TFTP delta, final pre-restore status and boot files, restore evidence, and post-restore state.
- Classified only candidate fetch, Rust entry, and pre-BootInfo entry-control reachability.
- Kept RP1 MMIO reads, mapped/unmapped behavior, GPIO ownership, interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone 11.2, and phase transition out of scope.

## Findings And Disposition

- fixed: the rerun used the accepted source-core candidate archive dcbcf06ebdf2304630dc52d0aac689c6ec363f04074a055bc391a0c7829e5f37; archive review retained kernel SHA-256 b3e62b950cf007a0ee8d1d7f420fd8c26c28573c5b6925a7f0d93d0b77a367ea and 51,808-byte da591740/kernel_2712.img.
- fixed: publication staged candidate boot tree ab88a3d8549837459c8cebf8cb22580b52b39665421b7eb6d6773ebce8c6f9c2 with effective kernel kernel_2712.img.
- fixed: the stable pre-restore TFTP delta observed two served da591740/kernel_2712.img fetches at 51,808 bytes, so the prior staging/capture blocker is not the result classification for this run.
- deferred: serial output from the fresh cursor contained Raspberry Pi firmware network boot logs and candidate config fetches, but did not contain TALOS: kernel_main, rpi5-rp1-entry-control: rust-entry-control, rpi5-rp1-entry-control: no-rp1-mmio, classification=entry-control-reached, or rpi5-rp1-entry-control: PASS.
- not-an-issue: no RP1 mapped/read-value, unmapped/trap, firmware-state, GPIO ownership, interrupt, DMA/cache, storage, generated-root, networking, or SSH behavior was accepted.
- not-an-issue: the final restore returned to pre-run tree hash a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with effective kernel kernel_2712.img.

## Evidence

- Evidence map: tasks/evidence/2026-06-05-phase11-rp1-entry-control-candidate-rerun/evidence-map.json.
- Classification: tasks/evidence/2026-06-05-phase11-rp1-entry-control-candidate-rerun/classification.json.
- Candidate identity and archive review: candidate-archive-sha256.txt, candidate-kernel-sha256.txt, and candidate-archive-review.log.
- Hardware lock and boot identity: pre-run-status.json, pre-run-boot-files.json, pre-run-snapshots.json, pre-run-snapshot-name.txt, and pre-run-snapshot-create.json.
- Candidate run: candidate-publish.json, candidate-status-after-publish.json, candidate-boot-files-after-publish.json, candidate-serial-cursor.json, candidate-tftp-cursor.json, candidate-power-cycle.json, candidate-serial-entry-control-observe.json, and candidate-tftp-delta-stable-pre-restore.json.
- Restore evidence: final-pre-restore-status.json, final-pre-restore-boot-files.json, tftp-tail-pre-restore.json, final-restore.json, post-restore-status.json, and post-restore-boot-files.json.

## Validation

- serialized Pi 5 hardware evidence through lab-controller endpoints: completed.
- serial hardware boot/output: fresh cursor output reached Pi firmware network boot and candidate config fetch logs, but no Talos or entry-control marker.
- TFTP hardware evidence: stable pre-restore delta observed two 51,808-byte da591740/kernel_2712.img candidate fetches.
- classification: candidate-fetch-observed-without-entry-control.
- git diff --check: passed.
- mdbook build: not run; no docs/src files were touched.
- git diff --cached --check before commit: passed.

## Result

Completed with blocker: candidate-fetch-observed-without-entry-control.

This run proves that the accepted entry-control candidate was fetched under the repaired TFTP stability rule, but it does not prove Rust entry, entry-control reachability, or any RP1 mapped/unmapped behavior. The smallest next discriminator is supervisor-planned source/handoff/runtime-readiness investigation before BootInfo or the focused scenario branch, not RP1 constants, GPIO ownership, interrupts, DMA/cache, networking, SSH, storage, generated-root work, broader PCIe, or Milestone 11.2.
