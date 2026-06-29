# Phase 12 SSH Live TCP Candidate Capture Window Run Ownership Contract

Task id: phase12-ssh-live-tcp-candidate-capture-window-run-ownership-contract-20260629

Status: accepted after commit.

Classification: capture-window-run-ownership-ready.

Evidence level: static inspection, local helper/checker repair, retained
fixture replay, blocked v5 evidence replay, task-owned JSON evidence, docs
build, and diff checks.

## Goal

Turn the v5 blocked-capture-window-contract result into an executable
run-ownership contract so the next Pi 5 preflight can either retain a clean
helper-owned pre-restore window or fail closed before packet-I/O or OpenSSH
work.

## Scope Performed

- Reviewed the accepted v5 preflight task, classification, preflight summary,
  repaired v5 checker, retained fixture gate, Phase 12 docs, roadmap, and
  supervisor state.
- Repaired `scripts/rpi5-capture-invariant-proof-bundle.sh` so
  `capture-window-order.json` starts incomplete and is marked complete only
  after the helper captures post-restore identity.
- Repaired `scripts/rpi5-candidate-capture-window-v5-check.sh` so
  candidate-capture-ready requires `helper_run_completed=true`,
  `completed_at`, and a completion event count matching the helper-owned
  stage list.
- Extended `scripts/rpi5-capture-chain-v4-retained-fixtures.sh` with a
  helper-incomplete fixture while preserving accepted, missing-order, and
  restore-contaminated coverage.
- Updated Phase 12 docs and roadmap to state the v6 execution contract.

No lab API mutation, hardwareTestLock acquisition, boot publication,
power-cycle, packet-I/O discriminator, OpenSSH/generated-root retry, remote
receipt, compatibility, service success, ssh-ready=true, broad shell work, or
phase transition was performed.

## Run-Ownership Contract

The next hardware preflight must run
`scripts/rpi5-capture-invariant-proof-bundle.sh` in the foreground to
completion with a runtime budget larger than the configured serial and TFTP
windows. No manual restore or external restore is allowed while the helper is
running. If helper completion is lost, the checker output is unavailable, or
the retained `capture-window-order.json` lacks matching completion metadata,
the run must be classified blocked-capture-window-contract or blocked-restore.

candidate-capture-ready requires one uninterrupted helper-owned run containing
preflight identity, pre-power cursors, power cycle, serial window, stable TFTP
delta, final pre-restore identity, helper-owned restore, post-restore identity,
and helper completion metadata. Restored-control byte/tree evidence or stale
post-restore replay cannot satisfy candidate pre-restore proof.

selected_next_task:
phase12-ssh-live-tcp-pi5-candidate-preflight-v6-20260629.

planningNeeded: false.

## Findings

- fixed: the helper now records `helper_run_started_at`,
  `helper_run_completed=false`, `completed_at=null`, and marks completion
  only after post-restore identity is captured.
- fixed: the checker fails closed on helper-incomplete windows via
  capture-window-helper-run-incomplete, capture-window-helper-completion-missing,
  and capture-window-helper-completion-count-mismatch.
- fixed: retained fixtures now cover 10 cases, including accepted,
  missing-order, restore-contaminated, and helper-incomplete windows.
- not-an-issue: the v5 selected-tree, byte, serial freshness, final identity,
  and restore-control rejection checks remain intact.
- blocked: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility, service success, ssh-ready=true, broad shell work, and phase
  transition remain blocked until the explicit v6 hardware preflight accepts
  candidate-capture-ready.

## Evidence Map

- Classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-candidate-capture-window-run-ownership-contract/classification.json.
- Evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-candidate-capture-window-run-ownership-contract/evidence-map.json.
- Reviewed v5 task:
  tasks/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v5.md.
- Reviewed v5 evidence:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v5/candidate-preflight-v5-20260629T172809Z/.

## Redaction Review

Task-owned evidence is metadata-only: task ids, file paths, helper/checker
names, classification strings, fixture names, rejection reason names, validation
commands/results, and summary counts. It does not retain packet payloads, key
material, session material, boot artifact bytes, private user data, peer
addresses, stable secret-derived identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits: pass; main ahead of origin with no
  uncommitted Talos repo changes before promotion.
- Static inspection of v5 task/evidence, v5 checker, capture-invariant helper,
  Phase 12 docs, roadmap, and supervisor state: pass.
- bash -n on touched shell scripts: pass.
- Retained capture-window fixture gate: pass; 10 fixtures including accepted,
  missing-order, restore-contaminated, and helper-incomplete windows.
- Blocked v5 evidence replay through the tightened checker: pass as blocker
  evidence; exited 1 with restored-control identity/byte reasons and
  helper-incomplete completion reasons.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

Terminal classification: capture-window-run-ownership-ready.

selected_next_task:
phase12-ssh-live-tcp-pi5-candidate-preflight-v6-20260629.

Commit: recorded in talos-supervisor-state.json after final commit.
