# Task: Phase 11 Observed GPIO14 Ownership/Route Source Contract

Task ID: phase11-rp1-observed-gpio-ownership-route-source-contract-20260608

Status: accepted

Classification: accepted-observed-gpio14-ownership-route-source-contract

Evidence level: static source/doc inspection

## Goal

Define the smallest read-only observed-aperture GPIO14 ownership and
parent-route preflight contract before any GPIO writes, event generation, or
interrupt delivery attempt.

## Scope

- Inspected retained RP1/Linux GPIO, RIO, pad, IO_BANK0 source-status, and GIC
  route source evidence.
- Inspected accepted 0x1c UART0 observed-aperture proof, repaired 0x1c GPIO14
  STATUS/CTRL proof, prior 0x1f GPIO14/GPIO16 ownership blockers, accepted
  IO_BANK0/GIC route status evidence, and current roadmap/project docs.
- Selected only read-only observed-aperture preflight loads needed to classify
  GPIO14 function, per-pin status/control, bank source-enable/source-status,
  RIO OUT/OE/IN state, pad control, and accepted parent GIC route status.
- Recorded findings with disposition and updated task/project docs for the
  accepted source contract.

## Non-Goals

No runtime source change, Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, GPIO/RIO/pad/INTE/CTRL writes, IRQRESET,
interrupt unmasking, IAR/EOIR acknowledgement, ISR/handler install, event
generation, interrupt delivery, endpoint config retry, bridge setup write,
DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3, or phase
transition.

Do not treat GPIO14 STATUS/CTRL visibility as GPIO ownership or
event-generation readiness.

## Findings

- fixed: selected rp1-gpio14-ownership-route-observed-aperture-preflight-read
  as a read-only observed 0x1c aperture contract instead of reopening the prior
  source-expected 0x1f ownership/route preflight.
- fixed: retained the accepted observed GPIO14 STATUS/CTRL visibility proof as
  the new boundary. The accepted value ctrl-funcsel=4 decodes through source as
  uart0, so the preflight must be able to block on non-GPIO function without
  attempting a GPIO14 function change.
- fixed: retained source-backed GPIO14 ownership-adjacent fields: CTRL
  FUNCSEL/OUTOVER/OEOVER/INOVER/event-enable/IRQOVER, STATUS event status,
  IO_BANK0 INTE/INTS GPIO14 bit, RIO0 OUT/OE/IN GPIO14 bit, and GPIO14 pad
  input/output fields.
- fixed: translated the selected RP1 GPIO/RIO/pad reads through the accepted
  observed aperture: 0x1c000d0070, 0x1c000d0074, 0x1c000d011c,
  0x1c000d0124, 0x1c000e0000, 0x1c000e0004, 0x1c000e0008, and 0x1c000f003c.
- fixed: retained the accepted parent-route status inputs for INTID 160:
  GICD_ISENABLER5, GICD_ISPENDR5, GICD_ISACTIVER5, and GICC_HPPIR. These
  remain read-only status inputs, not delivery or acknowledgement claims.
- fixed: defined a paired no-MMIO/no-RP1/no-GIC control requirement that
  preserves output shape while constructing no RP1 GPIO/RIO/pads/clock/reset,
  MSI-X/PCIe/MIP, or GIC MMIO addresses.
- deferred: GPIO CTRL writes, IO_BANK0 INTE writes, IRQRESET acknowledgement,
  RIO OUT/OE writes, pad writes, parent-route masking writes, deterministic
  event-source generation, interrupt delivery, and restore-after-write
  semantics remain future supervisor-planned work.
- not-an-issue: prior 0x1f GPIO ownership blockers remain valid retained
  evidence for the source-expected aperture, but they are not the same hardware
  claim as this observed-aperture source contract.

No findings were removed in this task.

## Contract Summary

Accepted contract id:
phase11-rp1-observed-gpio-ownership-route-source-contract-v1.

~~~text
name: rp1-gpio14-ownership-route-observed-aperture-preflight-read
pin-selection rule: GPIO14 only, because accepted frontiers already use
  GPIO14 STATUS/CTRL, IO_BANK0 bit 14, and the IO_BANK0 route to INTID 160.
operation: read-only observed-aperture preflight; no MMIO writes, no event
  generation, no interrupt enablement, no acknowledgement, and no restore
  writes.
~~~

Allowed reads:

- GPIO14 STATUS at 0x1c000d0070, 32-bit volatile load.
- GPIO14 CTRL at 0x1c000d0074, 32-bit volatile load.
- IO_BANK0 INTE at 0x1c000d011c, 32-bit volatile load.
- IO_BANK0 INTS at 0x1c000d0124, 32-bit volatile load.
- RIO0 OUT at 0x1c000e0000, 32-bit volatile load.
- RIO0 OE at 0x1c000e0004, 32-bit volatile load.
- RIO0 IN at 0x1c000e0008, 32-bit volatile load.
- GPIO14 pad control at 0x1c000f003c, 32-bit volatile load.
- GICD_ISENABLER5 at 0x107fff9114, 32-bit volatile load.
- GICD_ISPENDR5 at 0x107fff9214, 32-bit volatile load.
- GICD_ISACTIVER5 at 0x107fff9314, 32-bit volatile load.
- GICC_HPPIR at 0x107fffa018, 32-bit volatile load.

No writes are allowed by this contract.

## Preflight Checks

The follow-up diagnostic may only report the preflight state. It must not
promote any result to GPIO ownership or event-generation readiness. The report
should decode:

- GPIO14 CTRL FUNCSEL, function name, OUTOVER, OEOVER, INOVER, raw
  event-enable bits, filtered event-enable bits, and IRQOVER.
- GPIO14 STATUS raw and filtered event-status bits.
- IO_BANK0 INTE/INTS GPIO14 bit 0x00004000 and nonzero masks.
- RIO0 OUT/OE/IN GPIO14 bit 0x00004000.
- GPIO14 pad input-enable, output-disable, pull, drive, schmitt, and slew
  fields.
- GIC INTID 160 bank/bit state from GICD_ISENABLER5, GICD_ISPENDR5,
  GICD_ISACTIVER5, and GICC_HPPIR.

The conservative pass-shaped observation for later supervisor planning is:

- GPIO14 FUNCSEL reports GPIO fsel 5 or another explicitly
  supervisor-accepted function for event sourcing.
- Parent route INTID 160 is not enabled, pending, or active in GIC status
  reads, and HPPIR does not report INTID 160.
- IO_BANK0 INTE/INTS and GPIO14 STATUS do not show existing source-enable,
  source-status, or event-status bits requiring write-backed cleanup.
- The diagnostic reaches a terminal marker with all fields present.

Any other state is retained evidence and should be classified as a preflight
blocker or warning, not fixed in place.

## Classifications

- observed-gpio14-ownership-route-preflight-visible
- observed-gpio14-ownership-preflight-blocked-non-gpio-function
- observed-gpio14-ownership-preflight-blocked-route-or-source-state
- observed-gpio14-ownership-preflight-sentinel
- observed-gpio14-ownership-preflight-all-ones
- observed-gpio14-ownership-preflight-zero
- observed-gpio14-ownership-preflight-no-return-or-trap
- observed-gpio14-ownership-preflight-inconclusive-capture
- no-mmio-observed-gpio14-ownership-route-control-visible
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
must quarantine the result under the sentinel/all-ones/zero, no-return/trap,
candidate/capture, or staging classifications and must not add retry writes,
reset writes, interrupt acknowledgement, pinmux changes, or parent-route
changes in the same task.

## Control Requirement

Before any real Pi 5 observed GPIO14 ownership/route preflight proof, a paired
no-MMIO/no-RP1/no-GIC control must be accepted locally/static and then on Pi 5.
The control must branch from the same early entry point, preserve the same
serial/output shape and classification field, construct no RP1 GPIO/RIO/pads
/clock/reset, MSI-X/PCIe/MIP, or GIC MMIO address, perform no volatile load or
store to those paths, and emit simulated zero raw values plus a terminal marker
suitable for the repaired run-unique and boot-staging identity checkers.

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

## Why This Is Different

This contract is not a same-shaped GPIO14 STATUS/CTRL rerun. The accepted
repaired proof already established observed 0x1c STATUS/CTRL visibility. This
source contract uses that result as a prerequisite and adds only the read-only
ownership/route preflight fields needed to decide whether a later
supervisor-planned event-generation task is even mechanically safe: GPIO14
function, bank source-enable/source-status, RIO state, pad state, and parent
GIC route status. The task still accepts no GPIO ownership and no hardware
behavior beyond the source contract.

## Accepted Claims

This task accepts only a read-only observed-aperture source contract for one
GPIO14 ownership/route preflight diagnostic and its paired no-MMIO/no-RP1
/no-GIC control requirement. It accepts exact source-backed register
addresses, decoded fields, report shape, classifications, forbidden
operations, and cleanup/quarantine rules for that preflight.

It does not accept GPIO ownership, GPIO event generation, interrupt pending
generation, interrupt enablement or delivery, GIC acknowledgement, ISR/handler
ownership, GPIO CTRL/INTE/RIO/pad writes, parent-route masking writes,
clock/reset programming, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe enumeration, Milestone 11.3, or a phase transition.

## Validation

- Static source/doc inspection: retained in
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-source-contract/source-register-evidence.md.
- jq empty on task-owned evidence-map/classification JSON: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Result

Accepted as a read-only observed-aperture source contract. Follow-up
implementation remains bounded to the explicitly queued local/static core task;
no runtime behavior or hardware behavior is accepted by this task.

## Follow-Up

Promote phase11-rp1-observed-gpio-ownership-route-core-20260608 only if it
remains queued and mechanically unblocked. That task may implement the
local/static real/control candidates only; hardware remains gated on the
separate control and real Pi 5 tasks.
