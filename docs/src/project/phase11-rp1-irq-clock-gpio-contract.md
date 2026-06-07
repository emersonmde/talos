# Phase 11 RP1 IRQ/Clock/GPIO Contract

This contract defines the first Milestone 11.2 slice after the accepted Milestone 11.1 RP1 UART0 FR mapping frontier. It is a source-backed contract for one narrow read-only GPIO status diagnostic and its no-MMIO control. It does not claim GPIO ownership, pin-control writes, interrupt delivery, clock/reset programming, DMA/cache policy, storage, generated-root, networking, SSH, or broader PCIe enumeration.

## Inputs

- Accepted Milestone 11.1 RP1 mapping frontier: `phase11-rp1-mapping-frontier-checkpoint-20260607`.
- Raspberry Pi Linux `rpi-6.12.y` device-tree sources retained under `tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/`.
- Raspberry Pi Linux `rpi-6.12.y` RP1 MFD, clock, and pinctrl sources retained under `tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/`.

## Source Contract

The accepted Milestone 11.1 translation remains:

```text
cpu_phys = 0x1f_0000_0000 + (rp1_bus - 0xc0_4000_0000)
```

Relevant translated RP1 objects for this slice:

| RP1 object | RP1 bus address | CPU physical address | Width | Use |
| --- | ---: | ---: | ---: | --- |
| RP1 IO_BANK0 base | `0xc0_400d_0000` | `0x1f_000d_0000` | 32-bit MMIO | Source-backed GPIO status block |
| RP1 GPIO14 STATUS | `0xc0_400d_0070` | `0x1f_000d_0070` | 32-bit read | First Milestone 11.2 diagnostic target |
| RP1 RIO bank0 IN | `0xc0_400e_0008` | `0x1f_000e_0008` | 32-bit read | Documented only |
| RP1 GPIO14 pad control | `0xc0_400f_003c` | `0x1f_000f_003c` | 32-bit MMIO | Documented only; do not touch |
| RP1 clocks block | `0xc0_4001_8000` | `0x1f_0001_8000` | 32-bit MMIO | Documented only; do not touch |

`pinctrl-rp1.c` defines `RP1_GPIO_STATUS = 0x0000` and derives each bank0 pin register pair as `IO_BANK0 + pin * 8`. GPIO14 therefore has STATUS at `0x0d0000 + 14 * 8 = 0x0d0070`. The same source defines status bits for raw and filtered falling/rising/low/high state.

## Interrupt And Clock Boundary

`rp1.dtsi` makes the RP1 node an interrupt controller and gives `rp1_gpio` three level-high interrupts for `RP1_INT_IO_BANK0`, `RP1_INT_IO_BANK1`, and `RP1_INT_IO_BANK2`. `mfd-rp1.c` maps RP1 hwirqs through PCI MSI-X vectors and uses MSI-X config bits for enable and IACK. `pinctrl-rp1.c` enables GPIO events through GPIO `CTRL` bits and `INTE` writes.

The first diagnostic must not enable any of that path. Interrupt delivery is source-documented here only.

`rp1.dtsi` declares the RP1 clocks block and UART0 clock dependencies, while `clk-rp1.c` names the UART clock control registers. This first GPIO status read does not program clocks or resets. It depends on the same firmware/RP1 PCIe initialization accepted for Milestone 11.1; a fault or missing output must be classified as proof evidence, not expanded in place into clock/reset writes.

## Diagnostic Contract

Contract id: `phase11-rp1-irq-clock-gpio-contract-v1`.

```text
name: rp1-gpio14-status-read
address: 0x1f_000d_0070
width: 32-bit volatile little-endian load
operation: read-only
source target: RP1 IO_BANK0 GPIO14 STATUS
expected success class: diagnostic-result-visible
```

The diagnostic should report contract id, target, address, width, raw status value, interpreted raw and filtered falling/rising/low/high bits, and classification: `diagnostic-result-visible`, `bus-fault-or-trap-visible`, `candidate-fetch-without-diagnostic-marker`, `capture-staging-blocked`, or `staging/build-blocker`.

## No-MMIO Control

Before any real Pi 5 proof for this diagnostic, the paired no-MMIO control must be accepted locally/static and then on Pi 5. The control must preserve the same serial/output shape while constructing no RP1 GPIO, RIO, pads, clock, reset, or MSI-X address and performing no RP1 volatile load or store.

Accepted by phase11-rp1-gpio-status-no-mmio-control-pi5-20260607: the no-MMIO control output shape is visible on Pi 5. The accepted rerun fetched the selected 46,160-byte da591740/kernel_2712.img, passed the pi5-capture-transaction-v2 identity join with no rejection reasons, retained repeated TALOS: gpio14-status-control output, and restored the pre-run boot tree.

This proof accepts only control output-shape visibility and proof-chain readiness. It does not accept real RP1 GPIO/status, interrupt, clock/reset, or broader hardware behavior.

## Real Diagnostic Proof

phase11-rp1-gpio-status-diagnostic-pi5-20260607 is accepted as
capture-staging-blocked. The real diagnostic archive matched the accepted
local/static SHA-256
7bc21b39a5d0150221a244701285d733c8faef4e153085a49a34b5069c1fecea, and lab
publication selected boot tree
cb7827b07a3822370fc610dfd18a8ab580cea31a47c4559e41a242975976f83a with a
46,336-byte da591740/kernel_2712.img. A marker-visible candidate run retained
483 TALOS: gpio14-status-result occurrences, but the pi5-capture-transaction-v2
identity join rejected it due to non-empty pre-power serial drain, expected
TFTP byte mismatch, final selected-tree mismatch, and final expected-fetch byte
mismatch. The required known-good control and candidate rerun were retained.
The final candidate rerun had empty pre-power serial drain and final
selected-tree identity, but no candidate-tied TFTP fetch and no diagnostic
marker, then restored the lab to the pre-run boot tree.

This result accepts only the blocker classification for the first real GPIO14
STATUS proof attempt. Same-shaped GPIO14 STATUS hardware reruns require
supervisor planning for a different discriminator or capture/staging repair.

## Accepted Boundary

Accepted by `phase11-rp1-irq-clock-gpio-source-contract-20260607`: source-backed GPIO14 status target and address translation, source-backed interrupt path notes for later work, source-backed clock/reset assumptions for this read-only slice, and no-MMIO control requirement before real hardware proof.

Accepted by phase11-rp1-gpio-status-no-mmio-control-pi5-20260607: Pi 5 visibility of the paired no-MMIO GPIO14 status control output shape and proof-chain readiness for the real diagnostic proof.

Accepted by phase11-rp1-gpio-status-diagnostic-pi5-20260607 and
phase11-rp1-irq-clock-gpio-diagnostic-closeout-20260607: the first real Pi 5
GPIO14 STATUS proof attempt is capture-staging-blocked with retained
known-good-control, candidate-rerun, and restore evidence.

Unaccepted: real RP1 GPIO14 STATUS read behavior, bus-fault/trap behavior,
GPIO/pin-control ownership or writes, pad writes, interrupt enablement/routing
proof/delivery, clock/reset programming, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.3, and phase transition.

## Interrupt-Routing Source Contract

Accepted by `phase11-rp1-interrupt-routing-source-contract-20260607`:
`phase11-rp1-interrupt-routing-source-contract-v1` is the first
source-backed routing contract after the GPIO14 STATUS read frontier. It
selects exactly one next diagnostic target:
`rp1-io-bank0-msix-cfg-read`, a read-only/no-enable 32-bit volatile load from
RP1 `RP1_PCIE_APBS` `MSIX_CFG(0)` at CPU physical `0x1f00108008`.

Source references identify `RP1_INT_IO_BANK0 = 0`, the `rp1_gpio` bank0
parent interrupt, Linux's RP1 irqdomain/MSI-X vector path, and the BCM2712
`pcie2`/`mip0` MSI route. Source inspection predicts hwirq 0 maps through
PCI MSI-X vector 0 and MIP0 MSI vector 0 to GIC SPI 128 / INTID 160, but this
remains an unaccepted routing assumption until a later hardware proof.

The next diagnostic may report only the contract id, target, hwirq, predicted
MSI-X vector, predicted GIC SPI/INTID, address, width, raw MSI-X config value,
decoded enable/test/IACK/IACK_EN bits, and one of the bounded routing
classifications. It must not write GPIO `INTE`, GPIO `CTRL`, MSI-X
`ENABLE`/`IACK`, PCIe MSI, MIP, or GIC registers, install an ISR, or unmask
interrupts.

The required paired control must preserve the same output shape while
constructing no RP1 GPIO/RIO/PADS/clock/reset/MSI-X address, no PCIe
config/MSI/MIP/GIC address, and performing no volatile load/store to those
paths. Accepted claims are source-contract only; interrupt enablement,
interrupt delivery, GPIO ownership, clock/reset programming, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe, Milestone 11.3, and
phase transition remain unaccepted.
