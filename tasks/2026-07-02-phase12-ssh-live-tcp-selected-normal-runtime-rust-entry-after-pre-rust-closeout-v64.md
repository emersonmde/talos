# Phase 12 SSH Live TCP Selected Normal Runtime Rust Entry After Pre-Rust Closeout V64

Task id: phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-after-pre-rust-closeout-v64-20260702

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-rust-entry-frontier-proved.

Evidence level: accepted v63 static discriminator contract, accepted v64
serialized Pi 5 preflight evidence, selected post-power identity, selected
TFTP byte service, selected final pre-restore identity, serial marker-family
summary, restore proof, task-owned JSON evidence, docs build, and diff checks.
No hardware action, lab publication, boot snapshot mutation, Pi 5 power cycle,
BootInfo/target-init/exceptions/kernel_main proof, route-start/runtime-ready
proof, packet-I/O implementation, OpenSSH/generated-root retry, remote receipt,
compatibility claim, service success claim, ssh-ready=true, fake/kernel-backed
command expansion, broad shell work, or phase transition was performed by this
closeout.

## Goal

Reconcile the v64 selected normal-runtime rust_entry Pi 5 evidence and decide
whether post-rust-entry source/static reconciliation, a blocked boundary, or
supervisor planning is next.

## Scope Performed

- Promoted this queued no-hardware closeout after
  phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-after-pre-rust-preflight-v64-20260702
  accepted selected-normal-runtime-rust-entry-marker-retained and selected this
  exact task.
- Compared the accepted v64 result against the accepted v63 rust_entry
  discriminator contract and the v64 Pi 5 preflight evidence.
- Preserved the decisive v64 facts: selected post-power identity remained
  staged, same-window TFTP served da591740/kernel_2712.img twice at the
  selected 152,816-byte size, final pre-restore identity remained selected, the
  lab restored to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z, and the
  serial marker family retained TALOS: rust_entry 208 times.
- Stopped before BootInfo parsing, target init, exceptions, kernel_main,
  route-start/runtime-ready claims, packet-I/O, OpenSSH/generated-root retry,
  remote receipt, compatibility/service readiness, ssh-ready=true, fake command
  expansion, broad shell work, hardware action, or phase transition.

## Terminal Classification

selected-normal-runtime-rust-entry-frontier-proved.

v63 defined the rust_entry discriminator so v64 could separate selected
pre-rust proof from entry into Rust code and later normal-runtime milestones.
v64 resolved the staging and TFTP parts decisively: the selected tree
d0a5132b630258a98de56fa7e9c0eb9d1cdb41358b68e91321384461a835b6b2 stayed
staged after power, TFTP served selected da591740/kernel_2712.img at 152,816
bytes in-window, and final pre-restore identity remained selected.

The selected candidate did reach rust_entry on Pi 5: TALOS: rust_entry was
retained 208 times in the fresh serial window. BootInfo parsing, target init,
exceptions, kernel_main, route-start, runtime-blocked, and runtime-ready are
not accepted because the same window retained zero occurrences of the later
marker family.

The first missing fact is now after TALOS: rust_entry and before the next
explicit post-rust-entry normal-runtime boundary. This closeout does not select
BootInfo parsing proof, target-init proof, exceptions proof, kernel_main proof,
route-start/runtime-ready proof, packet-I/O, OpenSSH/generated-root retry,
remote receipt, compatibility/service readiness, ssh-ready=true, fake command
expansion, broad shell work, or a phase transition.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-post-rust-entry-continuation-reconciliation-v65-20260702.

planningNeeded: false.

## Findings

- fixed: reconciled v64 against the accepted v63 rust_entry discriminator
  contract and Pi 5 preflight evidence. The current selected normal-runtime
  frontier is selected rust_entry, not merely asm_pre_rust_entry.
- fixed: preserved the evidence boundary that selected post-power identity,
  selected same-window TFTP service, selected final pre-restore identity,
  marker-family serial observation, and restore proof are no longer missing
  facts for this branch.
- fixed: selected the queued v65 post-rust-entry continuation reconciliation
  because v64 proved the exact rust_entry frontier and v65 dependencies are
  now mechanically satisfiable.
- not-an-issue: known-good control and candidate rerun were not required by
  v64 because the first selected candidate identity/TFTP/serial/restore
  evidence was decisive, not inconclusive.
- deferred: BootInfo parsing, target init, exceptions, kernel_main,
  route-start, runtime-blocked, runtime-ready, packet-I/O, OpenSSH
  compatibility, service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition remain unproved.
- removed: BootInfo/target-init/exceptions/kernel_main proof,
  route-start/runtime-ready proof, packet-I/O, OpenSSH/generated-root retry,
  remote receipt, compatibility/service readiness, ssh-ready=true, fake command
  expansion, broad shell work, hardware action, and phase transition as
  immediate successors.

## Evidence Map

- Closeout evidence map:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-after-pre-rust-closeout-v64/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-after-pre-rust-closeout-v64/classification.json.
- Reconciliation summary:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-after-pre-rust-closeout-v64/static/reconciliation-summary.md.
- Accepted v63 rust_entry reconciliation:
  tasks/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-after-pre-rust-reconciliation-v63.md.
- Accepted v64 Pi 5 rust_entry preflight:
  tasks/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-after-pre-rust-preflight-v64.md.

## Redaction Review

This closeout retained no raw serial text, raw TFTP peer/log-line fields,
packet payloads, SSH/session/key material, boot artifact bytes, private user
data, stable secret-derived identifiers, public-key blobs, signatures,
fingerprints, operator identities, or unnecessary hardware data. It references
task-owned hardware evidence retained by the accepted predecessor.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before closeout promotion.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-post-rust-entry-continuation-reconciliation-v65-20260702.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
