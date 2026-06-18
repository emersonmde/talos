# Phase 10 Pi 5 Serial Command0 Lab Boundary After Precondition Recovery Pi 5 Proof

Task id: phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-pi5-proof-20260617

Status: accepted

Classification:
command0-lab-boundary-after-precondition-final-identity-regressed

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock,
lab-controller API, selected-kernel/TFTP precondition evidence, prearmed live
/serial/read during /serial/write, task-owned JSON evidence, restore proof, and
diff checks.

## Goal

Retry the command0 lab write-boundary proof only after selected-kernel/TFTP
precondition recovery was accepted, using the accepted prearmed live-read
contract around POST /serial/write rootinfo.

## Result

The generated-root command-input candidate archive
target/talos-rpi5-generated-root-command-input-proof-core-20260617.tar.gz was
published with archive SHA-256
8f6a4d0d3436308a5f3b51e4f113b75d1acbf807a7b7af2d66453806c586cf0c. Its
da591740/kernel_2712.img SHA-256 was
c37b34ebc955a7ce11cd0660bc18424ebcec4550de23ae1e6b0b9ad7c867e4bd and its
byte count was 208984.

The rerun retained selected-kernel/TFTP precondition evidence before evaluating
command0 delivery: post-publish status reported selected tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212, effective
kernel kernel_2712.img, and da591740/kernel_2712.img at 208984 bytes. The
stable same-power-cycle TFTP delta retained two 208984-byte
da591740/kernel_2712.img serves.

The serial side retained the expected command0 delivery evidence. The boot
reached source=firmware-initramfs, reason=valid-artifact, ready command=0, and
the talos> prompt. The immediate pre-write read was empty. POST /serial/write
accepted rootinfo with 9 bytes, and the prearmed POST /serial/read retained
rootinfo, line command=0, dispatch command=0 status=handled, responses=1, and
ready command=1 in order.

The proof does not accept command0 input delivery because the required final
pre-restore identity gate failed: before the explicit restore call,
lab-controller status reported the baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 and a
104136-byte da591740/kernel_2712.img. The explicit restore call then confirmed
the baseline tree. The first failing invariant is
final-pre-restore-identity-regressed-after-command0.

An earlier same-wake attempt is retained as incomplete evidence only: archive
member extraction omitted the ./ prefix, wrote zero-byte local kernel/initramfs
facts, and skipped command0 evaluation despite selected TFTP evidence. The
rerun above supersedes that attempt.

## Findings

- fixed: retained selected-kernel/TFTP precondition evidence before command0
  evaluation.
- fixed: used the accepted prearmed live-read discriminator instead of a
  same-shaped saturated observe retry.
- fixed: rootinfo reached command0 and produced ordered dispatch/response/ready
  output under the prearmed-read contract.
- blocked: final pre-restore lab identity regressed to the baseline tree before
  explicit restore, so command0 input delivery remains unaccepted.
- removed: the incomplete first attempt is excluded from acceptance because its
  local archive-member extraction bug skipped command0 evaluation.
- rejected: source-response retention, generated-root command-input success,
  storage, networking, SSH, Phase 11/12 expansion, and phase transition.

## Evidence

- Accepted precondition closeout:
  tasks/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-closeout.md.
- Accepted lab-boundary discriminator core:
  tasks/2026-06-17-phase10-pi5-serial-command0-lab-write-boundary-discriminator-core.md.
- Task classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-pi5-proof/candidate-rerun-command0-after-precondition-prearmed-read-20260618T002703Z/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-pi5-proof/candidate-rerun-command0-after-precondition-prearmed-read-20260618T002703Z/evidence-map.json.
- Run evidence:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-pi5-proof/candidate-rerun-command0-after-precondition-prearmed-read-20260618T002703Z/.
- Selected-kernel/TFTP precondition evidence:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-pi5-proof/candidate-rerun-command0-after-precondition-prearmed-read-20260618T002703Z/tftp/tftp-delta-stable-pre-command.json.
- Pre-write freshness read:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-pi5-proof/candidate-rerun-command0-after-precondition-prearmed-read-20260618T002703Z/serial/command0-pre-write-read.json.
- /serial/write evidence:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-pi5-proof/candidate-rerun-command0-after-precondition-prearmed-read-20260618T002703Z/serial/command0-write.json.
- Prearmed live-read evidence:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-pi5-proof/candidate-rerun-command0-after-precondition-prearmed-read-20260618T002703Z/serial/command0-prearmed-read.json.
- Final pre-restore identity:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-pi5-proof/candidate-rerun-command0-after-precondition-prearmed-read-20260618T002703Z/final-pre-restore-status.json.
- Restore proof:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-pi5-proof/candidate-rerun-command0-after-precondition-prearmed-read-20260618T002703Z/restore/post-restore-status.json.

## Acceptance Check

- The proof records selected-kernel/TFTP precondition pass before evaluating
  command0 input delivery: satisfied.
- Accepted command0 input delivery requires /serial/write success plus ordered
  prearmed serial evidence that rootinfo reached command0 and produced the
  expected command0 effect under the selected candidate: not satisfied because
  final pre-restore identity regressed to baseline before explicit restore.
- Blocked proof records the precise first failing invariant without accepting
  source-response retention or generated-root command-input success: satisfied
  with final-pre-restore-identity-regressed-after-command0.
- Inconclusive proof triage before code changes: not applicable; the rerun is
  classified blocked, not inconclusive.
- selected_next_task is the after-precondition recovery closeout task, or null
  with planningNeeded=true: satisfied with the closeout selected.

## Validation

- serialized Pi 5 hardware proof under hardwareTestLock: blocked at final
  pre-restore identity.
- selected-kernel/TFTP precondition evidence before command0 evaluation: pass.
- fresh serial cursor and pre-write freshness read: pass.
- /serial/write result evidence: pass; rootinfo accepted with 9 bytes.
- prearmed live-read evidence: pass; command0 rootinfo/dispatch/response/ready
  retained in order.
- final pre-restore boot identity: failed; baseline tree was observed before
  explicit restore.
- post-run baseline restore proof before releasing hardwareTestLock: pass.
- task-owned proof validator/classifier: pass for blocked classification.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote
phase10-pi5-serial-command0-lab-boundary-after-precondition-recovery-closeout-20260617
on the next worker wake if dependencies remain satisfied. The closeout must
reconcile that selected-kernel/TFTP and ordered command0 serial delivery passed
before the final pre-restore identity gate regressed, and must not accept
source-response retention, generated-root command-input success, storage,
networking, SSH, Phase 11/12 expansion, or a phase transition from this proof.
