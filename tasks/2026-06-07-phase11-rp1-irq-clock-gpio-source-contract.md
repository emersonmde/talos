# Task: Phase 11 RP1 IRQ/Clock/GPIO Source Contract

Task ID: `phase11-rp1-irq-clock-gpio-source-contract-20260607`

Status: accepted

Evidence level: static source/doc inspection

## Goal

Define the first Milestone 11.2 RP1 interrupt, clock, and GPIO contract after the accepted Milestone 11.1 RP1 UART0 FR mapping frontier, and name exactly one next diagnostic plus its no-MMIO control requirement.

## Scope

- Retain source references for the RP1 GPIO/pads block, RP1 interrupt path, and RP1 clock/reset assumptions.
- Name the first Milestone 11.2 diagnostic and whether it is read-only.
- Define the no-MMIO control requirements before any real Pi 5 diagnostic proof.
- Update roadmap/project docs for the accepted Milestone 11.2 source-contract boundary.

## Non-Goals

No Talos source/runtime implementation, hardware run, archive publication, hardwareTestLock acquisition, RP1 GPIO/pin-control writes, pad writes, clock/reset programming, interrupt enablement, DMA/cache work, storage, generated-root, networking, SSH, or broader PCIe enumeration.

## Findings

- fixed: retained Raspberry Pi Linux `rpi-6.12.y` source references for RP1 interrupt IDs, clock IDs, pinctrl/GPIO register offsets, clock driver UART details, and RP1 MFD MSI-X interrupt mapping.
- fixed: selected one exact next diagnostic: `rp1-gpio14-status-read`, a single read-only 32-bit volatile load from CPU physical `0x1f000d0070`, translated from RP1 GPIO14 IO_BANK0 STATUS.
- fixed: no-MMIO control requirements are explicit: the control must preserve the same serial/output shape while constructing no RP1 GPIO, pads, RIO, clock, or MSI-X MMIO address and performing no RP1 volatile load.
- deferred: Talos-owned GPIO/pin-control, pad updates, interrupt enablement, MSI-X/GIC delivery, clock/reset programming, DMA/cache policy, storage, generated-root, networking, SSH, broader PCIe enumeration, and Milestone 11.2 hardware proof.
- not-an-issue: GPIO14 may be muxed as UART0 TXD by firmware/overlays; the chosen register is a source-backed status read only, not a GPIO ownership or pinmux claim.

## Contract Summary

Accepted contract id: `phase11-rp1-irq-clock-gpio-contract-v1`.

```text
name: rp1-gpio14-status-read
address: 0x1f000d0070
width: 32-bit volatile little-endian load
source target: RP1 IO_BANK0 GPIO14 STATUS
operation: read-only; no writes or toggles
```

The diagnostic should report the contract id, target, address, width, raw value, interpreted raw/filtered level/event bits, and one of the bounded classifications named in the contract doc. The first Pi 5 proof still requires the no-MMIO control to pass first.

## Validation

- Static source/doc inspection: retained in `tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/source-reference-notes.md`.
- `git diff --check`: pass.
- `/home/node/.cargo/bin/mdbook build`: pass.
- `git diff --cached --check`: pass.

## Result

Accepted. This accepts only a source-backed Milestone 11.2 contract and the read-only `rp1-gpio14-status-read` local/static next diagnostic. It does not accept any RP1 GPIO ownership, pin-control writes, interrupts, clock/reset programming, DMA/cache, storage, generated-root, networking, SSH, broader PCIe enumeration, or hardware behavior.

## Follow-Up

Promote `phase11-rp1-gpio-status-diagnostic-core-20260607` only after this task is committed. That task owns the real local/static candidate and paired no-MMIO control artifacts; it still must not run hardware.
