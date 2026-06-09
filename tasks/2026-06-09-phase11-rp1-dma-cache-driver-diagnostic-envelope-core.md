# Phase 11 RP1 DMA/Cache Driver Diagnostic Envelope Core

Task: phase11-rp1-dma-cache-driver-diagnostic-envelope-core-20260609

Status: accepted

Evidence level: static inspection, local implementation, fmt/lint/typecheck,
unit tests, JSON checks, documentation build, and git diff checks.

## Goal

Implement the bounded local/static driver-diagnostic envelope selected by
phase11-rp1-dma-cache-driver-diagnostic-envelope-contract-v1, consuming only
accepted DmaCacheMaintenanceExecutorEvidence before any live DMA or hardware
work.

## Scope

- Add DmaCacheDriverDiagnosticEnvelope input and evidence vocabulary in
  src/dma_cache.rs.
- Preserve descriptor, sync-plan, maintenance-sequence, and executor contract
  identities, source ids, CPU/RP1 addresses, line coverage, direction,
  cacheability, owner transition, IOMMU classification, prerequisite rejected
  claims, and executor rejected claims.
- Record unresolved DMA diagnostic gaps: RP1 DMA channel ownership,
  descriptor-ring layout/ownership, transfer completion and interrupt policy,
  IOMMU/runtime policy, allocation/pinning, hardware proof, and
  device-specific consumer.
- Add focused unit tests for accepted envelope construction and deterministic
  rejection cases.
- Record findings with disposition.

## Non-Goals

- No Pi 5 hardware run, boot archive publication, hardwareTestLock
  acquisition, RP1 MMIO writes, DMA channel programming, descriptor-ring
  construction, interrupt completion, Ethernet/storage driver work,
  networking, SSH, Milestone 12 work, hardware validation claim, Milestone
  11.3 completion, or phase transition.
- No broad driver DMA API, no DMA-safe allocation or pinning expansion, no
  coherent/non-cacheable/IOMMU-backed runtime policy, and no claim that a
  driver has completed DMA.

## Implementation

- Added DMA_CACHE_DRIVER_DIAGNOSTIC_ENVELOPE_CONTRACT_ID for
  phase11-rp1-dma-cache-driver-diagnostic-envelope-contract-v1.
- Added DMA_DRIVER_DIAGNOSTIC_ENVELOPE_LOCAL_STATIC_CLASSIFICATION and a fixed
  unresolved-gap vocabulary covering RP1 DMA channel ownership,
  descriptor-ring layout/ownership, transfer-completion/interrupt policy,
  IOMMU/runtime policy, DMA-safe allocation/pinning, hardware proof, and
  device-specific consumer.
- Added DmaCacheDriverDiagnosticEnvelopeInput, DmaCacheDriverDiagnosticEnvelope,
  and DmaCacheDriverDiagnosticEnvelopeEvidence. The input carries accepted
  executor evidence plus explicit rejected completion-claim booleans; it does
  not accept raw driver addresses.
- Added DmaCacheDriverDiagnosticEnvelopeError with named rejection reasons for
  driver DMA completion claims, hardware/device completion claims, missing
  prerequisite identity, non-accepted executor classification, missing
  rejected-runtime-claim identity, zero line coverage, range mismatch,
  overflow, unsupported cacheability claims, and unsupported IOMMU claims.
- Added build_dma_cache_driver_diagnostic_envelope and
  dma_cache_driver_diagnostic_envelope_evidence. The formatter preserves all
  prerequisite identities and rejected-runtime-claim slices and records that
  driver/hardware completion remains unclaimed.
- Tightened maintenance-sequence range validation so an overflowing line range
  is classified as RangeOverflow before cache-line shape mismatch.
- Updated roadmap, project contract, and memory architecture docs for the
  accepted local/static envelope frontier.

## Findings

- fixed: implemented the selected diagnostic-envelope core in src/dma_cache.rs
  without exposing raw driver address input or a broad driver DMA API.
- fixed: required the envelope to consume accepted
  DmaCacheMaintenanceExecutorEvidence and preserve descriptor, sync-plan,
  maintenance-sequence, and executor identities.
- fixed: preserved CPU/RP1 addresses, descriptor length, line coverage,
  direction, cacheability, owner transition, IOMMU classification,
  prerequisite rejected claims, executor rejected claims, and local/static
  classification in envelope evidence.
- fixed: validators reject non-accepted executor classification, missing
  prerequisite ids, invalid line coverage, overflow, unsupported
  cacheability/IOMMU claims, missing rejected-runtime-claim identity, and
  driver DMA or hardware/device completion claims.
- fixed: corrected maintenance-sequence overflow classification discovered by
  the focused dma_cache test run.
- fixed: updated roadmap/project/memory docs to describe the accepted envelope
  frontier and retained limits.
- deferred: RP1 DMA channel ownership, descriptor-ring layout/ownership,
  transfer completion and interrupt policy, IOMMU/runtime policy,
  DMA-safe allocation or pinning expansion, hardware proof, a device-specific
  consumer, live driver DMA completion, Ethernet, storage, networking, SSH,
  Milestone 12 work, Milestone 11.3 completion, and phase transition.
- not-an-issue: no Pi 5 hardware run was required because this task accepts
  only local/static source and unit-test evidence.

No findings were removed.

## Validation

- static inspection: reviewed the accepted runtime/source contract,
  src/dma_cache.rs, src/smp.rs, docs/src/architecture/memory.md, and
  docs/src/project/phase11-rp1-pcie-map-contract.md.
- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet dma_cache passed; the
  custom runner executed the full no_std test set, 448 passed.
- jq JSON checks: task-owned classification.json and evidence-map.json passed
  jq empty.
- docs: /home/node/.cargo/bin/mdbook build passed after docs updates.
- diff checks: git diff --check and git diff --cached --check passed.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- src/dma_cache.rs exposes a local/static diagnostic-envelope type and
  evidence formatter consuming only accepted DmaCacheMaintenanceExecutorEvidence:
  satisfied.
- Envelope evidence preserves all prerequisite contract identities and
  rejected-claim identities required by the accepted contract: satisfied.
- Validators reject non-accepted executor evidence, missing prerequisite ids,
  invalid line coverage or overflow, unsupported cacheability/IOMMU claims,
  and any input that claims driver DMA completion or hardware/device
  completion: satisfied.
- Focused tests cover one accepted envelope and deterministic rejection cases:
  satisfied.
- Accepted implementation/evidence is committed before the closeout task
  starts: satisfied by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase11-rp1-dma-cache-driver-diagnostic-envelope-closeout-20260609 on the next
worker wake. That checkpoint must reconcile only the accepted local/static
envelope core and must not accept Pi 5 hardware validation, RP1 MMIO/DMA
programming, descriptor rings, interrupt completion, Ethernet, storage,
networking, SSH, Milestone 12 work, Milestone 11.3 completion, or phase
transition by implication.
