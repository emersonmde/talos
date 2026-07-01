# Phase 12 SSH Live TCP Selected Normal Runtime Target Init Continuation Closeout V42

Task id: phase12-ssh-live-tcp-selected-normal-runtime-target-init-continuation-closeout-v42-20260701

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-target-init-frontier-proved.

Evidence level: accepted v34 selected normal-runtime assembly-entry proof,
accepted v36 selected normal-runtime pre-rust proof, accepted v38 selected
normal-runtime rust_entry proof, accepted v40 selected normal-runtime BootInfo
proof, accepted v41 static target-init discriminator contract, accepted v42
serialized Pi 5 hardware preflight evidence, task-owned JSON evidence, docs
build, and diff checks. No hardware action, lab publication, boot snapshot
mutation, Pi 5 power cycle, packet-I/O, OpenSSH/generated-root retry, remote
receipt, compatibility claim, service success claim, ssh-ready=true,
fake/kernel-backed command expansion, broad shell work, or phase transition
was performed.

## Goal

Reconcile the v42 selected normal-runtime target-init continuation result
without shrinking acceptance toward packet-I/O or OpenSSH shims.

## Scope Performed

- Promoted this queued no-hardware closeout after
  phase12-ssh-live-tcp-pi5-selected-normal-runtime-target-init-continuation-preflight-v42-20260701
  accepted selected-normal-runtime-target-init-marker-retained and selected
  this exact closeout.
- Compared v42 against the accepted v34 selected normal-runtime assembly-entry
  proof, the v36 selected normal-runtime pre-rust proof, the v38 selected
  normal-runtime rust_entry proof, the v40 selected normal-runtime BootInfo
  proof, and the v41 selected normal-runtime target-init discriminator
  contract.
- Preserved the decisive v42 facts: the selected v41 target-init archive served
  da591740/kernel_2712.img twice at 152,880 bytes, retained final pre-restore
  identity on selected tree
  478473f46d7dd2d64a42a9fd8f9e56e68de691a1237cb85108ef972f31045305, retained
  TALOS: target init 3,006 times, and restored to the named baseline.
- Stopped before exceptions ready, kernel_main, route-start, runtime-ready,
  packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, hardware action, or phase transition.

## Terminal Classification

selected-normal-runtime-target-init-frontier-proved.

v34 proves the selected normal-runtime archive class can enter Talos assembly
and retain TALOS: asm_start on Pi 5. v36 proves the next assembly setup
boundary: TALOS: asm_pre_rust_entry after CPACR setup, BSS clear, and stack
setup but before Rust. v38 proves the selected normal-runtime archive reaches
TALOS: rust_entry after Rust begins. v40 proves the selected normal-runtime
archive reaches TALOS: boot info parsed after BootInfo parsing. v41 defines the
next selected normal-runtime discriminator by looping on TALOS: target init
only after target::init(&boot_info) returns. v42 proves that exact selected
152,880-byte target-init archive is served by TFTP on Pi 5 and reaches TALOS:
target init in the authoritative serial hardware summary.

First unresolved continuation facts: exceptions ready, kernel_main,
route-start, runtime-ready, packet-I/O, remote receipt,
compatibility/service readiness, OpenSSH, ssh-ready=true, fake command
expansion, broad shell work, and phase transition.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-exceptions-continuation-reconciliation-v43-20260701.

planningNeeded: false.

## Findings

- fixed: reconciled v42 against v34, v36, v38, v40, and v41. The accepted
  selected normal-runtime frontier now reaches target init on Pi 5 with
  selected-byte TFTP service and restore proof.
- fixed: advanced the current selected normal-runtime frontier from
  selected-byte/boot-info-parsed-retained to
  selected-byte/target-init-retained.
- fixed: preserved v42's evidence boundary: the target-init marker-loop proves
  target initialization only and intentionally withholds exceptions,
  kernel_main, route-start, runtime-ready, packet-I/O, service readiness, and
  OpenSSH claims.
- not-an-issue: no additional hardware action is needed for closeout because
  v42 already captured selected-byte service, marker retention, final
  pre-restore identity, restore proof, hardware lock release, and redaction
  review.
- deferred: exceptions ready, kernel_main, route-start, runtime-ready,
  packet-I/O, OpenSSH compatibility, service readiness, ssh-ready=true, fake
  command expansion, broad shell work, and phase transition remain unproved.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, hardware action, and phase transition as immediate
  successors.

## Evidence Map

- Closeout evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-target-init-continuation-closeout-v42/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-target-init-continuation-closeout-v42/classification.json.
- Accepted v34 selected normal-runtime entry closeout:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-entry-repair-closeout-v34.md.
- Accepted v36 selected normal-runtime pre-rust closeout:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-continuation-closeout-v36.md.
- Accepted v38 selected normal-runtime rust_entry closeout:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-continuation-closeout-v38.md.
- Accepted v40 selected normal-runtime BootInfo closeout:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-continuation-closeout-v40.md.
- Accepted v41 static target-init reconciliation:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-target-init-continuation-reconciliation-v41.md.
- Accepted v42 Pi 5 target-init preflight:
  tasks/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-target-init-continuation-preflight-v42.md.

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
phase12-ssh-live-tcp-selected-normal-runtime-exceptions-continuation-reconciliation-v43-20260701.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
