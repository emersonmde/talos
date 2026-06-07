# Phase 11 RP1 GPIO Ownership/Restore Core Static Inspection

Task: phase11-rp1-gpio-ownership-restore-core-20260607

Evidence level: static source, archive, strings, and disassembly inspection.

## Inputs

- Real archive review:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-core/real-archive-review.txt
- Control archive review:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-core/control-archive-review.txt
- Real strings:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-core/real-candidate-strings.txt
- Control strings:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-core/control-candidate-strings.txt
- Real disassembly:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-core/real-gpio14-ownership-preflight-asm.txt
- Control disassembly:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-core/control-gpio14-ownership-preflight-asm.txt
- Static grep summary:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-core/static-grep-summary.txt

## Real Candidate Boundary

- run_rp1_gpio14_ownership_route_preflight_read implements only the accepted
  phase11-rp1-gpio-ownership-restore-source-contract-v1 target
  rp1-gpio14-ownership-route-preflight-read.
- The selected function constructs GPIO14 STATUS/CTRL, IO_BANK0 INTE/INTS,
  RIO0 OUT/OE/IN, GPIO14 pad control, GICD_ISENABLER5, GICD_ISPENDR5,
  GICD_ISACTIVER5, and GICC_HPPIR read addresses.
- The selected function performs the contracted 32-bit volatile RP1 loads and
  uses the existing GicV2 read-only status helpers for INTID 160 status.
- The report loop prints the accepted fields: contract id, target, pin,
  selected addresses, width, raw values, GPIO14 function/override/event
  decodes, IO_BANK0 source decodes, RIO/pad decodes, INTID 160 route status,
  HPPIR INTID, and classification.
- No GPIO CTRL writes, IO_BANK0 INTE writes, IRQRESET acknowledgement, RIO
  writes, pad writes, GIC writes, GICC_IAR reads, GICC_EOIR writes, interrupt
  unmasking, ISR installation, MSI-X enable/IACK writes, PCIe/MIP writes,
  clock/reset writes, DMA/cache, storage, generated-root, networking, SSH,
  broader PCIe, or Milestone 11.3 path is part of this candidate.

## Control Boundary

- run_rp1_gpio14_ownership_route_preflight_no_mmio_control emits the paired
  marker TALOS: rp1-gpio14-ownership-route-preflight-control.
- It preserves the field shape and classification field while reporting
  not-constructed address fields and simulated zero raw values.
- The control image strings omit the real diagnostic marker and forbidden
  selected RP1/GIC address strings checked by the archive review helper.
- The control function disassembly constructs only code/rodata and UART10
  serial addresses. The recurring 0x10_7d00_1000 address and [x9, #0x18]
  loads are the existing UART10 FR flush path; [x9] stores are UART10 DR
  serial output.
- Static grep of the selected control strings and disassembly found no
  forbidden 0x1f000d0070, 0x1f000d0074, 0x1f000d011c, 0x1f000d0124,
  0x1f000e0000, 0x1f000e0004, 0x1f000e0008, 0x1f000f003c, 0x107fff9114,
  0x107fff9214, 0x107fff9314, 0x107fffa018, 0x1f00108008, 0x107fff9000, or
  0x107fffa000 construction.

## Disposition

- fixed: added explicit Pi 5 boot scenarios for real and no-MMIO GPIO14
  ownership/route preflight candidates.
- fixed: real candidate implements the exact read-only GPIO14 ownership/route
  preflight snapshot boundary.
- fixed: control candidate preserves output/capture shape while avoiding
  forbidden RP1 GPIO/RIO/pads/clock/reset/MSI-X/PCIe/MIP/GIC MMIO address
  construction.
- deferred: serialized Pi 5 control proof, real Pi 5 preflight proof, GPIO
  event generation, interrupt enablement, parent interrupt delivery, GIC
  acknowledgement, ISR/handler ownership, broad GPIO ownership, clock/reset
  programming, DMA/cache, storage, generated-root, networking, SSH, broader
  PCIe enumeration, Milestone 11.3, and phase transition.
- not-an-issue: UART10 FR/DR polling remains the required serial output path.
