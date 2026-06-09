# Phase 11 Observed GPIO16 Ownership/Event Source Register Evidence

Task:
phase11-rp1-observed-gpio16-ownership-event-source-contract-20260609

Evidence level: static source/doc inspection.

## Retained Inputs

- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md
- tasks/2026-06-08-phase11-rp1-observed-gpio-ownership-route-source-contract.md
- tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-source-contract/source-register-evidence.md
- tasks/2026-06-08-phase11-rp1-observed-gpio-ownership-route-pi5.md
- tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-pi5/classification.json
- tasks/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-source-contract.md
- tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-source-contract/source-reference-notes.md
- tasks/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-pi5.md
- tasks/2026-06-07-phase11-rp1-gic-visible-route-source-contract.md

## Accepted Frontier Inputs

- The accepted observed-aperture GPIO14 ownership/route proof established
  read-only observed-aperture visibility at GPIO14 STATUS/CTRL, IO_BANK0
  INTE/INTS, RIO0 OUT/OE/IN, GPIO14 pad control, and read-only INTID 160 GIC
  route status inputs.
- That accepted proof classified GPIO14 as funcsel 4 / uart0, so it blocks
  treating GPIO14 as a Talos-owned event-generation target without a future
  explicit GPIO14 function-change task.
- The prior source-expected GPIO16 event discriminator retained source facts
  for GPIO16 selection, function table, bank0 bit, GPIO/RIO/pad offsets, and
  the no-MMIO control requirement.
- The prior GPIO16 hardware blocker used the source-expected 0x1f aperture and
  a write-backed discriminator. It is retained as blocker evidence, but this
  task intentionally selects only read-only observed 0x1c aperture preflight
  loads and accepts no writes.
- The accepted GIC-visible route proof retains INTID 160 status reads as
  read-only parent-route inputs only.

## GPIO16 Selection Facts

Retained Pi 5 source identifies RP1 GPIO16 as a generic GPIO16 line. Retained
fixed board consumers do not reference RP1 GPIO16. The retained debug UART is
uart10, and prior Talos RP1 UART0 diagnostics are confined to GPIO14/GPIO15.
The retained source also aliases uart0_ctsrts on GPIO16/GPIO17, but the
retained uart0 default pinctrl path selected GPIO14/GPIO15, and this task
performs no GPIO16 function write.

Retained pinctrl-rp1.c maps GPIO16 functions as:

~~~text
fsel 0: spi1
fsel 1: dpi
fsel 2: dsi0_te_ext
fsel 3: _
fsel 4: uart0
fsel 5: gpio
fsel 6: proc_rio
fsel 7: pio
fsel 8: _
~~~

The accepted GPIO14 observed-aperture ownership/route result reported GPIO14
funcsel 4 / uart0. GPIO16 is therefore the next safer feature-led preflight
candidate because it avoids GPIO14/GPIO15 serial-console ownership risk while
remaining in the same IO_BANK0 source route.

## Source Facts

Retained RP1 pinctrl/GPIO facts:

- RP1_GPIO_STATUS = 0x0000
- RP1_GPIO_CTRL = 0x0004
- RP1_GPIO_PCIE_INTE = 0x011c
- RP1_GPIO_PCIE_INTS = 0x0124
- raw event bits are STATUS bits 20-23
- filtered event bits are STATUS bits 24-27
- CTRL contains FUNCSEL, OUTOVER, OEOVER, INOVER, raw event enables, filtered
  event enables, IRQOVER, and IRQRESET fields
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

## Observed-Aperture Address Facts

Retained source offsets:

- IO_BANK0 base: 0xc0_400d_0000
- GPIO16 per-pin offset: 16 * 8 = 0x80
- IO_BANK0 INTE offset: 0x011c
- IO_BANK0 INTS offset: 0x0124
- RIO0 base: 0xc0_400e_0000
- RIO0 OUT/OE/IN offsets: 0x00, 0x04, 0x08
- pads bank0 base: 0xc0_400f_0000
- bank0 pads data offset: 0x0004
- GPIO16 pad stride: 16 * 4 = 0x40

Selected observed CPU physical reads:

- GPIO16 STATUS: 0x1c000d0080
- GPIO16 CTRL: 0x1c000d0084
- IO_BANK0 INTE: 0x1c000d011c
- IO_BANK0 INTS: 0x1c000d0124
- RIO0 OUT: 0x1c000e0000
- RIO0 OE: 0x1c000e0004
- RIO0 IN: 0x1c000e0008
- GPIO16 pad control: 0x1c000f0044

The GPIO16 bit in IO_BANK0 and RIO bank0 is 1 << 16 = 0x00010000.

Parent-route read-only inputs retained from the accepted GIC-visible route:

- GICD_ISENABLER5 at 0x107fff9114
- GICD_ISPENDR5 at 0x107fff9214
- GICD_ISACTIVER5 at 0x107fff9314
- GICC_HPPIR at 0x107fffa018
- INTID 160 is bank 5 bit 0.

## Selected Contract

~~~text
contract: phase11-rp1-observed-gpio16-ownership-event-source-contract-v1
target: rp1-gpio16-ownership-event-observed-aperture-preflight-read
pin: GPIO16
bank: IO_BANK0
bit mask: 0x00010000
operation: read-only observed-aperture preflight
allowed writes: none
~~~

Allowed reads are the GPIO16 observed-aperture STATUS/CTRL pair, IO_BANK0
INTE/INTS, RIO0 OUT/OE/IN, GPIO16 pad control, and the already accepted GIC
route status registers for INTID 160.

## Review Findings

- fixed: selected GPIO16 as the next read-only observed-aperture preflight
  target after GPIO14 was classified as UART0.
- fixed: recorded exact GPIO16, IO_BANK0, RIO, pad, and GIC route status
  addresses through the observed 0x1c RP1 aperture.
- fixed: retained source-backed GPIO16 function, CTRL, RIO, pad, IO_BANK0
  INTE/INTS, and parent-route facts while forbidding every write previously
  associated with the source-expected GPIO16 event discriminator.
- fixed: recorded why this preflight is qualitatively different from the
  prior GPIO16 blocker: observed aperture, read-only, and no event/action
  writes or restore claims.
- deferred: all GPIO CTRL/INTE/RIO/pad writes, IRQRESET acknowledgement,
  parent-route masking writes, interrupt delivery, event generation, and
  restore-after-write semantics remain future supervisor-planned work.
- not-an-issue: read-only GIC status remains a parent-route status input, not
  proof of interrupt delivery or permission to acknowledge an interrupt.

No findings were removed.
