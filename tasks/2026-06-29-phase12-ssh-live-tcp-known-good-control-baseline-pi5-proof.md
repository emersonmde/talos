# Phase 12 SSH Live TCP Known-Good Control Baseline Pi 5 Proof

Task id: phase12-ssh-live-tcp-known-good-control-baseline-pi5-proof-20260629

Status: accepted after commit.

Classification: known-good-control-ready.

Evidence level: serialized Pi 5 hardware boot/output, lab-controller API
identity, fresh serial cursor/completeness diagnostics, stable same-cursor TFTP
delta before restore, selected-control readiness classifier, restore proof,
task-owned JSON evidence, docs build, and diff checks. No candidate archive was
published, and no packet-I/O discriminator, OpenSSH/generated-root retry,
remote receipt, compatibility claim, service success claim, ssh-ready=true,
broad shell work, or phase transition was performed.

## Goal

Run one bounded Pi 5 hardware proof for the newly selected known-good/control
baseline before any candidate preflight resumes.

## Scope Performed

- Promoted the mechanically selected baseline proof after the no-power control
  reselection contract accepted
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z as the selected control
  snapshot.
- Acquired hardwareTestLock before restore, cursor capture, power-cycle, TFTP
  observation, and restore evidence.
- Restored the selected snapshot, confirmed the a0452458... tree with
  effective_kernel=kernel_2712.img and 104,136-byte da591740/kernel_2712.img,
  then ran exactly one selected-control power cycle.
- Retained the empty pre-power serial drain/completeness diagnostic, post-power
  saturated-cursor direct-read serial window, stable same-cursor TFTP delta,
  final pre-restore identity, selected-control readiness classification, and
  post-restore identity.
- Restored the same selected control snapshot before releasing the hardware
  lock.

## Terminal Classification

known-good-control-ready.

The selected control snapshot satisfied the accepted v3 production-timer
known-good contract. The readiness classifier reported
valid-known-good-talos-readiness-v3 using the retained serial hardware artifact,
stable lab-controller identity, and stable TFTP evidence. The required
rpi5-production-timer-preemption: PASS marker was present. TALOS: kernel_main
was absent from the retained serial window, but remains metadata-only under the
accepted v3 policy when the downstream production-timer PASS marker is present.

Stable identity and TFTP evidence joined the same selected control tree. The
pre-run and final-pre-restore status both reported tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
effective_kernel=kernel_2712.img. Stable TFTP evidence retained 13 events,
including two matching 104,136-byte da591740/kernel_2712.img serves, and the
capture identity join rejection list was empty. Post-restore status returned to
the same selected control tree.

## Findings

- fixed: restored and proved the selected
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z control snapshot under
  hardwareTestLock.
- fixed: retained fresh serial cursor/completeness evidence before power and a
  6,551-byte post-power saturated-cursor direct-read serial window.
- fixed: stable same-cursor TFTP evidence retained two matching 104,136-byte
  da591740/kernel_2712.img serves from the selected a0452458... tree.
- fixed: accepted selected-control readiness through
  valid-known-good-talos-readiness-v3 and selected
  phase12-ssh-live-tcp-pi5-candidate-preflight-v4-20260629 as the only
  successor.
- deferred: candidate archive publication, packet-I/O, OpenSSH/generated-root
  retry, remote receipt, compatibility, service success, ssh-ready=true, broad
  shell work, and phase transition remain deferred to explicitly selected
  successor tasks.
- not-an-issue: TALOS: kernel_main was absent from the retained serial window;
  the accepted v3 classifier records it as metadata-only when the downstream
  production-timer PASS marker is present.
- removed: no source, helper, docs, task, or evidence artifact was removed.

## Evidence Map

- Evidence directory:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-control-baseline-pi5-proof/selected-known-good-control-baseline-20260629T155308Z/.
- Classification:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-control-baseline-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-control-baseline-pi5-proof/evidence-map.json.
- Capture-invariant summary:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-control-baseline-pi5-proof/selected-known-good-control-baseline-20260629T155308Z/capture-invariant-summary.json.
- Selected-control readiness classifier:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-control-baseline-pi5-proof/selected-known-good-control-baseline-20260629T155308Z/readiness-v3-classification.json.
- Runtime readiness primary artifact:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-control-baseline-pi5-proof/selected-known-good-control-baseline-20260629T155308Z/selected-known-good-control-baseline-20260629T155308Z-runtime-readiness-primary.json.
- Stable TFTP delta:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-control-baseline-pi5-proof/selected-known-good-control-baseline-20260629T155308Z/tftp-delta-stable-pre-restore.json.
- Restore evidence:
  tasks/evidence/2026-06-29-phase12-ssh-live-tcp-known-good-control-baseline-pi5-proof/selected-known-good-control-baseline-20260629T155308Z/restore-snapshot.json.

## Redaction Review

Task summaries omit packet payloads, key/session material, SSH identifiers,
public-key blobs, signatures, fingerprints, peer identifiers, operator
identity, private user data, and secret-derived identifiers. Raw lab-controller
TFTP artifacts remain task-owned hardware evidence and may include endpoint
local client fields.

## Validation

- git status --short --branch before edits/action: pass; main was ahead of
  origin with no uncommitted Talos changes before task-owned evidence creation.
- Lab API GET /status, GET /boot/files, and GET /boot/snapshots before power
  and after restore: pass; the selected snapshot restored and remained on tree
  a0452458... with effective_kernel=kernel_2712.img.
- Fresh serial cursor/completeness diagnostics before power: pass; empty
  pre-power /serial/read drain retained.
- GET /tftp/logs tail cursor and stable same-cursor delta before restore: pass;
  stable helper retained 13 events and two 104,136-byte served kernel_2712.img
  fetches.
- Accepted selected-control readiness classifier/contract gate: pass;
  valid-known-good-talos-readiness-v3.
- Restore to named snapshot/control state: pass; post-restore GET /status
  reports the selected a0452458... tree.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-pi5-candidate-preflight-v4-20260629.

planningNeeded: false.

Candidate archive publication, packet-I/O, OpenSSH/generated-root retry, remote
receipt, compatibility, service success, ssh-ready=true, broad shell work, and
phase transition remain blocked until the selected candidate preflight task is
promoted and accepted.

Commit: recorded in talos-supervisor-state.json after final commit.
