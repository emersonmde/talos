# Phase 12 SSH Live TCP Pi 5 Selected Normal Runtime BootInfo Continuation Preflight V40

Task id: phase12-ssh-live-tcp-pi5-selected-normal-runtime-bootinfo-continuation-preflight-v40-20260701

Status: accepted after serialized Pi 5 validation.

Classification: selected-normal-runtime-bootinfo-marker-retained.

Evidence level: hardwareTestLock, static archive/image review, lab-controller
API identity, fresh serial cursor/drain, stable same-cursor TFTP evidence,
serial hardware output marker summary, final pre-restore identity, restore
proof, task-owned JSON evidence, docs build, and diff checks.

## Goal

Run the v39-selected normal-runtime BootInfo continuation discriminator on the
Pi 5 and classify whether the selected Image reaches the TALOS: boot info
parsed boundary after rust_entry parses the firmware handoff and before target
init, exceptions, kernel_main, packet-I/O, OpenSSH, or service readiness.

## Scope Performed

- Promoted this queued hardware task after v39 accepted and selected it.
- Acquired hardwareTestLock before lab publication, boot mutation, or Pi 5
  power action.
- Published only the v39-selected BootInfo marker-loop archive:
  target/tmp/selected-normal-runtime-bootinfo-v39.tar.gz.
- Captured candidate identity, fresh serial/TFTP cursors, stable same-window
  TFTP evidence, serial marker output, final pre-restore identity, and restore
  proof.
- Restored the lab to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirmed the final
  restored tree before releasing hardwareTestLock.

## Hardware Result

Candidate: selected-normal-runtime-bootinfo-v40.

The published selected archive matched selected tree
1682bbc6158a718b5f630b9a42762b2e1e39a38f7137764b4fde5de16b81b0b0 before
power. da591740/kernel_2712.img was present at 152,880 bytes, effective kernel
was kernel_2712.img, and final pre-restore identity remained on the same
selected tree.

The stable same-cursor TFTP delta observed 13 events and served
da591740/kernel_2712.img twice at 152,880 bytes. The saturated direct-read
serial window retained TALOS: boot info parsed 1,826 times. The pre-power
retained sample had zero TALOS: boot info parsed occurrences, while stale
predecessor rust_entry occurrences from earlier retained output were present.
The helper identity join allowed decisive hardware classification with no
rejection reasons because selected tree identity, selected-byte TFTP service,
final pre-restore identity, restore proof, and the required BootInfo marker
were joined under the same run label.

Post-restore identity returned to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Terminal Classification

selected-normal-runtime-bootinfo-marker-retained.

This proves the selected 152,880-byte normal-runtime BootInfo marker-loop
archive reaches BootInfo parsing on Pi 5 and emits TALOS: boot info parsed
after BootInfo::from_aarch64_x0(dtb_pa) returns. It does not prove target init,
exceptions, kernel_main, route-start, runtime-ready, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility/service readiness,
ssh-ready=true, fake command expansion, broad shell work, or phase transition.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-bootinfo-continuation-closeout-v40-20260701.

planningNeeded: false.

## Findings

- fixed: published and ran the v39-selected BootInfo continuation
  discriminator under hardwareTestLock with selected-byte TFTP evidence and
  restore proof.
- fixed: selected tree identity, selected fetch byte count, same-window TFTP,
  serial marker output, final pre-restore identity, and restore identity are
  joined under one run label.
- fixed: TALOS: boot info parsed was retained 1,826 times in the serial
  hardware window, proving the selected archive reaches the BootInfo parsing
  boundary.
- not-an-issue: stale retained rust_entry output was present in the saturated
  pre-power serial tail, but the required BootInfo marker had zero pre-power
  occurrences and 1,826 post-power occurrences.
- not-an-issue: the generic nonce-oriented freshness checker rejects this
  static marker-loop nonce as stale; this task's accepted gate is the
  helper-owned identity join plus required-marker freshness for TALOS: boot
  info parsed.
- deferred: target init, exceptions, kernel_main, route-start, runtime-ready,
  packet-I/O, OpenSSH compatibility, service readiness, ssh-ready=true, fake
  command expansion, broad shell work, and phase transition remain unproved.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as immediate successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-bootinfo-continuation-preflight-v40/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-bootinfo-continuation-preflight-v40/evidence-map.json.
- Candidate run:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-bootinfo-continuation-preflight-v40/lab/v40-candidate/.
- Redacted candidate summary:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-bootinfo-continuation-preflight-v40/lab/v40-candidate/run-summary-redacted.json.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-bootinfo-continuation-preflight-v40/validation/archive-review.stdout.txt.
- BootInfo archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-bootinfo-continuation-preflight-v40/validation/bootinfo-archive-review.stdout.txt.

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
- Lab API GET /status and /boot/files after publication, before power, final
  pre-restore, and after restore: pass.
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
