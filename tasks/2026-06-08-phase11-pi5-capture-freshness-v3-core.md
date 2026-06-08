# Phase 11 Pi 5 Capture Freshness V3 Core

Task id: phase11-pi5-capture-freshness-v3-core-20260608

Status: accepted

Classification: capture-transaction-v3-contract-accepted

## Goal

Make Pi 5 serial/TFTP capture freshness a first-class accepted discriminator
after the observed GPIO14 STATUS/CTRL candidate and known-good control both
failed the repaired freshness gate.

## Scope

- Inspected the accepted serial-drain freshness repair, observed GPIO status
  closeout, real candidate blocker, known-good control blocker,
  `rpi5-capture-invariant-proof-bundle.sh`, and the v2 identity-join checker.
- Added a v3 replay checker for retained proof bundles.
- Recorded a v3 hardware retry procedure for the queued known-good/control/real
  tasks without acquiring hardwareTestLock, publishing archives, power-cycling,
  or changing runtime RP1 diagnostics.

## V3 Contract

V3 keeps the v2 selected-tree, effective-kernel, expected-fetch, stable TFTP
delta, final pre-restore identity, and restore identity checks. It only changes
the saturated serial freshness discriminator.

For saturated direct-read serial, a non-empty bounded pre-power drain can be
accepted only when all of these are true:

- the serial window used `deadline-loop-direct-read-after-saturated-cursor`;
- the required marker is absent from every retained pre-power drain response;
- the same required marker is present in the post-power serial window;
- the TFTP delta contains the expected served fetches and the final selected
  tree still matches the preflight tree.

If the required marker is already present before power, V3 rejects the run as
`capture-staging-blocked`. Marker-visible direct-read serial by itself is still
not accepted.

## Findings And Disposition

- fixed: added `scripts/rpi5-proof-identity-join-v3-check.sh` as the
  task-owned v3 replay checker.
- fixed: documented the v3 contract and retry procedure in the lab-controller
  guide.
- fixed: updated the RP1/PCIe map contract so same-shaped observed GPIO14
  STATUS/CTRL retries remain blocked unless they pass v3 in the explicit queued
  tasks.
- fixed: retained a dry-run of the v3 known-good procedure using the existing
  bounded drain parameters and production-timer PASS marker.
- fixed: retained fixture replays showing the prior known-good evidence can be
  mechanically joined by v3 while a synthetic stale-marker replay is rejected.
- deferred: no lab-controller monotonic serial cursor endpoint was added; V3 is
  a repository-side replay contract until the lab exposes a stronger freshness
  primitive.
- not-an-issue: the prior real GPIO14 STATUS/CTRL marker-visible evidence is
  retained as a fixture replay only; this task does not retroactively accept
  GPIO14 STATUS/CTRL visibility or any GPIO/interrupt/RP1 ownership claim.

No findings were removed.

## Evidence

- Static problem analysis:
  tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-core/static-problem-analysis.md.
- Evidence map:
  tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-core/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-core/classification.json.
- Known-good retained v3 replay:
  tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-core/known-good-retained-v3-replay.json.
- Real retained fixture replay:
  tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-core/real-retained-v3-fixture-replay.json.
- Stale-marker rejection fixture:
  tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-core/stale-marker-replay-v3.json.
- Procedure dry-run:
  tasks/evidence/2026-06-08-phase11-pi5-capture-freshness-v3-core/v3-known-good-procedure-dry-run.json.

## Validation

- static inspection of observed GPIO status blocker evidence, known-good control
  evidence, prior repair evidence, and pi5-capture-transaction-v2: passed.
- bash -n scripts/rpi5-proof-identity-join-v3-check.sh
  scripts/rpi5-capture-invariant-proof-bundle.sh
  scripts/rpi5-proof-identity-join-check.sh: passed.
- retained v3 fixture replay: passed for known-good and real retained fixtures.
- stale-marker replay fixture: rejected as capture-staging-blocked.
- jq empty on evidence-map/classification/replay/dry-run JSON: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as capture-transaction-v3-contract-accepted. The next hardware task may
promote only the queued v3 known-good proof and must use the documented v3
checker; this task does not acquire hardware or accept GPIO14 STATUS/CTRL
visibility.
