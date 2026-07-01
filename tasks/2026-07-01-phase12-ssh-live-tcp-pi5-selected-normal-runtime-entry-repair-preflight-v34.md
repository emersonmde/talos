# Phase 12 SSH Live TCP Pi 5 Selected Normal Runtime Entry Repair Preflight V34

Task id: phase12-ssh-live-tcp-pi5-selected-normal-runtime-entry-repair-preflight-v34-20260701

Status: accepted after serialized Pi 5 validation.

Classification: selected-normal-runtime-entry-marker-retained.

Evidence level: hardwareTestLock, static archive review, lab-controller API
identity, stable same-cursor TFTP evidence, serial hardware output marker
summary, restore proof, and task-owned JSON evidence.

## Goal

Run the v33-selected normal-runtime entry-loop discriminator on the Pi 5 and
classify whether the selected Image reaches the predecessor-named earliest
entry marker.

## Scope Performed

- Promoted this queued hardware task after v33 accepted and selected it.
- Acquired hardwareTestLock before lab publication, boot mutation, or Pi 5
  power action.
- Published only the v33-selected archive:
  target/tmp/selected-normal-runtime-entry-loop-v33.tar.gz.
- Captured selected-tree identity, fresh serial and TFTP cursors, Pi 5 serial
  output, stable same-cursor TFTP delta, final pre-restore identity, and
  restore proof.
- Restored the lab to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirmed the final
  restored tree before releasing hardwareTestLock.

## Hardware Result

Primary candidate: selected-normal-runtime-entry-v34-candidate.

The selected archive served da591740/kernel_2712.img twice at 152,144 bytes
with SHA-256
5aa2b4ab51afa018d4c39fc5843e5df01a76dbc42bce2b40287693b5c77d311d. The
authoritative helper summary retained TALOS: asm_start 504 times in the marker
family counts, with the deepest present marker also TALOS: asm_start. Final
pre-restore identity stayed on selected tree
23d7eecf13a716d6e762318df5375d567b0abbfb209d37e4cd40e3a46b79cc7f, and restore
returned the lab to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The capture helper's suggested_classification remained
reset-side-effect-without-visible-marker-candidate because its
has_required_marker field was false despite required_marker_occurrences=504 and
marker_family.present=true. This task uses the explicit marker-family evidence
for the terminal classification and records the helper classifier mismatch as
deferred source cleanup, because source edits were not allowed after hardware
began.

## Terminal Classification

selected-normal-runtime-entry-marker-retained.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-entry-repair-closeout-v34-20260701.

planningNeeded: false.

Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, fake command expansion, broad
shell work, and phase transition remain blocked.

## Findings

- fixed: published and ran the v33-selected normal-runtime entry-loop
  discriminator under hardwareTestLock with selected-byte TFTP evidence and
  restore proof.
- fixed: the selected 152,144-byte da591740/kernel_2712.img served twice and
  retained TALOS: asm_start 504 times in the authoritative helper summary
  marker-family counts.
- not-an-issue: known-good control and unchanged candidate rerun were not
  required because the primary selected run retained the predecessor-named
  earliest marker.
- deferred: the helper suggested_classification did not follow the
  marker-family retention evidence; source cleanup is outside this
  no-source-edits-after-hardware task.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as immediate successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-entry-repair-preflight-v34/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-entry-repair-preflight-v34/evidence-map.json.
- Primary candidate:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-entry-repair-preflight-v34/lab/candidate/.
- Authoritative helper summary:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-entry-repair-preflight-v34/lab/candidate/original-helper-summary.json.

## Redaction Review

Task-owned summaries retain task ids, run labels, hashes, byte counts, marker
names, marker counts, selected-tree hashes, classifications, and validation
outcomes. They do not retain raw serial text, raw TFTP peer/log-line fields,
packet payloads, SSH/session/key material, boot artifact bytes, private data,
or stable secret-derived identifiers.

## Validation

- git status --short --branch before lab action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- static archive/image review before publication: pass.
- Lab API GET /status, /boot/files, and nonfatal GET / root-endpoint metadata
  before publication, after publication, before power, final pre-restore, and
  after restore: pass.
- fresh serial cursor/drain and GET /tftp/logs cursor before Pi 5 power action:
  pass.
- stable same-cursor TFTP delta before restore: pass in the authoritative
  helper summary.
- restore to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirm
  post-restore tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10: pass.

Implementation commit: recorded in supervisor state after commit creation.
