# Task: Phase 11 RP1 GPIO Ownership/Restore Core

Task ID: phase11-rp1-gpio-ownership-restore-core-20260607

Status: accepted

Evidence level: static/archive inspection, fmt/lint/typecheck, unit tests

## Goal

Implement the accepted GPIO ownership/restore diagnostic core and paired
no-MMIO/no-RP1/no-GIC control locally, producing candidate artifacts but no
hardware run.

## Scope

- Used only the diagnostic target, register operations, ownership checks,
  parent-route masking/restore rules, cleanup/quarantine rules, report fields,
  forbidden operations, and classifications accepted by
  phase11-rp1-gpio-ownership-restore-source-contract-20260607.
- Implemented the real candidate as the smallest read-only GPIO14 ownership
  route preflight snapshot.
- Implemented the paired no-MMIO/no-RP1/no-GIC control candidate with the same
  serial/output shape and simulated zero raw values.
- Retained static/archive evidence for the real and control candidate
  boundary.

## Non-Goals

No Pi 5 hardware run, hardwareTestLock acquisition, published boot archive,
GPIO event generation outside the accepted contract, interrupt delivery,
GIC IAR/EOIR acknowledgement, ISR installation, broad GPIO driver ownership,
unplanned pin-control/pad/RIO writes, clock/reset programming, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, or phase transition.

## Findings

- fixed: added two explicit Pi 5 boot scenarios:
  rpi5_rp1_gpio14_ownership_route_preflight_read and
  rpi5_rp1_gpio14_ownership_route_preflight_no_mmio_control.
- fixed: added source-backed RIO0 OUT/OE/IN constants at
  0x1f_000e_0000, 0x1f_000e_0004, and 0x1f_000e_0008, with unit-test
  coverage for the accepted address translation.
- fixed: the real candidate emits
  TALOS: rp1-gpio14-ownership-route-preflight-result with the accepted
  contract id, target, pin, selected RP1/GIC addresses, raw register values,
  GPIO14 CTRL/event/status decodes, IO_BANK0 INTE/INTS decodes, RIO/pad
  decodes, INTID 160 GIC route status decodes, HPPIR INTID, and one of the
  accepted preflight classifications.
- fixed: the real candidate performs only the accepted read operations:
  GPIO14 STATUS/CTRL, IO_BANK0 INTE/INTS, RIO0 OUT/OE/IN, GPIO14 pad control,
  GICD_ISENABLER5, GICD_ISPENDR5, GICD_ISACTIVER5, and GICC_HPPIR.
- fixed: the control candidate emits
  TALOS: rp1-gpio14-ownership-route-preflight-control with the same field
  shape, not-constructed address fields, simulated zero raw values, and
  classification=simulated/control.
- fixed: control archive and disassembly review prove the control image omits
  the real diagnostic marker, selected RP1/GIC address strings, and forbidden
  RP1 GPIO/RIO/pads/clock/reset/MSI-X/PCIe/MIP/GIC MMIO address construction.
- deferred: serialized Pi 5 no-MMIO control proof, real Pi 5 preflight proof,
  GPIO CTRL writes, IO_BANK0 INTE writes, RIO/pad writes, deterministic event
  source generation, interrupt pending/delivery, GIC acknowledgement,
  ISR/handler ownership, broad GPIO ownership, clock/reset programming,
  DMA/cache, storage, generated-root, networking, SSH, broader PCIe
  enumeration, Milestone 11.3, and phase transition.
- not-an-issue: UART10 FR/DR polling remains present in both candidates
  because it is the existing firmware-preserved serial output path, not RP1
  GPIO/RIO/pads/clock/reset/MSI-X/PCIe/MIP/GIC MMIO.

No findings were removed in this task.

## Candidate Artifacts

Real candidate:

- Archive:
  target/talos-rpi5-rp1-gpio14-ownership-route-preflight-read-core.tar.gz
- Archive SHA256:
  7ccb204d6c14f0b2ad6d9c3796ec4fe000956d98ab25c02a014f75d01184f40e
- kernel_2712.img SHA256:
  cb4155be67ee9188dda9f0f17c55afd3e539381e96e35afdec74a2a3a2ebdc19
- kernel_2712.img size: 50056 bytes
- Marker: TALOS: rp1-gpio14-ownership-route-preflight-result

Control candidate:

- Archive:
  target/talos-rpi5-rp1-gpio14-ownership-route-preflight-no-mmio-control-core.tar.gz
- Archive SHA256:
  5977bfdd8880a7eebe5a7d31c1db8cde10bea65994c9da3d14c41b1913dba170
- kernel_2712.img SHA256:
  c406268f9c5b5257bd3671d9502b1328910d56352e720d46ae6d5cf34e6964e7
- kernel_2712.img size: 48368 bytes
- Marker: TALOS: rp1-gpio14-ownership-route-preflight-control

These are local/static artifacts only. No hardware behavior is accepted.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass, 423 talos no_std tests.
- Archive review:
  scripts/rpi5-rp1-gpio14-ownership-route-preflight-read-review.sh passed.
- Archive review:
  scripts/rpi5-rp1-gpio14-ownership-route-preflight-no-mmio-control-review.sh
  passed.
- Static disassembly/source inspection: passed; real candidate constructs the
  accepted RP1 GPIO/RIO/pad addresses and accepted GIC status reads, while the
  control candidate constructs no forbidden RP1/GIC/MSI-X/PCIe/MIP/GPIO/pads/
  RIO/clock/reset MMIO address.
- git diff --check: pass.
- mdbook build: not run; no docs/src files were touched.
- git diff --cached --check: pass.

## Result

Accepted. This accepts only the local/static real candidate, the local/static
no-MMIO/no-RP1/no-GIC control candidate, and their archive/static boundary. It
does not accept Pi 5 hardware behavior, GPIO ownership, event generation,
interrupt pending generation, interrupt enablement or delivery, GIC IAR/EOIR
acknowledgement, ISR/handler ownership, GPIO CTRL/INTE/RIO/pad writes,
clock/reset programming, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe, Milestone 11.3, or a phase transition.

## Follow-Up

Promote phase11-rp1-gpio-ownership-restore-control-pi5-20260607 only after
this task is accepted and committed and hardwareTestLock remains
unlocked/restored. The next task owns the serialized Pi 5 no-MMIO/no-RP1/no-GIC
control proof before any real GPIO ownership/restore preflight run.
