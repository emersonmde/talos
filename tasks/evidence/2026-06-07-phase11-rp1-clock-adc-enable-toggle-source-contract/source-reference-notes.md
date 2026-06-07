# Phase 11 RP1 Clock ADC Enable Toggle Source Reference Notes

Task: phase11-rp1-clock-adc-enable-toggle-source-contract-20260607

Evidence level: static source/doc inspection.

## Retained Sources

- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md
- tasks/2026-06-07-phase11-rp1-clock-reset-write-restore-closeout.md
- tasks/evidence/2026-06-07-phase11-rp1-clock-reset-write-restore-closeout/evidence-map.json
- tasks/evidence/2026-06-07-phase11-rp1-clock-reset-write-restore-source-contract/source-reference-notes.md
- tasks/evidence/2026-06-07-phase11-rp1-clock-reset-write-restore-pi5/evidence-map.json
- tasks/evidence/2026-06-07-phase11-rp1-gpio-event-latch-source-contract/source-reference-notes.md
- tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-source-contract/source-reference-notes.md
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/rp1.dtsi
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/clk-rp1.c
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/mfd-rp1.c
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/rp1-clock.h
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/rp1-mfd.h
- src/target/rpi5.rs

## Accepted Frontier Inputs

- The accepted clock/reset status proof observed pll-sys-lock=true,
  clk-sys-enabled=true, and clk-uart-enabled=true.
- The accepted CLK_ADC_CTRL idempotent write/readback/restore proof reached
  Pi 5 and restored the lab. The accepted run reported pre_raw=0xdeaddead,
  post_raw=0xdeaddead, restore_raw=0xdeaddead, post_eq_pre=true, and
  restore_eq_pre=true.
- The accepted closeout limits the frontier to the selected CLK_ADC_CTRL
  write/readback/restore boundary. It does not accept non-idempotent clock
  programming, reset-controller writes, GPIO ownership, event generation,
  interrupt delivery, or handler ownership.
- GPIO14 and GPIO16 ownership/event paths retained fsel 13 blockers, so this
  task must not select GPIO/RIO/pad/event/interrupt-source writes.
- The accepted RP1 mapping translates RP1 bus addresses with
  cpu_phys = 0x1f00000000 + (rp1_bus - 0xc040000000).

## Source Facts

- rp1.dtsi declares rp1_clocks: clocks@18000 with compatible
  raspberrypi,rp1-clocks and register range 0xc0_4001_8000 size 0x10038; the
  accepted translation maps the clock manager base to CPU physical
  0x1f00018000.
- clk-rp1.c defines CLK_ADC_CTRL = 0x00144, CLK_ADC_DIV_INT = 0x00148, and
  CLK_ADC_SEL = 0x00150.
- clk-rp1.c defines CLK_CTRL_ENABLE = BIT(11),
  CLK_CTRL_AUXSRC_MASK = 0x000003e0, CLK_CTRL_AUXSRC_SHIFT = 5, and
  CLK_CTRL_SRC_SHIFT = 0.
- clk-rp1.c registers RP1_CLK_ADC as clk_adc with parents xosc and clksrc_gp0
  through clksrc_gp5, ctrl_reg = CLK_ADC_CTRL, div_int_reg =
  CLK_ADC_DIV_INT, sel_reg = CLK_ADC_SEL, div_int_max = DIV_INT_8BIT_MAX,
  and max_freq = 50 MHz.
- rp1_clock_on writes clockman_read(ctrl_reg) | CLK_CTRL_ENABLE to the clock's
  own control register. rp1_clock_off writes clockman_read(ctrl_reg) &
  ~CLK_CTRL_ENABLE.
- GPCLK output-enable writes occur only when the clock descriptor has a
  nonzero oe_mask; the retained clk_adc descriptor has no oe_mask.
- rp1.dtsi defines rp1_adc: adc@c8000 as the consumer of RP1_CLK_ADC, and the
  retained device-tree source marks the ADC node status = "disabled".
- clk_sys and clk_slow_sys are marked critical/always enabled in hardware, and
  clk_uart is the accepted UART clock. Those paths remain forbidden.
- mfd-rp1.c obtains an optional reset control and calls reset_control_reset
  during Linux RP1 probe. Reset-controller writes remain forbidden because
  retained source evidence still does not provide a narrow Talos reset
  status/restore path.

## Selected Contract

~~~text
contract: phase11-rp1-clock-adc-enable-toggle-source-contract-v1
target: rp1-clk-adc-ctrl-enable-bit-toggle-restore
operation: reversible pre-state-derived enable-bit transition
base: 0x1f00018000
register: CLK_ADC_CTRL
source offset: 0x00144
cpu physical: 0x1f00018144
width: 32
transition mask: 0x00000800
~~~

Allowed real-candidate operations:

1. Pre-read CLK_ADC_CTRL, retain and report pre_raw.
2. Compute transition_raw = pre_raw ^ 0x00000800.
3. Write transition_raw to CLK_ADC_CTRL.
4. Post-read CLK_ADC_CTRL, retain and report post_raw.
5. Restore-write pre_raw to CLK_ADC_CTRL.
6. Restore-read CLK_ADC_CTRL, retain and report restore_raw.

This is the first selected non-idempotent CLK_ADC_CTRL operation. It flips
exactly the source-defined enable bit and must restore the pre-read raw value.

## Expected Invariants

- transition_raw != pre_raw.
- transition_raw ^ pre_raw == 0x00000800.
- Accepted restored transition: post_raw ^ pre_raw == 0x00000800.
- Accepted restore: restore_raw == pre_raw.
- Enable decodes as flipped after the transition and returned after restore.
- Auxsrc and source decodes do not change.

## Report Fields

- contract id and target name.
- register name, source offset, CPU physical address, width, and transition
  mask.
- pre, transition, post, and restore raw values.
- decoded enable, auxsrc, and source fields for pre/post/restore reads.
- one-bit-transition and restore equality booleans.
- retained prior idempotent CLK_ADC_CTRL proof context.
- retained GPIO14/GPIO16 fsel 13 blocker context.
- terminal classification.

## Classifications

- rp1-clock-adc-ctrl-enable-toggle-restored
- rp1-clock-adc-ctrl-enable-toggle-mismatch-restored
- rp1-clock-adc-ctrl-enable-toggle-restore-failed
- rp1-clock-adc-ctrl-enable-toggle-blocked-missing-clock-manager
- rp1-clock-adc-ctrl-enable-toggle-blocked-incoherent-transition
- rp1-clock-adc-ctrl-enable-toggle-inconclusive-capture
- no-mmio-clock-adc-ctrl-enable-toggle-control-visible
- staging/build-blocker

## Paired Control Requirement

The paired control must preserve the real diagnostic's serial/output shape and
classification vocabulary while constructing no RP1 clock/reset, GPIO/RIO/pads,
MSI-X/PCIe/MIP, or GIC MMIO address and performing no volatile load/store to
those paths. The control may emit simulated raw values and a control
classification only.

## Partial-Failure Cleanup

The cleanup value is the pre-read raw value. A real implementation must emit
the pre-state before any transition write, write only the transition value
derived from that pre-state, and then immediately restore pre_raw. If a
hardware run observes transition evidence without restore evidence, the task
must classify the run as restore-failed or inconclusive and must not accept
clock ownership. A later cleanup task, if required, must use the retained
pre-state rather than inventing a new default.

## Forbidden Operations

- Any write value other than pre_raw ^ 0x00000800 and pre_raw.
- Divider, source, PLL, frequency-counter, GPCLK output-enable, or
  reset-controller writes.
- Writes to critical system clocks, UART clocks, GPIO/RIO/pads, IO_BANK0
  event/IRQ state, MSI-X, PCIe config, MIP, or GIC registers.
- Interrupt delivery, GIC acknowledgement, ISR installation, DMA/cache,
  storage, generated-root, networking, SSH, broader PCIe enumeration,
  Milestone 11.3, or phase transition.

## Review Findings

- fixed: selected one source-backed reversible CLK_ADC_CTRL enable-bit
  transition with exact allowed operations, masks, report fields, and
  classifications.
- fixed: required pre-state capture/reporting and exact restore of the
  pre-read raw value.
- fixed: excluded boot UART, critical clocks, PCIe/RP1 access, GPIO14/GPIO16,
  interrupt routing, GPCLK output-enable, and reset-controller paths.
- deferred: broad clock/reset ownership, reset-controller operations, GPIO
  ownership, event generation, interrupt delivery, and handler ownership.
- not-an-issue: the prior 0xdeaddead value is not treated as a decoded
  ownership claim; the new contract uses only source-defined masks and exact
  restore.

No findings were removed.
