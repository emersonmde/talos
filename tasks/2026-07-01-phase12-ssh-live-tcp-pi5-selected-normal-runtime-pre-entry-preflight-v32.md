# Phase 12 SSH Live TCP Pi 5 Selected Normal Runtime Pre-Entry Preflight V32

Task id: phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-entry-preflight-v32-20260701

Status: accepted after serialized Pi 5 validation.

Classification: blocked-selected-normal-runtime-rust-entry-missing.

Evidence level: hardwareTestLock, static archive review, lab-controller API
identity, stable same-cursor TFTP evidence, serial hardware output, known-good
control, unchanged candidate rerun, restore proof, and task-owned JSON
evidence.

## Goal

Run the v31-selected normal-runtime pre-entry discriminator on the Pi 5 and
classify whether the selected Image reaches assembly pre-entry, Rust entry, or
later normal-runtime markers.

## Scope Performed

- Promoted this queued hardware task after v31 accepted and selected it.
- Acquired hardwareTestLock before lab publication, boot mutation, or Pi 5
  power action.
- Published only the v31-selected archive:
  target/tmp/selected-normal-runtime-pre-entry-v31.tar.gz.
- Captured the primary candidate, a known-good control, and an unchanged
  candidate rerun before accepting the marker-missing classification.
- Restored the lab to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z after every run and
  confirmed the final restored tree before releasing hardwareTestLock.

## Hardware Result

Primary candidate: selected-normal-runtime-pre-entry-v32-candidate.

The selected archive served da591740/kernel_2712.img twice at 152,144 bytes
with SHA-256
c169c9553096f3bae24802762f14c03588fc6d6e811b732c8ac6515c47ca8f95. The
fresh serial window retained Raspberry Pi firmware NETWORK output, but retained
zero occurrences of every ordered normal-runtime pre-entry marker:

- TALOS: asm_start
- TALOS: asm_pre_rust_entry
- TALOS: rust_entry
- TALOS: boot info parsed
- TALOS: target init
- TALOS: exceptions ready
- TALOS: kernel_main
- TALOS: ssh-service-smoltcp-runtime-route-start capture-nonce=runtime-marker-route-static source=network-device-smoltcp-runtime
- TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=runtime-marker-route-static

Known-good control:
selected-normal-runtime-pre-entry-v32-known-good-control.

The restored production-timer control served da591740/kernel_2712.img twice at
104,136 bytes and retained rpi5-production-timer-preemption: PASS once,
proving the same capture path could retain a known-good Talos marker.

Unchanged candidate rerun:
selected-normal-runtime-pre-entry-v32-candidate-rerun.

The unchanged v32 candidate again served da591740/kernel_2712.img twice at
152,144 bytes and again retained zero occurrences of every ordered pre-entry
or normal-runtime marker.

## Terminal Classification

blocked-selected-normal-runtime-rust-entry-missing.

First missing fact: the selected 152,144-byte
rpi5_ssh_service_smoltcp_runtime_ready Image is served by TFTP in the primary
run and unchanged rerun, but Pi 5 serial retained no TALOS: asm_start,
TALOS: asm_pre_rust_entry, TALOS: rust_entry, or later ordered
normal-runtime marker.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-pre-entry-closeout-v32-20260701.

planningNeeded: false.

Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, fake command expansion, broad
shell work, and phase transition remain blocked.

## Findings

- fixed: executed the v31 selected pre-entry discriminator under
  hardwareTestLock with selected-byte TFTP evidence and restore proof.
- fixed: ran the required known-good control after primary marker absence; the
  control retained rpi5-production-timer-preemption: PASS.
- fixed: ran the required unchanged candidate rerun after primary marker
  absence; the rerun repeated the selected-byte/no-marker result.
- not-an-issue: the repaired capture helper produced marker_family summaries
  for candidate, known-good control, and rerun.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as immediate successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-entry-preflight-v32/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-entry-preflight-v32/evidence-map.json.
- Primary candidate:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-entry-preflight-v32/lab/candidate/.
- Known-good control:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-entry-preflight-v32/lab/known-good-control/.
- Unchanged candidate rerun:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-pre-entry-preflight-v32/lab/candidate-rerun/.

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
- Lab API status and boot files before publication, after publication, before
  power, final pre-restore, and after restore: pass.
- fresh serial cursor/drain and TFTP cursor before every Pi 5 power action:
  pass.
- stable same-cursor TFTP delta before restore: pass for candidate, control,
  and rerun.
- known-good control and unchanged candidate rerun after primary marker absence:
  pass.
- restore to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirm
  post-restore tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10:
  pass.

Implementation commit: recorded in supervisor state after commit creation.
