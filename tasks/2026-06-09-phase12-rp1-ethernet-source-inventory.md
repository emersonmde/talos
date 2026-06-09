# Phase 12 RP1 Ethernet Source Inventory

Task: phase12-rp1-ethernet-source-inventory-20260609

Status: accepted

Evidence level: static inspection of retained Raspberry Pi Linux device-tree
sources, fetched Raspberry Pi Linux rpi-6.12.y source excerpts, accepted Phase
11 closeout evidence, and Talos roadmap/project docs.

## Goal

Build a source-cited inventory of the RP1 Ethernet/GEM path and its
prerequisites before choosing an implementation path.

## Scope

- Consume the accepted Phase 11 closeout because its nextAction explicitly
  selects this source-only Phase 12.1 inventory.
- Inspect retained and fetched Linux/device-tree/source references for
  `rp1_eth`, `raspberrypi,rp1-gem`, `cdns,macb`, RP1 PCIe address space,
  interrupts, clocks/resets, DMA/cache/IOMMU, MDIO/PHY/reset, and buffer
  ownership implications.
- Identify accepted Phase 11 frontiers usable as research prerequisites and
  retained blockers before implementation.
- Record unknown hardware behaviors as future diagnostics, not implementation
  work.
- Record findings with disposition.

## Non-Goals

- No code implementation, hardware run, boot archive publication,
  hardwareTestLock acquisition, RP1 MMIO/DMA programming, Ethernet driver,
  packet TX/RX, network stack, sockets, SSH, descriptor-ring/channel-ownership
  implementation, or Phase 12.2 work.

## Retained Inputs

- docs/src/roadmap.md
- docs/src/project/phase11-rp1-pcie-map-contract.md
- tasks/2026-06-09-phase11-rp1-hardware-substrate-closeout.md
- tasks/evidence/2026-06-09-phase11-rp1-hardware-substrate-closeout/classification.json
- tasks/2026-06-09-phase11-rp1-irq-clock-gpio-milestone-closeout.md
- tasks/2026-06-09-phase11-rp1-dma-cache-milestone-11-3-closeout.md
- tasks/2026-06-09-phase11-rp1-dma-cache-source-inventory.md
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/rp1.dtsi
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi-5-b.dts
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi.dtsi

## Fetched Source Excerpts

Fetched from Raspberry Pi Linux `rpi-6.12.y` for this task and retained under
`tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/`:

- `linux-rpi-6.12-rp1.dtsi`
- `linux-rpi-6.12-bcm2712-rpi-5-b.dts`
- `linux-rpi-6.12-cdns-macb.yaml`
- `linux-rpi-6.12-macb_main.c`
- `linux-rpi-6.12-rp1-mfd.h`
- `linux-rpi-6.12-rp1-clock.h`
- `sha256sums.txt`

## Source Facts

- `rp1.dtsi` defines `rp1_eth: ethernet@100000` with `reg =
  <0xc0 0x40100000 0x0 0x4000>`, compatible strings
  `raspberrypi,rp1-gem` and `cdns,macb`, one `RP1_INT_ETH`
  level-high interrupt, clocks `RP1_CLK_SYS`, `RP1_CLK_SYS`,
  `RP1_CLK_ETH_TSU`, and `RP1_CLK_ETH`, clock names `pclk`, `hclk`,
  `tsu_clk`, and `tx_clk`, `phy-mode = "rgmii-id"`, Cadence pipe/fill
  properties, a zero local MAC placeholder, and default `status =
  "disabled"`.
- `bcm2712-rpi-5-b.dts` enables `&rp1_eth`, assigns `phy-handle = <&phy1>`,
  uses `phy-reset-gpios = <&rp1_gpio 32 GPIO_ACTIVE_LOW>`, sets
  `phy-reset-duration = <5>`, and declares an `ethernet-phy@1` with Broadcom
  powerdown/EEE quirks.
- `bcm2712-rpi.dtsi` aliases `ethernet0 = &rp1_eth`; the overlay parameters
  reference the same PHY for LED modes and maximum-speed configuration.
- Fetched `include/dt-bindings/mfd/rp1.h` defines `RP1_INT_ETH` as 6. Fetched
  `include/dt-bindings/clock/rp1.h` defines `RP1_CLK_SYS` as 12,
  `RP1_CLK_ETH` as 16, and `RP1_CLK_ETH_TSU` as 29.
- `cdns,macb.yaml` lists `raspberrypi,rp1-gem` as a Cadence GEM-compatible
  Ethernet controller and requires compatible, reg, interrupts, clocks,
  clock-names, and phy-mode. It permits `phy-handle` and an optional MDIO
  child node.
- `macb_main.c` is not a small MMIO-only polled path. It includes Linux DMA
  mapping, phylink, OF MDIO, PHY, clock, and interrupt dependencies; it
  allocates TX/RX descriptor rings, writes ring base addresses, configures DMA,
  handles TX/RX completion through NAPI/interrupt paths, adjusts `tx_clk` by
  link speed, and drives MDIO/PHY reset behavior.

## Talos Prerequisite Frontiers

Accepted as usable research prerequisites:

- Phase 11 closed as a source/research substrate frontier and selected this
  exact source-only inventory.
- The source PCIe/RP1 address formula maps RP1 bus `0xc0_40100000` to source
  CPU physical `0x1f_0010_0000` through the pcie2 non-prefetchable window, but
  accepted hardware evidence does not prove broad live RP1 endpoint MMIO for
  Ethernet.
- The accepted PCIe2 host-link proof shows host-link visibility, while
  endpoint config identity and bridge/outbound setup remain retained blockers.
- Milestone 11.2 accepts source-backed interrupt-route documentation and
  serial-captured GPIO/clock/interrupt blockers, but not interrupt delivery,
  clock/reset ownership, GPIO ownership, or event generation.
- Milestone 11.3 accepts documented/local-static DMA/cache ownership,
  address-translation, cache-maintenance, and visibility/control frontiers, but
  not live DMA, descriptor rings, channel ownership, transfer completion, or
  interrupt completion.

Retained implementation blockers:

- RP1 Ethernet requires live RP1 MMIO access to the GEM register block; accepted
  Phase 11 evidence is not broad Ethernet MMIO readiness.
- Packet I/O depends on descriptor-ring layout/ownership, DMA-safe buffers,
  cache maintenance, and interrupt/completion policy that remain unaccepted.
- `phy-reset-gpios = <&rp1_gpio 32 GPIO_ACTIVE_LOW>` depends on RP1 GPIO
  ownership and pin/clock/reset behavior that remain unaccepted.
- `phy-mode = "rgmii-id"` and `tx_clk` rate changes require a clock/reset
  ownership policy before live link bring-up.
- The retained source evidence does not attach `iommus = <&iommu5>` to
  `rp1_eth`; the current Talos classification remains source-unassigned for
  RP1 Ethernet DMA until a later task proves or chooses the runtime policy.

## Future Diagnostics

These are candidate diagnostic questions only; this task does not create or
run them:

- Can Talos read a harmless RP1 GEM identity/status register through a
  freshness-controlled paired no-MMIO/no-Ethernet control after bridge/outbound
  setup blockers are resolved?
- Which exact RP1 clock/reset bits gate `RP1_CLK_ETH`, `RP1_CLK_ETH_TSU`, and
  `tx_clk`, and can they be observed without enabling the MAC?
- Can GPIO32 PHY reset ownership be observed or proven independently of packet
  I/O?
- Does a source-selected MACB register read produce stable non-sentinel output
  before any descriptor-ring, DMA, or interrupt work?
- What DMA address-width and descriptor-ring mode does RP1 GEM expose once
  live MMIO reads are accepted?

## Inventory Classification

Accepted by this inventory:

- Source-backed RP1 Ethernet compatible strings and GEM/Cadence binding shape.
- Source-backed RP1 Ethernet register window, interrupt name/number,
  clock names/ids, PHY mode, PHY reset GPIO, and PHY address/quirk facts.
- Source-backed statement that the Linux MACB/GEM path depends on clocks,
  phylink/MDIO/PHY reset, DMA descriptors, ring base programming, interrupts,
  and packet-buffer ownership.
- Talos prerequisite map from accepted Phase 11 frontiers to retained
  implementation blockers.
- The exact next task is the Phase 12.1 path-selection ADR/design note.

Rejected by this inventory:

- Ethernet driver readiness.
- Live RP1 Ethernet MMIO access.
- Packet TX/RX.
- Descriptor-ring construction or ownership.
- RP1 DMA channel ownership or live DMA.
- Transfer completion or interrupt completion.
- Network stack, sockets, SSH, and Phase 12.2 implementation.
- Treating the source CPU physical address as hardware-proven Ethernet MMIO.

## Findings

- fixed: selected source inventory and path-selection ADR as the first Phase
  12.1 work instead of implementing an Ethernet driver.
- fixed: recorded the RP1 Ethernet compatible strings, source register window,
  interrupt, clocks, PHY mode, PHY reset GPIO, and PHY address facts.
- fixed: tied the Cadence MACB/GEM Linux path to DMA descriptors, packet-buffer
  ownership, interrupts, clocks, MDIO, PHY reset, and phylink dependencies.
- fixed: separated source-backed address calculation from hardware-proven
  broad Ethernet MMIO readiness.
- fixed: carried Phase 11 DMA/cache, interrupt, GPIO, clock/reset, and
  bridge/setup retained risks into the Phase 12 research boundary.
- deferred: path selection between direct Cadence GEM, no_std driver reuse, or
  simpler staged transport remains for the ADR task.
- deferred: all live Ethernet MMIO, DMA, descriptor-ring, interrupt, PHY reset,
  packet I/O, network stack, sockets, and SSH work.
- not-an-issue: the Linux `cdns,macb` driver complexity is useful design input,
  not a requirement that Talos port Linux networking internals wholesale.

No findings were removed.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Inventory cites source/doc evidence for compatible strings, address path,
  interrupt route, clock/reset dependencies, DMA/cache/IOMMU implications,
  MDIO/PHY/reset considerations, and Talos prerequisite frontiers: satisfied.
- Inventory lists unknown hardware behaviors as future diagnostics with
  non-goals: satisfied.
- Inventory rejects Ethernet driver readiness, packet I/O, network stack,
  sockets, SSH, live DMA, descriptor rings, and Phase 12.2 implementation by
  implication: satisfied.
- Accepted inventory is committed: satisfied by the task commit recorded in
  supervisor state.

## Next Action

Mechanically promote phase12-rp1-ethernet-path-adr-20260609 on the next worker
wake. That task should choose the initial Phase 12.1 Ethernet path from this
source inventory. It must not implement Ethernet, run hardware, program RP1
MMIO/DMA, create descriptor rings, perform packet I/O, build networking, open
sockets, add SSH, or start Phase 12.2 work.
