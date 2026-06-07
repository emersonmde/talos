# Phase 11 RP1 GIC-Visible Route Diagnostic Core Static Inspection

Task: `phase11-rp1-gic-visible-route-diagnostic-core-20260607`

Evidence level: static source, archive, strings, and disassembly inspection.

## Inputs

- Real archive review:
  `tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-diagnostic-core/real-archive-review.txt`
- Control archive review:
  `tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-diagnostic-core/control-archive-review.txt`
- Real strings:
  `tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-diagnostic-core/real-candidate-strings.txt`
- Control strings:
  `tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-diagnostic-core/control-candidate-strings.txt`
- Real disassembly:
  `tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-diagnostic-core/real-gic-visible-route-asm.txt`
- Control disassembly:
  `tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-diagnostic-core/control-gic-visible-route-asm.txt`

## Real Candidate Boundary

- `run_rp1_gic_visible_route_status_read` constructs `GICD_BASE`
  `0x10_7fff_9000` and uses the accepted GICv2 helper read paths.
- The selected function calls the distributor read helper with offsets `0x114`,
  `0x214`, and `0x314`, corresponding to `GICD_ISENABLER5`,
  `GICD_ISPENDR5`, and `GICD_ISACTIVER5`.
- It then calls `GicV2::highest_pending`, which adds offset `0x18` to
  `GICC_BASE` `0x10_7fff_a000`, corresponding to `GICC_HPPIR`.
- The report loop prints the accepted fields and classification
  `gic-route-status-visible`.
- No GIC enable path, `GICC_IAR` read, `GICC_EOIR` write, interrupt unmasking,
  ISR installation, RP1 read/write, MSI-X read/write, PCIe/MIP, GPIO, pads,
  RIO, clock/reset, DMA/cache, storage, generated-root, networking, or SSH path
  is part of this candidate.

## Control Boundary

- `run_rp1_gic_visible_route_no_mmio_control` emits the paired marker
  `TALOS: rp1-gic-route-status-control`.
- It preserves the report shape and classification field while reporting GICD,
  GICC, bit mask, and all selected register addresses as `not-constructed`.
- The control image strings omit the real diagnostic marker, real
  `classification=gic-route-status-visible`, and the forbidden GIC/RP1 address
  strings checked by the archive review helper.
- The selected control function disassembly constructs only code/rodata and
  UART10 serial addresses. The recurring `0x10_7d00_1000` address and
  `[x9, #0x18]` loads are the existing UART10 FR flush path.
- It does not construct `0x10_7fff_9000`, `0x10_7fff_a000`,
  `0x10_7fff_9114`, `0x10_7fff_9214`, `0x10_7fff_9314`,
  `0x10_7fff_a018`, `0x1f_0010_8008`, `0x1f_000d_0070`,
  `0x1f_000f_003c`, or any other accepted-forbidden GIC/RP1 diagnostic
  address.

## Disposition

- fixed: real candidate implements the exact read-only/no-ack GIC-visible
  route status boundary.
- fixed: control candidate preserves output/capture shape while avoiding
  forbidden GIC/RP1/MSI-X/PCIe/MIP/GPIO/pads/RIO/clock/reset MMIO address
  construction.
- deferred: hardware control proof and real hardware diagnostic proof.
- not-an-issue: UART10 FR polling remains the required serial flush path.
