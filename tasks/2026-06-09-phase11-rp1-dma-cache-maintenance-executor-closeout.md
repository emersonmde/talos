# Phase 11 RP1 DMA/Cache Maintenance Executor Closeout

Task: phase11-rp1-dma-cache-maintenance-executor-closeout-20260609

Status: accepted

Evidence level: static inspection, documentation reconciliation, JSON checks,
documentation build, and git diff checks.

## Goal

Close out the maintenance-executor core frontier and decide the next
driver-adjacent DMA/cache boundary from accepted evidence.

## Scope

- Reconcile the accepted maintenance-executor core against the
  runtime/execution contract, descriptor evidence, sync-plan evidence, and
  maintenance-sequence evidence.
- Record which executor claims are accepted and which claims remain
  unaccepted.
- Update roadmap/project docs only for accepted frontier changes.
- Select the next action only if accepted executor evidence makes it
  mechanically objective.
- Record findings with disposition.

## Non-Goals

- No runtime source changes beyond documentation/evidence reconciliation.
- No Pi 5 hardware run, boot archive publication, hardwareTestLock
  acquisition, RP1 MMIO writes, DMA channel programming, descriptor rings,
  interrupt completion, Ethernet/storage driver work, networking, SSH,
  Milestone 12 work, hardware validation claim, Milestone 11.3 completion by
  implication, or phase transition.

## Reconciled Inputs

- tasks/2026-06-09-phase11-rp1-dma-cache-maintenance-executor-core.md
- tasks/2026-06-09-phase11-rp1-dma-cache-runtime-execution-contract.md
- tasks/2026-06-09-phase11-rp1-dma-cache-maintenance-sequence-core.md
- tasks/2026-06-09-phase11-rp1-dma-cache-maintenance-sequence-closeout.md
- tasks/2026-06-09-phase11-rp1-dma-cache-sync-plan-core.md
- tasks/2026-06-09-phase11-rp1-dma-cache-substrate-core.md
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-maintenance-executor-core/classification.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-maintenance-executor-core/evidence-map.json
- src/dma_cache.rs
- src/smp.rs
- docs/src/architecture/memory.md
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md

## Accepted Frontier

The accepted maintenance-executor frontier is:

- phase11-rp1-dma-cache-maintenance-executor-contract-v1 implemented in
  src/dma_cache.rs.
- execute_dma_cache_maintenance_sequence consumes only accepted
  DmaCacheMaintenanceSequenceEvidence, not raw driver-supplied addresses.
- Executor validation preserves descriptor, sync-plan, and
  maintenance-sequence identity and rejects wrong contract ids, non-accepted
  sequence classification, wrong cacheability/IOMMU identity, missing
  prerequisite rejected-runtime-claims identity, zero line coverage,
  line-range mismatch, range overflow, and unsupported operation,
  instruction, or barrier vocabulary.
- Architecture-gated dispatch maps the accepted operation vocabulary to
  dc cvac, dc ivac, dc civac, and a final dsb sy boundary.
- Non-AArch64 local/unit-test builds use a no-op dispatch boundary, so local
  tests validate executor evidence and rejection behavior without making a
  hardware claim.
- Runtime-execution evidence records the executor contract id, prerequisite
  contract ids, operation, instruction, barrier, line coverage, CPU/RP1
  addresses, direction, cacheability, owner transition, IOMMU classification,
  prerequisite rejected runtime claims, executor rejected claims, and
  runtime-execution classification.
- Focused unit tests cover accepted clean, invalidate, and clean+invalidate
  executor vocabularies plus representative bypass/rejection cases and
  rejected-evidence formatting.

This frontier is an architecture-gated executor boundary only. It is not a
driver DMA completion path, DMA channel program, descriptor ring, Ethernet or
storage driver, networking or SSH milestone, hardware proof, or Milestone 11.3
completion.

## Milestone 11.3 Acceptance Status

The Milestone 11.3 requirement for documented DMA buffer ownership and
cache-maintenance rules is partially satisfied. The accepted evidence now
documents and implements the descriptor validation, sync-plan,
maintenance-sequence, and maintenance-executor chain for cacheable low-tail
buffers with source-unassigned RP1 DMA/IOMMU classification.

Milestone 11.3 remains incomplete because Talos still lacks a small DMA or
driver-adjacent diagnostic, live DMA completion evidence, RP1 DMA channel
programming, descriptor rings, interrupt completion, DMA-safe allocation or
pinning beyond descriptor validation, coherent/non-cacheable/IOMMU-backed
runtime policy, hardware validation, Ethernet/storage driver readiness,
networking, and SSH.

## Same-Shaped Retry Policy

Same-shaped maintenance-executor core retries are closed. Re-validating the
same accepted DmaCacheMaintenanceSequenceEvidence chain and the same dc cvac,
dc ivac, dc civac, and dsb sy vocabulary is not progress unless a future
supervisor task supplies materially different driver-adjacent runtime scope,
source evidence, hardware evidence, or acceptance criteria.

## Findings

- fixed: reconciled the accepted runtime/execution contract,
  maintenance-executor implementation, tests, evidence, and docs into one
  explicit architecture-gated executor frontier.
- fixed: confirmed that executor inputs must preserve the accepted descriptor,
  sync-plan, and maintenance-sequence chain before any dispatch occurs.
- fixed: kept the accepted dispatch vocabulary bounded to dc cvac, dc ivac,
  dc civac, and dsb sy while preserving non-AArch64 no-op local test behavior.
- fixed: documented that Milestone 11.3's ownership/cache-maintenance-rule
  requirement is partially satisfied, while the diagnostic/runtime DMA half of
  the milestone remains unaccepted.
- fixed: closed same-shaped maintenance-executor retries because the accepted
  core already covers the executor evidence, operation vocabulary, and
  required rejection cases.
- fixed: selected the next boundary as the queued
  driver-adjacent runtime/source-contract task because the accepted executor
  evidence now makes a bounded consumer contract mechanically objective.
- deferred: driver DMA completion, RP1 MMIO writes, DMA channel programming,
  descriptor rings, interrupt completion, Ethernet, storage, networking, SSH,
  coherent/non-cacheable/IOMMU-backed runtime policy, DMA-safe allocation or
  pinning beyond descriptor validation, hardware validation, Milestone 12
  work, and Milestone 11.3 completion.
- not-an-issue: no Pi 5 hardware run was required because this checkpoint
  only closes accepted source/unit-test/documentation evidence and selects a
  guarded follow-up contract.

No findings were removed.

## Rejected Claims

This closeout does not accept:

- Driver DMA completion.
- RP1 MMIO writes or DMA channel programming.
- DMA descriptor rings.
- Interrupt completion.
- Ethernet readiness, storage readiness, networking, SSH, or Milestone 12
  progress.
- Cache-coherent, non-cacheable, or IOMMU-backed driver policy.
- DMA-safe allocation or pinning beyond accepted descriptor validation.
- Pi 5 hardware validation.
- Milestone 11.3 completion by implication.
- Phase transition.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles accepted executor evidence and rejected claims without
  accepting DMA completion, RP1 MMIO/DMA programming, descriptor rings,
  Ethernet, storage, networking, SSH, hardware validation, Milestone 12 work,
  Milestone 11.3 completion, or phase transition by implication: satisfied.
- Checkpoint states whether the Milestone 11.3 requirement for documented DMA
  buffer ownership/cache-maintenance rules is satisfied, partially satisfied,
  or still blocked: satisfied; partially satisfied.
- NextAction selects a bounded driver-adjacent runtime/source-contract task
  only if accepted executor evidence makes it objective; otherwise it sets
  planningNeeded with a precise blocker: satisfied by selecting
  phase11-rp1-dma-cache-driver-adjacent-runtime-contract-20260609.
- Accepted checkpoint is committed before any next task starts: satisfied by
  the commit recorded in supervisor state after this task.

## Validation

- static inspection: reviewed accepted executor task record, evidence map,
  classification JSON, runtime/execution contract, maintenance-sequence
  evidence chain, source diff, src/dma_cache.rs, src/smp.rs,
  docs/src/architecture/memory.md,
  docs/src/project/phase11-rp1-pcie-map-contract.md, and docs/src/roadmap.md.
- jq JSON checks: task-owned classification.json and evidence-map.json passed
  jq empty.
- docs: /home/node/.cargo/bin/mdbook build passed after docs updates.
- diff checks: git diff --check and git diff --cached --check passed.

## Next Action

Mechanically promote
phase11-rp1-dma-cache-driver-adjacent-runtime-contract-20260609 on the next
worker wake. That task must define only the smallest driver-adjacent
runtime/source contract that can consume accepted maintenance-executor
evidence before any DMA-capable driver diagnostic. It must not run Pi 5
hardware, publish boot archives, program RP1 MMIO or DMA channels, create
descriptor rings, implement Ethernet, storage, networking, SSH, or accept
Milestone 11.3 by implication unless its own accepted scope explicitly
authorizes that work.
