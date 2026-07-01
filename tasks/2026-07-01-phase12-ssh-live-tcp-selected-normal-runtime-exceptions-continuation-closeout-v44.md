# Phase 12 SSH Live TCP Selected Normal Runtime Exceptions Continuation Closeout V44

Task id: phase12-ssh-live-tcp-selected-normal-runtime-exceptions-continuation-closeout-v44-20260701

Status: accepted after final validation and commit.

Classification: selected-normal-runtime-exceptions-frontier-proved.

Evidence level: accepted v34 selected normal-runtime assembly-entry proof,
accepted v36 selected normal-runtime pre-rust proof, accepted v38 selected
normal-runtime rust_entry proof, accepted v40 selected normal-runtime BootInfo
proof, accepted v42 selected normal-runtime target-init proof, accepted v43
static exceptions-ready discriminator contract, accepted v44 serialized Pi 5
hardware preflight evidence, task-owned JSON evidence, docs build, and diff
checks. No hardware action, lab publication, boot snapshot mutation, Pi 5
power cycle, packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility claim, service success claim, ssh-ready=true, fake/kernel-backed
command expansion, broad shell work, or phase transition was performed by this
closeout.

## Goal

Reconcile the v44 selected normal-runtime exceptions-ready continuation result
without shrinking acceptance toward kernel_main, packet-I/O, OpenSSH, or
service-readiness claims.

## Scope Performed

- Promoted this queued no-hardware closeout after
  phase12-ssh-live-tcp-pi5-selected-normal-runtime-exceptions-continuation-preflight-v44-20260701
  accepted selected-normal-runtime-exceptions-ready-marker-retained and
  selected this exact closeout.
- Compared v44 against the accepted v34 selected normal-runtime assembly-entry
  proof, the v36 selected normal-runtime pre-rust proof, the v38 selected
  normal-runtime rust_entry proof, the v40 selected normal-runtime BootInfo
  proof, the v42 selected normal-runtime target-init proof, and the v43
  selected normal-runtime exceptions-ready discriminator contract.
- Preserved the decisive v44 facts: the selected v43 exceptions-ready archive
  served da591740/kernel_2712.img twice at 152,880 bytes, retained final
  pre-restore identity on selected tree
  2c0d4152ebae130632caa5a9e8fa776704ec0336d2c54609ab00a5981328fcde,
  retained TALOS: exceptions ready 2,145 times, did not accept TALOS:
  kernel_main, and restored to the named baseline.
- Stopped before kernel_main, route-start, runtime-ready, packet-I/O,
  OpenSSH/generated-root retry, remote receipt, compatibility/service
  readiness, ssh-ready=true, fake command expansion, broad shell work,
  hardware action, or phase transition.

## Terminal Classification

selected-normal-runtime-exceptions-frontier-proved.

v34 proves the selected normal-runtime archive class can enter Talos assembly
and retain TALOS: asm_start on Pi 5. v36 proves the next assembly setup
boundary: TALOS: asm_pre_rust_entry after CPACR setup, BSS clear, and stack
setup but before Rust. v38 proves the selected normal-runtime archive reaches
TALOS: rust_entry after Rust begins. v40 proves the selected normal-runtime
archive reaches TALOS: boot info parsed after BootInfo parsing. v42 proves the
selected normal-runtime archive reaches TALOS: target init after
target::init(&boot_info). v43 defines the next selected normal-runtime
discriminator by looping on TALOS: exceptions ready only after
arch::aarch64::exceptions::init() returns. v44 proves that exact selected
152,880-byte exceptions-ready archive is served by TFTP on Pi 5 and reaches
TALOS: exceptions ready in the authoritative serial hardware summary.

First unresolved continuation facts: kernel_main, route-start, runtime-ready,
packet-I/O, remote receipt, compatibility/service readiness, OpenSSH,
ssh-ready=true, fake command expansion, broad shell work, and phase transition.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-continuation-reconciliation-v45-20260701.

planningNeeded: false.

## Findings

- fixed: reconciled v44 against v34, v36, v38, v40, v42, and v43. The accepted
  selected normal-runtime frontier now reaches exceptions ready on Pi 5 with
  selected-byte TFTP service and restore proof.
- fixed: advanced the current selected normal-runtime frontier from
  selected-byte/target-init-retained to
  selected-byte/exceptions-ready-retained.
- fixed: preserved v44's evidence boundary: the exceptions-ready marker-loop
  proves exception initialization only and intentionally withholds kernel_main,
  route-start, runtime-ready, packet-I/O, service readiness, and OpenSSH
  claims.
- not-an-issue: no additional hardware action is needed for closeout because
  v44 already captured selected-byte service, marker retention, final
  pre-restore identity, restore proof, hardware lock release, and redaction
  review.
- deferred: kernel_main, route-start, runtime-ready, packet-I/O, OpenSSH
  compatibility, service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition remain unproved.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, hardware action, and phase transition as immediate
  successors.

## Evidence Map

- Closeout evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-continuation-closeout-v44/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-continuation-closeout-v44/classification.json.
- Reconciliation summary:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-continuation-closeout-v44/static/reconciliation-summary.md.
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
- Accepted v43 static exceptions-ready reconciliation:
  tasks/2026-07-01-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-continuation-reconciliation-v43.md.
- Accepted v44 Pi 5 exceptions-ready preflight:
  tasks/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-exceptions-continuation-preflight-v44.md.

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
phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-continuation-reconciliation-v45-20260701.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
