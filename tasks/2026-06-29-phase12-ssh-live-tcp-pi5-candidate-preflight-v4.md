# Phase 12 SSH Live TCP Pi 5 Candidate Preflight V4

Task id: phase12-ssh-live-tcp-pi5-candidate-preflight-v4-20260629

Status: accepted after commit.

Classification: inconclusive-with-required-discriminator.

Evidence level: source/archive identity, lab-controller API identity, serialized
Pi 5 hardware boot/output, stable TFTP delta, restore proof, task-owned JSON
evidence, docs build, and diff checks.

## Goal

Run the repaired known-good-control successor candidate preflight far enough to
prove or precisely block candidate capture readiness before any packet-I/O
discriminator, OpenSSH/generated-root retry, remote receipt, compatibility,
service success, ssh-ready=true claim, broad shell work, or phase transition.

## Scope Performed

- Promoted the mechanically unblocked candidate preflight v4 task and acquired
  hardwareTestLock.
- Recorded source identity at commit
  3f7548cd71b9a7a1130e2edabf536b7c73499472 and confirmed the accepted runtime
  commits are ancestors.
- Built and reviewed a Pi 5 boot archive with da591740/kernel_2712.img present.
- Published the reviewed candidate archive and retained post-publication lab
  identity.
- Ran one candidate power cycle from fresh serial and TFTP cursors.
- Restored the lab to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirmed the
  a0452458... selected-control identity after restore.
- Stopped before any packet-I/O discriminator or OpenSSH/generated-root retry.

## Terminal Classification

inconclusive-with-required-discriminator.

Candidate archive identity was established: archive SHA-256
19531e484284f5fd7da29ca87102298ffd8c2d0ed916dcd6543e9c008c794d57,
archive bytes 175336, expected fetch da591740/kernel_2712.img, expected fetch
bytes 87432, and post-publication tree
18e467bf70316e41fa4232d3b7b3ea328160c3c77befa799e00dffe707fa603e.

The hardware capture is not accepted as candidate-capture-ready because the
capture window became contaminated: the original capture bundle continued
through restore while manual completion used the same saved cursors. The
retained TFTP/final-identity evidence therefore labels the restored
a0452458... control tree and 104136-byte kernel entries instead of a clean
pre-restore candidate window. The first missing fact is a clean candidate
pre-restore capture window after publication and before restore.

No packet-I/O discriminator, OpenSSH/generated-root retry, remote receipt,
compatibility, service success, ssh-ready=true, runtime russh adoption, fake
command expansion, broad shell work, or phase transition is accepted.

## Findings

- fixed: source identity, accepted ancestry, archive metadata, archive review,
  post-publication identity, fresh cursors, power-cycle evidence, restore proof,
  and redaction were retained.
- blocked: candidate-capture-ready is not accepted because the retained
  pre-restore TFTP/final-identity evidence is contaminated by the restore
  window.
- deferred: packet-I/O discriminator remains deferred until a later explicit
  task records a clean candidate preflight or a different accepted
  discriminator.
- not-an-issue: the known-good/control baseline remains restored and valid after
  this run.
- removed: the generated upload archive was removed from target/tmp after
  metadata retention; boot artifact bytes are not checked in.

## Evidence Map

- Evidence directory:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v4/candidate-preflight-v4-20260629T161757Z/.
- Classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v4/candidate-preflight-v4-20260629T161757Z/classification.json.
- Evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v4/candidate-preflight-v4-20260629T161757Z/evidence-map.json.
- Preflight summary:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v4/candidate-preflight-v4-20260629T161757Z/preflight-summary.json.
- Candidate archive metadata:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v4/candidate-preflight-v4-20260629T161757Z/candidate-identity/archive-metadata.json.
- Candidate capture summary:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v4/candidate-preflight-v4-20260629T161757Z/candidate-run/capture-invariant-summary.json.

## Redaction Review

Task-owned JSON evidence was scrubbed after capture to replace peer IP/MAC
fields and firmware/dnsmasq peer identifiers with redacted placeholders.
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
- Stable TFTP delta: inconclusive for candidate readiness because the retained
  pre-restore window was contaminated by restore/control identity.
- Restore to named selected-control snapshot: pass; final GET /status reports
  tree_hash=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
  and effective_kernel=kernel_2712.img.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task: null.

planningNeeded: true.

planningReason: candidate preflight v4 did not retain a clean candidate
pre-restore capture window; supervisor planning is required before any further
candidate preflight, packet-I/O discriminator, OpenSSH/generated-root retry,
remote receipt, compatibility, service success, ssh-ready=true, broad shell
work, or phase transition.

Commit: recorded in talos-supervisor-state.json after final commit.
