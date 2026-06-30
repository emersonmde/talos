# Phase 12 SSH Live TCP Pi 5 Candidate Entry Bisect Discriminator V14

Task id: phase12-ssh-live-tcp-pi5-candidate-entry-bisect-discriminator-v14-20260630

Status: accepted after commit.

Classification: blocked-control-entry.

Evidence level: serialized Pi 5 hardware discriminator with lab-controller
identity, run-unique current-tree minimal entry-control archive review, fresh
serial cursor, repaired same-cursor TFTP delta, final pre-restore identity,
restore proof, local JSON classification, docs build, and diff checks.

## Goal

Execute the contracted current-tree minimal entry-control selected-path
discriminator between the v12 live TCP selected-fetch/no-entry endpoint and the
v13 production-timer selected-path passing endpoint.

## Scope Performed

- Promoted this queued hardware task after
  phase12-ssh-live-tcp-candidate-entry-bisect-control-contract-v14-20260630
  accepted candidate-entry-bisect-control-ready and selected this exact task.
- Acquired hardwareTestLock before lab publication, Pi 5 power action, and
  restore-affecting lab operations.
- Built and published one run-unique rpi5_minimal_entry_control archive with
  capture nonce phase12-entry-bisect-v14-20260630T125233Z.
- Power-cycled the Pi 5, captured fresh serial and TFTP windows, recorded final
  pre-restore identity, restored to the accepted selected-control snapshot, and
  released the hardware lock after restore proof.
- Classified the contracted firmware/phase-line/minimal-entry markers.
- Stopped before packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility claim, service success claim, ssh-ready=true, fake/kernel-backed
  command expansion, broad shell work, or phase transition.

## Terminal Classification

blocked-control-entry.

The discriminator run phase12-entry-bisect-v14-20260630T125233Z retained
selected da591740/kernel_2712.img identity before hardware action and before
restore. The run-unique selected kernel was 52,848 bytes with SHA-256
1f2b2246fb3c5d00852ee981d0be93275786fe6e92520de5a4945d2c9862d38e, and the
published tree hash was
59b29c6004943c7789d910071809291ab279a1c90cc1d1c6963fba7ea7b60f7b.

The repaired same-cursor TFTP delta was stable and observed two selected
kernel serves with the expected 52,848-byte count. The serial window retained
firmware NETWORK output but did not retain TALOS: rust_entry, TALOS: boot info
parsed, TALOS: target init, TALOS: exceptions ready, TALOS: kernel_main, or
the required nonce-bearing TALOS: minimal-entry-control-ready marker. Final
pre-restore identity stayed on the minimal entry-control tree, and restore to
phase12-ssh-v10-openssh-clean-pre-20260624T074100Z succeeded with post-restore
tree hash a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

selected_next_task:
phase12-ssh-live-tcp-kernel-entry-boundary-closeout-v14-20260630.

planningNeeded: false.

## Phase-Line Classification

- Firmware NETWORK: present.
- TALOS: rust_entry: absent.
- TALOS: boot info parsed: absent.
- TALOS: target init: absent.
- TALOS: exceptions ready: absent.
- TALOS: kernel_main: absent.
- TALOS: minimal-entry-control-ready: absent.

The run is not classified as blocked-identity, blocked-tftp-capture,
blocked-restore, or inconclusive-with-required-discriminator because selected
fetch identity, stable TFTP byte agreement, final pre-restore identity, and
restore proof were decisive. The known-good control branch of inconclusive
triage was therefore not required.

## Findings

- fixed: executed the one predecessor-contracted serialized Pi 5 minimal
  entry-control discriminator and recorded the hardware lock lifecycle.
- fixed: proved current-tree selected fetch for the minimal entry-control
  archive with stable same-cursor TFTP byte agreement and final pre-restore
  identity.
- fixed: classified the contracted firmware, phase-line, kernel_main, and
  minimal-entry-control marker set.
- not-an-issue: the run-unique nonce changed the selected kernel byte count
  from the predecessor's static nonce review; this task recorded its own
  archive and selected-kernel identity before hardware action.
- deferred: v14 closeout must reconcile why a minimal kernel_main hook still
  lacks the nonce-bearing marker despite selected fetch and firmware NETWORK
  evidence.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt, service
  readiness, and phase transition as permissible immediate successors.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-entry-bisect-discriminator-v14/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-entry-bisect-discriminator-v14/classification.json.
- Run directory:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-entry-bisect-discriminator-v14/phase12-entry-bisect-v14-20260630T125233Z/.

## Redaction Review

This task summary retains task ids, commit ids, path labels, hashes, byte
counts, marker labels, classifications, validation command results, and
selected successor metadata. It omits raw serial text, raw TFTP peer/log-line
fields, packet payloads, SSH keys/session material, boot artifact bytes,
private user data, stable secret-derived identifiers, and unnecessary hardware
data. Raw lab-controller serial/TFTP endpoint artifacts are retained only under
task-owned hardware evidence.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- Lab API identity before hardware run: pass; current-tree minimal
  entry-control tree 59b29c6004943c7789d910071809291ab279a1c90cc1d1c6963fba7ea7b60f7b
  with effective_kernel=kernel_2712.img and selected
  da591740/kernel_2712.img at 52,848 bytes.
- Fresh serial cursor and repaired TFTP delta: pass; stable same-cursor TFTP
  delta observed two selected kernel serves and retained the fresh serial
  capture window.
- Restore to predecessor-named snapshot and confirm with lab API: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Result

selected_next_task:
phase12-ssh-live-tcp-kernel-entry-boundary-closeout-v14-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
