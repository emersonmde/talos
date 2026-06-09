# Phase 11 RP1 DMA Cache Small Diagnostic Visibility V3 Closeout

Task: phase11-rp1-dma-cache-small-diagnostic-visibility-v3-closeout-20260609
Status: accepted
Classification: rp1-dma-cache-small-diagnostic-visibility-v3-frontier-closed

## Goal

Close out the run-unique Pi 5 small diagnostic visibility/control proof
without expanding acceptance to live DMA, descriptor rings, channel ownership,
Milestone 11.3 completion by implication, or a phase transition.

## Scope

- Reconcile the accepted v3 retry task record, evidence map, classification,
  hardware summary, capture summaries, project contract doc, and roadmap.
- Preserve the accepted candidate/control visibility evidence and its exact
  runtime/hardware limits.
- State whether same-shaped visibility/control hardware retries are closed.
- Select exactly one queued conditional follow-up task by id.

## Non-Goals

No runtime source changes, no additional Pi 5 hardware run, no boot archive
publication, no hardwareTestLock acquisition, no RP1 MMIO writes, no DMA
channel programming, no channel ownership, no descriptor-ring construction, no
transfer completion, no interrupt completion, no Ethernet/storage work, no
networking, no SSH, no Milestone 12 work, no Milestone 11.3 completion by
implication, and no phase transition.

## Reconciled Inputs

- tasks/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-visibility-v3-retry.md
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-visibility-v3-retry/classification.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-visibility-v3-retry/evidence-map.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-visibility-v3-retry/hardware-run-summary.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-visibility-v3-retry/control-run/capture-invariant-summary.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-visibility-v3-retry/candidate-run/capture-invariant-summary.json
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md

## Closeout Decision

The accepted v3 retry closes the small diagnostic visibility/control hardware
proof frontier. The no-plan control and candidate both passed the repaired
run-unique freshness discriminator, selected-tree identity, expected TFTP
fetch, boot-staging identity replay, final identity, restore, and evidence-map
gates.

Accepted control evidence:

- Staged tree:
  3813289a7df48f04313329b90073683fb07eb0188b719290a862af587f86739b
- kernel_2712.img bytes: 48,704
- kernel_2712.img SHA-256:
  5bad74366a4e510940e25424beda0dedfce5598574dab8f1bb4590d2e51489d1
- Capture nonce: dma-vis-v3-control-20260609T211037Z-8d1ce237
- Run-unique replay: nonce absent before power and present 29 times after
  power.
- Classification: no-plan control report path visible with accepted plan
  fields withheld.

Accepted candidate evidence:

- Staged tree:
  f0b229ab1da582050f68af75b1de9953e9010b2cc4443ff3ee0002789e7572b2
- kernel_2712.img bytes: 49,968
- kernel_2712.img SHA-256:
  d46844db2747fa9457844acf66b473f63d3dca345bb7e306bd284f82ba49d60a
- Capture nonce: dma-vis-v3-candidate-20260609T211037Z-8d1ce237
- Run-unique replay: nonce absent before power and present 20 times after
  power.
- Classification: accepted local/static RP1 DMA small diagnostic plan report
  fields visible, including plan/source/envelope identity, controller facts,
  buffer/cache fields, rejected runtime/hardware claims, and retained risks.

The lab restored to the baseline tree before this closeout:

    a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10

Same-shaped visibility/control hardware retries are closed. Repeating the same
candidate/control visibility proof would not create new durable evidence unless
future supervisor scope supplies materially different source evidence, runtime
evidence, hardware evidence, freshness requirements, or acceptance criteria.

The next mechanically objective queued task is
phase11-rp1-dma-cache-milestone-11-3-closeout-20260609. It must be a checkpoint
over accepted Milestone 11.3 evidence and retained risks only. It must not
start channel ownership, descriptor rings, live DMA, RP1 MMIO/DMA programming,
transfer completion, interrupt completion, Ethernet/storage, networking, SSH,
Milestone 12 work, or a phase transition.

## Findings

- fixed: reconciled the accepted v3 retry hardware proof with project and
  roadmap frontier wording.
- fixed: documented the no-plan control as current-run serial-visible under
  the run-unique nonce discriminator while withholding accepted plan evidence.
- fixed: documented the candidate as current-run serial-visible under a
  distinct run-unique nonce while showing the accepted local/static plan report
  fields.
- fixed: closed same-shaped visibility/control hardware retries unless future
  supervisor scope changes evidence or acceptance criteria.
- fixed: selected
  phase11-rp1-dma-cache-milestone-11-3-closeout-20260609 as the single next
  queued conditional follow-up.
- deferred: live DMA, RP1 MMIO/DMA programming, channel ownership, descriptor
  rings, transfer completion, interrupt completion, Ethernet/storage,
  networking, SSH, Milestone 12 work, and phase transition remain separate
  work.
- not-an-issue: no additional hardware run was required because this task is a
  static checkpoint over committed Pi 5 evidence.

No findings were removed.

## Rejected Claims

This checkpoint does not accept:

- live DMA
- RP1 MMIO writes
- RP1 DMA channel ownership
- DMA channel programming
- descriptor-ring construction or ownership
- transfer completion
- interrupt completion
- hardware/device completion
- Ethernet readiness
- storage readiness
- networking
- SSH
- Milestone 11.3 completion by implication
- phase transition

## Validation

- static inspection: v3 retry task record, evidence map, classification JSON,
  hardware summary, capture summaries, project contract doc, and roadmap.
- JSON checks: jq empty on task-owned evidence-map/classification JSON.
- diff checks: git diff --check and git diff --cached --check.
- documentation build: /home/node/.cargo/bin/mdbook build.

## Acceptance

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles v3 retry evidence without expanding acceptance to live
  DMA, RP1 MMIO/DMA programming, descriptor rings, transfer completion,
  interrupt completion, Ethernet/storage, networking, SSH, Milestone 11.3
  completion by implication, or phase transition: satisfied.
- Checkpoint states whether same-shaped visibility/control hardware retries
  are closed: satisfied; closed unless future supervisor scope changes
  evidence or acceptance criteria.
- Checkpoint nextAction selects exactly one conditional queued follow-up by id:
  satisfied.
- Accepted checkpoint is committed before any follow-up starts: satisfied by
  the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase11-rp1-dma-cache-milestone-11-3-closeout-20260609 on the next worker wake.
Do not promote
phase11-rp1-dma-cache-channel-ownership-source-contract-20260609 from this
closeout, and do not start live DMA, descriptor-ring/channel-ownership,
Milestone 12 work, or a phase transition.
