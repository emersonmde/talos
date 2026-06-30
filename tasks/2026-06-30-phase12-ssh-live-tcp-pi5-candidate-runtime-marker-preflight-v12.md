# Phase 12 SSH Live TCP Pi 5 Candidate Runtime-Marker Preflight V12

Task id: phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v12-20260630

Status: accepted after commit.

Classification: blocked-candidate-kernel-not-starting.

Evidence level: source/archive identity, lab-controller API identity,
serialized Pi 5 hardware power/output, repaired pre-restore TFTP capture,
serial marker capture, restore proof, task-owned JSON evidence, docs build, and
diff checks.

## Goal

Run one serialized Pi 5 candidate preflight under the repaired TFTP capture
contract, then either unlock the packet-I/O discriminator or preserve the first
precise blocker.

## Scope Performed

- Promoted the queued v12 Pi 5 runtime-marker preflight after the accepted TFTP
  capture-boundary reconciliation selected this exact task.
- Acquired hardwareTestLock before lab publication and Pi 5 power action, then
  restored the lab to the selected control snapshot before release.
- Materialized a nonce-bearing rpi5_ssh_service_smoltcp_runtime_ready archive
  from source commit 1a199ad3f5f8416af2d5088214c5a6d3bf433433.
- Published the reviewed candidate archive, power-cycled once, retained fresh
  serial cursor evidence, repaired stable same-cursor TFTP delta, final
  pre-restore identity, route-start/runtime-ready marker check, restore proof,
  and post-restore identity.
- Stopped before packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility, service success, ssh-ready=true, broad shell work, or phase
  transition.

## Terminal Classification

blocked-candidate-kernel-not-starting.

The repaired TFTP contract succeeded for candidate identity:

- candidate source commit:
  1a199ad3f5f8416af2d5088214c5a6d3bf433433;
- candidate archive SHA-256:
  054be8c7b6a44e9c221d85121974ff323d4ec91c642e6bc1f081861187ad6042;
- published and final pre-restore tree:
  400bf7c5f4ae49ca484322499c7d2ec06cd7f8f57961241d705f01b823035ca9;
- selected expected fetch: da591740/kernel_2712.img, 152,176 bytes,
  SHA-256 76cf4214a0ced6fc85c107bbc123a703133f72999c112abcb9786ab5018a6050;
- repaired stable same-cursor TFTP delta observed 13 events, including two
  selected da591740/kernel_2712.img serves with matching byte counts;
- final pre-restore identity still showed the candidate tree and selected
  kernel path.

candidate-capture-ready remains rejected because the retained serial window had
firmware NETWORK output but no TALOS: kernel_main, no nonce-bearing
TALOS: ssh-service-smoltcp-runtime-route-start, and no nonce-bearing
TALOS: ssh-service-smoltcp-runtime-ready marker. The first missing fact moved
from TFTP capture to candidate kernel/runtime entry visibility.

selected_next_task:
phase12-ssh-live-tcp-tftp-capture-boundary-closeout-v12-20260630.

planningNeeded: false.

## Findings

- fixed: the repaired TFTP helper captured selected candidate fetch identity
  in-window before restore.
- blocked: candidate-capture-ready is rejected because nonce-bearing
  route-start/runtime-ready markers are absent after the selected fetch proof.
- fixed: the lab restore returned to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z /
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- deferred: packet-I/O discriminator, OpenSSH/generated-root retry, remote
  receipt, compatibility, service success, ssh-ready=true, broad shell work,
  and phase transition remain blocked.

## Evidence Map

- Evidence directory:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v12/candidate-runtime-marker-preflight-v12-20260630T094011Z/.
- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v12/candidate-runtime-marker-preflight-v12-20260630T094011Z/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v12/candidate-runtime-marker-preflight-v12-20260630T094011Z/candidate-run/capture-invariant-summary.json.
- TFTP delta:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v12/candidate-runtime-marker-preflight-v12-20260630T094011Z/candidate-run/tftp-delta-stable-pre-restore.json.
- Marker pair check:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v12/candidate-runtime-marker-preflight-v12-20260630T094011Z/candidate-run/route-start-runtime-ready-marker-check.json.
- Restore proof:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v12/candidate-runtime-marker-preflight-v12-20260630T094011Z/candidate-run/restore-snapshot.json and
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-runtime-marker-preflight-v12/candidate-runtime-marker-preflight-v12-20260630T094011Z/status-after-manual-restore.json.

## Redaction Review

Task summary JSON omits raw serial text and peer fields. Raw lab-controller
serial and TFTP artifacts remain task-owned hardware evidence and may include
local lab endpoint fields. Evidence does not retain packet payloads, key
material, SSH/session material, boot artifact bytes, private user data, stable
secret-derived identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no conflicting tracked Talos changes before task-owned evidence.
- Candidate archive build/materialization and static marker/order review: pass.
- Lab API GET /status, GET /boot/files, and GET /boot/snapshots before power
  and after restore: pass.
- Fresh serial cursor/completeness diagnostics before power: pass.
- Repaired GET /tftp/logs capture contract before restore: pass; stable
  same-cursor delta captured selected da591740/kernel_2712.img fetches.
- Task-owned marker/capture checker: blocked; route-start/runtime-ready nonce
  markers were absent in the retained serial window.
- Restore to named selected-control snapshot and confirm with lab API
  GET /status: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-tftp-capture-boundary-closeout-v12-20260630.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
