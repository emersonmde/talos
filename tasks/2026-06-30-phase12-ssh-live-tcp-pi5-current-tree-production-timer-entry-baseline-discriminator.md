# Phase 12 SSH Live TCP Pi 5 Current-Tree Production-Timer Entry Baseline Discriminator

Task id: phase12-ssh-live-tcp-pi5-current-tree-production-timer-entry-baseline-discriminator-20260630

Status: accepted after commit.

Classification: current-tree-entry-path-passes-control.

Evidence level: serialized Pi 5 hardware boot/output, lab-controller API
identity, fresh serial cursor/completeness diagnostics, stable same-cursor TFTP
delta before restore, marker capture/readiness-v3 classifier, restore proof,
task-owned JSON evidence, docs build, and diff checks. No live TCP candidate
retry, packet-I/O discriminator, OpenSSH/generated-root retry, remote receipt,
compatibility claim, service success claim, ssh-ready=true, broad shell work,
or phase transition was performed.

## Goal

Distinguish a broad current-tree boot-entry regression from a
minimal/live-TCP-scenario-specific entry defect by booting the current-tree
production-timer control on the Pi 5 under the selected-path capture contract.

## Scope Performed

- Promoted the queued hardware discriminator only after the no-hardware
  current-tree production-timer control accepted and selected this exact task.
- Acquired hardwareTestLock before lab publication, Pi 5 power action, and
  restore-affecting lab operations.
- Published a freshly materialized current-tree production-timer boot archive
  from source commit 1b428d0e4fda3fd26fe3869c7e5f43d648bba11a, with matching
  root/selected da591740/kernel_2712.img at 104,136 bytes and kernel SHA-256
  2343a009a14972d050ccf0fc706539163b6b5cb3ee3717b9cb6753f2ec7c2328.
- Ran the Pi 5 current-tree control and retained selected TFTP fetches,
  firmware NETWORK serial output, rpi5-production-timer-preemption: PASS, final
  pre-restore identity, and restore proof.
- A second same-contract run was also retained after the first helper process
  remained active while the worker shell returned early; both runs independently
  produced the same accepted classification.
- Restored phase12-ssh-v10-openssh-clean-pre-20260624T074100Z before releasing
  the hardware lock.

## Terminal Classification

current-tree-entry-path-passes-control.

The current-tree production-timer control reaches the downstream
production-timer PASS marker on Pi 5 under the selected-path capture contract.
The primary retained run
current-tree-production-timer-entry-baseline-20260630T051123Z observed stable
current-tree identity 4edd4f1dad12ea06e3c45b1435f9a2d16e9c2046226d8963a0d8413a9f7226d1,
two selected da591740/kernel_2712.img TFTP serves, 8,263 post-power serial
bytes, firmware NETWORK output, one rpi5-production-timer-preemption: PASS
marker, valid readiness-v3 classification, and restore to the accepted
a0452458... selected-control tree. The repeated retained run
current-tree-production-timer-entry-baseline-20260630T050929Z observed the same
classification with 8,203 post-power serial bytes.

TALOS: kernel_main was absent from both retained serial windows. As with the
accepted selected known-good baseline, readiness-v3 records this as metadata
only when the downstream production-timer PASS marker, selected fetch identity,
stable boot identity, and cursor windows are present.

selected_next_task:
phase12-ssh-live-tcp-minimal-control-scenario-specific-reconciliation-20260630.

planningNeeded: false.

## Findings

- fixed: proved the current-tree production-timer control on Pi 5 reaches the
  downstream PASS marker under selected-path capture.
- fixed: retained selected current-tree TFTP fetch identity, final pre-restore
  current-tree identity, serial window, readiness-v3 classification, and
  restore proof.
- fixed: restored the lab to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z
  and released hardwareTestLock after the proof.
- not-an-issue: TALOS: kernel_main is absent, but readiness-v3 treats it as
  metadata-only for this production-timer control when PASS is present.
- deferred: minimal-control scenario-specific reconciliation is selected next;
  live TCP candidate preflight, packet-I/O, OpenSSH/generated-root retry,
  remote receipt, compatibility, service success, ssh-ready=true, broad shell
  work, and phase transition remain dependency-gated.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-current-tree-production-timer-entry-baseline-discriminator/evidence-map.json.
- Primary run:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-current-tree-production-timer-entry-baseline-discriminator/current-tree-production-timer-entry-baseline-20260630T051123Z/.
- Repeated run:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-current-tree-production-timer-entry-baseline-discriminator/current-tree-production-timer-entry-baseline-20260630T050929Z/.

## Redaction Review

Task summaries omit packet payloads, key/session material, SSH identifiers,
public-key blobs, signatures, fingerprints, peer identifiers, operator
identity, private user data, and secret-derived identifiers. Raw
lab-controller serial and TFTP artifacts remain task-owned hardware evidence
and may include local endpoint fields.

## Validation

- git status --short --branch before action: pass; main was ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- jq empty on supervisor state before lab action and lock acquisition: pass.
- Acquire hardwareTestLock before lab publication or Pi 5 power action: pass.
- Lab API candidate identity before power: pass; current-tree boot tree
  4edd4f1dad12ea06e3c45b1435f9a2d16e9c2046226d8963a0d8413a9f7226d1 with
  effective_kernel=kernel_2712.img.
- Fresh serial cursor before power: pass.
- GET /tftp/logs cursor/tail before and stable same-cursor delta after power:
  pass; both retained runs observed two selected da591740/kernel_2712.img
  serves.
- Foreground capture helper completion: pass; retained primary serial artifacts
  for both runs.
- Marker-order/classifier check: pass; readiness-v3 accepted
  rpi5-production-timer-preemption: PASS for both runs.
- Restore to accepted selected-control snapshot and confirm with lab API
  GET /status: pass; post-restore tree a0452458...
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-minimal-control-scenario-specific-reconciliation-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
