# Task: Phase 11 RP1 GPIO Owned Event Discriminator Source Contract

Task ID: phase11-rp1-gpio-owned-event-discriminator-source-contract-20260607

Status: accepted

Classification: accepted-source-contract

Evidence level: static source/doc inspection

## Goal

Define the smallest source-backed Talos-owned RP1 GPIO event/pending
discriminator after the GPIO14 non-GPIO-function blocker.

## Scope

- Reviewed the accepted GPIO14 ownership/restore closeout, retained
  Raspberry Pi Linux RP1 pinctrl/GPIO/IRQ references, Pi 5 lab UART usage, and
  current Talos RP1 diagnostic helpers.
- Selected exactly one candidate pin and bounded event/pending discriminator:
  GPIO16 level-high event/source-status under a parent-route containment
  preflight.
- Named exact allowed reads/writes, ordering, cleanup and partial-failure
  restore requirements, report fields, classifications, paired no-MMIO
  control requirements, and forbidden operations.
- Recorded findings with disposition.
- Updated only roadmap/project contract docs for the accepted source-contract
  frontier.

## Non-Goals

No runtime implementation, hardware run, boot archive publication,
hardwareTestLock acquisition, GPIO14 event-generation retry, same-shaped
GPIO14 ownership/route preflight rerun, interrupt delivery, GIC IAR/EOIR
acknowledgement, ISR installation, broad GPIO driver ownership, unbounded
pin-control/pad/RIO writes, clock/reset programming, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3, or
phase transition.

## Findings And Disposition

- fixed: selected GPIO16, not GPIO14, because the Pi 5 source line name is
  generic GPIO16, no retained fixed board consumer references it, Talos lab
  UART usage is on UART10 or the already-quarantined RP1 UART0 GPIO14/GPIO15
  path, and the GPIO16 fsel table includes GPIO at fsel 5 and proc_rio at
  fsel 6.
- fixed: retained GPIO16 register identity from source: STATUS
  0x1f000d0080, CTRL 0x1f000d0084, CTRL SET 0x1f000d2084, CTRL CLR
  0x1f000d3084, pad control 0x1f000f0044, and bank0 mask 0x00010000.
- fixed: defined parent-route containment as a required read-only GIC
  preflight: INTID 160 must not be enabled, pending, active, or visible in
  HPPIR before any GPIO/RIO/pad/INTE write is attempted.
- fixed: selected one discriminator shape: snapshot GPIO16/IO_BANK0/RIO/pad
  state, configure GPIO16 as a GPIO/RIO output with input enabled, clear
  latched events, enable a raw level-high event, set the IO_BANK0 GPIO16
  source-enable bit only while the parent route remains contained, drive the
  RIO output high, read GPIO16 STATUS and IO_BANK0 INTS, then restore the
  exact preflight state.
- fixed: defined the paired no-MMIO/no-RP1/no-GIC control requirement for the
  later hardware proof.
- deferred: interrupt delivery, GIC acknowledgement, ISR/handler ownership,
  broad GPIO ownership, arbitrary header-pin ownership, clock/reset
  programming, and any GPIO14 event-generation retry remain future work.
- not-an-issue: GPIO14 remains blocked; this contract uses GPIO16 and keeps
  all GPIO14 CTRL/INTE/RIO/pad writes forbidden.

No findings were removed in this source-contract task.

## Contract Summary

Accepted contract id:
phase11-rp1-gpio-owned-event-discriminator-source-contract-v1.

~~~text
name: rp1-gpio16-owned-level-high-event-discriminator
pin: GPIO16
bank: IO_BANK0
bank bit mask: 0x00010000
source route: RP1 IO_BANK0 hwirq 0, source-predicted GIC SPI 128 / INTID 160
operation: bounded GPIO16 write/read discriminator with exact restore
~~~

GPIO16 source-backed fsel table:

~~~text
fsel 0: spi1
fsel 1: dpi
fsel 2: dsi0_te_ext
fsel 3: _
fsel 4: uart0
fsel 5: gpio
fsel 6: proc_rio
fsel 7: pio
fsel 8: _
~~~

GPIO16 is selected because retained Pi 5 device-tree line names call it
GPIO16, retained fixed board consumers do not reference RP1 GPIO16, and it is
not the GPIO14/GPIO15 RP1 UART0 pair used by prior Talos diagnostics.

## Allowed Reads

- GPIO16 STATUS at 0x1f000d0080, 32-bit volatile load.
- GPIO16 CTRL at 0x1f000d0084, 32-bit volatile load.
- IO_BANK0 INTE at 0x1f000d011c, 32-bit volatile load.
- IO_BANK0 INTS at 0x1f000d0124, 32-bit volatile load.
- RIO0 OUT at 0x1f000e0000, 32-bit volatile load.
- RIO0 OE at 0x1f000e0004, 32-bit volatile load.
- RIO0 IN at 0x1f000e0008, 32-bit volatile load.
- GPIO16 pad control at 0x1f000f0044, 32-bit volatile load.
- GICD_ISENABLER5 at 0x107fff9114, 32-bit volatile load.
- GICD_ISPENDR5 at 0x107fff9214, 32-bit volatile load.
- GICD_ISACTIVER5 at 0x107fff9314, 32-bit volatile load.
- GICC_HPPIR at 0x107fffa018, 32-bit volatile load.

## Allowed Writes

Only these writes are accepted, and only in the ordering below:

1. Snapshot GPIO16 CTRL, IO_BANK0 INTE, RIO0 OUT/OE/IN, GPIO16 pad control,
   GPIO16 STATUS, IO_BANK0 INTS, and INTID 160 GIC status.
2. Abort before any write unless INTID 160 is disabled, not pending, not
   active, and HPPIR is not INTID 160.
3. Clear IO_BANK0 INTE bit 16 through IO_BANK0 INTE CLR at 0x1f000d311c with
   mask 0x00010000.
4. Configure GPIO16 pad through 0x1f000f0044 so input is enabled and output is
   not disabled, retaining other pad fields unless the later implementation
   records an exact source-backed mask/update.
5. Configure GPIO16 CTRL through 0x1f000d0084 for GPIO fsel 5 with peripheral
   output-enable/input/output overrides suitable for RIO control, retaining
   unrelated fields except the event-enable and override fields explicitly
   named by this contract.
6. Drive GPIO16 low and enable RIO output with RIO0 OUT/OE set/clear alias
   writes for mask 0x00010000.
7. Clear raw event-enable bits with GPIO16 CTRL CLR at 0x1f000d3084 using
   RP1_INT_MASK << RP1_GPIO_EVENTS_SHIFT_RAW.
8. Clear latched events with GPIO16 CTRL SET at 0x1f000d2084 using
   RP1_GPIO_CTRL_IRQRESET.
9. Enable only the raw level-high event with GPIO16 CTRL SET at
   0x1f000d2084 using RP1_GPIO_CTRL_IRQEN_HIGH.
10. Set IO_BANK0 INTE bit 16 through IO_BANK0 INTE SET at 0x1f000d211c with
    mask 0x00010000.
11. Drive GPIO16 high with RIO0 OUT SET for mask 0x00010000.
12. Read and report GPIO16 STATUS, IO_BANK0 INTE/INTS, RIO0 OUT/OE/IN,
    GPIO16 pad control, and INTID 160 GIC status.
13. Restore in reverse containment order: clear IO_BANK0 INTE bit 16, drive
    GPIO16 low, write IRQRESET, restore GPIO16 CTRL, restore RIO0 OUT/OE bit
    16 to its snapshot state, restore GPIO16 pad control, restore IO_BANK0
    INTE bit 16 to its snapshot state, then read/report post-restore
    GPIO16/IO_BANK0/RIO/pad/GIC state.

No GIC, MSI-X, PCIe config, MIP, clock/reset, GPIO14, GPIO15, or unrelated
GPIO writes are allowed.

## Report Fields

The later implementation must report:

- contract id, target, selected pin, bank, bit mask, register addresses, and
  width.
- preflight GPIO16 CTRL/STATUS, IO_BANK0 INTE/INTS, RIO0 OUT/OE/IN, pad, and
  INTID 160 GIC status.
- action fields for each accepted write class and whether the write path was
  skipped by a preflight abort.
- post-action GPIO16 STATUS raw and decoded raw/filtered event-status masks,
  IO_BANK0 INTE/INTS raw values and GPIO16 bit, RIO0 OUT/OE/IN GPIO16 bits,
  pad input-enable/output-disable fields, and INTID 160 GIC status.
- post-restore GPIO16 CTRL/STATUS, IO_BANK0 INTE/INTS, RIO0 OUT/OE/IN, pad,
  and INTID 160 GIC status.
- one terminal classification.

Accepted classifications:

- gpio16-owned-level-high-event-discriminator-visible
- gpio16-owned-event-preflight-blocked-parent-route
- gpio16-owned-event-preflight-blocked-pin-function
- gpio16-owned-event-preflight-blocked-restore-mismatch
- gpio16-owned-event-preflight-blocked-missing-status-transition
- gpio16-owned-event-preflight-inconclusive-capture

## Control Requirement

The paired control must preserve the same serial/output shape and terminal
classifications while constructing no RP1 GPIO/RIO/pads/clock/reset,
MSI-X/PCIe/MIP, or GIC MMIO address and performing no volatile load or store
to those paths. It must emit simulated zero preflight/action/post/restore
fields and a terminal control classification before any real Pi 5 proof is
authorized.

## Forbidden Operations

Forbidden operations include GPIO14 or GPIO15 writes; any GIC write; GIC
IAR/EOIR acknowledgement; MSI-X, PCIe config, or MIP writes; clock/reset
programming; writes to any pin other than GPIO16; enabling interrupt delivery;
installing an ISR/handler; broad GPIO driver ownership; DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3, or
phase transition.

## Accepted Claims

This task accepts only the source contract for a bounded GPIO16
event/source-status discriminator and its required control. It does not accept
hardware behavior, GPIO event generation on Pi 5, interrupt delivery, GIC
acknowledgement, ISR/handler ownership, broad GPIO ownership, clock/reset
programming, DMA/cache, storage, generated-root, networking, SSH, broader
PCIe enumeration, Milestone 11.3, or a phase transition.

## Validation

- Static source/doc inspection: retained in
  tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-source-contract/source-reference-notes.md.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Result

Accepted as accepted-source-contract. The next mechanically unblocked task is
phase11-rp1-gpio-owned-event-discriminator-core-20260607 if this task remains
committed and the task's queued dependencies remain satisfied.
