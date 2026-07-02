# Phase 12 SSH Live TCP Pi 5 Selected Normal Runtime Exceptions After Target Init Preflight V71

Task id: phase12-ssh-live-tcp-pi5-selected-normal-runtime-exceptions-after-target-init-preflight-v71-20260702

Status: accepted after serialized Pi 5 hardware preflight.

Classification: selected-normal-runtime-exceptions-marker-retained.

Evidence level: git status inspection, static archive identity check,
lab-controller API, selected TFTP service, serial hardware boot/output,
restore proof, task-owned JSON evidence, docs build, and diff checks. No
kernel_main proof, route-start proof, runtime-ready proof, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility claim, service
success claim, ssh-ready=true, fake/kernel-backed command expansion, broad
shell work, or phase transition was performed.

## Goal

Run the serialized Pi 5 preflight selected by v70 and decide only whether the
selected exceptions-ready archive reaches the required exceptions-ready marker.

## Scope Performed

- Promoted the queued v71 hardware task after accepted v70 selected this exact
  task and hardwareTestLock was unlocked/restored.
- Acquired hardwareTestLock before lab publication, Pi 5 power action, serial
  capture, or TFTP capture.
- Rechecked the v70 archive SHA-256 and archive-review contract before
  publication.
- Published target/tmp/selected-normal-runtime-exceptions-ready-v70.tar.gz to
  the lab and captured selected identity through /status and /boot/files.
- Ran the capture-chain helper against the selected tree and required marker
  TALOS: exceptions ready capture-nonce=runtime-marker-route-static.
- Restored the lab to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z before
  releasing hardwareTestLock.

## Hardware Result

The accepted candidate window staged selected tree
b4c9bf0c09d122def872228a4e3d2a0f5836bfa0c7e4e4cdaa3b42ddf3e8ee9c with
effective kernel kernel_2712.img. The selected archive SHA-256 is
18007965ceb10991766e01ab2cf4d6f468530eca97d1a8c3a016a39b0402396b, and the
selected da591740/kernel_2712.img is 152,880 bytes with SHA-256
7a62150e4232fc8215a7c7ec8e502697bdabb3a9e6bcd62f640c75aba722e455.

The TFTP delta retained four selected da591740/kernel_2712.img fetches, all
with the expected 152,880-byte count. The serial window retained the deepest
marker family member TALOS: exceptions ready and 881 occurrences of the
required marker. The final pre-restore identity still reported the selected
tree and selected fetch bytes. Restore returned the lab to
phase12-ssh-v10-openssh-clean-pre-20260624T074100Z with tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

GET / returned 404 in this lab revision, so the helper used its documented
fallback rule: /boot/files and /status are the authoritative selected-tree
identity sources. A later attempted helper rerun produced an empty
serial-observe output before restore; the lab was already at/restored to the
baseline tree afterward, and that incomplete run is recorded but not used for
terminal classification.

## Terminal Classification

selected-normal-runtime-exceptions-marker-retained.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-exceptions-after-target-init-closeout-v71-20260702.

planningNeeded: false.

Kernel_main proof, route-start proof, runtime-ready proof, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility/service readiness,
ssh-ready=true, fake/kernel-backed command expansion, broad shell work, and
phase transition remain blocked.

## Findings

- fixed: acquired and released hardwareTestLock around all lab/hardware action.
- fixed: proved selected archive identity, selected TFTP byte service,
  exceptions-ready serial marker retention, final pre-restore selected
  identity, and restore proof in one retained candidate window.
- fixed: recorded the root endpoint 404 as endpoint semantics only; /status and
  /boot/files preserved selected-tree identity.
- fixed: quarantined the incomplete later helper rerun as non-classification
  evidence and restored the lab before releasing the lock.
- not-an-issue: no inconclusive triage is required because the accepted run has
  no identity-join rejection reasons.
- deferred: v71 closeout must reconcile the exceptions-ready frontier before
  any kernel_main, route-start, runtime-ready, packet-I/O, or OpenSSH successor.
- removed: later runtime/network/service milestones as mechanically unblocked
  work in this task.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-exceptions-after-target-init-preflight-v71/classification.json.
- Evidence map:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-exceptions-after-target-init-preflight-v71/evidence-map.json.
- Accepted run:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-exceptions-after-target-init-preflight-v71/run-20260702T095454Z.
- Run summary:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-exceptions-after-target-init-preflight-v71/validation/run-summary.json.

## Redaction Review

Task-owned evidence retains lab-controller boot identity, TFTP summaries,
serial marker output from Talos, hashes, byte counts, task ids, and validation
outcomes. It does not retain private user data, credentials, packet payloads,
SSH/session/key material, external account data, public-key blobs, signatures,
fingerprints, or operator identities.

## Validation

- git status --short --branch before lab action: pass.
- jq empty on supervisor state and task-owned JSON evidence: pass.
- Archive identity check against v70 metadata: pass.
- Static archive review and exceptions-ready archive review: pass.
- Serialized hardwareTestLock acquire/release with restored=true: pass.
- Lab API /status and /boot/files candidate identity before/after publication,
  final pre-restore identity, and post-restore identity: pass.
- Fresh serial cursor/window capture: pass; required marker retained 881 times.
- Lab API /tftp/logs cursor delta: pass; four selected 152,880-byte kernel
  fetches retained.
- Known-good control/candidate rerun: not run; the accepted candidate window was
  decisive for identity, serial, TFTP, and restore.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-exceptions-after-target-init-closeout-v71-20260702.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
