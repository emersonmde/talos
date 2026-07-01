# Phase 12 SSH Live TCP Selected Runtime Continuation Closeout V28

Task id: phase12-ssh-live-tcp-selected-runtime-continuation-closeout-v28-20260701

Status: accepted after commit.

Classification: selected-runtime-continuation-frontier-blocked-supervisor-planning.

Evidence level: accepted v26 selected-entry closeout, v27 selected normal-runtime
contract, v28 serialized Pi 5 hardware preflight evidence, task-owned JSON
evidence, docs build, and diff checks. No hardware action, lab publication,
boot snapshot mutation, Pi 5 power cycle, packet-I/O, OpenSSH/generated-root
retry, remote receipt, compatibility claim, service success claim,
ssh-ready=true, fake/kernel-backed command expansion, broad shell work, or phase
transition was performed.

## Goal

Reconcile the v28 selected normal-runtime continuation result against the v26
selected-entry boundary and the v27 runtime contract without shrinking
acceptance toward packet-I/O or OpenSSH shims.

## Scope Performed

- Promoted this queued closeout after
  phase12-ssh-live-tcp-pi5-selected-runtime-continuation-preflight-v28-20260701
  accepted blocked-selected-runtime-continuation-marker-missing and selected
  this exact closeout.
- Compared v28 against the accepted v26 rust_entry/UART10 selected-entry proof
  and the v27 normal-runtime continuation contract.
- Preserved the decisive v28 facts: selected
  rpi5_ssh_service_smoltcp_runtime_ready served da591740/kernel_2712.img twice
  at 152,144 bytes in the primary run and twice again in the unchanged
  candidate rerun, the serial path was proved by a known-good control retaining
  rpi5-production-timer-preemption: PASS, and the selected candidate retained
  no TALOS: kernel_main or contracted ssh-service-smoltcp runtime marker.
- Stopped before packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, or phase transition.

## Terminal Classification

selected-runtime-continuation-frontier-blocked-supervisor-planning.

The selected Image is proved through v26 to reach rust_entry and the UART10
early output path, and v28 proves the v27 normal-runtime continuation Image is
served by TFTP. The next boundary remains blocked before kernel_main and the
contracted ssh-service-smoltcp runtime marker: v28 retained no TALOS:
kernel_main, no TALOS: ssh-service-smoltcp-runtime-route-start, and no TALOS:
ssh-service-smoltcp-runtime-ready marker after the primary run, known-good
control, and unchanged candidate rerun.

First missing fact: selected rpi5_ssh_service_smoltcp_runtime_ready Image is
served by TFTP, but Pi 5 serial does not reach kernel_main or the contracted
ssh-service-smoltcp runtime marker.

selected_next_task: null.

planningNeeded: true.

planningReason: v28 reconciles the selected normal-runtime continuation frontier
as marker-missing before kernel_main. No queued successor has refreshed
dependencies for the next smallest feature step, so the supervisor must plan
before packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, fake command expansion, broad
shell work, or phase transition.

## Findings

- fixed: reconciled v28 against v26 and v27, preserving that selected Image
  execution was previously proved to rust_entry/UART10 while the selected
  normal-runtime continuation Image is served by TFTP.
- fixed: accepted the current missing boundary as normal runtime continuation
  before kernel_main and the contracted ssh-service-smoltcp runtime marker.
- not-an-issue: v28 selected-byte service, known-good control marker visibility,
  unchanged candidate rerun, final restore proof, and redaction review were
  sufficient for closeout without another hardware action.
- deferred: kernel_main, smoltcp runtime route start, ssh-service-smoltcp
  runtime readiness, packet I/O, remote receipt, compatibility/service
  readiness, and ssh-ready=true remain unproved and require refreshed
  supervisor planning.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as mechanically unblocked successors.

## Evidence Map

- Closeout evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-runtime-continuation-closeout-v28/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-runtime-continuation-closeout-v28/classification.json.
- Accepted v26 selected-entry closeout:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-entry-boot-path-closeout-v26.md.
- Accepted v27 selected normal-runtime continuation contract:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-runtime-continuation-contract-v27.md.
- Accepted v28 Pi 5 preflight:
  tasks/2026-07-01-phase12-ssh-live-tcp-pi5-selected-runtime-continuation-preflight-v28.md.

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

selected_next_task: null.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
