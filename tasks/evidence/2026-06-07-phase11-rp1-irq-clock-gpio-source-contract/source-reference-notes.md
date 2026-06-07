# Phase 11 RP1 IRQ/Clock/GPIO Source Reference Notes

Task: `phase11-rp1-irq-clock-gpio-source-contract-20260607`

Evidence level: static source/doc inspection.

## Retained Sources

All retained source files are from Raspberry Pi Linux `rpi-6.12.y`.

- `rp1-mfd.h`: RP1 base offsets and interrupt IDs.
- `rp1-clock.h`: RP1 clock IDs.
- `pinctrl-rp1.c`: RP1 GPIO/pads/RIO register offsets and pinctrl behavior.
- `clk-rp1.c`: RP1 clock register offsets and UART clock descriptor.
- `mfd-rp1.c`: RP1 PCI/MSI-X interrupt-domain behavior.
- Milestone 11.1 retained DTS files: `rp1.dtsi`, `bcm2712-rpi-5-b.dts`, and `bcm2712-rpi.dtsi`.

## Source Findings

- GPIO/pads: `rp1.dtsi` declares `rp1_gpio: gpio@d0000` with IO_BANK, RIO, and PADS ranges at RP1 bus `0xc0_400d_0000`, `0xc0_400e_0000`, and `0xc0_400f_0000`; the Pi 5 board enables `&rp1_gpio`.
- GPIO/pads: `pinctrl-rp1.c` defines `RP1_GPIO_STATUS = 0x0000`, `RP1_GPIO_CTRL = 0x0004`, `RP1_RIO_IN = 0x08`, and bank0 offsets `{ gpio 0x0000, inte 0x011c, ints 0x0124, rio 0x0000, pads 0x0004 }`.
- GPIO/pads: `pinctrl-rp1.c` derives each bank0 pin's GPIO status/control register as `gpio_base + bank->gpio_offset + j * sizeof(u32) * 2`, so GPIO14 STATUS is RP1 offset `0x0d0000 + 14 * 8 + 0x0 = 0x0d0070`.
- GPIO/pads: with the accepted Milestone 11.1 RP1 translation, GPIO14 STATUS is CPU physical `0x1f000d0070`. GPIO14/GPIO15 are also the source-backed UART0 TXD/RXD pinctrl pair; reading status is not a pinmux or ownership change.
- Interrupt path: `rp1-mfd.h` assigns `RP1_INT_IO_BANK0 = 0`, `RP1_INT_IO_BANK1 = 1`, `RP1_INT_IO_BANK2 = 2`, and `RP1_INT_UART0 = 25`.
- Interrupt path: `rp1.dtsi` declares the RP1 node as an interrupt controller and gives `rp1_gpio` three level-high interrupts for IO_BANK0/1/2.
- Interrupt path: `mfd-rp1.c` allocates one MSI-X vector per RP1 interrupt, builds an IRQ domain, maps RP1 hwirqs through `pci_irq_vector`, and uses MSI-X config bits for enable/IACK.
- Interrupt path: `pinctrl-rp1.c` shows GPIO interrupt enablement would write `INTE` and GPIO `CTRL` event bits, then clear events through `IRQRESET`. The selected first diagnostic intentionally does none of that.
- Clock/reset assumptions: `rp1.dtsi` declares `rp1_clocks: clocks@18000` at RP1 bus `0xc0_4001_8000`, assigns RP1 system clocks, and identifies UART0 clocks as `RP1_CLK_UART` and `RP1_PLL_SYS_PRI_PH`.
- Clock/reset assumptions: `rp1-clock.h` maps `RP1_PLL_SYS_PRI_PH = 6` and `RP1_CLK_UART = 15`; `clk-rp1.c` places UART clock control at `CLK_UART_CTRL/DIV_INT/SEL` and names the clock `clk_uart`.
- Clock/reset assumptions: `mfd-rp1.c` treats a short BAR1 as firmware-not-initialized evidence and reads the RP1 chip ID before child devices are populated. The first GPIO status diagnostic depends on the accepted firmware/RP1 PCIe setup from Milestone 11.1 and must classify a fault/blocker instead of adding clock/reset programming in place.

## Selected Diagnostic

```text
contract: phase11-rp1-irq-clock-gpio-contract-v1
target: rp1-gpio14-status-read
address: 0x1f000d0070
width: 32-bit volatile little-endian load
operation: read-only
```

Expected reporting fields: contract id, target, address, width, raw 32-bit status value, interpreted raw/filtered falling/rising/low/high bits, and classification from the contract doc.

## No-MMIO Control Requirement

Before any real Pi 5 GPIO status diagnostic proof, a paired no-MMIO control must be accepted. The control must branch from the same early entry point, preserve the same serial/output shape, construct no `0x1f000d0070`, `0x1f000e0008`, `0x1f000f003c`, `0x1f00018000`, or RP1 MSI-X/config MMIO address, perform no volatile load/store to RP1 GPIO/RIO/PADS/clock/reset/interrupt registers, and emit a simulated/control raw value plus terminal marker suitable for the later Pi 5 identity join.

## Review Findings

- fixed: first Milestone 11.2 diagnostic is source-backed and read-only.
- fixed: no-MMIO control requirements are explicit and block hardware proof until accepted.
- deferred: interrupt enablement/delivery, GPIO/pin-control writes, clock/reset programming, DMA/cache, storage, generated-root, networking, SSH, and broader PCIe enumeration.
- not-an-issue: GPIO14's UART0 pin role is acceptable because the diagnostic reads status only and does not claim pin ownership.
