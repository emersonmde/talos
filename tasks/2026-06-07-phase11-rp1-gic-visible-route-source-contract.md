# Task: Phase 11 RP1 GIC-Visible Route Source Contract

Task ID: `phase11-rp1-gic-visible-route-source-contract-20260607`

Status: accepted

Evidence level: static source/doc inspection

## Goal

Define the smallest read-only/no-ack GIC-visible status contract for the
source-predicted RP1 IO_BANK0 route, without enabling, acknowledging, or
delivering interrupts.

## Scope

- Reviewed accepted Phase 4 Pi 5 GIC-400 facts, Talos GICv2 helpers, retained
  BCM2712/MIP0/RP1 source references, and the accepted MSIX_CFG(0) interrupt
  routing frontier.
- Selected exactly one read-only/no-ack GIC-visible diagnostic shape for the
  predicted RP1 IO_BANK0 route to GIC SPI 128 / INTID 160.
- Named exact allowed register reads, report fields, classification names, and
  forbidden operations.
- Defined the required paired no-MMIO/no-GIC/no-RP1 control before any Pi 5
  real diagnostic proof.
- Recorded findings with disposition and updated docs for the accepted
  source-contract frontier.

## Non-Goals

No runtime implementation, hardware run, boot archive publication,
hardwareTestLock acquisition, GIC enable writes, GIC IAR/EOIR acknowledge path,
interrupt unmasking, ISR installation, RP1 writes, MSI-X enable/IACK writes,
GPIO ownership, pin-control or pad writes, clock/reset programming, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, or phase transition.

Do not treat any read-only GIC status value as proof of delivered interrupts or
handler ownership.

## Findings

- fixed: retained the accepted Pi 5 GIC-400 base addresses from Phase 4:
  distributor base `0x10_7fff_9000` and CPU interface base
  `0x10_7fff_a000`.
- fixed: retained the accepted source-predicted route from the MSIX_CFG(0)
  frontier: RP1 IO_BANK0 hwirq 0 -> PCI MSI-X vector 0 -> MIP0 MSI vector 0
  -> GIC SPI 128 / INTID 160.
- fixed: selected one next diagnostic shape,
  `rp1-io-bank0-gic-route-status-read`, a read-only/no-ack status snapshot
  of the GICv2 distributor enable, pending, and active banks for INTID 160,
  plus the GIC CPU-interface highest-pending register.
- fixed: made the selected INTID math explicit. INTID 160 is in distributor
  bank 5, bit 0. The selected bank offset is `0x14`, so the exact reads are
  `GICD_ISENABLER5` at `0x10_7fff_9114`, `GICD_ISPENDR5` at
  `0x10_7fff_9214`, `GICD_ISACTIVER5` at `0x10_7fff_9314`, and
  `GICC_HPPIR` at `0x10_7fff_a018`.
- fixed: defined a paired no-MMIO/no-GIC/no-RP1 control. The control must
  preserve the same serial/output shape while constructing no GIC, RP1, MSI-X,
  PCIe, MIP, GPIO, pads, RIO, or clock/reset MMIO address and performing no
  volatile load or store to those paths.
- deferred: GIC distributor or CPU-interface enable writes, GIC IAR/EOIR
  acknowledgement, interrupt unmasking, ISR/handler ownership, RP1 event
  programming, MSI-X enable/IACK writes, GPIO ownership, pin-control or pad
  writes, clock/reset programming, DMA/cache, storage, generated-root,
  networking, SSH, broader PCIe enumeration, Milestone 11.3, and phase
  transition.
- not-an-issue: `GICC_HPPIR` is selected only as a read-only/no-ack
  observation register. Reading it is not an interrupt acknowledge and does not
  imply the CPU interface is enabled, an interrupt is deliverable, or Talos owns
  a handler for INTID 160.

No findings were removed in this source-contract task.

## Contract Summary

Accepted contract id:
`phase11-rp1-gic-visible-route-source-contract-v1`.

```text
name: rp1-io-bank0-gic-route-status-read
source hwirq: RP1_INT_IO_BANK0 = 0
predicted pci msix vector: 0
predicted gic route: MIP0 MSI vector 0 -> GIC SPI 128 / INTID 160
gic distributor base: 0x10_7fff_9000
gic cpu interface base: 0x10_7fff_a000
intid: 160
spi: 128
distributor bank: 5
distributor bank offset: 0x14
intid bit mask: 0x00000001
allowed reads:
  GICD_ISENABLER5 at 0x10_7fff_9114, 32-bit volatile load
  GICD_ISPENDR5 at 0x10_7fff_9214, 32-bit volatile load
  GICD_ISACTIVER5 at 0x10_7fff_9314, 32-bit volatile load
  GICC_HPPIR at 0x10_7fff_a018, 32-bit volatile load
operation: read-only/no-ack; no writes, unmasking, IAR, EOIR, or ISR install
```

The next local/static diagnostic core may implement only this read-only report
shape. It should report the contract id, target, source hwirq, predicted
MSI-X vector, predicted GIC SPI/INTID, GICD/GICC bases, bank, bit mask,
register addresses, raw enable/pending/active bank values, decoded INTID 160
enable/pending/active bits, raw `GICC_HPPIR`, decoded HPPIR INTID,
`hppir-spurious`, `hppir-target-match`, and one of these classifications:

- `gic-route-status-visible`
- `gic-route-status-bus-fault-or-trap-visible`
- `candidate-fetch-without-gic-route-status-marker`
- `capture-staging-blocked`
- `staging/build-blocker`

The diagnostic must not read `GICC_IAR`, write `GICC_EOIR`, enable GIC
groups, enable INTID 160, alter priority/target/configuration registers,
unmask IRQs, install an ISR, or touch any RP1/MSI-X/PCIe/MIP/GPIO/pads/RIO or
clock/reset MMIO path.

## Control Requirement

Before any real Pi 5 GIC-visible route proof, a paired control must be
accepted locally/static and then on Pi 5. The control must branch from the same
early entry point, preserve the same serial/output shape and classification
field, construct no GICD/GICC/RP1/MSI-X/PCIe/MIP/GPIO/pads/RIO/clock/reset
MMIO address, perform no volatile load or store to those paths, and emit
simulated zero raw values with a terminal marker suitable for the repaired v2
identity join.

## Accepted Claims

This task accepts only the source contract: the Pi 5 GIC-400 base addresses as
already accepted Phase 4 facts, the source-predicted RP1 IO_BANK0 route to GIC
SPI 128 / INTID 160, the selected read-only/no-ack GIC status snapshot, and
the paired no-MMIO/no-GIC/no-RP1 control requirement.

It does not accept real interrupt pending state, interrupt delivery, CPU
interface delivery, IAR/EOIR acknowledgement, ISR/handler ownership, GPIO
ownership, pin-control state, interrupt enablement, MSI-X delivery, PCIe MSI
delivery, clock/reset programming, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe enumeration, Milestone 11.3, or a phase
transition.

## Validation

- Static source/doc inspection: retained in
  `tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-source-contract/source-reference-notes.md`.
- `git diff --check`: pass.
- `/home/node/.cargo/bin/mdbook build`: pass.
- `git diff --cached --check`: pass.

## Result

Accepted. This accepts only the read-only/no-ack GIC-visible route status
source contract and the paired no-MMIO/no-GIC/no-RP1 control requirement. It
does not accept interrupt delivery, handler ownership, GPIO ownership,
clock/reset programming, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe, Milestone 11.3, or hardware behavior.

## Follow-Up

Promote `phase11-rp1-gic-visible-route-diagnostic-core-20260607` only after
this task is accepted and committed. That task owns the local/static real and
no-MMIO/no-GIC/no-RP1 control candidates and still must not run hardware.
