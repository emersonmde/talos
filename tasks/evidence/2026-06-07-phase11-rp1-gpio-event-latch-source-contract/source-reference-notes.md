# Phase 11 RP1 GPIO Event-Latch Source Reference Notes

Task: phase11-rp1-gpio-event-latch-source-contract-20260607

Evidence level: static source/doc inspection.

## Retained Sources

- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md
- tasks/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract.md
- tasks/2026-06-07-phase11-rp1-gpio-bank-source-status-contract.md
- tasks/2026-06-07-phase11-rp1-gpio-bank-source-status-pi5.md
- tasks/2026-06-07-phase11-rp1-gpio-bank-source-status-closeout.md
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/pinctrl-rp1.c
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/rp1-mfd.h

## Accepted Frontier Inputs

- Accepted RP1 address translation maps RP1 bus 0xc0_400d_0000 to CPU
  physical 0x1f_000d_0000.
- GPIO14 STATUS was previously accepted as a read-only diagnostic at CPU
  physical 0x1f000d0070.
- IO_BANK0 INTE and INTS were previously accepted as read-only diagnostics at
  0x1f000d011c and 0x1f000d0124.
- The real GPIO bank source-status proof observed raw INTE=0xdeaddead and
  INTS=0xdeaddead, with GPIO14 mask 0x00004000 decoded as enabled and
  source-status visible. That accepted only read-only snapshot visibility.

## RP1 GPIO14 Register References

Retained pinctrl-rp1.c defines:

- RP1_GPIO_STATUS = 0x0000
- RP1_GPIO_CTRL = 0x0004
- RP1_SET_OFFSET = 0x2000
- RP1_CLR_OFFSET = 0x3000
- RP1_GPIO_EVENTS_SHIFT_RAW = 20
- RP1_GPIO_STATUS_FALLING = BIT(20)
- RP1_GPIO_STATUS_RISING = BIT(21)
- RP1_GPIO_STATUS_LOW = BIT(22)
- RP1_GPIO_STATUS_HIGH = BIT(23)
- RP1_GPIO_STATUS_F_FALLING = BIT(24)
- RP1_GPIO_STATUS_F_RISING = BIT(25)
- RP1_GPIO_STATUS_F_LOW = BIT(26)
- RP1_GPIO_STATUS_F_HIGH = BIT(27)
- RP1_GPIO_CTRL_IRQEN_FALLING = BIT(20)
- RP1_GPIO_CTRL_IRQEN_RISING = BIT(21)
- RP1_GPIO_CTRL_IRQEN_LOW = BIT(22)
- RP1_GPIO_CTRL_IRQEN_HIGH = BIT(23)
- RP1_GPIO_CTRL_IRQEN_F_FALLING = BIT(24)
- RP1_GPIO_CTRL_IRQEN_F_RISING = BIT(25)
- RP1_GPIO_CTRL_IRQEN_F_LOW = BIT(26)
- RP1_GPIO_CTRL_IRQEN_F_HIGH = BIT(27)
- RP1_GPIO_CTRL_IRQRESET = BIT(28)

For GPIO14, the retained GPIO14 STATUS address proves the per-pin offset is
0x70, so the adjacent CTRL register is:

- GPIO14 STATUS: 0x1f000d0070
- GPIO14 CTRL RW: 0x1f000d0074
- GPIO14 CTRL SET: 0x1f000d2074
- GPIO14 CTRL CLR: 0x1f000d3074

## RP1 IO_BANK0 Register References

Retained pinctrl-rp1.c defines bank0 as:

- min_gpio=0
- num_gpios=28
- gpio_offset=0x0000
- inte_offset=0x011c
- ints_offset=0x0124

For GPIO14, the bank bit mask is:

- 1 << 14 = 0x00004000

The retained bank register addresses are:

- IO_BANK0 INTE RW: 0x1f000d011c
- IO_BANK0 INTE SET: 0x1f000d211c
- IO_BANK0 INTE CLR: 0x1f000d311c
- IO_BANK0 INTS: 0x1f000d0124

## Source Behavior

rp1_gpio_irq_handler reads pc->gpio_base + bank->ints_offset, iterates active
bits, writes RP1_GPIO_CTRL_IRQRESET through each pin's GPIO CTRL SET alias,
and then dispatches the child IRQ.

rp1_gpio_irq_config writes 1 << pin->offset to the bank INTE set or clear
alias. When disabling, it also writes RP1_GPIO_CTRL_IRQRESET through the GPIO
CTRL SET alias to clear latched events.

rp1_irq_set_type maps Linux IRQ types to raw event-enable bits, clears raw
event enables with a GPIO CTRL CLR write, writes IRQRESET through GPIO CTRL
SET, and then writes the selected raw event-enable bits through GPIO CTRL SET.

## Blocker Analysis

The retained source evidence identifies how Linux programs GPIO event
detection, clears latched events, and enables the bank source bit. It does not
justify a Talos diagnostic that deliberately changes GPIO14 event or pending
state while staying inside this task's non-goals.

Reasons:

- GPIO14 may be firmware-owned as UART0 TXD; the retained source references
  do not prove Talos can safely use it as a GPIO event source.
- Source-backed event generation would require either changing GPIO14 input
  state or relying on its current firmware-driven level. Changing input/output
  state would require pinmux, RIO, or pad ownership that this task forbids.
- Event-enable and IRQRESET writes are GPIO CTRL writes. They are the
  source-backed event path, but retained evidence does not prove they are
  harmless for a firmware-owned pin or deterministic without ownership.
- Bank INTE writes are source interrupt enable/disable operations. The
  retained GIC-visible route frontier does not prove parent-route masking or
  delivery safety if Talos changes source enable state.
- Cleanup is not source-backed enough for a failed or partial run: retained
  evidence does not provide the exact prior event-enable, latch, and bank
  enable state restore contract for firmware-owned GPIO14.

## Disposition

- fixed: exact source-backed event and source-enable write paths are recorded.
- fixed: exact GPIO14 and IO_BANK0 register addresses and bit fields are
  recorded.
- fixed: classified the task as source-contract-blocked rather than accepting
  a speculative event/pending discriminator.
- deferred: future supervisor planning may revisit this with GPIO ownership,
  parent-route masking, restore semantics, and a deterministic event source.
- not-an-issue: prior read-only STATUS, INTE, and INTS frontiers remain
  accepted under their existing contracts but are not event generation.
