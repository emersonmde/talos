# Phase 12 SSH Live TCP Candidate Entry Control Contract V13

Task id: phase12-ssh-live-tcp-candidate-entry-control-contract-v13-20260630

Status: accepted after commit.

Classification: candidate-entry-control-contract-ready.

Evidence level: static review of accepted v12 selected-fetch evidence,
selected-control/current-tree production-timer evidence, task-owned JSON
evidence, docs build, and diff checks. No hardware action, lab publication,
boot snapshot mutation, Pi 5 power cycle, packet-I/O, OpenSSH/generated-root
retry, remote receipt, compatibility claim, service success claim,
ssh-ready=true, fake/kernel-backed command expansion, broad shell work, or
phase transition was performed.

## Goal

Define the smallest control discriminator for the selected-fetch/no-runtime-
marker boundary after static source/artifact reconciliation found no bounded
repair.

## Scope Performed

- Promoted this queued no-hardware task after
  phase12-ssh-live-tcp-candidate-entry-boundary-source-artifact-reconciliation-v13-20260630
  accepted candidate-entry-control-contract-required and selected this exact
  task.
- Reviewed the accepted v12 runtime-marker preflight and closeout evidence,
  the v13 source/artifact reconciliation evidence, and the accepted
  current-tree production-timer control/baseline evidence.
- Defined one serialized Pi 5 control discriminator that reuses the accepted
  current-tree production-timer selected-path control while adding explicit
  phase-line classification for rust_entry, boot-info-parsed, target-init,
  exceptions-ready, and kernel_main.
- Stopped before hardware, lab publication, packet-I/O, OpenSSH/generated-root
  retry, generated-root work, broad shell work, or phase transition.

## Terminal Classification

candidate-entry-control-contract-ready.

The decisive control is a current-tree production-timer selected-path
phase-line discriminator. It uses the accepted production-timer control lineage
because that image already proved that current-tree selected bytes can enter
Talos and reach a downstream Pi 5 marker under the same selected
da591740/kernel_2712.img fetch path. The missing fact is not another selected
fetch proof; it is whether the serial/capture policy should treat early entry
phase lines as required entry evidence, metadata-only context, or a blocker
when the downstream control marker is absent.

selected_next_task:
phase12-ssh-live-tcp-pi5-candidate-entry-control-discriminator-v13-20260630.

planningNeeded: false.

## Contract

- Control source/scenario:
  rpi5_production_timer_preemption_proof from source commit
  1b428d0e4fda3fd26fe3869c7e5f43d648bba11a, materialized by the existing
  production-timer boot-tree/archive path.
- Expected selected fetch:
  da591740/kernel_2712.img, 104,136 bytes, SHA-256
  2343a009a14972d050ccf0fc706539163b6b5cb3ee3717b9cb6753f2ec7c2328.
- Expected lab identity:
  current-tree control tree
  4edd4f1dad12ea06e3c45b1435f9a2d16e9c2046226d8963a0d8413a9f7226d1 before
  restore, effective_kernel=kernel_2712.img.
- Required downstream control marker:
  rpi5-production-timer-preemption: PASS.
- Phase-line classifier inputs:
  TALOS: rust_entry, TALOS: boot info parsed, TALOS: target init,
  TALOS: exceptions ready, and TALOS: kernel_main. The hardware task must
  record each as present or absent in the fresh post-power serial window.
- Control-entry-passes:
  selected fetch identity is captured in-window, lab identity remains on the
  control tree before restore, firmware NETWORK output is present, the PASS
  marker is present, phase-line presence/absence is classified, and restore
  succeeds. If PASS is present, absent early phase lines or absent kernel_main
  are metadata-only for this control and must not by themselves block the
  control.
- blocked-control-entry:
  selected fetch identity is captured but the PASS marker is absent; preserve
  phase-line classification as the first missing entry/capture fact.
- blocked-control-identity:
  publication, final pre-restore identity, effective kernel, selected path,
  byte count, or kernel hash does not match this contract.
- blocked-control-tftp-capture:
  the repaired same-cursor pre-restore TFTP delta cannot prove selected
  da591740/kernel_2712.img fetch identity.
- blocked-restore:
  restore to the accepted selected-control snapshot cannot be proved.
- inconclusive-with-required-discriminator:
  serial, TFTP, lab identity, or helper timing evidence conflicts in a way that
  cannot be reduced without a qualitatively different discriminator.

The successor hardware task must acquire hardwareTestLock before publication,
boot snapshot mutation, restore-affecting lab operations, or Pi 5 power action.
It must capture candidate/control identity, fresh serial cursor, repaired TFTP
delta, final pre-restore identity, marker/phase-line classification, restore
proof, post-restore identity, and redaction review. The standard inconclusive
triage order remains: identity, fresh serial cursor, TFTP delta, known-good
control if capture/staging is suspect, then one bounded contracted rerun only
if required by the retained evidence.

Restore target:
phase12-ssh-v10-openssh-clean-pre-20260624T074100Z /
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

Packet-I/O, OpenSSH/generated-root retry, generated-root work, remote receipt,
compatibility, service success, ssh-ready=true, broad shell work, and phase
transition remain blocked.

## Findings

- fixed: selected exactly one successor,
  phase12-ssh-live-tcp-pi5-candidate-entry-control-discriminator-v13-20260630,
  instead of a blind candidate rerun or packet-I/O step.
- fixed: defined explicit control-entry, identity, TFTP-capture, restore, and
  inconclusive classifications for the successor hardware task.
- fixed: made early phase-line and kernel_main handling explicit for the
  current-tree production-timer control: downstream PASS makes absent phase
  lines metadata-only for that control, while absence of PASS after selected
  fetch is the control-entry blocker.
- not-an-issue: reusing the production-timer control is acceptable because it
  exercises the same selected fetch, _start/rust_entry/kernel_main path, and Pi
  5 serial/TFTP capture machinery without packet-I/O or OpenSSH.
- deferred: executing the hardware discriminator is deferred to the selected
  successor task under hardwareTestLock.
- removed: packet-I/O/OpenSSH/generated-root retry as permissible successors
  from this contract.

## Evidence Map

- Contract evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-candidate-entry-control-contract-v13/evidence-map.json.
- Accepted v13 source/artifact reconciliation:
  tasks/2026-06-30-phase12-ssh-live-tcp-candidate-entry-boundary-source-artifact-reconciliation-v13.md.
- Accepted v12 hardware preflight:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v12.md.
- Accepted current-tree production-timer Pi 5 baseline:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-current-tree-production-timer-entry-baseline-discriminator.md.

## Redaction Review

This task retained task ids, commit ids, path labels, hashes, byte counts,
marker labels, classifications, validation command results, and selected
successor metadata. It retained no raw serial text, raw TFTP peer/log-line
fields, packet payloads, SSH keys/session material, boot artifact bytes,
private user data, stable secret-derived identifiers, or unnecessary hardware
data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before promotion.
- Static review of accepted v12 evidence and selected-control/current-tree
  evidence: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-candidate-entry-control-discriminator-v13-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
