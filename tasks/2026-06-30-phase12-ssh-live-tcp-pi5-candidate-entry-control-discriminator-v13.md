# Phase 12 SSH Live TCP Pi 5 Candidate Entry Control Discriminator V13

Task id: phase12-ssh-live-tcp-pi5-candidate-entry-control-discriminator-v13-20260630

Status: accepted after commit.

Classification: control-entry-passes.

Evidence level: serialized Pi 5 hardware/control run with lab-controller
identity, fresh serial cursor, repaired same-cursor TFTP delta, final
pre-restore identity, restore proof, local JSON classification, docs build, and
diff checks.

## Goal

Execute the contracted current-tree production-timer selected-path phase-line
control discriminator for the selected-fetch/no-runtime-marker boundary.

## Scope Performed

- Promoted this queued hardware/control task after
  phase12-ssh-live-tcp-candidate-entry-control-contract-v13-20260630 accepted
  candidate-entry-control-contract-ready and selected this exact task.
- Acquired hardwareTestLock before lab publication, Pi 5 power action, and
  restore-affecting lab operations.
- Published the current-tree production-timer control archive, power-cycled the
  Pi 5, captured fresh serial and TFTP windows, recorded final pre-restore
  identity, restored to the accepted selected-control snapshot, and released the
  hardware lock after restore proof.
- Classified the contracted phase lines in the fresh serial window.
- Stopped before packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility claim, service success claim, ssh-ready=true, fake/kernel-backed
  command expansion, broad shell work, or phase transition.

## Terminal Classification

control-entry-passes.

The control run phase12-entry-control-v13-20260630T111930Z retained selected
da591740/kernel_2712.img identity before hardware action and before restore.
The repaired same-cursor TFTP delta was stable and observed two selected
kernel serves with the expected 104,136-byte count. The serial window retained
firmware NETWORK output and one rpi5-production-timer-preemption: PASS marker.
Final pre-restore identity stayed on control tree
4edd4f1dad12ea06e3c45b1435f9a2d16e9c2046226d8963a0d8413a9f7226d1 with
effective_kernel=kernel_2712.img. Restore to
phase12-ssh-v10-openssh-clean-pre-20260624T074100Z succeeded and post-restore
tree hash was a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

selected_next_task:
phase12-ssh-live-tcp-kernel-entry-boundary-closeout-v13-20260630.

planningNeeded: false.

## Phase-Line Classification

- TALOS: rust_entry: absent.
- TALOS: boot info parsed: absent.
- TALOS: target init: absent.
- TALOS: exceptions ready: absent.
- TALOS: kernel_main: absent.

Per the accepted predecessor contract, absent phase lines and absent kernel_main
are metadata-only for this control because the downstream production-timer PASS
marker is present.

## Findings

- fixed: executed the one predecessor-contracted serialized Pi 5 control
  discriminator and recorded the hardware lock lifecycle.
- fixed: proved current-tree selected-path control entry via stable selected
  TFTP identity, firmware NETWORK serial output, downstream PASS marker, final
  identity, and restore proof.
- fixed: recorded explicit present/absent phase-line classification for the
  five contracted phase inputs.
- not-an-issue: the serial cursor was saturated; the capture helper used the
  accepted direct-read fallback, retained an empty pre-power serial drain, and
  still captured a post-power PASS marker.
- deferred: candidate kernel/runtime entry closeout remains required to
  reconcile why the live TCP candidate lacks runtime markers while this control
  passes entry.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt, service
  readiness, and phase transition as permissible immediate successors.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-entry-control-discriminator-v13/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-entry-control-discriminator-v13/classification.json.
- Run directory:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-entry-control-discriminator-v13/phase12-entry-control-v13-20260630T111930Z/.

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
- Lab API identity before hardware run: pass; current-tree control tree
  4edd4f1dad12ea06e3c45b1435f9a2d16e9c2046226d8963a0d8413a9f7226d1 with
  effective_kernel=kernel_2712.img and selected da591740/kernel_2712.img at
  104,136 bytes.
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
phase12-ssh-live-tcp-kernel-entry-boundary-closeout-v13-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
