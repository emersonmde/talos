# Phase 12 SSH Live TCP Pi 5 Selected Normal Runtime Rust Entry After Pre-Rust Preflight V64

Task id: phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-after-pre-rust-preflight-v64-20260702

Status: accepted after serialized Pi 5 validation.

Classification: selected-normal-runtime-rust-entry-marker-retained.

Evidence level: hardwareTestLock, static archive/image review, lab-controller
API identity, fresh serial cursor/drain, stable same-cursor TFTP evidence,
serial hardware output marker summary, final pre-restore identity, restore
proof, task-owned JSON evidence, docs build, and diff checks.

## Goal

Run the v63-selected rust_entry marker-loop archive on the Pi 5 and classify
whether the selected normal-runtime image reaches TALOS: rust_entry after the
accepted TALOS: asm_pre_rust_entry frontier.

## Scope Performed

- Promoted the ready v64 hardware preflight after accepted v63 selected this
  exact task and supervisor planning added it to the queue.
- Acquired hardwareTestLock before lab publication, boot mutation, Pi 5 power
  action, serial capture, or TFTP capture.
- Published only the v63 selected normal-runtime rust_entry archive:
  target/tmp/selected-normal-runtime-rust-entry-v63.tar.gz.
- Captured candidate identity, fresh serial/TFTP cursors, stable same-window
  TFTP evidence, marker-family serial output, final pre-restore identity, and
  restore proof.
- Restored the lab to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and released
  hardwareTestLock.

## Hardware Result

Candidate: selected-normal-runtime-rust-entry-v64-candidate.

The published selected archive matched selected tree
d0a5132b630258a98de56fa7e9c0eb9d1cdb41358b68e91321384461a835b6b2 before
power. da591740/kernel_2712.img was present at 152,816 bytes, effective kernel
was kernel_2712.img, and final pre-restore identity remained on the same
selected tree.

The stable same-cursor TFTP delta observed 13 events and served
da591740/kernel_2712.img twice at 152,816 bytes. The serial window retained
TALOS: rust_entry 208 times. TALOS: asm_start, TALOS:
asm_pre_rust_entry, route-start, runtime-blocked, and runtime-ready retained
zero occurrences in this saturated direct-read window; asm_pre_rust_entry
remains a predecessor fact from v62 for this feature chain.

Post-restore identity returned to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Terminal Classification

selected-normal-runtime-rust-entry-marker-retained.

This proves the selected 152,816-byte normal-runtime rust_entry marker-loop
archive can enter rust_entry on Pi 5 and emit TALOS: rust_entry after the v62
pre-rust frontier. It does not prove BootInfo parsing, target init, exceptions,
kernel_main, route-start, runtime-ready, packet-I/O, OpenSSH/generated-root
retry, remote receipt, compatibility/service readiness, ssh-ready=true,
fake/kernel-backed command expansion, broad shell work, or phase transition.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-after-pre-rust-closeout-v64-20260702.

planningNeeded: false.

## Findings

- fixed: published and ran the v63 selected rust_entry discriminator under
  hardwareTestLock with selected-byte TFTP evidence and restore proof.
- fixed: selected tree identity, selected fetch byte count, same-window TFTP,
  serial marker output, final pre-restore identity, and restore identity are
  joined under one run label.
- fixed: TALOS: rust_entry was retained 208 times in the serial hardware
  window, proving the selected archive entered rust_entry on Pi 5.
- not-an-issue: known-good control and candidate rerun were not required
  because the first candidate evidence was decisive, not inconclusive.
- deferred: BootInfo parsing, target init, exceptions, kernel_main,
  route-start, runtime-ready, packet-I/O, OpenSSH compatibility, service
  readiness, ssh-ready=true, fake command expansion, broad shell work, and
  phase transition remain unproved.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-after-pre-rust-preflight-v64/classification.json.
- Evidence map:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-after-pre-rust-preflight-v64/evidence-map.json.
- Archive review:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-after-pre-rust-preflight-v64/validation/archive-review.stdout.txt.
- Rust-entry archive review:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-after-pre-rust-preflight-v64/validation/rust-entry-archive-review.stdout.txt.
- Candidate summary:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-after-pre-rust-preflight-v64/lab/v64-candidate/candidate-summary.json.
- Capture helper summary:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-rust-entry-after-pre-rust-preflight-v64/lab/v64-candidate/capture-helper.stdout.json.

## Redaction Review

Task-owned aggregate summaries retain task ids, run labels, hashes, byte
counts, cursor metadata, marker names, marker counts, selected-tree hashes,
classifications, and validation outcomes. They do not retain raw serial text,
raw TFTP peer/log-line fields, packet payloads, SSH/session/key material, boot
artifact bytes, private data, or stable secret-derived identifiers in the task
record or classification.

## Validation

- git status --short --branch before lab publication/hardware action:
  ## main...origin/main [ahead 287].
- jq empty on supervisor state before/after hardwareTestLock changes and on
  task-owned JSON evidence: pass.
- Static archive/image review before publication: pass; archive SHA-256
  7211853ae0fe6008b10b340725799503ff3ff9be46518428d2e5d3fdbf4e641f and
  selected kernel SHA-256
  347679f5797d2c99d61a56d5b250ee0245a0f19e9ac5f927491c4b9a019709c6.
- Lab API candidate identity before publication, after publication,
  post-power/pre-observe, final pre-restore, and after restore: recorded.
- Fresh serial cursor and marker-family serial observation: recorded; TALOS:
  rust_entry retained 208 times.
- GET /tftp/logs cursor delta: recorded; selected candidate served
  da591740/kernel_2712.img twice at 152,816 bytes.
- Known-good control: not run because the first candidate was decisive.
- Candidate rerun: not run because the first candidate was decisive.
- Restore proof to accepted baseline before releasing hardwareTestLock: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-rust-entry-after-pre-rust-closeout-v64-20260702.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
