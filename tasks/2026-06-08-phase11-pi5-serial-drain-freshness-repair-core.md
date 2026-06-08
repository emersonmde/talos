# Phase 11 Pi 5 Serial Drain Freshness Repair Core

Task id: phase11-pi5-serial-drain-freshness-repair-core-20260608

Status: accepted

Classification: serial-drain-freshness-procedure-repaired

## Goal

Repair or decisively classify the saturated pre-power serial-drain freshness
blocker that prevented the observed GPIO status no-MMIO control proof from
being accepted.

## Scope

- Inspected the committed observed GPIO status control blocker, its candidate
  and known-good evidence, the capture-invariant proof bundle helper, the
  identity-join checker, retained identity fixtures, prior serial saturation
  repair notes, and the lab-controller serial endpoint contract.
- Preserved the pi5-capture-transaction-v2 acceptance bar: saturated
  direct-read serial is decisive only after an empty pre-power /serial/read
  drain.
- Repaired the repository-side retry procedure by making the pre-power serial
  drain bounds configurable and retaining the chosen discriminator in dry-run
  and proof-bundle metadata.
- Did not acquire hardwareTestLock, publish a boot archive, power-cycle the Pi
  5, rerun the observed GPIO status control, run a known-good control, or
  change RP1 runtime source.

## Problem Analysis

Invariant: before a saturated direct-read serial window can support decisive
hardware classification, the proof bundle must first prove freshness by
draining /serial/read until an empty device-buffer response is observed before
the power cycle.

Contradicting evidence: the observed GPIO status no-MMIO control candidate and
the known-good production-timer control both selected the expected boot trees,
retained matching stable TFTP fetches, emitted their expected serial markers,
and restored to the baseline tree. Both failed pi5-capture-transaction-v2 for
the same reasons: serial-drain-not-empty-before-power and
saturated-direct-read-without-empty-pre-power-drain. Each pre-power drain used
16 fixed read attempts, accumulated 182,528 bytes, ended at the 4 MiB cursor
cap, and still had an 11,408-byte final read.

Unproven assumptions:

- Sixteen pre-power reads are enough to drain stale retained serial backlog
  before a new power cycle.
- A candidate or known-good marker seen after a non-empty saturated drain is
  fresh enough to accept without an empty pre-power read.
- The identical candidate and known-good freshness failure says anything about
  RP1 GPIO status behavior.

Approaches considered:

- Relax pi5-capture-transaction-v2 so saturated direct-read markers plus
  matching TFTP fetches are accepted. Rejected: this would accept stale serial
  output and shrink the original feature proof into a capture shim.
- Keep the helper unchanged and repeat the same hardware flow. Rejected: the
  fixed 16-read drain already failed on both candidate and known-good evidence,
  so an unchanged retry lacks a new discriminator.
- Make the pre-power drain bounds explicit and retained, then require the next
  hardware retry to classify either empty-read-before-power or
  bounded-drain-exhausted-before-power before interpreting saturated direct-read
  serial. Accepted.

Smallest decisive discriminator: a retry procedure that records configurable
pre-power drain bounds. The accepted next control retry should use
--serial-drain-attempts 96 --serial-drain-read-timeout 1
--serial-drain-settle-ms 100 --serial-drain-max-bytes 65536. If the drain
records discriminator=empty-read-before-power, the normal v2 identity join may
accept or reject the proof on the remaining identity/TFTP/final-restore fields.
If the drain records discriminator=bounded-drain-exhausted-before-power, the
result remains capture-staging-blocked even if serial markers and TFTP fetches
are visible.

Workaround quarantine/removal plan: the larger drain bounds are a task
procedure for the freshness repair and follow-up observed GPIO status retry,
not a relaxed global acceptance rule. Remove the larger bound recommendation
or reduce it back to defaults after a later accepted lab-controller endpoint
fix exposes a monotonic serial cursor or another first-class freshness signal
that proves the same invariant without bounded direct-read draining.

## Findings And Disposition

- fixed: the task record explains why the candidate and known-good runs failed
  the same serial freshness gate without treating either as RP1 behavior.
- fixed: scripts/rpi5-capture-invariant-proof-bundle.sh now accepts
  --serial-drain-attempts, --serial-drain-read-timeout,
  --serial-drain-settle-ms, and --serial-drain-max-bytes.
- fixed: capture dry-run metadata records the pre-power serial-drain
  discriminator and bounds for the accepted retry procedure.
- fixed: proof-bundle summaries now retain drain attempt_limit,
  read_timeout_seconds, settle_ms, max_bytes_per_read, and discriminator.
- fixed: docs/src/project/lab-controller.md documents the bounded retry
  procedure and states that bounded-drain-exhausted-before-power is still a
  capture-chain blocker.
- not-an-issue: scripts/rpi5-proof-identity-join-check.sh already rejects the
  same saturated stale-output failure and does not require relaxation.
- not-an-issue: the retained identity fixture replay still separates rejected
  stale-saturated evidence from the accepted no-MMIO control proof.
- deferred: no lab-controller endpoint change was made; the workaround remains
  quarantined to the explicit retry procedure until a monotonic serial
  freshness source exists.

No findings were removed.

## Evidence

- Static problem analysis:
  tasks/evidence/2026-06-08-phase11-pi5-serial-drain-freshness-repair-core/static-problem-analysis.md.
- Evidence map:
  tasks/evidence/2026-06-08-phase11-pi5-serial-drain-freshness-repair-core/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-pi5-serial-drain-freshness-repair-core/classification.json.
- Procedure dry-run:
  tasks/evidence/2026-06-08-phase11-pi5-serial-drain-freshness-repair-core/repair-procedure-dry-run.json.
- Retained fixture replay:
  tasks/evidence/2026-06-08-phase11-pi5-serial-drain-freshness-repair-core/retained-fixture-replay.json.

## Validation

- static inspection of blocker evidence, capture helpers, prior repair tasks,
  and lab-controller serial contract: passed.
- bash -n scripts/rpi5-capture-invariant-proof-bundle.sh
  scripts/rpi5-proof-identity-join-check.sh
  scripts/rpi5-capture-identity-join-retained-fixtures.sh: passed.
- capture-bundle dry-run with the accepted retry drain bounds: passed.
- task-owned retained fixture replay: passed.
- jq empty on evidence-map/classification/dry-run/fixture JSON: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as serial-drain-freshness-procedure-repaired. The next hardware retry
may use the documented bounded drain procedure; it may not accept saturated
direct-read serial unless the pre-power drain records empty-read-before-power.
