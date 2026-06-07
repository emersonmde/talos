# Phase 11 RP1 Interrupt-Routing Source Reference Notes

Task: `phase11-rp1-interrupt-routing-source-contract-20260607`

Evidence level: static source/doc inspection.

## Retained Sources

All retained Raspberry Pi source files referenced here are already committed in
Talos task evidence:

- `tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/rp1-mfd.h`
- `tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/mfd-rp1.c`
- `tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/pinctrl-rp1.c`
- `tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/rp1.dtsi`
- `tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712.dtsi`
- `docs/src/architecture/interrupts-timers.md`
- `docs/src/project/phase11-rp1-pcie-map-contract.md`
- `docs/src/project/phase11-rp1-irq-clock-gpio-contract.md`

## Source Findings

- RP1 interrupt IDs: `rp1-mfd.h` defines `RP1_INT_IO_BANK0 = 0`,
  `RP1_INT_IO_BANK1 = 1`, `RP1_INT_IO_BANK2 = 2`, `RP1_INT_UART0 = 25`,
  and `RP1_INT_END = 61`.
- RP1 GPIO parent interrupts: `rp1.dtsi` makes `rp1` an
  `interrupt-controller` with `#interrupt-cells = <2>` and declares
  `rp1_gpio` with three level-high parent interrupts for IO_BANK0/1/2.
- RP1 GPIO child event path: `pinctrl-rp1.c` handles a parent bank interrupt
  by reading the bank `INTS` register, clearing each pending pin event through
  `GPIO_CTRL_IRQRESET`, and dispatching the GPIO child IRQ. GPIO event-type
  setup writes GPIO `CTRL` event bits, clears latched events, and GPIO IRQ
  enable/disable writes bank `INTE`.
- RP1 MSI-X path: `mfd-rp1.c` allocates `RP1_IRQS` PCI MSI-X vectors,
  builds a linear RP1 irqdomain, maps each RP1 hwirq to `pci_irq_vector`,
  installs a chained handler on each PCI MSI-X vector, and activates each RP1
  hwirq by setting `MSIX_CFG_ENABLE`.
- RP1 level-high handling: `mfd-rp1.c` sets `MSIX_CFG_IACK_EN` for
  `IRQ_TYPE_LEVEL_HIGH`, records the hwirq as level-triggered, and writes
  `MSIX_CFG_IACK` after the chained handler dispatches the child IRQ.
- RP1 MSI-X config register: `mfd-rp1.c` maps
  `RP1_PCIE_APBS_BASE = 0x108000`, defines `MSIX_CFG(x) = 0x8 + 4 * x`,
  and defines config bits `ENABLE` bit 0, `TEST` bit 1, `IACK` bit 2,
  and `IACK_EN` bit 3.
- Accepted RP1 address translation from Milestone 11.1 remains:
  `cpu_phys = 0x1f00000000 + (rp1_bus - 0xc040000000)`. The RP1 MSI-X config
  register for hwirq 0 is therefore source-translated as
  `0x1f00000000 + 0x108000 + 0x8 = 0x1f00108008`.
- PCIe/GIC-visible route: `bcm2712.dtsi` declares `pcie2` with
  `interrupt-names = "pcie", "msi"`, `msi-parent = <&mip0>`, and the
  controller's own MSI status interrupt as GIC SPI 234. The `mip0` MSI
  controller maps 64 edge-rising MSI entries to GIC SPI 128..191 with
  `brcm,msi-offset = <0>`. Source inspection therefore predicts RP1 hwirq 0
  uses PCI MSI-X vector 0 and MIP0 MSI vector 0, visible at GIC SPI 128 /
  INTID 160 if Talos later enables the full path. That remains an unaccepted
  routing assumption until hardware proof.
- Existing Talos Phase 4 interrupt docs already accept the Pi 5 GIC-400
  distributor/CPU-interface path for EL2 physical timer INTID 26. This RP1
  contract reuses only the GICv2 terminology and hardware-validation discipline;
  it does not accept any RP1 interrupt delivery.

## Selected Diagnostic

```text
contract: phase11-rp1-interrupt-routing-source-contract-v1
target: rp1-io-bank0-msix-cfg-read
source hwirq: RP1_INT_IO_BANK0 = 0
predicted pci msix vector: 0
predicted gic route: MIP0 MSI vector 0 -> GIC SPI 128 / INTID 160
rp1 register: RP1_PCIE_APBS MSIX_CFG(0)
address: 0x1f00108008
width: 32-bit volatile little-endian load
operation: read-only/no-enable
```

Expected reporting fields: contract id, target, hwirq, predicted MSI-X vector,
predicted GIC SPI/INTID, address, width, raw MSI-X config value, decoded
`enable`, `test`, `iack`, and `iack_en` bits, and classification from
the task/contract docs.

## No-MMIO/No-Enable Control Requirement

Before any real Pi 5 interrupt-routing proof, a paired control must be accepted
locally/static and then on Pi 5. The control must branch from the same early
entry point, preserve the same serial/output shape, construct no
`0x1f00108008`, no RP1 GPIO/RIO/PADS/clock/reset/MSI-X address, no PCIe
config/MSI/MIP/GIC address, perform no volatile load or store to those paths,
and emit a simulated/control raw value plus terminal marker suitable for the
later Pi 5 identity join.

## Review Findings

- fixed: first interrupt-routing source contract is source-backed and
  read-only/no-enable.
- fixed: no-MMIO/no-enable control requirements are explicit and block hardware
  proof until accepted.
- deferred: GPIO event programming, INTE writes, IRQRESET writes, MSI-X
  enable/IACK writes, PCIe MSI programming, GIC SPI enablement, interrupt
  unmasking, ISR installation, clock/reset programming, DMA/cache, storage,
  generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3,
  and phase transition.
- not-an-issue: source-predicted GIC SPI 128 / INTID 160 is useful contract
  context, but it remains an unaccepted routing assumption until a later
  hardware proof.
