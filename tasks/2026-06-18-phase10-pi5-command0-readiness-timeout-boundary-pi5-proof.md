# Phase 10 Pi 5 Command0 Readiness Timeout Boundary Pi 5 Proof

Task id: phase10-pi5-command0-readiness-timeout-boundary-pi5-proof-20260618

Status: accepted

Classification:
command0-readiness-timeout-boundary-blocked

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock, lab
controller API identity/status evidence, TFTP delta evidence, serial hardware
boot/output, known-good control, candidate rerun, task-owned JSON evidence,
docs build, and diff checks.

## Goal

Run one serialized Pi 5 proof that writes rootinfo from a fresh command=0
readiness boundary before command-loop timeout advancement and record whether
ordered command0 delivery passes with stable selected identity.

## Result

Command0 input delivery remains blocked. The accepted discriminator did not
find ordered command0 rootinfo delivery from a fresh command=0 boundary.

The first candidate retained selected immediate/final identity and baseline
restore proof, but its same-power-cycle TFTP precondition failed: the task
evidence recorded zero expected kernel fetches before command evaluation.
Because that capture was inconclusive for the TFTP precondition, the task ran
a known-good control and a candidate rerun before accepting any terminal
classification.

The known-good control observed TFTP activity. The candidate rerun retained
selected post-publish, immediate post-command, and final pre-restore identity,
with stable selected TFTP fetches for the 208984-byte kernel_2712.img. It then
failed before a valid command0 write boundary: retained readiness output had
already advanced through empty input timeouts to command=3, and the post-write
window reached command=4 without ordered rootinfo, dispatch command=0
status=handled, responses=1, or ready command=1 evidence from the saved fresh
command=0 boundary.

This task accepts only the hardware proof classification. It rejects command0
input delivery acceptance, source-response retention, generated-root
command-input success, storage, networking, SSH, Phase 11/12 expansion, and
phase transition.

## Findings

- fixed: retained the terminal Pi 5 proof evidence under task-owned evidence
  roots instead of relying on live lab state.
- fixed: after the first candidate had an inconclusive TFTP precondition, ran
  the documented known-good control and a candidate rerun before accepting a
  terminal classification.
- fixed: recorded immediate post-command and final pre-restore selected
  identity before restore in the accepted rerun.
- fixed: restored the baseline after hardware mutation and released
  hardwareTestLock.
- not-an-issue: the selected-kernel/TFTP precondition passed in the accepted
  rerun; the failing invariant is serial readiness/write ordering.
- deferred: readiness/write orchestration needs supervisor closeout before any
  source-response retention or generated-root command-input success follow-up.

## Evidence

- First candidate classification:
  tasks/evidence/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-pi5-proof/candidate-readiness-timeout-boundary-20260618T025821Z/classification.json.
- First candidate evidence map:
  tasks/evidence/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-pi5-proof/candidate-readiness-timeout-boundary-20260618T025821Z/evidence-map.json.
- Known-good control:
  tasks/evidence/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-pi5-proof/control-known-good-20260618T030304Z/classification.json.
- Accepted rerun classification:
  tasks/evidence/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-pi5-proof/candidate-readiness-timeout-boundary-rerun-20260618T030332Z/classification.json.
- Accepted rerun evidence map:
  tasks/evidence/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-pi5-proof/candidate-readiness-timeout-boundary-rerun-20260618T030332Z/evidence-map.json.
- Accepted rerun TFTP delta:
  tasks/evidence/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-pi5-proof/candidate-readiness-timeout-boundary-rerun-20260618T030332Z/tftp/tftp-delta-stable-pre-command.json.
- Accepted rerun readiness summary:
  tasks/evidence/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-pi5-proof/candidate-readiness-timeout-boundary-rerun-20260618T030332Z/serial/readiness-summary.json.
- Accepted rerun pre-write boundary:
  tasks/evidence/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-pi5-proof/candidate-readiness-timeout-boundary-rerun-20260618T030332Z/serial/command0-pre-write-boundary.json.
- Accepted rerun serial write:
  tasks/evidence/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-pi5-proof/candidate-readiness-timeout-boundary-rerun-20260618T030332Z/serial/command0-write.json.
- Accepted rerun post-write read:
  tasks/evidence/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-pi5-proof/candidate-readiness-timeout-boundary-rerun-20260618T030332Z/serial/command0-post-write-read.json.
- Accepted rerun immediate identity:
  tasks/evidence/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-pi5-proof/candidate-readiness-timeout-boundary-rerun-20260618T030332Z/immediate-post-command-status.json.
- Accepted rerun final pre-restore identity:
  tasks/evidence/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-pi5-proof/candidate-readiness-timeout-boundary-rerun-20260618T030332Z/final-pre-restore-status.json.
- Accepted rerun restore proof:
  tasks/evidence/2026-06-18-phase10-pi5-command0-readiness-timeout-boundary-pi5-proof/candidate-readiness-timeout-boundary-rerun-20260618T030332Z/restore/post-restore-status.json.

## Acceptance Check

- HardwareTestLock acquired before lab mutation and released after restore:
  satisfied by state iteration history and retained restore evidence.
- Candidate identity, fresh serial cursor, selected-kernel/TFTP delta, and
  known-good control decision recorded before terminal classification:
  satisfied.
- Ordered command0 delivery accepted only from a fresh command=0 boundary:
  satisfied by rejection; no command0 delivery acceptance was made.
- Immediate and final pre-restore selected identity recorded before restore:
  satisfied in the accepted rerun.
- Exactly one terminal classification from the accepted discriminator contract:
  satisfied, command0-readiness-timeout-boundary-blocked.
- Inconclusive capture/staging evidence does not unblock later success claims:
  satisfied.

## Validation

- Pi 5 serialized hardware proof under hardwareTestLock: pass, blocked
  classification.
- Inconclusive-run triage before accepting rerun: pass, known-good control
  observed TFTP activity before candidate rerun.
- jq empty on task-owned JSON evidence: pass.
- evidence map references retained publish, status, serial, TFTP,
  immediate/final identity, and restore artifacts: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-command0-readiness-timeout-boundary-closeout-20260618 on
the next worker wake if dependencies remain satisfied, hardwareTestLock is
unlocked/restored, supervisorIntervention is inactive, and the repository has
no conflicting uncommitted changes.
