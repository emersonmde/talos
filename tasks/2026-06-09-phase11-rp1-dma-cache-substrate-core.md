# Phase 11 RP1 DMA/Cache Substrate Core

Task: phase11-rp1-dma-cache-substrate-core-20260609

Status: accepted

Evidence level: static inspection, local/static implementation review,
fmt/lint gate, unit tests, JSON checks, documentation build, and git diff
checks.

## Goal

Implement the smallest local/static DMA/cache substrate core accepted by the
contract before any RP1 DMA-capable driver, Ethernet, storage, networking, SSH,
or hardware DMA diagnostic work.

## Scope

- Add pure descriptor, direction, cacheability, address-path, ownership, and
  IOMMU-classification vocabulary for the accepted DMA/cache contract.
- Add pure validators for descriptor alignment, zero/overflow length rejection,
  accepted owned-memory span containment, RP1 address translation
  range/overflow checks, and forbidden high-memory, reserved-memory,
  cacheability, and IOMMU claims.
- Add an evidence struct/formatter surface carrying contract/source ids,
  address path, CPU/RP1 addresses, length, alignment, direction, cacheability,
  IOMMU classification, validation results, and local/static classification.
- Add focused unit tests for valid and rejected descriptor inputs.

## Non-Goals

- No hardware run, boot archive publication, hardwareTestLock acquisition,
  runtime cache maintenance for driver buffers, RP1 MMIO writes, DMA engine
  programming, DMA descriptor rings, Ethernet, storage, networking, SSH,
  Milestone 12 work, or Milestone 11.3 acceptance by implication.

## Implementation

- Added src/dma_cache.rs and registered it in src/main.rs.
- Exposed the accepted vocabulary:
  DmaDirection, DmaCacheability, DmaAddressPath, DmaBufferDescriptor,
  DmaBufferOwner, and DmaIommuClassification.
- Implemented validate_dma_buffer_descriptor for the accepted
  bootstrap-bump-owned low-tail span boundary. The validator rejects:
  zero-length buffers, invalid alignment, unaligned CPU address/length,
  address overflow, span escape, high-memory inputs, pre-owned-span reserved
  inputs, translation range/overflow failures, coherent/non-cacheable claims,
  and unsupported IOMMU claims.
- Implemented translate_rp1_bus_address for the source-backed RP1 RAM window
  and RP1 peripheral window.
- Implemented dma_descriptor_evidence and rejected_dma_input_evidence for
  local/static evidence classification.

## Findings

- fixed: implemented the accepted local/static DMA/cache vocabulary without
  adding runtime DMA, RP1 MMIO, cache-maintenance execution, or a driver
  consumer.
- fixed: tied RP1 bus-address evidence to named source dma-ranges paths rather
  than ad hoc driver arithmetic.
- fixed: made the accepted low-tail owned span an explicit validator input so
  high memory and reserved/bootstrap ranges cannot be silently treated as
  DMA-safe.
- fixed: encoded coherent, non-cacheable, and IOMMU-backed claims as rejected
  vocabulary instead of omitting those future policy names.
- fixed: added focused tests for a valid RP1 RAM-window descriptor and rejected
  alignment, ownership-span, high-memory, reserved-memory, translation, and
  forbidden-claim inputs.
- deferred: DMA-safe allocation/pinning, cache-maintenance instruction
  execution for driver buffers, DMA engine programming, descriptor rings,
  IOMMU policy, Ethernet, storage, networking, SSH, hardware validation, and
  Milestone 11.3 completion.
- not-an-issue: no Pi 5 hardware run was required because this task's accepted
  boundary is pure local/static implementation and test evidence.

No findings were removed.

## Validation

- static inspection: reviewed accepted contract and touched source modules.
- fmt/lint/typecheck: cargo fmt --all -- --check passed after applying rustfmt
  formatting.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- jq JSON checks: task-owned classification.json and evidence-map.json passed
  jq empty.
- docs: /home/node/.cargo/bin/mdbook build passed after docs updates.
- diff checks: git diff --check and git diff --cached --check passed.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Implementation exposes the accepted descriptor/cache/address/IOMMU vocabulary:
  satisfied.
- Pure validators cover alignment, zero/overflow length, owned-span
  containment, RP1 translation range/overflow, and forbidden high/reserved
  inputs: satisfied.
- Evidence formatter emits contract id, source inventory id, address path,
  CPU/RP1 addresses, length, alignment, direction, cacheability, IOMMU
  classification, validation results, and local/static classification:
  satisfied.
- Focused tests prove a valid RP1 RAM-window descriptor and rejected invalid
  inputs for alignment, ownership span, translation overflow, and forbidden
  claims: satisfied.
- No runtime path executes cache maintenance for driver buffers, programs
  DMA/RP1 MMIO, or creates Ethernet/storage/networking/SSH behavior:
  satisfied by static inspection and local/static implementation shape.
- Accepted implementation and evidence are committed before the closeout
  checkpoint starts: satisfied by the task commit recorded in supervisor state.

## Next Action

Mechanically promote phase11-rp1-dma-cache-substrate-closeout-20260609 on the
next worker wake. That checkpoint must reconcile only the accepted
local/static substrate core and must not accept working DMA, executed cache
maintenance, coherent/non-cacheable/IOMMU policy, DMA-safe allocation beyond
descriptor validation, Ethernet, storage, networking, SSH, or Milestone 11.3 by
implication.
