# Static Implementation Review

Task id: phase11-rp1-observed-gpio-ownership-route-core-20260608

## Source Inspection

- fixed: src/target/rpi5.rs defines observed-aperture constants for the
  accepted RP1 GPIO/RIO/pad read set: 0x1c000d0070, 0x1c000d0074,
  0x1c000d011c, 0x1c000d0124, 0x1c000e0000, 0x1c000e0004, 0x1c000e0008, and
  0x1c000f003c.
- fixed: run_rp1_gpio14_ownership_route_preflight_read uses those constants
  for all RP1 loads, plus the existing read-only GicV2 status helpers for
  INTID 160. It does not call GIC IAR/EOIR helpers and performs no MMIO store.
- fixed: run_rp1_gpio14_ownership_route_preflight_no_mmio_control emits the
  same report shape with not-constructed address fields and does not use the
  RP1/GIC address constants.
- fixed: review scripts require the accepted observed contract id and target
  and reject retained old-contract/control strings.

## Archive Inspection

- image/archive inspection: real archive review passed and retained artifact
  SHA-256, kernel SHA-256, kernel size, and result marker.
- image/archive inspection: no-MMIO control archive review passed and retained
  artifact SHA-256, kernel SHA-256, kernel size, control marker, and forbidden
  string absence.

## Deferred

Pi 5 control and real hardware proofs remain separate queued tasks. GPIO
ownership, event generation, interrupt delivery, GIC acknowledgement,
GPIO/RIO/pad/INTE writes, and parent-route masking writes remain unaccepted.
