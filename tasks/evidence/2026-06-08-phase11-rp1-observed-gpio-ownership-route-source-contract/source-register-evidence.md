# Phase 11 Observed GPIO14 Ownership/Route Source Register Evidence

Task: phase11-rp1-observed-gpio-ownership-route-source-contract-20260608

Evidence level: static source/doc inspection.

## Retained Inputs

- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md
- tasks/2026-06-07-phase11-rp1-gpio-ownership-restore-source-contract.md
- tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-source-contract/source-reference-notes.md
- tasks/2026-06-07-phase11-rp1-gpio-bank-source-status-contract.md
- tasks/2026-06-07-phase11-rp1-gic-visible-route-source-contract.md
- tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-source-contract/source-evidence-excerpts.md
- tasks/2026-06-08-phase11-rp1-observed-gpio-status-after-staging-repair-pi5.md
- tasks/2026-06-08-phase11-rp1-observed-gpio-status-after-staging-repair-closeout.md

## Accepted Frontier Inputs

- The accepted observed-aperture UART0 proof established a real RP1 read at
  observed CPU physical 0x1c00030018.
- The accepted repaired GPIO14 STATUS/CTRL proof established read-only
  observed-aperture visibility at 0x1c000d0070 and 0x1c000d0074, with
  gpio14-status-raw=0xabe3300, gpio14-ctrl-raw=0x84, and ctrl-funcsel=4.
- Retained Linux source still identifies IO_BANK0 GPIO14 STATUS/CTRL, INTE,
  INTS, RIO0 OUT/OE/IN, and GPIO14 pad control offsets.
- The source-expected 0x1f ownership/route preflight is retained as prior
  context, but the next contracted hardware address family is the observed
  0x1c RP1 aperture.
- The accepted GIC-visible route proof retains INTID 160 status reads as
  read-only parent-route inputs only.

## GPIO14 Source Facts

Retained pinctrl-rp1.c facts from the accepted source-expected preflight:

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

GPIO14 function table retained from the same source:

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

The accepted repaired observed GPIO14 proof reported ctrl-funcsel=4, which
source-decodes as uart0, not GPIO ownership. That makes this task a read-only
ownership/route preflight, not a write or event-generation task.

## Observed-Aperture Address Facts

Retained source offsets:

- IO_BANK0 base: 0xc0_400d_0000
- GPIO14 per-pin offset: 14 * 8 = 0x70
- IO_BANK0 INTE offset: 0x011c
- IO_BANK0 INTS offset: 0x0124
- RIO0 base: 0xc0_400e_0000
- RIO0 OUT/OE/IN offsets: 0x00, 0x04, 0x08
- pads bank0 base: 0xc0_400f_0000
- bank0 pads data offset: 0x0004
- GPIO14 pad stride: 14 * 4 = 0x38

Selected observed CPU physical reads:

- GPIO14 STATUS: 0x1c000d0070
- GPIO14 CTRL: 0x1c000d0074
- IO_BANK0 INTE: 0x1c000d011c
- IO_BANK0 INTS: 0x1c000d0124
- RIO0 OUT: 0x1c000e0000
- RIO0 OE: 0x1c000e0004
- RIO0 IN: 0x1c000e0008
- GPIO14 pad control: 0x1c000f003c

The GPIO14 bit in IO_BANK0 and RIO bank0 is 1 << 14 = 0x00004000.

Parent-route read-only inputs retained from the accepted GIC-visible route:

- GICD_ISENABLER5 at 0x107fff9114
- GICD_ISPENDR5 at 0x107fff9214
- GICD_ISACTIVER5 at 0x107fff9314
- GICC_HPPIR at 0x107fffa018
- INTID 160 is bank 5 bit 0.

## Selected Contract

~~~text
contract: phase11-rp1-observed-gpio-ownership-route-source-contract-v1
target: rp1-gpio14-ownership-route-observed-aperture-preflight-read
pin: GPIO14
bank: IO_BANK0
bit mask: 0x00004000
operation: read-only observed-aperture preflight
allowed writes: none
~~~

Allowed reads are the GPIO14 observed-aperture STATUS/CTRL pair, IO_BANK0
INTE/INTS, RIO0 OUT/OE/IN, GPIO14 pad control, and the already accepted GIC
route status registers for INTID 160.

## Review Findings

- fixed: selected the observed 0x1c RP1 aperture for ownership-adjacent
  GPIO/RIO/pad/source-status reads after the accepted repaired GPIO14
  STATUS/CTRL visibility proof.
- fixed: retained the prior source-backed GPIO14 function, CTRL, RIO, pad,
  IO_BANK0 INTE/INTS, and parent-route facts while changing only the RP1
  peripheral CPU aperture from source-expected 0x1f to observed 0x1c.
- fixed: recorded that ctrl-funcsel=4 from accepted evidence is UART0 and
  therefore blocks treating GPIO14 as Talos-owned GPIO.
- deferred: all GPIO CTRL/INTE/RIO/pad writes, IRQRESET acknowledgement,
  parent-route masking writes, interrupt delivery, and restore-after-write
  semantics remain future supervisor-planned work.
- not-an-issue: read-only GIC status remains a parent-route status input, not
  proof of interrupt delivery or permission to acknowledge an interrupt.
