# Phase 11 RP1 DMA/Cache Small Diagnostic Plan Core

Task: phase11-rp1-dma-cache-small-diagnostic-plan-core-20260609

Status: accepted

Evidence level: static inspection, local implementation, fmt/lint/typecheck,
unit tests, JSON checks, and git diff checks.

## Goal

Implement the bounded local/static RP1 DMA small diagnostic plan selected by
phase11-rp1-dma-cache-small-diagnostic-plan-contract-v1.

## Scope

- Consume accepted DmaCacheDriverDiagnosticEnvelopeEvidence plus retained RP1
  DMA controller source facts from the accepted source contract.
- Add local/static plan evidence that preserves the contract id, envelope
  contract id, descriptor/sync-plan/maintenance-sequence/executor identities,
  translated RP1 DMA controller base, compatible string, channel count, target
  count, source interrupt and clock names, CPU/RP1 buffer addresses, descriptor
  length, cache line coverage, direction, cacheability, owner transition,
  IOMMU classification, unresolved diagnostic gaps, and local/static
  classification.
- Add deterministic validators and rejection evidence for unsupported envelope
  classification, missing prerequisite identities, missing rejected completion
  claims, missing unresolved gaps, unsupported cacheability/IOMMU claims, zero
  channel count, invalid translated controller base, and runtime/device
  overclaims.
- Keep implementation local/static: source-backed plan construction and tests
  only; no RP1 MMIO or DMA programming.
- Record findings with disposition.

## Non-Goals

No Pi 5 hardware run, boot archive publication, hardwareTestLock acquisition,
RP1 MMIO writes, DMA channel programming, descriptor-ring construction,
descriptor-ring ownership, transfer completion, interrupt completion, live
cache-maintenance proof for driver buffers beyond accepted executor boundaries,
Ethernet/storage driver work, networking, SSH, Milestone 12 work, hardware
validation claim, Milestone 11.3 completion, or phase transition.

## Implementation

- Added DMA_CACHE_SMALL_DIAGNOSTIC_PLAN_CONTRACT_ID and
  DMA_SMALL_DIAGNOSTIC_PLAN_LOCAL_STATIC_CLASSIFICATION.
- Added retained RP1 DMA source constants for compatible string
  snps,axi-dma-1.01a, RP1 bus base 0xc0_4018_8000, translated CPU physical
  base 0x1f_0018_8000, 8 channels, 64 targets, RP1_INT_DMA, and RP1_CLK_DMA /
  RP1_CLK_SYS.
- Added Rp1DmaControllerSourceFacts,
  DmaCacheSmallDiagnosticPlanInput, DmaCacheSmallDiagnosticPlan,
  DmaCacheSmallDiagnosticPlanEvidence, and DmaCacheSmallDiagnosticPlanError.
- Added build_dma_cache_small_diagnostic_plan,
  dma_cache_small_diagnostic_plan_evidence, and rejected evidence formatting.
- Validators require accepted envelope classification and prerequisite
  identities, fixed rejected completion/runtime claim vocabulary, fixed
  unresolved gaps, cacheable-maintained/source-unassigned IOMMU classification,
  nonzero channel count, and the accepted RP1 peripheral window translation for
  the DMA controller source facts.
- Validators reject RP1 channel ownership, descriptor-ring readiness, transfer
  completion, interrupt completion, hardware/device completion, Ethernet
  readiness, storage readiness, networking, and SSH claims.
- Added focused tests for accepted construction and deterministic rejection
  cases.

## Findings

- fixed: implemented the selected small diagnostic plan core in src/dma_cache.rs
  as a local/static evidence boundary.
- fixed: required the plan to consume accepted
  DmaCacheDriverDiagnosticEnvelopeEvidence and retained RP1 DMA controller
  source facts instead of raw driver addresses or an arbitrary channel choice.
- fixed: preserved prerequisite descriptor, sync-plan, maintenance-sequence,
  executor, and envelope identities in plan evidence.
- fixed: recorded source-backed RP1 DMA controller identity, translated base,
  channel count, target count, interrupt name, clock names, buffer addresses,
  descriptor length, cache line coverage, direction, cacheability, owner
  transition, IOMMU classification, unresolved diagnostic gaps, and
  local/static classification.
- fixed: validators reject invalid envelope/source inputs, unsupported
  cacheability/IOMMU claims, missing rejected completion claims, missing
  unresolved gaps, zero channel count, invalid controller base translation, and
  all premature runtime/device readiness claims.
- deferred: RP1 DMA channel ownership, descriptor-ring layout and ownership,
  transfer completion, interrupt completion, runtime IOMMU policy, DMA-safe
  allocation or pinning expansion, hardware proof, a device-specific consumer,
  Ethernet, storage, networking, SSH, Milestone 12 work, Milestone 11.3
  completion, and phase transition.
- not-an-issue: no Pi 5 hardware run was required because this task accepts
  only local/static source and unit-test evidence.

No findings were removed.

## Rejected Claims

This task does not accept:

- RP1 DMA channel ownership or programming.
- DMA descriptor rings or descriptor-ring ownership.
- Transfer completion or interrupt completion.
- Hardware or device completion.
- Ethernet readiness, storage readiness, networking, SSH, or Milestone 12
  progress.
- Cache-coherent, non-cacheable, or IOMMU-backed runtime policy.
- DMA-safe allocation or pinning beyond accepted descriptor validation.
- Pi 5 hardware validation.
- Milestone 11.3 completion by implication.
- Phase transition.

## Validation

- static inspection: reviewed the accepted small diagnostic source contract,
  accepted driver diagnostic envelope evidence, retained RP1 DMA source facts,
  and src/dma_cache.rs.
- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet dma_cache passed; the
  custom runner executed the full no_std test set, 452 passed.
- jq JSON checks: task-owned classification.json and evidence-map.json passed
  jq empty.
- diff checks: git diff --check and git diff --cached --check passed.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Implementation constructs local/static small diagnostic plan evidence only
  from accepted envelope evidence and retained RP1 DMA source facts: satisfied.
- Accepted plan evidence records all required contract, prerequisite identity,
  RP1 controller, buffer/cache, rejected-claim, unresolved-gap, and
  local/static classification fields: satisfied.
- Validators reject invalid or overclaiming inputs, including RP1 channel
  ownership, descriptor-ring readiness, transfer completion, interrupt
  completion, hardware/device completion, Ethernet/storage readiness,
  networking, SSH, unsupported cacheability/IOMMU claims, invalid translated
  base, and zero channel count: satisfied.
- Focused tests cover accepted construction and deterministic rejection cases:
  satisfied.
- Accepted implementation and evidence is committed before closeout starts:
  satisfied by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase11-rp1-dma-cache-small-diagnostic-plan-closeout-20260609 on the next
worker wake. That checkpoint must reconcile only the accepted local/static
small diagnostic plan core and must not accept Pi 5 hardware validation,
RP1 MMIO/DMA programming, descriptor rings, transfer completion, interrupt
completion, Ethernet, storage, networking, SSH, Milestone 11.3 completion, or
phase transition by implication.
