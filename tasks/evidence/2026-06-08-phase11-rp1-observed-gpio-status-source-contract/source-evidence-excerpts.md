# Phase 11 Observed GPIO Status Source/Evidence Excerpts

Task id: phase11-rp1-observed-gpio-status-source-contract-20260608

Evidence level: static source/evidence inspection.

## Source Excerpts

- Retained `rp1.dtsi` identifies the GPIO block as `rp1_gpio: gpio@d0000`,
  compatible `raspberrypi,rp1-gpio`, with GPIO and interrupt-controller
  roles. That block is the source identity for IO_BANK0 GPIO14.
- Retained `pinctrl-rp1.c` defines:
  - `RP1_GPIO_STATUS = 0x0000`;
  - `RP1_GPIO_CTRL = 0x0004`;
  - `RP1_GPIO_PCIE_INTE = 0x011c`;
  - `RP1_GPIO_PCIE_INTS = 0x0124`;
  - raw event bits at STATUS bits 20-23 and filtered event bits at bits 24-27;
  - CTRL function-select, override, raw IRQ enable, filtered IRQ enable, and
    IRQRESET fields.
- Retained `pinctrl-rp1.c` derives each bank0 pin register pair with
  `pin->gpio = pc->gpio_base + bank->gpio_offset + j * sizeof(u32) * 2`.
  For GPIO14, `14 * 8 = 0x70`, so STATUS is offset 0x70 and CTRL is offset
  0x74 from IO_BANK0.
- Retained `pinctrl-rp1.c` shows interrupt configuration and acknowledgement
  require writes through GPIO CTRL SET/CLR aliases and are not part of this
  read-only source contract.

## Address Reconciliation

Source-backed RP1 bus addresses:

~~~text
IO_BANK0 base:        0xc0_400d_0000
GPIO14 STATUS offset: 0x0000 + 14 * 8 + 0x0 = 0x70
GPIO14 CTRL offset:   0x0000 + 14 * 8 + 0x4 = 0x74
GPIO14 STATUS bus:    0xc0_400d_0070
GPIO14 CTRL bus:      0xc0_400d_0074
~~~

Observed-aperture candidate CPU addresses:

~~~text
GPIO14 STATUS observed CPU physical: 0x1c_000d_0070
GPIO14 CTRL observed CPU physical:   0x1c_000d_0074
~~~

The retained source-expected 0x1f comparators remain blocked for same-shaped
reruns:

~~~text
GPIO14 STATUS source-expected CPU physical: 0x1f_000d_0070
GPIO14 CTRL source-expected CPU physical:   0x1f_000d_0074
~~~

## Prior Evidence Inputs

- `phase11-rp1-observed-aperture-closeout-20260608` accepts only the selected
  0x1c00030018 UART0 FR read as visible, with raw=0x187 and
  raw-is-pl011-fr-shaped=true.
- `phase11-rp1-irq-clock-gpio-repaired-proof-closeout-20260607` accepted only
  the 0x1f000d0070 GPIO14 STATUS diagnostic boundary and explicitly blocked
  same-shaped GPIO14 STATUS hardware reruns without a different discriminator.
- Prior GPIO bank source-status work selected 0x1f000d011c/0x1f000d0124 for
  INTE/INTS. Those registers are not selected here because the current
  discriminator is per-pin observed-aperture status/control visibility, not
  interrupt source-status or event-delivery behavior.

## Findings And Disposition

- fixed: source excerpts support selecting observed 0x1c GPIO14 STATUS and
  CTRL as the smallest useful next read-only discriminator.
- fixed: optional IO_BANK0 INTE/INTS reads are intentionally excluded from the
  acceptance-critical read set.
- deferred: GPIO ownership, event generation, pending generation, interrupt
  enablement/delivery, GIC acknowledgement, pad/RIO/clock/reset ownership,
  DMA/cache, networking, SSH, Milestone 11.3, and phase transition.
- not-an-issue: decoding CTRL funcsel/override fields is still a read-only
  preflight; it does not claim Talos owns GPIO14.
