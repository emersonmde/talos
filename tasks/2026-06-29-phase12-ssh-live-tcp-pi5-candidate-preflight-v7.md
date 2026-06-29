# Phase 12 SSH Live TCP Pi 5 Candidate Preflight V7

Task id: phase12-ssh-live-tcp-pi5-candidate-preflight-v7-20260629

Status: accepted after commit.

Classification: blocked-candidate-identity.

Evidence level: source/archive identity, lab-controller API identity,
serialized hardware lock lifecycle, lab publication/restore evidence,
task-owned JSON evidence, docs build, and diff checks.

## Goal

Run one serialized Pi 5 candidate preflight after the runtime marker route
repair, then either unlock packet-I/O or preserve the first precise blocker.

## Scope Performed

- Promoted the queued v7 candidate preflight after
  runtime-marker-route-ready was accepted.
- Acquired hardwareTestLock before any lab/API or hardware action.
- Built a nonce-bearing runtime-marker-route archive from commit
  8cba0860d1670f45484c5f5cba86fbf62acf2ce0 and reviewed the marker route.
- Published the reviewed archive to the lab, retained candidate identity, and
  ran the foreground capture helper.
- The helper failed closed before power because the expected selected fetch
  path was absent, so no power-cycle, serial window, TFTP delta, packet-I/O,
  OpenSSH/generated-root retry, remote receipt, compatibility, service success,
  ssh-ready=true, broad shell work, or phase transition was performed.
- Restored the lab to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z after the helper abort
  and retained restore proof.

## Terminal Classification

blocked-candidate-identity.

The accepted runtime-marker-route source/archive path built and reviewed
successfully, and lab publication reported selected tree
3d85d4aca0303a73beb577129aea36cc49d38cedb959f783646b0b63c9d3dfa0 with
kernel_2712.img as the effective kernel. The first missing fact was that the
published candidate boot tree exposed root kernel_2712.img/kernel8.img entries
but did not expose the serial-prefixed da591740/kernel_2712.img path required by
the accepted candidate capture contract. The foreground helper rejected the run
at preflight identity before power.

selected_next_task: null.

planningNeeded: true.

planningReason: repair or reconcile the selected Pi 5 fetch path before another
candidate preflight, packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility, service success, ssh-ready=true, broad shell work, or phase
transition.

## Findings

- fixed: hardwareTestLock was acquired before lab action and released only
  after restore evidence was retained.
- fixed: the v7 archive/static marker review proved the nonce-bearing runtime
  marker route was present in the generated candidate kernel.
- fixed: the failed helper run did not power-cycle the Pi and did not collect
  post-restore/control evidence as candidate pre-restore proof.
- blocked: candidate-capture-ready is rejected because the selected lab boot
  tree did not expose da591740/kernel_2712.img under the accepted capture
  expected_fetch path.
- deferred: packet-I/O discriminator remains deferred until a future explicit
  task restores candidate identity and records candidate-capture-ready.
- not-an-issue: the lab restore path returned to the a0452458... control tree
  after the helper abort.
- removed: the generated upload archive was removed from target/tmp after
  metadata retention; boot artifact bytes are not checked in.

## Evidence Map

- Evidence directory:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v7/candidate-preflight-v7-20260629T202403Z/.
- Classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v7/candidate-preflight-v7-20260629T202403Z/classification.json.
- Evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v7/candidate-preflight-v7-20260629T202403Z/evidence-map.json.
- Preflight summary:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v7/candidate-preflight-v7-20260629T202403Z/preflight-summary.json.
- Candidate archive metadata:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v7/candidate-preflight-v7-20260629T202403Z/candidate-identity/archive-metadata.json.
- Candidate preflight identity:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v7/candidate-preflight-v7-20260629T202403Z/candidate-run/preflight-identity.json.
- Restore summary:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v7/candidate-preflight-v7-20260629T202403Z/restore-summary.json.

## Redaction Review

The helper stopped before power, so no fresh serial window, TFTP delta, packet
payload, key material, session material, or boot artifact bytes were retained.
Durable evidence keeps task ids, source/archive metadata, tree hashes, byte
counts, preflight identity, restore status, validation outputs, and
claim-boundary metadata only.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- Candidate archive materialization: pass; rpi5-boot-tree exited 0.
- Candidate archive/static marker review: pass; runtime marker route tokens and
  nonce were found.
- Lab API GET /status, GET /boot/files, and GET /boot/snapshots before action:
  pass.
- Foreground capture helper: pass as blocker evidence; exited 1 before power
  with preflight-staging-publication-mismatch because
  da591740/kernel_2712.img was absent.
- Restore to named selected-control snapshot: pass; final GET /status reports
  tree_hash=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
  and effective_kernel=kernel_2712.img.
- Capture-window checker: pass as blocker evidence; exited 1 because the helper
  did not progress past preflight identity or complete a capture window.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Result

selected_next_task: null.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
