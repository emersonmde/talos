# Static Reconciliation Notes

Task: phase11-rp1-pcie-endpoint-config-discriminator-closeout-20260608

Classification: pcie2-host-link-up-rp1-window-sentinel-frontier-closed

## Inputs Inspected

- Source contract:
  tasks/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-source-contract.md
- Core task:
  tasks/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-core.md
- Control proof:
  tasks/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-control-pi5.md
- Real proof:
  tasks/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-pi5.md
- Core, control, and real proof evidence maps and classifications.
- Project contract:
  docs/src/project/phase11-rp1-pcie-map-contract.md
- Roadmap Phase 11 section:
  docs/src/roadmap.md

## Chain Summary

- Source contract selected exactly one read-only discriminator:
  pcie2-host-link-status-read, a 32-bit PCIE_MISC_PCIE_STATUS load at
  0x1000124068.
- Core implementation produced a real candidate that performs that read and a
  paired no-MMIO/no-RP1/no-GIC control candidate that preserves output shape.
- Control Pi 5 proof accepted the simulated/control output path as visible and
  identity-joined with two 46,672-byte TFTP fetches and 118 control markers.
- Real Pi 5 proof accepted visible/link-up PCIe2 host status: raw=0x3e0b0,
  pcie-port=true, dl-active=true, phylinkup=true, link-in-l23=false,
  status-is-deaddead=false, retained-rp1-window-sentinel=true.

## Accepted Boundary

The accepted boundary is only visible/link-up BCM2712 PCIe2 host status while
the retained RP1 SYSINFO/clock-window path remains sentinel-shaped. This is a
useful discriminator because it shows the host-controller status path is live
outside the retained RP1 0xdeaddead aperture. It does not prove endpoint
config-space access, endpoint ownership, broad RP1 mapping, interrupt
delivery, or DMA/cache readiness.

## Rejected Claims

- Endpoint config-space access, BAR discovery, enumeration, bridge setup,
  PERST/link-control changes, PCIe writes, MSI/MIP/GIC operations, and bus
  mastering.
- Broad RP1 mapping, endpoint ownership, clock/reset ownership, GPIO
  ownership, event generation, interrupt delivery, ISR/handler ownership,
  DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3, and
  phase transition.

## Next Action

Supervisor planning is required before any new Milestone 11.2 feature slice or
phase transition. Same-shaped PCIe2 host-link status hardware reruns should not
be treated as progress without a different discriminator or new acceptance
criteria.
