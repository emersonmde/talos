# Phase 12 SSH Live TCP Candidate Capture Window Contract Repair

Task id: phase12-ssh-live-tcp-candidate-capture-window-contract-repair-20260629

Status: accepted after commit.

Classification: candidate-capture-window-contract-ready.

Evidence level: static inspection, local helper contract repair, retained
fixture replay, task-owned JSON evidence, docs build, and diff checks.

## Goal

Turn the v4 candidate preflight capture-window contamination into a mechanical,
fail-closed preflight contract before any further Pi 5 candidate retry.

## Scope Performed

- Reviewed the accepted v4 task record and task-owned JSON evidence:
  classification, evidence map, preflight summary, and candidate-run
  capture-invariant summary.
- Repaired `scripts/rpi5-capture-invariant-proof-bundle.sh` so future bundles
  emit `capture-window-order.json` with helper-owned stage ordering before and
  after restore.
- Added `scripts/rpi5-candidate-capture-window-v5-check.sh`, which wraps the
  v4 identity/freshness checker and fails closed unless stable TFTP and final
  pre-restore identity are ordered before restore.
- Updated the retained fixture gate to exercise the v5 checker, including
  accepted, missing-order, and restore-contaminated-order cases.
- Updated the lab-controller, Phase 12 SSH, and roadmap docs for the repaired
  capture-window contract.

No hardware, boot publication, packet-I/O discriminator,
OpenSSH/generated-root retry, remote receipt, compatibility, service success,
ssh-ready=true, broad shell work, or phase transition was performed or
accepted.

## Terminal Classification

candidate-capture-window-contract-ready.

The repaired contract names these owner paths:

- Helper: `scripts/rpi5-capture-invariant-proof-bundle.sh`
- Checker: `scripts/rpi5-candidate-capture-window-v5-check.sh`
- Retained fixture gate: `scripts/rpi5-capture-chain-v4-retained-fixtures.sh`

The v5 contract requires this stage order in `capture-window-order.json`:
preflight identity, pre-power cursors, power cycle, serial observe, stable TFTP
delta, final pre-restore identity, restore, and post-restore identity. Missing
order metadata, manual resume without helper-owned ordering, TFTP or final
identity captured after restore, post-restore/control identity in the
final-pre-restore slot, and selected-tree byte mismatches all block
candidate-capture-ready.

selected_next_task:
phase12-ssh-live-tcp-pi5-candidate-preflight-v5-20260629.

planningNeeded: false.

## Findings

- fixed: future capture-invariant bundles retain `capture-window-order.json`
  with helper-owned capture stages and evidence file references.
- fixed: the v5 checker combines the v4 selected-tree/TFTP/serial/final
  identity join with pre-restore-before-restore ordering.
- fixed: retained fixtures now include accepted, missing-order, and
  restore-contaminated-order cases.
- not-an-issue: v4 contaminated evidence remains blocked and cannot be
  reclassified as candidate-capture-ready.
- deferred: the next serialized Pi 5 hardware attempt belongs only to
  phase12-ssh-live-tcp-pi5-candidate-preflight-v5-20260629.

## Evidence Map

- Classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-candidate-capture-window-contract-repair/classification.json.
- Evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-candidate-capture-window-contract-repair/evidence-map.json.
- Reviewed v4 task:
  tasks/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v4.md.
- Reviewed v4 JSON:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v4/candidate-preflight-v4-20260629T161757Z/.

## Redaction Review

Task-owned evidence is metadata-only: task ids, file paths, helper names,
contract/stage names, classification strings, count/byte summaries, and
validation outcomes. It does not retain packet payloads, key material, session
material, boot artifact bytes, private user data, peer addresses, stable
secret-derived identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned promotion.
- Static inspection of v4 classification.json, evidence-map.json,
  preflight-summary.json, and candidate-run/capture-invariant-summary.json:
  pass.
- jq empty on retained v4 JSON evidence: pass; 38 JSON files.
- capture-invariant bundle dry-run: pass; `capture-window-order.json` appears
  in the declared would-write set.
- retained capture-window fixture gate: pass; 9 fixtures with v5 accepted,
  missing-order, and restore-contaminated-order coverage.
- v4 contaminated evidence through v5 checker: pass; expected exit 1 and
  capture-staging-blocked.
- shell script gate: pass with bash -n; shellcheck unavailable.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-candidate-preflight-v5-20260629.

Commit: recorded in talos-supervisor-state.json after final commit.
