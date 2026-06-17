# Phase 10 Pi 5 Command0 Selected-Kernel Lab-Boundary Recurrence Checkpoint

Task id: phase10-pi5-command0-selected-kernel-lab-boundary-recurrence-checkpoint-20260617

Status: accepted

Classification:
selected-kernel-lab-boundary-recurrence-checkpoint-precondition-discriminator-selected

Evidence level: static/source/task/evidence consistency review, accepted
paired-sentinel recovery evidence, accepted lab-boundary regression evidence,
task-owned JSON evidence, docs build, and diff checks. No code change,
hardware run, boot archive publication, lab mutation, hardwareTestLock
acquisition, command0 write, source-response-retention proof, generated-root
command-input acceptance, storage, networking, SSH, Phase 11/12 expansion, or
phase transition was performed.

## Goal

Reconcile the selected-kernel/TFTP recurrence observed after the accepted
paired-sentinel recovery and decide whether the next step can be a bounded
precondition discriminator rather than another command0 retry.

## Checkpoint

The accepted paired-sentinel Pi 5 proof recovered the selected-kernel/TFTP
publication boundary for a no-command-write candidate/control pair. The
generated-root command-input candidate selected tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212 with a
208984-byte da591740/kernel_2712.img, retained two same-power-cycle 208984-byte
TFTP serves, retained final selected identity, and restored to baseline. The
paired control selected tree
9e3442962d40beff3b31668065df4f5d8ac37ee770fa3c4bba725d049bc78db3 with a
47832-byte da591740/kernel_2712.img, retained two 47832-byte TFTP serves,
retained final selected identity, and restored to the same baseline. That
evidence proves the selected-kernel publication path can work for the paired
sentinel boundary.

The later lab-boundary proof selected the same generated-root command-input
candidate tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212 and the lab
API exposed da591740/kernel_2712.img at 208984 bytes before power. Its
same-power-cycle TFTP delta then served da591740/kernel_2712.img twice at
104136 bytes, and final pre-restore identity reported the baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. The
positive prearmed serial delivery shape is retained as useful evidence, but
command0 input delivery remains unaccepted because the selected-kernel/TFTP
precondition failed first.

The paired-sentinel recovery therefore did not durably unblock later command0
lab-boundary proofs by itself. Its acceptance is scope-limited to the retained
no-command-write candidate/control runs and their selected same-power-cycle
TFTP/final-identity evidence. It must not be used as a standing precondition
pass for subsequent command0 retries after any later run observes baseline
TFTP serves or baseline final identity.

The selected next task is
phase10-pi5-command0-selected-kernel-precondition-discriminator-core-20260617.
That task should encode the exact selected-kernel/TFTP precondition contract
needed before command0 lab-boundary evidence can be accepted again, replay the
retained lab-boundary regression as a negative, and reject single-run durable
unblocker claims. No hardware is selected by this checkpoint.

## Findings

- fixed: reconciled the accepted paired-sentinel recovery with the later
  lab-boundary selected-kernel/TFTP regression.
- fixed: named the recurring failed invariant as
  selected-kernel-tftp-precondition-missing: post-publish selected identity is
  insufficient when same-power-cycle TFTP serves baseline bytes or final
  pre-restore identity is baseline.
- fixed: selected the local/static precondition discriminator core instead of a
  same-shaped command0 retry.
- deferred: command0 lab-boundary retry remains blocked until a future
  precondition closeout accepts fresh selected-kernel/TFTP evidence under the
  accepted discriminator.
- deferred: source-response retention remains non-evaluable until command0
  input delivery is separately accepted under a satisfied selected-kernel/TFTP
  precondition.
- not-an-issue: no helper/code change, hardware lock, boot publication, lab
  mutation, command write, or mdbook architecture rewrite was required for this
  static checkpoint.
- rejected: command0 input delivery acceptance, source-response retention,
  generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition.

## Scope-Limited Evidence

- The paired-sentinel recovery proof remains accepted only for the retained
  no-command-write candidate/control publication boundary. It is not a durable
  unblocker for later command0 lab-boundary proofs.
- The earlier selected-kernel/TFTP precondition acceptance remains accepted for
  its retained run, but later baseline-served evidence supersedes it as an
  unblocker for command0 retries.
- The lab-boundary proof's prearmed serial command0 delivery shape remains
  useful non-accepting evidence. It cannot accept command0 input delivery until
  the selected-kernel/TFTP precondition is satisfied for the same selected
  candidate boundary.

## Evidence

- Paired-sentinel recovery closeout:
  tasks/2026-06-17-phase10-pi5-command0-selected-kernel-recovery-closeout.md.
- Paired-sentinel recovery classification:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-recovery-closeout/classification.json.
- Paired-sentinel Pi 5 proof:
  tasks/2026-06-17-phase10-pi5-command0-selected-kernel-paired-sentinel-pi5-proof.md.
- Paired-sentinel Pi 5 proof classification:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-paired-sentinel-pi5-proof/classification.json.
- Lab-boundary Pi 5 proof:
  tasks/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-pi5-proof.md.
- Lab-boundary Pi 5 proof classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-pi5-proof/candidate-command0-lab-boundary-prearmed-read-20260617T222745Z/classification.json.
- Lab-boundary closeout:
  tasks/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-closeout.md.
- Lab-boundary closeout classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-closeout/classification.json.
- This checkpoint classification:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-lab-boundary-recurrence-checkpoint/classification.json.
- This checkpoint evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-lab-boundary-recurrence-checkpoint/evidence-map.json.

## Acceptance Check

- Checkpoint names the exact selected-kernel/TFTP invariant that failed in the
  lab-boundary proof using retained evidence, not a same-shaped retry:
  satisfied.
- Checkpoint explicitly reconciles why the accepted paired-sentinel recovery
  did not durably unblock the later command0 lab-boundary proof, or records the
  missing evidence needed to decide: satisfied by scope-limiting the
  paired-sentinel proof to its retained no-command-write boundary.
- Any earlier evidence that should no longer unblock command0 retries is
  quarantined or scope-limited in the task record: satisfied.
- selected_next_task is
  phase10-pi5-command0-selected-kernel-precondition-discriminator-core-20260617,
  or null with planningNeeded=true and a precise planningReason: satisfied
  with the precondition discriminator core selected.
- Rejected claims include command0 input delivery acceptance, source-response
  retention, generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition: satisfied.

## Validation

- static/source/task/evidence consistency review: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote
phase10-pi5-command0-selected-kernel-precondition-discriminator-core-20260617
on the next worker wake if dependencies remain satisfied. Do not run hardware
or retry command0 from this checkpoint.
