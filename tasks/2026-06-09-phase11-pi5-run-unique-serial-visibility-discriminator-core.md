# Phase 11 Pi 5 Run-Unique Serial Visibility Discriminator Core

Task id: phase11-pi5-run-unique-serial-visibility-discriminator-core-20260609

Status: accepted

Classification: run-unique-serial-visibility-discriminator-repaired

## Goal

Repair or classify the Pi 5 run-unique serial visibility discriminator after the
GPIO16 no-MMIO control staged and fetched correctly but the prior checker
reported no current-run marker after power.

## Scope

- Inspected the committed GPIO16 no-MMIO control blocker evidence, the accepted
  run-unique/v3/boot-staging checker evidence, and the capture/checker scripts.
- Repaired only the local run-unique replay checker.
- Updated the lab-controller, RP1/PCIe map-contract, and roadmap docs for the
  accepted nonce-token serial visibility rule.
- Retained replay evidence for the observed blocker shape and the rejection
  cases needed before another GPIO16 no-MMIO control attempt.
- Did not acquire the hardware lock, publish an archive, power-cycle the Pi 5,
  or accept GPIO/RP1/GIC/PCIe hardware behavior.

## Findings And Disposition

- fixed: the checker now treats a run-unique
  capture-nonce=&lt;nonce&gt; token as the current-run serial visibility
  discriminator when exact marker matching fails only because the marker line's
  field order differs.
- fixed: the checker still requires the nonce token to be absent before power
  and present after power.
- fixed: non-serial v3 gates for selected identity, TFTP, final identity, and
  restore proof remain mandatory.
- fixed: fixture replay covers nonce-token-present, marker-absent-after-power,
  stale-before-power, and staging-mismatch cases.
- fixed: docs now say the run-unique contract requires nonce-token absence
  before power and presence after power, not a brittle field-order-specific
  marker string.
- deferred: no Pi 5 retry is performed by this task.
- not-an-issue: the prior GPIO16 control run remains recorded as a committed
  blocker; this task accepts only the local/static discriminator repair and
  replay behavior.

No findings were removed.

## Evidence

- Blocker analysis:
  tasks/evidence/2026-06-09-phase11-pi5-run-unique-serial-visibility-discriminator-core/blocker-analysis.md.
- Evidence map:
  tasks/evidence/2026-06-09-phase11-pi5-run-unique-serial-visibility-discriminator-core/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-09-phase11-pi5-run-unique-serial-visibility-discriminator-core/classification.json.
- Replay outputs:
  tasks/evidence/2026-06-09-phase11-pi5-run-unique-serial-visibility-discriminator-core/nonce-token-present.json,
  tasks/evidence/2026-06-09-phase11-pi5-run-unique-serial-visibility-discriminator-core/marker-absent-after-power.json,
  tasks/evidence/2026-06-09-phase11-pi5-run-unique-serial-visibility-discriminator-core/stale-before-power.json, and
  tasks/evidence/2026-06-09-phase11-pi5-run-unique-serial-visibility-discriminator-core/staging-mismatch.json.

## Validation

- static inspection of the blocker evidence, accepted checker evidence, and
  touched checker script: passed.
- task-owned fixture/replay validation: passed; nonce-token-present exits 0,
  marker-absent-after-power exits 1, stale-before-power exits 1, and
  staging-mismatch exits 1.
- bash -n on touched shell script: passed.
- jq empty on evidence-map, classification, and replay JSON artifacts: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

No Rust files were touched, so cargo gates were not required.

## Result

Accepted as run-unique-serial-visibility-discriminator-repaired. The next
GPIO16 no-MMIO control retry remains responsible for producing the serialized
Pi 5 control proof with retained candidate identity, TFTP, final identity,
restore proof, and current-run serial visibility.
