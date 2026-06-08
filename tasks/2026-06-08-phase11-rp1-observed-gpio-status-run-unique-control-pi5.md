# Phase 11 RP1 Observed GPIO Status Run-Unique Control Pi 5

Task id: phase11-rp1-observed-gpio-status-run-unique-control-pi5-20260608

Status: accepted

Classification: no-mmio-observed-gpio-status-run-unique-control-visible

## Goal

Prove the observed GPIO status no-MMIO/no-RP1/no-GIC control with the accepted
run-unique serial freshness discriminator.

## Scope

- Acquired the hardware lock for this task only.
- Built the no-MMIO observed GPIO status control archive with
  TALOS_CAPTURE_NONCE=ru20260608T195401Z-f84941d7.
- Static-reviewed the archive with the same nonce before publication.
- Captured the Pi 5 proof bundle with the required marker
  TALOS: rp1-observed-gpio-status-control capture-nonce=ru20260608T195401Z-f84941d7.
- Replayed the retained bundle through the run-unique checker and restored the
  lab to the pre-run tree.

## Findings And Disposition

- fixed: the no-MMIO control passed pi5-capture-transaction-run-unique-v1
  with the task-owned nonce present after power and absent before power.
- fixed: retained pre-power serial drain evidence reached
  empty-read-before-power after 2 attempts.
- fixed: retained stable TFTP evidence has two matching
  da591740/kernel_2712.img fetches of 49,072 bytes for selected tree
  2e0fbbdc8da0ec3066ddc4b74949887c8bcf80c70ac6c4a68edffb5dca6f5173.
- fixed: retained final pre-restore identity still pointed at the selected
  tree, and restore returned to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- not-an-issue: the control output includes synthetic no-MMIO GPIO14
  STATUS/CTRL report fields; these are control-shape evidence only and do not
  accept RP1/GPIO hardware behavior.
- deferred: real observed GPIO14 STATUS/CTRL hardware proof remains a separate
  queued task.

No findings were removed.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-control-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-control-pi5/classification.json.
- Run-unique checker:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-control-pi5/run-unique-check.json.
- Capture bundle:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-control-pi5/control-run/.
- Static archive review:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-control-pi5/static-archive-review.txt.
- Restore proof:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-control-pi5/control-run/restore-snapshot.json and
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-run-unique-control-pi5/control-run/post-restore-status.json.

## Validation

- lab-controller API: acquired snapshot, published the staged archive,
  power-cycled the Pi 5, captured serial/TFTP/final identity, and restored the
  pre-run tree.
- serial hardware boot/output: passed; required nonce marker was absent before
  power and present 37 times after power.
- TFTP evidence: passed; stable same-cursor delta retained 13 events with two
  matching 49,072-byte candidate fetches.
- run-unique checker: passed; classification
  capture-transaction-run-unique-ready, no rejection reasons.
- jq empty on classification, evidence map, run-unique checker, and capture
  summary JSON: passed.
- git diff --check: passed.
- git diff --cached --check: passed.

No docs/src files were touched, so mdbook was not required.

## Result

Accepted as no-mmio-observed-gpio-status-run-unique-control-visible. This
accepts only the no-MMIO control output/capture path. It does not accept
GPIO14 STATUS/CTRL visibility, GPIO ownership, event generation, interrupt
delivery, broad RP1 mapping, DMA/cache, networking, SSH, Milestone 11.3, or a
phase transition.
