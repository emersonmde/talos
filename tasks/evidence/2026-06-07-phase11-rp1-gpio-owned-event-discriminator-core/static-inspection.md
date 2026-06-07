# Static Inspection: GPIO16 Owned Event Discriminator Core

Task: phase11-rp1-gpio-owned-event-discriminator-core-20260607

Evidence level: static source/archive/disassembly inspection.

## Source Boundary

- Real scenario:
  rpi5_rp1_gpio16_owned_event_discriminator.
- Control scenario:
  rpi5_rp1_gpio16_owned_event_discriminator_no_mmio_control.
- Source files:
  build.rs, src/main.rs, src/target/rpi5.rs, and task-owned rpi5-rp1-gpio16
  archive/review scripts.

The real candidate uses the accepted source contract id
phase11-rp1-gpio-owned-event-discriminator-source-contract-v1 and target
rp1-gpio16-owned-level-high-event-discriminator. It snapshots GPIO16 STATUS,
GPIO16 CTRL, IO_BANK0 INTE/INTS, RIO0 OUT/OE/IN, GPIO16 pad, and INTID 160
GIC visible status before writes.

The parent-route preflight is checked before writes. If INTID 160 is enabled,
pending, active, or HPPIR reports INTID 160, the candidate emits
gpio16-owned-event-preflight-blocked-parent-route without writing GPIO16/RIO/
pad/INTE state. If GPIO16 function selection is outside GPIO/proc_rio, it emits
gpio16-owned-event-preflight-blocked-pin-function without falling back to
GPIO14 or any other pin.

When preflight passes, the real candidate performs only the accepted write
order and then restores the exact preflight bit/register state for IO_BANK0
INTE bit 16, GPIO16 CTRL, RIO0 OUT/OE bit 16, and GPIO16 pad. The accepted
classification is limited to visible level-high/source-status transition,
restore mismatch, missing status transition, parent-route blocker, or
pin-function blocker.

## Control Boundary

The control candidate emits the same report shape with not-constructed address
fields, zero simulated snapshots, skipped action fields, and
classification=simulated/control. The control archive review confirms the
real result marker and selected address strings are absent.

## Retained Evidence

- real-archive-review.txt and control-archive-review.txt: archive shape,
  marker, hash, and forbidden-string review.
- real-candidate-strings.txt and control-candidate-strings.txt: retained
  string-level boundary evidence.
- real-gpio16-owned-event-discriminator-asm.txt and
  control-gpio16-owned-event-discriminator-asm.txt: AArch64 disassembly
  extracts from rustup llvm-objdump.
- static-grep-summary.txt: selected real marker and empty control forbidden
  match summary.

## Findings

- fixed: real candidate uses only the accepted GPIO16/RIO/pad/IO_BANK0/GIC
  status boundary and accepted restore path.
- fixed: control candidate preserves the report shape without constructing
  selected RP1/GIC/MMIO address strings or the real marker.
- deferred: serialized Pi 5 control and real proofs remain queued.
- not-an-issue: UART10 MMIO remains in both candidates as the existing serial
  output channel.
