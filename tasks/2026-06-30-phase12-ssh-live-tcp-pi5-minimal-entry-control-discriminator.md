# Phase 12 SSH Live TCP Pi 5 Minimal Entry-Control Discriminator

Task id: phase12-ssh-live-tcp-pi5-minimal-entry-control-discriminator-20260630

Status: accepted after commit.

Classification: blocked-selected-path-entry-control.

Evidence level: source/archive identity, lab-controller API identity,
serialized Pi 5 hardware power/output, helper-owned capture/restore window,
minimal entry-control marker checker, task-owned JSON evidence, docs build,
and diff checks. No live TCP runtime route, packet-I/O discriminator,
OpenSSH/generated-root retry, remote receipt, compatibility, service success,
ssh-ready=true, broad shell work, or phase transition was performed.

## Goal

Run the smallest serialized Pi 5 control that keeps the selected
da591740/kernel_2712.img fetch path but strips the live TCP runtime route to a
minimal early entry marker.

## Scope Performed

- Promoted the queued hardware discriminator only after the accepted
  minimal-entry-control contract selected this exact task.
- Acquired hardwareTestLock before lab status reads, archive publication, or
  Pi 5 power action.
- Built a nonce-bearing rpi5_minimal_entry_control archive from commit
  6e1ff8bb4a2a23d11f97078cf6c07e24d5fc4fcc with root and da591740/selected
  Pi 5 boot files.
- Published the reviewed control archive, ran one foreground capture bundle,
  retained selected-fetch TFTP evidence, final pre-restore control identity,
  serial marker metadata, restore proof, post-restore identity, and checker
  output.
- Stopped before live TCP, packet-I/O, OpenSSH/generated-root retry, remote
  receipt, compatibility, service success, ssh-ready=true, broad shell work, or
  phase transition.

## Terminal Classification

blocked-selected-path-entry-control.

The minimal entry-control run preserved the selected-fetch and pre-restore
identity contract:

- control archive SHA-256:
  72dffa14b6bca6711288ab2684e23c64ae8755a0653b5fc68ce4dba1836c6a06;
- selected expected fetch: da591740/kernel_2712.img, 52,848 bytes,
  SHA-256 adca8ccfe7a9d7fa23ef93275c6afec3730c27ed2df5fc308ef2bc772060d8ed;
- post-publication and final pre-restore tree:
  ab20b3c96ce9b71aa8b19e5a277ebb119c6a0d34465d940b9c34fbdac2897c6b;
- TFTP stable same-cursor delta observed two selected
  da591740/kernel_2712.img serves and both matched 52,848 bytes;
- helper-owned restore returned the lab to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The minimal marker checker found zero occurrences of the required marker:

`TALOS: minimal-entry-control-ready capture-nonce=minimal-entry-control-20260630T011323Z`

The post-power serial window still retained firmware NETWORK markers while the
required nonce-bearing minimal entry marker was absent. This is not a live TCP,
packet-I/O, or OpenSSH-ready result. It keeps the first missing fact at the
selected-path entry-control boundary: selected control bytes are served, but
the selected kernel does not reach the earliest control marker in the retained
post-power serial window.

selected_next_task: null.

planningNeeded: true.

planningReason: Minimal entry-control discriminator preserved the first missing
fact; supervisor planning is required before retrying candidate/runtime work or
attempting packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility, service success, ssh-ready=true, broad shell work, or phase
transition.

## Findings

- fixed: the selected da591740/kernel_2712.img fetch path remained present
  through archive review, lab publication, same-window TFTP delta, and final
  pre-restore identity.
- fixed: the foreground capture helper completed and restored the lab to the
  selected control tree.
- blocked: the required nonce-bearing minimal-entry-control-ready marker was
  absent after the selected control fetch.
- deferred: live TCP candidate repair/retry, packet-I/O, and OpenSSH remain
  blocked until supervisor planning selects a new bounded task.
- not-an-issue: the control archive retained fail-closed claims for live TCP,
  packet I/O, OpenSSH, ssh-ready, service success, and phase transition.
- removed: generated upload archive and boot tree remain untracked target/tmp
  artifacts and are not part of durable evidence.

## Evidence Map

- Evidence directory:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-control-discriminator/minimal-entry-control-20260630T011323Z/.
- Classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-control-discriminator/minimal-entry-control-20260630T011323Z/classification.json.
- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-control-discriminator/minimal-entry-control-20260630T011323Z/evidence-map.json.
- Control archive metadata:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-control-discriminator/minimal-entry-control-20260630T011323Z/control-identity/archive-metadata.json.
- Capture summary:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-control-discriminator/minimal-entry-control-20260630T011323Z/control-run/capture-invariant-summary.json.
- Minimal entry-control marker checker:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-control-discriminator/minimal-entry-control-20260630T011323Z/control-run/minimal-entry-control-marker-check.json.
- Capture-window checker:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-minimal-entry-control-discriminator/minimal-entry-control-20260630T011323Z/control-run/capture-window-v5-check.json.

## Redaction Review

Task-owned JSON evidence was scrubbed to replace raw serial text and serial
base64 with redaction placeholders and remove raw TFTP peer/log-line fields.
Durable evidence retains task ids, source/archive metadata, tree hashes, byte
counts, marker counters, cursor/capture classifications, validation commands
and results, and metadata-only counters. It does not retain packet payload
contents, key material, session material, boot artifact bytes, private user
data, stable secret-derived identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- Control archive build/materialization and static marker/order review: pass;
  archive review confirmed selected da591740/kernel_2712.img and the
  nonce-bearing minimal entry-control tokens.
- Lab API GET /status, GET /boot/files, and GET /boot/snapshots before power
  and after restore: pass.
- Fresh serial cursor/completeness diagnostics before power: pass.
- GET /tftp/logs tail cursor and stable same-cursor delta before restore:
  pass; two selected 52,848-byte kernel serves.
- Minimal entry-control marker checker: pass as blocker evidence;
  blocked-selected-path-entry-control.
- Capture-window checker: pass as blocker evidence; rejected only the missing
  nonce/minimal control marker.
- Restore to named selected-control snapshot: pass; final GET /status reports
  tree_hash=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
  and effective_kernel=kernel_2712.img.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: null.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
