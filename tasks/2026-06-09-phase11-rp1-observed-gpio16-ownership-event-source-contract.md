# Task: Phase 11 Observed GPIO16 Ownership/Event Source Contract

Task ID: phase11-rp1-observed-gpio16-ownership-event-source-contract-20260609

Status: accepted

Classification: accepted-observed-gpio16-ownership-event-source-contract

Evidence level: static source/doc inspection

## Goal

Define the smallest source-backed read-only observed-aperture GPIO16
ownership/event preflight before any GPIO writes, event generation, interrupt
delivery, or GPIO14 ownership change.

## Scope

- Inspected retained RP1/Linux GPIO, RIO, pad, IO_BANK0 source-status, and GIC
  route source evidence.
- Inspected accepted observed-aperture GPIO14 STATUS/CTRL and ownership/route
  evidence, prior source-expected GPIO16 blocker evidence, accepted clock and
  write-restore evidence, and current roadmap/project docs.
- Selected only read-only observed-aperture loads needed to classify GPIO16
  function, per-pin status/control, bank source-enable/source-status, RIO
  OUT/OE/IN state, pad state, and accepted parent GIC route status.
- Recorded findings with disposition and updated task/project docs for the
  accepted source contract.

## Non-Goals

No runtime source change, Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, GPIO/RIO/pad/INTE/CTRL writes, IRQRESET,
interrupt unmasking, IAR/EOIR acknowledgement, ISR/handler install, event
generation, interrupt delivery, GPIO14 function change, endpoint config retry,
bridge setup write, clock/reset write, DMA/cache, storage, generated-root,
networking, SSH, Milestone 11.3, or phase transition.

Do not treat a read-only GPIO16 preflight as GPIO ownership,
event-generation readiness, or interrupt-delivery proof.

## Findings

- fixed: selected
  rp1-gpio16-ownership-event-observed-aperture-preflight-read as the next
  read-only observed 0x1c aperture contract after GPIO14 was classified as
  UART0 in the accepted ownership/route preflight.
- fixed: retained GPIO16 as the next safer candidate because retained Pi 5
  source names it as generic GPIO16, retained fixed board consumers do not
  reference GPIO16, the debug UART is uart10, and prior Talos RP1 UART0 usage
  is confined to GPIO14/GPIO15.
- fixed: retained GPIO16 source facts from the prior source-expected
  discriminator while changing this task to a read-only observed-aperture
  preflight: STATUS/CTRL, bank0 INTE/INTS, RIO0 OUT/OE/IN, GPIO16 pad
  control, GPIO16 bit 0x00010000, and the accepted INTID 160 GIC route status
  registers.
- fixed: translated the selected RP1 GPIO/RIO/pad reads through the accepted
  observed aperture: 0x1c000d0080, 0x1c000d0084, 0x1c000d011c,
  0x1c000d0124, 0x1c000e0000, 0x1c000e0004, 0x1c000e0008, and 0x1c000f0044.
- fixed: retained the accepted parent-route status inputs for INTID 160:
  GICD_ISENABLER5, GICD_ISPENDR5, GICD_ISACTIVER5, and GICC_HPPIR. These
  remain read-only status inputs, not interrupt delivery or acknowledgement
  claims.
- fixed: defined a paired no-MMIO/no-RP1/no-GIC control requirement that
  preserves output shape while constructing no RP1 GPIO/RIO/pads/clock/reset,
  MSI-X/PCIe/MIP, or GIC MMIO addresses.
- deferred: GPIO16 CTRL writes, IO_BANK0 INTE writes, IRQRESET
  acknowledgement, RIO OUT/OE writes, pad writes, parent-route masking writes,
  deterministic event-source generation, interrupt delivery, and
  restore-after-write semantics remain future supervisor-planned work.
- not-an-issue: the prior source-expected 0x1f GPIO16 fsel 13 blocker remains
  valid retained evidence, but it was a write-backed discriminator using the
  source-expected aperture. This task is qualitatively different: read-only,
  observed 0x1c aperture, and acceptance is limited to a preflight contract.

No findings were removed in this task.

## Contract Summary

Accepted contract id:
phase11-rp1-observed-gpio16-ownership-event-source-contract-v1.

~~~text
name: rp1-gpio16-ownership-event-observed-aperture-preflight-read
pin-selection rule: GPIO16 only, because GPIO14 is currently muxed to UART0
  in the accepted observed-aperture preflight and GPIO16 is the retained
  non-console candidate with no fixed board consumer evidence.
operation: read-only observed-aperture preflight; no MMIO writes, no event
  generation, no interrupt enablement, no acknowledgement, and no restore
  writes.
~~~

Allowed reads, in order:

- GPIO16 STATUS at 0x1c000d0080, 32-bit volatile load.
- GPIO16 CTRL at 0x1c000d0084, 32-bit volatile load.
- IO_BANK0 INTE at 0x1c000d011c, 32-bit volatile load.
- IO_BANK0 INTS at 0x1c000d0124, 32-bit volatile load.
- RIO0 OUT at 0x1c000e0000, 32-bit volatile load.
- RIO0 OE at 0x1c000e0004, 32-bit volatile load.
- RIO0 IN at 0x1c000e0008, 32-bit volatile load.
- GPIO16 pad control at 0x1c000f0044, 32-bit volatile load.
- GICD_ISENABLER5 at 0x107fff9114, 32-bit volatile load.
- GICD_ISPENDR5 at 0x107fff9214, 32-bit volatile load.
- GICD_ISACTIVER5 at 0x107fff9314, 32-bit volatile load.
- GICC_HPPIR at 0x107fffa018, 32-bit volatile load.

No writes are allowed by this contract.

## Preflight Checks

The follow-up diagnostic may only report the preflight state. It must not
promote any result to GPIO ownership, event-generation readiness, interrupt
pending generation, or interrupt delivery. The report should decode:

- GPIO16 CTRL FUNCSEL, function name, OUTOVER, OEOVER, INOVER, raw
  event-enable bits, filtered event-enable bits, and IRQOVER.
- GPIO16 STATUS raw and filtered event-status bits.
- IO_BANK0 INTE/INTS GPIO16 bit 0x00010000 and nonzero masks.
- RIO0 OUT/OE/IN GPIO16 bit 0x00010000.
- GPIO16 pad input-enable, output-disable, pull, drive, schmitt, and slew
  fields.
- GIC INTID 160 bank/bit state from GICD_ISENABLER5, GICD_ISPENDR5,
  GICD_ISACTIVER5, and GICC_HPPIR.

The conservative pass-shaped observation for later supervisor planning is:

- GPIO16 FUNCSEL reports GPIO fsel 5, or another explicitly
  supervisor-accepted GPIO/RIO-compatible function.
- Parent route INTID 160 is not enabled, pending, or active in GIC status
  reads, and HPPIR does not report INTID 160.
- IO_BANK0 INTE/INTS and GPIO16 STATUS do not show existing source-enable,
  source-status, or event-status bits requiring write-backed cleanup.
- The diagnostic reaches a terminal marker with all fields present.

Any other state is retained evidence and should be classified as a preflight
blocker or warning, not fixed in place.

## Classifications

- observed-gpio16-ownership-event-preflight-visible
- observed-gpio16-ownership-preflight-blocked-non-gpio-function
- observed-gpio16-ownership-preflight-blocked-route-or-source-state
- observed-gpio16-ownership-preflight-sentinel
- observed-gpio16-ownership-preflight-all-ones
- observed-gpio16-ownership-preflight-zero
- observed-gpio16-ownership-preflight-no-return-or-trap
- observed-gpio16-ownership-preflight-inconclusive-capture
- no-mmio-observed-gpio16-ownership-event-control-visible
- staging/build-blocker

## Report Fields

- contract
- target
- pin
- gpio16-bit-mask
- gpio16-status-address
- gpio16-ctrl-address
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
- gpio16-status-raw
- gpio16-ctrl-raw
- gpio16-funcsel
- gpio16-func-name
- gpio16-outover
- gpio16-oeover
- gpio16-inover
- gpio16-raw-event-enable-mask
- gpio16-filtered-event-enable-mask
- gpio16-status-event-mask
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
must quarantine the result under the sentinel/all-ones/zero, no-return/trap,
candidate/capture, or staging classifications and must not add retry writes,
reset writes, interrupt acknowledgement, pinmux changes, or parent-route
changes in the same task.

## Control Requirement

Before any real Pi 5 observed GPIO16 ownership/event preflight proof, a paired
no-MMIO/no-RP1/no-GIC control must be accepted locally/static and then on Pi 5.
The control must branch from the same early entry point, preserve the same
serial/output shape and classification field, construct no RP1 GPIO/RIO/pads
or clock/reset, MSI-X/PCIe/MIP, or GIC MMIO address, perform no volatile load or
store to those paths, and emit simulated zero raw values plus a terminal marker
suitable for the repaired V3/run-unique and boot-staging identity checkers.

## Forbidden Operations

- GPIO16 CTRL, CTRL SET, or CTRL CLR writes.
- IO_BANK0 INTE SET, INTE CLR, or INTE RW writes.
- GPIO IRQRESET acknowledgement writes.
- RIO0 OUT/OE/IN writes or aliases.
- GPIO16 pad writes.
- GPIO14 or GPIO15 function changes or ownership attempts.
- Parent-route masking writes.
- GIC IAR/EOIR acknowledgement or any GIC write.
- MSI-X, PCIe config, MIP, clock/reset, DMA/cache, storage, generated-root,
  networking, SSH, Milestone 11.3, or phase transition work.

## Accepted Claims

This task accepts only the source-backed observed-aperture register identity,
read-only report shape, classification boundary, and paired
no-MMIO/no-RP1/no-GIC control requirement for GPIO16 ownership/event preflight.
It does not accept GPIO ownership, event generation, interrupt pending
generation, interrupt delivery, GIC acknowledgement, handler ownership,
GPIO/RIO/pad/INTE/CTRL writes, GPIO14 ownership changes, parent-route masking
writes, DMA/cache, networking, SSH, Milestone 11.3, or a phase transition.

## Validation

- Static source/doc inspection: retained in
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-source-contract/source-register-evidence.md.
- jq empty on task-owned evidence-map/classification JSON: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Result

Accepted as accepted-observed-gpio16-ownership-event-source-contract. The next
mechanically unblocked task is
phase11-rp1-observed-gpio16-ownership-event-core-20260609 if this task remains
committed and the queued dependencies remain satisfied.
