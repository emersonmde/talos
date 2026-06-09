# Phase 11 RP1 DMA/Cache Driver-Adjacent Runtime Contract

Task: phase11-rp1-dma-cache-driver-adjacent-runtime-contract-20260609

Status: accepted

Evidence level: static inspection, documentation reconciliation, JSON checks,
documentation build, and git diff checks.

## Goal

Define the smallest driver-adjacent runtime/source contract that may consume the
accepted maintenance-executor evidence before any DMA-capable driver diagnostic,
without starting Ethernet, storage, networking, SSH, Pi 5 hardware proof, RP1
DMA programming, descriptor rings, or Milestone 11.3 completion by implication.

## Scope

- Use the accepted maintenance-executor closeout and runtime/execution contract
  to name the next bounded driver-adjacent DMA/cache diagnostic boundary.
- Preserve accepted descriptor, sync-plan, maintenance-sequence, and executor
  evidence as prerequisites.
- Name remaining DMA addressability, RP1 channel ownership, descriptor-ring,
  interrupt-completion, IOMMU, allocation, and hardware-proof gaps.
- Include inconclusive-run triage requirements before any future Pi 5 hardware
  task if hardware becomes necessary.
- Record findings with disposition.

## Non-Goals

- No implementation, runtime source changes, Pi 5 hardware run, boot archive
  publication, hardwareTestLock acquisition, RP1 MMIO writes, DMA channel
  programming, descriptor rings, interrupt completion, Ethernet/storage driver
  work, networking, SSH, Milestone 12 work, Milestone 11.3 completion, or phase
  transition.
- No general driver DMA API, no DMA-safe allocation or pinning expansion, no
  coherent/non-cacheable/IOMMU-backed runtime policy, and no claim that a driver
  has completed DMA.

## Retained Inputs

- tasks/2026-06-09-phase11-rp1-dma-cache-maintenance-executor-closeout.md
- tasks/2026-06-09-phase11-rp1-dma-cache-maintenance-executor-core.md
- tasks/2026-06-09-phase11-rp1-dma-cache-runtime-execution-contract.md
- tasks/2026-06-09-phase11-rp1-dma-cache-maintenance-sequence-closeout.md
- tasks/2026-06-09-phase11-rp1-dma-cache-sync-plan-closeout.md
- tasks/2026-06-09-phase11-rp1-dma-cache-substrate-closeout.md
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-maintenance-executor-closeout/classification.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-maintenance-executor-core/classification.json
- src/dma_cache.rs
- src/smp.rs
- docs/src/architecture/memory.md
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md

## Selected Contract

The selected next boundary is
phase11-rp1-dma-cache-driver-diagnostic-envelope-contract-v1: a local/static
driver-adjacent diagnostic envelope that may consume only accepted
DmaCacheMaintenanceExecutorEvidence and bind it to the explicit gaps a future
DMA-capable RP1 diagnostic must resolve.

This is the smallest useful driver-adjacent DMA/cache diagnostic boundary after
the executor frontier because the accepted chain can now describe and execute
cache maintenance for a cacheable low-tail buffer, but it still has no device
consumer. Before Talos should program a DMA channel or descriptor ring, the next
contract must prove that a candidate diagnostic is only carrying accepted
buffer/cache evidence plus explicit unresolved hardware obligations. That keeps
the feature path aimed at a real driver-adjacent diagnostic without pretending
that source evidence is already working DMA.

This is not Ethernet, storage, networking, or SSH progress. The boundary has no
protocol behavior, no device data path, no RP1 DMA channel programming, no
descriptor-ring allocation, no interrupt completion path, and no hardware
proof. It is a contract for the preflight envelope a later diagnostic can
consume.

## Accepted Prerequisites

- Descriptor evidence must satisfy
  phase11-rp1-dma-cache-substrate-contract-v1.
- Sync-plan evidence must satisfy
  phase11-rp1-dma-cache-sync-plan-contract-v1.
- Maintenance-sequence evidence must satisfy
  phase11-rp1-dma-cache-maintenance-sequence-contract-v1.
- Executor evidence must satisfy
  phase11-rp1-dma-cache-maintenance-executor-contract-v1 and carry
  runtime-execution-dma-cache-maintenance-executor-visible classification.
- The accepted evidence chain must preserve descriptor/source ids, operation,
  instruction, barrier, 64-byte cache-line source, line-aligned CPU start,
  covered length, line count, CPU/RP1 addresses, descriptor length, direction,
  cacheability, owner transition, IOMMU classification, prerequisite rejected
  claims, executor rejected claims, and runtime-execution classification.
- The only accepted buffer class remains cacheable low-tail memory with
  source-unassigned RP1 DMA/IOMMU classification.

## Minimal Future API Surface

A future implementation task is mechanically objective only if it remains within
this diagnostic-envelope shape:

- DmaCacheDriverDiagnosticEnvelope input is accepted
  DmaCacheMaintenanceExecutorEvidence, not raw driver addresses.
- Envelope evidence records this contract id, all prerequisite contract ids,
  descriptor/source ids, operation, instruction, barrier, CPU/RP1 addresses,
  descriptor length, line coverage, direction, cacheability, owner transition,
  IOMMU classification, prerequisite rejected claims, executor rejected claims,
  unresolved DMA-diagnostic gaps, and a local/static driver-adjacent
  classification.
- Validators reject non-accepted executor classification, missing prerequisite
  identities, zero line coverage, cache-line-size mismatch, address/length
  overflow, unsupported cacheability or IOMMU claims, and any attempt to clear
  rejected runtime claims without later hardware evidence.
- The unresolved gap list must include RP1 DMA channel ownership, descriptor
  ring layout/ownership, transfer completion and interrupt policy, hardware
  proof, DMA-safe allocation or pinning beyond descriptor validation,
  coherent/non-cacheable/IOMMU-backed policy, and a device-specific consumer.
- The implementation may add focused local/unit tests for accepted envelope
  construction and rejection cases.

That future implementation must still not program RP1 MMIO or DMA channels,
construct descriptor rings, run Pi 5 hardware, publish boot archives, implement
Ethernet, storage, networking, SSH, claim driver DMA completion, claim hardware
validation, or accept Milestone 11.3 by implication.

## Source Evidence

- src/dma_cache.rs exposes accepted descriptor, sync-plan,
  maintenance-sequence, and maintenance-executor evidence vocabulary and
  validators.
- The accepted executor evidence records the cache-maintenance operation,
  instruction and barrier vocabulary, line coverage, CPU/RP1 addresses,
  direction, cacheability, owner transition, IOMMU classification, prerequisite
  rejected claims, executor rejected claims, and runtime-execution
  classification.
- src/smp.rs remains instruction/barrier-shape evidence for bounded AArch64
  cache helpers; it is not a general driver DMA API.
- docs/src/architecture/memory.md states that data-cache-enabled boot is not a
  DMA coherency contract and does not define DMA-safe allocation, explicit
  driver cache-maintenance ownership, or broad driver policy.
- The accepted maintenance-executor closeout states that Milestone 11.3's
  documented buffer ownership/cache-maintenance rules are partially satisfied
  but that no small DMA or driver-adjacent diagnostic is accepted.

## Validation Strategy

The next implementation, if supervisor-planned, should remain local/static
first:

- static inspection of this contract, accepted descriptor/sync-plan/
  maintenance-sequence/executor evidence, src/dma_cache.rs, src/smp.rs, and
  memory architecture docs;
- cargo fmt and focused unit tests for accepted envelope construction, rejected
  executor classification, missing prerequisite ids, overflow, unsupported
  cacheability/IOMMU claims, and retained rejected-claim identity;
- jq checks for task-owned evidence JSON;
- git diff checks and mdbook build if docs are touched.

No Pi 5 hardware task is mechanically required by this source contract. If a
later supervisor-planned hardware proof becomes necessary, it must serialize on
hardwareTestLock and run the inconclusive-run triage sequence before changing
code after any inconclusive run: candidate identity, fresh serial cursor, TFTP
delta, known-good control, then candidate rerun.

## Findings

- fixed: selected a local/static driver-diagnostic-envelope contract as the
  smallest driver-adjacent boundary after accepted maintenance execution.
- fixed: required any future diagnostic envelope to consume accepted
  DmaCacheMaintenanceExecutorEvidence instead of raw driver addresses.
- fixed: preserved descriptor, sync-plan, maintenance-sequence, and executor
  evidence identity as prerequisites for any future DMA-capable diagnostic.
- fixed: named the remaining RP1 DMA channel, descriptor-ring,
  interrupt-completion, allocation, IOMMU, hardware-proof, and device-consumer
  gaps instead of treating the executor as working DMA.
- fixed: kept Ethernet, storage, networking, SSH, Milestone 12 work, and
  Milestone 11.3 completion outside this boundary.
- deferred: driver DMA completion, RP1 MMIO writes, DMA channel programming,
  descriptor rings, interrupt completion, coherent/non-cacheable/IOMMU-backed
  runtime policy, DMA-safe allocation or pinning beyond descriptor validation,
  a device-specific diagnostic consumer, hardware validation, Ethernet,
  storage, networking, SSH, Milestone 12 work, and Milestone 11.3 completion.
- not-an-issue: no Pi 5 hardware run was required because this task accepts
  only a source/static contract and future validation strategy.

No findings were removed.

## Rejected Claims

This contract does not accept:

- A small DMA or driver-adjacent diagnostic implementation.
- Driver DMA completion.
- RP1 MMIO writes, DMA channel programming, descriptor rings, or interrupt
  completion.
- A general driver DMA API.
- Cache-coherent, non-cacheable, or IOMMU-backed driver policy.
- DMA-safe allocation or pinning beyond accepted descriptor validation.
- Ethernet readiness, storage readiness, networking, SSH, or Milestone 12
  progress.
- Pi 5 hardware validation.
- Milestone 11.3 completion by implication.
- Phase transition.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Contract names the smallest next useful driver-adjacent DMA/cache diagnostic
  boundary and explains why it is not premature Ethernet/storage/networking/SSH
  work: satisfied.
- Contract preserves accepted descriptor, sync-plan, maintenance-sequence, and
  executor evidence as prerequisites and names remaining DMA addressability,
  RP1 channel ownership, descriptor-ring, interrupt-completion, IOMMU,
  allocation, and hardware-proof gaps: satisfied.
- Contract includes inconclusive-run triage requirements before any future
  Pi 5 hardware task if hardware becomes necessary: satisfied.
- Accepted contract is committed or the task is blocked with a precise reason:
  satisfied by the commit recorded in supervisor state after this task.

## Validation

- static inspection: reviewed accepted executor closeout, runtime/execution
  contract, maintenance-executor core evidence, src/dma_cache.rs, src/smp.rs,
  docs/src/architecture/memory.md,
  docs/src/project/phase11-rp1-pcie-map-contract.md, and docs/src/roadmap.md.
- jq JSON checks: task-owned classification.json and evidence-map.json passed
  jq empty.
- docs: /home/node/.cargo/bin/mdbook build passed after docs updates.
- diff checks: git diff --check and git diff --cached --check passed.

## Next Action

Set planningNeeded=true for supervisor planning. This contract makes a bounded
phase11-rp1-dma-cache-driver-diagnostic-envelope-core task mechanically
objective, but the worker must not create that task itself. That future task
should implement only the smallest local/static diagnostic envelope for
accepted DmaCacheMaintenanceExecutorEvidence and must not run hardware, publish
boot archives, program RP1 MMIO or DMA channels, create descriptor rings,
implement Ethernet, storage, networking, SSH, or accept Milestone 11.3 by
implication.
