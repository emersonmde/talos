# Phase 12 SSH Live TCP Selected-Path Known-Good Entry Baseline Discriminator

Task id: phase12-ssh-live-tcp-selected-path-known-good-entry-baseline-discriminator-20260630

Status: accepted after commit.

Classification: known-good-entry-baseline-passes.

Evidence level: serialized Pi 5 hardware boot/output, lab-controller API
identity, fresh serial cursor/completeness diagnostics, stable same-cursor TFTP
delta before restore, marker capture checker, selected-control readiness
classifier, restore proof, task-owned JSON evidence, docs build, and diff
checks. No live TCP candidate retry, packet-I/O discriminator,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, broad shell work, or phase transition was
performed.

## Goal

Check whether the selected a0452458... known-good/control snapshot still emits
an expected Talos-side entry marker under the same selected-path capture
contract after the minimal-control selected-path blocker.

## Scope Performed

- Promoted the queued discriminator only after
  phase12-ssh-live-tcp-selected-path-entry-control-source-reconciliation-20260630
  accepted serial-capture-control-required and selected this exact task.
- Acquired hardwareTestLock before lab restore, cursor capture, power-cycle,
  serial/TFTP observation, final identity sampling, and restore proof.
- Restored phase12-ssh-v10-openssh-clean-pre-20260624T074100Z, confirmed tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective_kernel=kernel_2712.img, and ran exactly one selected-control power
  cycle.
- Configured the marker checker for
  rpi5-production-timer-preemption: PASS, retained the post-power serial
  window, stable same-cursor TFTP delta, final pre-restore identity, and
  readiness-v3 classification.
- Restored the same selected control snapshot before releasing the hardware
  lock.

## Terminal Classification

known-good-entry-baseline-passes.

The selected known-good/control baseline emitted the accepted entry/readiness
marker under the selected-path capture contract. The run retained an empty
pre-power serial drain, firmware NETWORK output, 6,580 post-power serial bytes,
one rpi5-production-timer-preemption: PASS marker, stable pre/final selected
identity on a0452458..., two selected da591740/kernel_2712.img TFTP serves at
104,136 bytes, and restore proof returning to the same selected snapshot.

TALOS: kernel_main was absent from the retained serial window, but remains
metadata-only for the selected v10 known-good control when the downstream
production-timer PASS marker appears. The result proves the missing
minimal-entry-control marker is not a generic selected-path serial/capture
outage. It does not authorize live TCP candidate retry, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility, service success,
ssh-ready=true, broad shell work, or a phase transition. planningNeeded is set
for supervisor planning of the next bounded source/artifact or candidate-entry
boundary.

selected_next_task: null.

planningNeeded: true.

## Findings

- fixed: restored and proved the selected
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z control snapshot under
  hardwareTestLock.
- fixed: retained fresh pre-power serial drain/cursor evidence and a
  post-power selected-control serial window.
- fixed: retained stable same-cursor TFTP evidence with two 104,136-byte
  da591740/kernel_2712.img serves from the selected a0452458... tree.
- fixed: marker capture and readiness-v3 classification both accepted
  rpi5-production-timer-preemption: PASS for the selected known-good baseline.
- not-an-issue: TALOS: kernel_main was absent; the accepted v3 policy records
  it as metadata-only when the downstream PASS marker is present.
- deferred: supervisor planning is required before any live TCP candidate
  retry, packet-I/O discriminator, OpenSSH/generated-root retry, remote
  receipt, compatibility, service success, ssh-ready=true, broad shell work, or
  phase transition.
- removed: an initial partial evidence directory from a failed GET / sample was
  removed before the accepted run; the deployed lab API exposes /status rather
  than / for status in this environment.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-path-known-good-entry-baseline-discriminator/evidence-map.json.
- Run directory:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-path-known-good-entry-baseline-discriminator/selected-path-known-good-entry-baseline-20260630T021700Z/.
- Serial window:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-path-known-good-entry-baseline-discriminator/selected-path-known-good-entry-baseline-20260630T021700Z/serial-observe-window.json.
- Stable TFTP delta:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-path-known-good-entry-baseline-discriminator/selected-path-known-good-entry-baseline-20260630T021700Z/tftp-delta-stable-pre-restore.json.
- Readiness classifier:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-path-known-good-entry-baseline-discriminator/selected-path-known-good-entry-baseline-20260630T021700Z/readiness-v3-classification.json.
- Restore proof:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-path-known-good-entry-baseline-discriminator/selected-path-known-good-entry-baseline-20260630T021700Z/restore-snapshot.json,
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-path-known-good-entry-baseline-discriminator/selected-path-known-good-entry-baseline-20260630T021700Z/post-restore-status.json.

## Redaction Review

Task summaries omit packet payloads, key/session material, SSH identifiers,
public-key blobs, signatures, fingerprints, peer identifiers, operator identity,
private user data, and secret-derived identifiers. Raw lab-controller serial
and TFTP artifacts remain task-owned hardware evidence and may include local
endpoint fields.

## Validation

- git status --short --branch before action: pass; main was ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- Known-good/control snapshot identity review: pass; restored selected snapshot
  reported tree a0452458... and effective_kernel=kernel_2712.img.
- Lab API GET /status, GET /boot/files, and GET /boot/snapshots before power
  and after restore: pass.
- Fresh serial cursor/completeness diagnostics before power: pass;
  serial-drain-before-power reached empty-read-before-power.
- GET /tftp/logs tail cursor and stable same-cursor delta before restore: pass;
  stable helper retained selected da591740/kernel_2712.img fetches.
- Task-owned marker/capture checker configured for the expected known-good
  marker: pass; rpi5-production-timer-preemption: PASS observed once.
- Restore to named selected-control snapshot and confirm with lab API
  GET /status: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

selected_next_task: null.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
