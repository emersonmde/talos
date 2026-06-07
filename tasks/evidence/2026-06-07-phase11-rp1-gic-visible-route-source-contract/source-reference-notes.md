# Phase 11 RP1 GIC-Visible Route Source Reference Notes

Task: `phase11-rp1-gic-visible-route-source-contract-20260607`

Evidence level: static source/doc inspection.

## Retained Sources

All retained source files referenced here are already committed in Talos task
evidence or source:

- `docs/src/architecture/interrupts-timers.md`
- `src/arch/aarch64/gicv2.rs`
- `src/target/rpi5.rs`
- `docs/src/project/phase11-rp1-pcie-map-contract.md`
- `tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-source-contract/source-reference-notes.md`
- `tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-diagnostic-closeout/evidence-map.json`
- `tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/rp1-mfd.h`
- `tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/mfd-rp1.c`
- `tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712.dtsi`

## Source Findings

- Pi 5 GIC facts: Phase 4 accepts the Pi 5 GIC-400/GICv2 distributor base
  `0x10_7fff_9000` and CPU interface base `0x10_7fff_a000`.
- Talos GICv2 helper facts: `src/arch/aarch64/gicv2.rs` names
  `GICD_ISENABLER` offset `0x100`, `GICD_ISPENDR` offset `0x200`,
  `GICD_ISACTIVER` offset `0x300`, and `GICC_HPPIR` offset `0x18`.
  Existing helper methods read those status banks and highest-pending value
  without acknowledging an interrupt.
- Accepted RP1 route facts: the prior MSIX_CFG(0) source contract and
  closeout retain `RP1_INT_IO_BANK0 = 0`, Linux RP1 irqdomain/MSI-X
  behavior, and BCM2712 `pcie2` using `mip0` as its MSI parent. Source
  inspection predicts RP1 hwirq 0 through PCI MSI-X vector 0 and MIP0 MSI
  vector 0 to GIC SPI 128 / INTID 160.
- INTID arithmetic: GIC SPI 128 is hardware INTID 160. For GICv2 distributor
  banks, `bank = intid / 32 = 5`, `bank_offset = bank * 4 = 0x14`, and
  `bit = 1 << (intid & 31) = 0x00000001`.
- Selected register addresses:
  - `GICD_ISENABLER5`: `0x10_7fff_9000 + 0x100 + 0x14 = 0x10_7fff_9114`.
  - `GICD_ISPENDR5`: `0x10_7fff_9000 + 0x200 + 0x14 = 0x10_7fff_9214`.
  - `GICD_ISACTIVER5`: `0x10_7fff_9000 + 0x300 + 0x14 = 0x10_7fff_9314`.
  - `GICC_HPPIR`: `0x10_7fff_a000 + 0x18 = 0x10_7fff_a018`.
- Operation boundary: these are read-only/no-ack observations. The contract
  forbids `GICC_IAR`, `GICC_EOIR`, GIC enable/configuration writes,
  interrupt unmasking, ISR installation, and all RP1/MSI-X/PCIe/MIP/GPIO/pads
  or clock/reset MMIO.

## Selected Diagnostic

```text
contract: phase11-rp1-gic-visible-route-source-contract-v1
target: rp1-io-bank0-gic-route-status-read
source hwirq: RP1_INT_IO_BANK0 = 0
predicted pci msix vector: 0
predicted gic route: MIP0 MSI vector 0 -> GIC SPI 128 / INTID 160
gic distributor base: 0x10_7fff_9000
gic cpu interface base: 0x10_7fff_a000
allowed reads:
  GICD_ISENABLER5 @ 0x10_7fff_9114
  GICD_ISPENDR5 @ 0x10_7fff_9214
  GICD_ISACTIVER5 @ 0x10_7fff_9314
  GICC_HPPIR @ 0x10_7fff_a018
operation: read-only/no-ack
```

Expected reporting fields: contract id, target, hwirq, predicted MSI-X vector,
predicted GIC SPI/INTID, GICD/GICC bases, bank, bit mask, selected register
addresses, raw enable/pending/active bank values, decoded INTID 160
enable/pending/active bits, raw `GICC_HPPIR`, decoded HPPIR INTID,
`hppir-spurious`, `hppir-target-match`, and classification from the
task/contract docs.

## No-MMIO/No-GIC/No-RP1 Control Requirement

Before any real Pi 5 GIC-visible route proof, a paired control must be
accepted locally/static and then on Pi 5. The control must branch from the same
early entry point, preserve the same serial/output shape, construct no
GICD/GICC/RP1/MSI-X/PCIe/MIP/GPIO/pads/RIO/clock/reset MMIO address, perform
no volatile load or store to those paths, and emit simulated zero raw values
plus a terminal marker suitable for the later Pi 5 identity join.

## Review Findings

- fixed: source-backed GIC-visible route status contract selects one exact
  read-only/no-ack diagnostic shape.
- fixed: exact Pi 5 GIC bases, INTID bank/bit math, register offsets, and
  resulting CPU physical addresses are explicit.
- fixed: no-MMIO/no-GIC/no-RP1 control requirements are explicit and block
  hardware proof until accepted.
- deferred: GIC enable writes, IAR/EOIR acknowledgement, interrupt unmasking,
  ISR/handler ownership, RP1 event programming, MSI-X enable/IACK writes,
  GPIO ownership, pin-control or pad writes, clock/reset programming,
  DMA/cache, storage, generated-root, networking, SSH, broader PCIe
  enumeration, Milestone 11.3, and phase transition.
- not-an-issue: reading `GICC_HPPIR` is an ack-free status observation, not
  proof of interrupt delivery or handler ownership.
