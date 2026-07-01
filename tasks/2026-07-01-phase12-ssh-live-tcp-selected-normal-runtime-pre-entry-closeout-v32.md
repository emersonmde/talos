# Phase 12 SSH Live TCP Selected Normal Runtime Pre-Entry Closeout V32

Task id: phase12-ssh-live-tcp-selected-normal-runtime-pre-entry-closeout-v32-20260701

Status: accepted after commit.

Classification: selected-normal-runtime-pre-entry-frontier-blocked-supervisor-planning.

Evidence level: accepted v26 selected-entry rust_entry/UART10 proof, accepted
v30 selected normal-runtime marker-missing closeout, accepted v31 pre-entry
discriminator repair, accepted v32 serialized Pi 5 hardware preflight,
task-owned JSON evidence, docs build, and diff checks. No hardware action, lab
publication, boot snapshot mutation, Pi 5 power cycle, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, fake/kernel-backed command expansion, broad
shell work, or phase transition was performed.

## Goal

Reconcile the v32 selected normal-runtime pre-entry result against v26, v30,
and v31 without shrinking acceptance toward packet-I/O or OpenSSH shims.

## Scope Performed

- Promoted this ready no-hardware closeout after
  phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-entry-preflight-v32-20260701
  accepted blocked-selected-normal-runtime-rust-entry-missing and selected this
  exact task.
- Compared v32 against the accepted v26 rust_entry/UART10 selected-entry
  proof, the v30 selected normal-runtime marker-missing boundary, and the v31
  repair/discriminator contract.
- Preserved the decisive v32 facts: the selected v31 normal-runtime pre-entry
  archive served da591740/kernel_2712.img twice at 152,144 bytes in the primary
  run and twice again in the unchanged candidate rerun; the known-good
  production-timer control retained rpi5-production-timer-preemption: PASS on
  the same capture path; the selected candidate retained zero ordered
  pre-entry or normal-runtime markers, including TALOS: asm_start,
  TALOS: asm_pre_rust_entry, and TALOS: rust_entry.
- Stopped before packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, or phase transition.

## Terminal Classification

selected-normal-runtime-pre-entry-frontier-blocked-supervisor-planning.

The v26 selected-entry proof remains valid for the earlier 45,400-byte
rust_entry UART10 marker-loop image: selected Image execution reached
rust_entry and the UART10 early output path. The v30/v32 hardware evidence
proves the later 152,144-byte selected normal-runtime feature image is served
by TFTP. v31 added assembly pre-entry provenance to that same feature image,
but v32 retained no TALOS: asm_start, TALOS: asm_pre_rust_entry,
TALOS: rust_entry, or later ordered marker from the selected image.

First missing fact: selected rpi5_ssh_service_smoltcp_runtime_ready Image is
served by TFTP in the primary run and unchanged rerun, but Pi 5 serial retains
no TALOS: asm_start from that selected normal-runtime image.

selected_next_task: null.

planningNeeded: true.

planningReason: v32 reconciles the selected normal-runtime pre-entry frontier
as marker-missing before TALOS: asm_start. No queued successor has refreshed
dependencies for the next bounded repair or discriminator, so the supervisor
must plan before packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, fake command expansion, broad
shell work, or phase transition.

## Findings

- fixed: reconciled v32 against v26, v30, and v31. The accepted
  rust_entry/UART10 proof applies to the earlier selected-entry marker-loop
  image, while v32 proves the 152,144-byte selected normal-runtime pre-entry
  image is served but does not retain TALOS: asm_start.
- fixed: accepted the current selected normal-runtime feature boundary as
  marker-missing before the first assembly pre-entry provenance marker, not
  merely before rust_entry, kernel_main, or the runtime-ready marker.
- not-an-issue: no additional hardware action is needed for closeout because
  v32 already captured selected-byte primary service, known-good serial
  visibility, unchanged candidate rerun, final pre-restore identity, restore
  proof, and redaction review.
- deferred: the cause of the 152,144-byte selected normal-runtime image failing
  to retain TALOS: asm_start requires a supervisor-planned bounded repair or
  discriminator; this closeout does not authorize a phase transition or
  packet-I/O/OpenSSH work.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as mechanically unblocked successors.

## Evidence Map

- Closeout evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-entry-closeout-v32/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-entry-closeout-v32/classification.json.
- Accepted v26 selected-entry closeout:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-entry-boot-path-closeout-v26.md.
- Accepted v30 selected runtime phase-marker closeout:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-runtime-phase-marker-closeout-v30.md.
- Accepted v31 selected normal-runtime pre-entry reconciliation:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-entry-reconciliation-v31.md.
- Accepted v32 Pi 5 preflight:
  tasks/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-entry-preflight-v32.md.

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
