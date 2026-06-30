# Phase 12 SSH Live TCP Selected-Image Entry Boundary Closeout V15

Task id: phase12-ssh-live-tcp-selected-image-entry-boundary-closeout-v15-20260630

Status: accepted after commit.

Classification: selected-image-entry-no-entry-supervisor-planning.

Evidence level: accepted v15 Pi 5 preflight task/evidence inspection,
task-owned JSON evidence, docs build, and diff checks. No code implementation,
hardware action, lab publication, boot snapshot mutation, Pi 5 power cycle,
packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility claim,
service success claim, ssh-ready=true, fake/kernel-backed command expansion,
broad shell work, or phase transition was performed.

## Goal

Reconcile the accepted v15 selected-image entry preflight without shrinking the
acceptance boundary toward a shim or authorizing blind reruns, packet-I/O, or
OpenSSH/generated-root retry.

## Scope Performed

- Promoted this queued closeout after
  phase12-ssh-live-tcp-pi5-selected-image-entry-preflight-v15-20260630
  accepted blocked-selected-image-entry and selected this exact task.
- Inspected the accepted v15 selected-image invariant reconciliation, repaired
  Pi 5 preflight record, classification JSON, and task-owned evidence map.
- Preserved the decisive hardware facts: the repaired minimal entry-control
  selected image was published, da591740/kernel_2712.img was served twice with
  matching 52,848-byte counts, final pre-restore identity stayed on tree
  4380329c..., and restore to the predecessor-named baseline succeeded.
- Preserved the entry blocker: the fresh serial window retained firmware
  NETWORK output but no TALOS: asm_start, TALOS: asm_pre_rust_entry, Rust phase
  lines, kernel_main, nonce-bearing minimal-entry-control-ready marker, or run
  nonce.
- Stopped at supervisor planning because no existing queued successor is
  mechanically unblocked by blocked-selected-image-entry.

## Terminal Classification

selected-image-entry-no-entry-supervisor-planning.

The first missing fact is now the selected-image entry path for a repaired
minimal entry-control kernel image after firmware has requested and received
the selected bytes. The v15 repair pushed the marker contract below the Rust
entry path by requiring TALOS: asm_start and TALOS: asm_pre_rust_entry in the
selected artifact, and the Pi 5 preflight proved that the repaired selected
image was actually served before restore. The run therefore does not classify
as selected-image identity, TFTP capture, restore, or inconclusive evidence.

The fresh serial window still retained only firmware NETWORK output. It did
not retain TALOS: asm_start, TALOS: asm_pre_rust_entry, TALOS: rust_entry,
TALOS: boot info parsed, TALOS: target init, TALOS: exceptions ready,
TALOS: kernel_main, the nonce-bearing minimal-entry-control-ready marker, or
the run nonce. That keeps the missing boundary below packet I/O, OpenSSH,
remote receipt, compatibility, SSH service readiness, and broad shell behavior.

No queued successor is mechanically unblocked:

- phase12-ssh-live-tcp-selected-image-entry-control-contract-v15-20260630
  requires the v15 invariant reconciliation to accept
  selected-image-entry-control-contract-required and select that exact control
  contract; it instead accepted selected-image-entry-source-repair-ready.
- phase12-ssh-live-tcp-pi5-selected-image-entry-control-discriminator-v15-20260630
  requires an accepted selected-image-entry-control-contract-ready predecessor;
  no such predecessor exists.
- Packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility,
  service success, ssh-ready=true, fake/kernel-backed command expansion, broad
  shell work, and phase transition remain blocked because the repaired
  selected-image entry marker contract remains absent.

selected_next_task: null.

planningNeeded: true.

planningReason: v15 proved selected-byte service for the repaired minimal
entry-control image, final pre-restore identity, and restore, but still
observed no TALOS assembly-entry, Rust phase, kernel_main,
minimal-entry-control-ready, or nonce markers after firmware NETWORK.
Supervisor must plan the next bounded selected-image entry investigation or
discriminator before any rerun, packet-I/O, OpenSSH/generated-root retry,
remote receipt, compatibility/service readiness claim, broad shell work, or
phase transition.

## Findings

- fixed: reconciled selected identity, same-cursor TFTP byte agreement, final
  pre-restore identity, and restore proof as decisive for the v15 repaired
  minimal entry-control preflight.
- blocked: the repaired selected image still produced no retained assembly
  entry, Rust entry, kernel_main, nonce-bearing minimal-entry-control marker,
  or run nonce.
- deferred: the source/entry reason for missing selected-image entry markers
  requires supervisor planning as a new bounded task or discriminator.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt, service
  readiness, broad shell work, and phase transition as immediate successors.
- not-an-issue: hardwareTestLock remained unlocked because this closeout is
  no-hardware and relies on accepted predecessor evidence.

## Evidence Map

- Closeout evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-selected-image-entry-boundary-closeout-v15/evidence-map.json.
- Accepted v15 selected-image invariant reconciliation:
  tasks/2026-06-30-phase12-ssh-live-tcp-selected-image-entry-invariant-reconciliation-v15.md.
- Accepted v15 Pi 5 selected-image entry preflight:
  tasks/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-entry-preflight-v15.md.
- Accepted v15 Pi 5 preflight classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-entry-preflight-v15/classification.json.

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

selected_next_task: null.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
