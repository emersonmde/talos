# Phase 11 RP1 DMA/Cache Substrate Contract

Task: phase11-rp1-dma-cache-contract-20260609

Status: accepted

Evidence level: static inspection of the accepted RP1 DMA/cache source
inventory, retained Raspberry Pi Linux source evidence, and current Talos
memory/cache architecture docs.

## Goal

Define the minimal local/static DMA/cache substrate contract needed before any
RP1 DMA-capable driver, Ethernet path, block storage, networking, SSH, or
hardware DMA diagnostic work.

## Scope

- Reconcile the accepted DMA/cache source inventory with Talos memory and cache
  ownership boundaries.
- Name exact ownership boundaries, API surface, evidence output, accepted
  assumptions, and rejected claims for DMA/cache substrate work.
- Keep the contract feature-led: it should enable a later driver-adjacent
  diagnostic without starting networking or storage.
- Record findings with disposition.

## Non-Goals

- No runtime source changes, hardware run, boot archive publication,
  hardwareTestLock acquisition, RP1 MMIO writes, DMA engine programming,
  Ethernet, block driver work, cache-maintenance implementation, allocator
  refactor, networking, SSH, Milestone 12 work, or Milestone 11.3 acceptance by
  implication.

## Retained Inputs

- tasks/2026-06-09-phase11-rp1-dma-cache-source-inventory.md
- tasks/evidence/2026-06-09-phase11-rp1-dma-cache-source-inventory/classification.json
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md
- docs/src/architecture/memory.md
- docs/src/architecture/scheduler.md
- docs/src/project/phase6-smp-safe-primitives-source-inventory.md
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712.dtsi
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi-5-b.dts
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi.dtsi
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/rp1.dtsi

## Accepted Assumptions

- The first substrate implementation is local/static. It may add pure data
  structures, address-translation helpers, cache-maintenance contract enums,
  and focused unit tests, but it must not issue RP1 MMIO, DMA descriptor writes,
  cache maintenance instructions for driver buffers, or hardware boots.
- The only ordinary allocation span accepted today is the low-tail
  bootstrap-bump-owned span described in docs/src/architecture/memory.md.
  Treating a buffer in that span as DMA-safe requires a new explicit
  descriptor, alignment, lifetime, cacheability, and ownership proof.
- Source dma-ranges are the accepted basis for RP1-facing address translation:
  RAM-facing buffers use the retained RP1 inbound-to-PCIe RAM window facts, and
  RP1 peripheral-facing addresses use the retained RP1 peripheral window facts.
- The current SMP cache-line helpers are source evidence for required
  instruction shape and ordering only. They are not a driver DMA API and must
  not be hidden behind a generic lock or allocator boundary.
- Retained source evidence attaches iommu5 to selected display/camera masters,
  not to rp1_dma or rp1_eth. The first substrate must record IOMMU domain as
  absent/source-unassigned for rp1_dma-derived buffers unless a later task
  accepts a different source.

## Ownership Boundaries

- Buffer owner: exactly one kernel owner owns a DMA buffer descriptor at a time.
  The descriptor must record whether ownership is CPU-owned, device-owned, or
  shared only across an explicit synchronization boundary.
- Memory owner: a descriptor may describe only memory from an already accepted
  kernel-owned physical span. High-memory banks and firmware/reserved ranges
  are out of contract until separately mapped and owned.
- Address owner: CPU physical address, CPU visible address, RP1 bus address,
  length, alignment, and source dma-ranges path are explicit fields. A driver
  must not recompute or infer RP1 bus addresses ad hoc.
- Cache owner: cache state transitions are named by direction and boundary:
  CPU-to-device clean before device ownership, device-to-CPU invalidate after
  device ownership, and bidirectional clean+invalidate around shared transfer
  windows.
- Driver owner: a future RP1 driver receives only an already-prepared
  descriptor plus evidence fields. It does not allocate arbitrary kernel memory
  and declare it DMA-safe.

## Minimal API Surface

The next implementation task is mechanically objective if it stays within this
local/static shape:

- DmaDirection: ToDevice, FromDevice, Bidirectional.
- DmaCacheability: at least CacheableRequiresMaintenance; non-cacheable or
  coherent variants must remain unaccepted unless a later task supplies mapping
  and hardware evidence.
- DmaAddressPath: Rp1RamWindow and Rp1PeripheralWindow, each carrying the
  retained source range identity used for translation.
- DmaBufferDescriptor: CPU physical address, CPU visible address, RP1 bus
  address, length, alignment, direction, cacheability, lifetime owner, and
  IOMMU domain classification.
- Pure validators for alignment, range containment in an accepted owned memory
  span, address translation overflow, and forbidden high-memory/reserved-memory
  inputs.
- Evidence formatter fields that print or serialize the contract id, selected
  address path, CPU/RP1 addresses, length, alignment, direction, cacheability,
  IOMMU classification, and rejected runtime claims.

The implementation may include focused unit tests and static/archive checks for
those pure functions. It must not add DMA descriptor rings, RP1 DMA channel
programming, interrupt handling, cache-maintenance execution for driver buffers,
or a network/storage consumer.

## Evidence Output Contract

A future local/static core should emit or retain evidence with these fields:

- contract: phase11-rp1-dma-cache-substrate-contract-v1;
- source inventory: phase11-rp1-dma-cache-source-inventory-20260609;
- buffer: CPU physical, CPU visible, RP1 bus, length, alignment;
- address path: rp1-ram-window or rp1-peripheral-window;
- direction and cacheability;
- IOMMU classification: source-unassigned-rp1-dma unless later evidence
  changes this;
- validation: alignment, ownership-span containment, translation-range check,
  and forbidden-claim checks;
- classification: local-static-dma-cache-contract-visible,
  contract-rejected-input, or staging/build-blocker.

This evidence is local/static only. It is not hardware validation.

## Contract Classification

Accepted by this contract:

- A local/static DMA buffer descriptor boundary.
- RP1-facing address-translation contract fields derived from retained
  dma-ranges source evidence.
- Direction-specific cache-maintenance semantics as required driver contract
  fields.
- IOMMU classification as explicit evidence rather than an implicit policy.
- Testable validator and evidence-output requirements for the next
  implementation task.

Rejected by this contract:

- Working DMA behavior.
- DMA engine programming or descriptor-ring ownership.
- Executed cache maintenance for driver buffers.
- Cache-coherent, non-cacheable, or IOMMU-backed driver policy.
- DMA-safe allocation or pinning beyond pure descriptor validation.
- RP1 Ethernet readiness, storage readiness, networking, SSH, or Milestone 12
  progress.
- Hardware validation or Milestone 11.3 acceptance by implication.

## Findings

- fixed: converted the source inventory into explicit ownership boundaries for
  buffer, memory, address, cache, and future driver consumers.
- fixed: defined a minimal pure API surface that can be implemented and tested
  without RP1 MMIO, DMA programming, cache-maintenance execution, hardware
  boots, networking, or storage.
- fixed: required RP1 bus address translation to carry the retained source
  dma-ranges path instead of becoming an ad hoc arithmetic helper.
- fixed: made cache-maintenance direction and cacheability explicit contract
  fields while keeping existing SMP cache helpers out of the driver API.
- fixed: required IOMMU classification evidence so source-unassigned rp1_dma
  does not silently become an IOMMU bypass or coherent policy.
- deferred: DMA-safe allocator ownership, cache-maintenance implementation,
  DMA channel programming, interrupt-backed completion, IOMMU policy,
  Ethernet, storage, networking, SSH, and hardware validation.
- not-an-issue: keeping the first substrate local/static is consistent with the
  feature-led path because it defines the smallest durable interface required
  before a driver-adjacent DMA diagnostic can be meaningful.

No findings were removed.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Contract names exact ownership boundaries, API surface, evidence output,
  accepted assumptions, and rejected claims: satisfied.
- Contract explicitly blocks networking/SSH and real DMA device work until the
  substrate is implemented and validated: satisfied.
- NextAction identifies an implementation task only if the contract makes it
  mechanically objective; otherwise planningNeeded=true: satisfied by selecting
  a concrete local/static substrate core boundary while requesting supervisor
  planning to instantiate the next queued task.
- Accepted contract is committed: satisfied by the task commit recorded in
  supervisor state.

## Next Action

Set planningNeeded=true for supervisor planning. The contract makes the next
implementation boundary objective: a local/static
phase11-rp1-dma-cache-substrate-core task that adds only pure descriptor,
translation, cache-direction, IOMMU-classification, validator, evidence
formatter, and unit-test surfaces. The worker must not create that task itself.
That follow-up must not program DMA, run hardware, execute cache maintenance
for driver buffers, start Ethernet/storage/networking/SSH work, or accept
Milestone 11.3 by implication.
