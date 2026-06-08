# Phase 11 Pi 5 Boot Staging Identity Repair Core

Task id: phase11-pi5-boot-staging-identity-repair-core-20260608

Status: accepted

Classification: boot-staging-identity-discriminator-accepted

## Goal

Repair or decisively classify the Pi 5 boot-staging identity mismatch that made
the run-unique observed GPIO14 STATUS/CTRL proof non-decisive.

## Scope

- Inspected the run-unique observed GPIO status blocker, run-unique control
  proof, run-unique checker output, lab-controller boot identity docs, and
  capture/proof helper scripts.
- Added `scripts/rpi5-boot-staging-identity-check.sh`, a retained-evidence
  checker for boot-staging identity only.
- Replayed the helper against the accepted run-unique no-MMIO control, the
  primary real run, and the clean real retry.
- Updated the lab-controller proof procedure so marker-visible run-unique serial
  output must pass the staging discriminator before any RP1/GPIO claim.

## Non-Goals

No Pi 5 hardware run, boot archive publication, power-cycle, hardwareTestLock
acquisition, RP1 runtime diagnostic change, GPIO/RP1/PCIe/GIC MMIO change,
GPIO write, interrupt enablement or delivery, endpoint config retry, bridge
setup write, DMA/cache, storage, generated-root, networking, SSH, Milestone
11.3, or phase transition.

## Findings And Disposition

- fixed: recorded a first-principles problem statement, invariant,
  contradicting evidence, assumptions, approaches, decisive discriminator, and
  quarantine plan.
- fixed: explained why the primary run could retain nonce-bearing GPIO14
  STATUS/CTRL serial text while TFTP and final identity matched baseline:
  saturated-cursor direct-read serial text is not authoritative unless the
  selected tree, expected TFTP fetch bytes, final identity, and restore identity
  join to one candidate boot.
- fixed: explained the clean retry result: the fresh nonce was not visible after
  power, and TFTP/final identity again proved baseline-sized fetches.
- fixed: added `rpi5-boot-staging-identity-check.sh`, which ignores serial and
  RP1 output and accepts only selected tree plus expected TFTP fetch bytes plus
  final pre-restore selected-tree identity plus restore identity.
- fixed: retained replay evidence showing the accepted no-MMIO control passes
  as `boot-staging-identity-ready`.
- fixed: retained replay evidence showing both real observed GPIO14 STATUS/CTRL
  runs fail as `boot-staging-identity-blocked` with
  `tftp-expected-fetch-byte-mismatch`,
  `final-pre-restore-selected-tree-mismatch`,
  `final-pre-restore-is-baseline`, and
  `final-pre-restore-expected-fetch-byte-mismatch`.
- fixed: updated the next hardware procedure to run a no-RP1/no-MMIO
  known-good staging control through the staging discriminator before any real
  observed GPIO14 STATUS/CTRL retry.
- deferred: repository evidence does not prove why the lab-visible boot tree
  returned to baseline between candidate preflight and final identity; this
  helper quarantines that risk for future hardware tasks.
- not-an-issue: no runtime source changes were needed because this task repairs
  the acceptance/procedure gate, not the GPIO/RP1 diagnostic.

No findings were removed.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-pi5-boot-staging-identity-repair-core/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-pi5-boot-staging-identity-repair-core/classification.json.
- Static problem analysis:
  tasks/evidence/2026-06-08-phase11-pi5-boot-staging-identity-repair-core/static-problem-analysis.md.
- Accepted control replay:
  tasks/evidence/2026-06-08-phase11-pi5-boot-staging-identity-repair-core/control-staging-identity-check.json.
- Primary real run replay:
  tasks/evidence/2026-06-08-phase11-pi5-boot-staging-identity-repair-core/real-run-staging-identity-check.json.
- Clean real retry replay:
  tasks/evidence/2026-06-08-phase11-pi5-boot-staging-identity-repair-core/real-run-final-staging-identity-check.json.

## Validation

- static inspection: run-unique blocker evidence, run-unique checker output,
  accepted control evidence, lab-controller proof docs, and helper scripts
  inspected.
- bash -n on touched shell script: passed.
- retained replay validation: accepted no-MMIO control passed
  `boot-staging-identity-ready`; primary and clean real runs failed as
  `boot-staging-identity-blocked`.
- jq empty on evidence-map, classification, and retained replay JSON artifacts:
  passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as boot-staging-identity-discriminator-accepted. The next mechanically
unblocked hardware task is the already queued known-good no-RP1/no-MMIO staging
control, provided hardwareTestLock remains unlocked/restored.
