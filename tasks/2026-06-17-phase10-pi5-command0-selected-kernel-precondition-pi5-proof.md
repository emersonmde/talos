# Phase 10 Pi 5 Command0 Selected-Kernel Precondition Pi 5 Proof

Task id: phase10-pi5-command0-selected-kernel-precondition-pi5-proof-20260617

Status: accepted

Classification:
selected-kernel-precondition-pi5-proof-accepted

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock,
lab-controller API, fresh serial cursor, stable same-power-cycle TFTP delta,
final pre-restore identity, restore proof, task-owned discriminator/classifier,
task-owned JSON evidence, docs build, and diff checks. No command0 write,
source-response-retention proof, generated-root command-input acceptance,
storage, networking, SSH, Phase 11/12 expansion, or phase transition was
performed.

## Goal

Run the serialized no-command-write Pi 5 proof for the accepted
selected-kernel/TFTP precondition discriminator before another command0
lab-boundary retry is eligible.

## Result

The selected-kernel/TFTP precondition is accepted for the generated-root
command-input candidate. The proof published
target/talos-rpi5-generated-root-command-input-proof-core-20260617.tar.gz with
archive SHA-256
8f6a4d0d3436308a5f3b51e4f113b75d1acbf807a7b7af2d66453806c586cf0c. Its
kernel_2712.img SHA-256 was
c37b34ebc955a7ce11cd0660bc18424ebcec4550de23ae1e6b0b9ad7c867e4bd and its
expected byte count was 208984.

Before power-cycle, /boot/files reported selected tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212, effective
kernel kernel_2712.img, and da591740/kernel_2712.img at 208984 bytes. The
same-power-cycle TFTP delta was stable and retained two selected-kernel fetches,
both at 208984 bytes:

- Jun 17 23:56:13 dnsmasq-tftp[1]: sent
  /var/tftpboot/da591740/kernel_2712.img to 10.42.1.4.
- Jun 17 23:56:14 dnsmasq-tftp[1]: sent
  /var/tftpboot/da591740/kernel_2712.img to 10.42.1.4.

Final pre-restore identity still reported the selected tree and expected
208984-byte fetch. Restore returned the lab to baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with the
104136-byte baseline kernel.

The task-owned discriminator accepted the retained evidence under
selected-kernel-tftp-precondition-lab-boundary-v1. This proof accepts only the
selected-kernel/TFTP precondition; it does not accept command0 input delivery,
source-response retention, generated-root command-input success, storage,
networking, SSH, Phase 11/12 expansion, or phase transition.

## Findings

- fixed: acquired hardwareTestLock before lab mutation and retained restore
  evidence before release.
- fixed: selected post-publish identity exposed the 208984-byte generated-root
  command-input candidate kernel.
- fixed: same-power-cycle TFTP delta retained two
  da591740/kernel_2712.img serves, both matching the selected 208984-byte
  kernel.
- fixed: final pre-restore identity stayed selected and restore returned to
  baseline.
- fixed: task-owned discriminator accepted the retained evidence under the
  selected precondition contract.
- rejected: command0 input delivery acceptance, source-response retention,
  generated-root command-input success, storage, networking, SSH, Phase 11/12
  expansion, and phase transition.

## Evidence

- Task classification:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-pi5-proof/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-pi5-proof/evidence-map.json.
- Accepted run:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-pi5-proof/candidate-selected-kernel-precondition-20260617T235548Z/.
- Run summary:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-pi5-proof/candidate-selected-kernel-precondition-20260617T235548Z/run-summary.json.
- Discriminator output:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-pi5-proof/candidate-selected-kernel-precondition-20260617T235548Z/discriminator-output.json.
- Stable TFTP delta:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-precondition-pi5-proof/candidate-selected-kernel-precondition-20260617T235548Z/capture/tftp-delta-stable-pre-restore.json.

## Acceptance Check

- Accepted proof retains selected post-publish identity, fresh
  same-power-cycle TFTP served bytes matching selected kernel, final
  pre-restore selected identity, and restore proof under hardwareTestLock:
  satisfied.
- Blocked proof records the precise first failing invariant without shrinking
  acceptance toward command0 serial evidence: not applicable; no selected
  precondition invariant failed.
- Inconclusive proof completes candidate identity, fresh serial cursor, TFTP
  delta, known-good/control, and candidate rerun triage before code changes:
  not applicable because the retained identity, TFTP, final identity, and
  restore evidence were decisive.
- selected_next_task is
  phase10-pi5-command0-selected-kernel-precondition-closeout-20260617:
  satisfied.
- Rejected claims include command0 input delivery acceptance, source-response
  retention, generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition: satisfied.

## Validation

- serialized Pi 5 hardware proof under hardwareTestLock: pass.
- candidate identity via lab API before power: pass.
- fresh serial cursor before power: pass.
- TFTP delta tied to selected candidate: pass; two 208984-byte selected-kernel
  fetches.
- final pre-restore boot identity: pass.
- post-run baseline restore proof before releasing hardwareTestLock: pass.
- task-owned proof validator/classifier: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote
phase10-pi5-command0-selected-kernel-precondition-closeout-20260617 on the next
worker wake if dependencies remain satisfied, the repository remains clean,
hardwareTestLock is unlocked/restored, and supervisorIntervention is inactive.
Do not retry command0 input delivery directly from this proof.
