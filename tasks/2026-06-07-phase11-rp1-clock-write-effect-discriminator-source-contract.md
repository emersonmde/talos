# Phase 11 RP1 Clock Write-Effect Discriminator Source Contract

Task id: phase11-rp1-clock-write-effect-discriminator-source-contract-20260607

Status: accepted

Classification: accepted-source-contract

## Goal

Define the smallest source-backed discriminator that explains the
CLK_ADC_CTRL enable-toggle mismatch before any further RP1 clock writes.

## Scope

- Reviewed the accepted CLK_ADC_CTRL enable-toggle mismatch-restored closeout,
  prior clock-manager status and write/readback/restore evidence, retained
  Raspberry Pi Linux RP1 clock/reset/device-tree sources, and current Talos
  RP1 diagnostic helpers.
- Selected exactly one next discriminator that is qualitatively different from
  the same-shaped CLK_ADC_CTRL enable-bit toggle rerun.
- Recorded exact allowed reads, report fields, classifications, paired
  no-MMIO/no-RP1/no-GIC control requirements, and exact forbidden operations.
- Updated roadmap/project contract docs for the accepted source-contract
  frontier.
- Recorded findings with disposition.

## Non-Goals

No runtime implementation, hardware run, boot archive publication,
hardwareTestLock acquisition, RP1 clock/reset writes, reset-controller writes,
GPIO/RIO/pad writes, event generation, interrupt enablement or delivery, GIC
IAR/EOIR acknowledgement, ISR installation, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3, or
phase transition.

## Accepted Frontier Inputs

- phase11-rp1-clock-adc-enable-toggle-closeout-20260607 accepted the prior
  chain as rp1-clock-adc-ctrl-enable-toggle-mismatch-restored-frontier-closed.
  The decisive Pi 5 result attempted transition_raw = pre_raw ^ 0x00000800
  against CLK_ADC_CTRL at 0x1f00018144, observed pre_raw=0xdeaddead,
  transition_raw=0xdeadd6ad, post_raw=0xdeaddead, restore_raw=0xdeaddead, and
  restore_eq_pre=true.
- The mismatch is precise and restored, but it does not prove successful
  non-idempotent clock ownership. Same-shaped CLK_ADC_CTRL enable-bit
  transition reruns are blocked.
- The accepted clock-manager status proof showed the broader clock manager
  status path visible enough to report pll-sys-lock=true, clk-sys-enabled=true,
  and clk-uart-enabled=true.
- The accepted idempotent CLK_ADC_CTRL write/readback/restore proof showed
  the same raw value, 0xdeaddead, preserved across a write-back and restore.

## Source Facts

- rp1.dtsi declares rp1_clocks: clocks@18000 with compatible
  raspberrypi,rp1-clocks and register range 0xc0_4001_8000 size 0x10038; the
  accepted translation maps the clock manager base to CPU physical
  0x1f00018000.
- clk-rp1.c defines CLK_ADC_CTRL = 0x00144, CLK_ADC_DIV_INT = 0x00148, and
  CLK_ADC_SEL = 0x00150.
- clk-rp1.c registers RP1_CLK_ADC as clk_adc with ctrl_reg = CLK_ADC_CTRL,
  div_int_reg = CLK_ADC_DIV_INT, sel_reg = CLK_ADC_SEL, div_int_max =
  DIV_INT_8BIT_MAX, max_freq = 50 MHz, and parents xosc plus clksrc_gp0
  through clksrc_gp5.
- rp1_clock_recalc_rate reads the clock's div_int_reg, and
  rp1_clock_get_parent reads sel_reg and, when sel is zero, ctrl_reg source and
  auxsrc fields. These source paths make the adjacent ADC divider and selector
  registers relevant to deciding whether the CLK_ADC_CTRL readback is a real
  clock-window state or an incoherent/sentinel readback.
- rp1_clock_set_rate and rp1_clock_set_parent write divider and source fields,
  but this contract does not select those writes. They are retained only as
  source context for why the divider/selector window matters.
- rp1.dtsi defines rp1_adc: adc@c8000 as the consumer of RP1_CLK_ADC and marks
  that ADC node status = disabled.
- clk_sys and clk_slow_sys are critical/always-enabled context, and clk_uart
  is the accepted UART clock. Reset-controller source behavior remains
  forbidden context only.

## Selected Contract

~~~text
contract: phase11-rp1-clock-write-effect-discriminator-source-contract-v1
target: rp1-clk-adc-window-coherence-read
operation: read-only ADC clock-register window coherence discriminator
base: 0x1f00018000
width: 32
~~~

Allowed 32-bit volatile reads:

1. CLK_SYS_CTRL at 0x1f00018014.
2. CLK_UART_CTRL at 0x1f00018054.
3. CLK_ADC_CTRL at 0x1f00018144.
4. CLK_ADC_CTRL at 0x1f00018144 again, after a local ordering barrier in the
   implementation.
5. CLK_ADC_DIV_INT at 0x1f00018148.
6. CLK_ADC_SEL at 0x1f00018150.

No writes are selected. No restore operation is needed because the
discriminator is read-only.

## Expected Invariants

- The diagnostic must report whether clk_sys and clk_uart remain enabled.
- The two CLK_ADC_CTRL reads must be reported separately, with a stability
  boolean.
- CLK_ADC_CTRL, CLK_ADC_DIV_INT, and CLK_ADC_SEL must be reported as a window,
  not interpreted as broad clock ownership.
- The diagnostic must report whether the ADC window looks like a repeated
  sentinel pattern: all three ADC window raw values equal each other, or all
  three equal the retained 0xdeaddead mismatch value.
- CLK_ADC_SEL must be decoded as zero, one-hot, or multi-bit. Source context
  treats SEL as one-hot and zero as a not-yet-enabled parent-selector state.
- The report must retain the prior mismatch context:
  pre_raw=0xdeaddead, transition_raw=0xdeadd6ad, post_raw=0xdeaddead,
  restore_raw=0xdeaddead, and restore_eq_pre=true.

## Report Fields

- contract id and target name.
- register names, source offsets, CPU physical addresses, and width.
- raw clk_sys_ctrl, clk_uart_ctrl, adc_ctrl_first, adc_ctrl_second,
  adc_div_int, and adc_sel values.
- decoded clk_sys_enable and clk_uart_enable bits.
- decoded ADC ctrl enable bit, auxsrc bits, and source bits for both
  CLK_ADC_CTRL reads.
- adc_ctrl_stable, adc_window_all_equal, adc_window_all_deaddead,
  adc_sel_zero, adc_sel_one_hot, and adc_sel_multi_bit booleans.
- retained prior enable-toggle mismatch/restored fields.
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

## Findings And Disposition

- fixed: selected one read-only ADC clock-register window discriminator that
  is qualitatively different from the blocked CLK_ADC_CTRL enable-bit toggle
  rerun.
- fixed: tied the discriminator to source-backed CLK_ADC_CTRL, CLK_ADC_DIV_INT,
  and CLK_ADC_SEL interactions instead of broadening into a clock/reset driver
  or generalized register-write API.
- fixed: retained the prior mismatch evidence and required explicit
  sentinel/coherence report fields to decide whether future writes are
  justified or blocked.
- fixed: required a paired no-MMIO/no-RP1/no-GIC control with the same output
  shape before any real Pi 5 proof.
- deferred: divider/source writes, reset-controller operations, GPIO
  ownership, event generation, interrupt delivery, handler ownership,
  DMA/cache, storage, generated-root, networking, SSH, broader PCIe
  enumeration, Milestone 11.3, and phase transition.
- not-an-issue: choosing a read-only discriminator is acceptable here because
  the named unknown is whether the CLK_ADC register window readback is
  coherent enough to justify any further clock-manager write.

No findings were removed.

## Evidence

- Source reference notes:
  tasks/evidence/2026-06-07-phase11-rp1-clock-write-effect-discriminator-source-contract/source-reference-notes.md.
- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-clock-write-effect-discriminator-source-contract/evidence-map.json.

## Validation

- Static inspection: accepted CLK_ADC enable-toggle closeout evidence, prior
  clock-manager status/write evidence, docs/src/project/phase11-rp1-pcie-map-contract.md,
  docs/src/project/reference-notes.md, retained Raspberry Pi Linux RP1
  clock/reset/device-tree sources, and current Talos RP1 diagnostic helpers
  inspected.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

Accepted as accepted-source-contract.

Next mechanically unblocked task:
phase11-rp1-clock-write-effect-discriminator-core-20260607. Implement only the
accepted read-only ADC clock-window discriminator and paired no-MMIO/no-RP1/
no-GIC control; do not acquire hardwareTestLock for the core task.
