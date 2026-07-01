# Phase 12 SSH Live TCP Pi 5 Selected Runtime Continuation Preflight V28

Task id: phase12-ssh-live-tcp-pi5-selected-runtime-continuation-preflight-v28-20260701

Status: accepted after commit.

Classification: blocked-selected-runtime-continuation-marker-missing.

Evidence level: serialized Pi 5 hardware preflight with selected-tree identity,
stable same-cursor TFTP selected-byte evidence before restore, fresh serial
capture, known-good control, unchanged candidate rerun, final pre-restore
identity, restore proof, task-owned JSON evidence, docs build, and diff checks.

## Goal

Run the v27 selected normal-runtime continuation archive on the Pi 5 and decide
whether selected Image execution continues past rust_entry into the normal
runtime marker family.

## Scope Performed

- Promoted the queued v28 hardware preflight after v27 accepted the exact
  selected normal-runtime continuation contract.
- Acquired hardwareTestLock before lab publication, boot mutation, and Pi 5
  power action.
- Re-materialized and reviewed the
  rpi5_ssh_service_smoltcp_runtime_ready archive before publication.
- Published the selected archive and captured selected-tree identity, fresh
  serial/TFTP cursors, post-power serial output, stable TFTP delta, final
  pre-restore identity, restore proof, and post-restore identity.
- Because the primary helper classified serial freshness as not proved when the
  marker was absent, ran the required known-good control and unchanged
  candidate rerun before accepting the final classification.

## Hardware Result

Primary run: selected-runtime-continuation-v28-20260701T021454Z.

The published selected archive retained the v27 contract:

- selected path: da591740/kernel_2712.img;
- selected kernel byte count: 152,144;
- selected kernel SHA-256:
  665d993ab7c36065fa4810ae09613ed9d92aba30cdd5881e06e23b50b4d25a72;
- expected runtime marker:
  TALOS: ssh-service-smoltcp-runtime-ready
  capture-nonce=runtime-marker-route-static.

The primary run served da591740/kernel_2712.img twice at 152,144 bytes and
restored the lab to
phase12-ssh-v10-openssh-clean-pre-20260624T074100Z. The serial window did not
retain TALOS: kernel_main or the contracted runtime marker.

Known-good control:
selected-runtime-continuation-v28-known-good-control-20260701T021947Z.

The control retained rpi5-production-timer-preemption: PASS once with no
identity-join rejection reasons, proving the hardware serial capture path could
retain a known-good marker in the same run family.

Unchanged candidate rerun:
selected-runtime-continuation-v28-rerun-20260701T021947Z.

The unchanged candidate rerun again served da591740/kernel_2712.img twice at
152,144 bytes and restored to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. It again
retained no TALOS: kernel_main and no contracted runtime marker.

## Terminal Classification

blocked-selected-runtime-continuation-marker-missing.

First missing fact: selected rpi5_ssh_service_smoltcp_runtime_ready Image is
served by TFTP, but Pi 5 serial does not reach kernel_main or the contracted
ssh-service-smoltcp runtime marker.

selected_next_task:
phase12-ssh-live-tcp-selected-runtime-continuation-closeout-v28-20260701.

planningNeeded: false.

Packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility/service readiness, ssh-ready=true, broad shell work,
fake/kernel-backed command expansion, and phase transition remain blocked.

## Findings

- fixed: executed the v27 selected normal-runtime continuation archive under
  hardwareTestLock with serialized Pi 5 capture-chain evidence.
- fixed: ran required triage after the first serial-freshness mismatch:
  known-good control retained its PASS marker, and the unchanged candidate
  rerun again served the selected image twice.
- deferred: the contracted selected normal-runtime marker was not retained;
  closeout must preserve the first missing post-rust_entry fact before any
  packet-I/O/OpenSSH work.
- not-an-issue: selected-byte service, final pre-restore identity, control
  marker visibility, unchanged candidate rerun, and restore proof were checked
  before accepting the marker-missing classification.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as immediate successors.

## Evidence Map

- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-runtime-continuation-preflight-v28/evidence-map.json.
- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-runtime-continuation-preflight-v28/classification.json.
- Triage summary:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-runtime-continuation-preflight-v28/triage-summary.json.
- Primary hardware run directory:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-runtime-continuation-preflight-v28/selected-runtime-continuation-v28-20260701T021454Z/.
- Known-good control directory:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-runtime-continuation-preflight-v28/selected-runtime-continuation-v28-known-good-control-20260701T021947Z/.
- Unchanged candidate rerun directory:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-runtime-continuation-preflight-v28/selected-runtime-continuation-v28-rerun-20260701T021947Z/.

## Redaction Review

Task-owned evidence retains task ids, run labels, hashes, byte counts, marker
names, classifications, validation outcomes, selected-tree hashes, and
redacted lab status metadata. Raw serial text, raw TFTP peer/log-line fields,
packet payloads, SSH/session/key material, boot artifact bytes, private data,
and stable secret-derived identifiers were not retained.

## Validation

- git status --short --branch before lab action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- static archive/image review before publication: pass.
- Lab API status and boot files before publication, before power action, final
  pre-restore, and after restore: pass.
- fresh serial cursor/drain before Pi 5 power action: pass.
- GET /tftp/logs cursor delta before restore: pass.
- known-good control and unchanged candidate rerun after primary
  serial-freshness mismatch: pass.
- restore to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirm
  post-restore tree: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

Commit: recorded in talos-supervisor-state.json after final commit.
