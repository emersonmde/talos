# Task: Phase 11 RP1 GIC-Visible Route Diagnostic Core

Task ID: `phase11-rp1-gic-visible-route-diagnostic-core-20260607`

Status: accepted

Evidence level: static/archive inspection, fmt/lint/typecheck, unit tests

## Goal

Implement the accepted GIC-visible route diagnostic core and paired
no-MMIO/no-GIC/no-RP1 control locally, producing candidate artifacts but no
hardware run.

## Scope

- Used only the target, register reads, report fields, forbidden operations,
  and classifications accepted by
  `phase11-rp1-gic-visible-route-source-contract-20260607`.
- Implemented the real candidate as the smallest read-only/no-ack GICv2 status
  snapshot for the predicted RP1 IO_BANK0 route to GIC SPI 128 / INTID 160.
- Implemented the paired control candidate with the same serial/output shape
  and no constructed GICD/GICC/RP1/MSI-X/PCIe/MIP/GPIO/pads/RIO/clock/reset
  MMIO addresses.
- Retained static/archive evidence for the real and control candidate boundary.

## Non-Goals

No Pi 5 hardware run, hardwareTestLock acquisition, published boot archive,
broad GIC abstraction, GIC enable writes, IAR/EOIR acknowledgement, interrupt
unmasking, ISR installation, RP1 writes, MSI-X enable/IACK writes, GPIO
ownership, pin-control or pad writes, clock/reset programming, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, or phase transition.

## Findings

- fixed: added two explicit Pi 5 boot scenarios:
  `rpi5_rp1_gic_visible_route_status_read` and
  `rpi5_rp1_gic_visible_route_no_mmio_control`.
- fixed: the real candidate emits
  `TALOS: rp1-gic-route-status-result` with the accepted contract id, target,
  hwirq, predicted MSI-X vector, predicted GIC SPI/INTID, GICD/GICC bases,
  distributor bank, bit mask, selected register addresses, raw enable/pending
  /active bank values, decoded INTID 160 bits, raw `GICC_HPPIR`, decoded HPPIR
  INTID, `hppir-spurious`, `hppir-target-match`, and
  `classification=gic-route-status-visible`.
- fixed: the real candidate performs only the accepted GICv2 read operations:
  `GICD_ISENABLER5`, `GICD_ISPENDR5`, `GICD_ISACTIVER5`, and `GICC_HPPIR`.
- fixed: the control candidate emits
  `TALOS: rp1-gic-route-status-control` with the same output shape and
  simulated zero raw values while reporting all GIC/base/address fields as
  `not-constructed`.
- fixed: control archive review proves the control image omits the real
  diagnostic marker, real classification, and forbidden GIC/RP1 address
  strings.
- deferred: Pi 5 control run, Pi 5 real diagnostic run, interrupt delivery,
  GIC IAR/EOIR acknowledgement, ISR/handler ownership, GPIO ownership,
  clock/reset programming, DMA/cache, storage, generated-root, networking, SSH,
  broader PCIe enumeration, Milestone 11.3, and phase transition.
- not-an-issue: UART10 FR polling loads/stores remain present in both
  candidates because they are the existing firmware-preserved serial flush
  path, not GIC/RP1 interrupt/GPIO/pads/RIO/clock/reset/MSI-X/PCIe/MIP MMIO.

No findings were removed in this task.

## Candidate Artifacts

Real candidate:

- Archive: `target/talos-rpi5-rp1-gic-visible-route-status-read-core.tar.gz`
- Archive SHA256:
  `6cd353262326572afb217cfe1a348199cf4be20e5bd24cf54d41527100b12f4a`
- `kernel_2712.img` SHA256:
  `cf499fb46542395bddd00b5232bc6f19b35298b15d08fd731a0008d73a969137`
- `kernel_2712.img` size: 47816 bytes
- Marker: `TALOS: rp1-gic-route-status-result`

Control candidate:

- Archive:
  `target/talos-rpi5-rp1-gic-visible-route-no-mmio-control-core.tar.gz`
- Archive SHA256:
  `37360a3f30c3f60af9c2e497caff498e3a85328c39f9fc90953a088df4d6b56a`
- `kernel_2712.img` SHA256:
  `85b21994d20d539ec62ef99142e6589ef7bab3bf5070b8399b76fa84bfe262c5`
- `kernel_2712.img` size: 47040 bytes
- Marker: `TALOS: rp1-gic-route-status-control`

These are local/static artifacts only. No hardware behavior is accepted.

## Validation

- `cargo fmt --all -- --check`: pass.
- `cargo -Zjson-target-spec test --quiet`: pass.
- Archive review:
  `scripts/rpi5-rp1-gic-visible-route-status-read-review.sh` passed.
- Archive review:
  `scripts/rpi5-rp1-gic-visible-route-no-mmio-control-review.sh` passed.
- Static disassembly/source inspection: passed; real candidate uses the four
  accepted GICv2 read-only status reads, and control candidate constructs no
  forbidden GIC/RP1/MSI-X/PCIe/MIP/GPIO/pads/RIO/clock/reset MMIO address.
- `git diff --check`: pass.
- `mdbook build`: not run; no `docs/src` files were touched.
- `git diff --cached --check`: pass.

## Result

Accepted. This accepts only the local/static real candidate, the local/static
no-MMIO/no-GIC/no-RP1 control candidate, and their archive/static boundary. It
does not accept Pi 5 hardware behavior, interrupt delivery, handler ownership,
GPIO ownership, clock/reset programming, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe, Milestone 11.3, or a phase transition.

## Follow-Up

Promote `phase11-rp1-gic-visible-route-no-mmio-control-pi5-20260607` only
after this task is accepted and committed and `hardwareTestLock` remains
unlocked/restored. The next task owns the serialized Pi 5 no-MMIO/no-GIC/no-RP1
control proof before any real GIC-visible route diagnostic run.
