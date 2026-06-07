# Task: Phase 11 RP1 GPIO Bank Source-Status Contract

Task ID: `phase11-rp1-gpio-bank-source-status-contract-20260607`

Status: accepted

Evidence level: static source/doc inspection

## Goal

Define the smallest read-only RP1 GPIO bank interrupt-source status contract
for IO_BANK0 before any interrupt enablement or delivery work.

## Scope

- Reviewed the accepted RP1 GPIO14 STATUS, interrupt-routing, and GIC-visible
  route frontiers.
- Reused the retained Raspberry Pi Linux `rpi-6.12.y` RP1 pinctrl/MFD
  references and accepted RP1 address translation.
- Selected exactly one next diagnostic target:
  `rp1-io-bank0-source-status-read`.
- Named exact allowed register reads, report fields, classification names, and
  forbidden operations.
- Defined the paired no-MMIO/no-RP1/no-GIC control requirement before any
  local/static implementation or Pi 5 proof.

## Non-Goals

No runtime implementation, hardware run, boot archive publication,
hardwareTestLock acquisition, GPIO event generation, GPIO interrupt
enablement, GPIO `CTRL` writes, IRQRESET acknowledgement, GIC reads or writes,
MSI-X enable/IACK writes, interrupt unmasking, ISR installation, GPIO
ownership, pin-control or pad writes, clock/reset programming, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, or phase transition.

## Findings

- fixed: retained the source-backed RP1 IO_BANK0 GPIO interrupt status path
  used by Linux's `rp1_gpio_irq_handler`: read `bank->ints_offset`, then
  acknowledge each active pin separately through a GPIO `CTRL` IRQRESET write.
- fixed: selected one next diagnostic shape,
  `rp1-io-bank0-source-status-read`, a read-only 32-bit volatile load from
  IO_BANK0 `INTS` at CPU physical `0x1f000d0124`.
- fixed: retained IO_BANK0 `INTE` at CPU physical `0x1f000d011c` as a
  read-only companion report field, so the later diagnostic can distinguish
  source-visible status from the current enable-mask state without writing
  enable bits.
- fixed: made the selected bit interpretation explicit. Bank0 covers GPIO0
  through GPIO27; bit `n` reports source status for GPIO`n`, including
  GPIO14 at mask `0x00004000`.
- fixed: defined the paired no-MMIO/no-RP1/no-GIC control. The control must
  preserve the same serial/output shape while constructing no RP1 GPIO/RIO/pads
  or clock/reset, MSI-X/PCIe/MIP, or GIC MMIO address and performing no
  volatile load or store to those paths.
- deferred: GPIO event programming, INTE writes, IRQRESET acknowledgement,
  parent interrupt delivery, GIC acknowledgement, ISR/handler ownership, GPIO
  ownership, pin-control or pad writes, clock/reset programming, DMA/cache,
  storage, generated-root, networking, SSH, broader PCIe enumeration,
  Milestone 11.3, and phase transition.
- not-an-issue: reading IO_BANK0 `INTS` is non-destructive in the retained
  source path; Linux performs event acknowledgement with separate GPIO
  `CTRL` IRQRESET writes, which this contract forbids.

No findings were removed in this source-contract task.

## Contract Summary

Accepted contract id:
`phase11-rp1-gpio-bank-source-status-contract-v1`.

```text
name: rp1-io-bank0-source-status-read
rp1 io_bank0 base: 0xc0_400d_0000
cpu io_bank0 base: 0x1f_000d_0000
bank: 0
bank gpios: GPIO0..GPIO27
primary allowed read:
  IO_BANK0_INTS at 0x1f_000d_0124, 32-bit volatile load
companion allowed read:
  IO_BANK0_INTE at 0x1f_000d_011c, 32-bit volatile load
gpio14 bit mask: 0x00004000
operation: read-only; no writes, event generation, enablement, reset, ack, or ISR
```

The next local/static diagnostic core may implement only this read-only report
shape. It should report the contract id, target, bank, bank GPIO range, source
interrupt hwirq `RP1_INT_IO_BANK0 = 0`, CPU physical addresses, widths,
raw `INTE` and `INTS` values, decoded GPIO14 enable/status bits, a decoded
nonzero status mask, and one of these classifications:

- `gpio-bank-source-status-visible`
- `gpio-bank-source-status-bus-fault-or-trap-visible`
- `candidate-fetch-without-gpio-bank-source-status-marker`
- `capture-staging-blocked`
- `staging/build-blocker`

The diagnostic must not write GPIO `INTE`, GPIO `CTRL`, GPIO `IRQRESET`,
MSI-X, PCIe config/MSI, MIP, or GIC registers; read or write GIC acknowledge
registers; unmask interrupts; install an ISR; generate GPIO events; claim GPIO
ownership; or touch any pad, RIO, clock, or reset MMIO path.

## Control Requirement

Before any real Pi 5 GPIO bank source-status proof, a paired control must be
accepted locally/static and then on Pi 5. The control must branch from the same
early entry point, preserve the same serial/output shape and classification
field, construct no RP1 GPIO/RIO/pads/clock/reset/MSI-X/PCIe/MIP/GIC MMIO
address, perform no volatile load or store to those paths, and emit simulated
zero raw values with a terminal marker suitable for the repaired v2 identity
join.

## Accepted Claims

This task accepts only the source contract: the accepted RP1 address
translation for IO_BANK0, the retained Linux pinctrl source-status register
offsets for bank0 `INTE` and `INTS`, the selected read-only/non-destructive
source-status snapshot, and the paired no-MMIO/no-RP1/no-GIC control
requirement.

It does not accept GPIO event generation, interrupt pending generation,
interrupt enablement, parent interrupt delivery, CPU interface delivery,
IAR/EOIR acknowledgement, ISR/handler ownership, GPIO ownership, pin-control
state, pad writes, clock/reset programming, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe enumeration, Milestone 11.3, or a phase
transition.

## Validation

- Static source/doc inspection: retained in
  `tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-contract/source-reference-notes.md`.
- `git diff --check`: pass.
- `/home/node/.cargo/bin/mdbook build`: pass.
- `git diff --cached --check`: pass.

## Result

Accepted. This accepts only the read-only RP1 IO_BANK0 source-status source
contract and paired no-MMIO/no-RP1/no-GIC control requirement. It does not
accept GPIO event generation, interrupt enablement or delivery, handler
ownership, GPIO ownership, clock/reset programming, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.3, or hardware
behavior.

## Follow-Up

Promote `phase11-rp1-gpio-bank-source-status-core-20260607` only after this
task is accepted and committed. That task owns the local/static real and
no-MMIO/no-RP1/no-GIC control candidates and still must not run hardware.
