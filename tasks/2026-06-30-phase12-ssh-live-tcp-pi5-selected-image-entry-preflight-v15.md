# Phase 12 SSH Live TCP Pi 5 Selected-Image Entry Preflight V15

Task id: phase12-ssh-live-tcp-pi5-selected-image-entry-preflight-v15-20260630

Status: accepted after commit.

Classification: blocked-selected-image-entry.

Evidence level: serialized Pi 5 hardware preflight with lab-controller
identity, run-unique repaired minimal entry-control archive review, fresh serial
cursor, repaired same-cursor TFTP delta, final pre-restore identity, restore
proof, local JSON classification, docs build, and diff checks.

## Goal

Execute the repaired v15 selected-image entry preflight selected by
phase12-ssh-live-tcp-selected-image-entry-invariant-reconciliation-v15-20260630.

## Scope Performed

- Promoted this queued hardware task after the v15 source repair accepted
  selected-image-entry-source-repair-ready and selected this exact task.
- Acquired hardwareTestLock before lab publication, Pi 5 power action, and
  restore-affecting lab operations.
- Built and published one run-unique repaired rpi5_minimal_entry_control
  archive with capture nonce phase12-entry-preflight-v15-20260630T141202Z.
- Power-cycled the Pi 5, captured fresh serial and TFTP windows, recorded final
  pre-restore identity, restored to the predecessor-named baseline, and released
  the hardware lock after restore proof.
- Classified the predecessor-required selected-image entry marker ladder.
- Stopped before packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility claim, service success claim, ssh-ready=true, fake/kernel-backed
  command expansion, broad shell work, or phase transition.

## Terminal Classification

blocked-selected-image-entry.

The repaired run phase12-entry-preflight-v15-20260630T141202Z retained selected
da591740/kernel_2712.img identity before hardware action and before restore.
The run-unique selected kernel was 52,848 bytes with SHA-256
f68905fc306a360f33c0347c0b23c7c053451491fdc9ec0c5c32169230da5615, and the
published tree hash was
4380329ce566e557569cb1d6ad9844a2eb6d10ae8ca742e1fc4d78b6617deaab.

The repaired same-cursor TFTP delta was stable and observed two selected
kernel serves with the expected 52,848-byte count. The serial window retained
firmware NETWORK output but did not retain TALOS: asm_start,
TALOS: asm_pre_rust_entry, TALOS: rust_entry, TALOS: boot info parsed,
TALOS: target init, TALOS: exceptions ready, TALOS: kernel_main, the
nonce-bearing TALOS: minimal-entry-control-ready marker, or the run nonce. Final
pre-restore identity stayed on the repaired minimal entry-control tree, and
restore to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z succeeded with
post-restore tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

selected_next_task:
phase12-ssh-live-tcp-selected-image-entry-boundary-closeout-v15-20260630.

planningNeeded: false.

## Marker Classification

- Firmware NETWORK: present, 2 occurrences.
- TALOS: asm_start: absent.
- TALOS: asm_pre_rust_entry: absent.
- TALOS: rust_entry: absent.
- TALOS: boot info parsed: absent.
- TALOS: target init: absent.
- TALOS: exceptions ready: absent.
- TALOS: kernel_main: absent.
- TALOS: minimal-entry-control-ready: absent.
- capture-nonce=phase12-entry-preflight-v15-20260630T141202Z: absent.

The run is not classified as blocked-selected-image-identity,
blocked-selected-image-tftp-capture, blocked-restore, or
inconclusive-with-required-discriminator because selected fetch identity, stable
TFTP byte agreement, final pre-restore identity, and restore proof were
decisive. The known-good control branch of inconclusive triage was therefore
not required.

## Findings

- fixed: executed the one predecessor-selected serialized Pi 5 repaired
  selected-image entry preflight and recorded the hardware lock lifecycle.
- fixed: proved current-tree selected fetch for the repaired minimal
  entry-control archive with stable same-cursor TFTP byte agreement and final
  pre-restore identity.
- fixed: classified the repaired assembly-entry, Rust phase-line, kernel_main,
  and minimal-entry-control marker set.
- not-an-issue: the run-unique nonce changed the selected kernel byte count
  from the predecessor's static materialization; this task recorded its own
  archive and selected-kernel identity before hardware action.
- deferred: v15 closeout must reconcile the first missing selected-image entry
  fact before any packet-I/O or OpenSSH/generated-root path can resume.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt, service
  readiness, and phase transition as permissible immediate successors.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-entry-preflight-v15/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-entry-preflight-v15/classification.json.
- Run directory:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-selected-image-entry-preflight-v15/phase12-entry-preflight-v15-20260630T141202Z/.

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
- Lab API identity before hardware run: pass; repaired selected-image tree
  4380329ce566e557569cb1d6ad9844a2eb6d10ae8ca742e1fc4d78b6617deaab with
  effective_kernel=kernel_2712.img and selected da591740/kernel_2712.img at
  52,848 bytes.
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
phase12-ssh-live-tcp-selected-image-entry-boundary-closeout-v15-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
