# Phase 11 RP1 DMA/Cache Substrate Closeout

Task: phase11-rp1-dma-cache-substrate-closeout-20260609

Status: accepted

Evidence level: static inspection, documentation reconciliation, JSON checks,
documentation build, and git diff checks.

## Goal

Close out the accepted local/static DMA/cache substrate core and choose the
next DMA/cache frontier without accepting runtime DMA behavior, cache
maintenance execution, Ethernet, storage, networking, SSH, or Milestone 11.3
completion by implication.

## Scope

- Reconcile the accepted DMA/cache source inventory, substrate contract,
  local/static core implementation, tests, evidence, docs, and retained risks.
- Record findings with disposition.
- State which local/static claims are accepted and which runtime/hardware
  claims remain blocked.
- Select a mechanically objective follow-up only if the accepted closeout
  evidence supports it.

## Non-Goals

- No runtime source changes, Pi 5 hardware run, boot archive publication,
  hardwareTestLock acquisition, RP1 MMIO writes, DMA programming, executed
  cache maintenance for driver buffers, Ethernet, storage, networking, SSH,
  Milestone 12 work, Milestone 11.3 completion, or phase transition.

## Reconciled Inputs

- tasks/2026-06-09-phase11-rp1-dma-cache-source-inventory.md
- tasks/2026-06-09-phase11-rp1-dma-cache-contract.md
- tasks/2026-06-09-phase11-rp1-dma-cache-substrate-core.md
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-source-inventory/classification.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-contract/classification.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-substrate-core/classification.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-substrate-core/evidence-map.json
- src/dma_cache.rs
- src/main.rs
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md

## Accepted Frontier

The accepted local/static frontier is:

- RP1 DMA/cache vocabulary for direction, cacheability, address path, buffer
  descriptor, buffer owner, and IOMMU classification.
- RP1 RAM-window and RP1 peripheral-window translation helpers derived from
  retained dma-ranges evidence.
- Pure descriptor validation for alignment, zero/overflow length,
  bootstrap-bump-owned low-tail span containment, RP1 translation
  range/overflow, high-memory and reserved-memory rejection, and forbidden
  cacheability/IOMMU claims.
- Local/static evidence fields carrying the contract id, source inventory id,
  CPU and RP1 addresses, length, alignment, direction, cacheability, owner,
  IOMMU classification, validation results, and classification.
- Focused unit tests for one valid RP1 RAM-window descriptor and rejected
  alignment, ownership-span, high-memory, reserved-memory, translation,
  cacheability, and IOMMU inputs.

This frontier is local/static only. It is not a DMA-safe allocator, DMA device
API, hardware proof, or driver-ready cache maintenance API.

## Findings

- fixed: reconciled the accepted source inventory, contract, implementation,
  tests, evidence, and docs into one explicit local/static DMA/cache frontier.
- fixed: selected the next boundary as a driver-adjacent source contract
  because the accepted core now makes it meaningful to define the smallest
  cache-maintenance or DMA diagnostic consumer without jumping to networking or
  storage.
- fixed: kept RP1 bus address translation attached to retained dma-ranges
  evidence and blocked ad hoc driver-side recomputation.
- fixed: kept coherent, non-cacheable, and IOMMU-backed policies rejected
  until a later task supplies mapping, policy, and hardware/source evidence.
- deferred: cache-maintenance instruction execution for driver buffers,
  DMA-safe allocation or pinning beyond descriptor validation, descriptor
  rings, DMA channel programming, driver-adjacent hardware proof, Ethernet,
  storage, networking, SSH, and Milestone 11.3 completion.
- deferred: CPU-visible address alias/equality policy remains evidence-only in
  the accepted descriptor and must be decided before a future driver consumes
  non-identity or high-memory buffers.
- not-an-issue: no Pi 5 hardware run was required because the checkpoint only
  closes a local/static implementation and selects a source-contract follow-up.

No findings were removed.

## Rejected Claims

This closeout does not accept:

- Working DMA behavior.
- DMA engine programming or descriptor-ring ownership.
- Executed cache maintenance for driver buffers.
- Cache-coherent, non-cacheable, or IOMMU-backed driver policy.
- DMA-safe allocation or pinning beyond pure descriptor validation.
- RP1 Ethernet readiness, storage readiness, networking, SSH, or Milestone 12
  progress.
- Pi 5 hardware validation or Milestone 11.3 completion by implication.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles accepted contract/core evidence and names the exact
  accepted local/static frontier: satisfied.
- Checkpoint rejects working DMA, executed cache maintenance,
  coherent/non-cacheable/IOMMU policy, DMA-safe allocation beyond descriptor
  validation, Ethernet/storage/networking/SSH, and Milestone 11.3 acceptance by
  implication: satisfied.
- NextAction selects one mechanically objective follow-up task: satisfied by
  selecting phase11-rp1-dma-cache-driver-adjacent-source-contract-20260609.
- Accepted closeout is committed before any next DMA/cache task starts:
  satisfied by the commit recorded in supervisor state after this task.

## Validation

- static inspection: reviewed accepted source inventory, contract, substrate
  core task records, classification JSON, evidence map, src/dma_cache.rs,
  src/main.rs, docs/src/project/phase11-rp1-pcie-map-contract.md, and
  docs/src/roadmap.md.
- jq JSON checks: task-owned classification.json and evidence-map.json passed
  jq empty.
- docs: /home/node/.cargo/bin/mdbook build passed after docs updates.
- diff checks: git diff --check and git diff --cached --check passed.

## Next Action

Mechanically promote
phase11-rp1-dma-cache-driver-adjacent-source-contract-20260609 on the next
worker wake. That task must remain a source-contract task only and must not run
Pi 5 hardware, publish boot archives, program RP1 MMIO or DMA channels, execute
cache maintenance for driver buffers, implement Ethernet/storage/networking/SSH,
or accept Milestone 11.3 by implication.
