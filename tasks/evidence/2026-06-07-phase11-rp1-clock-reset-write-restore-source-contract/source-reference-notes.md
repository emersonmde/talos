# Phase 11 RP1 Clock/Reset Write/Restore Source Reference Notes

Task: phase11-rp1-clock-reset-write-restore-source-contract-20260607

Evidence level: static source/doc inspection.

## Retained Sources

- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md
- tasks/2026-06-07-phase11-rp1-clock-reset-status-closeout.md
- tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-source-contract/source-reference-notes.md
- tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-pi5/evidence-map.json
- tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-pi5/evidence-map.json
- tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-pi5/evidence-map.json
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/rp1.dtsi
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/clk-rp1.c
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/mfd-rp1.c
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/rp1-clock.h
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/rp1-mfd.h

## Accepted Frontier Inputs

- The accepted clock/reset status closeout retained RP1 clock-manager
  visibility on Pi 5, with PLL_SYS locked and CLK_SYS/CLK_UART enabled.
- GPIO14 ownership preflight and GPIO16 owned event preflight both reached Pi 5
  and reported fsel 13 / unknown function, so same-shaped GPIO ownership/event
  paths remain blocked.
- The accepted RP1 mapping translates RP1 bus addresses with
  cpu_phys = 0x1f00000000 + (rp1_bus - 0xc040000000).
- The accepted frontiers do not accept clock/reset writes, reset ownership,
  GPIO ownership, interrupt delivery, or handler ownership.

## Source Facts

- rp1.dtsi declares rp1_clocks: clocks@18000 with compatible
  raspberrypi,rp1-clocks and register range 0xc040018000 size 0x10038; the
  accepted translation maps the clock manager base to CPU physical
  0x1f00018000.
- clk-rp1.c defines CLK_ADC_CTRL = 0x00144, CLK_ADC_DIV_INT = 0x00148, and
  CLK_ADC_SEL = 0x00150.
- clk-rp1.c registers RP1_CLK_ADC as clk_adc with parents xosc and clksrc_gp0
  through clksrc_gp5, ctrl_reg = CLK_ADC_CTRL, div_int_reg =
  CLK_ADC_DIV_INT, sel_reg = CLK_ADC_SEL, div_int_max = DIV_INT_8BIT_MAX,
  and max_freq = 50 MHz.
- rp1_clock_on writes clockman_read(ctrl_reg) | CLK_CTRL_ENABLE to the
  clock's own ctrl_reg; rp1_clock_off writes clockman_read(ctrl_reg) &
  ~CLK_CTRL_ENABLE. For GPCLKs only, a nonzero oe_mask also writes
  GPCLK_OE_CTRL.
- The clk_adc descriptor has no oe_mask, so the selected ADC clock control
  register is not coupled to GPCLK output-enable writes.
- rp1.dtsi defines rp1_adc: adc@c8000 as the consumer of RP1_CLK_ADC, and the
  retained device-tree source marks the ADC node status = "disabled".
- clk_sys and clk_slow_sys are marked critical/always enabled in hardware,
  while clk_uart is the accepted UART clock. Those are retained only as
  forbidden context for this write proof.
- mfd-rp1.c obtains an optional reset control and calls reset_control_reset
  during Linux RP1 probe. The retained task does not have source evidence for
  a narrow Talos reset-controller status/restore path, so reset-controller
  writes remain forbidden.

## Selected Contract

~~~text
contract: phase11-rp1-clock-reset-write-restore-source-contract-v1
target: rp1-clk-adc-ctrl-idempotent-write-restore
operation: idempotent clock-manager write/readback/restore
base: 0x1f00018000
register: CLK_ADC_CTRL
source offset: 0x00144
cpu physical: 0x1f00018144
width: 32
~~~

Allowed real-candidate operations:

1. Pre-read CLK_ADC_CTRL and retain pre_raw.
2. Write pre_raw back to CLK_ADC_CTRL.
3. Post-read CLK_ADC_CTRL and retain post_raw.
4. Restore-write pre_raw back to CLK_ADC_CTRL.
5. Restore-read CLK_ADC_CTRL and retain restore_raw.

No bit transition is selected. The proof is an idempotent MMIO store/readback
and restore-discipline proof for the clock manager write path.

## Report Fields

- contract id and target name.
- register name, source offset, CPU physical address, and width.
- pre_raw, post_raw, and restore_raw.
- decoded enable bit, aux source bits, and source bits for each raw value.
- booleans for post/pre equality and restore/pre equality.
- retained GPIO14/GPIO16 fsel 13 blocker context.
- terminal classification.

## Classifications

- rp1-clock-adc-ctrl-idempotent-write-restored
- rp1-clock-adc-ctrl-idempotent-write-mismatch-restored
- rp1-clock-adc-ctrl-idempotent-write-restore-failed
- rp1-clock-adc-ctrl-idempotent-write-blocked-missing-clock-manager
- rp1-clock-adc-ctrl-idempotent-write-inconclusive-capture
- staging/build-blocker

## Paired Control Requirement

The paired control must preserve the real diagnostic's serial/output shape and
classification vocabulary while constructing no RP1 clock/reset, GPIO/RIO/pads,
MSI-X/PCIe/MIP, or GIC MMIO address and performing no volatile load/store to
those paths. The control may emit simulated raw values and a control-only
classification.

## Forbidden Operations

- Any write value other than the pre-read CLK_ADC_CTRL raw value.
- Clock enable/disable, divider, source, PLL, frequency-counter, GPCLK
  output-enable, or reset-controller writes.
- Writes to critical system clocks, UART clocks, GPIO/RIO/pads, IO_BANK0
  event/IRQ state, MSI-X, PCIe config, MIP, or GIC registers.
- Interrupt delivery, GIC acknowledgement, ISR installation, DMA/cache,
  storage, generated-root, networking, SSH, broader PCIe enumeration,
  Milestone 11.3, or phase transition.

## Review Findings

- fixed: selected one reversible clock-manager write target with exact allowed
  operations and report fields.
- fixed: avoided critical CLK_SYS/CLK_SLOW_SYS, boot UART clock, GPCLK
  output-enable, GPIO14/GPIO16, interrupt, PCIe/MSI-X/MIP, and reset paths.
- fixed: required an idempotent write of the pre-read value so a partial run
  does not intentionally change hardware state.
- deferred: non-idempotent clock enable/disable, divider/source/PLL changes,
  reset ownership, GPIO ownership, event generation, interrupt delivery, and
  handler ownership.
- not-an-issue: this contract proves a narrow write/readback/restore boundary,
  not broad RP1 clock/reset ownership.

No findings were removed.
