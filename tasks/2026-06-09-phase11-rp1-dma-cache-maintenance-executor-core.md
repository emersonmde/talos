# Phase 11 RP1 DMA/Cache Maintenance Executor Core

Task: phase11-rp1-dma-cache-maintenance-executor-core-20260609

Status: accepted

Evidence level: static inspection, architecture-gated source inspection,
fmt/lint gate, unit tests, JSON checks, documentation build, and git diff
checks.

## Goal

Implement the bounded architecture-gated cache-maintenance executor selected by
phase11-rp1-dma-cache-maintenance-executor-contract-v1 without accepting driver
DMA completion, hardware validation, RP1 MMIO/DMA programming, descriptor
rings, Ethernet, storage, networking, SSH, or Milestone 11.3 completion.

## Scope

- Consume accepted DmaCacheMaintenanceSequenceEvidence rather than raw
  driver-supplied addresses.
- Validate descriptor, sync-plan, maintenance-sequence, cacheability, IOMMU,
  rejected-runtime-claims, line-coverage, operation, instruction, and barrier
  identity before dispatch.
- Dispatch only the accepted operation vocabulary: dc cvac, dc ivac, dc civac,
  and a final dsb sy through an architecture-gated boundary.
- Return executor evidence that preserves prerequisite contract ids, operation,
  instruction, barrier, line coverage, CPU/RP1 addresses, direction,
  cacheability, owner transition, IOMMU classification, rejected runtime
  claims, and runtime-execution classification.
- Record findings with disposition.

## Non-Goals

- No Pi 5 hardware run, boot archive publication, hardwareTestLock
  acquisition, RP1 MMIO writes, DMA channel programming, descriptor rings,
  interrupt completion, Ethernet/storage driver work, networking, SSH,
  Milestone 12 work, hardware validation claim, general driver DMA API,
  coherent/non-cacheable/IOMMU-backed runtime policy, DMA-safe allocation
  expansion, high-memory/pinning support, Milestone 11.3 completion, or phase
  transition.

## Implementation

- Added DMA_CACHE_MAINTENANCE_EXECUTOR_CONTRACT_ID for
  phase11-rp1-dma-cache-maintenance-executor-contract-v1.
- Added DMA_MAINTENANCE_EXECUTOR_RUNTIME_CLASSIFICATION and executor retained
  rejected-claim vocabulary that continues to reject driver DMA completion,
  RP1 MMIO writes, DMA channel programming, descriptor rings, interrupt
  completion, Ethernet/storage/networking/SSH, hardware validation, and
  Milestone 11.3 completion.
- Added DmaCacheMaintenanceExecutorError with named rejection reasons for
  contract identity mismatch, non-accepted sequence classification,
  cacheability/IOMMU mismatch, missing rejected-runtime-claim identity, zero
  line coverage, line-range mismatch, overflow, unsupported operation,
  unsupported instruction, and unsupported barrier vocabulary.
- Added DmaCacheMaintenanceExecutorEvidence to emit the executor contract id,
  prerequisite contract ids, operation, instruction, barrier, cache-line
  coverage, CPU/RP1 addresses, descriptor length, direction, cacheability,
  owner transition, IOMMU classification, prerequisite rejected-runtime claims,
  executor rejected claims, and runtime classification.
- Added execute_dma_cache_maintenance_sequence, which validates accepted
  DmaCacheMaintenanceSequenceEvidence, dispatches line-by-line cache
  maintenance, emits a final barrier, and returns executor evidence.
- Added AArch64-gated dispatch for dc cvac, dc ivac, dc civac, and dsb sy.
  Non-AArch64 builds use no-op dispatch for local/unit-test validation without
  making hardware claims.
- Added focused tests for accepted clean, invalidate, and clean+invalidate
  executor vocabularies plus bypass/rejection cases and rejected-evidence
  formatting.

## Findings

- fixed: implemented the executor selected by the accepted runtime/execution
  contract as a bounded architecture-gated source boundary in src/dma_cache.rs.
- fixed: required the executor to consume accepted
  DmaCacheMaintenanceSequenceEvidence so callers cannot bypass descriptor,
  sync-plan, or maintenance-sequence identity.
- fixed: validated contract ids, local/static sequence classification,
  cacheability/IOMMU identity, prerequisite rejected-runtime-claims identity,
  nonzero 64-byte line coverage, overflow-safe line range, and accepted
  operation/instruction/barrier vocabulary before dispatch.
- fixed: mapped clean, invalidate, and clean+invalidate operations to dc cvac,
  dc ivac, and dc civac, followed by dsb sy, while preserving no-op local test
  behavior off AArch64.
- fixed: updated roadmap, project contract, and memory architecture docs to
  describe the accepted executor frontier and retained limits.
- deferred: driver DMA completion, RP1 MMIO writes, DMA channel programming,
  descriptor rings, interrupt completion, Ethernet, storage, networking, SSH,
  hardware validation, coherent/non-cacheable/IOMMU-backed policy, DMA-safe
  allocation expansion, high-memory/pinning support, Milestone 12 work, and
  Milestone 11.3 completion.
- not-an-issue: no Pi 5 hardware run was required because this task accepts
  source, architecture-gated implementation, and unit-test evidence only.

No findings were removed.

## Validation

- static inspection: reviewed the accepted runtime/execution contract,
  src/dma_cache.rs, src/smp.rs, docs/src/architecture/memory.md, and the
  accepted maintenance-sequence evidence chain.
- architecture-gated source inspection: src/dma_cache.rs contains the new
  AArch64 dc cvac, dc ivac, dc civac, and dsb sy dispatch boundary, with
  non-AArch64 no-op dispatch for local tests.
- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed, including the new
  DMA/cache maintenance-executor cases.
- jq JSON checks: task-owned classification.json and evidence-map.json passed
  jq empty.
- docs: /home/node/.cargo/bin/mdbook build passed after roadmap/project/memory
  doc updates.
- diff checks: git diff --check and git diff --cached --check passed.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Implementation consumes accepted DmaCacheMaintenanceSequenceEvidence and
  rejects attempts to bypass descriptor, sync-plan, and sequence identity:
  satisfied.
- Executor validation rejects wrong contract ids/classification, wrong
  cacheability/IOMMU identity, missing rejected-runtime-claims identity, zero
  line coverage, overflow/range mismatch, and unsupported operation/barrier
  vocabulary: satisfied.
- Accepted operation dispatch covers dc cvac, dc ivac, dc civac, and final
  dsb sy through an architecture-gated boundary without accepting driver DMA
  completion: satisfied.
- Focused tests cover one accepted sequence plus representative rejection paths
  and evidence formatting: satisfied.
- Accepted implementation is committed before the closeout task starts, or the
  task is blocked with a precise reason: satisfied by the commit recorded in
  supervisor state after this task.

## Next Action

Mechanically promote
phase11-rp1-dma-cache-maintenance-executor-closeout-20260609 on the next worker
wake. That checkpoint must reconcile only the accepted executor evidence and
must not accept hardware validation, DMA programming, descriptor rings,
networking, SSH, or Milestone 11.3 completion by implication.
