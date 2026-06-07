# Phase 11 RP1 Interrupt-Routing Diagnostic Core

Task id: phase11-rp1-interrupt-routing-diagnostic-core-20260607

Status: accepted

## Goal

Implement the accepted interrupt-routing diagnostic core and paired
no-MMIO/no-enable control locally, producing candidate artifacts but no
hardware run.

## Scope

- Added boot scenarios rpi5_rp1_interrupt_routing_msix_cfg_read and
  rpi5_rp1_interrupt_routing_no_mmio_control.
- Routed both scenarios directly from rust_entry, before BootInfo parsing,
  target initialization, boot reports, memory planning, allocator setup,
  scheduler work, command-loop work, or broader Phase 11 behavior.
- Added a real RP1 interrupt-routing candidate that performs one 32-bit
  volatile load from CPU physical 0x1f00108008 and, if the load returns, loops
  forever reporting contract id, target, hwirq, predicted MSI-X/GIC fields,
  address, width, raw MSIX_CFG value, decoded enable/test/iack/iack-en bits,
  and classification=routing-msix-cfg-visible.
- Added a matching no-MMIO/no-enable control candidate that constructs no
  forbidden RP1 interrupt/GPIO/pads/RIO/clock/reset/MSI-X/PCIe-config/MIP/GIC
  address, performs no forbidden MMIO, and loops forever reporting the same
  field shape with address=not-constructed, simulated raw value 0, and
  classification=simulated/control.
- Added task-owned image, boot-tree, archive, and archive-review scripts for
  both candidates.
- Produced non-published candidate archives:
  target/talos-rpi5-rp1-interrupt-routing-msix-cfg-read-core.tar.gz and
  target/talos-rpi5-rp1-interrupt-routing-no-mmio-control-core.tar.gz.

## Non-Goals

No Pi 5 hardware run, boot archive publication, hardwareTestLock acquisition,
broad driver structure, GPIO ownership, pin-control writes, pad writes,
clock/reset programming, interrupt enablement/delivery, MSI-X enable/IACK
writes, DMA/cache, Ethernet, networking, SSH, storage, generated-root, broader
PCIe enumeration, Milestone 11.3, phase transition, or hardware behavior
acceptance.

## Candidates

RP1 interrupt-routing candidate:
target/talos-rpi5-rp1-interrupt-routing-msix-cfg-read-core.tar.gz

- Archive SHA-256:
  09c5f12dcf9a6d52d4ba265e2a6cfcb1c2797a278e342e59f08edba8a27de00b
- Kernel SHA-256:
  f076867595f93b71632c09b058d55177e9814f195bc6e102e63725c26d153748
- Kernel size: 46,648 bytes
- Arm64 Image fields: text_offset=0, header_image_size=46648, flags=12,
  magic=ARMd
- Repeated marker: TALOS: rp1-interrupt-routing-result

No-MMIO/no-enable control archive:
target/talos-rpi5-rp1-interrupt-routing-no-mmio-control-core.tar.gz

- Archive SHA-256:
  df030dd8c696e9cb5f3bd8a28abaa3fc6584d2d540811e29b2c37b3bf156668c
- Kernel SHA-256:
  114c1e76a54f30c6f49f39d2418c0974b52a8af63b74ccf99f034b1f7df6a154
- Kernel size: 46,520 bytes
- Arm64 Image fields: text_offset=0, header_image_size=46520, flags=12,
  magic=ARMd
- Repeated marker: TALOS: rp1-interrupt-routing-control

## Static Path

The real candidate reports
rpi5-rp1-interrupt-routing-msix-cfg-read: before-rp1-load, flushes UART10, then
performs one volatile 32-bit load from 0x1f00108008. Assembly review shows the
selected function constructs x19 = 0x1f00108008 and has exactly one
ldr w22, [x19]. The terminal loop repeatedly reports contract id
phase11-rp1-interrupt-routing-source-contract-v1, target
rp1-io-bank0-msix-cfg-read, hwirq 0, predicted MSI-X vector 0, predicted GIC
SPI 128 / INTID 160, address, width 32, the retained raw value, decoded
MSIX_CFG bits, and classification=routing-msix-cfg-visible.

The no-MMIO/no-enable control candidate reports
rpi5-rp1-interrupt-routing-control: no-rp1-msix-pcie-gic-mmio, then loops
forever emitting TALOS: rp1-interrupt-routing-control with the same field
shape, address=not-constructed, raw value 0, decoded bit fields, and
classification=simulated/control. Assembly review shows no forbidden
RP1/MSI-X/PCIe/GIC address construction and zero forbidden loads/stores; the
retained ldr w10, [x9, #0x18] instructions are UART10 FR polling in the early
serial flush path.

## Findings And Disposition

- fixed: added the exact read-only/no-enable MSIX_CFG(0) diagnostic accepted by
  the source contract.
- fixed: added a paired no-MMIO/no-enable control preserving the serial output
  field shape without constructing the contracted RP1/MSI-X/PCIe/GIC address.
- fixed: retained task-owned archive review, candidate identity, image/header
  metadata, strings, and assembly evidence for both candidates.
- deferred: Pi 5 visibility of the no-MMIO/no-enable control and real RP1
  interrupt-routing candidate remains queued hardware work.
- not-an-issue: predicted MSI-X/GIC fields are report metadata only; this task
  does not accept interrupt delivery, GPIO ownership, or clock/reset state.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-diagnostic-core/evidence-map.json.
- Static inspection:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-diagnostic-core/static-inspection.md.
- Assembly review:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-diagnostic-core/assembly-review.txt.
- Archive reviews:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-diagnostic-core/rp1-archive-review.txt and
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-diagnostic-core/control-archive-review.txt.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- image/archive inspection: both task-owned archive review scripts passed.
- static disassembly/source inspection: passed.
- git diff --check: passed.
- mdbook build: not run; no docs/src files touched.
- git diff --cached --check before commit: passed.

## Result

Accepted as local-static-interrupt-routing-core-accepted.

The next queued task is the serialized no-MMIO/no-enable Pi 5 control. Real RP1
interrupt-routing hardware behavior, GPIO ownership, pin-control behavior, pad
writes, interrupt enablement/delivery, MSI-X enable/IACK writes, PCIe MSI
delivery, GIC delivery, clock/reset programming, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.3, and phase
transition remain unaccepted.
