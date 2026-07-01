# Phase 12 SSH Live TCP Pi 5 Selected Runtime Phase Marker Preflight V30

Task id: phase12-ssh-live-tcp-pi5-selected-runtime-phase-marker-preflight-v30-20260701

Status: accepted after final validation.

Classification: blocked-selected-runtime-phase-marker-missing.

Evidence level: serialized Pi 5 hardware preflight with hardwareTestLock,
static archive review, lab-controller API identity, stable same-cursor TFTP
evidence, serial hardware output, known-good control, unchanged candidate
rerun, restore proof, and task-owned JSON evidence.

## Goal

Run the v29-selected normal-runtime early-phase marker discriminator on the Pi
5 and classify the earliest retained boundary before kernel_main/runtime
service work.

## Scope Performed

- Promoted this queued hardware task after v29 accepted and selected it.
- Acquired hardwareTestLock before lab publication, boot mutation, or Pi 5
  power action.
- Published only the v29-selected archive:
  target/tmp/selected-runtime-phase-marker-v30.tar.gz.
- Captured the primary candidate, a known-good control, and an unchanged
  candidate rerun before accepting the marker-missing classification.
- Restored the lab to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z after every run.

## Hardware Result

Primary candidate: selected-runtime-phase-marker-v30-candidate.

The selected archive served da591740/kernel_2712.img twice at 152,144 bytes
with SHA-256
665d993ab7c36065fa4810ae09613ed9d92aba30cdd5881e06e23b50b4d25a72. The
fresh serial window retained Raspberry Pi firmware NETWORK output, but retained
zero occurrences of every ordered normal-runtime marker:

- TALOS: rust_entry
- TALOS: boot info parsed
- TALOS: target init
- TALOS: exceptions ready
- TALOS: kernel_main
- TALOS: ssh-service-smoltcp-runtime-route-start capture-nonce=runtime-marker-route-static source=network-device-smoltcp-runtime
- TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=runtime-marker-route-static

Known-good control:
selected-runtime-phase-marker-v30-known-good-control.

The restored production-timer control served da591740/kernel_2712.img twice
at 104,136 bytes and retained rpi5-production-timer-preemption: PASS once,
proving the same capture path could retain a known-good Talos marker.

Unchanged candidate rerun:
selected-runtime-phase-marker-v30-candidate-rerun.

The unchanged v30 candidate again served da591740/kernel_2712.img twice at
152,144 bytes and again retained zero occurrences of every ordered
normal-runtime marker.

## Terminal Classification

blocked-selected-runtime-phase-marker-missing.

First missing fact: selected rpi5_ssh_service_smoltcp_runtime_ready Image was
served by TFTP in the primary run and unchanged rerun, but Pi 5 serial retained
no ordered normal-runtime marker family member; the first missing marker is
TALOS: rust_entry.

selected_next_task:
phase12-ssh-live-tcp-selected-runtime-phase-marker-closeout-v30-20260701.

planningNeeded: false.

Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, fake command expansion, broad
shell work, and phase transition remain blocked.

## Findings

- fixed: executed the v29 selected marker-family discriminator under
  hardwareTestLock with selected-byte TFTP evidence and restore proof.
- fixed: ran the required known-good control and unchanged candidate rerun
  after the primary marker-missing result.
- deferred: the capture bundle helper captured and restored all three runs, but
  its final summary generation failed because the final jq program references
  $marker_family without passing --arg marker_family; task-owned summary JSON
  was generated from retained evidence without changing source in this
  no-source-change hardware task.
- deferred: the selected normal-runtime Image does not retain even
  TALOS: rust_entry; closeout must reconcile this against the accepted v26
  rust_entry/UART10 proof before any packet-I/O/OpenSSH work.
- not-an-issue: lab publication identity, selected TFTP byte counts, final
  pre-restore identity, known-good marker visibility, unchanged rerun, and
  restore proof were captured before acceptance.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as immediate successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-runtime-phase-marker-preflight-v30/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-runtime-phase-marker-preflight-v30/evidence-map.json.
- Triage summary:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-runtime-phase-marker-preflight-v30/triage-summary.json.
- Primary candidate:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-runtime-phase-marker-preflight-v30/lab/candidate/.
- Known-good control:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-runtime-phase-marker-preflight-v30/lab/known-good-control/.
- Unchanged candidate rerun:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-runtime-phase-marker-preflight-v30/lab/candidate-rerun/.

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
  restore, and after restore: pass.
- fresh serial cursor/drain and TFTP cursor before Pi 5 power action: pass.
- stable same-cursor TFTP delta before restore: pass.
- known-good control and unchanged candidate rerun after primary
  marker-missing result: pass.
- restore to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirm
  post-restore tree: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass.
