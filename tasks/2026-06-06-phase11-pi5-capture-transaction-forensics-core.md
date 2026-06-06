# Phase 11 Pi 5 Capture Transaction Forensics Core

Task id: phase11-pi5-capture-transaction-forensics-core-20260606

Status: accepted

## Goal

Repair or classify the Pi 5 proof capture transaction after the repaired RP1
FR-read proof produced candidate-like serial output while TFTP and final
identity matched the restored known-good tree.

## Scope

- Inspected the retained repaired-proof candidate, known-good control,
  candidate rerun, observe-helper trace, capture-invariant helper, identity
  checker, lab-controller contract, and task records.
- Recorded the repetition-escape first-principles analysis in static evidence.
- Repaired the proof transaction contract without changing RP1 runtime source,
  constants, or boot archives.
- Added pi5-capture-transaction-v2, which requires an empty pre-power serial
  drain before saturated direct-read output can be accepted as fresh.
- Replayed the retained f274ff7 candidate run through the v2 checker.

## Classification

capture-transaction-v2-ready.

The retained f274ff7 mismatch is explained as
serial-freshness-contract-fixed. The v1 contract rejected the run because TFTP
and final pre-restore identity matched the restored known-good tree, but it did
not explicitly require proof that the saturated direct-read serial path began
after an empty pre-power drain. That left large direct-read output useful as
blocker evidence but too weak as fresh candidate evidence.

The v2 contract keeps the existing selected-tree, expected-fetch, stable TFTP,
final pre-restore, and restore requirements, and adds
serial-drain-before-power.json. The drain must reach an empty read before the
power cycle. If direct-read is used from the saturated cursor, it is decisive
only when that empty drain proof is present and the rest of the selected
candidate identity still joins.

Replaying the retained f274ff7 candidate run under v2 remains
capture-staging-blocked with rejection reasons:
missing-v2-serial-drain-contract, serial-drain-not-empty-before-power,
saturated-direct-read-without-empty-pre-power-drain,
tftp-expected-fetch-byte-mismatch,
final-pre-restore-selected-tree-mismatch, and
final-pre-restore-expected-fetch-byte-mismatch.

No RP1 hardware behavior is accepted by this task. RP1 UART0 FR
mapped/read-value behavior, bus-fault/trap behavior, firmware-state behavior,
GPIO, interrupts, DMA/cache, storage, generated-root, networking, SSH, broader
PCIe, Milestone 11.2, and phase transition remain unaccepted.

## Findings And Disposition

- fixed: the capture bundle now records serial-drain-before-power.json,
  draining up to eight 64 KiB serial chunks until an empty pre-power read is
  observed.
- fixed: the proof contract version is now pi5-capture-transaction-v2 for new
  bundles and summaries.
- fixed: the identity checker now rejects saturated direct-read serial unless
  the v2 empty pre-power serial drain is present.
- fixed: replaying old bundles without the v2 drain emits JSON blocker evidence
  instead of exiting before classification.
- fixed: dry-run output exposes the v2 required fields before hardware use.
- deferred: the queued no-RP1-MMIO sentinel must prove the v2 transaction on
  Pi 5 hardware before any RP1 FR-read candidate retry.
- removed: no retained saturated direct-read serial text can be promoted to
  candidate behavior without the v2 drain and identity join.
- not-an-issue: the prior known-good control remains useful v1 proof-chain
  health evidence, but the next sentinel must use v2 before unlocking further
  RP1 hardware classification.

## Evidence

- Static evidence inspection:
  tasks/evidence/2026-06-06-phase11-pi5-capture-transaction-forensics-core/static-evidence-inspection.md.
- Evidence map:
  tasks/evidence/2026-06-06-phase11-pi5-capture-transaction-forensics-core/evidence-map.json.
- v2 dry-run:
  tasks/evidence/2026-06-06-phase11-pi5-capture-transaction-forensics-core/capture-transaction-v2-dry-run.json.
- f274ff7 replay:
  tasks/evidence/2026-06-06-phase11-pi5-capture-transaction-forensics-core/f274ff7-v2-replay.json.
- Classification:
  tasks/evidence/2026-06-06-phase11-pi5-capture-transaction-forensics-core/classification.json.

## Validation

- static inspection of f274ff7 candidate-run, known-good-control,
  candidate-rerun, observe-helper trace, capture-invariant summary, and
  identity-join checker evidence: passed.
- bash -n scripts/rpi5-capture-invariant-proof-bundle.sh
  scripts/rpi5-proof-identity-join-check.sh: passed.
- no-hardware dry-run of the v2 capture transaction contract: passed.
- no-hardware replay of f274ff7 retained evidence: passed with expected
  nonzero blocker exit and classification capture-staging-blocked.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as capture-transaction-v2-ready. The next mechanically unblocked task
is the no-RP1-MMIO sentinel hardware run, which must use the v2 proof contract
before any RP1 UART0 FR-read candidate retry.
