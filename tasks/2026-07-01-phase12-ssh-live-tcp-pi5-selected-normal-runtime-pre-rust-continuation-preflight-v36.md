# Phase 12 SSH Live TCP Pi 5 Selected Normal Runtime Pre-Rust Continuation Preflight V36

Task id: phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-rust-continuation-preflight-v36-20260701

Status: accepted after serialized Pi 5 validation.

Classification: selected-normal-runtime-pre-rust-marker-retained.

Evidence level: hardwareTestLock, static archive review, lab-controller API
identity, stable same-cursor TFTP evidence, serial hardware output marker
summary, known-good control, unchanged candidate rerun, restore proof, and
task-owned JSON evidence.

## Goal

Run the v35-selected normal-runtime pre-rust continuation discriminator on the
Pi 5 and classify whether the selected Image reaches TALOS:
asm_pre_rust_entry after assembly setup and before rust_entry.

## Scope Performed

- Promoted this queued hardware task after v35 accepted and selected it.
- Acquired hardwareTestLock before lab publication, boot mutation, or Pi 5
  power action.
- Published only the v35-selected pre-rust archive:
  target/tmp/selected-normal-runtime-pre-rust-v35.tar.gz.
- Captured primary candidate evidence, a known-good control, and an unchanged
  candidate rerun after the primary helper window became contaminated.
- Restored the lab to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirmed the final
  restored tree before releasing hardwareTestLock.

## Hardware Result

Primary candidate: selected-normal-runtime-pre-rust-v36-candidate.

The primary preflight identity matched selected tree
28e048845ae76bc90c6227959536e079d007e7d1e71a17122ddc1011cb42d345 with
da591740/kernel_2712.img at 152,144 bytes. The helper run did not complete its
capture-window summary; manual rescue serial captured TALOS: asm_pre_rust_entry
13 times, but the same-cursor TFTP/final identity window had already reverted
to the restored 104,136-byte tree. This primary run is recorded as
inconclusive-contaminated-before-restore, not as decisive selected-byte
evidence.

Known-good control:
selected-normal-runtime-pre-rust-v36-known-good-control.

The restored production-timer control served da591740/kernel_2712.img twice at
104,136 bytes and retained rpi5-production-timer-preemption: PASS once, proving
the serial/TFTP path could still show a known-good Talos marker. The helper
rejected decisive control classification because stale retained control markers
prevented serial-freshness-v1; no source change was made.

Unchanged candidate rerun:
selected-normal-runtime-pre-rust-v36-candidate-rerun.

The unchanged v36 candidate rerun served da591740/kernel_2712.img twice at
152,144 bytes, retained final pre-restore identity on selected tree
28e048845ae76bc90c6227959536e079d007e7d1e71a17122ddc1011cb42d345, and retained
TALOS: asm_pre_rust_entry 542 times in the authoritative helper summary. The
summary identity join was decisive with no rejection reasons, and the lab was
restored to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Terminal Classification

selected-normal-runtime-pre-rust-marker-retained.

This proves the selected 152,144-byte normal-runtime pre-rust archive can reach
the assembly pre-rust loop on Pi 5. It does not prove rust_entry, BootInfo
parsing, target init, exceptions, kernel_main, packet-I/O, OpenSSH/generated-root
retry, remote receipt, compatibility/service readiness, ssh-ready=true, fake
command expansion, broad shell work, or phase transition.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-pre-rust-continuation-closeout-v36-20260701.

planningNeeded: false.

## Findings

- fixed: published and ran the v35-selected pre-rust continuation discriminator
  under hardwareTestLock with selected-byte TFTP evidence and restore proof.
- fixed: the unchanged candidate rerun retained TALOS: asm_pre_rust_entry 542
  times with selected 152,144-byte TFTP service and selected final
  pre-restore identity.
- fixed: ran a known-good control and unchanged candidate rerun after the
  primary helper window became contaminated before restore.
- not-an-issue: the primary manual rescue serial marker is useful supporting
  evidence, but it is not used for terminal classification because selected
  TFTP/final identity had been lost.
- deferred: rust_entry and later normal-runtime progress remain the first
  missing feature facts for closeout/supervisor planning.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as immediate successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-rust-continuation-preflight-v36/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-rust-continuation-preflight-v36/evidence-map.json.
- Primary candidate:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-rust-continuation-preflight-v36/lab/candidate/.
- Known-good control:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-rust-continuation-preflight-v36/lab/known-good-control/.
- Unchanged candidate rerun:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-rust-continuation-preflight-v36/lab/candidate-rerun/.

## Redaction Review

Task-owned summaries retain task ids, run labels, hashes, byte counts, marker
names, marker counts, selected-tree hashes, classifications, and validation
outcomes. They do not retain raw serial text, raw TFTP peer/log-line fields,
packet payloads, SSH/session/key material, boot artifact bytes, private data,
or stable secret-derived identifiers. Raw local lab JSON is retained only under
task-owned evidence directories for local review.

## Validation

- git status --short --branch before lab action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- static archive/image review before publication: pass.
- Lab API GET /status and /boot/files before/after publication, before power,
  final pre-restore, and after restore: pass for the decisive rerun.
- fresh serial cursor/drain and GET /tftp/logs cursor before Pi 5 power action:
  pass for the decisive rerun.
- stable same-cursor TFTP delta before restore: pass for the decisive rerun.
- known-good control and unchanged candidate rerun after primary contamination:
  pass.
- restore to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirm
  post-restore tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10: pass.

Implementation commit: recorded in supervisor state after commit creation.
