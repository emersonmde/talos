# Phase 10 Pi 5 Serial Command 0 Saturated-Capture Pi 5 Proof

Task id: phase10-pi5-serial-command0-saturated-capture-pi5-proof-20260617

Status: accepted

Classification:
command0-saturated-capture-blocked-selected-kernel-tftp-precondition-regressed

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock,
lab-controller API, saturated direct /serial/read readiness, stable same-cursor
TFTP delta, final identity, restore proof, task-owned guard validator, JSON
evidence, docs build, and diff checks.

## Goal

Run one serialized Pi 5 proof of command0 write delivery using the accepted
command0-saturated-capture-guard-v1 contract.

## Result

The proof did not evaluate command0 write delivery. The first failing invariant
was selected-kernel/TFTP agreement: the run published the selected generated-root
candidate, but the same-power-cycle stable TFTP delta retained two
da591740/kernel_2712.img serves at 104136 bytes instead of the selected
208984-byte candidate. Final pre-restore identity also exposed the baseline
tree and 104136-byte kernel.

The saturated direct-read readiness capture retained generated-root readiness
text, including source=firmware-initramfs, reason=valid-artifact, ready
command=0, and a visible talos> prompt. It also overran into command0 timeout
and ready command=1 before rootinfo was written. Because the selected-kernel
TFTP precondition had already failed, no command0 write was sent and no
command0 write-delivery claim is accepted or rejected by this task.

The lab was explicitly restored to the baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings

- fixed: hardwareTestLock was held across publication, power-cycle, TFTP/final
  identity capture, restore, and evidence classification.
- fixed: candidate publication evidence retained the selected tree
  06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212 and
  208984-byte da591740/kernel_2712.img before power-cycle.
- blocked: same-power-cycle TFTP served two baseline-sized 104136-byte
  da591740/kernel_2712.img fetches, so the selected-kernel/TFTP precondition
  regressed before command0 behavior could be evaluated.
- blocked: final pre-restore identity exposed the baseline tree and
  104136-byte kernel, not the selected candidate tree.
- not-an-issue: restore proof returned the lab to the baseline tree.
- deferred: saturated-capture closeout must reconcile this precondition
  regression with the previously accepted selected-kernel/TFTP precondition
  before any same-shaped retry or source-response retention proof is selected.
- rejected: command0 write-delivery success, command0 source-response retention
  success, generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition.

## Evidence

- Task classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-saturated-capture-pi5-proof/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-saturated-capture-pi5-proof/evidence-map.json.
- Run evidence:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-saturated-capture-pi5-proof/candidate-command0-saturated-capture-20260617T163347Z/.
- Run classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-saturated-capture-pi5-proof/candidate-command0-saturated-capture-20260617T163347Z/classification.json.
- Guard evidence:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-saturated-capture-pi5-proof/candidate-command0-saturated-capture-20260617T163347Z/saturated-capture-evidence.json.
- Stable TFTP delta:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-saturated-capture-pi5-proof/candidate-command0-saturated-capture-20260617T163347Z/tftp/tftp-delta-stable-pre-restore.json.
- Restore proof:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-saturated-capture-pi5-proof/candidate-command0-saturated-capture-20260617T163347Z/restore/post-restore-boot-files.json.

## Acceptance Check

- Accepted proof retains selected-kernel/TFTP agreement, same-boot
  generated-root readiness, accepted rootinfo write, ordered command0
  write-delivery evidence, final identity, and restore proof: not satisfied;
  selected-kernel/TFTP agreement and final identity failed before command0
  write.
- Blocked proof records the first failing invariant and distinguishes
  capture endpoint failure, write delivery failure, staging/TFTP failure, and
  stale later-command processing: satisfied as a selected-kernel/TFTP
  precondition regression.
- Inconclusive-run triage is retained before any code or contract change:
  satisfied; no code changes follow this blocked proof in this task.
- selected_next_task is
  phase10-pi5-serial-command0-saturated-capture-closeout-20260617: satisfied.
- Rejected claims remain explicit: satisfied.

## Validation

- serialized Pi 5 hardware proof under hardwareTestLock: blocked at
  selected-kernel/TFTP precondition regression.
- candidate identity via lab API before power-cycle: pass for post-publish
  selected tree and selected kernel bytes.
- fresh serial cursor/readiness evidence per accepted guard: not accepted for
  command0; saturated direct read retained readiness but also stale command0
  timeout before write, and selected-kernel/TFTP failed first.
- stable same-cursor TFTP delta before restore: blocked; two selected fetch
  paths were served at 104136 bytes.
- post-run baseline restore proof: pass.
- task-owned proof validator: expected reject for the retained blocked evidence.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-serial-command0-saturated-capture-closeout-20260617 on the
next worker wake if dependencies remain satisfied and hardwareTestLock remains
unlocked/restored. The closeout must reconcile this TFTP/final-identity
regression before selecting any same-shaped command0 retry. Do not accept
command0 source-response retention, generated-root command-input success,
storage, networking, SSH, Phase 11/12 expansion, or phase transition from this
proof.
