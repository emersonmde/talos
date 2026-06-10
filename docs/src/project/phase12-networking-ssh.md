# Phase 12 Networking and SSH

Phase 12 starts with source-only RP1 Ethernet research. The accepted Phase 11
frontier allows source inventory and design selection, but it does not accept
Ethernet implementation, packet I/O, live DMA, networking, sockets, or SSH.

## RP1 Ethernet Source Inventory

phase12-rp1-ethernet-source-inventory-20260609 accepts the source-backed RP1
Ethernet inventory for Milestone 12.1. Retained and fetched Raspberry Pi Linux
sources identify `rp1_eth` as `raspberrypi,rp1-gem` / `cdns,macb` at RP1 bus
`0xc0_40100000`, with source CPU physical translation `0x1f_0010_0000`,
`RP1_INT_ETH`, clocks `RP1_CLK_SYS`, `RP1_CLK_ETH_TSU`, and `RP1_CLK_ETH`,
`phy-mode = "rgmii-id"`, and Pi 5 PHY reset through RP1 GPIO32.

The inventory also accepts that the Linux Cadence MACB/GEM path depends on DMA
descriptor rings, packet buffers, MDIO/PHY reset, phylink, clocks, interrupts,
and completion handling. Talos keeps those as prerequisites, not accepted
runtime behavior.

## Retained Boundaries

The Phase 12.1 inventory does not accept broad RP1 Ethernet MMIO readiness,
descriptor-ring ownership, RP1 DMA channel ownership, transfer completion,
interrupt completion, GPIO32 PHY reset ownership, clock/reset ownership, packet
I/O, network stack behavior, sockets, SSH, or Phase 12.2 work. The next
bounded step is a path-selection ADR/design note before any implementation.

## Path Selection ADR

phase12-rp1-ethernet-path-adr-20260609 selects the direct RP1 Cadence GEM path
as the Phase 12.1 target, but only after staged hardware-substrate proofs. The
decision rejects immediate driver work because the accepted Linux MACB/GEM
source path depends on unaccepted Ethernet MMIO, DMA descriptors and packet
buffers, ring-base programming, MDIO/PHY reset, clocks, interrupts, and
completion handling.

The ADR defers no_std driver reuse until Talos proves the required hardware and
driver substrate boundaries. It also defers a simpler non-RP1 transport because
that would not retire the explicit RP1 Ethernet research unknowns. The next
program step requires supervisor planning for a bounded read-only RP1
GEM/Ethernet MMIO source-contract diagnostic with paired no-Ethernet/no-MMIO
control evidence. No Ethernet implementation, packet I/O, network stack,
sockets, SSH, live DMA, descriptor rings, or Phase 12.2 work is accepted by
the ADR.

## GEM MID Source Contract

phase12-rp1-ethernet-gem-mid-source-contract-20260609 accepts the source-only
contract for the first bounded GEM visibility diagnostic. The exact future
candidate target is MACB_MID at offset 0x00fc from rp1_eth, producing source
RP1 bus address 0xc0_401000fc and source-translated CPU physical address
0x1f001000fc. The contract is limited to a 32-bit little-endian volatile load
in a future diagnostic, paired with a no-Ethernet/no-MMIO control report that
withholds the Ethernet MMIO target.

Retained Raspberry Pi Linux source evidence defines the rp1_eth base at
0xc0_40100000, MACB_MID at 0x00fc, and the IDNUM and REV fields inside MID.
This remains source evidence only: live broad RP1 Ethernet MMIO readiness, RP1
MMIO writes, DMA, descriptor rings, interrupts, clock/reset ownership, PHY
reset ownership, packet I/O, networking, sockets, SSH, and Phase 12.2 remain
unaccepted.

## GEM MID Diagnostic Core

phase12-rp1-ethernet-gem-mid-diagnostic-core-20260609 accepts the local/static
report construction for the GEM MID source contract. The candidate report
preserves the accepted rp1_eth/MACB_MID target, source evidence, rejected
claims, retained risks, and hardware-proof boundary classification. The paired
control uses the same report contract while withholding Ethernet MMIO target
fields and carrying no-ethernet-no-mmio-rp1-ethernet-gem-mid-control.

The diagnostic core is not a hardware proof. Live GEM visibility, broad
Ethernet MMIO readiness, RP1 MMIO/DMA programming, descriptors, interrupts,
clock/reset ownership, PHY reset ownership, packet I/O, networking, sockets,
SSH, and Phase 12.2 remain unaccepted until later explicit tasks.
