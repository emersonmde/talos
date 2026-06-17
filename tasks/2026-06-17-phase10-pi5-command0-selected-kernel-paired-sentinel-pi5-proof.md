# Phase 10 Pi 5 Command0 Selected-Kernel Paired Sentinel Pi 5 Proof

Task id: phase10-pi5-command0-selected-kernel-paired-sentinel-pi5-proof-20260617

Status: accepted

Classification:
selected-kernel-paired-sentinel-pi5-proof-accepted

Evidence level: serialized Pi 5 hardware proof under hardwareTestLock,
lab-controller API, archive/static inspection, candidate/control post-publish
identity, fresh serial cursor capture, stable same-cursor TFTP deltas, final
pre-restore identity, restore proof, task-owned discriminator replay, JSON
evidence, docs build, and diff checks. No command0 write,
source-response-retention proof, generated-root command-input acceptance,
storage, networking, SSH, Phase 11/12 expansion, or phase transition was
performed.

## Goal

Run the serialized no-command-write paired selected-kernel/TFTP identity proof
authorized by the local/static paired sentinel discriminator.

## Result

The proof is accepted for the selected-kernel/TFTP publication boundary. The
candidate generated-root command-input archive
target/talos-rpi5-generated-root-command-input-proof-core-20260617.tar.gz was
published with selected tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212 and
208984-byte da591740/kernel_2712.img. The same-power-cycle TFTP delta served
two matching 208984-byte kernel fetches, final pre-restore identity stayed on
the selected tree, and restore returned to baseline tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The paired no-command-write control archive selected tree
9e3442962d40beff3b31668065df4f5d8ac37ee770fa3c4bba725d049bc78db3 with a
47832-byte da591740/kernel_2712.img. Its same-power-cycle TFTP delta served
two matching 47832-byte kernel fetches, final pre-restore identity stayed on
the selected tree, and restore returned to the same baseline.

A candidate rerun also retained selected tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212 with two
matching 208984-byte TFTP serves and final selected identity. Its broader
capture-chain identity join recorded serial-freshness-v1-not-proven for the
marker supplied to the generic capture helper, so the accepted paired-sentinel
classification relies on the task-owned selected-kernel/TFTP discriminator and
the accepted candidate/control pair.

## Findings

- fixed: acquired hardwareTestLock before lab mutation and retained release
  after restore evidence.
- fixed: candidate selected-kernel/TFTP publication boundary recovered after
  the prior baseline-served regression.
- fixed: control selected-kernel/TFTP publication boundary also retained
  selected identity and matching TFTP served bytes.
- fixed: task-owned discriminator accepted only the paired no-command-write
  candidate/control evidence with distinct selected identities.
- deferred: command0 write delivery remains a separate discriminator after
  selected-kernel recovery closeout.
- rejected: command0 write-delivery success, source-response retention,
  generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition.

## Evidence

- Task classification:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-paired-sentinel-pi5-proof/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-paired-sentinel-pi5-proof/evidence-map.json.
- Paired proof evidence:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-paired-sentinel-pi5-proof/paired-sentinel-proof-evidence.json.
- Discriminator output:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-paired-sentinel-pi5-proof/discriminator-output.json.
- Candidate run:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-paired-sentinel-pi5-proof/candidate-selected-kernel-paired-sentinel-20260617T193449Z/.
- Control run:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-paired-sentinel-pi5-proof/control-selected-kernel-paired-sentinel-20260617T193449Z/.
- Candidate rerun:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-paired-sentinel-pi5-proof/candidate-rerun-selected-kernel-paired-sentinel-20260617T193449Z/.
- Final post-restore identity:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-paired-sentinel-pi5-proof/final-post-restore-boot-files.json.

## Acceptance Check

- Accepted proof retains selected post-publish identity, stable
  same-power-cycle TFTP served bytes matching the selected kernel, final
  pre-restore selected identity, and restore proof: satisfied for candidate and
  control.
- Blocked proof records the precise first failing invariant: not applicable;
  no selected-kernel/TFTP failing invariant remained in the accepted pair.
- Inconclusive proof completes candidate identity, fresh serial cursor, TFTP
  delta, known-good/control, and candidate rerun triage: not applicable to
  acceptance because candidate/control evidence was decisive; candidate rerun
  evidence is retained.
- selected_next_task is
  phase10-pi5-command0-selected-kernel-recovery-closeout-20260617: satisfied.
- Rejected claims include command0 write-delivery success, source-response
  retention, generated-root command-input success, storage, networking, SSH,
  Phase 11/12 expansion, and phase transition: satisfied.

## Validation

- serialized Pi 5 hardware proof under hardwareTestLock: pass.
- candidate/control identity via lab API before power: pass.
- fresh serial cursor before power: pass for the accepted candidate/control
  capture bundle.
- stable same-cursor TFTP delta before restore: pass.
- final pre-restore boot identity: pass.
- post-run baseline restore proof: pass.
- task-owned proof validator/classifier: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Promote phase10-pi5-command0-selected-kernel-recovery-closeout-20260617 on the
next worker wake if dependencies remain satisfied and hardwareTestLock remains
unlocked/restored. Do not accept command0 write-delivery,
source-response retention, generated-root command-input success, storage,
networking, SSH, Phase 11/12 expansion, or phase transition from this proof.
