# Phase 10 Pi 5 Serial Command0 Lab Boundary After Precondition Recovery Closeout

Task id: phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-closeout-20260617

Status: accepted

Classification:
command0-lab-boundary-after-precondition-closed-final-identity-regressed-planning-needed

Evidence level: task/evidence consistency review, accepted selected-kernel/TFTP
precondition closeout, accepted lab-boundary discriminator core, accepted
serialized Pi 5 after-precondition recovery proof, task-owned JSON evidence,
docs build, and diff checks. No code change, Pi 5 hardware run, boot archive
publication, power-cycle, lab mutation, hardwareTestLock acquisition,
source-response retention proof, generated-root command-input success, storage,
networking, SSH, Phase 11/12 expansion, or phase transition was performed.

## Goal

Close out the after-precondition-recovery lab-boundary proof and decide whether
command0 input delivery can select source-response retention v3.

## Closeout

The accepted selected-kernel/TFTP precondition closeout allowed exactly one
command0 lab-boundary retry after no-command-write Pi 5 evidence showed
selected post-publish identity, two same-power-cycle selected 208984-byte TFTP
serves, final selected identity, and baseline restore.

The after-precondition-recovery Pi 5 proof then satisfied the pre-command
selected-kernel/TFTP gate again for the generated-root command-input candidate:
post-publish status reported selected tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212,
effective kernel_2712.img, and a 208984-byte da591740/kernel_2712.img. The
stable same-power-cycle TFTP delta retained two 208984-byte
da591740/kernel_2712.img serves before command0 evaluation.

The accepted lab-boundary serial discriminator also passed. The candidate
reached source=firmware-initramfs, reason=valid-artifact, ready command=0, and
the talos> prompt. The immediate pre-write read was empty, POST /serial/write
accepted the rootinfo payload with 9 bytes, and the prearmed live
POST /serial/read retained rootinfo, line command=0, dispatch command=0
status=handled, responses=1, and ready command=1 in order.

Command0 input delivery is still not accepted from this proof because the final
pre-restore identity gate regressed before the explicit restore call:
lab-controller status reported the baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 and a
104136-byte da591740/kernel_2712.img. The restore then confirmed the same
baseline tree. The first failing invariant remains
final-pre-restore-identity-regressed-after-command0.

Source-response retention v3 is not selected because command0 input delivery
was not accepted under a stable final selected-candidate identity. Generated-root
command-input success, storage, networking, SSH, Phase 11/12 expansion, and
phase transition remain unaccepted.

Supervisor planning is required before another worker task is promoted. Any
follow-up should explain why selected-kernel/TFTP and ordered command0 serial
delivery can both pass before final lab identity regresses to baseline, or
define a distinct discriminator for that exact boundary.

## Findings

- fixed: reconciled accepted selected-kernel/TFTP precondition evidence with
  the after-precondition-recovery lab-boundary proof.
- fixed: preserved ordered prearmed command0 delivery evidence as useful
  non-accepting evidence: rootinfo reached command0 and produced dispatch,
  response, and next-ready output.
- blocked: command0 input delivery remains unaccepted because final
  pre-restore identity regressed to the baseline tree after command0 and before
  the explicit restore call.
- deferred: source-response retention v3 remains dependency-gated until
  command0 input delivery is accepted by an explicit future task.
- not-an-issue: no hardware lock, boot publication, lab mutation, source
  change, or proof-helper change was required for this static closeout.
- rejected: source-response retention, generated-root command-input success,
  storage, networking, SSH, Phase 11/12 expansion, and phase transition.

## Evidence

- Selected-kernel/TFTP precondition closeout:
  tasks/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-closeout.md.
- Lab-boundary discriminator core:
  tasks/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-discriminator-core.md.
- After-precondition recovery Pi 5 proof:
  tasks/2026-06-17-phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-pi5-proof.md.
- After-precondition recovery proof classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-pi5-proof/candidate-rerun-command0-after-precondition-prearmed-read-20260618T002703Z/classification.json.
- After-precondition recovery proof evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-pi5-proof/candidate-rerun-command0-after-precondition-prearmed-read-20260618T002703Z/evidence-map.json.
- Selected-kernel/TFTP precondition evidence:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-pi5-proof/candidate-rerun-command0-after-precondition-prearmed-read-20260618T002703Z/tftp/tftp-delta-stable-pre-command.json.
- Prearmed live-read evidence:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-pi5-proof/candidate-rerun-command0-after-precondition-prearmed-read-20260618T002703Z/serial/command0-prearmed-read.json.
- Final pre-restore identity:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-pi5-proof/candidate-rerun-command0-after-precondition-prearmed-read-20260618T002703Z/final-pre-restore-status.json.
- This closeout classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-closeout/classification.json.
- This closeout evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-closeout/evidence-map.json.

## Acceptance Check

- Closeout accurately reconciles selected-kernel/TFTP and command0 serial
  evidence from the after-recovery proof: satisfied.
- Command0 input delivery is accepted only if both selected-kernel/TFTP
  precondition and ordered command0 serial delivery passed: satisfied by not
  accepting input delivery because the final pre-restore identity gate regressed
  after those passes.
- Source-response retention v3 is selected only if command0 input delivery is
  accepted: satisfied by not selecting source-response retention v3.
- Generated-root command-input success and phase transition remain rejected
  unless separately proven by explicit future tasks: satisfied.
- selected_next_task is source-response retention v3, or null with
  planningNeeded=true and planningReason: satisfied with selected_next_task
  null and planningNeeded=true.

## Validation

- task/evidence consistency review: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

selected_next_task=null and planningNeeded=true. Supervisor planning is required
before any follow-up worker task is promoted. The retained selected-kernel/TFTP
precondition and ordered command0 serial delivery evidence must not unblock
source-response retention v3 or generated-root command-input success until a
future task resolves the final pre-restore identity regression.
