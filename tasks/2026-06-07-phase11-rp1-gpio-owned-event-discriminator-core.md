# Task: Phase 11 RP1 GPIO Owned Event Discriminator Core

Task ID: phase11-rp1-gpio-owned-event-discriminator-core-20260607

Status: accepted

Evidence level: static/archive inspection, fmt/lint/typecheck, unit tests

## Goal

Implement the accepted Talos-owned RP1 GPIO event/pending discriminator core
and paired no-MMIO/no-RP1/no-GIC control locally, producing candidate
artifacts but no hardware run.

## Scope

- Used only the selected GPIO16 pin, allowed operations, ordering, restore
  rules, report fields, classifications, paired control requirements, and
  forbidden operations accepted by
  phase11-rp1-gpio-owned-event-discriminator-source-contract-20260607.
- Implemented the real candidate as a bounded GPIO16 level-high
  event/source-status discriminator with parent-route preflight, exact
  snapshot, accepted write order, post-action report, and restore report.
- Implemented the paired no-MMIO/no-RP1/no-GIC control candidate with the same
  serial/output shape, simulated zero fields, and no forbidden MMIO address
  construction.
- Retained static/archive evidence for the real and control candidate
  boundary.

## Non-Goals

No Pi 5 hardware run, hardwareTestLock acquisition, published boot archive,
GPIO14 event-generation retry, same-shaped GPIO14 preflight rerun, broad GPIO
abstraction, interrupt delivery, GIC IAR/EOIR acknowledgement, ISR
installation, unplanned non-GPIO16 pin-control/pad/RIO writes, clock/reset
programming, DMA/cache, storage, generated-root, networking, SSH, broader
PCIe enumeration, Milestone 11.3, or phase transition.

## Findings

- fixed: added two explicit Pi 5 boot scenarios:
  rpi5_rp1_gpio16_owned_event_discriminator and
  rpi5_rp1_gpio16_owned_event_discriminator_no_mmio_control.
- fixed: added source-backed GPIO16 STATUS/CTRL/CTRL_SET/CTRL_CLR, IO_BANK0
  INTE SET/CLR, RIO OUT/OE SET/CLR, and GPIO16 pad constants for the accepted
  local/static diagnostic.
- fixed: the real candidate performs parent-route containment reads before any
  GPIO16/RIO/pad/INTE write and aborts with the accepted parent-route or
  pin-function classifications when preflight is incompatible.
- fixed: the real candidate performs only the accepted write order:
  IO_BANK0 INTE clear, bounded GPIO16 pad and CTRL update, RIO low/OE setup,
  raw event-enable clear, IRQRESET, raw level-high event enable, IO_BANK0 INTE
  set, RIO high, then reverse-order restore.
- fixed: the real candidate emits
  TALOS: rp1-gpio16-owned-event-discriminator-result with contract id, target,
  pin, register identities, pre/action/post/restore fields, and one accepted
  terminal classification.
- fixed: the control candidate emits
  TALOS: rp1-gpio16-owned-event-discriminator-control with the same report
  shape, not-constructed address fields, simulated zero snapshots, skipped
  action fields, and classification=simulated/control.
- fixed: archive review proves the real image omits the control marker and the
  control image omits the real marker and forbidden selected-address strings.
- deferred: serialized Pi 5 no-MMIO control proof, real Pi 5 GPIO16
  discriminator proof, interrupt delivery, GIC acknowledgement, ISR/handler
  ownership, broad GPIO ownership, clock/reset programming, DMA/cache,
  storage, generated-root, networking, SSH, broader PCIe enumeration,
  Milestone 11.3, and phase transition.
- not-an-issue: UART10 FR/DR polling remains present in both candidates
  because it is the firmware-preserved serial output path, not RP1
  GPIO/RIO/pad/clock/reset/MSI-X/PCIe/MIP/GIC MMIO.

No findings were removed in this task.

## Candidate Artifacts

Real candidate:

- Archive:
  target/talos-rpi5-rp1-gpio16-owned-event-discriminator-core.tar.gz
- Archive SHA256:
  479f5978f1a8fdcc1d8fa2447cbb2919454ec8f648adef5f5640b399b16e6ca5
- kernel_2712.img SHA256:
  585adf175826e6f33e5e2eb727100117b9a1ca1d110115252e336c18a573ff81
- kernel_2712.img size: 52056 bytes
- Marker: TALOS: rp1-gpio16-owned-event-discriminator-result

Control candidate:

- Archive:
  target/talos-rpi5-rp1-gpio16-owned-event-discriminator-no-mmio-control-core.tar.gz
- Archive SHA256:
  b1968a1ff0fa4652f9a156ca48a17958f9b99bda54326fa589deaa74a8576c3f
- kernel_2712.img SHA256:
  8e5f3a6a3c58b23a5e707b2b112e852a706da8c3752748b2a14d82d567a35e80
- kernel_2712.img size: 49480 bytes
- Marker: TALOS: rp1-gpio16-owned-event-discriminator-control

These are local/static artifacts only. No hardware behavior is accepted.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass, 423 talos no_std tests.
- Archive build/review:
  scripts/rpi5-rp1-gpio16-owned-event-discriminator-archive.sh and
  scripts/rpi5-rp1-gpio16-owned-event-discriminator-no-mmio-control-archive.sh
  produced local/static candidate archives.
- Archive review:
  scripts/rpi5-rp1-gpio16-owned-event-discriminator-review.sh and
  scripts/rpi5-rp1-gpio16-owned-event-discriminator-no-mmio-control-review.sh
  passed.
- Static disassembly/source inspection: passed; real candidate constructs the
  accepted GPIO16/RIO/pad/IO_BANK0/GIC status boundary and accepted restore
  path, while the control candidate constructs no forbidden RP1/GIC/MSI-X/
  PCIe/MIP/GPIO/pad/RIO/clock/reset MMIO address.
- git diff --check: pass.
- mdbook build: not run; no docs/src files were touched.
- git diff --cached --check: pass.

## Result

Accepted. This accepts only the local/static real candidate, the local/static
no-MMIO/no-RP1/no-GIC control candidate, and their archive/static boundary. It
does not accept Pi 5 hardware behavior, GPIO event generation, interrupt
pending generation, interrupt enablement or delivery, GIC IAR/EOIR
acknowledgement, ISR/handler ownership, broad GPIO ownership, clock/reset
programming, DMA/cache, storage, generated-root, networking, SSH, broader
PCIe, Milestone 11.3, or a phase transition.

## Follow-Up

Promote phase11-rp1-gpio-owned-event-discriminator-control-pi5-20260607 only
after this task is accepted and committed and hardwareTestLock remains
unlocked/restored. The next task owns the serialized Pi 5 no-MMIO/no-RP1/no-GIC
control proof before any real GPIO16 event discriminator run.
