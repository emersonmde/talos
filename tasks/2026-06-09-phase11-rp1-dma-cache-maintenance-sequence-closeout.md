# Phase 11 RP1 DMA/Cache Maintenance Sequence Closeout

Task: phase11-rp1-dma-cache-maintenance-sequence-closeout-20260609

Status: accepted

Evidence level: static inspection, documentation reconciliation, JSON checks,
documentation build, and git diff checks.

## Goal

Close out the local/static cache-maintenance sequence frontier and decide the
next safe DMA/cache boundary from accepted evidence.

## Scope

- Reconcile the accepted maintenance-sequence core with the accepted descriptor
  substrate, sync-plan core, driver-adjacent source contract, and memory/cache
  architecture boundaries.
- Record findings with disposition.
- State exactly which instruction/barrier sequence claims are accepted and
  which runtime/hardware claims remain rejected.
- State whether same-shaped local/static sequence retries are closed.
- Select the next guarded runtime/execution-contract boundary only if the
  checkpoint evidence makes it mechanically objective.

## Non-Goals

- No runtime source changes, Pi 5 hardware run, boot archive publication,
  hardwareTestLock acquisition, executed cache maintenance, live
  barrier-ordering claim, RP1 MMIO writes, DMA programming, descriptor rings,
  Ethernet, storage, networking, SSH, Milestone 12 work, Milestone 11.3
  completion, or phase transition.

## Reconciled Inputs

- tasks/2026-06-09-phase11-rp1-dma-cache-driver-adjacent-diagnostic-contract.md
- tasks/2026-06-09-phase11-rp1-dma-cache-maintenance-sequence-core.md
- tasks/2026-06-09-phase11-rp1-dma-cache-sync-plan-closeout.md
- tasks/2026-06-09-phase11-rp1-dma-cache-sync-plan-core.md
- tasks/2026-06-09-phase11-rp1-dma-cache-substrate-closeout.md
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-maintenance-sequence-core/classification.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-maintenance-sequence-core/evidence-map.json
- src/dma_cache.rs
- src/smp.rs
- docs/src/architecture/memory.md
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md

## Accepted Frontier

The accepted local/static maintenance-sequence frontier is:

- phase11-rp1-dma-cache-maintenance-sequence-contract-v1 vocabulary in
  src/dma_cache.rs.
- Static DmaCacheMaintenanceInstruction values for
  CleanByVirtualAddressToPoC, InvalidateByVirtualAddressFromPoC, and
  CleanInvalidateByVirtualAddressToPoC, with the retained source mnemonics
  dc cvac, dc ivac, and dc civac.
- Static DmaCacheMaintenanceBarrier::DataSynchronizationBarrierSy, carrying
  the retained dsb sy barrier shape as source evidence only.
- DmaCacheMaintenanceSequence derivation only from accepted
  DmaCacheSyncPlanEvidence.
- Sequence evidence preserving accepted descriptor and sync-plan identities,
  line-aligned CPU start, covered length, line count, CPU/RP1 addresses,
  descriptor length, direction, cacheability, owner transition, IOMMU
  classification, rejected runtime claims, and local/static classification.
- Deterministic rejection of non-accepted sync-plan classification,
  descriptor/sync-plan mismatches, zero covered length, cache-line mismatch,
  range overflow, unsupported runtime claims, and unsupported operations.
- Focused unit-test coverage for valid clean, invalidate, and
  clean+invalidate sequences plus required rejection cases.

This frontier is local/static only. It is not an executed dc/dsb path, live
barrier-ordering proof, working DMA path, descriptor-ring contract, hardware
proof, or driver-ready runtime cache-maintenance API.

## Same-Shaped Retry Policy

Same-shaped local/static sequence retries are closed. Re-deriving the same
clean, invalidate, clean+invalidate, and dsb sy vocabulary from the same
accepted sync-plan evidence is not progress unless a future supervisor task
adds a materially different runtime/execution contract, new source evidence, or
new acceptance criteria.

## Findings

- fixed: reconciled the accepted driver-adjacent source contract,
  maintenance-sequence implementation, tests, evidence, and docs into one
  explicit local/static instruction/barrier frontier.
- fixed: kept dc cvac, dc ivac, dc civac, and dsb sy as static source-backed
  vocabulary rather than accepting execution or live ordering.
- fixed: preserved descriptor and sync-plan identity in the accepted sequence
  evidence so later runtime work must consume the validated descriptor and
  sync-plan chain.
- fixed: closed same-shaped local/static sequence retries because the accepted
  core already covers the clean, invalidate, and clean+invalidate vocabulary
  plus required rejection cases.
- fixed: selected the next boundary as the queued runtime/execution-contract
  task because the local/static instruction/barrier sequence now names the
  exact evidence a future executor would have to consume.
- deferred: executed cache maintenance for driver buffers, live barrier
  ordering, working DMA behavior, DMA/RP1 MMIO programming, descriptor rings,
  coherent/non-cacheable/IOMMU-backed driver policy, DMA-safe allocation beyond
  descriptor validation, Ethernet, storage, networking, SSH, hardware
  validation, Milestone 12 work, and Milestone 11.3 completion.
- not-an-issue: no Pi 5 hardware run was required because the checkpoint only
  closes a local/static implementation and selects a guarded contract
  follow-up.

No findings were removed.

## Rejected Claims

This closeout does not accept:

- Executed cache maintenance for driver buffers.
- Live barrier ordering for a driver path.
- Working DMA behavior.
- RP1 MMIO writes or DMA channel programming.
- DMA descriptor rings.
- Cache-coherent, non-cacheable, or IOMMU-backed driver policy.
- DMA-safe allocation or pinning beyond accepted descriptor validation.
- Ethernet readiness, storage readiness, networking, SSH, or Milestone 12
  progress.
- Pi 5 hardware validation or Milestone 11.3 completion by implication.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles accepted instruction/barrier sequence evidence and
  preserves rejected runtime/hardware claims: satisfied.
- Checkpoint explicitly states that the maintenance-sequence local/static
  frontier is accepted and same-shaped local/static sequence retries are
  closed: satisfied.
- NextAction selects a precise guarded follow-up task without implying working
  DMA/cache, hardware validation, Ethernet, storage, networking, SSH, or
  Milestone 11.3 completion: satisfied by selecting
  phase11-rp1-dma-cache-runtime-execution-contract-20260609.
- Accepted checkpoint evidence is committed before any runtime/execution
  contract task starts: satisfied by the commit recorded in supervisor state
  after this task.

## Validation

- static inspection: reviewed accepted maintenance-sequence core task record,
  classification JSON, evidence map, driver-adjacent diagnostic contract,
  sync-plan core/closeout, substrate closeout, src/dma_cache.rs, src/smp.rs,
  docs/src/architecture/memory.md, docs/src/project/phase11-rp1-pcie-map-contract.md,
  and docs/src/roadmap.md.
- jq JSON checks: task-owned classification.json and evidence-map.json passed
  jq empty.
- docs: /home/node/.cargo/bin/mdbook build passed after docs updates.
- diff checks: git diff --check and git diff --cached --check passed.

## Next Action

Mechanically promote
phase11-rp1-dma-cache-runtime-execution-contract-20260609 on the next worker
wake. That task must define only the smallest runtime/execution-contract
boundary selected by this checkpoint and must not execute cache maintenance,
run Pi 5 hardware, publish boot archives, program RP1 MMIO or DMA channels,
create descriptor rings, implement Ethernet, storage, networking, SSH, or
accept Milestone 11.3 by implication.
