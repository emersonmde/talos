# Phase 12 SSH Live TCP Pi 5 Selected Normal Runtime Exceptions Continuation Preflight V44

Task id: phase12-ssh-live-tcp-pi5-selected-normal-runtime-exceptions-continuation-preflight-v44-20260701

Status: accepted after serialized Pi 5 validation.

Classification: selected-normal-runtime-exceptions-ready-marker-retained.

Evidence level: hardwareTestLock, static archive/image review, lab-controller
API identity, fresh serial cursor/drain, stable same-cursor TFTP evidence,
serial hardware output marker summary, restore proof, redacted task-owned JSON
evidence, docs build, and diff checks.

## Goal

Run the v43-selected normal-runtime exceptions-ready continuation
discriminator on the Pi 5 and classify whether the selected Image reaches
TALOS: exceptions ready after target init and arch::aarch64::exceptions::init(),
before kernel_main, route-start, runtime-ready, packet-I/O, OpenSSH, or service
readiness.

## Scope Performed

- Promoted this ready hardware task after supervisor planning added v44 and
  verified v43 accepted at 7f5c82fed47f402a6f2bff5eced46d4108f07918.
- Acquired hardwareTestLock before lab publication, boot mutation, Pi 5 power
  action, or hardware capture.
- Published only the v43 exceptions-ready marker-loop archive:
  target/tmp/selected-normal-runtime-exceptions-ready-v43.tar.gz.
- Captured candidate identity, fresh serial/TFTP cursors, same-window TFTP,
  serial marker output, final pre-restore identity, and restore proof.
- Redacted raw serial text and raw TFTP peer/log-line fields from retained
  task-owned JSON after deriving marker counts and byte-count summaries.
- Restored the lab to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirmed the final
  restored tree before releasing hardwareTestLock.

## Hardware Result

Candidate: selected-normal-runtime-exceptions-v44.

The accepted run published selected tree
2c0d4152ebae130632caa5a9e8fa776704ec0336d2c54609ab00a5981328fcde with
effective kernel_2712.img. The final pre-restore identity remained on that
same selected tree. The stable same-cursor TFTP delta observed 13 events and
served da591740/kernel_2712.img twice at 152,880 bytes. The saturated
direct-read serial window retained TALOS: exceptions ready 2,145 times and did
not retain TALOS: kernel_main. Post-restore identity returned to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Terminal Classification

selected-normal-runtime-exceptions-ready-marker-retained.

This proves the selected 152,880-byte normal-runtime exceptions-ready
marker-loop archive reaches TALOS: exceptions ready on Pi 5 after target init
and arch::aarch64::exceptions::init(). It does not prove kernel_main,
route-start, runtime-ready, packet-I/O, OpenSSH/generated-root retry, remote
receipt, compatibility/service readiness, ssh-ready=true, fake command
expansion, broad shell work, or phase transition.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-exceptions-continuation-closeout-v44-20260701.

planningNeeded: false.

## Findings

- fixed: published and ran the v43-selected exceptions-ready continuation
  discriminator under hardwareTestLock with selected-byte TFTP evidence and
  restore proof.
- fixed: accepted only the run whose selected tree identity, selected fetch
  byte count, TFTP delta, final pre-restore identity, serial marker output, and
  restore proof are joined under the same candidate run.
- fixed: redacted raw serial text plus raw TFTP peer/log-line fields from
  retained task-owned JSON while preserving marker counts, byte counts, hashes,
  and classifications.
- not-an-issue: the accepted exceptions-ready marker loop intentionally retains
  only the deepest normal-runtime continuation markers in the saturated
  direct-read window; earlier frontiers remain proven by v34, v36, v38, v40,
  and v42.
- deferred: kernel_main, route-start, runtime-ready, packet-I/O, OpenSSH
  compatibility, service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition remain unproved.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as immediate successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-exceptions-continuation-preflight-v44/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-exceptions-continuation-preflight-v44/evidence-map.json.
- Redacted run summary:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-exceptions-continuation-preflight-v44/lab/run-summary-redacted.json.
- Accepted candidate:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-exceptions-continuation-preflight-v44/lab/v44-candidate/.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-exceptions-continuation-preflight-v44/validation/archive-review.stdout.txt.
- Exceptions-ready archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-exceptions-continuation-preflight-v44/validation/exceptions-ready-archive-review.stdout.txt.

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
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

Implementation commit: recorded in supervisor state after commit creation.
