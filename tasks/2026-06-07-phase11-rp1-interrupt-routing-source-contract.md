# Task: Phase 11 RP1 Interrupt-Routing Source Contract

Task ID: `phase11-rp1-interrupt-routing-source-contract-20260607`

Status: accepted

Evidence level: static source/doc inspection

## Goal

Define the smallest source-backed RP1 interrupt-routing contract after the
accepted GPIO14 STATUS read frontier, without implementing or enabling
interrupts.

## Scope

- Analyze retained Raspberry Pi Linux RP1 sources and Talos Phase 4/11
  interrupt docs for the RP1 hwirq, MSI-X, PCIe, and GIC routing path relevant
  to the first future interrupt observation.
- Select exactly one first interrupt-routing diagnostic shape for the next
  local/static task.
- Name the required paired no-MMIO/no-enable control before any real Pi 5
  diagnostic proof.
- Record findings with disposition: fixed, removed, deferred, or not-an-issue.
- Update roadmap/project contract docs only for accepted source-contract
  frontier changes.

## Non-Goals

No runtime implementation, hardware run, boot archive publication,
hardwareTestLock acquisition, GPIO/pin-control writes, pad writes,
clock/reset programming, interrupt enablement or delivery, ISR installation,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, or phase transition.

Do not treat the accepted GPIO14 STATUS raw value as proof of GPIO ownership,
interrupt state, or firmware pinmux ownership.

## Findings

- fixed: retained the RP1 GPIO bank0 source interrupt identity:
  `RP1_INT_IO_BANK0 = 0`, with `rp1_gpio` using RP1 interrupts 0, 1, and 2
  as level-high parent interrupts for IO_BANK0/1/2.
- fixed: retained the Linux RP1 interrupt-domain behavior. `mfd-rp1.c`
  allocates one PCI MSI-X vector per RP1 hwirq, maps children through an RP1
  irqdomain, sets MSI-X enable on domain activation, uses IACK enable for
  level-high sources, and writes IACK after chained handling for level sources.
- fixed: retained the BCM2712 PCIe/GIC-visible routing assumption for this
  slice: `pcie2` uses `mip0` as its MSI parent, and `mip0` maps 64 edge
  MSI entries to GIC SPI 128..191. Source inspection therefore predicts RP1
  hwirq 0 routes through MSI vector 0 to GIC SPI 128 / INTID 160, but that is
  not accepted as hardware behavior.
- fixed: selected one next diagnostic shape:
  `rp1-io-bank0-msix-cfg-read`, a read-only/no-enable 32-bit volatile load
  from the RP1 MSI-X config register for hwirq 0 at CPU physical
  `0x1f00108008`.
- fixed: defined a paired no-MMIO/no-enable control. The control must preserve
  the same serial/output shape while constructing no RP1 GPIO, RIO, pads,
  clock/reset, MSI-X, PCIe config, MIP, or GIC MMIO address and performing no
  volatile load or store to those paths.
- deferred: GPIO event-type programming, GPIO INTE writes, GPIO IRQRESET,
  MSI-X enable/IACK writes, PCIe MSI programming, GIC SPI enablement,
  interrupt unmasking, ISR installation, clock/reset programming, DMA/cache,
  storage, generated-root, networking, SSH, broader PCIe enumeration,
  Milestone 11.3, and phase transition.
- not-an-issue: the diagnostic target is IO_BANK0 routing state, not GPIO14
  pin state. It is intentionally paired with the accepted GPIO14 STATUS
  frontier only because GPIO14 belongs to IO_BANK0.

No findings were removed in this source-contract task.

## Contract Summary

Accepted contract id:
`phase11-rp1-interrupt-routing-source-contract-v1`.

```text
name: rp1-io-bank0-msix-cfg-read
source hwirq: RP1_INT_IO_BANK0 = 0
predicted pci msix vector: 0
predicted gic route: MIP0 MSI vector 0 -> GIC SPI 128 / INTID 160
rp1 target register: RP1_PCIE_APBS MSIX_CFG(0)
address: 0x1f00108008
width: 32-bit volatile little-endian load
operation: read-only/no-enable; no writes, unmasking, IACK, or ISR install
```

The next local/static diagnostic core may implement only this read-only report
shape. It should report the contract id, target, hwirq, predicted MSI-X vector,
predicted GIC SPI/INTID, address, width, raw MSI-X config value, decoded
`enable`, `test`, `iack`, and `iack_en` bits, and one of these
classifications:

- `routing-msix-cfg-visible`
- `routing-msix-cfg-bus-fault-or-trap-visible`
- `candidate-fetch-without-routing-marker`
- `capture-staging-blocked`
- `staging/build-blocker`

## Accepted Claims

This task accepts only the source contract: RP1 IO_BANK0 hwirq identity, Linux
RP1 irqdomain/MSI-X routing behavior as source reference, BCM2712 pcie2/MIP0
GIC routing assumptions as source reference, the selected read-only/no-enable
diagnostic shape, and the paired no-MMIO/no-enable control requirement.

It does not accept real interrupt state, GPIO ownership, pin-control state,
interrupt enablement, MSI-X delivery, PCIe MSI delivery, GIC delivery,
clock/reset programming, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe enumeration, Milestone 11.3, or a phase transition.

## Validation

- Static source/doc inspection: retained in
  `tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-source-contract/source-reference-notes.md`.
- `git diff --check`: pass.
- `/home/node/.cargo/bin/mdbook build`: pass.
- `git diff --cached --check`: pass.

## Result

Accepted. This accepts only a source-backed interrupt-routing contract and the
read-only/no-enable `rp1-io-bank0-msix-cfg-read` local/static next diagnostic.
It does not accept interrupt enablement/delivery, GPIO ownership, clock/reset
programming, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.3, or hardware behavior.

## Follow-Up

Promote `phase11-rp1-interrupt-routing-diagnostic-core-20260607` only after
this task is accepted and committed. That task owns the local/static real and
no-MMIO/no-enable control candidates and still must not run hardware.
