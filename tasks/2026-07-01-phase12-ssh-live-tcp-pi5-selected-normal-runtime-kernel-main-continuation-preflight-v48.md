# Phase 12 SSH Live TCP Pi 5 Selected Normal Runtime Kernel Main Continuation Preflight V48

Task id: phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v48-20260701

Status: accepted after serialized Pi 5 validation.

Classification: selected-normal-runtime-kernel-main-marker-retained.

Evidence level: hardwareTestLock, static archive/image review, lab-controller
API identity, fresh serial cursor/drain, stable same-cursor TFTP evidence,
serial hardware marker summary, restore proof, redacted task-owned JSON
evidence, docs build, and diff checks.

## Goal

Run the reconciled v47-selected normal-runtime kernel_main continuation
contract on the Pi 5 and classify whether the selected Image reaches
TALOS: kernel_main capture-nonce=runtime-marker-route-static before
route-start, runtime-ready, packet-I/O, OpenSSH, service readiness, or phase
transition.

## Scope Performed

- Promoted this queued hardware task after v47 accepted the authoritative
  hash-contract reconciliation.
- Acquired hardwareTestLock before lab publication, boot mutation, Pi 5 power
  action, or hardware capture.
- Published only the reconciled v45/v47 kernel_main marker-loop archive:
  target/tmp/selected-normal-runtime-kernel-main-v45.tar.gz.
- Captured selected identity, fresh serial/TFTP cursors, same-window TFTP,
  serial marker output, final pre-restore identity, and restore proof.
- Redacted raw serial text and raw TFTP peer/log-line fields from retained
  task-owned JSON after deriving marker counts and byte-count summaries.
- Restored the lab to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirmed the final
  restored tree before releasing hardwareTestLock.

## Hardware Result

Candidate: selected-normal-runtime-kernel-main-v48.

The accepted run published selected tree
9d2f354810e8f445705dd083c8876f47bd25fa5f1aec52762c5af98662fdf60a with
effective kernel_2712.img. The final pre-restore identity remained on that
same selected tree. Stable same-cursor TFTP served
da591740/kernel_2712.img twice at 152,896 bytes. The saturated direct-read
serial window retained TALOS: kernel_main
capture-nonce=runtime-marker-route-static 1,794 times. Post-restore identity
returned to tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Terminal Classification

selected-normal-runtime-kernel-main-marker-retained.

This proves the selected 152,896-byte normal-runtime kernel_main marker-loop
archive reaches boot::rpi5::kernel_main on Pi 5 with selected-byte TFTP service
and restore proof. It does not prove route-start, runtime-ready, packet-I/O,
OpenSSH compatibility, remote receipt, service readiness, ssh-ready=true, fake
command expansion, broad shell work, or phase transition.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-kernel-main-continuation-closeout-v48-20260701.

planningNeeded: false.

## Findings

- fixed: published and ran the reconciled v47-selected kernel_main
  continuation contract under hardwareTestLock with selected-byte TFTP evidence
  and restore proof.
- fixed: accepted only the run whose selected tree identity, selected fetch byte
  count, TFTP delta, final pre-restore identity, serial marker output, and
  restore proof are joined under the same candidate run.
- fixed: redacted raw serial text plus raw TFTP peer/log-line fields from
  retained task-owned JSON while preserving marker counts, byte counts, hashes,
  and classifications.
- not-an-issue: scripts/rpi5-observe-serial-window.sh returned exit 1 because
  the exact has_required_marker flag did not trip, but the same redacted
  capture summary's marker-family count retained the required kernel_main
  marker 1,794 times and the identity-join contract had no rejection reasons.
- deferred: route-start, runtime-ready, packet-I/O, OpenSSH compatibility,
  remote receipt, service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition remain unproved.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as immediate successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v48/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v48/evidence-map.json.
- Redacted run summary:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v48/lab/run-summary-redacted.json.
- Accepted candidate:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v48/lab/v48-candidate/.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v48/validation/kernel-main-archive-review.stdout.txt.
- Capture helper dry-run:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-kernel-main-continuation-preflight-v48/validation/capture-bundle-dry-run.json.

## Redaction Review

Task-owned aggregate summaries retain task ids, run labels, hashes, byte
counts, marker names, marker counts, selected-tree hashes, classifications, and
validation outcomes. Retained JSON evidence has raw serial text, raw TFTP
peer/log-line fields, packet payloads, SSH/session/key material, boot artifact
bytes, private data, and stable secret-derived identifiers redacted or absent.

## Validation

- git status --short --branch before lab action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- static archive/image review before publication: pass.
- hardwareTestLock acquisition before lab publication/power/capture: pass.
- Lab API identity after publication, final pre-restore, and after restore:
  pass.
- fresh serial cursor/drain and GET /tftp/logs cursor before Pi 5 power action:
  pass.
- stable same-cursor TFTP delta before restore: pass.
- restore to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirm
  post-restore tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10: pass.
- known-good control: not run because the first candidate was decisive.
- candidate rerun: not run because no inconclusive identity, serial freshness,
  or TFTP capture reason remained.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

Implementation commit: recorded in supervisor state after commit creation.
