# Phase 12 SSH Live TCP Candidate Vs Control Entry Delta Reconciliation V14

Task id: phase12-ssh-live-tcp-candidate-vs-control-entry-delta-reconciliation-v14-20260630

Status: accepted after commit.

Classification: candidate-entry-bisect-contract-required.

Evidence level: no-hardware static review of accepted v12/v13 task records,
retained serial/TFTP evidence, source/artifact metadata, task-owned JSON
evidence, docs build, and diff checks. No hardware action, lab publication,
boot snapshot mutation, Pi 5 power cycle, packet-I/O, OpenSSH/generated-root
retry, remote receipt, compatibility claim, service success claim,
ssh-ready=true, fake/kernel-backed command expansion, broad shell work, or
phase transition was performed.

## Goal

Isolate the candidate-specific selected-fetch/no-entry gap by comparing the
accepted v12 live TCP candidate against the accepted v13 passing selected-path
control before any hardware retry.

## Scope Performed

- Promoted this ready no-hardware task after the accepted v13 closeout selected
  it explicitly.
- Reviewed the accepted v12 runtime-marker candidate preflight, v13
  source/artifact reconciliation, v13 control contract, v13 control
  discriminator, and v13 closeout evidence.
- Inspected retained v12 serial evidence for early phase-line markers beyond
  the route-start/runtime-ready checker.
- Compared candidate and control source scenario selection, selected
  da591740/kernel_2712.img identity, Image header fields, section/symbol
  layout, marker strings, archive helper contracts, and marker emission order.
- Stopped before hardware, lab publication, packet-I/O,
  OpenSSH/generated-root retry, fake command expansion, broad shell work, or
  phase transition.

## Terminal Classification

candidate-entry-bisect-contract-required.

The first missing fact remains candidate-specific. The v12 candidate proved
stable selected da591740/kernel_2712.img fetch and final pre-restore identity,
but the retained serial window contains firmware NETWORK/config output only:
rust_entry, boot-info-parsed, target-init, exceptions-ready, kernel_main,
route-start, and runtime-ready labels are all absent. The v13 control proved
that a current-tree production-timer selected-path image can use the same
selected da591740/kernel_2712.img fetch path and reach a downstream PASS
marker, but its absent early phase lines are metadata-only only because PASS
appeared.

No bounded source/helper/archive repair is accepted in this task. The candidate
archive helper already uses the same selected serial-prefixed mirroring pattern
as the accepted control helpers, the v12 retained metadata shows matching root
and selected kernel bytes/hash, the Image header size matched the served file
size, and v13 static review retained _start/rust_entry/kernel_main symbols and
runtime marker strings in the candidate image. Those facts reject a simple
publication, selected-prefix, Image header, or missing-marker-string repair.

selected_next_task:
phase12-ssh-live-tcp-candidate-entry-bisect-control-contract-v14-20260630.

planningNeeded: false.

## Required Bisect Contract

The successor no-hardware contract must define exactly one qualitatively
different Pi 5 discriminator before any new hardware action:

- Control side: current-tree rpi5_minimal_entry_control selected-path archive
  produced by scripts/rpi5-minimal-entry-control-boot-tree.sh with a run-unique
  TALOS_CAPTURE_NONCE.
- Required control marker: TALOS: minimal-entry-control-ready
  capture-nonce=<run nonce>.
- Control purpose: prove whether a minimal current-tree kernel_main hook can
  enter Talos under the same selected da591740/kernel_2712.img path without
  production-timer secondary-core work, live TCP route construction, packet-I/O,
  OpenSSH, remote receipt, service readiness, or phase transition claims.
- Candidate side: the accepted live TCP runtime-marker candidate source/helper
  contract remains the failing endpoint; no same-shaped candidate rerun is
  authorized until the contract names the exact variable being changed or
  bracketed.
- Fail-closed classifications must distinguish selected identity/TFTP capture,
  control entry, candidate entry, restore, and inconclusive capture/staging.
- Packet-I/O/OpenSSH/generated-root retry remain blocked unless a later Pi 5
  task accepts candidate-capture-ready with selected fetch plus route-start and
  nonce-bearing runtime-ready markers.

## Findings

- fixed: promoted exactly one ready no-hardware v14 reconciliation and kept
  packet-I/O/OpenSSH/generated-root retry blocked.
- fixed: recorded the v12 retained serial phase-line disposition: firmware
  NETWORK/config output present, but rust_entry, boot-info-parsed, target-init,
  exceptions-ready, kernel_main, route-start, and runtime-ready absent.
- fixed: reconciled v13 control-entry-passes as generic selected-path control
  evidence, not live TCP candidate readiness.
- not-an-issue: the candidate helper's selected serial-prefix mirroring and
  Image header contract are not enough to explain the no-entry symptom; retained
  v12/v13 evidence shows matching root/selected kernel identity and valid Image
  header sizing.
- deferred: a minimal-entry current-tree selected-path control is the next
  discriminator; the exact hardware contract is deferred to the selected
  successor task.
- removed: blind live TCP rerun, packet-I/O, OpenSSH/generated-root retry,
  remote receipt, service readiness, and phase transition as permissible
  immediate successors.

## Evidence Map

- Task-owned evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-vs-control-entry-delta-reconciliation-v14/evidence-map.json.
- Accepted v12 candidate preflight:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v12.md.
- Accepted v13 source/artifact reconciliation:
  tasks/2026-06-30-phase12-ssh-live-tcp-candidate-entry-boundary-source-artifact-reconciliation-v13.md.
- Accepted v13 control contract:
  tasks/2026-06-30-phase12-ssh-live-tcp-candidate-entry-control-contract-v13.md.
- Accepted v13 control discriminator:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-entry-control-discriminator-v13.md.
- Accepted v13 closeout:
  tasks/2026-06-30-phase12-ssh-live-tcp-kernel-entry-boundary-closeout-v13.md.

## Redaction Review

This task retained task ids, commit ids, path labels, hashes, byte counts,
marker labels, classifications, validation command results, and selected
successor metadata. It retained no packet payloads, SSH keys/session material,
boot artifact bytes, private user data, stable secret-derived identifiers, or
unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before promotion.
- jq empty on referenced v12/v13 JSON evidence and task-owned evidence: pass.
- Retained v12 serial phase-line review: pass; early Talos phase labels and
  live TCP route/runtime labels are absent while firmware NETWORK is present.
- Static candidate-vs-control source/artifact comparison: pass; no bounded
  repair accepted, successor discriminator required.
- sh -n for touched shell helpers/classifiers: not run; no shell helpers or
  classifiers were touched.
- Non-published archive/helper review: satisfied by retained v12/v13 static
  archive/helper evidence; no generated boot/archive bytes retained by this
  task.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-candidate-entry-bisect-control-contract-v14-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
