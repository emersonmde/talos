# Phase 11 RP1 DMA/Cache Small Diagnostic Visibility Closeout

Task: phase11-rp1-dma-cache-small-diagnostic-visibility-closeout-20260609

Status: accepted

Evidence level: static inspection, documentation reconciliation, JSON checks,
documentation build, and git diff checks.

## Goal

Close out the local/static small diagnostic visibility-report frontier before
any serialized Pi 5 proof.

## Scope

- Reconcile the accepted hardware-proof contract, visibility-core
  implementation, tests, evidence JSON, project/roadmap wording, and retained
  risks.
- Record findings with disposition.
- State exactly which local/static visibility/control claims are accepted and
  which hardware/live-DMA claims remain rejected.
- Decide whether the serialized Pi 5 plan visibility/control proof is
  mechanically objective from accepted evidence.
- Update roadmap/project docs only for accepted frontier or checkpoint changes.

## Non-Goals

No runtime source changes, Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, RP1 MMIO writes, DMA channel programming,
descriptor-ring construction, transfer completion, interrupt completion,
Ethernet/storage driver work, networking, SSH, Milestone 12 work, hardware
validation claim, Milestone 11.3 completion, or phase transition.

## Reconciled Inputs

- tasks/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-hardware-proof-contract.md
- tasks/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-visibility-core.md
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-hardware-proof-contract/classification.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-visibility-core/classification.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-visibility-core/evidence-map.json
- src/dma_cache.rs
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md

## Closeout Decision

The accepted visibility-core implementation satisfies
phase11-rp1-dma-cache-small-diagnostic-visibility-report-contract-v1 as a
local/static report surface. The candidate report constructs evidence only
from accepted DmaCacheSmallDiagnosticPlanEvidence. The paired control report
uses the same report contract/source identity, withholds accepted plan fields,
and carries the explicit no-plan control classification.

The accepted candidate report preserves the small diagnostic plan contract id,
source contract id, driver diagnostic envelope contract id,
descriptor/sync-plan/maintenance-sequence/executor identities, RP1 DMA
compatible string, RP1 bus base, translated CPU physical base, channel count,
target count, interrupt and clock names, CPU/RP1 buffer addresses, descriptor
length, cache line coverage, direction, cacheability, owner transition, IOMMU
classification, rejected runtime claims, unresolved diagnostic gaps, rejected
hardware claims, retained risks, and the hardware-proof boundary
classification.

The accepted validators reject invalid report shapes, non-accepted plan
evidence, missing prerequisite identities, missing rejected runtime claims,
missing unresolved gaps, unsupported cacheability or IOMMU claims, RP1 MMIO
writes, RP1 channel ownership, DMA channel programming, descriptor-ring
readiness, transfer completion, interrupt completion, hardware/device
completion, Ethernet/storage readiness, networking, SSH, Milestone 11.3
completion, and phase transition.

Same-shaped local/static visibility report retries are closed unless future
scope supplies materially different runtime evidence, hardware evidence, source
evidence, or acceptance criteria. Repeating the same local/static candidate or
control report construction would not add progress.

The next mechanically objective task is the serialized Pi 5 visibility/control
proof already queued as
phase11-rp1-dma-cache-small-diagnostic-visibility-pi5-20260609. That task is
selected only as a guarded plan visibility/control proof. It must acquire
hardwareTestLock before staging, capture candidate/control identity, artifact
digest, fresh serial cursor/output, TFTP delta, restore evidence, and
classification JSON, and it must keep all live-DMA claims rejected.

## Findings

- fixed: reconciled the accepted hardware-proof contract with the accepted
  visibility-core implementation and evidence.
- fixed: documented the accepted frontier as local/static candidate report and
  paired no-plan control report construction only.
- fixed: confirmed the candidate report preserves the required plan identity,
  RP1 controller, buffer/cache, rejected-claim, unresolved-gap, and
  hardware-proof boundary fields.
- fixed: confirmed the paired control preserves the reporting path while
  withholding accepted plan evidence.
- fixed: closed same-shaped local/static visibility report retries unless
  future scope supplies materially different evidence or acceptance criteria.
- fixed: selected the guarded serialized Pi 5 plan visibility/control proof as
  the single next mechanically objective task.
- deferred: Pi 5 serial visibility/control proof execution, hardwareTestLock
  serialization, boot archive publication, RP1 DMA channel ownership,
  descriptor-ring layout and ownership, transfer completion, interrupt
  completion, runtime IOMMU policy, DMA-safe allocation or pinning expansion,
  device-consumer selection, live hardware DMA proof, Ethernet, storage,
  networking, SSH, Milestone 12 work, Milestone 11.3 completion, and phase
  transition.
- not-an-issue: no runtime source changes or Pi 5 hardware run were required
  because this task is the checkpoint before the guarded hardware proof.

No findings were removed.

## Rejected Claims

This checkpoint does not accept:

- Pi 5 hardware validation.
- Boot archive publication or hardwareTestLock acquisition by this task.
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

- static inspection: reviewed the accepted hardware-proof contract,
  visibility-core task record, visibility-core classification/evidence JSON,
  src/dma_cache.rs, roadmap, and project contract doc.
- jq JSON checks: task-owned classification.json and evidence-map.json passed
  jq empty.
- docs: /home/node/.cargo/bin/mdbook build passed after docs updates.
- diff checks: git diff --check and git diff --cached --check passed.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles accepted visibility-core evidence against the
  hardware-proof contract without expanding acceptance to live DMA, hardware
  validation, descriptor rings, interrupt completion, networking, SSH,
  Milestone 11.3 completion, or phase transition: satisfied.
- Checkpoint states whether same-shaped local/static visibility-report retries
  are closed unless future scope supplies different runtime/hardware evidence
  or acceptance criteria: satisfied.
- Checkpoint nextAction explicitly selects or rejects the serialized Pi 5
  visibility/control proof task: satisfied.
- Accepted checkpoint is committed before any Pi 5 visibility proof starts:
  satisfied by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase11-rp1-dma-cache-small-diagnostic-visibility-pi5-20260609 on the next
worker wake only as a serialized plan visibility/control proof. It must acquire
hardwareTestLock before staging, capture candidate/control identity, artifact
digest, fresh serial cursor/output, TFTP delta, restore evidence, and
classification JSON, and it must not program RP1 MMIO or DMA channels, create
descriptor rings, claim transfer or interrupt completion, implement Ethernet,
storage, networking, SSH, accept Milestone 11.3 completion, or create a phase
transition.
