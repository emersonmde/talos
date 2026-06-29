# Phase 12 SSH Live TCP Pi 5 Candidate Preflight V8

Task id: phase12-ssh-live-tcp-pi5-candidate-preflight-v8-20260629

Status: accepted after commit.

Classification: blocked-candidate-kernel-not-starting.

Evidence level: source/archive identity, lab-controller API identity,
serialized Pi 5 hardware power/output, helper-owned capture/restore window,
capture-window checker output, task-owned JSON evidence, docs build, and diff
checks.

## Goal

Run exactly one serialized Pi 5 candidate preflight after fetch-path-ready, then
either unlock packet-I/O or preserve the first precise blocker.

## Scope Performed

- Promoted the queued v8 candidate preflight after selected-fetch-path
  reconciliation accepted fetch-path-ready.
- Acquired hardwareTestLock before lab read, archive publication, or Pi 5 power
  action, and released it only after restore evidence showed the lab back on
  the a0452458... control tree.
- Built a nonce-bearing runtime-marker-route archive from commit
  3a8f0092c9653a291be66c0581fb4287b9eda99e with both root and
  da591740/selected Pi 5 boot files.
- Published the reviewed candidate archive, ran the foreground helper to
  completion, and retained fresh serial cursor, TFTP cursor/delta, final
  pre-restore identity, helper-owned restore proof, post-restore identity, and
  checker output.
- Stopped before packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility, service success, ssh-ready=true, broad shell work, or phase
  transition.

## Terminal Classification

blocked-candidate-kernel-not-starting.

The selected-fetch repair worked in hardware-facing evidence:

- archive source commit:
  3a8f0092c9653a291be66c0581fb4287b9eda99e;
- candidate archive SHA-256:
  2e599aff901b68e46ff2e8d495f0e6e1017bc44f51e53d93b035c0fa83e88372;
- selected expected fetch: da591740/kernel_2712.img, 152,160 bytes,
  SHA-256 09ee965066407ebdba1c384f10c4fa2210a5befd6a8f663b3df047b5049a6b4e;
- post-publication and final pre-restore tree:
  2f5083a58d2371dc13431cd545c5f9846ca9287a00531bcb31d1656d5665fb3a;
- TFTP stable same-cursor delta observed 13 events, including two selected
  da591740/kernel_2712.img serves, both matching 152,160 bytes;
- helper completion metadata reports helper_run_completed=true, completion
  event count 8, TFTP delta before restore, and final pre-restore identity
  before restore;
- helper-owned restore returned the lab to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

candidate-capture-ready remains rejected because the nonce-bearing
`TALOS: ssh-service-smoltcp-runtime-ready` marker did not appear after power.
The capture-window checker rejected the run with:

- run-unique-capture-nonce-not-present-after-power
- required-marker-not-present-after-power

selected_next_task: null.

planningNeeded: true.

planningReason: v8 retained a clean selected-fetch and helper-owned
capture/restore window, but the nonce-bearing runtime marker was absent after
power; packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility, service success, ssh-ready=true, broad shell work, and phase
transition remain blocked pending supervisor planning.

## Findings

- fixed: the selected da591740/kernel_2712.img fetch path is now present in the
  archive, lab publication, same-window TFTP delta, and final pre-restore
  identity.
- fixed: the helper owned the power, serial, TFTP, final pre-restore identity,
  restore, post-restore identity, and completion metadata for the accepted
  v5 capture-window contract.
- blocked: candidate-capture-ready is rejected because the nonce-bearing
  runtime marker was absent after power.
- deferred: packet-I/O discriminator remains deferred until a future explicit
  task records candidate-capture-ready or the supervisor selects a narrower
  discriminator for the missing runtime marker.
- not-an-issue: the lab restore path returned to the selected a0452458...
  control tree without manual restore after helper completion.
- removed: the generated upload archive was removed from target/tmp after
  metadata retention; boot artifact bytes are not checked in.
- deferred: an earlier v8 shell attempt stopped before publication or power
  while materializing the archive; it is retained as aborted pre-publication
  evidence and did not contribute to the terminal classification.

## Evidence Map

- Evidence directory:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v8/candidate-preflight-v8-20260629T214225Z/.
- Classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v8/candidate-preflight-v8-20260629T214225Z/classification.json.
- Evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v8/candidate-preflight-v8-20260629T214225Z/evidence-map.json.
- Preflight summary:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v8/candidate-preflight-v8-20260629T214225Z/preflight-summary.json.
- Candidate archive metadata:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v8/candidate-preflight-v8-20260629T214225Z/candidate-identity/archive-metadata.json.
- Candidate capture summary:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v8/candidate-preflight-v8-20260629T214225Z/candidate-run/capture-invariant-summary.json.
- Capture-window checker:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v8/candidate-preflight-v8-20260629T214225Z/candidate-run/capture-window-v5-check.json.
- Aborted pre-publication materialization attempt:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-pi5-candidate-preflight-v8/candidate-preflight-v8-20260629T214041Z/.

## Redaction Review

Task-owned JSON evidence was scrubbed to replace raw serial text, serial
base64, TFTP peer/address fields, and raw TFTP log lines with redaction
placeholders. Durable evidence retains task ids, source/archive metadata, tree
hashes, byte counts, cursor/capture classifications, validation commands and
results, and metadata-only counters. It does not retain packet payload
contents, key material, session material, boot artifact bytes, private user
data, stable secret-derived identifiers, or unnecessary hardware data.

## Validation

- git status --short --branch before edits/action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- Candidate archive materialization: pass on terminal run; rpi5-boot-tree
  exited 0 and the archive/static review exited 0.
- Candidate archive/static marker and selected-fetch review: pass; the archive
  contained the nonce-bearing runtime marker route and selected
  da591740/kernel_2712.img path.
- Lab API GET /status, GET /boot/files, and GET /boot/snapshots before power
  and after restore: pass.
- Fresh serial cursor and TFTP cursor: pass; retained before power.
- Foreground capture helper: pass; exited 0 and retained post-restore identity.
- Capture-window checker: pass as blocker evidence; exited 1 with
  run-unique-capture-nonce-not-present-after-power and
  required-marker-not-present-after-power.
- Restore to named selected-control snapshot: pass; final GET /status reports
  tree_hash=a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10
  and effective_kernel=kernel_2712.img.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass; html backend wrote book/ and
  emitted only the large search-index warning.
- git diff --cached --check: pass.

## Result

selected_next_task: null.

planningNeeded: true.

Commit: recorded in talos-supervisor-state.json after final commit.
