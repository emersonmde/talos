# Phase 11 RP1 GPIO Ownership/Restore Source Reference Notes

Task: phase11-rp1-gpio-ownership-restore-source-contract-20260607

Evidence level: static source/doc inspection.

## Retained Sources

- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md
- tasks/2026-06-07-phase11-rp1-gpio-event-latch-source-contract.md
- tasks/evidence/2026-06-07-phase11-rp1-gpio-event-latch-source-contract/source-reference-notes.md
- tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-contract/source-reference-notes.md
- tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-source-contract/source-reference-notes.md
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/pinctrl-rp1.c
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/rp1-mfd.h

## Accepted Frontier Inputs

- Accepted RP1 translation maps RP1 bus 0xc0_400d_0000 to CPU physical
  0x1f000d0000 for IO_BANK0.
- Accepted source-status frontiers already use GPIO14 STATUS at
  0x1f000d0070, IO_BANK0 INTE at 0x1f000d011c, IO_BANK0 INTS at
  0x1f000d0124, and GPIO14 mask 0x00004000.
- Accepted GIC-visible route source/status frontiers identify RP1 IO_BANK0
  hwirq 0 through source-predicted INTID 160 and accept read-only GIC status
  observation only.
- The accepted event-latch source-contract task blocked GPIO14 event writes
  because ownership, parent-route masking, deterministic event source, and
  restore semantics were not yet proven.

## GPIO14 Pin Ownership Facts

Retained pinctrl-rp1.c defines:

- RP1_GPIO_CTRL_FUNCSEL_MASK = 0x0000001f
- RP1_GPIO_CTRL_OUTOVER_MASK = 0x00003000
- RP1_GPIO_CTRL_OEOVER_MASK = 0x0000c000
- RP1_GPIO_CTRL_INOVER_MASK = 0x00030000
- RP1_GPIO_CTRL_IRQOVER_MASK = 0xc0000000
- RP1_FSEL_GPIO = 0x05
- RP1_RIO_OUT = 0x00
- RP1_RIO_OE = 0x04
- RP1_RIO_IN = 0x08
- RP1_PAD_IN_ENABLE_MASK = 0x00000040
- RP1_PAD_OUT_DISABLE_MASK = 0x00000080

For GPIO14, the retained fsel table is:

~~~text
fsel 0: pwm0
fsel 1: dpi
fsel 2: uart4
fsel 3: i2c3
fsel 4: uart0
fsel 5: gpio
fsel 6: proc_rio
fsel 7: pio
fsel 8: spi5
~~~

This is why the next discriminator is read-only. GPIO14 can be UART0 in the
source-backed table, and prior Talos evidence uses firmware-preserved RP1 UART
state. A preflight may report fsel and ownership-adjacent state, but it must
not switch function or direction.

## GPIO14 Address Facts

Bank0 source facts:

- bank0 min_gpio = 0
- bank0 num_gpios = 28
- bank0 gpio_offset = 0x0000
- bank0 inte_offset = 0x011c
- bank0 ints_offset = 0x0124
- bank0 rio_offset = 0x0000
- bank0 pads_offset = 0x0004

The GPIO14 per-pin offset is 14 * 8 = 0x70. With accepted RP1 translation:

- GPIO14 STATUS: 0x1f000d0070
- GPIO14 CTRL: 0x1f000d0074
- IO_BANK0 INTE: 0x1f000d011c
- IO_BANK0 INTS: 0x1f000d0124
- RIO0 OUT: 0x1f000e0000
- RIO0 OE: 0x1f000e0004
- RIO0 IN: 0x1f000e0008
- GPIO14 pad: 0x1f000f003c

The GPIO14 bit in IO_BANK0 and RIO bank0 is 1 << 14 = 0x00004000.

## Source Behavior

rp1_get_fsel reads GPIO CTRL and treats a pin as none when OEOVER is not
peripheral or FUNCSEL is outside the fsel table.

rp1_set_fsel reads GPIO CTRL, enables pad input and output paths, updates
OUTOVER/OEOVER/FUNCSEL, and writes GPIO CTRL. This task forbids that write.

rp1_get_dir reads RIO OE. rp1_set_dir writes RIO OE set/clear aliases. This
task permits only reading RIO OE.

rp1_get_value reads RIO IN. rp1_set_value writes RIO OUT set/clear aliases.
This task permits only reading RIO OUT/OE/IN.

rp1_pinconf_get reads pad control. rp1_pad_update and rp1_pull_config_set
perform pad writes. This task permits only reading GPIO14 pad control.

rp1_irq_set_type clears raw event enables, writes IRQRESET, then writes raw
event-enable bits through GPIO CTRL aliases. rp1_gpio_irq_config writes IO_BANK
INTE set/clear aliases and, when disabling, writes IRQRESET. This task forbids
all of those writes.

## Selected Diagnostic

~~~text
contract: phase11-rp1-gpio-ownership-restore-source-contract-v1
target: rp1-gpio14-ownership-route-preflight-read
pin: GPIO14
bank: IO_BANK0
bit mask: 0x00004000
operation: read-only preflight
allowed writes: none
~~~

Allowed reads are GPIO14 STATUS/CTRL, IO_BANK0 INTE/INTS, RIO0 OUT/OE/IN,
GPIO14 pad control, and the previously accepted GIC-visible route status
registers for INTID 160.

## Restore Analysis

The selected next discriminator requires no GPIO, RIO, pad, INTE, MSI-X, MIP,
or GIC restore writes because it is read-only. That is the only restore
contract accepted here.

For a future supervisor-planned write task, retained source evidence indicates
that a minimally defensible restore plan would have to snapshot original
GPIO14 CTRL, IO_BANK0 INTE, RIO OUT/OE, and GPIO14 pad values; verify parent
route status before writes; perform bounded writes through source-backed
set/clear/RW paths; and then restore exact preflight state with post-restore
reads. This task does not accept those writes because partial-failure restore,
firmware-owned UART interaction, and parent-route masking writes remain
unproven.

## Review Findings

- fixed: selected one read-only preflight target and exact allowed reads.
- fixed: recorded GPIO14 pin-function, CTRL, RIO, pad, INTE/INTS, and GIC
  route status facts.
- fixed: classified restore for this preflight as no-op state cleanup because
  no writes are allowed.
- fixed: defined report fields, classification names, paired control shape,
  cleanup/quarantine rules, and forbidden operations.
- deferred: all GPIO/RIO/pad/INTE/GIC writes and deterministic event
  generation remain future supervisor-planned work.
- not-an-issue: prior read-only frontiers remain accepted inputs but do not
  prove GPIO ownership or event generation.
