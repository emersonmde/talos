# Phase 11 RP1 DMA/Cache Small Diagnostic Plan Closeout

Task: phase11-rp1-dma-cache-small-diagnostic-plan-closeout-20260609

Status: accepted

Evidence level: static inspection, documentation reconciliation, JSON checks,
documentation build, and git diff checks.

## Goal

Close out the local/static RP1 DMA small diagnostic plan frontier and decide
the next bounded boundary without accepting hardware or live DMA by
implication.

## Scope

- Reconcile accepted small diagnostic source contract, accepted plan-core
  implementation/evidence, retained RP1 DMA source facts, and unresolved gaps.
- Record findings with disposition.
- State whether the local/static plan core satisfies the accepted plan-contract
  boundary and which same-shaped local/static retries are closed.
- Select the next action only if it is a bounded source-contract or checkpoint;
  keep hardware proof, DMA programming, descriptor rings, and consumer driver
  work explicit and separate.
- Update roadmap/project docs only for accepted frontier and checkpoint
  changes.

## Non-Goals

No runtime source changes unless required to correct task-owned documentation or
evidence, no Pi 5 hardware run, boot archive publication, hardwareTestLock
acquisition, RP1 MMIO writes, DMA channel programming, descriptor-ring
construction, transfer completion, interrupt completion, Ethernet/storage
driver work, networking, SSH, Milestone 12 work, hardware validation claim,
Milestone 11.3 completion, or phase transition.

## Reconciled Inputs

- tasks/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-source-contract.md
- tasks/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-plan-core.md
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-source-contract/classification.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-plan-core/classification.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-plan-core/evidence-map.json
- src/dma_cache.rs
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md

## Closeout Decision

The accepted local/static plan core satisfies
phase11-rp1-dma-cache-small-diagnostic-plan-contract-v1. The accepted frontier
is limited to constructing and formatting evidence for one source-backed RP1
AXI DMA diagnostic plan from accepted DmaCacheDriverDiagnosticEnvelopeEvidence
and retained rp1_dma source facts.

The accepted plan evidence preserves the source contract id, envelope contract
id, descriptor/sync-plan/maintenance-sequence/executor identities, RP1 DMA
compatible string, RP1 bus base, translated CPU physical base, channel count,
target count, interrupt and clock names, CPU/RP1 buffer addresses, descriptor
length, cache line coverage, direction, cacheability, owner transition, IOMMU
classification, rejected runtime claims, unresolved diagnostic gaps, and
local/static classification.

The accepted validators reject invalid or overclaiming inputs, including
non-accepted envelope evidence, missing prerequisite identities, missing
rejected completion claims, missing unresolved gaps, unsupported cacheability
or IOMMU claims, zero channel count, invalid translated controller base, RP1
channel ownership, descriptor-ring readiness, transfer completion, interrupt
completion, hardware/device completion, Ethernet/storage readiness,
networking, and SSH.

Same-shaped local/static small diagnostic plan retries are closed. A future
retry is not progress unless supervisor scope supplies materially different
source evidence, runtime evidence, hardware evidence, or acceptance criteria.

The next bounded boundary is
phase11-rp1-dma-cache-small-diagnostic-hardware-proof-contract-20260609. That
task is contract-only: it may decide whether a future serialized Pi 5 proof is
allowed, blocked, or limited to plan visibility/control output. It does not
authorize a Pi 5 run, live DMA, RP1 MMIO writes, DMA channel programming,
descriptor rings, transfer completion, interrupt completion, Ethernet,
storage, networking, SSH, Milestone 11.3 completion, or a phase transition.

## Findings

- fixed: reconciled the accepted source contract and plan-core implementation
  as satisfying phase11-rp1-dma-cache-small-diagnostic-plan-contract-v1.
- fixed: documented the accepted frontier as local/static plan evidence only,
  not a live DMA diagnostic.
- fixed: closed same-shaped local/static plan retries unless future scope
  supplies materially different evidence or acceptance criteria.
- fixed: selected the guarded hardware-proof source-contract boundary as the
  next mechanically objective checkpoint, while keeping any hardware run
  unauthorised by this closeout.
- deferred: RP1 DMA channel ownership, descriptor-ring layout and ownership,
  transfer completion, interrupt completion, runtime IOMMU policy, DMA-safe
  allocation or pinning expansion, device-consumer selection, serialized Pi 5
  proof, Ethernet, storage, networking, SSH, Milestone 12 work, Milestone 11.3
  completion, and phase transition.
- not-an-issue: no runtime source changes or Pi 5 hardware run were required
  because this task accepts only static closeout evidence and documentation.

No findings were removed.

## Rejected Claims

This checkpoint does not accept:

- Pi 5 hardware validation.
- Boot archive publication or hardwareTestLock acquisition.
- RP1 MMIO writes or DMA channel programming.
- RP1 DMA channel ownership.
- DMA descriptor rings or descriptor-ring ownership.
- Transfer completion or interrupt completion.
- Hardware/device completion.
- Ethernet readiness, storage readiness, networking, SSH, or Milestone 12
  progress.
- Cache-coherent, non-cacheable, or IOMMU-backed runtime policy.
- DMA-safe allocation or pinning beyond accepted descriptor validation.
- Milestone 11.3 completion by implication.
- Phase transition.

## Validation

- static inspection: reviewed the accepted source-contract and plan-core task
  records, classification JSON, evidence map, src/dma_cache.rs, roadmap, and
  project contract doc.
- jq JSON checks: task-owned classification.json and evidence-map.json passed
  jq empty.
- docs: /home/node/.cargo/bin/mdbook build passed after docs updates.
- diff checks: git diff --check and git diff --cached --check passed.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles accepted source-contract and plan-core evidence without
  shrinking or expanding accepted claims: satisfied.
- Checkpoint explicitly rejects hardware validation, live DMA, channel
  ownership, descriptor rings, transfer completion, interrupt completion,
  Ethernet/storage readiness, networking, SSH, Milestone 11.3 completion, and
  phase transition by implication: satisfied.
- Same-shaped retry policy is explicit for local/static small diagnostic plan
  evidence: satisfied.
- Next action is either a guarded source-contract boundary selected by this
  checkpoint or a precise blocker requiring supervisor/human planning:
  satisfied.
- Accepted checkpoint is committed before any follow-up task starts: satisfied
  by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase11-rp1-dma-cache-small-diagnostic-hardware-proof-contract-20260609 on the
next worker wake only as a contract-only boundary. It must not run hardware,
publish boot archives, acquire hardwareTestLock, program RP1 MMIO or DMA
channels, create descriptor rings, claim transfer or interrupt completion,
implement Ethernet, storage, networking, SSH, accept Milestone 11.3 completion,
or create a phase transition.
