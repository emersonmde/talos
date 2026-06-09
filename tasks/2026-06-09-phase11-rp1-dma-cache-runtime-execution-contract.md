# Phase 11 RP1 DMA/Cache Runtime Execution Contract

Task: phase11-rp1-dma-cache-runtime-execution-contract-20260609

Status: accepted

Evidence level: static inspection, documentation reconciliation, JSON checks,
documentation build, and git diff checks.

## Goal

Define the smallest next runtime cache-maintenance execution boundary after the
accepted local/static maintenance-sequence closeout, without executing cache
maintenance, running hardware, programming RP1/DMA MMIO, creating descriptor
rings, or starting Ethernet, storage, networking, or SSH work.

## Scope

- Use the accepted descriptor, sync-plan, and maintenance-sequence evidence to
  name the next runtime/execution contract boundary.
- Explain why that boundary is useful for a future driver path but is not
  premature Ethernet, storage, networking, or SSH progress.
- Preserve accepted local/static evidence as prerequisites and name remaining
  execution, ordering, DMA addressability, IOMMU, allocation, and hardware
  proof gaps.
- Include inconclusive-run triage requirements before any future Pi 5 hardware
  task if hardware becomes necessary.
- Record findings with disposition.

## Non-Goals

- No runtime source changes, Pi 5 hardware run, boot archive publication,
  hardwareTestLock acquisition, executed cache maintenance, live
  barrier-ordering claim, RP1 MMIO writes, DMA channel programming,
  descriptor rings, Ethernet/storage driver work, networking, SSH,
  Milestone 12 work, Milestone 11.3 completion, or phase transition.
- No general driver DMA API, coherent/non-cacheable/IOMMU-backed runtime
  policy, DMA-safe allocation expansion, or high-memory/pinning support beyond
  the accepted descriptor/sync-plan/sequence evidence.

## Retained Inputs

- tasks/2026-06-09-phase11-rp1-dma-cache-maintenance-sequence-closeout.md
- tasks/2026-06-09-phase11-rp1-dma-cache-maintenance-sequence-core.md
- tasks/2026-06-09-phase11-rp1-dma-cache-driver-adjacent-diagnostic-contract.md
- tasks/2026-06-09-phase11-rp1-dma-cache-sync-plan-closeout.md
- tasks/2026-06-09-phase11-rp1-dma-cache-substrate-closeout.md
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-maintenance-sequence-closeout/classification.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-maintenance-sequence-core/classification.json
- src/dma_cache.rs
- src/smp.rs
- docs/src/architecture/memory.md
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md

## Selected Contract

The selected next boundary is
phase11-rp1-dma-cache-maintenance-executor-contract-v1: a future
architecture-gated runtime executor contract that may consume only accepted
DmaCacheMaintenanceSequenceEvidence and translate it into the corresponding
line-by-line cache-maintenance operation and final ordering barrier.

The contract is the smallest useful runtime cache-maintenance boundary because
the accepted local/static frontier already names descriptor evidence,
sync-plan evidence, operation, instruction mnemonic, barrier mnemonic,
line-aligned CPU start, covered length, line count, direction, cacheability,
owner transition, IOMMU classification, rejected runtime claims, and
local/static classification. A future driver cannot safely evaluate DMA buffer
ownership transfer until there is a bounded executor contract saying exactly
which accepted evidence it may consume and which claims it still must not make.

This is not Ethernet, storage, networking, or SSH progress. The boundary has no
device consumer, no RP1 DMA channel programming, no descriptor ring, no
interrupt completion path, no protocol behavior, and no hardware proof. It is a
runtime cache-maintenance contract only.

## Accepted Prerequisites

- The descriptor evidence must satisfy
  phase11-rp1-dma-cache-substrate-contract-v1.
- The sync-plan evidence must satisfy
  phase11-rp1-dma-cache-sync-plan-contract-v1.
- The maintenance-sequence evidence must satisfy
  phase11-rp1-dma-cache-maintenance-sequence-contract-v1 and carry
  local-static-dma-cache-maintenance-sequence-visible classification.
- The accepted sequence must preserve descriptor and sync-plan identity,
  operation, instruction mnemonic, barrier mnemonic, 64-byte cache-line source,
  line-aligned CPU start, covered length, line count, CPU/RP1 addresses,
  descriptor length, direction, cacheability, owner transition, IOMMU
  classification, rejected runtime claims, and local/static classification.
- The only accepted cacheability remains CacheableRequiresMaintenance and the
  only accepted IOMMU classification remains source-unassigned-rp1-dma.
- High memory, reserved memory, coherent mappings, non-cacheable mappings,
  IOMMU-backed policy, DMA-safe allocation beyond descriptor validation,
  descriptor-ring ownership, RP1 DMA channel ownership, and runtime driver
  ownership remain out of contract.

## Minimal Future API Surface

A future implementation task is mechanically objective only if it stays within
this execution-contract shape:

- A small architecture-gated executor surface whose input is accepted
  DmaCacheMaintenanceSequenceEvidence, not raw addresses supplied directly by a
  driver.
- Validation that the evidence carries the accepted maintenance-sequence
  contract id, accepted sync-plan contract id, accepted descriptor contract and
  source ids, accepted local/static sequence classification, accepted
  cacheability/IOMMU identity, the expected rejected-runtime-claims identity,
  nonzero 64-byte line coverage, and a line-count/range that cannot overflow.
- Dispatch from the accepted instruction vocabulary only:
  CleanByVirtualAddressToPoC maps to dc cvac,
  InvalidateByVirtualAddressFromPoC maps to dc ivac, and
  CleanInvalidateByVirtualAddressToPoC maps to dc civac.
- A final DataSynchronizationBarrierSy ordering point after the line loop.
- Evidence output that records this executor contract id, the prerequisite
  descriptor/sync-plan/maintenance-sequence contract ids, operation,
  instruction, barrier, line coverage, CPU/RP1 addresses, direction,
  cacheability, owner transition, IOMMU classification, rejected runtime
  claims, and a runtime-execution-contract classification.

That future implementation must still not program RP1 MMIO or DMA channels,
construct descriptor rings, create Ethernet/storage/networking/SSH behavior,
claim DMA completion, claim hardware validation, or accept Milestone 11.3 by
implication.

## Source Evidence

- src/dma_cache.rs now carries the accepted descriptor, sync-plan, and
  maintenance-sequence evidence vocabulary and validators. It does not execute
  cache maintenance.
- src/smp.rs contains AArch64 source evidence for dc cvac plus dsb sy and
  dc ivac plus dsb sy helpers used in bounded SMP publication paths. Those
  helpers are instruction/barrier-shape evidence only, not an accepted driver
  DMA API.
- The accepted maintenance-sequence core statically names dc civac as the
  clean+invalidate operation shape, but no runtime helper has executed or
  validated that instruction for driver buffers.
- docs/src/architecture/memory.md states that the accepted data-cache-enabled
  boot state is not a DMA coherency contract and does not define DMA-safe
  allocation, explicit clean/invalidate APIs, or driver cache-maintenance
  ownership.
- Retained bcm2712 source evidence records 64-byte data-cache and L2 cache-line
  sizes for the Pi 5 CPU/cache hierarchy.

## Validation Strategy

The next implementation, if supervisor-planned, should remain the thinnest
runtime executor core first:

- static inspection of this contract, accepted descriptor/sync-plan/sequence
  evidence, src/dma_cache.rs, src/smp.rs, and memory architecture docs;
- cargo fmt and focused unit tests for evidence validation, line-count/range
  rejection, operation dispatch, and rejected-input evidence formatting;
- source/disassembly or archive inspection if the future implementation adds
  architecture-gated assembly or inline assembly;
- jq checks for task-owned evidence JSON;
- git diff checks and mdbook build if docs are touched.

No Pi 5 hardware task is mechanically required by this source contract. If a
later supervisor-planned hardware proof becomes necessary, it must serialize on
hardwareTestLock and run the inconclusive-run triage sequence before changing
code after any inconclusive run: candidate identity, fresh serial cursor, TFTP
delta, known-good control, then candidate rerun.

## Findings

- fixed: selected a bounded maintenance-executor contract as the smallest next
  runtime boundary after accepted local/static instruction/barrier sequencing.
- fixed: required future runtime execution to consume accepted
  DmaCacheMaintenanceSequenceEvidence instead of raw driver addresses, so the
  descriptor, sync-plan, and sequence validation chain cannot be bypassed.
- fixed: preserved dc cvac, dc ivac, dc civac, and dsb sy as the only
  accepted operation vocabulary for a future executor while rejecting live
  execution in this task.
- fixed: kept the boundary below Ethernet, storage, networking, SSH, RP1 DMA
  channel programming, descriptor rings, interrupt completion, and hardware
  proof.
- deferred: actual cache-maintenance execution for driver buffers, live
  barrier ordering, DMA channel programming, descriptor rings, interrupt-backed
  completion, coherent/non-cacheable/IOMMU-backed policy, DMA-safe allocation
  beyond descriptor validation, Ethernet, storage, networking, SSH, hardware
  validation, and Milestone 11.3 completion.
- not-an-issue: no Pi 5 hardware run was required because this task accepts
  only a source/static contract and future validation strategy.

No findings were removed.

## Rejected Claims

This contract does not accept:

- Executed cache maintenance for driver buffers.
- Live barrier ordering for a driver path.
- Working DMA behavior.
- RP1 MMIO writes, DMA engine programming, descriptor rings, or interrupt
  completion.
- A general driver DMA API.
- Cache-coherent, non-cacheable, or IOMMU-backed driver policy.
- DMA-safe allocation or pinning beyond accepted descriptor validation.
- Ethernet readiness, storage readiness, networking, SSH, or Milestone 12
  progress.
- Pi 5 hardware validation or Milestone 11.3 completion by implication.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Contract names the smallest next useful runtime cache-maintenance boundary
  and explains why it is not premature Ethernet/storage/networking/SSH work:
  satisfied.
- Contract preserves accepted descriptor, sync-plan, and maintenance-sequence
  evidence as prerequisites and names remaining execution, ordering, DMA
  addressability, IOMMU, allocation, and hardware-proof gaps: satisfied.
- Contract includes inconclusive-run triage requirements before any future
  Pi 5 hardware task if hardware becomes necessary: satisfied.
- Accepted contract is committed or the task is blocked with a precise reason:
  satisfied by the commit recorded in supervisor state after this task.

## Next Action

Set planningNeeded=true for supervisor planning. This contract makes a bounded
phase11-rp1-dma-cache-maintenance-executor-core task mechanically objective,
but the worker must not create that task itself. That future task should
implement only the smallest architecture-gated executor core for accepted
DmaCacheMaintenanceSequenceEvidence and must not run hardware, publish boot
archives, program RP1 MMIO or DMA channels, create descriptor rings, implement
Ethernet, storage, networking, SSH, or accept Milestone 11.3 by implication.
