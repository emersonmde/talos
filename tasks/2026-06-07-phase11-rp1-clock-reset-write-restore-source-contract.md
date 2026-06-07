# Phase 11 RP1 Clock/Reset Write/Restore Source Contract

Task id: phase11-rp1-clock-reset-write-restore-source-contract-20260607

Status: accepted

Classification: accepted-source-contract

## Goal

Decide the smallest source-backed reversible RP1 clock/reset
write/readback/restore ownership proof that can unblock later GPIO or
interrupt work without accepting clock/reset ownership broadly.

## Scope

- Reviewed the accepted RP1 clock manager status frontier, retained GPIO14 and
  GPIO16 fsel 13 blockers, Raspberry Pi Linux RP1 clock/MFD/device-tree
  sources, and current Talos RP1 constants.
- Selected one bounded clock-manager write/readback/restore target:
  `rp1-clk-adc-ctrl-idempotent-write-restore`.
- Defined exact pre-read, write, post-read, restore-write, and restore-read
  operations, report fields, classifications, paired control requirements, and
  forbidden operations.
- Updated project contract and roadmap docs for the accepted source-contract
  frontier.

## Non-Goals

No runtime implementation, hardware run, boot archive publication,
hardwareTestLock acquisition, reset-controller writes, GPIO/RIO/pad writes,
event generation, interrupt enablement or delivery, GIC IAR/EOIR
acknowledgement, ISR installation, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe enumeration, Milestone 11.3, or phase
transition.

## Findings

- fixed: selected a single idempotent clock-manager control-register write
  rather than changing a live clock's enable, divider, parent, PLL, or reset
  state.
- fixed: chose `CLK_ADC_CTRL` because `rp1.dtsi` marks the ADC node disabled,
  `clk-rp1.c` models `clk_adc` as a normal RP1 clock with no GPCLK output
  enable mask, and the register is not one of the critical system, slow
  system, UART, PCIe, GPIO, interrupt, or reset paths.
- fixed: made partial-failure cleanup idempotent: the only real write is the
  pre-read raw value written back to the same register, followed by an
  identical restore write. A failure after the first write leaves the register
  at the pre-read value.
- fixed: retained reset-controller behavior as forbidden source context; no
  reset-controller read or write is selected because retained sources do not
  define a narrow status/restore path for Talos.
- deferred: non-idempotent clock enable/disable, divider/source/PLL changes,
  reset ownership, GPIO ownership retries, event generation, interrupt
  delivery, and handler ownership require later source contracts.
- not-an-issue: an idempotent write/readback/restore proves the MMIO store path
  and restore discipline for one clock-manager register, but it is not evidence
  that Talos owns RP1 clocks or resets broadly.

No findings were removed.

## Accepted Source Contract

Contract id: phase11-rp1-clock-reset-write-restore-source-contract-v1

```text
target: rp1-clk-adc-ctrl-idempotent-write-restore
operation: idempotent clock-manager write/readback/restore
source block: RP1 clocks@18000, compatible raspberrypi,rp1-clocks
translated base: 0x1f00018000
selected register: CLK_ADC_CTRL
source offset: 0x00144
cpu physical address: 0x1f00018144
width: 32-bit little-endian volatile load/store
```

Allowed real-candidate operations, in order:

1. Pre-read `CLK_ADC_CTRL` at `0x1f00018144` and retain `pre_raw`.
2. Write `pre_raw` back to `CLK_ADC_CTRL` at `0x1f00018144`.
3. Post-read `CLK_ADC_CTRL` and retain `post_raw`.
4. Restore-write `pre_raw` back to `CLK_ADC_CTRL`.
5. Restore-read `CLK_ADC_CTRL` and retain `restore_raw`.

Expected unchanged fields:

- Entire 32-bit `CLK_ADC_CTRL` raw value remains unchanged:
  `post_raw == pre_raw` and `restore_raw == pre_raw`.
- Decoded `CLK_CTRL_ENABLE` bit 11 remains unchanged.
- Decoded `CLK_CTRL_AUXSRC` bits 9:5 remain unchanged.
- Decoded clock source bits remain unchanged.

Report fields:

- contract id and target name.
- register name, CPU physical address, width, and source offset.
- `pre_raw`, `post_raw`, `restore_raw`.
- decoded pre/post/restore enable, aux source, and source fields.
- equality booleans for `post_raw == pre_raw` and `restore_raw == pre_raw`.
- retained GPIO14/GPIO16 fsel 13 blocker context.
- terminal classification.

Accepted classifications:

- `rp1-clock-adc-ctrl-idempotent-write-restored`
- `rp1-clock-adc-ctrl-idempotent-write-mismatch-restored`
- `rp1-clock-adc-ctrl-idempotent-write-restore-failed`
- `rp1-clock-adc-ctrl-idempotent-write-blocked-missing-clock-manager`
- `rp1-clock-adc-ctrl-idempotent-write-inconclusive-capture`
- `staging/build-blocker`

The paired no-MMIO/no-RP1/no-GIC control must preserve the same output shape
and classification vocabulary while constructing no RP1 clock/reset,
GPIO/RIO/pads, MSI-X/PCIe/MIP, or GIC MMIO address and performing no volatile
load/store to those paths. The control may emit simulated raw values and a
control-only classification.

## Why This Target Is Bounded

`clk-rp1.c` defines `CLK_ADC_CTRL` at offset `0x00144` and registers
`clk_adc` as a normal RP1 clock using that control register, `CLK_ADC_DIV_INT`,
and `CLK_ADC_SEL`. The source enable/disable helpers modify only bit 11 of the
clock's own control register. Unlike GPCLK entries, `clk_adc` has no `oe_mask`,
so the helper does not write `GPCLK_OE_CTRL` or drive a GPIO clock output.

`rp1.dtsi` defines `rp1_adc: adc@c8000` as the consumer of `RP1_CLK_ADC`, and
the retained source marks that ADC node `status = "disabled"`. The selected
operation does not enable, disable, reparent, divide, or measure `clk_adc`; it
writes the pre-read raw value back to the same register and immediately
restores that same value. It therefore does not disturb boot UART, critical
system clocks, PCIe/RP1 access, GPIO14/GPIO16 state, interrupt routing, or
serial capture.

The only cleanup path needed after a partial run is the same idempotent
restore-write of `pre_raw`. If execution fails after the first write, the
register already contains `pre_raw`; a later bounded cleanup can still write
the retained pre-read value if it was captured, and the hardware state has not
been intentionally changed.

## Forbidden Operations

- Any write value other than the pre-read `CLK_ADC_CTRL` raw value.
- Any non-idempotent clock enable/disable, divider, source, PLL, frequency
  counter, GPCLK output-enable, or reset-controller operation.
- Any write to `CLK_SYS`, `CLK_SLOW_SYS`, `CLK_UART`, `PLL_SYS`, GPIO14,
  GPIO16, RIO, pads, IO_BANK0 event/IRQ registers, MSI-X, PCIe config, MIP,
  GIC distributor/CPU interface, or reset-controller registers.
- GIC IAR/EOIR reads or writes, interrupt unmasking, interrupt delivery
  acceptance, ISR installation, DMA/cache, storage, generated-root,
  networking, SSH, broader PCIe enumeration, Milestone 11.3, or phase
  transition claims.

## Accepted Claims

This task accepts only the source contract for one idempotent
`CLK_ADC_CTRL` write/readback/restore proof and its paired
no-MMIO/no-RP1/no-GIC control requirement. It does not accept runtime
behavior, hardware behavior, non-idempotent clock changes, reset ownership,
GPIO ownership, GPIO event generation, interrupt delivery, GIC
acknowledgement, handler ownership, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe enumeration, Milestone 11.3, or a phase
transition.

## Evidence

- tasks/evidence/2026-06-07-phase11-rp1-clock-reset-write-restore-source-contract/source-reference-notes.md
- tasks/evidence/2026-06-07-phase11-rp1-clock-reset-write-restore-source-contract/evidence-map.json
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md

## Validation

- Static inspection of accepted clock/reset status closeout evidence, retained
  GPIO14/GPIO16 blocker evidence, project contract docs, retained Raspberry Pi
  Linux RP1 clock/MFD/device-tree sources, and Talos RP1 constants: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Next Action

Promote phase11-rp1-clock-reset-write-restore-core-20260607 on the next worker
wake if dependencies remain satisfied. Do not acquire hardwareTestLock for the
local/static core.
