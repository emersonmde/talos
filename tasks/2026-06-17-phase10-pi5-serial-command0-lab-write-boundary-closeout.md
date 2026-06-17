# Phase 10 Pi 5 Serial Command0 Lab Write Boundary Closeout

Task id: phase10-pi5-serial-command0-lab-write-boundary-closeout-20260617

Status: accepted

Classification:
command0-lab-write-boundary-closed-selected-kernel-tftp-precondition-regressed-planning-needed

Evidence level: static/task/evidence consistency review, accepted lab-boundary
discriminator core, accepted serialized Pi 5 proof, task-owned JSON evidence,
docs build, and diff checks. No code change, hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, source-response
retention proof, generated-root command-input acceptance, storage, networking,
SSH, Phase 11/12 expansion, or phase transition was performed.

## Goal

Close out the lab write-boundary Pi 5 proof and decide whether command0 input
delivery, source-response retention, or another bounded follow-up can be
selected.

## Closeout

The accepted lab-boundary discriminator core selected a prearmed live
/serial/read around the normal POST /serial/write rootinfo payload. That shape
was chosen because the previous saturated post-write observe proof retained
zero bytes and a post-write-only direct read could still race command0 output.
The selected proof was allowed to accept command0 input delivery only after the
selected-kernel/TFTP precondition passed for the generated-root command-input
candidate.

The Pi 5 proof published the selected generated-root candidate and the lab API
reported the expected selected tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212 with a
208984-byte da591740/kernel_2712.img. The same-power-cycle TFTP delta then
contradicted the selected identity: both da591740/kernel_2712.img fetches were
104136 bytes, and final pre-restore identity reported the baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The serial discriminator itself retained useful but non-accepting evidence.
The boot reached generated-root command0 readiness and a visible talos>
prompt, the immediate pre-write read was empty, POST /serial/write accepted
rootinfo with 9 bytes, and the prearmed live read retained rootinfo, line
command=0, dispatch command=0 status=handled, responses=1, and ready command=1
in order. That proves the selected capture shape can observe command0 delivery,
but it cannot accept command0 input delivery for the selected candidate because
the selected-kernel/TFTP precondition failed first.

The first failing invariant remains selected-kernel-tftp-precondition-missing.
Source-response retention is not selected because command0 input delivery was
not accepted. Generated-root command-input success, storage, networking, SSH,
Phase 11/12 expansion, and phase transition remain unaccepted.

Supervisor planning is required before another worker task is promoted. Any
follow-up must reconcile the recurrence of selected-kernel/TFTP regression
against the earlier paired-sentinel recovery evidence before using the retained
serial command0 delivery shape as an unblocker.

## Findings

- fixed: reconciled the accepted lab-boundary core and serialized Pi 5 proof
  into a terminal closeout classification.
- fixed: preserved the positive prearmed serial read evidence as useful
  non-accepting evidence: rootinfo reached command0 and produced ordered
  dispatch/response/ready output.
- blocked: command0 input delivery remains unaccepted because same-power-cycle
  TFTP served baseline-sized 104136-byte kernel_2712.img files and final
  pre-restore identity exposed the baseline tree.
- deferred: source-response retention remains non-evaluable until command0
  input delivery is separately accepted under a satisfied selected-kernel/TFTP
  precondition.
- deferred: any retry, selected-kernel/TFTP reconciliation, or helper
  quarantine needs supervisor planning; this closeout selects no same-shaped
  Pi 5 retry.
- not-an-issue: no hardware lock, boot publication, lab mutation, source
  change, or proof-helper change was required for this static closeout.
- rejected: source-response retention, generated-root command-input success,
  storage, networking, SSH, Phase 11/12 expansion, and phase transition.

## Evidence

- Lab-boundary discriminator core:
  tasks/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-discriminator-core.md.
- Lab-boundary discriminator core classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-discriminator-core/classification.json.
- Lab-boundary discriminator contract:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-discriminator-core/discriminator-contract.json.
- Lab-boundary Pi 5 proof:
  tasks/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-pi5-proof.md.
- Lab-boundary Pi 5 proof classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-pi5-proof/candidate-command0-lab-boundary-prearmed-read-20260617T222745Z/classification.json.
- Lab-boundary Pi 5 proof evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-pi5-proof/candidate-command0-lab-boundary-prearmed-read-20260617T222745Z/evidence-map.json.
- Prearmed live-read evidence:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-pi5-proof/candidate-command0-lab-boundary-prearmed-read-20260617T222745Z/serial/command0-prearmed-read.json.
- Stable TFTP delta:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-pi5-proof/candidate-command0-lab-boundary-prearmed-read-20260617T222745Z/tftp/tftp-delta-stable-pre-restore.json.
- Final pre-restore identity:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-pi5-proof/candidate-command0-lab-boundary-prearmed-read-20260617T222745Z/final-pre-restore-status.json.
- This closeout classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-closeout/classification.json.
- This closeout evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-closeout/evidence-map.json.

## Acceptance Check

- Closeout accurately reconciles accepted evidence and rejected claims from
  the lab-boundary chain: satisfied.
- Command0 input delivery is accepted only with evidence that /serial/write
  payload reached command0 and produced the expected command0 effect:
  satisfied by not accepting input delivery because the selected-kernel/TFTP
  precondition failed first.
- Source-response retention is selected only if command0 input delivery is
  accepted: satisfied by not selecting source-response retention.
- Generated-root command-input success and phase transition remain rejected
  unless separately proven by explicit future tasks: satisfied.
- selected_next_task is explicit and dependency-satisfied, or null with
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
before any follow-up worker task is promoted. The retained prearmed serial
evidence should not unblock source-response retention or generated-root
command-input success unless a future task first reconciles and satisfies the
selected-kernel/TFTP precondition for the same candidate boundary.
