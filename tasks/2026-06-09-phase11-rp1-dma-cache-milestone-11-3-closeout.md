# Phase 11 RP1 DMA/Cache Milestone 11.3 Closeout

Task: phase11-rp1-dma-cache-milestone-11-3-closeout-20260609
Status: accepted
Classification: rp1-dma-cache-milestone-11-3-frontier-closed

## Goal

Close Milestone 11.3 only from accepted DMA/cache substrate and diagnostic
evidence, without accepting live DMA, RP1 MMIO/DMA programming, descriptor
rings, transfer completion, interrupt completion, Ethernet/storage readiness,
networking, SSH, Milestone 12 work, or a phase transition.

## Scope

- Reconcile the accepted Milestone 11.3 chain: source inventory, DMA/cache
  substrate, cache-sync planning, maintenance sequence, maintenance executor,
  driver-adjacent diagnostic envelope, small diagnostic plan, visibility/control
  proof, project docs, roadmap, and retained risks.
- Decide whether the roadmap criteria are satisfied by accepted evidence:
  documented DMA buffer ownership/cache-maintenance rules and a small DMA or
  driver-adjacent diagnostic before networking depends on DMA.
- Record findings with disposition.
- Update the required project/roadmap docs for the accepted Milestone 11.3
  frontier only.

## Non-Goals

No implementation changes, hardware run, boot archive publication,
hardwareTestLock acquisition, RP1 MMIO/DMA programming, channel ownership,
descriptor-ring construction or ownership, transfer completion, interrupt
completion, Ethernet/storage work, networking, SSH, Milestone 12 work, or phase
transition.

## Reconciled Inputs

- tasks/2026-06-09-phase11-rp1-dma-cache-source-inventory.md
- tasks/2026-06-09-phase11-rp1-dma-cache-contract.md
- tasks/2026-06-09-phase11-rp1-dma-cache-substrate-core.md
- tasks/2026-06-09-phase11-rp1-dma-cache-substrate-closeout.md
- tasks/2026-06-09-phase11-rp1-dma-cache-driver-adjacent-source-contract.md
- tasks/2026-06-09-phase11-rp1-dma-cache-sync-plan-core.md
- tasks/2026-06-09-phase11-rp1-dma-cache-sync-plan-closeout.md
- tasks/2026-06-09-phase11-rp1-dma-cache-driver-adjacent-diagnostic-contract.md
- tasks/2026-06-09-phase11-rp1-dma-cache-maintenance-sequence-core.md
- tasks/2026-06-09-phase11-rp1-dma-cache-maintenance-sequence-closeout.md
- tasks/2026-06-09-phase11-rp1-dma-cache-runtime-execution-contract.md
- tasks/2026-06-09-phase11-rp1-dma-cache-maintenance-executor-core.md
- tasks/2026-06-09-phase11-rp1-dma-cache-maintenance-executor-closeout.md
- tasks/2026-06-09-phase11-rp1-dma-cache-driver-adjacent-runtime-contract.md
- tasks/2026-06-09-phase11-rp1-dma-cache-driver-diagnostic-envelope-core.md
- tasks/2026-06-09-phase11-rp1-dma-cache-driver-diagnostic-envelope-closeout.md
- tasks/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-source-contract.md
- tasks/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-plan-core.md
- tasks/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-plan-closeout.md
- tasks/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-hardware-proof-contract.md
- tasks/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-visibility-core.md
- tasks/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-visibility-closeout.md
- tasks/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-visibility-v3-retry.md
- tasks/2026-06-09-phase11-rp1-dma-cache-small-diagnostic-visibility-v3-closeout.md
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-milestone-11-3-closeout/evidence-map.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-milestone-11-3-closeout/classification.json
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md
- src/dma_cache.rs

## Closeout Decision

Milestone 11.3 closes as
rp1-dma-cache-milestone-11-3-frontier-closed.

The DMA buffer ownership and cache-maintenance rule criterion is satisfied by
the accepted source inventory, substrate contract/core/closeout, cache-sync
plan, static maintenance sequence, and architecture-gated maintenance executor:

- phase11-rp1-dma-cache-source-inventory-20260609 commit 633b5536 records the
  source facts for RP1 dma-ranges, rp1_dma identity, selected iommu5
  attachment inventory, and Talos DMA/cache ownership gaps.
- phase11-rp1-dma-cache-contract-20260609 commit 1fb7f4cc accepts the
  phase11-rp1-dma-cache-substrate-contract-v1 boundary for DMA buffer
  descriptor fields, address translation, cache-maintenance semantics, and
  IOMMU classification.
- phase11-rp1-dma-cache-substrate-core-20260609 commit 89f9f8b2 and
  phase11-rp1-dma-cache-substrate-closeout-20260609 commit b9a35c6e accept
  local/static descriptor validation, RP1 address translation helpers, and
  evidence fields for low-tail bootstrap-bump-owned cacheable buffers.
- phase11-rp1-dma-cache-driver-adjacent-source-contract-20260609 commit
  33a913ae, phase11-rp1-dma-cache-sync-plan-core-20260609 commit 1afe6703,
  and phase11-rp1-dma-cache-sync-plan-closeout-20260609 commit 7d1bc988 accept
  source-backed cache-sync planning, operation selection, and 64-byte line
  coverage derived only from accepted descriptor evidence.
- phase11-rp1-dma-cache-driver-adjacent-diagnostic-contract-20260609 commit
  ff121255, phase11-rp1-dma-cache-maintenance-sequence-core-20260609 commit
  a9b2e40a, and phase11-rp1-dma-cache-maintenance-sequence-closeout-20260609
  commit 0d75a5ab accept static clean/invalidate/clean+invalidate and dsb sy
  sequence evidence derived only from accepted sync-plan evidence.
- phase11-rp1-dma-cache-runtime-execution-contract-20260609 commit f9a2ffa6,
  phase11-rp1-dma-cache-maintenance-executor-core-20260609 commit 30c94de1,
  and phase11-rp1-dma-cache-maintenance-executor-closeout-20260609 commit
  f15742b4 accept the bounded architecture-gated executor evidence chain after
  validating descriptor, sync-plan, and maintenance-sequence identities.

The small DMA or driver-adjacent diagnostic criterion is satisfied at the
accepted driver-adjacent/local-static and visibility-proof levels, not by a
live DMA transfer:

- phase11-rp1-dma-cache-driver-adjacent-runtime-contract-20260609 commit
  6b787a90, phase11-rp1-dma-cache-driver-diagnostic-envelope-core-20260609
  commit 1ea79026, and
  phase11-rp1-dma-cache-driver-diagnostic-envelope-closeout-20260609 commit
  8c864112 accept a driver-adjacent diagnostic envelope that consumes only the
  accepted maintenance-executor evidence and preserves unresolved RP1 DMA
  channel, descriptor-ring, interrupt/completion, IOMMU/runtime-policy,
  allocation/pinning, hardware-proof, and device-consumer gaps.
- phase11-rp1-dma-cache-small-diagnostic-source-contract-20260609 commit
  ebe48537, phase11-rp1-dma-cache-small-diagnostic-plan-core-20260609 commit
  894e8875, and phase11-rp1-dma-cache-small-diagnostic-plan-closeout-20260609
  commit ad7ae554 accept a local/static RP1 DMA small diagnostic plan that
  connects the accepted cache-maintained low-tail diagnostic envelope to
  retained rp1_dma source facts.
- phase11-rp1-dma-cache-small-diagnostic-hardware-proof-contract-20260609
  commit 26d7eabc, phase11-rp1-dma-cache-small-diagnostic-visibility-core
  commit 9f717e34, and
  phase11-rp1-dma-cache-small-diagnostic-visibility-closeout-20260609 commit
  6c645e5d accept only the local/static visibility report surface and the
  guarded future Pi 5 visibility/control proof boundary.
- phase11-rp1-dma-cache-small-diagnostic-visibility-v3-retry-20260609 commit
  577beab4 and
  phase11-rp1-dma-cache-small-diagnostic-visibility-v3-closeout-20260609
  commit b91ea912 accept the run-unique Pi 5 candidate/control visibility
  proof: the candidate prints the accepted plan report, the no-plan control
  uses the same path while withholding accepted plan evidence, both pass the
  repaired run-unique freshness discriminator, and same-shaped hardware
  visibility/control retries are closed.

This closeout does not reinterpret the diagnostic as live DMA. The accepted
Milestone 11.3 frontier is limited to documented DMA/cache ownership rules,
local/static descriptor/sync/maintenance/executor/diagnostic evidence, and Pi 5
visibility/control proof of the accepted plan report path. It remains
insufficient for channel ownership, descriptor-ring construction or ownership,
transfer completion, interrupt completion, Ethernet/storage readiness,
networking, SSH, Milestone 12 work, or a phase transition.

No explicit mechanically unblocked follow-up task remains in this milestone
slice after this closeout. Supervisor planning is required before any Phase 12,
descriptor-ring/channel-ownership, live DMA, networking, or later work starts.

## Findings And Disposition

- fixed: reconciled accepted RP1 DMA source inventory, dma-ranges,
  controller identity, IOMMU inventory, and Talos DMA/cache ownership gaps.
- fixed: reconciled the accepted DMA buffer ownership/cache-maintenance rules
  across descriptor validation, cache-sync planning, maintenance sequencing,
  and the architecture-gated maintenance executor.
- fixed: reconciled the accepted driver-adjacent diagnostic envelope and small
  diagnostic plan as the milestone's diagnostic evidence while preserving all
  unresolved live-DMA gaps.
- fixed: reconciled the run-unique Pi 5 visibility/control proof as real
  hardware visibility of the accepted plan report path, not live DMA behavior.
- fixed: updated the project contract and roadmap docs to mark only the
  Milestone 11.3 accepted frontier and retained risks.
- deferred: live DMA, RP1 MMIO/DMA programming, channel ownership, descriptor
  rings, transfer completion, interrupt completion, hardware/device
  completion, Ethernet/storage readiness, networking, SSH, Milestone 12 work,
  and phase transition remain future supervisor-planned work.
- not-an-issue: no hardware run was required; this task is a static checkpoint
  over already accepted source, local/static, and serial hardware visibility
  evidence.

No findings were removed.

## Accepted Claims

- RP1 DMA addressability, dma-ranges, selected IOMMU inventory, and
  cache-coherency requirements are documented from retained source evidence.
- Talos has accepted local/static DMA buffer descriptor ownership,
  RP1 address-translation, cache-sync planning, cache-maintenance sequencing,
  and architecture-gated executor evidence for the low-tail cacheable buffer
  shape.
- Talos has an accepted driver-adjacent diagnostic envelope and local/static
  RP1 DMA small diagnostic plan before networking depends on DMA.
- Pi 5 serial hardware evidence proves visibility of the accepted small
  diagnostic plan report path and a paired no-plan control path under the
  repaired run-unique freshness discriminator.
- Milestone 11.3 roadmap acceptance criteria are satisfied at this documented
  frontier and with the retained risk boundaries below.

## Rejected Claims And Retained Risks

This checkpoint does not accept live DMA, RP1 MMIO writes, RP1 DMA channel
ownership, DMA channel programming, descriptor-ring construction or ownership,
transfer completion, interrupt completion, hardware/device completion,
Ethernet readiness, storage readiness, networking, SSH, Milestone 12 work,
Phase 12 planning, or a phase transition.

Retained risks:

- no live RP1 DMA transfer has run;
- no descriptor-ring layout, ownership, or channel ownership has been accepted;
- no transfer-completion or interrupt-completion policy has been accepted;
- IOMMU runtime policy and DMA-safe allocation/pinning remain deferred;
- no Ethernet/storage DMA consumer is accepted.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-09-phase11-rp1-dma-cache-milestone-11-3-closeout/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-09-phase11-rp1-dma-cache-milestone-11-3-closeout/classification.json.
- Project contract:
  docs/src/project/phase11-rp1-pcie-map-contract.md.
- Roadmap:
  docs/src/roadmap.md.
- Implementation evidence:
  src/dma_cache.rs.

## Validation

- static inspection: accepted Milestone 11.3 task records, evidence JSON,
  docs, git history, and src/dma_cache.rs.
- JSON checks: jq empty on task-owned evidence-map/classification JSON.
- diff checks: git diff --check and git diff --cached --check.
- documentation build: /home/node/.cargo/bin/mdbook build.

## Acceptance

- Task record lists findings with disposition: satisfied.
- Checkpoint cites accepted commits/evidence that satisfy DMA buffer ownership
  and cache-maintenance rules: satisfied.
- Checkpoint cites accepted small DMA or driver-adjacent diagnostic evidence
  and states exact limits: satisfied.
- Checkpoint rejects live DMA, RP1 MMIO/DMA programming, descriptor rings,
  transfer completion, interrupt completion, Ethernet/storage readiness,
  networking, SSH, Milestone 12 work, and phase transition: satisfied.
- Roadmap/project docs are updated only for the accepted Milestone 11.3
  frontier and retained risks: satisfied.
- Accepted closeout is committed before any Phase 12 or later planning starts:
  satisfied by the commit recorded in supervisor state after this task.

## Next Action

Supervisor planning required. No explicit mechanically unblocked follow-up task
remains in the accepted Milestone 11.3 slice. Do not promote
phase11-rp1-dma-cache-channel-ownership-source-contract-20260609 because the
accepted v3 closeout selected this milestone closeout instead. Do not start
Phase 12, descriptor-ring/channel-ownership, live DMA, networking, or phase
transition work without supervisor planning.
