# Phase 10 Pi 5 Command0 Selected-Kernel Regression Reconciliation Source Checkpoint

Task id: phase10-pi5-command0-selected-kernel-regression-reconciliation-source-checkpoint-20260617

Status: accepted

Classification:
command0-selected-kernel-regression-reconciled-stability-discriminator-selected

Evidence level: static/source/task evidence inspection, task-owned JSON
evidence, docs build, and diff checks. No hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, command0 write,
source-response-retention proof, generated-root command-input acceptance,
storage, networking, SSH, Phase 11/12 expansion, or phase transition was
performed.

## Goal

Reconcile the selected-kernel/TFTP precondition regression before any further
command0 retry.

## Reconciliation

The invariant that must hold for command0 behavior evidence is:

1. post-publish boot files expose the selected tree and selected
   da591740/kernel_2712.img byte count;
2. the same-power-cycle TFTP delta serves that selected kernel byte count;
3. final pre-restore boot files still expose the same selected tree and kernel
   byte count;
4. restore returns to the saved baseline.

The accepted selected-kernel/TFTP precondition proof satisfied that invariant
for candidate tree
06eb7fd758522c4da95317cb6c9b6fb515612e22bbea3e291a4c2b3d13952212: post-publish
identity, same-power-cycle TFTP bytes, final pre-restore identity, and restore
all agreed for the selected 208984-byte kernel.

The command0 write-delivery v2 proof also satisfied the selected-kernel/TFTP
precondition and then blocked at command0 write delivery: /serial/write
accepted 9 bytes for rootinfo, but post-write direct reads retained no rootinfo,
command 0 line, dispatch command=0 status=handled, responses=1, or ready
command=1.

The post-write observe proof retained selected-kernel/TFTP agreement again, but
was inconclusive at serial capture freshness: /serial/observe from saturated
cursor 4194304 retained zero readiness or post-write bytes. Its non-gating
post-run peek showed the rootinfo write eventually processed as stale later
command=3 after command=1/2 timeouts, which does not satisfy command0 write
delivery.

The saturated-capture proof then regressed the selected-kernel/TFTP
precondition before command0 could be evaluated. It recorded post-publish boot
files exposing the selected tree and 208984-byte kernel, but the same-power-cycle
stable TFTP delta served two 104136-byte baseline kernel fetches and final
pre-restore boot files exposed the baseline tree. No rootinfo write was sent.

The first failing invariant from the saturated-capture proof is therefore
selected-kernel/TFTP/final identity regression, not command0 write delivery.
The earlier selected-kernel/TFTP proof remains valid only as a single-run
precondition proof; it no longer unblocks another command0 retry by itself.

## Helper And Classifier Review

- fixed: the accepted frontier is narrowed to selected-kernel stability across
  post-publish identity, same-power-cycle TFTP bytes, final pre-restore
  identity, and restore before any command0 retry.
- fixed: the earlier accepted selected-kernel/TFTP precondition is quarantined
  as a single-run proof, not a durable unblocker for later command0 retry
  planning.
- not-an-issue: the existing selected-kernel precondition helper rejected the
  saturated-capture evidence because same-power-cycle TFTP bytes and final
  identity did not match the selected 208984-byte tree.
- not-an-issue: the command0 write-delivery, post-write observe, and
  saturated-capture guards remain valid command0 transaction gates once the
  selected-kernel stability prerequisite is reproven.
- deferred: root-cause publication or lab-service diagnosis is deferred unless
  the selected-kernel stability discriminator records a precise post-publish,
  TFTP, final identity, cursor freshness, or restore blocker.
- rejected: command0 write-delivery success, source-response retention,
  generated-root command-input success, storage, networking, SSH, Phase 11/12
  expansion, and phase transition.

The next local/static task may change only the selected-kernel stability
classifier surface and task-owned fixtures needed to classify post-publish
identity, same-power-cycle TFTP bytes, final identity, restore, stale cursor,
and no-fresh-TFTP cases. The allowed implementation surface is:

- scripts/rpi5-generated-root-command-input-direct-read-proof-review.sh or a
  directly paired selected-kernel stability helper;
- task-owned fixtures under
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-stability-discriminator-core/;
- the selected-kernel stability core task record;
- docs/src/roadmap.md only if the accepted frontier or selected next task
  changes.

## Evidence

- Accepted selected-kernel/TFTP precondition proof:
  tasks/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-pi5-proof.md.
- Accepted selected-kernel/TFTP precondition proof classification:
  tasks/evidence/2026-06-17-phase10-pi5-command0-tftp-selected-kernel-precondition-pi5-proof/candidate-selected-kernel-tftp-precondition-20260617T121556Z/classification.json.
- Accepted write-delivery v2 proof:
  tasks/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof-v2-after-tftp-precondition.md.
- Accepted write-delivery v2 classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-write-delivery-pi5-proof-v2-after-tftp-precondition/candidate-command0-write-delivery-v2-20260617T124424Z/classification.json.
- Accepted post-write observe proof:
  tasks/2026-06-17-phase10-pi5-serial-command0-post-write-observe-pi5-proof.md.
- Accepted post-write observe classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-post-write-observe-pi5-proof/candidate-command0-post-write-observe-20260617T150944Z/classification.json.
- Accepted saturated-capture proof:
  tasks/2026-06-17-phase10-pi5-serial-command0-saturated-capture-pi5-proof.md.
- Accepted saturated-capture classification:
  tasks/evidence/2026-06-17-phase10-pi5-serial-command0-saturated-capture-pi5-proof/candidate-command0-saturated-capture-20260617T163347Z/classification.json.
- This task classification:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-regression-reconciliation-source-checkpoint/classification.json.
- This task evidence map:
  tasks/evidence/2026-06-17-phase10-pi5-command0-selected-kernel-regression-reconciliation-source-checkpoint/evidence-map.json.

## Acceptance Check

- Findings record each relevant prior proof/classifier with disposition:
  satisfied.
- Invariant across post-publish boot files, same-power-cycle TFTP bytes, final
  pre-restore identity, and restore is stated: satisfied.
- First failing invariant from saturated-capture proof is preserved as
  selected-kernel/TFTP/final identity regression, not command0 write delivery:
  satisfied.
- Stale helper/classifier assumption is quarantined: satisfied; earlier
  selected-kernel/TFTP precondition evidence is single-run only and no longer
  unblocks command0 retry by itself.
- selected_next_task is
  phase10-pi5-command0-selected-kernel-stability-discriminator-core-20260617:
  satisfied.
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

Promote
phase10-pi5-command0-selected-kernel-stability-discriminator-core-20260617 on
the next worker wake if dependencies remain satisfied. Do not run hardware,
retry command0, or select source-response retention directly from this
checkpoint.
