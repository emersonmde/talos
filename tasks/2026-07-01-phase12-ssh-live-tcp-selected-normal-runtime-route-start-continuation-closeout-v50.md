# Phase 12 SSH Live TCP Selected Normal Runtime Route Start Continuation Closeout V50

Task id: phase12-ssh-live-tcp-selected-normal-runtime-route-start-continuation-closeout-v50-20260701

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-route-start-frontier-proved.

Evidence level: accepted v49 static route-start discriminator contract,
accepted v50 serialized Pi 5 hardware preflight evidence, selected-byte TFTP
summary, serial hardware marker summary, restore proof, task-owned JSON
evidence, docs build, and diff checks. No hardware action, lab publication,
boot snapshot mutation, Pi 5 power cycle, packet-I/O, OpenSSH/generated-root
retry, remote receipt, compatibility claim, service success claim,
ssh-ready=true, fake/kernel-backed command expansion, broad shell work, or
phase transition was performed by this closeout.

## Goal

Reconcile the v50 selected normal-runtime route-start hardware result without
shrinking acceptance toward runtime-ready, packet-I/O, OpenSSH, service
readiness, or shell behavior.

## Scope Performed

- Promoted this ready no-hardware closeout after
  phase12-ssh-live-tcp-pi5-selected-normal-runtime-route-start-continuation-preflight-v50-20260701
  accepted selected-normal-runtime-route-start-marker-retained and selected
  this exact closeout.
- Compared the accepted route-start result against the accepted v49 static
  discriminator contract and v50 Pi 5 preflight evidence.
- Preserved the decisive v50 facts: the v49-selected route-start archive
  served da591740/kernel_2712.img twice at 152,640 bytes, retained final
  pre-restore identity on selected tree
  e1c8ce434afb82517063c9535f53d127ae220b76e2756d65b110fc808193ac63,
  retained TALOS: ssh-service-smoltcp-runtime-route-start
  capture-nonce=runtime-marker-route-static 2,326 times, and restored to the
  named baseline.
- Stopped before runtime-ready, packet-I/O, OpenSSH/generated-root retry,
  remote receipt, compatibility/service readiness, ssh-ready=true, fake command
  expansion, broad shell work, hardware action, or phase transition.

## Terminal Classification

selected-normal-runtime-route-start-frontier-proved.

v49 defines the selected normal-runtime route-start discriminator by looping on
TALOS: ssh-service-smoltcp-runtime-route-start
capture-nonce=runtime-marker-route-static only after the accepted kernel_main
frontier and before runtime-ready, packet-I/O, service success, ssh-ready, fake
command expansion, broad shell work, or phase-transition claims. v50 proves
that exact selected 152,640-byte route-start archive is served by TFTP on Pi 5
and reaches the route-start marker in the authoritative serial hardware
summary.

First unresolved continuation facts: runtime-ready, packet-I/O, remote
receipt, compatibility/service readiness, OpenSSH, ssh-ready=true, fake command
expansion, broad shell work, and phase transition.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-continuation-reconciliation-v51-20260701.

planningNeeded: false.

## Findings

- fixed: reconciled v50 against the accepted v49 route-start discriminator and
  Pi 5 preflight evidence. The accepted selected normal-runtime frontier now
  reaches the route-start marker on Pi 5 with selected-byte TFTP service and
  restore proof.
- fixed: advanced the current selected normal-runtime frontier from
  selected-byte/kernel_main-retained to selected-byte/route-start-retained.
- fixed: preserved v50's evidence boundary: the route-start marker-loop proves
  the runtime route-start boundary only and intentionally withholds
  runtime-ready, packet-I/O, service readiness, and OpenSSH claims.
- not-an-issue: no additional hardware action is needed for closeout because
  v50 already captured selected-byte service, marker retention, final
  pre-restore identity, restore proof, hardware lock release, and redaction
  review.
- deferred: runtime-ready, packet-I/O, OpenSSH compatibility, remote receipt,
  service readiness, ssh-ready=true, fake command expansion, broad shell work,
  and phase transition remain unproved.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, hardware action, and phase transition as immediate
  successors.

## Evidence Map

- Closeout evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-route-start-continuation-closeout-v50/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-route-start-continuation-closeout-v50/classification.json.
- Reconciliation summary:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-route-start-continuation-closeout-v50/static/reconciliation-summary.md.
- Accepted v49 route-start reconciliation:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-route-start-continuation-reconciliation-v49.md.
- Accepted v50 Pi 5 route-start preflight:
  tasks/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-route-start-continuation-preflight-v50.md.

## Redaction Review

This closeout retained no raw serial text, raw TFTP peer/log-line fields,
packet payloads, SSH/session/key material, boot artifact bytes, private user
data, stable secret-derived identifiers, or unnecessary hardware data. It
references task-owned hardware evidence retained by the accepted predecessor.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before closeout promotion.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-runtime-ready-continuation-reconciliation-v51-20260701.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
