# Phase 12 SSH Live TCP Pi 5 Selected Normal Runtime Rust Entry Continuation Preflight V38

Task id: phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-continuation-preflight-v38-20260701

Status: accepted after serialized Pi 5 validation.

Classification: selected-normal-runtime-rust-entry-marker-retained.

Evidence level: hardwareTestLock, static archive/image review, lab-controller
API identity, fresh serial cursor/drain, stable same-cursor TFTP evidence,
serial hardware output marker summary, final pre-restore identity, restore
proof, task-owned JSON evidence, docs build, and diff checks.

## Goal

Run the v37-selected normal-runtime rust_entry continuation discriminator on
the Pi 5 and classify whether the selected Image reaches TALOS: rust_entry
after rust_entry begins and before BootInfo parsing, target init, exceptions,
kernel_main, packet-I/O, OpenSSH, or service readiness.

## Scope Performed

- Promoted this queued hardware task after v37 accepted and selected it.
- Acquired hardwareTestLock before lab publication, boot mutation, or Pi 5
  power action.
- Published only the v37-selected rust_entry marker-loop archive:
  target/tmp/selected-normal-runtime-rust-entry-v37.tar.gz.
- Captured candidate identity, fresh serial/TFTP cursors, stable same-window
  TFTP evidence, serial marker output, final pre-restore identity, and restore
  proof.
- Restored the lab to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirmed the final
  restored tree before releasing hardwareTestLock.

## Hardware Result

Candidate: selected-normal-runtime-rust-entry-v38-candidate.

The published selected archive matched selected tree
74c090dd99abf3b3b6cc49bb2bc6a52f3e79f193632f7e9c4b17ab9a1514baed before
power. da591740/kernel_2712.img was present at 152,816 bytes, effective kernel
was kernel_2712.img, and final pre-restore identity remained on the same
selected tree.

The stable same-cursor TFTP delta observed 13 events and served
da591740/kernel_2712.img twice at 152,816 bytes. The serial window retained
TALOS: rust_entry 208 times. The marker-family summary treats the earlier
assembly provenance markers as predecessor facts from v34/v36 and records
TALOS: rust_entry as the deepest marker visible in this run. The helper
identity join allowed decisive hardware classification with no rejection
reasons.

Post-restore identity returned to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Terminal Classification

selected-normal-runtime-rust-entry-marker-retained.

This proves the selected 152,816-byte normal-runtime rust_entry marker-loop
archive can enter rust_entry on Pi 5 and emit TALOS: rust_entry after Rust
begins. It does not prove BootInfo parsing, target init, exceptions,
kernel_main, packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, fake command expansion, broad
shell work, or phase transition.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-continuation-closeout-v38-20260701.

planningNeeded: false.

## Findings

- fixed: published and ran the v37-selected rust_entry continuation
  discriminator under hardwareTestLock with selected-byte TFTP evidence and
  restore proof.
- fixed: selected tree identity, selected fetch byte count, same-window TFTP,
  serial marker output, final pre-restore identity, and restore identity are
  joined under one run label.
- fixed: TALOS: rust_entry was retained 208 times in the serial hardware
  window, proving rust_entry entry for the selected archive.
- not-an-issue: TALOS: asm_start and TALOS: asm_pre_rust_entry are absent from
  this saturated direct-read window, but v34/v36 already prove those ordered
  predecessor boundaries for selected normal-runtime artifacts; this task's
  required marker is TALOS: rust_entry.
- deferred: BootInfo parsing, target init, exceptions, kernel_main, packet-I/O,
  OpenSSH compatibility, service readiness, ssh-ready=true, fake command
  expansion, broad shell work, and phase transition remain unproved.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as immediate successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-continuation-preflight-v38/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-continuation-preflight-v38/evidence-map.json.
- Candidate run:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-continuation-preflight-v38/lab/candidate/.
- Redacted candidate summary:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-continuation-preflight-v38/lab/candidate/run-summary-redacted.json.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-continuation-preflight-v38/validation/archive-review.stdout.txt.
- Rust-entry archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-continuation-preflight-v38/validation/rust-entry-archive-review.stdout.txt.

## Redaction Review

Task-owned aggregate summaries retain task ids, run labels, hashes, byte
counts, marker names, marker counts, selected-tree hashes, classifications, and
validation outcomes. They do not retain raw serial text, raw TFTP peer/log-line
fields, packet payloads, SSH/session/key material, boot artifact bytes, private
data, or stable secret-derived identifiers. Raw local lab JSON is retained only
under task-owned evidence directories for local review.

## Validation

- git status --short --branch before lab action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- static archive/image review before publication: pass.
- Lab API GET /status and /boot/files before publication, after publication,
  before power, final pre-restore, and after restore: pass.
- fresh serial cursor/drain and GET /tftp/logs cursor before Pi 5 power action:
  pass.
- stable same-cursor TFTP delta before restore: pass.
- restore to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirm
  post-restore tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

Implementation commit: recorded in supervisor state after commit creation.
