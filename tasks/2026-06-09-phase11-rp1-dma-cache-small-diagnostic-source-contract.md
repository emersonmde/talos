# Phase 11 RP1 DMA/Cache Small Diagnostic Source Contract

Task: phase11-rp1-dma-cache-small-diagnostic-source-contract-20260609

Status: accepted

Evidence level: static inspection, documentation reconciliation, JSON checks,
documentation build, and git diff checks.

## Goal

Define the smallest source-backed small DMA diagnostic question that can
consume the accepted driver-diagnostic-envelope evidence, without starting
Ethernet, storage, networking, SSH, RP1 DMA programming, descriptor rings,
hardware proof, or Milestone 11.3 completion by implication.

## Scope

- Use the accepted driver-diagnostic-envelope closeout and retained RP1 source
  evidence to name the next bounded DMA diagnostic contract boundary.
- Preserve accepted envelope evidence as a prerequisite and keep unresolved RP1
  channel, descriptor-ring, transfer-completion, interrupt-policy,
  IOMMU/runtime, allocation/pinning, hardware-proof, and device-consumer gaps
  explicit.
- Include serialized hardware lock and inconclusive-run triage requirements
  before any future Pi 5 hardware task if hardware proof becomes necessary.
- Record findings with disposition.

## Non-Goals

- No implementation, runtime source changes, Pi 5 hardware run, boot archive
  publication, hardwareTestLock acquisition, RP1 MMIO writes, DMA channel
  programming, descriptor rings, interrupt completion, Ethernet/storage driver
  work, networking, SSH, Milestone 12 work, Milestone 11.3 completion, or phase
  transition.
- No claim of RP1 DMA channel ownership, transfer completion, interrupt-backed
  completion, device-specific consumer readiness, cache-coherent/non-cacheable
  or IOMMU-backed runtime policy, or DMA-safe allocation beyond accepted
  descriptor validation.

## Retained Inputs

- tasks/2026-06-09-phase11-rp1-dma-cache-driver-diagnostic-envelope-closeout.md
- tasks/2026-06-09-phase11-rp1-dma-cache-driver-diagnostic-envelope-core.md
- tasks/2026-06-09-phase11-rp1-dma-cache-driver-adjacent-runtime-contract.md
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-driver-diagnostic-envelope-closeout/classification.json
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-driver-diagnostic-envelope-core/classification.json
- src/dma_cache.rs
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi-5-b.dts
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/rp1.dtsi

## Source Facts

- Retained rp1.dtsi names rp1_dma as compatible snps,axi-dma-1.01a at RP1 bus
  0xc0_40188000, with RP1_INT_DMA, clocks RP1_CLK_DMA and RP1_CLK_SYS,
  #dma-cells = <1>, 8 DMA channels, one master, 64 targets, 128-bit data
  width, per-channel block size 0x40000, and source AXI burst limits.
- Retained bcm2712-rpi-5-b.dts enables rp1_dma and maps the RP1 peripheral
  window so the DMA controller's RP1 bus base translates through the accepted
  RP1 peripheral path to CPU physical 0x1f_0018_8000.
- Retained rp1.dtsi names source DMA consumers before networking: SPI, PIO,
  audio/I2S, and optional UART aliases. Those consumers prove RP1 DMA is a real
  substrate dependency, but they do not select a Talos device consumer for this
  task.
- The accepted driver-diagnostic-envelope frontier can already prove the
  cache-maintained low-tail buffer evidence chain, but it has no RP1 channel
  ownership, descriptor-ring layout, transfer-completion policy, interrupt
  policy, runtime IOMMU policy, allocation/pinning expansion, hardware proof,
  or device consumer.

## Selected Contract

The selected next boundary is
phase11-rp1-dma-cache-small-diagnostic-plan-contract-v1: a local/static source
contract for a future small DMA diagnostic plan that may consume only accepted
DmaCacheDriverDiagnosticEnvelopeEvidence plus retained RP1 DMA controller
source facts.

The smallest next diagnostic question is:

Can Talos construct an evidence-backed plan for one RP1 AXI DMA diagnostic using
an accepted cache-maintained low-tail buffer envelope and the source-backed
rp1_dma controller identity, while still rejecting channel programming,
descriptor-ring construction, transfer completion, interrupt completion, and
hardware/device completion?

This is useful because it is the first boundary that connects accepted DMA/cache
buffer evidence to the RP1 DMA controller source shape. It is not Ethernet,
storage, networking, or SSH work because it has no protocol behavior, no GEM or
storage consumer, no live device transfer, no descriptor ring, no interrupt
handler, and no hardware proof. It also is not a working small DMA diagnostic;
it is a source contract for the plan a later task may implement locally before
any serialized hardware proof is considered.

## Accepted Prerequisites

- Driver-diagnostic envelope evidence must satisfy
  phase11-rp1-dma-cache-driver-diagnostic-envelope-contract-v1 and carry
  local-static-dma-cache-driver-diagnostic-envelope-visible classification.
- Envelope evidence must preserve descriptor, sync-plan, maintenance-sequence,
  and executor contract identities and source ids, CPU/RP1 addresses,
  descriptor length, line coverage, direction, cacheability, owner transition,
  IOMMU classification, prerequisite rejected claims, executor rejected claims,
  unresolved diagnostic gaps, and local/static classification.
- The accepted buffer class remains cacheable low-tail memory with
  source-unassigned RP1 DMA/IOMMU classification.
- RP1 DMA controller identity must come from retained source evidence:
  snps,axi-dma-1.01a, RP1 bus base 0xc0_40188000, CPU physical base
  0x1f_0018_8000, 8 channels, 64 targets, and the retained clock/interrupt
  names.

## Minimal Future API Surface

A future implementation task is mechanically objective only if it remains
within this local/static plan shape:

- Rp1DmaSmallDiagnosticPlan input is accepted
  DmaCacheDriverDiagnosticEnvelopeEvidence plus retained RP1 DMA controller
  source facts, not raw driver addresses or an arbitrary channel choice.
- Plan evidence records this contract id, the envelope contract id, all
  prerequisite descriptor/sync-plan/maintenance-sequence/executor ids, the
  translated RP1 DMA controller base, source compatible string, channel count,
  target count, source interrupt and clock names, CPU/RP1 buffer addresses,
  descriptor length, cache line coverage, direction, cacheability, owner
  transition, IOMMU classification, unresolved diagnostic gaps, and local/static
  classification.
- Validators reject non-accepted envelope classification, missing prerequisite
  identities, missing rejected completion claims, missing unresolved gaps,
  unsupported cacheability/IOMMU claims, zero channel count, invalid translated
  controller base, and any input that claims RP1 DMA channel ownership,
  descriptor-ring readiness, transfer completion, interrupt completion,
  hardware/device completion, Ethernet readiness, storage readiness,
  networking, or SSH.
- Evidence formatting must preserve serialized hardware-lock requirements and
  future Pi 5 inconclusive-run triage requirements before any task can turn the
  plan into hardware proof.

That future implementation may add focused local/unit tests for accepted plan
construction and deterministic rejection cases. It must not program RP1 MMIO or
DMA channels, allocate or write descriptor rings, run Pi 5 hardware, publish
boot archives, implement Ethernet, storage, networking, SSH, claim driver DMA
completion, claim hardware validation, or accept Milestone 11.3 by implication.

## Hardware and Triage Requirements

No Pi 5 hardware task is mechanically required by this source contract. If a
later supervisor-planned hardware proof becomes necessary, it must:

- acquire hardwareTestLock and serialize with any other Pi 5 run;
- capture candidate identity, serial cursor, TFTP delta, artifact digest,
  classification, and restore evidence;
- run the inconclusive-run triage sequence before changing code after any
  inconclusive run: candidate identity, fresh serial cursor, TFTP delta,
  known-good control, then candidate rerun;
- keep failed hardware boots as evidence, not incidents, unless concrete
  hardware/lab risk or confused evidence requires restore.

## Findings

- fixed: selected phase11-rp1-dma-cache-small-diagnostic-plan-contract-v1 as
  the smallest next source-backed diagnostic boundary after the accepted
  driver-diagnostic envelope.
- fixed: tied the next diagnostic question to retained RP1 AXI DMA controller
  identity instead of jumping directly to Ethernet, storage, networking, SSH,
  or a complete transfer path.
- fixed: required any future plan implementation to consume accepted
  DmaCacheDriverDiagnosticEnvelopeEvidence and preserve the full
  descriptor/sync-plan/maintenance-sequence/executor identity chain.
- fixed: preserved unresolved RP1 channel ownership, descriptor-ring,
  transfer-completion, interrupt-policy, IOMMU/runtime, allocation/pinning,
  hardware-proof, and device-consumer gaps as explicit blockers.
- fixed: recorded serialized hardware-lock and Pi 5 inconclusive-run triage
  requirements before any future hardware proof task.
- deferred: RP1 DMA channel ownership, descriptor-ring layout and ownership,
  transfer completion, interrupt completion, runtime IOMMU policy, DMA-safe
  allocation or pinning expansion, a device-specific diagnostic consumer,
  hardware validation, Ethernet, storage, networking, SSH, Milestone 12 work,
  Milestone 11.3 completion, and phase transition.
- not-an-issue: no Pi 5 hardware run was required because this task accepts
  only a source contract and future validation strategy.

No findings were removed.

## Rejected Claims

This contract does not accept:

- A small DMA diagnostic implementation.
- Driver DMA completion.
- Hardware or device completion.
- RP1 DMA channel ownership or programming.
- RP1 MMIO writes.
- DMA descriptor rings.
- Transfer completion or interrupt completion.
- Ethernet readiness, storage readiness, networking, SSH, or Milestone 12
  progress.
- Cache-coherent, non-cacheable, or IOMMU-backed runtime policy.
- DMA-safe allocation or pinning beyond accepted descriptor validation.
- Pi 5 hardware validation.
- Milestone 11.3 completion by implication.
- Phase transition.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Contract names the smallest next diagnostic question and explains why it is
  not premature Ethernet/storage/networking/SSH work: satisfied.
- Contract consumes accepted envelope evidence as a prerequisite and preserves
  unresolved RP1 channel, descriptor-ring, transfer-completion,
  interrupt-policy, IOMMU/runtime, allocation/pinning, hardware-proof, and
  device-consumer gaps: satisfied.
- Contract includes serialized hardware lock and inconclusive-run triage
  requirements before any future Pi 5 hardware task if hardware proof becomes
  necessary: satisfied.
- Accepted contract is committed or the task is blocked with a precise reason:
  satisfied by the commit recorded in supervisor state after this task.

## Validation

- static inspection: reviewed accepted envelope closeout, accepted envelope
  core, envelope classification JSON, src/dma_cache.rs,
  docs/src/project/phase11-rp1-pcie-map-contract.md, docs/src/roadmap.md, and
  retained RP1 source evidence for rp1_dma.
- jq JSON checks: task-owned classification.json and evidence-map.json passed
  jq empty.
- docs: /home/node/.cargo/bin/mdbook build passed after docs updates.
- diff checks: git diff --check and git diff --cached --check passed.

## Next Action

No explicit queued follow-up task exists. Set planningNeeded=true for
supervisor planning of a bounded
phase11-rp1-dma-cache-small-diagnostic-plan-core task derived from
phase11-rp1-dma-cache-small-diagnostic-plan-contract-v1. That future task must
remain local/static unless explicit supervisor scope authorizes runtime or
hardware work. It must not run hardware, publish boot archives, program RP1 MMIO
or DMA channels, create descriptor rings, implement Ethernet, storage,
networking, SSH, or accept Milestone 11.3 by implication.
