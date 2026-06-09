# Phase 11 RP1 DMA/Cache Driver-Adjacent Source Contract

Task: phase11-rp1-dma-cache-driver-adjacent-source-contract-20260609

Status: accepted

Evidence level: static inspection, documentation reconciliation, JSON checks,
documentation build, and git diff checks.

## Goal

Define the next driver-adjacent DMA/cache source contract after the accepted
local/static descriptor substrate, without starting DMA device work, Ethernet,
storage, networking, SSH, hardware proof, or Milestone 11.3 completion by
implication.

## Scope

- Use the accepted substrate closeout to select the smallest driver-adjacent
  DMA/cache boundary that can produce useful evidence before any RP1 driver
  consumes descriptors.
- Name exact source evidence, accepted prerequisites, forbidden claims, and
  validation strategy.
- Record findings with disposition.

## Non-Goals

- No implementation, Pi 5 hardware run, boot archive publication,
  hardwareTestLock acquisition, RP1 MMIO writes, DMA channel programming,
  executed cache maintenance for driver buffers, Ethernet/storage driver work,
  networking, SSH, Milestone 12 work, Milestone 11.3 completion, or phase
  transition.

## Retained Inputs

- tasks/2026-06-09-phase11-rp1-dma-cache-source-inventory.md
- tasks/2026-06-09-phase11-rp1-dma-cache-contract.md
- tasks/2026-06-09-phase11-rp1-dma-cache-substrate-core.md
- tasks/2026-06-09-phase11-rp1-dma-cache-substrate-closeout.md
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-substrate-closeout/classification.json
- src/dma_cache.rs
- src/smp.rs
- docs/src/architecture/memory.md
- docs/src/project/phase11-rp1-pcie-map-contract.md
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712.dtsi

## Selected Contract

The selected next boundary is
phase11-rp1-dma-cache-sync-plan-contract-v1: a local/static
driver-adjacent cache synchronization plan derived only from an accepted
DmaBufferDescriptor.

The contract is driver-adjacent because it defines the first handoff a future
RP1 driver would need before and after device ownership. It is not networking
or storage work because it has no device-specific consumer, no RP1 MMIO path,
no DMA descriptor ring, no DMA channel programming, no interrupt completion
path, and no Ethernet or block-device protocol.

## Accepted Prerequisites

- The descriptor must already pass
  phase11-rp1-dma-cache-substrate-contract-v1 validation.
- The descriptor memory must remain limited to the accepted low-tail
  bootstrap-bump-owned span; high memory and reserved memory remain out of
  contract.
- The descriptor cacheability must be CacheableRequiresMaintenance and the
  IOMMU classification must remain source-unassigned-rp1-dma.
- RP1 bus addresses must come from the accepted descriptor evidence; a future
  driver must not recompute dma-ranges translation.
- The retained BCM2712 source evidence reports 64-byte data-cache and L2 cache
  line sizes. A later implementation may use 64-byte line coverage for local
  range planning, but it must keep the line-size source explicit in evidence.

## Minimal API Surface

The next implementation task is mechanically objective if it remains within
this local/static shape:

- DmaCacheSyncBoundary: BeforeDeviceOwnership, AfterDeviceOwnership, and
  SharedSynchronizationBoundary.
- DmaCacheSyncOperation: CleanToPointOfCoherency,
  InvalidateFromPointOfCoherency, and CleanInvalidateToPointOfCoherency.
- DmaCacheSyncPlan: descriptor evidence identity, sync boundary, operation,
  cache-line size, line-aligned CPU start, covered length, direction,
  cacheability, owner transition, RP1 bus address, and rejected runtime claims.
- Pure validators that derive the operation from DmaDirection and boundary:
  ToDevice before ownership requires clean; FromDevice after ownership requires
  invalidate; Bidirectional shared boundaries require clean+invalidate.
- Pure range planning that covers the descriptor CPU-visible range with
  64-byte line alignment and rejects zero length, overflow, non-cacheable or
  coherent claims, unsupported IOMMU claims, and descriptors that were not
  produced by the accepted substrate.
- Evidence formatter fields that identify this contract id, the descriptor
  contract id, source inventory id, sync boundary, operation, cache-line
  source, planned CPU range, descriptor CPU/RP1 addresses, length, direction,
  cacheability, IOMMU classification, and local/static classification.

The implementation may add focused unit tests for these pure plan/validation
surfaces. It must not execute dc cvac, dc ivac, dc civac, barriers, RP1 MMIO,
DMA channel writes, descriptor rings, hardware boots, or a network/storage
consumer.

## Source Evidence

- src/smp.rs currently contains proof-bounded AArch64 helpers using dc cvac
  plus dsb sy for cache-line clean to PoC and dc ivac plus dsb sy for
  cache-line invalidate from PoC. Those helpers are source evidence for
  instruction shape and ordering only; they are not accepted as a general
  driver DMA API.
- docs/src/architecture/memory.md states that the accepted data-cache-enabled
  boot state is not a DMA coherency contract and does not define DMA-safe
  allocation, explicit clean/invalidate APIs, or driver cache-maintenance
  ownership.
- Retained bcm2712.dtsi source evidence records 64-byte data-cache and L2
  cache-line sizes for the Pi 5 CPU/cache hierarchy.

## Validation Strategy

The next implementation should be local/static first:

- static inspection of this source contract, the accepted descriptor substrate,
  src/smp.rs instruction-shape evidence, and memory architecture docs;
- cargo fmt and unit tests for the pure sync-plan derivation and rejection
  cases;
- jq checks for task-owned evidence JSON;
- git diff checks and mdbook build if docs are touched.

No Pi 5 hardware task is mechanically required by this source contract. If a
later supervisor-planned hardware proof becomes necessary, it must serialize on
hardwareTestLock and run the inconclusive-run triage sequence before changing
code after any inconclusive run: candidate identity, fresh serial cursor, TFTP
delta, known-good control, then candidate rerun.

## Findings

- fixed: selected cache synchronization planning as the smallest
  driver-adjacent capability after descriptor validation, because a future RP1
  driver cannot safely cross CPU/device ownership without a named cache
  boundary.
- fixed: kept the next boundary local/static and source-backed, avoiding a
  premature jump to Ethernet, storage, networking, SSH, RP1 MMIO, DMA channel
  programming, or hardware proof.
- fixed: tied the cache operation vocabulary to descriptor direction and
  ownership boundary instead of exposing an unconstrained clean/invalidate API.
- fixed: retained existing SMP cache helpers as instruction-shape evidence
  only, not as a reusable driver DMA implementation.
- fixed: required 64-byte cache-line coverage evidence to name its retained
  BCM2712 source rather than becoming an implicit platform constant.
- deferred: executed cache maintenance for driver buffers, barriers in a live
  driver path, DMA channel programming, descriptor rings, interrupt-backed
  completion, Ethernet, storage, networking, SSH, hardware validation, and
  Milestone 11.3 completion.
- not-an-issue: no Pi 5 hardware run was required because this task accepts
  only a source contract and local/static validation strategy.

No findings were removed.

## Rejected Claims

This contract does not accept:

- Working DMA behavior.
- Executed cache maintenance for driver buffers.
- DMA engine programming or descriptor-ring ownership.
- A general driver DMA API.
- Cache-coherent, non-cacheable, or IOMMU-backed driver policy.
- DMA-safe allocation or pinning beyond accepted descriptor validation.
- RP1 Ethernet readiness, storage readiness, networking, SSH, or Milestone 12
  progress.
- Pi 5 hardware validation or Milestone 11.3 completion by implication.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Source contract names the smallest next user-visible substrate capability
  and why it is not premature networking/storage work: satisfied.
- Contract preserves the accepted local/static substrate as prerequisite
  evidence and names additional cache-maintenance, DMA addressability, IOMMU,
  and hardware-proof gaps: satisfied.
- Contract includes inconclusive-run triage requirements before any future
  Pi 5 hardware task if hardware becomes necessary: satisfied.
- Accepted contract is committed or the task is blocked with a precise reason:
  satisfied by the commit recorded in supervisor state after this task.

## Next Action

Set planningNeeded=true for supervisor planning. This source contract makes the
next implementation boundary mechanically objective: a local/static
phase11-rp1-dma-cache-sync-plan-core task that derives cache synchronization
plans from accepted DmaBufferDescriptor evidence and tests valid/rejected
direction/boundary/range cases. The worker must not create that task itself.
That follow-up must not run hardware, publish boot archives, execute cache
maintenance for driver buffers, program RP1 MMIO or DMA channels, create
Ethernet/storage/networking/SSH behavior, or accept Milestone 11.3 by
implication.
