# Phase 11 Pi 5 Capture Invariant Harness Static Evidence Inspection

Task id: phase11-pi5-capture-invariant-harness-core-20260606

## Inputs Inspected

- tasks/2026-06-06-phase11-rp1-post-handoff-marker-reset-pi5-discriminator.md
- tasks/2026-06-06-phase11-rp1-post-handoff-marker-reset-closeout.md
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-pi5-discriminator/classification.json
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-pi5-discriminator/evidence-map.json
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-pi5-discriminator/tftp-delta-stable-pre-restore.json
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-pi5-discriminator/tftp-delta-late-before-restore.json
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-pi5-discriminator/rerun-tftp-delta-stable-pre-restore.json
- tasks/evidence/2026-06-06-phase11-rp1-post-handoff-marker-reset-pi5-discriminator/control-tftp-delta-stable-pre-restore.json
- docs/src/project/lab-controller.md
- scripts/rpi5-wait-tftp-delta.sh
- scripts/rpi5-observe-runtime-readiness.sh

## Evidence Reconciliation

The latest marker/reset discriminator staged the accepted 51,736-byte
post-handoff marker/reset candidate and retained publication identity, fresh
serial/TFTP cursors, stable TFTP samples, late first-run TFTP replay, rerun
evidence, restored-control evidence, and restore proof. The closeout classified
the boundary as staging-capture-blocked because stable same-cursor TFTP samples
before restore did not prove candidate-tied fetches and the fresh serial windows
showed Raspberry Pi firmware/RP1 output without Talos entry or marker text.

The lab-controller proof contract already requires pre-run status/files,
snapshots, fresh serial and TFTP cursors, stable TFTP evidence before restore,
and final pre-restore state when inconclusive. The missing implementation piece
was a reusable helper that makes those invariants hard to skip and gives the
next worker one summary file to inspect before closeout.

## Accepted Harness Changes

- scripts/rpi5-observe-serial-window.sh accumulates serial from a fresh cursor
  until a requested deadline and annotates kernel-main, candidate marker, and
  firmware NETWORK observations.
- scripts/rpi5-capture-invariant-proof-bundle.sh records the full proof bundle
  after a caller stages the intended boot tree. It supports --dry-run so the
  workflow can be validated without power cycling hardware.
- docs/src/project/lab-controller.md now directs focused marker/reset proofs to
  use the capture-invariant helper and keeps the task record responsible for
  accepting or rejecting the feature boundary.

## Classification

Classification: ready-for-post-handoff-marker-reset-capture-recheck.

The next hardware proof can mechanically decide whether a run is
staging-publication-mismatch, tftp-capture-logging-blindness,
serial-only-firmware-reboot, post-handoff marker visible, candidate fetch
observed without marker, or a reset-side-effect candidate. The helper does not
accept those classifications by itself and does not accept RP1 mapped/unmapped
behavior.

## Findings And Disposition

- fixed: proof-bundle capture is now represented by an executable helper
  rather than only prose.
- fixed: focused serial observations now use the same deadline-loop principle
  already accepted for known-good runtime readiness.
- fixed: stable zero TFTP is quarantined to pre-restore same-cursor evidence.
- fixed: final pre-restore status/files are always retained after the bounded
  serial/TFTP observations, before restore.
- deferred: actual marker/reset classification remains blocked until the
  queued hardware recheck runs with the accepted candidate and hardware lock.
- not-an-issue: the helper's suggested_classification is only evidence
  annotation; acceptance still belongs to the task record and closeout.

## Validation Evidence

- bash -n: bash-n.log.
- helper dry-run/no-hardware: capture-invariant-proof-bundle-dry-run.json.
- git diff --check: git-diff-check.log.
- mdbook build: mdbook-build.log.
- git diff --cached --check: git-diff-cached-check.log.
