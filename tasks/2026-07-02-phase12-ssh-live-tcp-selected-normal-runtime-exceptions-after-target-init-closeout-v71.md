# Phase 12 SSH Live TCP Selected Normal Runtime Exceptions After Target Init Closeout V71

Task id: phase12-ssh-live-tcp-selected-normal-runtime-exceptions-after-target-init-closeout-v71-20260702

Status: accepted after no-hardware evidence closeout.

Classification: selected-normal-runtime-exceptions-frontier-proved.

Evidence level: git status inspection, v71 task record/evidence review,
task-owned JSON evidence, static source/order inspection, docs build, and diff
checks. No hardware action, lab publication, boot snapshot mutation, Pi 5 power
cycle, serial capture, TFTP capture, kernel_main proof, route-start proof,
runtime-ready proof, packet-I/O, OpenSSH/generated-root retry, remote receipt,
compatibility claim, service success claim, ssh-ready=true,
fake/kernel-backed command expansion, broad shell work, or phase transition was
performed.

## Goal

Reconcile the v71 selected exceptions-ready Pi 5 evidence and decide only
whether the selected normal-runtime frontier advances through exceptions ready.

## Scope Performed

- Promoted this queued no-hardware closeout after v71 accepted and selected this
  exact task.
- Reviewed the v71 task record, classification JSON, evidence map, retained run
  summary, selected TFTP events, serial marker counts, final pre-restore
  identity, restore proof, and redaction notes.
- Confirmed that v71 ties one selected candidate window to the selected v70
  archive identity, selected da591740/kernel_2712.img byte service, retained
  TALOS: exceptions ready marker output, final pre-restore selected identity,
  and restore proof.
- Selected the already queued v72 no-hardware kernel_main reconciliation as the
  next explicit task because the current source/helper surface has an objective
  post-exceptions kernel_main marker-loop boundary.

## Closeout Result

v71 accepted selected-normal-runtime-exceptions-marker-retained. The retained
candidate window staged selected tree
b4c9bf0c09d122def872228a4e3d2a0f5836bfa0c7e4e4cdaa3b42ddf3e8ee9c from archive
target/tmp/selected-normal-runtime-exceptions-ready-v70.tar.gz with SHA-256
18007965ceb10991766e01ab2cf4d6f468530eca97d1a8c3a016a39b0402396b. The selected
da591740/kernel_2712.img was 152,880 bytes with SHA-256
7a62150e4232fc8215a7c7ec8e502697bdabb3a9e6bcd62f640c75aba722e455.

The TFTP delta retained four selected da591740/kernel_2712.img fetches, all
matching 152,880 bytes. The serial window retained TALOS: exceptions ready as
the deepest marker family member and retained 881 occurrences of the required
marker TALOS: exceptions ready capture-nonce=runtime-marker-route-static. Final
pre-restore identity still reported the selected tree and selected fetch bytes.
Restore returned the lab to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z
with tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

GET / returned 404 in the lab revision used by v71. This is not an evidence
gap because v71 used the documented fallback rule: /boot/files and /status are
the authoritative selected-tree identity sources. The incomplete later helper
rerun is quarantined as non-classification evidence and does not weaken the
accepted candidate window.

## Terminal Classification

selected-normal-runtime-exceptions-frontier-proved.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-after-exceptions-reconciliation-v72-20260702.

planningNeeded: false.

Kernel_main proof, route-start proof, runtime-ready proof, packet-I/O,
OpenSSH/generated-root retry, remote receipt, compatibility/service readiness,
ssh-ready=true, fake/kernel-backed command expansion, broad shell work, and
phase transition remain blocked until a future explicitly scoped task proves
them.

## Findings

- fixed: reconciled v71 selected archive identity, selected TFTP service,
  exceptions-ready serial marker retention, final pre-restore identity, and
  restore proof into a proved exceptions-ready frontier.
- fixed: preserved the root endpoint 404 as endpoint semantics only; /status
  and /boot/files remain the selected identity sources for this evidence.
- fixed: quarantined the incomplete helper rerun as non-classification evidence.
- not-an-issue: no inconclusive triage is required because the accepted v71
  candidate window has no identity-join rejection reasons.
- deferred: kernel_main marker retention, route-start, runtime-ready,
  packet-I/O, OpenSSH/generated-root behavior, compatibility/service readiness,
  ssh-ready=true, fake command expansion, broad shell work, and phase
  transition are left to later explicit tasks.
- removed: hardware action, lab publication, serial/TFTP capture, kernel_main
  proof, and later service claims as mechanically unblocked work in this
  closeout.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-after-target-init-closeout-v71/classification.json.
- Evidence map:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-after-target-init-closeout-v71/evidence-map.json.
- Static closeout summary:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-selected-normal-runtime-exceptions-after-target-init-closeout-v71/static/closeout-summary.md.
- Source v71 classification:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-exceptions-after-target-init-preflight-v71/classification.json.
- Source v71 accepted run:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-exceptions-after-target-init-preflight-v71/run-20260702T095454Z.
- Source v71 run summary:
  tasks/evidence/2026-07-02-phase12-ssh-live-tcp-pi5-selected-normal-runtime-exceptions-after-target-init-preflight-v71/validation/run-summary.json.

## Redaction Review

Task-owned closeout evidence retains task ids, source/path labels, hashes, byte
counts, marker counts, classifications, and validation outcomes. It does not
retain private user data, credentials, packet payloads, SSH/session/key
material, public-key blobs, signatures, fingerprints, operator identities, or
external account data.

## Validation

- git status --short --branch before edits/action: pass.
- Review v71 task record, classification JSON, evidence-map JSON, retained lab
  summaries, TFTP delta, serial marker counts, and restore proof: pass.
- jq empty on supervisor state and task-owned JSON evidence: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

## Result

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-after-exceptions-reconciliation-v72-20260702.

planningNeeded: false.

Commit: recorded in talos-supervisor-state.json after final commit.
