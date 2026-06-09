# Phase 11 RP1 DMA/Cache Maintenance Sequence Core

Task: phase11-rp1-dma-cache-maintenance-sequence-core-20260609

Status: accepted

Evidence level: static inspection, local/static implementation review,
fmt/lint gate, unit tests, JSON checks, documentation build, and git diff
checks.

## Goal

Implement the accepted local/static cache-maintenance instruction/barrier
sequence derivation from DmaCacheSyncPlanEvidence without executing cache
maintenance or programming DMA.

## Scope

- Implement phase11-rp1-dma-cache-maintenance-sequence-contract-v1 as a pure
  local/static boundary in src/dma_cache.rs.
- Add static vocabulary for DmaCacheMaintenanceInstruction,
  DmaCacheMaintenanceBarrier, and DmaCacheMaintenanceSequence covering clean,
  invalidate, and clean+invalidate line coverage derived from accepted
  sync-plan evidence.
- Require accepted descriptor and sync-plan evidence, preserving contract and
  source ids, CPU/RP1 addresses, descriptor length, direction, cacheability,
  owner transition, IOMMU classification, cache-line source, aligned start,
  covered length, line count, rejected runtime claims, and local/static
  classification.
- Add pure validators and focused unit tests for valid sequence derivation and
  rejected input cases.
- Record findings with disposition.

## Non-Goals

- No Pi 5 hardware run, boot archive publication, hardwareTestLock
  acquisition, executed dc/dsb cache maintenance for driver buffers, live
  barrier-ordering claim, RP1 MMIO writes, DMA channel programming,
  descriptor rings, Ethernet/storage driver work, networking, SSH, Milestone 12
  work, Milestone 11.3 completion, or phase transition.
- No general driver DMA API, coherent/non-cacheable/IOMMU-backed runtime
  policy, DMA-safe allocation expansion, or high-memory/pinning support beyond
  the accepted descriptor/sync-plan evidence.

## Implementation

- Added DMA_CACHE_MAINTENANCE_SEQUENCE_CONTRACT_ID and the
  local-static-dma-cache-maintenance-sequence-visible classification.
- Added DmaCacheMaintenanceInstruction for CleanByVirtualAddressToPoC,
  InvalidateByVirtualAddressFromPoC, and
  CleanInvalidateByVirtualAddressToPoC, carrying the static instruction
  mnemonics dc cvac, dc ivac, and dc civac.
- Added DmaCacheMaintenanceBarrier::DataSynchronizationBarrierSy carrying the
  static dsb sy barrier mnemonic.
- Added DmaCacheMaintenanceSequence and DmaCacheMaintenanceSequenceEvidence to
  preserve sync-plan and descriptor identity, operation, instruction, barrier,
  64-byte cache-line source, line-aligned CPU start, covered length, line
  count, CPU/RP1 addresses, descriptor length, direction, cacheability, owner
  transition, IOMMU classification, rejected runtime claims, and local/static
  classification.
- Added derive_dma_cache_maintenance_sequence,
  dma_cache_maintenance_sequence_evidence, and
  rejected_dma_cache_maintenance_sequence_evidence.
- Added validation for accepted sync-plan contract id/classification,
  descriptor contract/source ids, accepted cacheability/IOMMU identity,
  rejected-runtime-claim identity, 64-byte cache-line source/size/alignment,
  nonzero covered length, range overflow, and operation/direction/owner
  transition consistency.
- Added focused unit tests for valid clean, invalidate, and clean+invalidate
  sequence derivation plus zero covered length, cache-line mismatch, range
  overflow, unsupported runtime claims, non-accepted sync-plan classification,
  descriptor/sync-plan mismatch, unsupported operation, and rejected-evidence
  formatting.

## Findings

- fixed: implemented the local/static maintenance-sequence vocabulary selected
  by phase11-rp1-dma-cache-maintenance-sequence-contract-v1.
- fixed: mapped accepted sync-plan operations to static dc cvac, dc ivac, and
  dc civac instruction names plus the source-backed dsb sy barrier shape
  without executing those instructions.
- fixed: preserved accepted descriptor and sync-plan evidence identity in the
  emitted sequence evidence so future runtime execution cannot bypass the
  local/static contract.
- fixed: tightened validators to reject non-accepted sync-plan classification,
  descriptor/sync-plan mismatches, zero covered length, cache-line mismatch,
  range overflow, unsupported runtime claims, and unsupported operations.
- fixed: updated roadmap and project contract docs to move the accepted
  frontier from a selected source contract to an implemented local/static
  maintenance-sequence core.
- deferred: executed cache maintenance for driver buffers, live barrier
  ordering, RP1 MMIO, DMA channel programming, descriptor rings, interrupt
  completion, Ethernet, storage, networking, SSH, hardware validation,
  runtime cache-maintenance execution contracts, and Milestone 11.3 completion.
- not-an-issue: no Pi 5 hardware run was required because this task accepts
  only local/static source and unit-test evidence.

No findings were removed.

## Validation

- static inspection: reviewed the accepted driver-adjacent diagnostic contract,
  sync-plan closeout/core evidence, src/dma_cache.rs, src/smp.rs, and
  docs/src/architecture/memory.md.
- fmt/lint/typecheck: cargo fmt --all -- --check passed after applying
  rustfmt formatting.
- unit tests: cargo -Zjson-target-spec test --quiet passed, including the new
  dma_cache maintenance-sequence cases.
- jq JSON checks: task-owned classification.json and evidence-map.json passed
  jq empty.
- docs: /home/node/.cargo/bin/mdbook build passed after roadmap/project doc
  updates.
- diff checks: git diff --check and git diff --cached --check passed.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- src/dma_cache.rs exposes static instruction, barrier, and sequence evidence
  for CleanByVirtualAddressToPoC, InvalidateByVirtualAddressFromPoC, and
  CleanInvalidateByVirtualAddressToPoC derived only from accepted
  DmaCacheSyncPlanEvidence: satisfied.
- Sequence derivation preserves accepted descriptor and sync-plan evidence
  identity, line coverage, direction, cacheability, owner transition, IOMMU
  classification, rejected runtime claims, and local/static classification:
  satisfied.
- Pure validators reject non-accepted sync-plan evidence,
  descriptor/sync-plan mismatches, zero covered length, cache-line-size
  mismatch, range overflow, and unsupported runtime claims: satisfied.
- Focused tests cover one valid sequence for each clean, invalidate, and
  clean+invalidate operation and the required rejection cases: satisfied.
- Accepted implementation is committed before any closeout or runtime/execution
  contract task starts: satisfied by the task commit recorded in supervisor
  state after this task.

## Next Action

Mechanically promote
phase11-rp1-dma-cache-maintenance-sequence-closeout-20260609 on the next
worker wake. That checkpoint must reconcile only the accepted local/static
maintenance-sequence core and must not accept executed cache maintenance, live
barrier ordering, working DMA, descriptor rings, Ethernet, storage, networking,
SSH, hardware validation, or Milestone 11.3 completion by implication.
