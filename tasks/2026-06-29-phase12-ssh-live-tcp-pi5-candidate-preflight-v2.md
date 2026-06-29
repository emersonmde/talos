# Phase 12 SSH Live TCP Pi 5 Candidate Preflight V2

Task id: phase12-ssh-live-tcp-pi5-candidate-preflight-v2-20260629

Status: accepted after commit.

Classification: blocked-known-good-control.

Evidence level: lab-controller API identity, archive inspection, serial
hardware boot/output, stable TFTP delta, restore proof, task-owned JSON
evidence, docs build, and diff checks.

## Goal

After the POSIX timeout source repair, rerun the accepted Pi 5 live TCP
candidate/capture preflight contract far enough to prove candidate identity,
known-good capture health, candidate capture, and candidate rerun before any
packet-I/O discriminator.

## Scope Performed

- Acquired hardwareTestLock for the task-owned Pi 5 evidence window.
- Rechecked source identity at commit
  2d6f2938e2187d8eec0905eecc0ddc25d2c1cde7 and confirmed the accepted
  runtime commits are ancestors.
- Built and reviewed a replacement Pi 5 boot archive with
  da591740/kernel_2712.img present.
- Retained baseline lab identity from GET /status, GET /boot/files, and
  GET /boot/snapshots.
- Ran the known-good control from a fresh serial cursor and TFTP cursor.
- Published the reviewed candidate archive once and retained one candidate
  capture bundle after the known-good control failed to establish the required
  capture/readiness path.
- Stopped before candidate rerun or any packet-I/O discriminator because the
  retained control/candidate evidence did not satisfy the proof contract.
- Restored the lab to abcontrol-secondary-workload-pre-20260524T231449Z.

## Terminal Classification

blocked-known-good-control.

The known-good control retained a stable TFTP delta, but the serial readiness
artifact used the saturated-cursor direct-read fallback and did not contain the
required production success marker. The known-good readiness classifier rejected
the control with:

- missing-production-success-marker
- missing-or-unstable-boot-identity-join

The candidate publication and first candidate capture were also retained, but
the candidate bundle rejected the identity join with tftp-expected-fetch-byte-
mismatch, final-pre-restore-tree-mismatch, final-pre-restore-selected-tree-
mismatch, and final-pre-restore-expected-fetch-byte-mismatch. That follow-up
evidence does not override the first missing fact: the known-good control did
not establish a trustworthy capture/readiness path. No candidate-capture-ready,
candidate rerun, packet-I/O discriminator, OpenSSH retry, generated-root retry,
remote receipt, compatibility, service success, ssh-ready=true, runtime russh
adoption, fake command expansion, broad shell work, or phase transition is
accepted.

## Findings

- fixed: acquired hardwareTestLock for the task-owned lab evidence window and
  restored the lab to the named baseline before release.
- fixed: source identity and accepted runtime ancestry were retained before the
  candidate archive review.
- fixed: replacement archive review passed with expected
  da591740/kernel_2712.img, 87432-byte kernel, archive byte count, archive
  SHA-256, and boot-tree manifest hash retained.
- fixed: known-good control retained fresh serial cursor, TFTP cursor, power
  cycle, primary serial readiness artifact, stable TFTP delta, classifier
  output, and restore identity.
- blocked: known-good control did not establish the required readiness/capture
  path.
- blocked: the retained first candidate capture also failed the identity join,
  so candidate-capture-ready remains blocked.
- not-an-issue: candidate rerun evidence is absent because the known-good
  control and first candidate identity join failed before rerun could be
  trusted under the accepted proof contract.
- removed: the generated candidate upload archive was removed from target/tmp
  after metadata retention; boot artifact bytes are not checked in.

## Evidence Map

- Evidence directory:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v2/candidate-preflight-v2-20260629T111759Z/.
- Classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v2/candidate-preflight-v2-20260629T111759Z/classification.json.
- Evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v2/candidate-preflight-v2-20260629T111759Z/evidence-map.json.
- Preflight summary:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v2/candidate-preflight-v2-20260629T111759Z/preflight-summary.json.
- Archive metadata:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v2/candidate-preflight-v2-20260629T111759Z/candidate-identity/archive-metadata.json.
- Known-good control:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v2/candidate-preflight-v2-20260629T111759Z/known-good-control/known-good-control-runtime-readiness-primary.json,
  known-good-control/tftp-delta-stable-pre-restore.json, and
  known-good-control/known-good-readiness-v3-classification.json.
- Candidate run:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v2/candidate-preflight-v2-20260629T111759Z/candidate-run/capture-invariant-summary.json.

## Redaction Review

Durable evidence records task ids, source commit, archive path labels, archive
byte counts, archive digests, tree hashes, public boot config fields, cursor
offsets, helper classifications, validation commands/results, and
metadata-only counters. It does not retain peer identifiers, addresses, packet
payload contents, key material, session material, boot artifact bytes, private
user data, stable secret-derived identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- Lab API GET /status, GET /boot/files, and GET /boot/snapshots before
  candidate action: pass.
- Candidate archive review: pass; archive SHA-256
  bad50bef561dc209702116d9dd9783e0a5a7da98ab8f1fcaf7e79cc2e1142051,
  archive bytes 175342, expected fetch da591740/kernel_2712.img, expected
  fetch bytes 87432, kernel SHA-256
  516b0014eaead2a090779fba7bd8ea4da630f71e923e01182f0570aa9fc2de43.
- Fresh serial cursor captured before known-good control: pass; cursor
  retained in task-owned evidence.
- GET /tftp/logs cursor delta captured for the known-good window: pass; stable
  TFTP helper exited 0.
- Known-good control: blocked; retained primary serial readiness exited 1 and
  classifier exited 1 with the rejection reasons listed above.
- Candidate run: blocked; capture helper exited 0 but the identity join rejected
  the bundle with TFTP byte and final-pre-restore identity mismatches.
- Candidate rerun: not completed because the known-good control and first
  candidate identity join failed before rerun could be trusted.
- Restore to named snapshot: pass; final GET /status reports
  tree_hash=6ead8933b3287cddaf08ddcdbda7d4c770b365658ef1d0ad80a5dd0cf323f3ef
  and effective_kernel=kernel_2712.img.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: null.

planningNeeded: true.

planningReason: known-good control did not establish the lab capture/readiness
path, and the retained first candidate capture failed the identity join;
supervisor planning is required before any further candidate preflight,
packet-I/O discriminator, OpenSSH retry, generated-root retry, remote receipt,
compatibility, service success, ssh-ready=true, runtime russh adoption, fake
command expansion, broad shell work, or phase transition.

Commit: recorded in talos-supervisor-state.json after final commit.
