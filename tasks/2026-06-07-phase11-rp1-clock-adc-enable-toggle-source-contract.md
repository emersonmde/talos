# Phase 11 RP1 Clock ADC Enable Toggle Source Contract

Task id: phase11-rp1-clock-adc-enable-toggle-source-contract-20260607

Status: accepted

Classification: accepted-source-contract

## Goal

Decide the smallest source-backed reversible non-idempotent CLK_ADC_CTRL
enable-bit write/readback/restore proof after the accepted idempotent write
boundary.

## Scope

- Reviewed the accepted CLK_ADC_CTRL idempotent write/readback/restore
  closeout, retained RP1 Linux clock/reset sources, RP1 device-tree ADC/clock
  facts, and current Talos RP1 diagnostic helpers.
- Selected one bounded pre-state-derived CLK_ADC_CTRL enable-bit transition:
  rp1-clk-adc-ctrl-enable-bit-toggle-restore.
- Defined exact pre-read, transition-write, post-read, restore-write, and
  restore-read operations, masks, expected invariants, report fields,
  classifications, paired control requirements, and forbidden operations.
- Updated project contract and roadmap docs for the accepted source-contract
  frontier.

## Non-Goals

No runtime implementation, hardware run, boot archive publication,
hardwareTestLock acquisition, additional RP1 clock/reset writes,
reset-controller writes, GPIO/RIO/pad writes, event generation, interrupt
enablement or delivery, GIC IAR/EOIR acknowledgement, ISR installation,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, or phase transition.

This task does not create another same-shaped idempotent write/readback/restore
proof; the selected operation intentionally flips exactly one source-defined
enable bit and then restores the pre-read value.

## Findings

- fixed: selected a single non-idempotent but bounded transition on the already
  accepted CLK_ADC_CTRL register rather than broad clock/reset ownership.
- fixed: made the transition pre-state-derived: the only transition value is
  pre_raw ^ CLK_CTRL_ENABLE, so all non-enable bits are preserved and the
  post-read must differ from pre-read only by bit 11.
- fixed: required pre-state reporting before the transition write, immediate
  restore-write of pre_raw, and restore-read equality so a partial run with
  retained pre-state has an explicit cleanup value.
- fixed: retained clk_adc isolation evidence: rp1.dtsi marks the ADC consumer
  disabled, Linux models clk_adc as a normal clock with no GPCLK output-enable
  mask, and the selected register is not the boot UART, critical clock,
  PCIe/RP1 access, GPIO14/GPIO16, interrupt-routing, or reset path.
- deferred: broad clock/reset ownership, reset-controller operations, GPIO
  ownership retries, event generation, interrupt delivery, and handler
  ownership require later supervisor planning.
- not-an-issue: the accepted prior hardware value 0xdeaddead is retained only
  as evidence that bit 11 was preserved by the idempotent write proof; the new
  source contract does not interpret reserved or unknown fields beyond the
  source-defined enable, auxsrc, and source masks it reports.

No findings were removed.

## Accepted Source Contract

Contract id: phase11-rp1-clock-adc-enable-toggle-source-contract-v1

~~~text
target: rp1-clk-adc-ctrl-enable-bit-toggle-restore
operation: reversible pre-state-derived clock enable-bit transition
source block: RP1 clocks@18000, compatible raspberrypi,rp1-clocks
translated base: 0x1f00018000
selected register: CLK_ADC_CTRL
source offset: 0x00144
cpu physical address: 0x1f00018144
width: 32-bit little-endian volatile load/store
transition mask: CLK_CTRL_ENABLE, bit 11, 0x00000800
~~~

Allowed real-candidate operations, in order:

1. Pre-read CLK_ADC_CTRL at 0x1f00018144, retain pre_raw, decode
   enable/auxsrc/source fields, and emit the pre-state before any write.
2. Compute transition_raw = pre_raw ^ 0x00000800.
3. Write transition_raw to CLK_ADC_CTRL.
4. Post-read CLK_ADC_CTRL and retain post_raw.
5. Restore-write pre_raw to CLK_ADC_CTRL.
6. Restore-read CLK_ADC_CTRL and retain restore_raw.

Expected invariants:

- transition_raw != pre_raw.
- transition_raw ^ pre_raw == 0x00000800.
- post_raw ^ pre_raw == 0x00000800 for an accepted restored transition.
- restore_raw == pre_raw.
- Decoded CLK_CTRL_ENABLE bit 11 flips in post_raw and returns to its
  pre-state in restore_raw.
- Decoded CLK_CTRL_AUXSRC bits 9:5 and source bits remain unchanged across
  pre, post, and restore reads.

Report fields:

- contract id and target name.
- register name, CPU physical address, width, source offset, and transition
  mask.
- pre_raw, transition_raw, post_raw, and restore_raw.
- decoded pre/post/restore enable, aux source, and source fields.
- booleans for one-bit transition, post/pre enable flip,
  post_raw ^ pre_raw == 0x00000800, and restore_raw == pre_raw.
- retained prior CLK_ADC_CTRL idempotent proof context and GPIO14/GPIO16 fsel
  13 blocker context.
- terminal classification.

Accepted classifications:

- rp1-clock-adc-ctrl-enable-toggle-restored
- rp1-clock-adc-ctrl-enable-toggle-mismatch-restored
- rp1-clock-adc-ctrl-enable-toggle-restore-failed
- rp1-clock-adc-ctrl-enable-toggle-blocked-missing-clock-manager
- rp1-clock-adc-ctrl-enable-toggle-blocked-incoherent-transition
- rp1-clock-adc-ctrl-enable-toggle-inconclusive-capture
- no-mmio-clock-adc-ctrl-enable-toggle-control-visible
- staging/build-blocker

The paired no-MMIO/no-RP1/no-GIC control must preserve the same output shape
and classification vocabulary while constructing no RP1 clock/reset,
GPIO/RIO/pads, MSI-X/PCIe/MIP, or GIC MMIO address and performing no volatile
load/store to those paths. The control may emit simulated raw values and the
control-only classification.

## Why This Target Is Bounded

clk-rp1.c defines CLK_ADC_CTRL at offset 0x00144 and registers clk_adc as a
normal RP1 clock using CLK_ADC_CTRL, CLK_ADC_DIV_INT, and CLK_ADC_SEL. Linux's
rp1_clock_on and rp1_clock_off helpers modify only CLK_CTRL_ENABLE on a normal
clock's own control register. GPCLK output-enable writes are guarded by
oe_mask, and the clk_adc descriptor has no oe_mask.

rp1.dtsi defines rp1_adc: adc@c8000 as the consumer of RP1_CLK_ADC, and the
retained source marks the ADC node status = "disabled". The selected operation
does not reparent the clock, change dividers, change PLLs, touch GPCLK
output-enable, touch reset controllers, touch GPIO14/GPIO16, enable interrupt
sources, acknowledge GIC state, or write the boot UART, CLK_SYS, CLK_SLOW_SYS,
or CLK_UART paths.

The partial-failure cleanup value is the pre-read raw value. A valid hardware
implementation must emit the decoded pre-state before the transition write and
must restore pre_raw immediately after the post-read. If transition evidence is
observed without restore evidence, the real hardware task must classify the
run as restore-failed or inconclusive rather than accepting ownership.

## Forbidden Operations

- Any write value other than transition_raw = pre_raw ^ 0x00000800 and the
  restore value pre_raw.
- Any write to CLK_ADC_DIV_INT, CLK_ADC_SEL, CLK_SYS, CLK_SLOW_SYS, CLK_UART,
  PLL, frequency-counter, GPCLK output-enable, or reset-controller registers.
- Any write to GPIO14, GPIO16, RIO, pads, IO_BANK0 event/IRQ registers, MSI-X,
  PCIe config, MIP, GIC distributor/CPU interface, or reset-controller
  registers.
- GIC IAR/EOIR reads or writes, interrupt unmasking, interrupt delivery
  acceptance, ISR installation, DMA/cache, storage, generated-root,
  networking, SSH, broader PCIe enumeration, Milestone 11.3, or phase
  transition claims.

## Accepted Claims

This task accepts only the source contract for one reversible CLK_ADC_CTRL
enable-bit transition/readback/restore proof and its paired no-MMIO/no-RP1/
no-GIC control requirement. It does not accept runtime behavior, hardware
behavior, broad clock/reset ownership, reset-controller writes, GPIO ownership,
GPIO event generation, interrupt delivery, GIC acknowledgement, handler
ownership, DMA/cache, storage, generated-root, networking, SSH, broader PCIe
enumeration, Milestone 11.3, or a phase transition.

## Evidence

- tasks/evidence/2026-06-07-phase11-rp1-clock-adc-enable-toggle-source-contract/source-reference-notes.md
- tasks/evidence/2026-06-07-phase11-rp1-clock-adc-enable-toggle-source-contract/evidence-map.json
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md

## Validation

- Static inspection of accepted clock/reset write/restore closeout evidence,
  retained GPIO14/GPIO16 blocker evidence, project contract docs, retained
  Raspberry Pi Linux RP1 clock/MFD/device-tree sources, and Talos RP1
  constants: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Next Action

Promote phase11-rp1-clock-adc-enable-toggle-core-20260607 on the next worker
wake if dependencies remain satisfied. Do not acquire hardwareTestLock for the
local/static core.
