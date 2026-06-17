# Phase 10 Pi 5 Command0 Selected-Kernel Stability Closeout

Task id: phase10-pi5-command0-selected-kernel-stability-closeout-20260617

Status: accepted

Classification:
selected-kernel-stability-closed-tftp-served-baseline-final-identity-regressed-planning-needed

Evidence level: static/source/task evidence inspection, accepted regression
reconciliation checkpoint, accepted local/static discriminator core, accepted
serialized Pi 5 proof/blocker, task-owned JSON evidence, docs build, and diff
checks. No hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, command0 write, source-response-retention proof,
generated-root command-input acceptance, storage, networking, SSH,
Phase 11/12 expansion, or phase transition was performed.

## Goal

Close out the selected-kernel stability discriminator and decide whether
command0 write-delivery work can resume.

## Closeout

The regression reconciliation checkpoint narrowed the command0 retry
precondition to a selected-kernel stability invariant: post-publish boot files
must expose the selected tree and selected da591740/kernel_2712.img byte count,
the same-power-cycle TFTP delta must serve that selected kernel byte count, the
final pre-restore boot files must still expose the selected tree and kernel
byte count, and restore must return to the saved baseline. It also quarantined
the earlier selected-kernel/TFTP precondition proof as a single-run proof that
does not durably unblock later command0 retries by itself.

The discriminator core encoded that invariant in
scripts/rpi5-selected-kernel-stability-discriminator.sh and rejected retained
baseline-served regression evidence, no-fresh-TFTP evidence, final-identity
mismatch evidence, stale cursor/cursor-boundary ambiguity, and restore failure.

The serialized Pi 5 proof then decisively blocked selected-kernel stability. It
published the selected generated-root candidate tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212 with
da591740/kernel_2712.img at 208984 bytes, but the stable same-power-cycle TFTP
delta served two da591740/kernel_2712.img fetches at 104136 bytes and final
pre-restore identity exposed baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10. Restore
returned the lab to that baseline tree.

Selected-kernel stability is therefore not accepted. The first failing
invariant remains same-power-cycle TFTP served bytes do not match the selected
kernel, with a secondary final-identity regression to the baseline tree. This
blocks command0 write delivery evaluation before any rootinfo or command0 write
is attempted.

Supervisor planning is required before any publication/TFTP-root
reconciliation, helper quarantine, command0 write-delivery retry,
source-response-retention proof, generated-root command-input acceptance,
transition checkpoint, storage work, networking, SSH, Phase 11/12 expansion, or
phase transition. This closeout selects no next command0 task and no
same-shaped retry.

## Findings

- fixed: reconciled the regression checkpoint, selected-kernel stability
  discriminator core, and serialized Pi 5 proof/blocker into one terminal
  closeout classification.
- fixed: preserved the post-publish selected identity evidence for selected
  tree 06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212 and
  208984-byte da591740/kernel_2712.img.
- blocked: selected-kernel stability remains unaccepted because
  same-power-cycle TFTP served two 104136-byte baseline kernel fetches instead
  of the selected 208984-byte kernel.
- blocked: final pre-restore identity exposed baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 instead of
  the selected tree.
- deferred: any retry, publication/TFTP-root reconciliation, helper quarantine,
  or source-response-retention proof needs supervisor planning because this
  closeout selects no follow-up task.
- not-an-issue: no hardware lock, boot publication, lab mutation, command0
  write, or source change was required for this static closeout.
- rejected: command0 write-delivery success, source-response retention,
  generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition.

## Evidence

- Regression reconciliation checkpoint:
  tasks/2026-06-17-phase10-pi5-command0-selected-kernel-regression-reconciliation-source-checkpoint.md.
- Regression reconciliation classification:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-regression-reconciliation-source-checkpoint/classification.json.
- Selected-kernel stability discriminator core:
  tasks/2026-06-17-phase10-pi5-command0-selected-kernel-stability-discriminator-core.md.
- Selected-kernel stability discriminator classification:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-discriminator-core/classification.json.
- Selected-kernel stability Pi 5 proof/blocker:
  tasks/2026-06-17-phase10-pi5-command0-selected-kernel-stability-pi5-proof.md.
- Selected-kernel stability Pi 5 proof/blocker classification:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-pi5-proof/classification.json.
- Selected-kernel stability Pi 5 proof/blocker evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-pi5-proof/evidence-map.json.
- Stable TFTP delta:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-pi5-proof/candidate-selected-kernel-stability-20260617T180240Z/capture/tftp-delta-stable-pre-restore.json.
- Final pre-restore identity:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-pi5-proof/candidate-selected-kernel-stability-20260617T180240Z/capture/final-pre-restore-boot-files.json.
- Restore proof:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-pi5-proof/candidate-selected-kernel-stability-20260617T180240Z/capture/restore-snapshot.json.
- This closeout classification:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-closeout/classification.json.
- This closeout evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-closeout/evidence-map.json.

## Acceptance Check

- Closeout classification matches retained checkpoint/core/proof evidence:
  satisfied.
- Selected-kernel stability is accepted, blocked, or paused with an unambiguous
  first failing invariant: satisfied as blocked on same-power-cycle TFTP served
  bytes not matching the selected kernel.
- If stability is accepted, any selected next command0 task must be a distinct
  discriminator and must not claim source-response retention or generated-root
  command-input success: not applicable because stability is not accepted.
- If stability remains blocked or inconclusive, planningNeeded=true or a
  precise blocker is recorded and no same-shaped retry is selected: satisfied;
  supervisor planning is required before any follow-up and selected_next_task is
  null.
- Rejected claims include command0 write-delivery success unless separately
  proven, source-response retention, generated-root command-input success,
  storage, networking, SSH, Phase 11/12 expansion, and phase transition:
  satisfied.

## Validation

- static/source/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Supervisor planning is required before any next worker task is promoted.
Selected-kernel stability remains blocked at same-power-cycle TFTP served bytes
and final pre-restore identity, so no same-shaped command0 retry,
source-response-retention proof, generated-root command-input acceptance, or
phase transition is selected.
