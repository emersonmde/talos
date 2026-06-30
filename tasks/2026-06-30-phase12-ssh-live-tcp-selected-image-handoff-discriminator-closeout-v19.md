# Phase 12 SSH Live TCP Selected-Image Handoff Discriminator Closeout V19

Task id: phase12-ssh-live-tcp-selected-image-handoff-discriminator-closeout-v19-20260630

Status: accepted after commit.

Classification: selected-image-handoff-frontier-reconciled-minimal-entry-route-repair-ready.

Evidence level: accepted v19 static/source reconciliation inspection, accepted
serialized Pi 5 handoff discriminator task/evidence inspection, task-owned JSON
evidence, docs build, and diff checks. No code implementation, hardware action,
lab publication, boot snapshot mutation, Pi 5 power cycle, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, fake/kernel-backed command expansion, broad
shell work, or phase transition was performed.

## Goal

Reconcile the accepted v19 selected-image handoff discriminator result and
select the next bounded source/static step without skipping into packet I/O,
OpenSSH, service readiness, or a phase transition.

## Scope Performed

- Promoted this queued closeout after
  phase12-ssh-live-tcp-pi5-selected-image-handoff-discriminator-v19-20260630
  accepted selected-image-handoff-entry-reached and selected this exact task.
- Inspected the accepted v19 handoff contract reconciliation, accepted v19 Pi 5
  discriminator task, classification JSON, and evidence map.
- Preserved the decisive hardware facts: selected tree
  4edd4f1dad12ea06e3c45b1435f9a2d16e9c2046226d8963a0d8413a9f7226d1, selected
  da591740/kernel_2712.img at 104,136 bytes with SHA-256
  2343a009a14972d050ccf0fc706539163b6b5cb3ee3717b9cb6753f2ec7c2328, two
  stable selected TFTP serves, empty pre-power drain on the decisive rerun,
  firmware NETWORK output, one rpi5-production-timer-preemption: PASS marker,
  final pre-restore selected identity, and restore to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.
- Reconciled that proof as selected-image handoff-entry evidence for the full
  current-tree production-timer selected path only. It does not prove the
  earlier minimal-entry selected image route, packet I/O, OpenSSH compatibility,
  remote receipt, service readiness, or SSH shell behavior.
- Refreshed the already queued
  phase12-ssh-live-tcp-minimal-entry-route-repair-v17-20260630 dependency text
  to make the accepted v19 closeout, rather than the stale v16 closeout, the
  exact predecessor required for promotion.

## Terminal Classification

selected-image-handoff-frontier-reconciled-minimal-entry-route-repair-ready.

The first accepted downstream handoff fact is now present for a normal
current-tree selected-path control: selected bytes reached the
rpi5-production-timer-preemption: PASS marker with final identity and restore
proof. That clears the firmware/Image handoff uncertainty that blocked the
route after v16/v18, but it does not repair the separate minimal-entry path
that previously served selected bytes without retaining its minimal-entry
marker.

The next bounded worker-owned task is the already queued
phase12-ssh-live-tcp-minimal-entry-route-repair-v17-20260630, with refreshed
dependencies requiring this v19 closeout to be accepted and committed with
selected_next_task set to that exact id. That task remains no-hardware until it
selects a later concrete hardware preflight with exact dependencies.

Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, broad shell work, and phase
transition remain blocked. This closeout does not authorize them.

selected_next_task:
phase12-ssh-live-tcp-minimal-entry-route-repair-v17-20260630.

planningNeeded: false.

## Findings

- fixed: reconciled the v19 PASS marker with selected-byte/final-identity and
  restore proof as accepted selected-image handoff-entry evidence for the
  current-tree production-timer selected path.
- fixed: selected the already queued minimal-entry route repair as the next
  bounded source/static task, with refreshed dependencies pointing at this v19
  closeout.
- not-an-issue: TALOS: kernel_main was absent from the retained direct-read
  window, because the accepted v19 contract made the production-timer PASS
  marker decisive when present.
- deferred: the minimal-entry route still needs a bounded no-hardware
  source/helper/archive repair or precise first-missing-fact classification.
- removed: immediate packet-I/O, OpenSSH/generated-root retry, remote receipt,
  service-readiness claims, broad shell work, and phase transition as
  successors from this closeout.

## Evidence Map

- Closeout evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-handoff-discriminator-closeout-v19/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-handoff-discriminator-closeout-v19/classification.json.
- Accepted v19 handoff contract reconciliation:
  tasks/2026-06-30-phase12-ssh-live-tcp-selected-image-handoff-contract-reconciliation-v19.md.
- Accepted v19 Pi 5 handoff discriminator:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-handoff-discriminator-v19.md.
- Accepted v19 Pi 5 discriminator classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-handoff-discriminator-v19/classification.json.

## Redaction Review

This closeout retained no raw serial text, raw TFTP peer/log-line fields,
packet payloads, SSH/session/key material, boot artifact bytes, private user
data, stable secret-derived identifiers, or unnecessary hardware data. It
references task-owned hardware evidence retained by the accepted predecessor.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before closeout promotion.
- jq empty on referenced JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-minimal-entry-route-repair-v17-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
