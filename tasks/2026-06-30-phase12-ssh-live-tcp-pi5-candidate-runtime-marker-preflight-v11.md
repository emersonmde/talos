# Phase 12 SSH Live TCP Pi 5 Candidate Runtime-Marker Preflight V11

Task id: phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v11-20260630

Status: accepted after commit.

Classification: blocked-candidate-tftp-capture.

Evidence level: source/archive identity, lab-controller API identity,
serialized Pi 5 hardware power/output, runtime-marker serial capture, TFTP
capture attempt, restore proof, task-owned JSON evidence, docs build, and diff
checks.

## Goal

Run one serialized Pi 5 candidate preflight after the runtime-marker readiness
helper repair, then either unlock the packet-I/O discriminator or preserve the
first precise blocker.

## Scope Performed

- Promoted the queued v11 Pi 5 runtime-marker preflight after the accepted
  no-runtime-marker source reconciliation selected this exact task.
- Acquired hardwareTestLock before lab publication and Pi 5 power action, then
  restored the lab to the selected control snapshot before release.
- Materialized a nonce-bearing rpi5_ssh_service_smoltcp_runtime_ready archive
  from source commit 55c6920fbe86b233ea82b7d764e18f595b1e50cb.
- Published the reviewed candidate archive, power-cycled once, retained fresh
  serial cursor evidence, route-start/runtime-ready marker evidence, stable
  same-cursor TFTP delta, final pre-restore identity, restore proof, and
  post-restore identity.
- Stopped before packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility, service success, ssh-ready=true, broad shell work, or phase
  transition.

## Terminal Classification

blocked-candidate-tftp-capture.

The selected v11 candidate publication and runtime-marker route both worked:

- candidate source commit:
  55c6920fbe86b233ea82b7d764e18f595b1e50cb;
- candidate archive SHA-256:
  0e50f503d090cbe3c9435a545abcc457ee8c2e7f76270ab2cbc15dbb6fac478c;
- published and final pre-restore tree:
  d0419aa69f7e6dc7600122eeebdb94aa287eab49f863b677252f2ad1e8e3c42e;
- selected expected fetch: da591740/kernel_2712.img, 152,176 bytes,
  SHA-256 d16c7d8dc44d0f8009a92e61f871be7713c360c162170167f5f3a8da379cd4c5;
- serial readiness helper observed both nonce-bearing required markers with
  TALOS_READINESS_REQUIRE_KERNEL_MARKER=false:
  TALOS: ssh-service-smoltcp-runtime-route-start and
  TALOS: ssh-service-smoltcp-runtime-ready;
- restore returned the lab to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z /
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

candidate-capture-ready remains rejected because the retained stable
same-cursor TFTP delta had zero events, so selected fetch identity was not
proved in the same pre-restore TFTP capture window. The runtime-marker serial
route is no longer the first missing fact.

selected_next_task: null.

planningNeeded: true.

planningReason: v11 candidate reached nonce-bearing runtime-marker readiness,
but the helper-owned same-cursor TFTP delta captured zero selected fetch events;
packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility,
service success, ssh-ready=true, broad shell work, and phase transition remain
blocked pending supervisor planning.

## Findings

- fixed: hardwareTestLock serialized candidate publication, power, capture,
  restore, and release.
- fixed: selected candidate root/da591740 kernel identity matched the accepted
  v11 contract in lab API status and final pre-restore identity.
- fixed: the repaired runtime-readiness checker observed the nonce-bearing
  route-start and runtime-ready markers with kernel_main optional.
- blocked: candidate-capture-ready is rejected because the same-cursor TFTP
  delta was stable but contained zero events, so selected fetch identity was
  not captured in-window.
- not-an-issue: the lab restore path returned to the selected a0452458...
  control tree without manual recovery.
- deferred: packet-I/O discriminator, OpenSSH/generated-root retry, remote
  receipt, compatibility, service success, ssh-ready=true, broad shell work,
  and phase transition remain blocked until supervisor planning selects a
  narrower next step.
- removed: generated upload archive and boot tree bytes were deleted from
  target/tmp after publication and metadata retention.

## Evidence Map

- Evidence directory:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v11/candidate-runtime-marker-preflight-v11-20260630T082624Z/.
- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v11/candidate-runtime-marker-preflight-v11-20260630T082624Z/evidence-map.json.
- Analysis:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v11/candidate-runtime-marker-preflight-v11-20260630T082624Z/candidate-preflight-analysis.json.
- Serial primary:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v11/candidate-runtime-marker-preflight-v11-20260630T082624Z/candidate-runtime-marker-preflight-v11-20260630T082624Z-runtime-readiness-primary.json.
- TFTP delta:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v11/candidate-runtime-marker-preflight-v11-20260630T082624Z/tftp-delta-stable-pre-restore.json.
- Restore proof:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v11/candidate-runtime-marker-preflight-v11-20260630T082624Z/restore-snapshot.json and
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v11/candidate-runtime-marker-preflight-v11-20260630T082624Z/post-restore-status.json.

## Redaction Review

Task summary JSON omits raw serial text and peer fields. Raw lab-controller
serial and TFTP artifacts remain task-owned hardware evidence and may include
local lab endpoint fields. Evidence does not retain packet payloads, key
material, SSH/session material, boot artifact bytes, private user data, stable
secret-derived identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no conflicting tracked Talos changes before task-owned evidence.
- Candidate archive build/materialization and static marker/order review:
  pass.
- Lab API GET /status, GET /boot/files, and GET /boot/snapshots before power
  and after restore: pass.
- Fresh serial cursor/completeness diagnostics before power: pass.
- Runtime-marker checker: pass; route-start and runtime-ready markers were
  observed with kernel_main optional per the v11 contract.
- GET /tftp/logs tail cursor and stable same-cursor delta before restore:
  blocked; stable delta contained zero events.
- Restore to named selected-control snapshot and confirm with lab API
  GET /status: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: null.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
