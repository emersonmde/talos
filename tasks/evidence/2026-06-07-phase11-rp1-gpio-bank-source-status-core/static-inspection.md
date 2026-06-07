# Phase 11 RP1 GPIO Bank Source-Status Core Static Inspection

Task: phase11-rp1-gpio-bank-source-status-core-20260607

Evidence level: static source, archive, strings, and disassembly inspection.

## Inputs

- Real archive review:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-core/real-archive-review.txt
- Control archive review:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-core/control-archive-review.txt
- Real strings:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-core/real-candidate-strings.txt
- Control strings:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-core/control-candidate-strings.txt
- Real disassembly:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-core/real-gpio-bank-source-status-asm.txt
- Control disassembly:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-core/control-gpio-bank-source-status-asm.txt

## Real Candidate Boundary

- run_rp1_gpio_bank_source_status_read implements only the accepted
  phase11-rp1-gpio-bank-source-status-contract-v1 target
  rp1-io-bank0-source-status-read.
- The selected function constructs IO_BANK0 INTE at 0x1f_000d_011c
  and performs one 32-bit volatile load.
- It then performs one companion 32-bit volatile load from IO_BANK0 INTS
  at 0x1f_000d_0124, encoded in the disassembly as the same base plus
  offset #0x8.
- The report loop prints the accepted fields: contract id, target,
  source-hwirq=0, bank metadata, selected register address fields, width,
  raw INTE and INTS, GPIO14 mask/enable/source-status bits, source-status
  mask/nonzero decode, and classification=gpio-bank-source-status-visible.
- No GPIO CTRL, GPIO INTE set/clear alias, IRQRESET acknowledgement,
  MSI-X enable/IACK, PCIe MSI/MIP/GIC write, GIC enable, IAR/EOIR,
  interrupt unmasking, ISR installation, GPIO ownership, pad/RIO/clock/reset,
  DMA/cache, storage, generated-root, networking, SSH, broader PCIe, or
  Milestone 11.3 path is part of this candidate.

## Control Boundary

- run_rp1_gpio_bank_source_status_no_mmio_control emits the paired marker
  TALOS: rp1-gpio-bank-source-status-control.
- It preserves the field shape and classification field while reporting
  inte-address=not-constructed and ints-address=not-constructed, using
  simulated zero raw values.
- The control image strings omit the real diagnostic marker, real
  classification=gpio-bank-source-status-visible, and forbidden RP1/GIC
  address strings checked by the archive review helper.
- The control function disassembly constructs only code/rodata and UART10
  serial addresses. The recurring 0x10_7d00_1000 address and [x9, #0x18]
  loads are the existing UART10 FR flush path.
- Static grep of the selected control disassembly found no 0x1f,
  0x107fff, 0x11c, 0x124, 0x8008, 0x7fff, or direct forbidden non-UART load
  shape.

## Disposition

- fixed: added the explicit Pi 5 boot scenarios
  rpi5_rp1_gpio_bank_source_status_read and
  rpi5_rp1_gpio_bank_source_status_no_mmio_control.
- fixed: real candidate implements the exact read-only IO_BANK0 INTE and
  INTS snapshot boundary.
- fixed: control candidate preserves output/capture shape while avoiding
  forbidden RP1 GPIO/RIO/pads/clock/reset/MSI-X/PCIe/MIP/GIC MMIO address
  construction.
- deferred: serialized Pi 5 control proof, real Pi 5 source-status proof,
  GPIO event generation, interrupt enablement, parent interrupt delivery,
  GIC acknowledgement, ISR/handler ownership, GPIO ownership, clock/reset
  programming, DMA/cache, storage, generated-root, networking, SSH, broader
  PCIe enumeration, Milestone 11.3, and phase transition.
- not-an-issue: UART10 FR polling remains the required serial flush path.
