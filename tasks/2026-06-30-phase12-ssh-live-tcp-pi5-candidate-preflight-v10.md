# Phase 12 SSH Live TCP Pi 5 Candidate Preflight V10

Task id: phase12-ssh-live-tcp-pi5-candidate-preflight-v10-20260630

Status: accepted after commit.

Classification: blocked-candidate-kernel-not-starting.

Evidence level: source/archive identity, lab-controller API identity,
serialized Pi 5 hardware power/output, TFTP and serial capture, restore proof,
task-owned JSON evidence, docs build, and diff checks.

## Goal

Run one serialized Pi 5 live TCP candidate preflight after the accepted v10
candidate-entry preflight contract, then either unlock packet-I/O or preserve
the first precise blocker.

## Scope Performed

- Promoted the queued v10 Pi 5 candidate preflight after the accepted
  candidate-entry contract selected this exact task.
- Acquired hardwareTestLock before lab publication or Pi 5 power action and
  released it only after restore proof showed the lab back on the selected
  a0452458... control tree.
- Materialized a capture-nonce-bearing
  rpi5_ssh_service_smoltcp_runtime_ready archive from source commit
  fb7f371765f4e5022dc56ffa038c5a6c338d5bcd with matching root and
  da591740/selected Pi 5 boot files.
- Published the reviewed candidate archive, power-cycled once, retained fresh
  serial and TFTP cursors, retained same-window TFTP delta, final pre-restore
  candidate identity, restore proof, and post-restore identity.
- Stopped before packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility, service success, ssh-ready=true, broad shell work, or phase
  transition.

## Terminal Classification

blocked-candidate-kernel-not-starting.

The selected v10 candidate publication and fetch path worked:

- candidate source commit:
  fb7f371765f4e5022dc56ffa038c5a6c338d5bcd;
- candidate archive SHA-256:
  d9cd5e672cb6e873d3f61a87a81e2f484f93f36b8a07ff72ddce45b3a99d84f4;
- published and final pre-restore tree:
  67035e440ea9b8cfc555ee55603808bdc5c99f9ef461260c348670705c23667d;
- selected expected fetch: da591740/kernel_2712.img, 152,152 bytes,
  SHA-256 3e260a556cbe39c0fd49c0649ce5ec523491cac77a342e843d1fbd62f23dbd3c;
- stable same-cursor TFTP delta observed 13 events, including two selected
  da591740/kernel_2712.img serves, both at 152,152 bytes;
- restore returned the lab to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z /
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

candidate-capture-ready remains rejected because the retained post-power serial
window contained firmware NETWORK/TFTP output but no nonce-bearing
TALOS: ssh-service-smoltcp-runtime-route-start or
TALOS: ssh-service-smoltcp-runtime-ready marker.

selected_next_task: null.

planningNeeded: true.

planningReason: selected v10 candidate bytes were published and served twice
from da591740/kernel_2712.img, but the retained post-power serial window showed
firmware NETWORK/TFTP output only and did not contain the nonce-bearing
route-start or runtime-ready marker. Packet-I/O, OpenSSH/generated-root retry,
remote receipt, compatibility, service success, ssh-ready=true, broad shell
work, and phase transition remain blocked pending supervisor planning.

## Findings

- fixed: hardwareTestLock serialized candidate publication, power, capture,
  restore, and release.
- fixed: selected candidate root/da591740 kernel identity matched the accepted
  v10 contract and was observed in same-window TFTP evidence.
- blocked: candidate-capture-ready is rejected because nonce-bearing runtime
  markers were absent after power despite selected kernel serves.
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
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-preflight-v10/candidate-preflight-v10-20260630T064353Z/.
- Evidence map:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-preflight-v10/candidate-preflight-v10-20260630T064353Z/evidence-map.json.
- Analysis:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-preflight-v10/candidate-preflight-v10-20260630T064353Z/candidate-preflight-analysis.json.
- Serial primary:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-preflight-v10/candidate-preflight-v10-20260630T064353Z/candidate-preflight-v10-20260630T064353Z-runtime-readiness-primary.json.
- TFTP delta:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-preflight-v10/candidate-preflight-v10-20260630T064353Z/tftp-delta-stable-pre-restore.json.
- Restore proof:
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-preflight-v10/candidate-preflight-v10-20260630T064353Z/restore-snapshot.json and
  tasks/evidence/2026-06-30-phase12-ssh-live-tcp-pi5-candidate-preflight-v10/candidate-preflight-v10-20260630T064353Z/post-restore-status.json.

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
- GET /tftp/logs tail cursor and stable same-cursor delta before restore:
  pass; two 152,152-byte da591740/kernel_2712.img serves observed.
- Task-owned marker/capture checker: pass as blocker evidence; selected TFTP
  fetches were present, route-start/runtime-ready markers were absent from
  serial.
- Restore to named selected-control snapshot and confirm with lab API
  GET /status: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Result

selected_next_task: null.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
