# Phase 11 Pi 5 Capture Identity Join Repair Core

Task id: phase11-pi5-capture-identity-join-repair-core-20260607

Status: accepted

## Goal

Repair or decisively classify the Pi 5 capture/staging identity-join boundary
that blocked the GPIO14 STATUS proof, using retained evidence before any
further hardware run.

## Scope

- Replayed retained GPIO14 evidence through the pi5-capture-transaction-v2
  identity join without acquiring hardwareTestLock or publishing a boot
  archive.
- Added a deterministic retained-fixture replay command covering the
  marker-visible rejected run, the clean candidate rerun, and the accepted
  no-MMIO control proof.
- Checked the fixture assertions for empty versus non-empty pre-power serial
  drain, selected-tree identity, expected kernel byte count, final pre-restore
  identity, and same-cursor TFTP deltas.

## Non-Goals

No Pi 5 hardware run, boot archive publication, hardwareTestLock acquisition,
runtime RP1 diagnostic source change, GPIO/pin-control writes, interrupt
enablement, clock/reset programming, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe enumeration, Milestone 11.3, phase transition,
or same-shaped GPIO14 STATUS hardware rerun.

## Classification

Accepted as capture-identity-join-no-change-needed.

The retained replay shows the existing pi5-capture-transaction-v2 identity
contract already separates the three relevant cases:

- marker-visible rejected real GPIO14 run: capture-staging-blocked. It retains
  46,336-byte expected candidate identity but fails because pre-power serial
  drain was not empty, saturated direct-read output was therefore not fresh
  enough, TFTP fetch bytes were 104,136 instead of 46,336, and final
  pre-restore identity no longer matched the selected candidate tree.
- clean candidate rerun: capture-staging-blocked. It has empty pre-power serial
  drain and final selected-tree identity, but the same-cursor TFTP delta has no
  expected candidate fetch, so hardware behavior still cannot be accepted.
- no-MMIO control proof: capture-transaction-v2-ready. It has empty pre-power
  serial drain, two selected 46,160-byte TFTP fetches, final selected-tree
  identity, and no rejection reasons.

No v2 contract relaxation is justified. The repair for this task is retained
regression coverage around the exact blocker boundary, not a change to the
hardware acceptance criteria.

## Findings And Disposition

- fixed: added scripts/rpi5-capture-identity-join-retained-fixtures.sh so the
  retained marker-visible, clean-rerun, and no-MMIO-control evidence can be
  replayed deterministically without hardware.
- fixed: the fixture asserts the pre-power serial drain distinction that blocks
  the marker-visible run but permits the no-MMIO control proof.
- fixed: the fixture asserts selected-tree identity, expected kernel byte
  counts, final pre-restore identity, and same-cursor TFTP fetch counts for all
  three retained runs.
- not-an-issue: the marker-visible GPIO14 serial text remains useful blocker
  evidence, but it is still not accepted RP1 GPIO14 STATUS behavior.
- not-an-issue: the clean candidate rerun does not require source changes; it
  fails only the candidate-tied TFTP requirement.
- deferred: the next queued hardware task must prove the repaired/audited
  identity chain through a no-MMIO control rerun before any real GPIO14 proof
  rerun.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-pi5-capture-identity-join-repair-core/evidence-map.json.
- Retained fixture replay:
  tasks/evidence/2026-06-07-phase11-pi5-capture-identity-join-repair-core/retained-fixture-replay.json.
- Classification:
  tasks/evidence/2026-06-07-phase11-pi5-capture-identity-join-repair-core/classification.json.

## Validation

- task-owned retained-evidence replay/regression command:
  scripts/rpi5-capture-identity-join-retained-fixtures.sh passed.
- bash -n scripts/rpi5-capture-identity-join-retained-fixtures.sh
  scripts/rpi5-proof-identity-join-check.sh
  scripts/rpi5-capture-invariant-proof-bundle.sh: passed.
- cargo fmt --all -- --check: not run; no Rust files touched.
- cargo -Zjson-target-spec test --quiet: not run; no Rust/shared proof helper
  code touched.
- git diff --check: passed.
- mdbook build: not run; no docs/src files touched.
- git diff --cached --check before commit: passed.

## Result

Accepted as capture-identity-join-no-change-needed. The next queued
phase11-pi5-capture-identity-repaired-control-pi5-20260607 hardware task is
mechanically unblocked on a future worker wake if hardwareTestLock remains
unlocked/restored.
