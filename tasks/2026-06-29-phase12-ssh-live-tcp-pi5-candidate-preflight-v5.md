# Phase 12 SSH Live TCP Pi 5 Candidate Preflight V5

Task id: phase12-ssh-live-tcp-pi5-candidate-preflight-v5-20260629

Status: accepted after commit.

Classification: blocked-capture-window-contract.

Evidence level: source/archive identity, lab-controller API identity,
serialized Pi 5 hardware power/output, restore proof, repaired capture-window
checker output, task-owned JSON evidence, docs build, and diff checks.

## Goal

Run one serialized candidate preflight after the v5 capture-window contract
repair, and either prove candidate-capture-ready or preserve the first missing
fact before any packet-I/O discriminator, OpenSSH/generated-root retry, remote
receipt, compatibility, service success, ssh-ready=true claim, broad shell
work, or phase transition.

## Scope Performed

- Promoted the mechanically unblocked v5 candidate preflight task and acquired
  hardwareTestLock before lab actions.
- Recorded source identity at commit
  04b8fbf6462abd35622287d136879b2292438e63 and confirmed the accepted runtime
  commits are ancestors.
- Built and reviewed a Pi 5 candidate archive with
  da591740/kernel_2712.img present.
- Published the reviewed candidate archive and retained post-publication lab
  identity for tree 18e467bf70316e41fa4232d3b7b3ea328160c3c77befa799e00dffe707fa603e
  and 87,432-byte selected kernel.
- Started one candidate power cycle through the repaired capture-invariant
  helper.
- Restored the lab to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirmed the
  a0452458... selected-control identity after restore.
- Ran the v5 capture-window checker and rejected candidate-capture-ready.

No packet-I/O discriminator, OpenSSH/generated-root retry, remote receipt,
compatibility, service success, ssh-ready=true, runtime russh adoption, fake
command expansion, broad shell work, or phase transition was performed or
accepted.

## Terminal Classification

blocked-capture-window-contract.

The candidate archive identity was established, but the retained capture window
is not accepted. During this worker run, a manual restore was performed while
the helper-owned capture bundle was still completing. The resulting retained
checker evidence reports restored-control TFTP/final identity rather than a
clean candidate pre-restore window:

- expected candidate tree: 18e467bf70316e41fa4232d3b7b3ea328160c3c77befa799e00dffe707fa603e.
- expected selected fetch: da591740/kernel_2712.img, 87,432 bytes.
- checker-observed TFTP selected fetch bytes: 104,136 and 104,136.
- checker-observed final pre-restore tree:
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The first missing fact is a clean helper-owned candidate pre-restore TFTP and
final identity window before any restore/control mutation. The retained v5
checker output is useful blocker evidence, but it cannot select the packet-I/O
discriminator.

selected_next_task: null.

planningNeeded: true.

## Findings

- fixed: candidate source/archive identity, accepted ancestry, archive review,
  post-publication identity, hardware lock lifecycle, restore proof, and
  redaction were retained.
- blocked: candidate-capture-ready is rejected because the retained capture
  window contains restored-control identity/byte evidence instead of a clean
  candidate pre-restore window.
- deferred: packet-I/O discriminator remains deferred until a future explicit
  task records candidate-capture-ready under an uncontaminated window.
- not-an-issue: the lab was restored to the selected known-control snapshot.
- removed: the generated upload archive was removed from target/tmp after
  metadata retention; boot artifact bytes are not checked in.

## Evidence Map

- Evidence directory:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v5/candidate-preflight-v5-20260629T172809Z/.
- Classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v5/candidate-preflight-v5-20260629T172809Z/classification.json.
- Evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v5/candidate-preflight-v5-20260629T172809Z/evidence-map.json.
- Preflight summary:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v5/candidate-preflight-v5-20260629T172809Z/preflight-summary.json.
- Candidate capture summary:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v5/candidate-preflight-v5-20260629T172809Z/candidate-run/capture-invariant-summary.json.
- V5 capture-window checker:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v5/candidate-preflight-v5-20260629T172809Z/candidate-run/capture-window-v5-check.json.

## Redaction Review

Task-owned JSON evidence was scrubbed to replace peer/address-bearing TFTP
fields and raw dnsmasq lines with redacted placeholders. Durable evidence
retains task ids, source/archive metadata, tree hashes, byte counts,
cursor/capture classifications, validation commands/results, and metadata-only
counters. It does not retain packet payload contents, key material, session
material, boot artifact bytes, private user data, stable secret-derived
identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- Candidate archive materialization: pass; rpi5-boot-tree exited 0.
- Candidate archive review: pass; rpi5-archive-review exited 0.
- Lab API GET /status, GET /boot/files, and GET /boot/snapshots before power
  and after restore: pass.
- Fresh serial cursor and TFTP cursor: pass; retained before power.
- Repaired capture-window checker: pass as blocker evidence; exited 1 with
  capture-staging-blocked and rejected restored-control byte/tree evidence.
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

planningReason: candidate preflight v5 did not retain a clean helper-owned
candidate pre-restore capture window before restore/control mutation;
supervisor planning is required before any further candidate preflight,
packet-I/O discriminator, OpenSSH/generated-root retry, remote receipt,
compatibility, service success, ssh-ready=true, broad shell work, or phase
transition.

Commit: recorded in talos-supervisor-state.json after final commit.
