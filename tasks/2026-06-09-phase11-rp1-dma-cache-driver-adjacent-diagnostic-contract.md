# Phase 11 RP1 DMA/Cache Driver-Adjacent Diagnostic Contract

Task: phase11-rp1-dma-cache-driver-adjacent-diagnostic-contract-20260609

Status: accepted

Evidence level: static inspection, documentation reconciliation, JSON checks,
documentation build, and git diff checks.

## Goal

Define the next driver-adjacent DMA/cache diagnostic or execution-contract
boundary after the accepted local/static sync-plan closeout, without starting
Ethernet, storage, networking, SSH, hardware proof, runtime cache maintenance,
DMA programming, or Milestone 11.3 completion by implication.

## Scope

- Use the accepted sync-plan closeout to select the smallest next boundary that
  can produce useful DMA/cache evidence without jumping to Ethernet, storage,
  networking, or SSH.
- Name exact source evidence, accepted prerequisites, forbidden claims,
  validation strategy, and whether the next task remains local/static,
  QEMU/substitute, or requires later serialized Pi 5 proof.
- Include inconclusive-run triage requirements for any later Pi 5 task.
- Record findings with disposition.

## Non-Goals

- No implementation, Pi 5 hardware run, boot archive publication,
  hardwareTestLock acquisition, executed cache maintenance for driver buffers,
  live barrier-ordering claim, RP1 MMIO writes, DMA channel programming,
  descriptor rings, Ethernet/storage driver work, networking, SSH,
  Milestone 12 work, Milestone 11.3 completion, or phase transition.

## Retained Inputs

- tasks/2026-06-09-phase11-rp1-dma-cache-sync-plan-closeout.md
- tasks/2026-06-09-phase11-rp1-dma-cache-sync-plan-core.md
- tasks/2026-06-09-phase11-rp1-dma-cache-driver-adjacent-source-contract.md
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-sync-plan-closeout/classification.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-sync-plan-core/classification.json
- src/dma_cache.rs
- src/smp.rs
- docs/src/architecture/memory.md
- docs/src/project/phase11-rp1-pcie-map-contract.md
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712.dtsi

## Selected Contract

The selected next boundary is
phase11-rp1-dma-cache-maintenance-sequence-contract-v1: a local/static
instruction-sequence contract derived only from an accepted
DmaCacheSyncPlanEvidence record.

This is the smallest useful driver-adjacent DMA/cache capability after the
sync-plan frontier because a future RP1 driver must know the exact cache-line
operation and barrier sequence required around CPU/device ownership transfer
before runtime code is allowed to execute it. It is not Ethernet, storage,
networking, or SSH progress because it has no device consumer, no RP1 MMIO
address, no DMA descriptor ring, no DMA channel programming, no interrupt
completion path, and no protocol behavior.

## Accepted Prerequisites

- A descriptor must already satisfy
  phase11-rp1-dma-cache-substrate-contract-v1 and emit accepted descriptor
  evidence.
- A sync plan must already satisfy
  phase11-rp1-dma-cache-sync-plan-contract-v1 and emit accepted sync-plan
  evidence.
- The sync-plan evidence must retain the 64-byte BCM2712 cache-line source,
  line-aligned CPU range, covered length, descriptor length, CPU/RP1
  addresses, direction, cacheability, owner transition, IOMMU classification,
  rejected runtime claims, and local/static classification.
- The only accepted cacheability remains CacheableRequiresMaintenance and the
  only accepted IOMMU classification remains source-unassigned-rp1-dma.
- High memory, reserved memory, coherent mappings, non-cacheable mappings,
  IOMMU-backed policy, DMA-safe allocation beyond descriptor validation, and
  runtime driver ownership remain out of contract.

## Minimal API Surface

The next implementation task is mechanically objective if it remains within
this local/static shape:

- DmaCacheMaintenanceInstruction: CleanByVirtualAddressToPoC,
  InvalidateByVirtualAddressFromPoC, and CleanInvalidateByVirtualAddressToPoC.
- DmaCacheMaintenanceBarrier: DataSynchronizationBarrierSy, retained as the
  only source-backed barrier shape from existing Talos SMP helpers until a
  later task accepts a narrower driver barrier policy.
- DmaCacheMaintenanceSequence: sync-plan evidence identity, operation,
  instruction, barrier, cache-line source, cache-line size, line-aligned CPU
  start, covered length, line count, CPU/RP1 addresses, descriptor length,
  direction, cacheability, owner transition, IOMMU classification, rejected
  runtime claims, and local/static classification.
- Pure validators that require accepted sync-plan evidence and map
  CleanToPointOfCoherency to dc cvac plus dsb sy,
  InvalidateFromPointOfCoherency to dc ivac plus dsb sy, and
  CleanInvalidateToPointOfCoherency to dc civac plus dsb sy.
- Pure range planning that computes a deterministic line count from the
  sync-plan line-aligned CPU start and covered length, rejects zero covered
  length, cache-line-size mismatch, range overflow, non-accepted sync-plan
  classification, descriptor evidence mismatches, and unsupported runtime
  claims.
- Evidence formatter fields that identify this contract id, the sync-plan
  contract id, the descriptor contract/source ids, operation, instruction,
  barrier, line coverage, rejected runtime claims, and local/static
  classification.

The implementation may add focused unit tests for these pure sequence and
validation surfaces. It must not execute dc cvac, dc ivac, dc civac, dsb, RP1
MMIO, DMA channel writes, descriptor rings, hardware boots, or a network/storage
consumer.

## Source Evidence

- src/dma_cache.rs now accepts the local/static sync-plan vocabulary,
  descriptor-evidence prerequisite, direction/boundary operation selection,
  source-backed 64-byte line coverage, and rejected runtime claims.
- src/smp.rs contains proof-bounded AArch64 cache helpers using dc cvac plus
  dsb sy for clean-to-PoC and dc ivac plus dsb sy for invalidate-from-PoC.
  These helpers remain instruction-shape and barrier-shape evidence only; they
  are not accepted as a reusable driver DMA cache-maintenance API.
- The AArch64 clean+invalidate operation shape remains source-contract work for
  the next local/static implementation and must be represented as a static
  instruction vocabulary before any runtime path can execute it.
- docs/src/architecture/memory.md states that the accepted data-cache-enabled
  boot state is not a DMA coherency contract and does not define DMA-safe
  allocation, explicit clean/invalidate APIs, or driver cache-maintenance
  ownership.
- Retained bcm2712.dtsi source evidence records 64-byte data-cache and L2
  cache-line sizes for the Pi 5 CPU/cache hierarchy.

## Validation Strategy

The next implementation should remain local/static first:

- static inspection of this contract, accepted descriptor and sync-plan
  evidence, src/smp.rs instruction-shape evidence, and memory architecture
  docs;
- cargo fmt and unit tests for pure instruction-sequence derivation and
  rejection cases;
- jq checks for task-owned evidence JSON;
- git diff checks and mdbook build if docs are touched.

No Pi 5 hardware task is mechanically required by this source contract. If a
later supervisor-planned hardware proof becomes necessary, it must serialize on
hardwareTestLock and run the inconclusive-run triage sequence before changing
code after any inconclusive run: candidate identity, fresh serial cursor, TFTP
delta, known-good control, then candidate rerun.

## Findings

- fixed: selected local/static cache-maintenance instruction sequencing as the
  smallest driver-adjacent capability after sync-plan derivation, because a
  future RP1 driver needs a named instruction/barrier sequence before runtime
  execution can be considered.
- fixed: kept the boundary source-backed and local/static, avoiding a premature
  jump to Ethernet, storage, networking, SSH, RP1 MMIO, DMA channel
  programming, descriptor rings, or hardware proof.
- fixed: preserved accepted descriptor and sync-plan evidence as prerequisites
  so future runtime cache maintenance cannot bypass local/static validation.
- fixed: retained existing SMP cache helpers as instruction/barrier-shape
  evidence only, not as a driver DMA implementation.
- deferred: executed cache maintenance for driver buffers, live barrier
  ordering, DMA channel programming, descriptor rings, interrupt-backed
  completion, Ethernet, storage, networking, SSH, hardware validation,
  coherent/non-cacheable/IOMMU-backed policy, and Milestone 11.3 completion.
- not-an-issue: no Pi 5 hardware run was required because this task accepts
  only a source contract and local/static validation strategy.

No findings were removed.

## Rejected Claims

This contract does not accept:

- Executed cache maintenance for driver buffers.
- Live barrier ordering for a driver path.
- Working DMA behavior.
- RP1 MMIO writes, DMA engine programming, or descriptor-ring ownership.
- A general driver DMA API.
- Cache-coherent, non-cacheable, or IOMMU-backed driver policy.
- DMA-safe allocation or pinning beyond accepted descriptor validation.
- RP1 Ethernet readiness, storage readiness, networking, SSH, or Milestone 12
  progress.
- Pi 5 hardware validation or Milestone 11.3 completion by implication.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Source contract names the smallest next useful DMA/cache capability and why
  it is not premature networking/storage work: satisfied.
- Contract preserves accepted descriptor and sync-plan evidence as
  prerequisites and names additional cache-maintenance, DMA addressability,
  IOMMU, and hardware-proof gaps: satisfied.
- Contract includes inconclusive-run triage requirements before any future
  Pi 5 hardware task if hardware becomes necessary: satisfied.
- Accepted contract is committed or the task is blocked with a precise reason:
  satisfied by the commit recorded in supervisor state after this task.

## Next Action

Set planningNeeded=true for supervisor planning. This source contract makes the
next implementation boundary mechanically objective: a local/static
phase11-rp1-dma-cache-maintenance-sequence-core task that derives a static
instruction/barrier sequence from accepted DmaCacheSyncPlanEvidence and tests
valid/rejected sequence cases. The worker must not create that task itself.
That follow-up must not run hardware, publish boot archives, execute cache
maintenance for driver buffers, claim live barrier ordering, program RP1 MMIO
or DMA channels, create descriptor rings, implement Ethernet, storage,
networking, SSH, or accept Milestone 11.3 by implication.
