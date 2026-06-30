# Phase 12 SSH Live TCP Pi 5 Selected-Image Boot Contract Discriminator V18

Task id: phase12-ssh-live-tcp-pi5-selected-image-boot-contract-discriminator-v18-20260630

Status: accepted after commit.

Classification: blocked-selected-image-handoff-after-boot-contract-discriminator.

Evidence level: serialized Pi 5 hardware discriminator with lab-controller
identity, selected-image boot-contract archive review, fresh serial/TFTP
capture, stable same-cursor TFTP byte agreement, final pre-restore identity,
restore proof, task-owned JSON classification, docs build, and diff checks.

## Goal

Execute the v18 selected-image boot/Image handoff discriminator selected by
phase12-ssh-live-tcp-selected-image-boot-contract-reconciliation-v17-20260630.

## Scope Performed

- Promoted this queued hardware task after v17 accepted
  selected-image-boot-contract-discriminator-ready and selected this exact
  task.
- Acquired hardwareTestLock before lab publication, Pi 5 power action, and
  restore-affecting lab operations.
- Rematerialized the predecessor-selected
  rpi5_selected_image_boot_contract_discriminator archive from source commit
  9b6f1654d85a73b4f5b07c89c41cfa99230a9c9f because the prior non-published
  workspace tmp archive had been cleaned. The rematerialized archive hash is
  48f7ee7506bd07521b3813a65541767527f1180e521665fb98c043daab8b8e99; the
  hardware contract stayed bound to the exact selected kernel bytes:
  da591740/kernel_2712.img at 87,432 bytes with SHA-256
  fb501f7374888158c60f090b3cc0805f8fda97d98fd18e966c307310b5c00753.
- Published the rematerialized archive, power-cycled the Pi 5, captured serial
  and TFTP evidence, recorded final pre-restore identity, restored to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z, and released the
  hardware lock after restore proof.
- Stopped before packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility claim, service success claim, ssh-ready=true, minimal-entry
  route repair, fake/kernel-backed command expansion, broad shell work, or
  phase transition.

## Terminal Classification

blocked-selected-image-handoff-after-boot-contract-discriminator.

Run phase12-boot-contract-v18-20260630T1709Z published tree
61811ab93aafa897cf5a0c937bf5485b0abdd4f7c14d770d399393d98f3866c9. The selected
da591740/kernel_2712.img was 87,432 bytes with SHA-256
fb501f7374888158c60f090b3cc0805f8fda97d98fd18e966c307310b5c00753. The stable
same-cursor TFTP delta observed two selected da591740/kernel_2712.img serves,
and both matched the expected selected kernel size. Final pre-restore identity
stayed on tree 61811ab93aafa897cf5a0c937bf5485b0abdd4f7c14d770d399393d98f3866c9,
and restore to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z succeeded
with post-restore tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The fresh serial window retained firmware NETWORK output but did not retain
the predecessor-named TALOS: boot-contract-v18 marker, TALOS: kernel_main, or
any later Talos marker. Because selected fetch identity, stable TFTP byte
agreement, final pre-restore identity, serial freshness, and restore proof
were decisive, the known-good control branch of inconclusive triage was not
required.

selected_next_task:
phase12-ssh-live-tcp-selected-image-boot-contract-discriminator-closeout-v18-20260630.

planningNeeded: false.

## Marker Classification

- Firmware NETWORK: present.
- TALOS: boot-contract-v18: absent.
- TALOS: kernel_main: absent.

The run is not classified as selected-image-handoff-entry-reached because the
repeated compact _start marker was absent. It is not classified as
inconclusive-selected-image-handoff because identity, TFTP, final identity,
serial freshness, and restore proof were decisive with no rejection reasons in
the capture identity join.

## Findings

- fixed: executed the one predecessor-selected serialized Pi 5 selected-image
  boot-contract discriminator and recorded the hardware lock lifecycle.
- fixed: proved current selected fetch identity for the boot-contract
  discriminator archive with stable same-cursor TFTP byte agreement and final
  pre-restore identity.
- fixed: classified the absence of a repeated compact _start marker below
  BSS clear, stack setup, rust_entry, kernel_main, networking, packet I/O,
  OpenSSH, or shell work.
- not-an-issue: rematerializing the non-published gzip archive changed the tar
  archive SHA-256 but preserved the source commit, helper, marker contract,
  kernel SHA-256, kernel byte count, Image header fields, and selected/root
  kernel equality that define the hardware handoff discriminator.
- deferred: v18 closeout must reconcile the blocked firmware-to-selected-image
  handoff boundary before any packet-I/O, OpenSSH/generated-root retry, remote
  receipt, compatibility/service readiness, minimal-entry route repair, broad
  shell work, or phase transition.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt, service
  readiness, minimal-entry route repair, broad shell work, and phase transition
  as permissible immediate successors.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-boot-contract-discriminator-v18/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-boot-contract-discriminator-v18/classification.json.
- Run directory:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-boot-contract-discriminator-v18/phase12-boot-contract-v18-20260630T1709Z/.

## Redaction Review

This task summary retains task ids, commit ids, path labels, hashes, byte
counts, marker labels, classifications, validation command results, and
selected successor metadata. It omits raw serial text, raw TFTP peer/log-line
fields, packet payloads, SSH keys/session material, boot artifact bytes,
private user data, stable secret-derived identifiers, and unnecessary hardware
data. Raw lab-controller serial/TFTP endpoint artifacts are retained only
under task-owned hardware evidence.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned promotion.
- Lab API candidate identity before hardware run: pass through the helper's
  pre-power root-endpoint/status/boot-files capture; GET / was recorded with
  endpoint fallback metadata and /boot/files/status provided the selected tree
  identity before power.
- Fresh serial cursor and TFTP delta: pass; stable same-cursor TFTP delta
  observed two selected kernel serves and retained the fresh post-power serial
  capture window.
- Restore to predecessor-named baseline and confirm with lab API: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Result

selected_next_task:
phase12-ssh-live-tcp-selected-image-boot-contract-discriminator-closeout-v18-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
