# Phase 12 SSH Live TCP Selected Normal Runtime Kernel Main Continuation Closeout V48

Task id: phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-continuation-closeout-v48-20260701

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-kernel-main-frontier-proved.

Evidence level: accepted v34 selected normal-runtime assembly-entry proof,
accepted v36 selected normal-runtime pre-rust proof, accepted v38 selected
normal-runtime rust_entry proof, accepted v40 selected normal-runtime BootInfo
proof, accepted v42 selected normal-runtime target-init proof, accepted v44
selected normal-runtime exceptions-ready proof, accepted v45 static kernel_main
discriminator contract, accepted v47 hash-contract reconciliation, accepted
v48 serialized Pi 5 hardware preflight evidence, task-owned JSON evidence,
docs build, and diff checks. No hardware action, lab publication, boot snapshot
mutation, Pi 5 power cycle, packet-I/O, OpenSSH/generated-root retry, remote
receipt, compatibility claim, service success claim, ssh-ready=true,
fake/kernel-backed command expansion, broad shell work, or phase transition was
performed by this closeout.

## Goal

Reconcile the v48 selected normal-runtime kernel_main continuation result
without shrinking acceptance toward route-start, runtime-ready, packet-I/O,
OpenSSH, or service-readiness claims.

## Scope Performed

- Promoted this queued no-hardware closeout after
  phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v48-20260701
  accepted selected-normal-runtime-kernel-main-marker-retained and selected
  this exact closeout.
- Compared v48 against the accepted v34 selected normal-runtime assembly-entry
  proof, v36 pre-rust proof, v38 rust_entry proof, v40 BootInfo proof, v42
  target-init proof, v44 exceptions-ready proof, v45 kernel_main static
  discriminator contract, and v47 hash-contract reconciliation.
- Preserved the decisive v48 facts: the selected v47/v45 kernel_main archive
  served da591740/kernel_2712.img twice at 152,896 bytes, retained final
  pre-restore identity on selected tree
  9d2f354810e8f445705dd083c8876f47bd25fa5f1aec52762c5af98662fdf60a,
  retained TALOS: kernel_main capture-nonce=runtime-marker-route-static 1,794
  times, and restored to the named baseline.
- Stopped before route-start, runtime-ready, packet-I/O, OpenSSH/generated-root
  retry, remote receipt, compatibility/service readiness, ssh-ready=true, fake
  command expansion, broad shell work, hardware action, or phase transition.

## Terminal Classification

selected-normal-runtime-kernel-main-frontier-proved.

v34 proves the selected normal-runtime archive class can enter Talos assembly
and retain TALOS: asm_start on Pi 5. v36 proves the next assembly setup
boundary: TALOS: asm_pre_rust_entry after CPACR setup, BSS clear, and stack
setup but before Rust. v38 proves the selected normal-runtime archive reaches
TALOS: rust_entry after Rust begins. v40 proves the selected normal-runtime
archive reaches TALOS: boot info parsed after BootInfo parsing. v42 proves the
selected normal-runtime archive reaches TALOS: target init after
target::init(&boot_info). v44 proves the selected normal-runtime archive
reaches TALOS: exceptions ready after arch::aarch64::exceptions::init()
returns. v45 defines the next selected normal-runtime discriminator by looping
on TALOS: kernel_main capture-nonce=runtime-marker-route-static only after
entering boot::rpi5::kernel_main. v47 reconciles the authoritative archive and
selected-kernel hash contract. v48 proves that exact selected 152,896-byte
kernel_main archive is served by TFTP on Pi 5 and reaches TALOS: kernel_main
in the authoritative serial hardware summary.

First unresolved continuation facts: route-start, runtime-ready, packet-I/O,
remote receipt, compatibility/service readiness, OpenSSH, ssh-ready=true, fake
command expansion, broad shell work, and phase transition.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-route-start-continuation-reconciliation-v49-20260701.

planningNeeded: false.

## Findings

- fixed: reconciled v48 against v34, v36, v38, v40, v42, v44, v45, and v47.
  The accepted selected normal-runtime frontier now reaches kernel_main on Pi 5
  with selected-byte TFTP service and restore proof.
- fixed: advanced the current selected normal-runtime frontier from
  selected-byte/exceptions-ready-retained to
  selected-byte/kernel_main-retained.
- fixed: preserved v48's evidence boundary: the kernel_main marker-loop proves
  entry into boot::rpi5::kernel_main only and intentionally withholds
  route-start, runtime-ready, packet-I/O, service readiness, and OpenSSH
  claims.
- not-an-issue: no additional hardware action is needed for closeout because
  v48 already captured selected-byte service, marker retention, final
  pre-restore identity, restore proof, hardware lock release, and redaction
  review.
- deferred: route-start, runtime-ready, packet-I/O, OpenSSH compatibility,
  service readiness, ssh-ready=true, fake command expansion, broad shell work,
  and phase transition remain unproved.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, hardware action, and phase transition as immediate
  successors.

## Evidence Map

- Closeout evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-continuation-closeout-v48/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-continuation-closeout-v48/classification.json.
- Reconciliation summary:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-continuation-closeout-v48/static/reconciliation-summary.md.
- Accepted v34 selected normal-runtime entry closeout:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-entry-repair-closeout-v34.md.
- Accepted v36 selected normal-runtime pre-rust closeout:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-continuation-closeout-v36.md.
- Accepted v38 selected normal-runtime rust_entry closeout:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-continuation-closeout-v38.md.
- Accepted v40 selected normal-runtime BootInfo closeout:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-continuation-closeout-v40.md.
- Accepted v42 selected normal-runtime target-init closeout:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-target-init-continuation-closeout-v42.md.
- Accepted v44 selected normal-runtime exceptions closeout:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-continuation-closeout-v44.md.
- Accepted v45 static kernel_main reconciliation:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-continuation-reconciliation-v45.md.
- Accepted v47 hash-contract reconciliation:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-contract-reconciliation-v47.md.
- Accepted v48 Pi 5 kernel_main preflight:
  tasks/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v48.md.

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
phase12-ssh-live-tcp-selected-normal-runtime-route-start-continuation-reconciliation-v49-20260701.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
