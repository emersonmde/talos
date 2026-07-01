# Phase 12 SSH Live TCP Pi 5 Selected Normal Runtime Target Init Continuation Preflight V42

Task id: phase12-ssh-live-tcp-pi5-selected-normal-runtime-target-init-continuation-preflight-v42-20260701

Status: accepted after serialized Pi 5 validation.

Classification: selected-normal-runtime-target-init-marker-retained.

Evidence level: hardwareTestLock, static archive/image review, lab-controller
API identity, fresh serial cursor/drain, stable same-cursor TFTP evidence,
known-good control, candidate rerun with changed capture ordering, serial
hardware output marker summary, final pre-restore identity, restore proof,
task-owned JSON evidence, docs build, and diff checks.

## Goal

Run the v41-selected normal-runtime target-init continuation discriminator on
the Pi 5 and classify whether the selected Image reaches TALOS: target init
after BootInfo parsing and target::init(&boot_info), before exceptions,
kernel_main, route-start, runtime-ready, packet-I/O, OpenSSH, or service
readiness.

## Scope Performed

- Promoted this ready hardware task after supervisor planning added v42 and
  verified v41 accepted at a5ef09d0914c63615e908d82e47c683b08f8069a.
- Acquired hardwareTestLock before lab publication, boot mutation, or Pi 5
  power action.
- Published only the v41 target-init marker-loop archive:
  target/tmp/selected-normal-runtime-target-init-v41.tar.gz.
- Captured candidate identity, fresh serial/TFTP cursors, same-window TFTP,
  serial marker output, final pre-restore identity, and restore proof.
- Ran the required inconclusive-run triage after the initial helper-owned
  capture was interrupted, then reran the same accepted candidate with changed
  capture ordering before restore.
- Restored the lab to
  phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirmed the final
  restored tree before releasing hardwareTestLock.

## Hardware Result

Candidate rerun: selected-normal-runtime-target-init-v42-rerun.

The accepted rerun published selected tree
478473f46d7dd2d64a42a9fd8f9e56e68de691a1237cb85108ef972f31045305 with
effective kernel_2712.img. The final pre-restore identity remained on that
same selected tree. The stable same-cursor TFTP delta observed 13 events and
served da591740/kernel_2712.img twice at 152,880 bytes. The saturated
direct-read serial window retained TALOS: target init 3,006 times. Post-restore
identity returned to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The first candidate attempt is classified only as inconclusive staging-window
evidence: the helper-owned serial capture hit a lab /serial/read 500, and
manual recovery found final identity on the restored/baseline tree with
104,136-byte kernel serves. That evidence was not used for acceptance. A
known-good control then proved baseline TFTP capture still observed the
104,136-byte kernel serves before the accepted candidate rerun changed capture
ordering by recording same-window TFTP and final selected identity before the
long serial direct-read.

## Terminal Classification

selected-normal-runtime-target-init-marker-retained.

This proves the selected 152,880-byte normal-runtime target-init marker-loop
archive reaches TALOS: target init on Pi 5 after BootInfo parsing and
target::init(&boot_info). It does not prove exceptions ready, kernel_main,
route-start, runtime-ready, packet-I/O, OpenSSH/generated-root retry, remote
receipt, compatibility/service readiness, ssh-ready=true, fake command
expansion, broad shell work, or phase transition.

selected_next_task:
phase12-ssh-live-tcp-selected-normal-runtime-target-init-continuation-closeout-v42-20260701.

planningNeeded: false.

## Findings

- fixed: published and ran the v41-selected target-init continuation
  discriminator under hardwareTestLock with selected-byte TFTP evidence and
  restore proof.
- fixed: completed the required inconclusive-run triage after the first
  helper-owned capture was interrupted by a lab serial-read failure.
- fixed: accepted only the rerun whose selected tree identity, selected fetch
  byte count, TFTP delta, final pre-restore identity, serial marker output, and
  restore proof are joined under the same candidate rerun.
- not-an-issue: the accepted target-init marker loop intentionally retains only
  TALOS: target init in the saturated direct-read window; earlier marker
  family counts are not required because v34, v36, v38, and v40 already prove
  those ordered predecessor frontiers.
- deferred: exceptions ready, kernel_main, route-start, runtime-ready,
  packet-I/O, OpenSSH compatibility, service readiness, ssh-ready=true, fake
  command expansion, broad shell work, and phase transition remain unproved.
- removed: packet-I/O, OpenSSH/generated-root retry, remote receipt,
  compatibility/service readiness, ssh-ready=true, fake command expansion,
  broad shell work, and phase transition as immediate successors.

## Evidence Map

- Classification:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-target-init-continuation-preflight-v42/classification.json.
- Evidence map:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-target-init-continuation-preflight-v42/evidence-map.json.
- Redacted run summary:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-target-init-continuation-preflight-v42/lab/run-summary-redacted.json.
- Accepted candidate rerun:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-target-init-continuation-preflight-v42/lab/v42-candidate-rerun/.
- Known-good control:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-target-init-continuation-preflight-v42/lab/v42-known-good-control/.
- Initial inconclusive candidate:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-target-init-continuation-preflight-v42/lab/v42-candidate/.
- Archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-target-init-continuation-preflight-v42/validation/archive-review.stdout.txt.
- Target-init archive review:
  tasks/evidence/2026-07-01-phase12-ssh-live-tcp-pi5-selected-normal-runtime-target-init-continuation-preflight-v42/validation/target-init-archive-review.stdout.txt.

## Redaction Review

Task-owned aggregate summaries retain task ids, run labels, hashes, byte
counts, marker names, marker counts, selected-tree hashes, classifications, and
validation outcomes. They do not retain raw serial text, raw TFTP peer/log-line
fields, packet payloads, SSH/session/key material, boot artifact bytes,
private data, or stable secret-derived identifiers. Raw local lab JSON is
retained only under task-owned evidence directories for local review.

## Validation

- git status --short --branch before lab action: pass; main ahead of origin
  with no uncommitted Talos changes before task-owned evidence creation.
- static archive/image review before publication: pass.
- Lab API GET /status and /boot/files after publication, final pre-restore,
  and after restore: pass.
- fresh serial cursor/drain and GET /tftp/logs cursor before Pi 5 power action:
  pass.
- stable same-cursor TFTP delta before restore: pass on accepted rerun.
- known-good control after first inconclusive candidate: pass for baseline TFTP
  visibility.
- restore to phase12-ssh-v10-openssh-clean-pre-20260624T074100Z and confirm
  post-restore tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10: pass.
- jq empty on task-owned JSON evidence and supervisor state: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass before commit.

Implementation commit: recorded in supervisor state after commit creation.
