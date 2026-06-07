# Phase 11 RP1 GPIO Bank Source-Status Source Reference Notes

Task: `phase11-rp1-gpio-bank-source-status-contract-20260607`

Evidence level: static source/doc inspection.

## Retained Sources

All retained source files referenced here are already committed in Talos task
evidence or source:

- `docs/src/project/phase11-rp1-pcie-map-contract.md`
- `docs/src/project/phase11-rp1-irq-clock-gpio-contract.md`
- `tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/pinctrl-rp1.c`
- `tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/rp1-mfd.h`
- `tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-closeout/evidence-map.json`

## Source Findings

- Accepted translation: Phase 11 maps RP1 bus `0xc0_4000_0000` to CPU
  physical `0x1f_0000_0000`, so IO_BANK0 at RP1 bus `0xc0_400d_0000`
  translates to CPU physical `0x1f_000d_0000`.
- RP1 MFD base facts: `rp1-mfd.h` names `RP1_IO_BANK0_BASE = 0x0d0000`,
  `RP1_IO_BANK1_BASE = 0x0d4000`, `RP1_IO_BANK2_BASE = 0x0d8000`, and
  `RP1_INT_IO_BANK0 = 0`.
- RP1 pinctrl bank facts: `pinctrl-rp1.c` defines bank0 with `min_gpio = 0`,
  `num_gpios = 28`, `gpio_offset = 0x0000`, `inte_offset = 0x011c`, and
  `ints_offset = 0x0124`.
- Selected register addresses:
  - IO_BANK0 `INTE`: `0x1f_000d_0000 + 0x011c = 0x1f_000d_011c`.
  - IO_BANK0 `INTS`: `0x1f_000d_0000 + 0x0124 = 0x1f_000d_0124`.
- Linux handler behavior: `rp1_gpio_irq_handler` reads
  `pc->gpio_base + bank->ints_offset`, iterates set bits, and then writes
  `RP1_GPIO_CTRL_IRQRESET` through each pin's GPIO `CTRL` set alias before
  dispatching the child IRQ. The read itself is not the acknowledgement path.
- Linux enable behavior: `rp1_gpio_irq_config` writes `1 << pin->offset` to
  `pin->inte + RP1_SET_OFFSET` or `pin->inte + RP1_CLR_OFFSET`; this
  contract permits only reading the base `INTE` value and forbids set/clear
  alias writes.
- Event programming behavior: `rp1_irq_set_type` clears GPIO event enables,
  writes `IRQRESET`, then writes selected event bits into GPIO `CTRL`.
  This contract forbids all GPIO `CTRL` writes and accepts no event
  generation or pending generation.
- Bit interpretation: bank0 covers GPIO0 through GPIO27. Bit `n` in bank0
  `INTE` or `INTS` corresponds to GPIO`n`; GPIO14 is mask
  `1 << 14 = 0x00004000`.

## Selected Diagnostic

```text
contract: phase11-rp1-gpio-bank-source-status-contract-v1
target: rp1-io-bank0-source-status-read
source hwirq: RP1_INT_IO_BANK0 = 0
bank: 0
bank gpios: GPIO0..GPIO27
primary allowed read:
  IO_BANK0_INTS @ 0x1f_000d_0124
companion allowed read:
  IO_BANK0_INTE @ 0x1f_000d_011c
width: 32-bit volatile little-endian loads
operation: read-only/non-destructive
```

Expected reporting fields: contract id, target, bank, bank GPIO range, source
hwirq, register addresses, widths, raw `INTE` value, raw `INTS` value,
decoded GPIO14 enable and source-status bits, decoded nonzero source-status
mask, and classification from the task/contract docs.

## No-MMIO/No-RP1/No-GIC Control Requirement

Before any real Pi 5 GPIO bank source-status proof, a paired control must be
accepted locally/static and then on Pi 5. The control must branch from the same
early entry point, preserve the same serial/output shape, construct no RP1
GPIO/RIO/pads/clock/reset/MSI-X/PCIe/MIP/GIC MMIO address, perform no volatile
load or store to those paths, and emit simulated zero raw values plus a
terminal marker suitable for the later Pi 5 identity join.

## Review Findings

- fixed: source-backed GPIO bank source-status contract selects one exact
  read-only/non-destructive diagnostic shape.
- fixed: exact IO_BANK0 `INTE` and `INTS` offsets, translated CPU physical
  addresses, widths, bank range, and GPIO14 bit mask are explicit.
- fixed: no-MMIO/no-RP1/no-GIC control requirements are explicit and block
  hardware proof until accepted.
- deferred: GPIO event programming, INTE writes, IRQRESET acknowledgement,
  parent interrupt delivery, GIC acknowledgement, ISR/handler ownership, GPIO
  ownership, pin-control or pad writes, clock/reset programming, DMA/cache,
  storage, generated-root, networking, SSH, broader PCIe enumeration,
  Milestone 11.3, and phase transition.
- not-an-issue: reading `INTS` does not acknowledge source status in the
  retained Linux path; acknowledgement is a separate GPIO `CTRL` IRQRESET
  write, which the contract forbids.
