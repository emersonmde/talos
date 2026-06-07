# Phase 11 RP1 GPIO Status Diagnostic Core

Task id: phase11-rp1-gpio-status-diagnostic-core-20260607

Status: accepted

## Goal

Implement the local/static real candidate and paired no-MMIO control for the
source-backed Milestone 11.2 RP1 GPIO14 STATUS diagnostic selected by
phase11-rp1-irq-clock-gpio-contract-v1.

## Scope

- Added boot scenarios rpi5_rp1_gpio14_status_read and
  rpi5_rp1_gpio14_status_no_mmio_control.
- Routed both scenarios directly from rust_entry, before BootInfo parsing,
  target initialization, boot reports, memory planning, allocator setup,
  scheduler work, command-loop work, or broader Phase 11 behavior.
- Added a real RP1 GPIO14 STATUS candidate that performs one 32-bit volatile
  load from CPU physical 0x1f000d0070 and, if the load returns, loops forever
  reporting the raw value, interpreted raw/filtered status bits, and
  classification=diagnostic-result-visible.
- Added a matching no-MMIO control candidate that constructs no contracted RP1
  GPIO/RIO/PADS/clock address, performs no contracted RP1 MMIO, and loops
  forever reporting the same field shape with address=not-constructed,
  simulated raw value 0, and classification=simulated/control.
- Added task-owned image, boot-tree, archive, and archive-review scripts for
  both candidates.
- Produced non-published candidate archives:
  target/talos-rpi5-rp1-gpio14-status-read-core.tar.gz and
  target/talos-rpi5-rp1-gpio14-status-no-mmio-control-core.tar.gz.

## Non-Goals

No Pi 5 hardware run, boot archive publication, hardwareTestLock acquisition,
GPIO ownership, pinmux change, pad write, clock/reset programming, interrupt
enable/handling, DMA/cache, Ethernet, networking, SSH, storage,
generated-root, broader PCIe enumeration, phase transition, or hardware
behavior acceptance.

## Candidates

RP1 GPIO14 status candidate:
target/talos-rpi5-rp1-gpio14-status-read-core.tar.gz

- Archive SHA-256:
  7bc21b39a5d0150221a244701285d733c8faef4e153085a49a34b5069c1fecea
- Boot-tree identity:
  2f9b2d308af2b7db1b0099e8910457be008ed98a4abfc75ec5b6b06432e49e6a
- Kernel SHA-256:
  e95dbb1cf6e3296bc29832c69115afadf78622c0995c4734638dd249f8867efc
- Kernel size: 46,336 bytes
- Arm64 Image fields: text_offset=0, header_image_size=46336, flags=12,
  magic=ARMd
- Repeated marker: TALOS: gpio14-status-result

No-MMIO control archive:
target/talos-rpi5-rp1-gpio14-status-no-mmio-control-core.tar.gz

- Archive SHA-256:
  0f3b084d686a12d101b5f43d6f8b264a57eee7241790e6ab87577454dd6ac611
- Boot-tree identity:
  6bfc9797b0efe254d4a02fe4569064f565281f77191ac56b87351a92c2b7160b
- Kernel SHA-256:
  148fd0e7dada9646be1c4c74796e13bea535bc4f03ec11330aa2cab237600b52
- Kernel size: 46,160 bytes
- Arm64 Image fields: text_offset=0, header_image_size=46160, flags=12,
  magic=ARMd
- Repeated marker: TALOS: gpio14-status-control

## Static Path

The real candidate reports rpi5-rp1-gpio14-status-read: before-rp1-load,
flushes UART10, then performs one volatile 32-bit load from 0x1f000d0070.
Assembly review shows the selected function constructs x19 = 0x1f000d0070 and
has exactly one ldr w8, [x19]. The terminal loop repeatedly reports contract id
phase11-rp1-irq-clock-gpio-contract-v1, target rp1-gpio14-status-read, address,
width 32, the retained raw value, raw/filtered falling/rising/low/high bits,
and classification=diagnostic-result-visible.

The no-MMIO control candidate reports
rpi5-rp1-gpio14-status-control: no-rp1-mmio, then loops forever emitting
TALOS: gpio14-status-control with the same field shape, address=not-constructed,
raw value 0, status bit fields, and classification=simulated/control. Assembly
review shows no contracted RP1 address construction and zero contracted RP1
loads/stores; the retained ldr w10, [x9, #0x18] instructions are UART10 FR
polling in the early serial flush path.

## Findings And Disposition

- fixed: added the exact read-only GPIO14 STATUS diagnostic accepted by the
  source contract.
- fixed: added a paired no-MMIO control preserving the serial output field
  shape without constructing the contracted RP1 address.
- fixed: retained task-owned archive review, candidate identity, image/header
  metadata, strings, and assembly evidence for both candidates.
- deferred: Pi 5 visibility of the no-MMIO control and real RP1 GPIO14 STATUS
  candidate remains queued hardware work.
- not-an-issue: GPIO14 may be muxed as UART0 TXD; this task reads status only
  and does not claim GPIO ownership or modify pin control.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-diagnostic-core/evidence-map.json.
- Static inspection:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-diagnostic-core/static-inspection.md.
- Assembly review:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-diagnostic-core/assembly-review.txt.
- Archive reviews:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-diagnostic-core/rp1-archive-review.txt and
  tasks/evidence/2026-06-07-phase11-rp1-gpio-status-diagnostic-core/control-archive-review.txt.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- image/archive inspection: both task-owned archive review scripts passed.
- static disassembly/source inspection: passed.
- git diff --check: passed.
- mdbook build: not run; no docs/src files touched.
- git diff --cached --check before commit: passed.

## Result

Accepted as local-static-gpio14-status-core-accepted.

The next queued task is the serialized no-MMIO Pi 5 control. Real RP1 GPIO14
STATUS hardware behavior, GPIO ownership, pinmux behavior, pad writes,
interrupts, clock/reset programming, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, and phase transition remain unaccepted.
