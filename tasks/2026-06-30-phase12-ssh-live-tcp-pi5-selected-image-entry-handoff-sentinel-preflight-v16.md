# Phase 12 SSH Live TCP Pi 5 Selected-Image Entry Handoff Sentinel Preflight V16

Task id: phase12-ssh-live-tcp-pi5-selected-image-entry-handoff-sentinel-preflight-v16-20260630

Status: accepted after commit.

Classification: blocked-selected-image-handoff.

Evidence level: serialized Pi 5 hardware preflight with lab-controller
identity, selected-image handoff sentinel archive review, fresh serial/TFTP
capture, stable same-cursor TFTP byte agreement, final pre-restore identity,
restore proof, local JSON classification, docs build, and diff checks.

## Goal

Execute the v16 selected-image handoff sentinel preflight selected by
phase12-ssh-live-tcp-selected-image-entry-handoff-sentinel-core-v16-20260630.

## Scope Performed

- Promoted this queued hardware task after the v16 core accepted
  selected-image-handoff-sentinel-ready and selected this exact task.
- Acquired hardwareTestLock before lab publication, Pi 5 power action, and
  restore-affecting lab operations.
- Built and published one rpi5_selected_image_handoff_sentinel archive from
  source commit 7f7c942db38a80e9a5e79559d49bb7cde94064d0.
- Power-cycled the Pi 5, captured serial and TFTP evidence, recorded final
  pre-restore identity, restored to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z, and released the
  hardware lock after restore proof.
- Stopped before packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility claim, service success claim, ssh-ready=true,
  fake/kernel-backed command expansion, broad shell work, or phase transition.

## Terminal Classification

blocked-selected-image-handoff.

Run phase12-handoff-sentinel-v16-20260630T155447Z published tree
531cc00d5d4f28696993a3c3852dde5b2fcbc32dee63a9412a472720371d31e0. The selected
da591740/kernel_2712.img was 87,432 bytes with SHA-256
7a841135cb2e5d6bf9be11d900e8c9fbabbac32a43f539780a90adc50e374888. The boot
archive SHA-256 was
b4e5b6b888c31fae1c254780c09210f62c8519a421c7e21928e6d366db6b47bf.

The stable same-cursor TFTP delta observed two selected
da591740/kernel_2712.img serves, and both matched the expected 87,432-byte
selected kernel size. The serial window was fresh for this run through an empty
pre-power serial drain followed by direct-read post-power firmware NETWORK
output. It did not retain TALOS: selected-image-handoff-sentinel-v16, TALOS:
kernel_main, or any later Talos marker. Final pre-restore identity stayed on
tree 531cc00d5d4f28696993a3c3852dde5b2fcbc32dee63a9412a472720371d31e0, and
restore to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z succeeded with
post-restore tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The generic candidate-capture-window-v5 checker reported
missing-run-unique-capture-nonce and required-marker-not-present-after-power.
That fixed-marker guard output is retained. It does not make this run
inconclusive because this task's predecessor-selected contract used a fixed
handoff marker, and the capture-helper identity join proved selected fetch
identity, selected byte agreement, serial freshness, final pre-restore identity,
and restore.

selected_next_task:
phase12-ssh-live-tcp-selected-image-entry-handoff-boundary-closeout-v16-20260630.

planningNeeded: false.

## Marker Classification

- Firmware NETWORK: present.
- TALOS: selected-image-handoff-sentinel-v16: absent.
- TALOS: kernel_main: absent.

The run is not classified as blocked-selected-image-identity,
blocked-selected-image-tftp-capture, blocked-restore, or
inconclusive-selected-image-handoff because selected fetch identity, stable TFTP
byte agreement, final pre-restore identity, serial freshness, and restore proof
were decisive. The known-good control branch of inconclusive triage was
therefore not required.

## Findings

- fixed: executed the one predecessor-selected serialized Pi 5 selected-image
  handoff sentinel preflight and recorded the hardware lock lifecycle.
- fixed: proved current-tree selected fetch for the handoff sentinel archive
  with stable same-cursor TFTP byte agreement and final pre-restore identity.
- fixed: classified the firmware-to-selected-image handoff marker absence below
  CPACR setup, BSS clear, stack setup, rust_entry, kernel_main, networking,
  packet I/O, OpenSSH, or shell work.
- not-an-issue: candidate-capture-window-v5 rejected the fixed-marker run for
  missing a run-unique nonce; this task's explicit marker contract was fixed,
  and the capture helper's empty pre-power drain plus post-power firmware output
  provided serial freshness for the negative marker result.
- deferred: v16 closeout must reconcile the blocked firmware-to-selected-image
  handoff boundary before any packet-I/O, OpenSSH/generated-root retry, remote
  receipt, compatibility/service readiness, broad shell work, or phase
  transition.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt, service
  readiness, broad shell work, and phase transition as permissible immediate
  successors.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-entry-handoff-sentinel-preflight-v16/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-entry-handoff-sentinel-preflight-v16/classification.json.
- Run directory:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-entry-handoff-sentinel-preflight-v16/phase12-handoff-sentinel-v16-20260630T155447Z/.

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
  with no uncommitted Talos changes before task-owned promotion.
- Lab API identity before hardware run: pass; selected-image handoff sentinel
  tree 531cc00d5d4f28696993a3c3852dde5b2fcbc32dee63a9412a472720371d31e0 with
  effective_kernel=kernel_2712.img and selected da591740/kernel_2712.img at
  87,432 bytes.
- Fresh serial cursor and TFTP delta: pass; stable same-cursor TFTP delta
  observed two selected kernel serves and retained the fresh post-power serial
  capture window.
- Restore to predecessor-named snapshot and confirm with lab API: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Result

selected_next_task:
phase12-ssh-live-tcp-selected-image-entry-handoff-boundary-closeout-v16-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
