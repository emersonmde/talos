# Phase 12 SSH Live TCP Pi 5 Entry Marker Discriminator V9

Task id: phase12-ssh-live-tcp-pi5-entry-marker-discriminator-v9-20260629

Status: accepted after commit.

Classification: blocked-candidate-entry-not-starting.

Evidence level: source/archive identity, lab-controller API identity,
serialized Pi 5 hardware power/output, helper-owned capture/restore window,
entry/runtime marker-order checker, capture-window checker, task-owned JSON
evidence, docs build, and diff checks. No packet-I/O, OpenSSH/generated-root
retry, remote receipt, compatibility, service success, ssh-ready=true, broad
shell work, or phase transition was performed.

## Goal

Run the smallest serialized Pi 5 discriminator after v8 selected-fetch proof
and the no-hardware kernel-entry reconciliation: distinguish selected-fetch
without `TALOS: kernel_main` from `kernel_main` without runtime readiness.

## Scope Performed

- Promoted the queued v9 task only after the accepted reconciliation selected
  this exact task.
- Acquired hardwareTestLock before lab status reads, archive publication, or
  Pi 5 power action.
- Built a nonce-bearing runtime-marker-route archive from commit
  6f50d28b3fc02a7897f6d672eea85da0e24ee549 with root and da591740/selected
  Pi 5 boot files.
- Published the reviewed candidate archive, ran one foreground capture bundle,
  retained selected-fetch TFTP evidence, final pre-restore candidate identity,
  serial marker metadata, restore proof, post-restore identity, and checker
  output.
- Stopped before packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility, service success, ssh-ready=true, broad shell work, or phase
  transition.

## Terminal Classification

blocked-candidate-entry-not-starting.

The v9 candidate preserved the selected-fetch and pre-restore identity
contract:

- candidate archive SHA-256:
  d88f87c38bd60040e80194ed081c7f6c4990aa99ddf87d2b6d907d52f219d6f5;
- selected expected fetch: da591740/kernel_2712.img, 152,152 bytes,
  SHA-256 6c08216a0487b1b78067b939fdfac5f9c456ec5e7e74154ebbcba502207826eb;
- post-publication and final pre-restore tree:
  49a9cb5bc267a3877979356cca273f1747cd7cc3430d82ac6c3bdbfddedc1a3e;
- TFTP stable same-cursor delta observed two selected
  da591740/kernel_2712.img serves and both matched 152,152 bytes;
- helper-owned restore returned the lab to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The entry/runtime marker-order checker found zero occurrences of:

- `TALOS: kernel_main`;
- `TALOS: ssh-service-smoltcp-runtime-route-start`;
- `TALOS: ssh-service-smoltcp-runtime-ready capture-nonce=entry-marker-v9-20260629T231514Z`.

The serial window still retained firmware `NETWORK` markers, so this is not a
packet-I/O or OpenSSH-ready result. It proves the current blocker is earlier:
selected candidate bytes are served, but the selected kernel does not reach the
earliest Rust-side `kernel_main` marker in the retained post-power serial
window.

selected_next_task: null.

planningNeeded: true.

planningReason: v9 produced a clean selected-fetch/no-kernel_main hardware
boundary. Supervisor planning is required before any further discriminator,
packet-I/O, OpenSSH/generated-root retry, remote receipt, compatibility,
service success, ssh-ready=true, broad shell work, or phase transition.

## Findings

- fixed: the selected da591740/kernel_2712.img fetch path remained present
  through archive review, lab publication, same-window TFTP delta, and final
  pre-restore identity.
- fixed: the foreground capture helper completed and restored the lab to the
  selected control tree.
- blocked: `TALOS: kernel_main`, runtime route-start, and nonce-bearing
  runtime-ready markers were absent after selected fetch.
- deferred: packet-I/O and OpenSSH remain blocked until a future explicit task
  explains or repairs the selected-fetch/no-entry boundary.
- not-an-issue: the candidate archive retained the runtime marker tokens and
  fail-closed claim boundary in static review; the blocker is not an accepted
  packet-I/O or SSH readiness failure.
- removed: generated upload archive and boot tree remain untracked target/tmp
  artifacts and are not part of durable evidence.

## Evidence Map

- Evidence directory:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-entry-marker-discriminator-v9/entry-marker-v9-20260629T231514Z/.
- Classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-entry-marker-discriminator-v9/entry-marker-v9-20260629T231514Z/classification.json.
- Evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-entry-marker-discriminator-v9/entry-marker-v9-20260629T231514Z/evidence-map.json.
- Candidate archive metadata:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-entry-marker-discriminator-v9/entry-marker-v9-20260629T231514Z/candidate-identity/archive-metadata.json.
- Capture summary:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-entry-marker-discriminator-v9/entry-marker-v9-20260629T231514Z/candidate-run/capture-invariant-summary.json.
- Entry/runtime marker-order checker:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-entry-marker-discriminator-v9/entry-marker-v9-20260629T231514Z/candidate-run/entry-runtime-marker-order-check.json.
- Capture-window checker:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-entry-marker-discriminator-v9/entry-marker-v9-20260629T231514Z/candidate-run/capture-window-v5-check.json.

## Redaction Review

Task-owned JSON evidence was scrubbed to replace raw serial text and serial
base64 with redaction placeholders and remove raw TFTP peer/log-line fields.
Durable evidence retains task ids, source/archive metadata, tree hashes, byte
counts, marker counters, cursor/capture classifications, validation commands
and results, and metadata-only counters. It does not retain packet payload
contents, key material, session material, boot artifact bytes, private user
data, stable secret-derived identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- Candidate archive build/materialization and static marker/order review:
  pass; archive review confirmed selected da591740/kernel_2712.img and the
  nonce-bearing runtime-marker route tokens.
- Lab API GET /status, GET /boot/files, and GET /boot/snapshots before power
  and after restore: pass.
- Fresh serial cursor/completeness diagnostics before power: pass.
- GET /tftp/logs tail cursor and stable same-cursor delta before restore:
  pass; two selected 152,152-byte kernel serves.
- Entry/runtime marker-order checker: pass as blocker evidence;
  blocked-candidate-entry-not-starting.
- Capture-window checker: pass as blocker evidence; rejected only the missing
  nonce/runtime marker.
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

Commit: recorded in talos-supervisor-state.json after final commit.
