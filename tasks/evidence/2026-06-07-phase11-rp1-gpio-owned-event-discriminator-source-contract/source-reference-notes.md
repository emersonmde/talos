# Phase 11 RP1 GPIO Owned Event Discriminator Source Reference Notes

Task: phase11-rp1-gpio-owned-event-discriminator-source-contract-20260607

Evidence level: static source/doc inspection.

## Retained Sources

- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/project/reference-notes.md
- docs/src/roadmap.md
- tasks/2026-06-07-phase11-rp1-gpio-ownership-restore-closeout.md
- tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-closeout/evidence-map.json
- tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-pi5/evidence-map.json
- tasks/evidence/2026-06-07-phase11-rp1-gpio-event-latch-source-contract/source-reference-notes.md
- tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-contract/source-reference-notes.md
- tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-source-contract/source-reference-notes.md
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi-5-b.dts
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi.dtsi
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/rp1.dtsi
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/pinctrl-rp1.c
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/rp1-mfd.h

## Accepted Frontier Inputs

- GPIO14 event-latch writes are blocked because the real Pi 5 ownership
  preflight observed GPIO14 fsel 13 / unknown function.
- The accepted RP1 mapping translates IO_BANK0 bus 0xc0_400d_0000 to CPU
  physical 0x1f000d0000, RIO0 to 0x1f000e0000, and pads to 0x1f000f0000.
- The accepted IO_BANK0 route source contract predicts hwirq 0 reaches GIC
  SPI 128 / INTID 160; the accepted GIC-visible route status reads are
  GICD_ISENABLER5, GICD_ISPENDR5, GICD_ISACTIVER5, and GICC_HPPIR.
- The accepted GPIO14 closeout did not accept GPIO ownership or event
  generation; it explicitly requires a different discriminator with ownership,
  parent-route containment, deterministic source, partial-write recovery, and
  restore semantics.

## GPIO16 Selection Evidence

Retained Pi 5 device-tree line names identify RP1 GPIO16 as "GPIO16". The
same retained board source names fixed board consumers for RP1 GPIO32
ETH_RST_N, GPIO34 CD0_IO0_MICCLK, GPIO44 RP1_STAT_LED, GPIO46 CD1_IO0_MICCLK,
and other board-specific lines, but no retained fixed board consumer references
RP1 GPIO16.

Retained Pi 5 source identifies the debug UART as uart10 on the JST-SH UART
connector. Talos prior RP1 UART0 diagnostics used GPIO14/GPIO15; this contract
does not write GPIO14 or GPIO15. The retained source aliases define
uart0_ctsrts on GPIO16/GPIO17, but the retained uart0 default pinctrl selects
only GPIO14/GPIO15, so GPIO16 is not part of the prior Talos UART output path.

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

## Register Derivation

RP1 pinctrl bank0 facts:

- min_gpio = 0
- num_gpios = 28
- gpio_offset = 0x0000
- inte_offset = 0x011c
- ints_offset = 0x0124
- rio_offset = 0x0000
- pads_offset = 0x0004

GPIO16 per-pin offset is 16 * 8 = 0x80. With the accepted translation:

- GPIO16 STATUS: 0x1f000d0080
- GPIO16 CTRL RW: 0x1f000d0084
- GPIO16 CTRL SET: 0x1f000d2084
- GPIO16 CTRL CLR: 0x1f000d3084
- IO_BANK0 INTE RW: 0x1f000d011c
- IO_BANK0 INTE SET: 0x1f000d211c
- IO_BANK0 INTE CLR: 0x1f000d311c
- IO_BANK0 INTS: 0x1f000d0124
- RIO0 OUT: 0x1f000e0000
- RIO0 OE: 0x1f000e0004
- RIO0 IN: 0x1f000e0008
- GPIO16 pad control: 0x1f000f0044

GPIO16's bank0 bit is 1 << 16 = 0x00010000.

## Source Behavior

rp1_get_fsel reads GPIO CTRL and returns a named fsel only when the pin is in
a source-recognized function state. rp1_set_fsel enables pad input/output
paths, updates OUTOVER/OEOVER/FUNCSEL, and writes GPIO CTRL. This contract
permits only the one bounded GPIO16 function/override update and requires exact
restore.

rp1_set_dir writes RIO OE set/clear aliases and rp1_set_value writes RIO OUT
set/clear aliases. This contract permits only GPIO16 bit writes and requires
snapshot/restore of that bit.

rp1_pinconf_get reads pad control, while rp1_pad_update and pull updates write
the pad register. This contract permits one bounded GPIO16 pad update to enable
input and output paths, with exact restore.

rp1_irq_set_type clears raw event enables with GPIO CTRL CLR, writes IRQRESET,
and then writes selected raw event-enable bits through GPIO CTRL SET. This
contract permits only raw level-high event enable for GPIO16.

rp1_gpio_irq_config writes the bank source-enable bit through IO_BANK0 INTE
set/clear aliases. This contract permits only GPIO16 bit set/clear while the
parent GIC route remains read-only contained.

## Selected Diagnostic

~~~text
contract: phase11-rp1-gpio-owned-event-discriminator-source-contract-v1
target: rp1-gpio16-owned-level-high-event-discriminator
pin: GPIO16
bank: IO_BANK0
bit mask: 0x00010000
operation: bounded write/read discriminator with exact restore
~~~

The discriminator is source-backed but not yet hardware-proven. It requires a
paired no-MMIO/no-RP1/no-GIC control before any real Pi 5 run.

## Restore Analysis

The implementation must snapshot GPIO16 CTRL, IO_BANK0 INTE, RIO0 OUT/OE/IN,
GPIO16 pad, GPIO16 STATUS, IO_BANK0 INTS, and INTID 160 GIC route status
before writes. It must restore IO_BANK0 INTE, GPIO16 event enables/latches,
RIO OUT/OE bit 16, GPIO16 pad control, and GPIO16 CTRL to the preflight state,
then report post-restore reads.

If the parent route is enabled, pending, active, or HPPIR reports INTID 160,
the implementation must abort before writes with
gpio16-owned-event-preflight-blocked-parent-route. If GPIO16 preflight state
is incompatible with the selected bounded ownership path, it must abort or
classify as gpio16-owned-event-preflight-blocked-pin-function rather than
falling back to GPIO14 or another pin.

## Review Findings

- fixed: selected one source-backed non-GPIO14 pin and one bounded
  event/source-status discriminator.
- fixed: recorded exact GPIO16, IO_BANK0, RIO, pad, and GIC-visible status
  addresses.
- fixed: defined parent-route containment, exact ordering, restore semantics,
  report fields, classifications, and no-MMIO control requirements.
- deferred: hardware behavior, interrupt delivery, GIC acknowledgement,
  handler ownership, broad GPIO ownership, and GPIO14 event-generation retry.
- not-an-issue: source-backed GPIO16 line naming and lack of fixed board
  consumer evidence are sufficient for this source-contract task; the later Pi
  5 proof remains gated by preflight and restore evidence.

No findings were removed.
