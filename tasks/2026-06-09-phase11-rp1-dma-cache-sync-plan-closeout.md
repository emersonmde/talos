# Phase 11 RP1 DMA/Cache Sync Plan Closeout

Task: phase11-rp1-dma-cache-sync-plan-closeout-20260609

Status: accepted

Evidence level: static inspection, documentation reconciliation, JSON checks,
documentation build, and git diff checks.

## Goal

Close out the accepted local/static cache-sync-plan core and choose the next
DMA/cache frontier without accepting runtime cache maintenance, DMA behavior,
Ethernet, storage, networking, SSH, or Milestone 11.3 completion by
implication.

## Scope

- Reconcile the accepted driver-adjacent source contract, sync-plan core,
  tests, evidence, docs, and retained risks.
- Record findings with disposition.
- State exactly which local/static sync-plan claims are accepted and which
  runtime/hardware claims remain blocked.
- Select one mechanically objective follow-up only if the checkpoint evidence
  supports it.

## Non-Goals

- No runtime source changes, Pi 5 hardware run, boot archive publication,
  hardwareTestLock acquisition, executed cache maintenance for driver buffers,
  RP1 MMIO writes, DMA programming, descriptor rings, Ethernet, storage,
  networking, SSH, Milestone 12 work, Milestone 11.3 completion, or phase
  transition.

## Reconciled Inputs

- tasks/2026-06-09-phase11-rp1-dma-cache-driver-adjacent-source-contract.md
- tasks/2026-06-09-phase11-rp1-dma-cache-sync-plan-core.md
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-driver-adjacent-source-contract/classification.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-sync-plan-core/classification.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-sync-plan-core/evidence-map.json
- src/dma_cache.rs
- src/smp.rs
- docs/src/architecture/memory.md
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md

## Accepted Frontier

The accepted local/static sync-plan frontier is:

- phase11-rp1-dma-cache-sync-plan-contract-v1 vocabulary in src/dma_cache.rs:
  DmaCacheSyncBoundary, DmaCacheSyncOperation, DmaCacheSyncPlan, and
  DmaCacheSyncPlanEvidence.
- DmaCacheSyncPlan derivation only from an accepted DmaBufferDescriptor plus
  accepted descriptor evidence.
- Operation selection tied to descriptor direction and CPU/device ownership
  boundary: ToDevice before device ownership maps to
  clean-to-point-of-coherency; FromDevice after device ownership maps to
  invalidate-from-point-of-coherency; Bidirectional shared synchronization maps
  to clean-invalidate-to-point-of-coherency.
- Source-backed 64-byte cache-line coverage carrying the retained BCM2712
  cache-line source, line-aligned CPU start, covered length, CPU/RP1
  addresses, descriptor length, direction, cacheability, owner transition,
  IOMMU classification, rejected runtime claims, and local/static
  classification.
- Deterministic rejection of zero length, range overflow, unsupported
  cacheability/IOMMU claims, unsupported direction/boundary combinations,
  non-accepted descriptor classification inputs, and descriptor-evidence
  mismatches.
- Focused unit-test coverage for valid ToDevice/before, FromDevice/after, and
  Bidirectional/shared plans plus rejected overflow, unsupported
  cacheability/IOMMU, unsupported direction/boundary, zero-length,
  non-accepted classification, and evidence mismatch inputs.

This frontier is local/static only. It is not an executed cache-maintenance
path, live barrier-ordering proof, working DMA path, descriptor-ring contract,
hardware proof, or driver-ready DMA API.

## Findings

- fixed: reconciled the accepted source contract, implementation, tests,
  evidence, and docs into one explicit local/static cache-sync-plan frontier.
- fixed: kept operation selection constrained by descriptor direction and
  CPU/device ownership boundary instead of accepting a general
  clean/invalidate API.
- fixed: preserved accepted descriptor evidence as a prerequisite for sync-plan
  derivation so future driver-adjacent work cannot bypass substrate
  validation.
- fixed: kept 64-byte cache-line coverage source-backed and visible in
  evidence rather than treating it as an implicit platform constant.
- fixed: selected the next boundary as the queued driver-adjacent
  diagnostic/source contract because the accepted plan now names the local
  inputs a future runtime boundary must consume without requiring hardware or
  driver behavior yet.
- deferred: executed cache maintenance for driver buffers, live barrier
  ordering, DMA-safe allocation or pinning beyond descriptor validation,
  coherent/non-cacheable/IOMMU-backed policy, descriptor rings, DMA channel
  programming, interrupt completion, Ethernet, storage, networking, SSH,
  hardware validation, Milestone 12 work, and Milestone 11.3 completion.
- not-an-issue: no Pi 5 hardware run was required because the checkpoint only
  closes a local/static implementation and selects a source-contract follow-up.

No findings were removed.

## Rejected Claims

This closeout does not accept:

- Executed cache maintenance for driver buffers.
- Live barrier ordering for a driver path.
- Working DMA behavior.
- DMA engine programming or descriptor-ring ownership.
- Cache-coherent, non-cacheable, or IOMMU-backed driver policy.
- DMA-safe allocation or pinning beyond accepted descriptor validation.
- Interrupt completion, RP1 Ethernet readiness, storage readiness, networking,
  SSH, or Milestone 12 progress.
- Pi 5 hardware validation or Milestone 11.3 completion by implication.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles accepted contract/core evidence and names the exact
  accepted local/static sync-plan frontier: satisfied.
- Checkpoint explicitly rejects executed cache maintenance, live barrier
  ordering, working DMA, descriptor rings, coherent/non-cacheable/IOMMU policy,
  DMA-safe allocation beyond descriptor validation, Ethernet, storage,
  networking, SSH, and Milestone 11.3 completion by implication: satisfied.
- NextAction selects one mechanically objective follow-up task: satisfied by
  selecting
  phase11-rp1-dma-cache-driver-adjacent-diagnostic-contract-20260609.
- Accepted closeout is committed before any next DMA/cache task starts:
  satisfied by the commit recorded in supervisor state after this task.

## Validation

- static inspection: reviewed accepted driver-adjacent source contract,
  sync-plan core task record, classification JSON, evidence map,
  src/dma_cache.rs, src/smp.rs, docs/src/architecture/memory.md,
  docs/src/project/phase11-rp1-pcie-map-contract.md, and docs/src/roadmap.md.
- jq JSON checks: task-owned classification.json and evidence-map.json passed
  jq empty.
- docs: /home/node/.cargo/bin/mdbook build passed after docs updates.
- diff checks: git diff --check and git diff --cached --check passed.

## Next Action

Mechanically promote
phase11-rp1-dma-cache-driver-adjacent-diagnostic-contract-20260609 on the next
worker wake. That task must remain a driver-adjacent source-contract boundary
only and must not run Pi 5 hardware, publish boot archives, execute cache
maintenance for driver buffers, claim live barrier ordering, program RP1 MMIO
or DMA channels, create descriptor rings, implement Ethernet, storage,
networking, SSH, or accept Milestone 11.3 by implication.
