# Phase 11 Known-Good Serial Cursor Completeness Pi 5 Proof

Task id: phase11-known-good-serial-cursor-completeness-pi5-20260606

Status: accepted

## Goal

Run a serialized known-good Pi 5 proof through the repaired serial capture path
to prove fresh Talos serial bytes can still be observed after the prior serial
cursor saturation boundary.

## Scope

- Acquired hardwareTestLock and used the restored known-good Talos boot tree
  already selected by the prior repair and closeout tasks.
- Recorded pre-run status, boot files, snapshots, fresh serial and TFTP
  cursors, a direct-read serial window from saturated cursor 4194304, stable
  same-cursor TFTP evidence, restore evidence, and post-restore status.
- Did not publish an RP1 candidate, change source, classify RP1 MMIO behavior,
  or advance GPIO, interrupts, DMA/cache, storage, generated-root, networking,
  SSH, broader PCIe, Milestone 11.2, or any phase transition.

## Selected Known-Good Identity

- Tree hash:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
- Effective kernel: kernel_2712.img
- Expected fetch: da591740/kernel_2712.img
- Expected fetch bytes: 104,136
- Restore snapshot:
  phase11-known-good-serial-cursor-pre-20260606T1209Z

## Findings And Disposition

- fixed: preflight identity matched the selected known-good tree, effective
  kernel, expected fetch path, and 104,136-byte kernel_2712.img before the
  power cycle.
- fixed: the fresh serial cursor before power cycle was the saturated retention
  boundary, 4194304, so the repaired helper selected direct /serial/read and
  recorded observe_contract=deadline-loop-direct-read-after-saturated-cursor.
- fixed: direct /serial/read captured 6,347 fresh bytes after the saturated
  cursor boundary in 26 seconds, including firmware NETWORK output and the
  known-good Talos marker rpi5-production-timer-preemption: PASS.
- fixed: the stable same-cursor TFTP delta retained 13 events before restore,
  including two served da591740/kernel_2712.img fetches with 104,136 bytes.
- fixed: the proof restored the pre-run snapshot and retained post-restore
  status showing the selected tree hash and kernel_2712.img.
- not-an-issue: the serial window did not include TALOS: kernel_main, but the
  accepted known-good production-timer PASS marker is downstream Talos output
  and is sufficient for this serial cursor/capture completeness task.
- removed: no RP1 mapped/read-value, trap/unmapped, firmware-state, pre-MMIO
  reachability, GPIO, interrupts, DMA/cache, storage, generated-root,
  networking, SSH, broader PCIe, Milestone 11.2, or phase transition claim is
  made from this known-good proof.
- deferred: an RP1 UART0 FR-read rerun still requires the queued closeout to
  reconcile this proof with the prior saturated FR-read evidence.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-06-phase11-known-good-serial-cursor-completeness-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-06-phase11-known-good-serial-cursor-completeness-pi5/classification.json.
- Full proof bundle:
  tasks/evidence/2026-06-06-phase11-known-good-serial-cursor-completeness-pi5/known-good-run/.
- Serial excerpt:
  tasks/evidence/2026-06-06-phase11-known-good-serial-cursor-completeness-pi5/serial-window-excerpt.txt.
- Validation summary:
  tasks/evidence/2026-06-06-phase11-known-good-serial-cursor-completeness-pi5/validation-summary.txt.

## Validation

- serialized Pi 5 hardware run through lab-controller API: passed.
- candidate/known-good identity before power cycle: passed through GET /status,
  GET /boot/files, and preflight identity JSON.
- fresh serial cursor and deadline-looped serial window: passed with
  direct-read fallback from cursor 4194304.
- stable same-cursor TFTP evidence before restore: passed with 13 stable events
  and two 104,136-byte kernel_2712.img fetches.
- restore proof and hardware lock release evidence: passed.
- git diff --check: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted with classification
known-good-fresh-talos-serial-visible-after-saturated-cursor.

This accepts only the serial cursor/capture completeness repair path for the
known-good control. It does not accept RP1 UART0 FR-read behavior or any
broader Phase 11 feature.
