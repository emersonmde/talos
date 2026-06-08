# Phase 11 RP1 Observed GPIO Status Source Contract

Task id: phase11-rp1-observed-gpio-status-source-contract-20260608

Status: accepted

Classification: accepted-observed-gpio-status-source-contract

## Goal

Define the next qualitatively different Milestone 11.2 discriminator after the
accepted observed-aperture UART0 FR proof: a read-only GPIO14 status/control
preflight at the same observed 0x1c RP1 aperture.

## Scope

- Used the accepted observed-aperture closeout as input: one selected
  0x1c00030018 RP1 UART0 FR read returned raw=0x187 under identity-joined
  Pi 5 evidence, while prior endpoint/bridge/0x1f RP1 shapes remain closed.
- Reused retained Raspberry Pi Linux `rpi-6.12.y` RP1 GPIO source and prior
  GPIO14 evidence only to define a narrow observed-aperture read contract.
- Selected one read-only pair:
  `rp1-gpio14-status-ctrl-observed-aperture-read`.
- Defined exact target addresses, widths, report fields, classification
  vocabulary, allowed reads, forbidden operations, and paired control shape.
- Updated roadmap and the Phase 11 RP1/PCIe map contract for the accepted
  source-contract boundary.
- Recorded findings with disposition.

## Non-Goals

No runtime source changes, hardware run, boot archive publication,
hardwareTestLock acquisition, endpoint ownership claim, broad RP1 mapping
claim, endpoint config retry, BAR discovery/programming, bridge setup writes,
PERST/link-control changes, GPIO/pad/clock/reset writes, interrupt
enablement/delivery, GIC acknowledgement, DMA/cache, storage, generated-root,
networking, SSH, Milestone 11.3, or phase transition.

This task does not rerun same-shaped endpoint config identity,
bridge/setup-state, 0x1f RP1 peripheral, 0x1f GPIO14 STATUS/CTRL, 0x1f
IO_BANK0 INTE/INTS, or 0x1c UART0 FR hardware tests.

## Findings And Disposition

- fixed: selected a qualitatively different observed-aperture GPIO14
  discriminator instead of repeating the accepted 0x1c UART0 FR proof or the
  closed 0x1f GPIO/status and GPIO bank source-status proofs.
- fixed: source-backed the register identity. Retained RP1 Linux source
  defines per-pin GPIO STATUS at offset 0x0 and CTRL at offset 0x4, and derives
  bank0 GPIO14 as bank base 0x0d0000 plus 14 * 8, giving RP1 bus addresses
  0xc0_400d_0070 and 0xc0_400d_0074.
- fixed: tied the selected CPU addresses to the accepted observed aperture
  without claiming broad RP1 mapping. The selected observed CPU physical
  addresses are 0x1c000d0070 and 0x1c000d0074, using the same observed 0x1c
  high aperture that made 0x1c00030018 visible.
- fixed: kept IO_BANK0 INTE/INTS out of this contract. They were already used
  in same-shaped 0x1f source-status work, and they are not acceptance-critical
  for the next question: whether the observed aperture reaches GPIO14 per-pin
  status/control state.
- fixed: required a paired no-MMIO/no-RP1/no-GIC control that preserves the
  report shape while constructing no forbidden address.
- deferred: GPIO ownership, event generation, interrupt pending generation,
  interrupt delivery, endpoint ownership, broad RP1 mapping, pad/RIO/clock or
  reset ownership, DMA/cache, networking, SSH, Milestone 11.3, and phase
  transition require later supervisor-planned tasks.
- not-an-issue: GPIO14 may still be firmware-owned or muxed for UART0 TXD; this
  preflight reads STATUS/CTRL only and treats any fsel/override value as
  evidence, not ownership.

No findings were removed.

## Accepted Source Contract

Contract id:
phase11-rp1-observed-gpio-status-source-contract-v1

~~~text
target: rp1-gpio14-status-ctrl-observed-aperture-read
operation: read-only observed RP1 GPIO14 status/control preflight
source targets:
  RP1 IO_BANK0 GPIO14 STATUS
  RP1 IO_BANK0 GPIO14 CTRL
source RP1 bus addresses:
  0xc0_400d_0070
  0xc0_400d_0074
observed CPU physical addresses:
  0x1c_000d_0070
  0x1c_000d_0074
width: two 32-bit volatile little-endian loads
retained source-expected comparators:
  0x1f_000d_0070 and 0x1f_000d_0074 remain blocked for same-shaped reruns
~~~

Allowed real-candidate sequence:

1. Emit a start marker that names
   phase11-rp1-observed-gpio-status-source-contract-v1.
2. Emit a before-read marker naming both selected addresses.
3. Perform exactly one 32-bit volatile load from 0x1c_000d_0070.
4. Perform exactly one 32-bit volatile load from 0x1c_000d_0074.
5. If both loads return, emit the report fields and one terminal
   classification.

No other RP1, PCIe, MIP, GIC, GPIO, RIO, pads, clock/reset, DMA, or other MMIO
load is selected. No MMIO store is selected.

## Source And Evidence Reconciliation

- Retained Linux `rp1.dtsi` declares `rp1_gpio: gpio@d0000` with IO_BANK0 at
  RP1 bus 0xc0_400d_0000.
- Retained Linux `pinctrl-rp1.c` defines `RP1_GPIO_STATUS = 0x0000`,
  `RP1_GPIO_CTRL = 0x0004`, `RP1_GPIO_PCIE_INTE = 0x011c`, and
  `RP1_GPIO_PCIE_INTS = 0x0124`.
- Retained Linux `pinctrl-rp1.c` derives each bank0 pin's GPIO register pair
  as `gpio_base + bank->gpio_offset + j * sizeof(u32) * 2`; for GPIO14 this is
  offset 0x70 for STATUS and 0x74 for CTRL.
- Retained first-light evidence and the accepted observed-aperture closeout
  support one selected 0x1c RP1 UART0 FR read as visible, but they do not
  accept endpoint ownership, broad RP1 mapping, GPIO ownership, or interrupt
  delivery.
- The next discriminator therefore tests whether the same observed 0x1c
  aperture reaches GPIO14 per-pin status/control registers. It deliberately
  does not add IO_BANK0 INTE/INTS, RIO, pads, GIC, or write paths.

## Report Fields

- contract id and target name.
- source RP1 bus addresses, observed CPU physical addresses, width, and
  register offsets.
- raw STATUS and raw CTRL values if both loads return.
- status raw event booleans: falling, rising, low, high.
- status filtered event booleans: falling, rising, low, high.
- ctrl funcsel value and outover/oeover/inover/irqover fields.
- ctrl raw IRQ enable booleans for raw and filtered falling/rising/low/high.
- per-register raw-is-deaddead, raw-is-all-ones, and raw-is-zero booleans.
- retained observed-aperture context: 0x1c00030018 raw=0x187 and
  raw-is-pl011-fr-shaped=true.
- terminal classification.

Accepted classifications:

- observed-aperture-gpio14-status-ctrl-visible
- observed-aperture-gpio14-status-ctrl-sentinel
- observed-aperture-gpio14-status-ctrl-all-ones
- observed-aperture-gpio14-status-ctrl-zero
- observed-aperture-gpio14-status-ctrl-no-return-or-trap
- observed-aperture-gpio14-status-ctrl-inconclusive-capture
- no-mmio-observed-gpio-status-control-visible
- staging/build-blocker

Classification rules:

- observed-aperture-gpio14-status-ctrl-visible: both selected loads return and
  the pair is not the all-sentinel, all-ones, or all-zero pair. The report must
  preserve raw values for later review; the classification is visibility only,
  not GPIO ownership or pinmux ownership.
- observed-aperture-gpio14-status-ctrl-sentinel: both selected loads return
  0xdead_dead.
- observed-aperture-gpio14-status-ctrl-all-ones: both selected loads return
  0xffff_ffff.
- observed-aperture-gpio14-status-ctrl-zero: both selected loads return
  0x0000_0000.
- observed-aperture-gpio14-status-ctrl-no-return-or-trap: the before-read
  marker is present and the accepted capture path shows no complete post-read
  report or records a trap/fault boundary.
- observed-aperture-gpio14-status-ctrl-inconclusive-capture: capture, TFTP,
  serial, selected-tree identity, or restore evidence cannot support one of
  the above classifications after required triage.
- no-mmio-observed-gpio-status-control-visible: the paired control preserves
  the report shape and classification vocabulary without constructing any
  forbidden MMIO address.
- staging/build-blocker: the candidate or control cannot be built, archived,
  reviewed, or staged under the task-owned gates.

## Paired Control Requirements

The paired control must preserve report shape and classification vocabulary
while constructing no BCM2712 PCIe, RP1 peripheral/SYSINFO/GPIO/RIO/pads/
clock/reset/MSI-X, MIP, GIC, DMA, or other MMIO address. It must not construct:

- 0x1c_000d_0070 or 0x1c_000d_0074;
- 0x1f_000d_0070 or 0x1f_000d_0074;
- 0x1c_000d_011c, 0x1c_000d_0124, 0x1f_000d_011c, or 0x1f_000d_0124;
- any 0x1c or 0x1f RP1 GPIO/RIO/pads/clock/reset address;
- any PCIe controller, MIP, GIC, or DMA address.

The control may use constants such as simulated raw values and strings so that
serial output shape remains comparable to the real candidate.

## Accepted Claims

- The selected discriminator is different from the closed endpoint config
  identity, bridge/setup-state, 0x1f RP1 peripheral, 0x1f GPIO14
  STATUS/CTRL/INTE/INTS, and 0x1c UART0 FR hardware tests.
- The allowed real operation is exactly two 32-bit volatile reads from
  0x1c_000d_0070 and 0x1c_000d_0074.
- The accepted control requirement forbids constructing RP1, PCIe, MIP, GIC,
  GPIO, RIO, pads, clock/reset, DMA, or other MMIO addresses.

## Rejected Claims And Retained Risks

This contract does not accept GPIO ownership, event generation, interrupt
pending generation, interrupt delivery, endpoint ownership, broad RP1 mapping,
pad/RIO/clock/reset ownership, DMA/cache, networking, SSH, Milestone 11.3, or
phase transition.

Same-shaped endpoint config identity, bridge/setup-state, 0x1f RP1 peripheral,
0x1f GPIO/status, 0x1f GPIO bank source-status, and 0x1c UART0 FR hardware
reruns remain closed unless a future supervisor task supplies a different
discriminator or new acceptance criteria.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-source-contract/evidence-map.json.
- Source/evidence excerpts:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-source-contract/source-evidence-excerpts.md.
- Accepted observed-aperture closeout:
  tasks/2026-06-08-phase11-rp1-observed-aperture-closeout.md.
- Retained RP1 GPIO source:
  tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/source-reference-notes.md,
  tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/pinctrl-rp1.c.

## Validation

- static inspection: passed for retained RP1 GPIO source, prior GPIO14
  evidence, accepted observed-aperture closeout, and this contract boundary.
- jq evidence map check: passed.
- git diff --check: passed.
- docs validation: mdbook build passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as accepted-observed-gpio-status-source-contract. The queued
observed GPIO status core task is mechanically unblocked on a future worker
wake after this task is committed.
