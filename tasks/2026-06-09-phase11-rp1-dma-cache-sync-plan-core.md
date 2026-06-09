# Phase 11 RP1 DMA/Cache Sync Plan Core

Task: phase11-rp1-dma-cache-sync-plan-core-20260609

Status: accepted

Evidence level: static inspection, local/static implementation review,
fmt/lint gate, unit tests, JSON checks, documentation build, and git diff
checks.

## Goal

Implement the bounded local/static DMA cache synchronization plan core selected
by the accepted driver-adjacent source contract, without executing cache
maintenance for driver buffers or starting DMA device work.

## Scope

- Own changes to src/dma_cache.rs for pure local/static cache synchronization
  vocabulary, validators, evidence fields, and focused unit tests.
- Add DmaCacheSyncBoundary, DmaCacheSyncOperation, DmaCacheSyncPlan, and
  evidence/validation helpers derived from an accepted DmaBufferDescriptor.
- Derive operation from descriptor direction and synchronization boundary:
  ToDevice before device ownership requires clean, FromDevice after device
  ownership requires invalidate, and Bidirectional shared synchronization
  requires clean+invalidate.
- Plan 64-byte cache-line coverage for the descriptor CPU-visible range while
  carrying the retained BCM2712 cache-line source in evidence.
- Reject zero length, range overflow, unsupported cacheability/IOMMU claims,
  unsupported direction/boundary combinations, and non-accepted descriptor
  classification inputs.
- Record findings with disposition.

## Non-Goals

- No Pi 5 hardware run, boot archive publication, hardwareTestLock
  acquisition, dc cvac/dc ivac/dc civac execution for driver buffers, live
  barrier-ordering claim, RP1 MMIO writes, DMA channel programming, descriptor
  rings, interrupt completion path, Ethernet/storage driver work, networking,
  SSH, Milestone 12 work, Milestone 11.3 completion, or phase transition.

## Implementation

- Added the accepted sync-plan contract id and local/static classification in
  src/dma_cache.rs.
- Added DmaCacheSyncBoundary for BeforeDeviceOwnership,
  AfterDeviceOwnership, and SharedSynchronizationBoundary.
- Added DmaCacheSyncOperation for CleanToPointOfCoherency,
  InvalidateFromPointOfCoherency, and CleanInvalidateToPointOfCoherency.
- Added DmaCacheSyncPlan and DmaCacheSyncPlanEvidence carrying descriptor
  identity, contract/source ids, sync boundary, operation, 64-byte cache-line
  source, line-aligned CPU range, CPU/RP1 addresses, direction, cacheability,
  owner transition, IOMMU classification, rejected runtime claims, and
  local/static classification.
- Added plan_dma_cache_sync, derive_dma_cache_sync_operation,
  dma_cache_sync_plan_evidence, and rejected_dma_cache_sync_plan_evidence.
- Added descriptor-evidence validation so a sync plan requires evidence from
  the accepted substrate contract/classification and rejects mismatched or
  non-accepted descriptor classification inputs.
- Added focused tests for valid ToDevice/before, FromDevice/after,
  Bidirectional/shared, unaligned CPU-visible cache-line coverage, overflow,
  unsupported cacheability/IOMMU, unsupported boundary, zero-length, rejected
  classification, and descriptor-evidence mismatch cases.

## Findings

- fixed: implemented the local/static sync-plan vocabulary and derivation
  helpers selected by phase11-rp1-dma-cache-sync-plan-contract-v1.
- fixed: tied operation selection to descriptor direction plus CPU/device
  ownership boundary rather than exposing unconstrained clean/invalidate names.
- fixed: made 64-byte cache-line coverage source-backed through the retained
  BCM2712 cache-line source and explicit line-aligned range evidence.
- fixed: required accepted descriptor evidence to preserve the substrate
  contract/source/classification before a sync plan can be derived.
- fixed: rejected zero-length, overflow, unsupported cacheability/IOMMU,
  unsupported direction/boundary, non-accepted classification, and evidence
  mismatch cases with deterministic error names.
- deferred: executed cache maintenance for driver buffers, live barrier
  ordering, RP1 MMIO, DMA channel programming, descriptor rings, interrupt
  completion, Ethernet, storage, networking, SSH, hardware validation, and
  Milestone 11.3 completion.
- not-an-issue: no Pi 5 hardware run was required because this task accepts
  only local/static code and unit-test evidence.

No findings were removed.

## Validation

- static inspection: reviewed the accepted source contract and touched source
  module src/dma_cache.rs.
- fmt/lint/typecheck: cargo fmt --all -- --check passed after applying
  rustfmt formatting.
- unit tests: cargo -Zjson-target-spec test --quiet passed with 435 tests,
  including the new dma_cache sync-plan cases.
- jq JSON checks: task-owned classification.json and evidence-map.json passed
  jq empty.
- docs: /home/node/.cargo/bin/mdbook build passed after roadmap/project doc
  updates.
- diff checks: git diff --check and git diff --cached --check passed.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- src/dma_cache.rs exposes pure local/static sync-plan types and derivation
  helpers matching phase11-rp1-dma-cache-sync-plan-contract-v1: satisfied.
- Evidence output includes sync-plan contract id, descriptor contract/source
  ids, sync boundary, operation, 64-byte cache-line source, line-aligned CPU
  start, covered length, CPU/RP1 addresses, descriptor length, direction,
  cacheability, owner transition, IOMMU classification, rejected runtime
  claims, and local/static classification: satisfied.
- Focused tests prove valid ToDevice/before, FromDevice/after, and
  Bidirectional/shared plans plus rejected overflow, unsupported
  cacheability/IOMMU, unsupported direction/boundary, and non-accepted
  descriptor classification inputs: satisfied.
- No runtime path executes cache-maintenance instructions for driver buffers,
  programs DMA/RP1 MMIO, creates descriptor rings, or adds
  Ethernet/storage/networking/SSH behavior: satisfied by static inspection and
  local/static implementation shape.
- Accepted implementation and evidence are committed before the closeout
  checkpoint starts: satisfied by the task commit recorded in supervisor state.

## Next Action

Mechanically promote phase11-rp1-dma-cache-sync-plan-closeout-20260609 on the
next worker wake. That checkpoint must reconcile only the accepted
local/static sync-plan core and must not accept executed cache maintenance,
live barrier ordering, working DMA, DMA/RP1 MMIO programming, descriptor rings,
Ethernet, storage, networking, SSH, hardware validation, or Milestone 11.3 by
implication.
