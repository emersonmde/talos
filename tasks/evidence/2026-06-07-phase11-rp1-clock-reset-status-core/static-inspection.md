# Phase 11 RP1 Clock/Reset Status Core Static Inspection

Task: phase11-rp1-clock-reset-status-core-20260607

Evidence level: static source, archive, strings, and disassembly inspection.

## Inputs

- Real archive review:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-core/real-archive-review.txt
- Control archive review:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-core/control-archive-review.txt
- Real strings:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-core/real-candidate-strings.txt
- Control strings:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-core/control-candidate-strings.txt
- Real disassembly:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-core/real-clock-manager-status-asm.txt
- Control disassembly:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-core/control-clock-manager-status-asm.txt

## Real Candidate Boundary

- run_rp1_clock_manager_status_read implements only the accepted
  phase11-rp1-clock-reset-status-source-contract-v1 target
  rp1-clock-manager-status-read.
- The selected function constructs the accepted RP1 clock manager addresses:
  PLL_SYS_CS at 0x1f_0002_0000 and the seven CLK_* registers under
  0x1f_0001_8000.
- It performs exactly the accepted 32-bit volatile loads before entering the
  serial report loop. The selected disassembly shows the clock-manager address
  construction via 0x1f high-half immediates and the load results stored for
  later reporting.
- The report loop prints the accepted fields: contract id, target, clock
  manager base, selected address/raw register fields, decoded PLL lock/refdiv,
  decoded clock enable/source/aux/divider/status fields, retained GPIO14 and
  GPIO16 fsel 13 blocker context, and
  classification=rp1-clock-manager-status-visible.
- No clock/reset write, divider/source programming, GPIO CTRL/INTE/RIO/pad
  write, GPIO event generation, GIC/MSI-X/PCIe/MIP write, interrupt unmasking,
  delivery acceptance, GIC acknowledgement, ISR installation, DMA/cache,
  storage, generated-root, networking, SSH, broader PCIe, or Milestone 11.3
  path is part of this candidate.

## Control Boundary

- run_rp1_clock_manager_status_no_mmio_control emits the paired marker
  TALOS: rp1-clock-manager-status-control.
- It preserves the field shape and classification field while reporting
  clock-manager-base=not-constructed, per-register address=not-constructed,
  and simulated zero raw values.
- The control image strings omit the real diagnostic marker, real
  classification=rp1-clock-manager-status-visible, and forbidden RP1/GIC
  address strings checked by the archive review helper.
- The control function disassembly constructs only code/rodata and UART10
  serial addresses. The recurring 0x10_7d00_1000 address and [x9, #0x18]
  loads are the existing UART10 FR flush path.
- Static grep of the selected control disassembly found no 0x1f, 0x107fff,
  0x18000, 0x20000, or direct forbidden non-UART load shape.

## Disposition

- fixed: added the explicit Pi 5 boot scenarios
  rpi5_rp1_clock_manager_status_read and
  rpi5_rp1_clock_manager_status_no_mmio_control.
- fixed: real candidate implements the exact read-only RP1 clock manager
  status snapshot boundary.
- fixed: control candidate preserves output/capture shape while avoiding
  forbidden RP1 clock/reset, GPIO/RIO/pads, MSI-X/PCIe/MIP, or GIC MMIO
  address construction.
- deferred: serialized Pi 5 control proof, real Pi 5 clock-manager proof,
  clock/reset writes, GPIO ownership retries, interrupt delivery, handler
  ownership, DMA/cache, storage, generated-root, networking, SSH, broader PCIe
  enumeration, Milestone 11.3, and phase transition.
- not-an-issue: UART10 FR polling remains the required serial flush path.
