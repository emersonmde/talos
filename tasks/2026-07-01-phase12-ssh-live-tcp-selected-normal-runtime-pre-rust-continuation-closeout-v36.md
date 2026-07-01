# Phase 12 SSH Live TCP Selected Normal Runtime Pre-Rust Continuation Closeout V36

Task id: phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-continuation-closeout-v36-20260701

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-pre-rust-frontier-proved-supervisor-planning.

Evidence level: accepted v34 selected normal-runtime entry proof, accepted v35
static pre-rust discriminator contract, accepted v36 serialized Pi 5 hardware
preflight evidence, task-owned JSON evidence, docs build, and diff checks. No
hardware action, lab publication, boot snapshot mutation, Pi 5 power cycle,
packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility claim,
service success claim, ssh-ready=true, fake/kernel-backed command expansion,
broad shell work, or phase transition was performed.

## Goal

Reconcile the v36 selected normal-runtime pre-rust continuation result against
v34 and v35 without shrinking acceptance toward packet-I/O or OpenSSH shims.

## Scope Performed

- Promoted this queued no-hardware closeout after
  phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-rust-continuation-preflight-v36-20260701
  accepted selected-normal-runtime-pre-rust-marker-retained and selected this
  exact closeout.
- Compared v36 against the accepted v34 selected normal-runtime assembly-entry
  proof and the v35 selected normal-runtime pre-rust discriminator contract.
- Preserved the decisive v36 facts: the unchanged candidate rerun served
  da591740/kernel_2712.img twice at 152,144 bytes, retained final
  pre-restore identity on selected tree
  28e048845ae76bc90c6227959536e079d007e7d1e71a17122ddc1011cb42d345, retained
  TALOS: asm_pre_rust_entry 542 times, and restored to the named baseline.
- Stopped before packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, hardware action, or phase transition.

## Terminal Classification

selected-normal-runtime-pre-rust-frontier-proved-supervisor-planning.

v34 proved the selected 152,144-byte normal-runtime entry-loop archive reaches
TALOS: asm_start on Pi 5. v35 then created a separate selected normal-runtime
pre-rust loop archive that keeps the selected service route and parks at
TALOS: asm_pre_rust_entry after CPACR setup, BSS clear, and stack setup but
before rust_entry. v36 proves that exact selected 152,144-byte archive is served
by TFTP on Pi 5 and reaches TALOS: asm_pre_rust_entry in the authoritative
serial hardware summary.

First unresolved continuation fact: the selected 152,144-byte normal-runtime
feature archive has not yet been proved to enter rust_entry or later
normal-runtime milestones, including BootInfo parsing, target init, exceptions,
kernel_main, packet-I/O, remote receipt, compatibility/service readiness, or
OpenSSH.

selected_next_task: null.

planningNeeded: true.

planningReason: v36 closes the selected normal-runtime pre-rust continuation
frontier as proved through TALOS: asm_pre_rust_entry for the 152,144-byte
selected feature archive, but no queued successor has refreshed dependencies
for the next bounded continuation step. The supervisor must plan before
rust_entry/later repair or discriminator work, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility/service readiness,
ssh-ready=true, fake command expansion, broad shell work, hardware action, or
phase transition.

## Findings

- fixed: reconciled v36 against v34 and v35. v34 proves selected assembly entry
  for the 152,144-byte archive class; v36 newly proves the same selected
  normal-runtime pre-rust archive reaches TALOS: asm_pre_rust_entry on Pi 5.
- fixed: advanced the current selected normal-runtime frontier from
  selected-byte/asm_start-retained to selected-byte/asm_pre_rust_entry-retained
  after assembly setup.
- fixed: preserved v36's control disposition: the primary helper window is not
  terminal evidence because it was contaminated before restore; the unchanged
  candidate rerun is the decisive selected-byte hardware evidence.
- not-an-issue: no additional hardware action is needed for closeout because
  v36 already captured selected-byte service, marker-family retention, final
  pre-restore identity, restore proof, hardware lock release, and redaction
  review.
- deferred: the next feature step needs supervisor planning around continuation
  into rust_entry and later normal-runtime milestones.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, hardware action, and phase transition as mechanically
  unblocked successors.

## Evidence Map

- Closeout evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-continuation-closeout-v36/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-continuation-closeout-v36/classification.json.
- Accepted v34 selected normal-runtime entry closeout:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-entry-repair-closeout-v34.md.
- Accepted v35 static pre-rust reconciliation:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-continuation-reconciliation-v35.md.
- Accepted v36 Pi 5 pre-rust preflight:
  tasks/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-rust-continuation-preflight-v36.md.

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
