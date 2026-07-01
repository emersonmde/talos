# Phase 12 SSH Live TCP Selected Runtime Phase Marker Closeout V30

Task id: phase12-ssh-live-tcp-selected-runtime-phase-marker-closeout-v30-20260701

Status: accepted after commit.

Classification: selected-runtime-phase-marker-frontier-blocked-supervisor-planning.

Evidence level: accepted v26 selected-entry closeout, v27 selected
normal-runtime contract, v28 selected normal-runtime continuation preflight and
closeout, v29 marker-family reconciliation, v30 serialized Pi 5 hardware
preflight evidence, task-owned JSON evidence, docs build, and diff checks. No
hardware action, lab publication, boot snapshot mutation, Pi 5 power cycle,
packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility claim,
service success claim, ssh-ready=true, fake/kernel-backed command expansion,
broad shell work, or phase transition was performed.

## Goal

Reconcile the v30 selected normal-runtime early-phase marker result against the
v26 rust_entry/UART10 proof, the v27 runtime contract, and the v28/v29
marker-missing chain without shrinking acceptance toward packet-I/O or OpenSSH
shims.

## Scope Performed

- Promoted this queued closeout after
  phase12-ssh-live-tcp-pi5-selected-runtime-phase-marker-preflight-v30-20260701
  accepted blocked-selected-runtime-phase-marker-missing and selected this exact
  closeout.
- Compared v30 against the accepted v26 selected-entry rust_entry/UART10 proof,
  the v27 selected normal-runtime contract, the v28 marker-missing hardware
  result and closeout, and the v29 marker-family evidence repair.
- Preserved the decisive v30 facts: the selected
  rpi5_ssh_service_smoltcp_runtime_ready archive served
  da591740/kernel_2712.img twice at 152,144 bytes in the primary run and twice
  again in the unchanged candidate rerun; the known-good control retained
  rpi5-production-timer-preemption: PASS on the same capture path; the selected
  candidate retained zero ordered normal-runtime markers, including
  TALOS: rust_entry.
- Stopped before packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, or phase transition.

## Terminal Classification

selected-runtime-phase-marker-frontier-blocked-supervisor-planning.

The selected-entry v26 proof remains valid for the earlier 45,400-byte
rust_entry UART10 marker-loop image: selected Image execution reached
rust_entry and the UART10 early output path. The v30 hardware evidence proves a
different and later selected normal-runtime image is served by TFTP at
152,144 bytes, but the Pi 5 serial window retains no ordered normal-runtime
marker family member from that image.

First missing fact: selected rpi5_ssh_service_smoltcp_runtime_ready Image is
served by TFTP, but Pi 5 serial retains no ordered normal-runtime marker family
member; the first missing marker is TALOS: rust_entry.

selected_next_task: null.

planningNeeded: true.

planningReason: v30 reconciles the selected normal-runtime early-phase marker
frontier as marker-missing before TALOS: rust_entry. No queued successor has
refreshed dependencies for the next smallest feature repair or discriminator,
so the supervisor must plan before packet-I/O, OpenSSH/generated-root retry,
remote receipt, compatibility/service readiness, ssh-ready=true, fake command
expansion, broad shell work, or phase transition.

## Findings

- fixed: reconciled v30 against v26, v27, v28, and v29. The accepted
  rust_entry/UART10 proof applies to the earlier 45,400-byte rust_entry
  marker-loop image, while v30 proves the 152,144-byte selected normal-runtime
  image is served but does not retain TALOS: rust_entry.
- fixed: accepted the current selected normal-runtime boundary as marker-missing
  before TALOS: rust_entry, not merely before kernel_main or the final
  runtime-ready marker.
- not-an-issue: no additional hardware action is needed for closeout because
  v30 already captured selected-byte primary service, known-good serial
  visibility, unchanged candidate rerun, final pre-restore identity, restore
  proof, and redaction review.
- deferred: the cause of the 152,144-byte selected normal-runtime image failing
  to retain TALOS: rust_entry requires a supervisor-planned bounded repair or
  discriminator; this closeout does not authorize a phase transition or
  packet-I/O/OpenSSH work.
- deferred: scripts/rpi5-capture-invariant-proof-bundle.sh final summary
  generation has a known jq $marker_family argument bug from v30. Task-owned
  summaries are adequate for this closeout, but the helper should be repaired
  before future runs depend on that generated summary.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as mechanically unblocked successors.

## Evidence Map

- Closeout evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-runtime-phase-marker-closeout-v30/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-runtime-phase-marker-closeout-v30/classification.json.
- Accepted v26 selected-entry closeout:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-entry-boot-path-closeout-v26.md.
- Accepted v27 selected normal-runtime contract:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-runtime-continuation-contract-v27.md.
- Accepted v28 selected normal-runtime preflight and closeout:
  tasks/2026-07-01-phase12-ssh-live-tcp-pi5-selected-runtime-continuation-preflight-v28.md and
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-runtime-continuation-closeout-v28.md.
- Accepted v29 marker-family reconciliation:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-runtime-phase-marker-reconciliation-v29.md.
- Accepted v30 Pi 5 preflight:
  tasks/2026-07-01-phase12-ssh-live-tcp-pi5-selected-runtime-phase-marker-preflight-v30.md.

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
