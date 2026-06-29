# Phase 12 SSH Live TCP Pi 5 Candidate Preflight V6

Task id: phase12-ssh-live-tcp-pi5-candidate-preflight-v6-20260629

Status: accepted after commit.

Classification: blocked-candidate-kernel-not-starting.

Evidence level: source/archive identity, lab-controller API identity,
serialized Pi 5 hardware power/output, helper-owned capture/restore window,
run-ownership checker output, task-owned JSON evidence, docs build, and diff
checks.

## Goal

Run one serialized candidate preflight after the run-ownership repair, and
either prove candidate-capture-ready or preserve the first missing fact before
packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility,
service success, ssh-ready=true, broad shell work, or phase transition.

## Scope Performed

- Promoted the mechanically unblocked v6 candidate preflight task and acquired
  hardwareTestLock before lab or hardware action.
- Recorded source identity at commit
  b3ed0fe1af34cb5c037cde43c05ec76f143fad55 and confirmed the accepted runtime
  and proof-contract commits are ancestors.
- Built and reviewed a nonce-bearing Pi 5 candidate archive with
  da591740/kernel_2712.img present.
- Published the reviewed candidate archive and retained post-publication lab
  identity for tree
  18e467bf70316e41fa4232d3b7b3ea328160c3c77befa799e00dffe707fa603e and
  87,432-byte selected kernel.
- Ran the capture helper in the foreground to completion and let it own restore
  to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z.
- Ran the repaired v5 capture-window checker against the retained helper-owned
  window.
- Stopped before packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility, service success, ssh-ready=true, broad shell work, or phase
  transition.

## Terminal Classification

blocked-candidate-kernel-not-starting.

The candidate archive and publication identity were established, and the v6
helper-owned capture window was clean:

- expected selected fetch: da591740/kernel_2712.img, 87,432 bytes;
- TFTP stable same-cursor delta observed two selected fetches, both 87,432
  bytes;
- final pre-restore identity still reported candidate tree
  18e467bf70316e41fa4232d3b7b3ea328160c3c77befa799e00dffe707fa603e;
- helper completion metadata reports helper_run_completed=true, completed_at,
  completion_event_count=8, final-pre-restore before restore, and TFTP delta
  before restore;
- post-restore lab identity returned to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The candidate is not candidate-capture-ready because serial readiness did not
show the nonce-bearing runtime marker after power. The checker rejected the run
with:

- run-unique-capture-nonce-not-present-after-power
- required-marker-not-present-after-power

selected_next_task: null.

planningNeeded: true.

planningReason: v6 retained a clean helper-owned capture/restore window, but
serial readiness did not show the nonce-bearing
TALOS: ssh-service-smoltcp-runtime-ready marker after power; packet-I/O and
OpenSSH remain blocked pending supervisor planning.

## Findings

- fixed: the v6 run avoided the v5 manual-restore race; the helper owned
  TFTP, final pre-restore identity, restore, post-restore identity, and
  completion metadata.
- fixed: candidate source/archive identity, accepted ancestry, archive review,
  post-publication identity, hardware lock lifecycle, restore proof, and
  redaction were retained.
- blocked: candidate-capture-ready is rejected because the nonce-bearing
  TALOS ssh-service-smoltcp runtime marker was absent after power.
- deferred: packet-I/O discriminator remains deferred until a future explicit
  task records candidate-capture-ready or supervisor selects a narrower
  discriminator for the missing runtime marker.
- not-an-issue: TFTP selected-byte and final pre-restore identity now match the
  candidate, so the previous restored-control contamination is not the blocker.
- removed: the generated upload archive was removed from target/tmp after
  metadata retention; boot artifact bytes are not checked in.

## Evidence Map

- Evidence directory:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v6/candidate-preflight-v6-20260629T184937Z/.
- Classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v6/candidate-preflight-v6-20260629T184937Z/classification.json.
- Evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v6/candidate-preflight-v6-20260629T184937Z/evidence-map.json.
- Preflight summary:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v6/candidate-preflight-v6-20260629T184937Z/preflight-summary.json.
- Candidate archive metadata:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v6/candidate-preflight-v6-20260629T184937Z/candidate-identity/archive-metadata.json.
- Candidate capture summary:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v6/candidate-preflight-v6-20260629T184937Z/candidate-run/capture-invariant-summary.json.
- Capture-window checker:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v6/candidate-preflight-v6-20260629T184937Z/candidate-run/capture-window-v5-check.json.

## Redaction Review

Task-owned JSON evidence was scrubbed to replace raw serial text, TFTP
peer/address fields, MAC fields, and raw log lines with redaction placeholders.
Durable evidence retains task ids, source/archive metadata, tree hashes, byte
counts, cursor/capture classifications, validation commands/results, and
metadata-only counters. It does not retain packet payload contents, key
material, session material, boot artifact bytes, private user data, stable
secret-derived identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- Candidate archive materialization: pass; rpi5-boot-tree exited 0.
- Candidate archive review: pass; rpi5-archive-review exited 0.
- Lab API GET /status, GET /boot/files, and GET /boot/snapshots before power
  and after restore: pass.
- Fresh serial cursor and TFTP cursor: pass; retained before power.
- Foreground capture helper: pass; exited 0 and retained post-restore identity.
- Repaired capture-window checker: pass as blocker evidence; exited 1 with
  run-unique-capture-nonce-not-present-after-power and
  required-marker-not-present-after-power.
- Restore to named selected-control snapshot: pass; final GET /status reports
  tree_hash=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
  and effective_kernel=kernel_2712.img.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass; html backend wrote book/ and
  emitted only the large search-index warning.
- git diff --cached --check: pending before commit.

## Result

selected_next_task: null.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
