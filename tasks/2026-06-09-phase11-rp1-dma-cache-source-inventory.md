# Phase 11 RP1 DMA/Cache Source Inventory

Task: phase11-rp1-dma-cache-source-inventory-20260609

Status: accepted

Evidence level: static inspection of retained Raspberry Pi Linux device-tree
sources, accepted Phase 11 checkpoint evidence, and Talos memory/cache
architecture docs.

## Goal

Inventory the source-backed RP1 DMA addressability, cache-maintenance,
memory-ownership, and driver-facing contract gaps before any DMA-capable RP1
driver, Ethernet path, networking, SSH, or storage work.

## Scope

- Inspect retained Raspberry Pi Linux source evidence for RP1 DMA
  addressability, dma-ranges, IOMMU/SMMU implications, cache-maintenance
  requirements, physical-memory ownership, and current Talos API gaps.
- Keep the result feature-led by naming the smallest future user-visible
  substrate that would let a DMA-capable RP1 driver be evaluated without
  jumping to networking.
- Record findings with disposition.
- Name the next contract boundary only if the inventory makes it mechanically
  objective.

## Non-Goals

- No runtime source changes, hardware run, boot archive publication,
  hardwareTestLock acquisition, RP1 MMIO writes, DMA engine programming,
  Ethernet, block driver work, cache-maintenance implementation, allocator
  refactor, networking, SSH, Milestone 12 work, or phase transition.

## Retained Inputs

- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md
- docs/src/architecture/memory.md
- docs/src/architecture/scheduler.md
- docs/src/architecture/interrupts-timers.md
- tasks/2026-06-09-phase11-rp1-irq-clock-gpio-milestone-closeout.md
- tasks/evidence/2026-06-09-phase11-rp1-irq-clock-gpio-milestone-closeout/classification.json
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712.dtsi
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi-5-b.dts
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi.dtsi
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/rp1.dtsi

## Source Facts

- Linux bcm2712.dtsi gives BCM2712 pcie2 a 4 MiB 32-bit non-prefetchable DMA
  window at PCIe 00_00000000 to CPU physical 0x1f_0000_0000, plus a 64 GiB
  64-bit prefetchable RAM-facing DMA window at PCIe 10_00000000 to CPU
  physical 0x0, plus a 4 KiB MIP0 window at PCIe ff_ffff_f000.
- Linux bcm2712-rpi-5-b.dts maps the RP1 child bus so RP1
  0xc0_40000000..0xc0_4040ffff reaches PCIe 00_00000000, and its dma-ranges
  say RP1 inbound 0x10_00000000 and 0x0_00000000 target the PCIe
  10_00000000 RAM window while RP1 inbound 0xc0_40000000 targets the RP1
  peripheral window.
- Linux rp1.dtsi exposes rp1_dma as compatible = "snps,axi-dma-1.01a" at RP1
  bus 0xc0_40188000, with 8 channels, one master, 64 targets, 128-bit data
  width, per-channel block size 0x40000, and AXI burst limits.
- The source names DMA consumers before networking: SPI, PIO, audio/I2S, and
  optional UART DMA aliases. RP1 Ethernet is a Cadence GEM-compatible device,
  but the retained Ethernet node does not itself provide a shortcut around the
  RP1 DMA/cache substrate.
- Linux bcm2712-rpi-5-b.dts adds iommus = <&iommu5> to selected display and
  camera RP1 bus masters (csi0, csi1, dsi0, dsi1, dpi, vec), but not to
  rp1_dma or rp1_eth in the retained source evidence.
- Linux bootargs include coherent_pool=1M; this is Linux policy context only.
  It does not prove Talos has coherent DMA buffers, cache maintenance, or an
  IOMMU contract.
- Talos architecture currently accepts only the low identity-mapped allocation
  span 0x2f010000..0x3fc00000 for ordinary early kernel allocation. High
  memory and any future DMA-safe buffers remain unowned.
- Talos has narrow cache maintenance only for secondary-core publication in
  src/smp.rs (dc cvac and dc ivac with dsb sy). That boundary is not a driver
  DMA API and does not cover arbitrary buffer ranges, ownership, direction,
  alignment, or lifetime.

## Inventory Classification

Accepted by this inventory:

- Source-backed RP1 DMA addressability facts from retained device-tree
  dma-ranges.
- Source-backed RP1 DMA controller identity and basic channel/target shape.
- Source-backed statement that some RP1 bus masters are explicitly tied to
  iommu5, while rp1_dma and rp1_eth are not in the retained evidence.
- Source-backed requirement that DMA/cache work must precede DMA-capable RP1
  drivers, Ethernet, networking, SSH, and storage.
- Talos gap inventory for DMA-safe buffer ownership, address translation,
  cache clean/invalidate semantics, and driver evidence fields.

Rejected by this inventory:

- Working DMA behavior.
- DMA engine programming.
- RP1 Ethernet readiness.
- Networking, SSH, storage, or Milestone 12 progress.
- Cache-coherent driver policy.
- IOMMU programming or bypass policy.
- Allocation of high memory or DMA-safe buffers.
- Use of existing SMP cache-line helpers as a general DMA API.

## Findings

- fixed: selected a source-backed DMA/cache inventory rather than starting
  networking, Ethernet, block storage, or DMA engine implementation.
- fixed: named the RP1 dma-ranges translation facts that make addressability a
  contract problem, not a simple fixed-MMIO assumption.
- fixed: separated rp1_dma controller facts from RP1 Ethernet facts so the next
  task can target the substrate rather than a network driver.
- fixed: identified that retained source evidence attaches iommu5 to selected
  display/camera masters, but not to rp1_dma or rp1_eth.
- fixed: mapped current Talos gaps to concrete contract fields: buffer
  ownership, address translation, cache clean/invalidate direction, alignment,
  memory barriers, and evidence output.
- deferred: implementation of DMA-safe buffers, cache maintenance APIs,
  IOMMU/SMMU behavior, DMA programming, RP1 Ethernet, storage, networking, and
  SSH.
- not-an-issue: Linux coherent_pool=1M is useful source context, but it is not
  Talos acceptance evidence for coherent memory.

No findings were removed.

## Smallest Future Substrate

The thinnest real feature path is a local/static DMA/cache substrate contract
that can later support a driver-adjacent diagnostic without touching hardware:

- a kernel-owned DMA buffer descriptor with explicit physical address,
  CPU-visible virtual/identity address, length, alignment, cacheability class,
  and lifetime owner;
- an RP1-facing bus address translation record derived from the accepted
  dma-ranges, including which path is used for RAM-facing buffers versus RP1
  peripheral-facing addresses;
- direction-specific cache maintenance requirements for CPU-to-device,
  device-to-CPU, and bidirectional buffers;
- evidence fields that prove no networking/SSH/storage driver can consume the
  contract until the substrate has explicit tests and later hardware evidence.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Inventory names source-backed RP1 DMA addressability, cache-maintenance,
  memory-ownership, and driver-facing contract gaps: satisfied.
- Inventory rejects networking/SSH progress until DMA/cache prerequisites are
  explicitly accepted: satisfied.
- NextAction selects a concrete contract task: satisfied.
- Accepted inventory is committed: satisfied by the task commit recorded in
  supervisor state.

## Next Action

Mechanically promote phase11-rp1-dma-cache-contract-20260609 on the next
worker wake. That task should define the minimal local/static DMA-safe buffer,
address-translation, cache-maintenance, and driver evidence contract. It must
not program DMA, run hardware, start Ethernet/storage/networking/SSH work, or
claim Milestone 11.3 acceptance by implication.
