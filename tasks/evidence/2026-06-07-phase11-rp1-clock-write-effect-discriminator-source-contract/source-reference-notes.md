# Phase 11 RP1 Clock Write-Effect Discriminator Source Reference Notes

Task: phase11-rp1-clock-write-effect-discriminator-source-contract-20260607

Evidence level: static source/doc inspection.

## Retained Sources

- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/project/reference-notes.md
- docs/src/roadmap.md
- tasks/2026-06-07-phase11-rp1-clock-adc-enable-toggle-closeout.md
- tasks/evidence/2026-06-07-phase11-rp1-clock-adc-enable-toggle-closeout/evidence-map.json
- tasks/evidence/2026-06-07-phase11-rp1-clock-adc-enable-toggle-pi5/evidence-map.json
- tasks/evidence/2026-06-07-phase11-rp1-clock-adc-enable-toggle-pi5/classification.json
- tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-pi5/evidence-map.json
- tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-pi5/classification.json
- tasks/evidence/2026-06-07-phase11-rp1-clock-reset-write-restore-pi5/evidence-map.json
- tasks/evidence/2026-06-07-phase11-rp1-clock-reset-write-restore-pi5/classification.json
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/rp1.dtsi
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/clk-rp1.c
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/mfd-rp1.c
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/rp1-clock.h
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/rp1-mfd.h
- scripts/rpi5-rp1-clock-adc-ctrl-enable-toggle-archive.sh
- scripts/rpi5-rp1-clock-adc-ctrl-enable-toggle-review.sh
- scripts/rpi5-rp1-clock-adc-ctrl-enable-toggle-no-mmio-control-archive.sh
- scripts/rpi5-rp1-clock-adc-ctrl-enable-toggle-no-mmio-control-review.sh

## Retained Mismatch Evidence

- The prior closeout accepted
  rp1-clock-adc-ctrl-enable-toggle-mismatch-restored-frontier-closed.
- The decisive Pi 5 enable-toggle rerun selected tree
  7024bb54a9446c681d4a8b9c80372fe52a4d4f93b7939f299a8eb2d7199a697a,
  retained two 47,512-byte candidate TFTP fetches, passed the v2 identity
  join, retained 78 real result markers, and restored the lab to the original
  pre-run tree.
- The observed operation result was:
  pre_raw=0xdeaddead, transition_raw=0xdeadd6ad, post_raw=0xdeaddead,
  restore_raw=0xdeaddead, one_bit_transition=true,
  post_delta_is_transition_mask=false, and restore_eq_pre=true.
- The idempotent write/readback/restore proof also observed
  pre_raw=0xdeaddead, post_raw=0xdeaddead, restore_raw=0xdeaddead, with both
  equality checks true.
- The read-only clock-manager status proof retained visible
  pll-sys-lock=true, clk-sys-enabled=true, and clk-uart-enabled=true.

## Clock-Manager Source Facts

- rp1.dtsi declares rp1_clocks: clocks@18000 with compatible
  raspberrypi,rp1-clocks and register range 0xc0_4001_8000 size 0x10038. The
  accepted mapping translates that to CPU physical base 0x1f00018000.
- clk-rp1.c defines CLK_SYS_CTRL = 0x00014, CLK_UART_CTRL = 0x00054,
  CLK_ADC_CTRL = 0x00144, CLK_ADC_DIV_INT = 0x00148, and CLK_ADC_SEL =
  0x00150.
- clk-rp1.c defines CLK_CTRL_ENABLE = BIT(11), CLK_CTRL_AUXSRC_MASK =
  0x000003e0, CLK_CTRL_AUXSRC_SHIFT = 5, and CLK_CTRL_SRC_SHIFT = 0.
- clk-rp1.c registers RP1_CLK_ADC as clk_adc with parents xosc and clksrc_gp0
  through clksrc_gp5, ctrl_reg = CLK_ADC_CTRL, div_int_reg = CLK_ADC_DIV_INT,
  sel_reg = CLK_ADC_SEL, div_int_max = DIV_INT_8BIT_MAX, max_freq = 50 MHz,
  and fc0_src = FC_NUM(5, 5).
- rp1_clock_recalc_rate reads the clock's div_int_reg. For clocks without a
  fractional divider, such as clk_adc, it treats a zero divider read as 2^16
  and otherwise computes rate from the integer divider.
- rp1_clock_get_parent reads sel_reg as a one-hot selector. If sel is zero, it
  falls back to ctrl_reg source bits; if the source is AUX_SEL, it reads the
  ctrl_reg AUXSRC field.
- rp1_clock_set_rate writes div_int_reg, and rp1_clock_set_parent writes
  ctrl_reg source/auxsrc fields. Those write paths explain why adjacent ADC
  divider/selector registers are relevant, but this contract does not select
  them for writing.
- rp1_clock_on/off modify only CLK_CTRL_ENABLE in the clock's own ctrl_reg and
  only touch GPCLK_OE_CTRL when the clock descriptor has a nonzero oe_mask.
  The retained clk_adc descriptor has no oe_mask.
- rp1.dtsi defines rp1_adc: adc@c8000 as the consumer of RP1_CLK_ADC and marks
  the ADC node status = disabled.
- mfd-rp1.c obtains an optional reset control and calls reset_control_reset
  during Linux RP1 probe. Reset-controller writes remain forbidden.

## Selected Discriminator

~~~text
contract: phase11-rp1-clock-write-effect-discriminator-source-contract-v1
target: rp1-clk-adc-window-coherence-read
operation: read-only ADC clock-register window coherence discriminator
base: 0x1f00018000
width: 32
~~~

Allowed 32-bit volatile reads:

- CLK_SYS_CTRL at 0x1f00018014.
- CLK_UART_CTRL at 0x1f00018054.
- CLK_ADC_CTRL at 0x1f00018144.
- CLK_ADC_CTRL at 0x1f00018144 again, after a local ordering barrier in the
  implementation.
- CLK_ADC_DIV_INT at 0x1f00018148.
- CLK_ADC_SEL at 0x1f00018150.

No writes are selected, so no restore operation is required.

## Expected Invariants

- clk_sys and clk_uart enable bits are retained as guard fields.
- The repeated CLK_ADC_CTRL reads are stable or classified as unstable.
- The ADC clock-register window reports raw CTRL, DIV_INT, and SEL values.
- A repeated sentinel condition is reported when CTRL, DIV_INT, and SEL all
  equal each other or all equal the retained 0xdeaddead mismatch value.
- CLK_ADC_SEL is decoded as zero, one-hot, or multi-bit according to the
  source's selector interpretation.
- The report retains the prior mismatch-restored context.

## Report Fields

- contract id and target name.
- register names, source offsets, CPU physical addresses, and width.
- raw clk_sys_ctrl, clk_uart_ctrl, adc_ctrl_first, adc_ctrl_second,
  adc_div_int, and adc_sel values.
- decoded clk_sys_enable and clk_uart_enable bits.
- decoded ADC ctrl enable, auxsrc, and source fields for both ADC CTRL reads.
- adc_ctrl_stable, adc_window_all_equal, adc_window_all_deaddead,
  adc_sel_zero, adc_sel_one_hot, and adc_sel_multi_bit booleans.
- retained previous enable-toggle mismatch fields.
- terminal classification.

## Classifications

- rp1-clock-adc-window-coherent-read
- rp1-clock-adc-window-readback-sentinel
- rp1-clock-adc-window-unstable-readback
- rp1-clock-adc-window-blocked-missing-clock-manager
- rp1-clock-adc-window-blocked-uart-clock-disabled
- rp1-clock-adc-window-inconclusive-capture
- no-mmio-clock-adc-window-coherence-control-visible
- staging/build-blocker

## Paired Control Requirement

The paired control must preserve the real diagnostic's serial/output shape and
classification vocabulary while constructing no RP1 clock/reset, GPIO/RIO/pads,
MSI-X/PCIe/MIP, or GIC MMIO address and performing no volatile load/store to
those paths. The control may emit simulated raw values and a control-only
classification.

## Forbidden Operations

- Any RP1 clock/reset write, including CLK_ADC_CTRL, CLK_ADC_DIV_INT,
  CLK_ADC_SEL, divider, source, PLL, frequency-counter, GPCLK output-enable,
  or reset-controller writes.
- GPIO/RIO/pad writes, IO_BANK0 event/IRQ writes, MSI-X, PCIe config, MIP, or
  GIC writes.
- Event generation, interrupt enablement or delivery, GIC acknowledgement, ISR
  installation, DMA/cache, storage, generated-root, networking, SSH, broader
  PCIe enumeration, Milestone 11.3, or phase transition.
- Same-shaped CLK_ADC_CTRL enable-bit transition reruns.

## Review Findings

- fixed: selected a read-only ADC clock-register window discriminator to test
  the named unknown before further RP1 clock writes.
- fixed: used source-backed CLK_ADC_CTRL, CLK_ADC_DIV_INT, and CLK_ADC_SEL
  relationships instead of broadening into a driver or generalized write API.
- fixed: retained the prior 0xdeaddead mismatch evidence as context, not as a
  decoded ownership claim.
- fixed: required sentinel/coherence fields that can block or justify later
  clock write contracts.
- deferred: divider/source writes, reset-controller operations, GPIO
  ownership, event generation, interrupt delivery, and handler ownership.
- not-an-issue: no restore semantics are needed because the selected
  discriminator is read-only.

No findings were removed.
