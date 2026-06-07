# Task: Phase 11 RP1 GPIO Ownership/Restore Source Contract

Task ID: phase11-rp1-gpio-ownership-restore-source-contract-20260607

Status: accepted

Classification: accepted-source-contract

Evidence level: static source/doc inspection

## Goal

Define the smallest source-backed GPIO ownership, parent-route masking,
deterministic event-source, and restore contract needed before Talos can
safely attempt RP1 GPIO event/pending generation.

## Scope

- Reviewed the accepted GPIO14 STATUS, GPIO bank source-status, GIC-visible
  route, interrupt-routing, and source-contract-blocked GPIO event-latch
  evidence.
- Reused retained Raspberry Pi Linux rpi-6.12.y RP1 pinctrl/MFD/device-tree
  evidence for GPIO ownership, function select, pads/RIO interaction,
  event-enable programming, IRQRESET, bank INTE, and parent-route state.
- Selected exactly one safe next discriminator:
  rp1-gpio14-ownership-route-preflight-read.
- Defined the candidate pin rule, exact allowed reads, no allowed writes,
  parent-route masking observations, preflight checks, cleanup/quarantine
  requirements, report fields, classification names, control requirements, and
  forbidden operations.
- Recorded findings with disposition.
- Updated roadmap/project contract docs for the accepted read-only preflight
  source contract.

## Non-Goals

No runtime implementation, hardware run, boot archive publication,
hardwareTestLock acquisition, GPIO event generation, interrupt pending
generation, interrupt delivery, GIC enable writes, GIC IAR/EOIR
acknowledgement, ISR installation, broad GPIO driver ownership, unbounded
pin-control/pad/RIO writes, clock/reset programming, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3, or
phase transition.

## Findings

- fixed: selected a read-only ownership/route preflight instead of reopening
  GPIO event-latch writes. The preflight can report whether GPIO14 is already
  in a state compatible with later event-source ownership without changing
  GPIO, RIO, pad, INTE, MSI-X, MIP, or GIC state.
- fixed: retained GPIO14 pin-function facts. Source pinctrl data maps GPIO14
  fsel values as pwm0, dpi, uart4, i2c3, uart0, gpio, proc_rio, pio, and
  spi5; GPIO fsel is 5 and proc_rio fsel is 6.
- fixed: retained GPIO14 ownership-state register fields. GPIO14 CTRL at
  0x1f000d0074 carries FUNCSEL, OUTOVER, OEOVER, INOVER, event enables, and
  IRQ override fields; STATUS at 0x1f000d0070 carries raw/filtered event
  status bits.
- fixed: retained source-backed RIO and pad state reads for GPIO14:
  RIO0 OUT/OE/IN at 0x1f000e0000/0x1f000e0004/0x1f000e0008 and pad control
  at 0x1f000f003c.
- fixed: retained parent-route observations from the accepted GIC-visible
  route frontier: GICD_ISENABLER5, GICD_ISPENDR5, GICD_ISACTIVER5, and
  GICC_HPPIR are read-only status inputs for INTID 160, not permission to
  enable, acknowledge, or deliver interrupts.
- fixed: defined restore semantics for this next discriminator as no-op
  hardware-state cleanup because the accepted diagnostic performs only reads.
  Any later write task must be supervisor-planned and must snapshot/restore
  GPIO CTRL, IO_BANK0 INTE, RIO OUT/OE, pad state, and parent-route status
  before it can mutate them.
- deferred: GPIO CTRL writes, IO_BANK0 INTE writes, RIO output/direction
  writes, pad writes, parent-route masking writes, IRQRESET acknowledgement,
  deterministic event-source generation, interrupt delivery, and restore-after
  partial-write semantics remain future work.
- not-an-issue: prior read-only GPIO14 STATUS, IO_BANK0 INTE/INTS, and
  GIC-visible route status frontiers remain useful inputs; this task does not
  reinterpret them as GPIO ownership or event generation.

No findings were removed in this source-contract task.

## Contract Summary

Accepted contract id:
phase11-rp1-gpio-ownership-restore-source-contract-v1.

~~~text
name: rp1-gpio14-ownership-route-preflight-read
pin-selection rule: GPIO14 only, because prior accepted frontiers already
  use GPIO14 STATUS, IO_BANK0 bit 14, and the IO_BANK0 route to INTID 160.
operation: read-only preflight; no MMIO writes, no event generation,
  no interrupt enablement, no acknowledgement, and no restore writes.
~~~

Allowed reads:

- GPIO14 STATUS at 0x1f000d0070, 32-bit volatile load.
- GPIO14 CTRL at 0x1f000d0074, 32-bit volatile load.
- IO_BANK0 INTE at 0x1f000d011c, 32-bit volatile load.
- IO_BANK0 INTS at 0x1f000d0124, 32-bit volatile load.
- RIO0 OUT at 0x1f000e0000, 32-bit volatile load.
- RIO0 OE at 0x1f000e0004, 32-bit volatile load.
- RIO0 IN at 0x1f000e0008, 32-bit volatile load.
- GPIO14 pad control at 0x1f000f003c, 32-bit volatile load.
- GICD_ISENABLER5 at 0x107fff9114, 32-bit volatile load.
- GICD_ISPENDR5 at 0x107fff9214, 32-bit volatile load.
- GICD_ISACTIVER5 at 0x107fff9314, 32-bit volatile load.
- GICC_HPPIR at 0x107fffa018, 32-bit volatile load.

No writes are allowed by this contract.

## Preflight Checks

The next diagnostic may only report the preflight state. It must not promote
any result to GPIO ownership. The report should decode:

- GPIO14 CTRL FUNCSEL, OUTOVER, OEOVER, INOVER, raw event-enable bits,
  filtered event-enable bits, and IRQOVER.
- GPIO14 STATUS raw and filtered event-status bits.
- IO_BANK0 INTE/INTS GPIO14 bit 0x00004000 and nonzero masks.
- RIO0 OUT/OE/IN GPIO14 bit 0x00004000.
- GPIO14 pad input-enable, output-disable, pull, drive, schmitt, and slew
  fields.
- GIC INTID 160 bank/bit state from GICD_ISENABLER5, GICD_ISPENDR5,
  GICD_ISACTIVER5, and GICC_HPPIR.

The conservative pass-shaped observation for a later supervisor decision is:

- GPIO14 FUNCSEL reports GPIO fsel 5 or another explicitly supervisor-accepted
  function for event sourcing.
- Parent route INTID 160 is not enabled, pending, or active in GIC status
  reads, and HPPIR does not report INTID 160.
- No raw/filtered event-enable bits are already set unless a later task
  explicitly accepts preserving/restoring that state.
- The diagnostic reaches a terminal marker with all fields present.

Any other state is retained evidence and should be classified as a preflight
blocker or warning, not fixed in place.

## Classifications

- gpio14-ownership-route-preflight-visible
- gpio14-ownership-preflight-blocked-non-gpio-function
- gpio14-ownership-preflight-blocked-parent-route-state
- gpio14-ownership-preflight-blocked-existing-event-or-enable-state
- gpio14-ownership-preflight-bus-fault-or-trap-visible
- candidate-fetch-without-gpio14-ownership-preflight-marker
- capture-staging-blocked
- staging/build-blocker

## Report Fields

- contract
- target
- pin
- gpio14-bit-mask
- gpio14-status-address
- gpio14-ctrl-address
- io-bank0-inte-address
- io-bank0-ints-address
- rio-out-address
- rio-oe-address
- rio-in-address
- pad-address
- gicd-isenabler5-address
- gicd-ispendr5-address
- gicd-isactiver5-address
- gicc-hppir-address
- width
- gpio14-status-raw
- gpio14-ctrl-raw
- gpio14-funcsel
- gpio14-func-name
- gpio14-outover
- gpio14-oeover
- gpio14-inover
- gpio14-raw-event-enable-mask
- gpio14-filtered-event-enable-mask
- gpio14-status-event-mask
- io-bank0-inte-raw
- io-bank0-ints-raw
- rio-out-raw
- rio-oe-raw
- rio-in-raw
- pad-raw
- pad-input-enable
- pad-output-disable
- gicd-isenabler5-raw
- gicd-ispendr5-raw
- gicd-isactiver5-raw
- gicc-hppir-raw
- intid160-enabled
- intid160-pending
- intid160-active
- hppir-intid
- classification

## Cleanup And Quarantine

Because the accepted diagnostic is read-only, cleanup requires no GPIO, RIO,
pad, INTE, MSI-X, MIP, or GIC restore writes. A later serialized hardware task
must still restore the lab boot tree under the normal hardware proof rules.

If any preflight read faults or the report is incomplete, the implementation
must quarantine the result under the bus-fault/trap, candidate-fetch, capture,
or staging classifications and must not add retry writes, reset writes,
interrupt acknowledgement, pinmux changes, or parent-route changes in the same
task.

## Control Requirement

Before any real Pi 5 ownership/route preflight proof, a paired no-MMIO/no-RP1
/no-GIC control must be accepted locally/static and then on Pi 5. The control
must branch from the same early entry point, preserve the same serial/output
shape and classification field, construct no RP1 GPIO/RIO/pads/clock/reset,
MSI-X/PCIe/MIP, or GIC MMIO address, perform no volatile load or store to
those paths, and emit simulated zero raw values plus a terminal marker
suitable for the repaired v2 identity join.

## Forbidden Operations

- GPIO14 CTRL, CTRL SET, or CTRL CLR writes.
- IO_BANK0 INTE SET, INTE CLR, or INTE RW writes.
- GPIO IRQRESET acknowledgement writes.
- RIO OUT or OE writes.
- Pad writes.
- GIC writes, GICC_IAR reads, GICC_EOIR writes, interrupt unmasking, and ISR
  installation.
- MSI-X enable/IACK writes, PCIe config/MSI writes, MIP writes, clock/reset
  writes, DMA/cache work, storage, generated-root, networking, SSH, broader
  PCIe enumeration, Milestone 11.3, and phase transition.
- Treating the preflight as GPIO ownership, event generation, interrupt
  pending generation, delivery, or restore readiness.

## Accepted Claims

This task accepts only a read-only source contract for one GPIO14
ownership/route preflight diagnostic and its paired no-MMIO/no-RP1/no-GIC
control requirement. It accepts exact source-backed register addresses,
decoded fields, report shape, classifications, forbidden operations, and
cleanup/quarantine rules for that preflight.

It does not accept GPIO ownership, GPIO event generation, interrupt pending
generation, interrupt enablement or delivery, GIC acknowledgement, ISR/handler
ownership, GPIO CTRL/INTE/RIO/pad writes, parent-route masking writes,
clock/reset programming, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe enumeration, Milestone 11.3, or a phase transition.

## Validation

- Static source/doc inspection: retained in
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-source-contract/source-reference-notes.md.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Result

Accepted as a read-only source contract. Follow-up implementation remains
supervisor-owned; no runtime behavior or hardware behavior is accepted by this
task.

## Follow-Up

Return to supervisor planning unless an explicit queued local/static
ownership-route preflight core task is already present and mechanically
unblocked. Any future write task must be planned separately after the preflight
evidence and must supply restore-after-partial-failure semantics before
mutating GPIO, RIO, pad, INTE, MSI-X, MIP, or GIC state.
