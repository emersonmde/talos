# Task: Phase 11 RP1 GPIO Bank Source-Status Core

Task ID: phase11-rp1-gpio-bank-source-status-core-20260607

Status: accepted

Evidence level: static/archive inspection, fmt/lint/typecheck, unit tests

## Goal

Implement the accepted read-only RP1 GPIO bank source-status diagnostic core
and paired no-MMIO/no-RP1/no-GIC control locally, producing candidate artifacts
but no hardware run.

## Scope

- Used only the target, register reads, report fields, forbidden operations,
  and classifications accepted by
  phase11-rp1-gpio-bank-source-status-contract-20260607.
- Implemented the real candidate as the smallest read-only IO_BANK0 source
  snapshot: one INTE load and one INTS load.
- Implemented the paired control candidate with the same serial/output shape,
  simulated zero raw values, and no constructed RP1/GIC/MSI-X/PCIe/MIP/GPIO/
  pads/RIO/clock/reset MMIO addresses.
- Retained static/archive evidence for the real and control candidate
  boundary.

## Non-Goals

No Pi 5 hardware run, hardwareTestLock acquisition, published boot archive,
broad RP1/GPIO/GIC abstraction, GPIO CTRL writes, GPIO INTE writes,
IRQRESET acknowledgement, MSI-X enable/IACK writes, PCIe MSI/MIP/GIC writes,
GIC enable writes, IAR/EOIR acknowledgement, interrupt unmasking, ISR
installation, GPIO ownership, pin-control or pad writes, clock/reset
programming, DMA/cache, storage, generated-root, networking, SSH, broader PCIe
enumeration, Milestone 11.3, or phase transition.

## Findings

- fixed: added two explicit Pi 5 boot scenarios:
  rpi5_rp1_gpio_bank_source_status_read and
  rpi5_rp1_gpio_bank_source_status_no_mmio_control.
- fixed: added IO_BANK0 source-status constants
  RP1_IO_BANK0_INTE = 0x1f_000d_011c and
  RP1_IO_BANK0_INTS = 0x1f_000d_0124, with unit-test coverage for the
  accepted address translation.
- fixed: the real candidate emits
  TALOS: rp1-gpio-bank-source-status-result with the accepted contract id,
  target, source hwirq, bank metadata, selected address fields, width, raw
  INTE and INTS, GPIO14 mask/enable/source-status bits, source-status
  mask/nonzero decode, and classification=gpio-bank-source-status-visible.
- fixed: the real candidate performs only the accepted read operations:
  IO_BANK0_INTE and IO_BANK0_INTS, both as 32-bit volatile loads.
- fixed: the control candidate emits
  TALOS: rp1-gpio-bank-source-status-control with the same output shape,
  not-constructed address fields, simulated zero raw values, and
  classification=simulated/control.
- fixed: control archive and disassembly review prove the control image omits
  the real diagnostic marker, real classification, selected RP1/GIC address
  strings, and forbidden RP1/GIC/MSI-X/PCIe/MIP/GPIO/pads/RIO/clock/reset MMIO
  address construction.
- deferred: Pi 5 control run, Pi 5 real diagnostic run, GPIO event generation,
  interrupt enablement, pending generation, parent interrupt delivery, GIC
  acknowledgement, ISR/handler ownership, GPIO ownership, clock/reset
  programming, DMA/cache, storage, generated-root, networking, SSH, broader
  PCIe enumeration, Milestone 11.3, and phase transition.
- not-an-issue: UART10 FR polling loads/stores remain present in both
  candidates because they are the existing firmware-preserved serial flush
  path, not RP1 GPIO/RIO/pads/clock/reset/MSI-X/PCIe/MIP/GIC MMIO.

No findings were removed in this task.

## Candidate Artifacts

Real candidate:

- Archive: target/talos-rpi5-rp1-gpio-bank-source-status-read-core.tar.gz
- Archive SHA256:
  52b71d101dce9884499bc94fcb3cce2121f5ec065881087b77ff6748e4e4a82b
- kernel_2712.img SHA256:
  54f57ef743fb1bc3e760dc6f13f5d6a9485efb71027834b13f32194cd24228cf
- kernel_2712.img size: 46904 bytes
- Marker: TALOS: rp1-gpio-bank-source-status-result

Control candidate:

- Archive:
  target/talos-rpi5-rp1-gpio-bank-source-status-no-mmio-control-core.tar.gz
- Archive SHA256:
  e3115c88b4d172df85169f95da399b18f41c5e827caaae08ac49aa2bd5a63a6f
- kernel_2712.img SHA256:
  e90c3508db01f03bf1572c8b42ac99111010c8e41077320ddb59f23b7ee67597
- kernel_2712.img size: 46832 bytes
- Marker: TALOS: rp1-gpio-bank-source-status-control

These are local/static artifacts only. No hardware behavior is accepted.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass, 423 talos no_std tests.
- Archive review:
  scripts/rpi5-rp1-gpio-bank-source-status-read-review.sh passed.
- Archive review:
  scripts/rpi5-rp1-gpio-bank-source-status-no-mmio-control-review.sh passed.
- Static disassembly/source inspection: passed; real candidate constructs
  IO_BANK0 INTE at 0x1f000d011c and reads INTE plus INTS at +0x8; control
  candidate constructs no forbidden RP1/GIC/MSI-X/PCIe/MIP/GPIO/pads/
  RIO/clock/reset MMIO address.
- git diff --check: pass.
- mdbook build: not run; no docs/src files were touched.
- git diff --cached --check: pass.

## Result

Accepted. This accepts only the local/static real candidate, the local/static
no-MMIO/no-RP1/no-GIC control candidate, and their archive/static boundary. It
does not accept Pi 5 hardware behavior, GPIO event generation, interrupt
pending generation, interrupt enablement or delivery, GIC IAR/EOIR
acknowledgement, ISR/handler ownership, GPIO ownership, clock/reset
programming, DMA/cache, storage, generated-root, networking, SSH, broader
PCIe, Milestone 11.3, or a phase transition.

## Follow-Up

Promote phase11-rp1-gpio-bank-source-status-control-pi5-20260607 only after
this task is accepted and committed and hardwareTestLock remains
unlocked/restored. The next task owns the serialized Pi 5 no-MMIO/no-RP1/no-GIC
control proof before any real GPIO bank source-status diagnostic run.
