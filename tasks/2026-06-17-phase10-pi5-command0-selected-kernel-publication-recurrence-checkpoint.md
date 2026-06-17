# Phase 10 Pi 5 Command0 Selected-Kernel Publication Recurrence Checkpoint

Task id: phase10-pi5-command0-selected-kernel-publication-recurrence-checkpoint-20260617

Status: accepted

Classification:
selected-kernel-publication-recurrence-checkpoint-paired-sentinel-core-selected

Evidence level: static/source/task evidence inspection, retained
lab-controller API evidence review, retained serial hardware boot/output
evidence review, retained stable same-cursor TFTP delta evidence review,
task-owned JSON evidence, docs build, and diff checks. No hardware run, boot
archive publication, lab mutation, hardwareTestLock acquisition, command0
write, source-response-retention proof, generated-root command-input
acceptance, storage, networking, SSH, Phase 11/12 expansion, or phase
transition was performed.

## Goal

Name the recurring selected-kernel publication/TFTP invariant and select the
next no-command-write discriminator before command0 write-delivery work can
resume.

## Checkpoint

The first still-failing invariant remains same-power-cycle TFTP served bytes
not matching the selected kernel, with a secondary final pre-restore identity
regression to the baseline tree. The selected-kernel stability proof retained
post-publish lab API identity for selected tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212 and
208984-byte da591740/kernel_2712.img, but the same-power-cycle TFTP delta
served two 104136-byte baseline kernel_2712.img fetches and final pre-restore
identity exposed baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

That recurrence contradicts the 2026-06-14 capture-staging minimal sentinel
recovery at the selected-tree/TFTP/final-identity boundary. The accepted
sentinel control selected tree
9e3442962d40beff3b31668065df4f5d8ac37ee770fa3c4bba725d049bc78db3 and saw two
matching 47832-byte da591740/kernel_2712.img TFTP serves with final
pre-restore identity still on the selected tree. The accepted candidate rerun
selected tree 520785f412ba93da8c25577e5f5e4635ffba02b2969fbf3e02a346e97e061799
and saw two matching 47848-byte serves with final pre-restore identity still
on the selected tree. Both runs restored to the 104136-byte baseline tree.

The recurrence therefore is not a generic claim that the lab cannot ever
publish selected trees. It is a renewed selected-kernel/TFTP publication
failure on the command0 generated-root candidate after an earlier minimal
sentinel pair proved the capture-staging path could recover. The next
discriminator must keep command writes out of scope and reprove the publication
boundary with paired selected-kernel sentinel evidence before any command0
write-delivery retry.

## Selected Discriminator

Selected discriminator:
selected-kernel-paired-sentinel-publication-boundary-v1.

The discriminator accepts only paired no-command-write candidate/control
selected-kernel evidence where post-publish identity exposes the selected
tree/kernel bytes, same-power-cycle TFTP serves da591740/kernel_2712.img at the
selected byte count, final pre-restore identity still exposes the selected
tree/kernel bytes, and restore returns to the saved baseline. It must reject
retained baseline-served selected-kernel evidence, single-run precondition
proofs as durable unblockers, stale/no-fresh-TFTP evidence, final identity
mismatch, restore failure, and any command0 write-delivery or
source-response-retention claim.

The selected next task is
phase10-pi5-command0-selected-kernel-paired-sentinel-core-20260617.

## Findings

- fixed: compared the accepted 2026-06-14 capture-staging minimal sentinel
  recovery with the 2026-06-17 selected-kernel stability regression at the
  shared selected-tree/TFTP/final-identity boundary.
- fixed: preserved the first still-failing invariant as same-power-cycle TFTP
  served bytes not matching the selected kernel.
- fixed: selected exactly one no-command-write follow-up discriminator:
  selected-kernel-paired-sentinel-publication-boundary-v1.
- deferred: hardware proof remains dependency-gated behind the paired sentinel
  core task; this checkpoint performs no hardware, publication, lab mutation,
  or command0 retry.
- not-an-issue: the 2026-06-14 minimal sentinel recovery remains valid for its
  no-MDIO/no-Ethernet minimal boundary; it just does not prove the later
  command0 generated-root selected-kernel candidate.
- rejected: command0 write-delivery success, source-response retention,
  generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition.

## Evidence

- Selected-kernel stability closeout:
  tasks/2026-06-17-phase10-pi5-command0-selected-kernel-stability-closeout.md.
- Selected-kernel stability closeout classification:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-closeout/classification.json.
- Selected-kernel stability proof:
  tasks/2026-06-17-phase10-pi5-command0-selected-kernel-stability-pi5-proof.md.
- Selected-kernel stability proof classification:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-pi5-proof/classification.json.
- Selected-kernel stability TFTP delta:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-pi5-proof/candidate-selected-kernel-stability-20260617T180240Z/capture/tftp-delta-stable-pre-restore.json.
- Prior selected-kernel/TFTP precondition lineage:
  tasks/2026-06-17-phase10-pi5-command0-selected-kernel-regression-reconciliation-source-checkpoint.md.
- Capture-staging minimal sentinel proof:
  tasks/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof.md.
- Capture-staging minimal sentinel classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-minimal-sentinel-pi5-proof/classification.json.
- Capture-staging recovery closeout:
  tasks/2026-06-14-phase12-rp1-ethernet-capture-staging-recovery-closeout.md.
- Capture-staging recovery closeout classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-capture-staging-recovery-closeout/classification.json.
- Lab-controller served-root evidence contract:
  docs/src/project/lab-controller.md.
- Selected-kernel stability helper:
  scripts/rpi5-selected-kernel-stability-discriminator.sh.
- This checkpoint classification:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-publication-recurrence-checkpoint/classification.json.
- This checkpoint evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-publication-recurrence-checkpoint/evidence-map.json.

## Acceptance Check

- The checkpoint names the first still-failing invariant using retained
  evidence, not a same-shaped command0 retry: satisfied.
- The checkpoint explicitly compares the 2026-06-14 accepted capture-staging
  minimal sentinel recovery with the 2026-06-17 selected-kernel stability
  regression: satisfied.
- The checkpoint selects exactly one next task id or records a precise blocker
  with planningNeeded=true: satisfied; selected_next_task is
  phase10-pi5-command0-selected-kernel-paired-sentinel-core-20260617.
- If a hardware follow-up is selected, it is a no-command-write
  selected-kernel/TFTP identity discriminator with paired control/rerun triage,
  not command0 write delivery: satisfied; hardware is dependency-gated behind
  the paired sentinel core.
- Rejected claims include command0 write-delivery success, source-response
  retention, generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition: satisfied.

## Validation

- static/source/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-command0-selected-kernel-paired-sentinel-core-20260617 on
the next worker wake if dependencies remain satisfied and the accepted
checkpoint-selected discriminator remains
selected-kernel-paired-sentinel-publication-boundary-v1. Do not run hardware,
publish a boot archive, mutate the lab, retry command0, select source-response
retention, accept generated-root command-input success, or transition phases
from this checkpoint.
