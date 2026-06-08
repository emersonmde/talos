# RP1 Bridge/Config Preflight Source Notes

Task:
phase11-rp1-bridge-config-preflight-source-contract-20260608

## Sources Inspected

- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md
- tasks/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-closeout.md
- tasks/2026-06-08-phase11-rp1-endpoint-config-identity-closeout.md
- tasks/2026-06-08-phase11-rp1-endpoint-config-identity-pi5.md
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712.dtsi
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi-5-b.dts
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/rp1.dtsi
- tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-source-contract/pcie-brcmstb.c
- src/target/rpi5.rs

## Source Facts

- fixed: bcm2712.dtsi defines pcie2 as compatible brcm,bcm2712-pcie,
  controller register base 0x10_0012_0000, PCI domain 2, and a downstream
  non-prefetchable PCIe window mapping PCIe 0x00_0000_0000 to CPU physical
  0x1f_0000_0000.
- fixed: bcm2712-rpi-5-b.dts binds RP1 under &pcie2, keeps pcie2 enabled,
  and maps RP1 bus 0xc0_4000_0000..0xc0_4040_ffff to PCIe 0x00_0000_0000.
- fixed: retained pcie-brcmstb.c defines PCIE_MISC_MISC_CTRL at offset
  0x4008, SCB_ACCESS_EN as 0x1000, CFG_READ_UR_MODE as 0x2000,
  RCB_MPS_MODE as 0x400, RCB_64B_MODE as 0x80, and MAX_BURST_SIZE as
  0x300000.
- fixed: retained brcm_pcie_setup() sets those PCIE_MISC_MISC_CTRL fields
  before inbound-window setup, root-complex class-code setup, and outbound
  CPU-to-PCIe window programming.
- fixed: retained brcm_pcie_map_bus() handles root-bus config reads from
  controller registers directly, gates endpoint config access on link-up, and
  uses EXT_CFG_INDEX before EXT_CFG_DATA for non-root-bus endpoint config
  access.
- fixed: the accepted PCIe2 host-link hardware proof reported
  PCIE_MISC_PCIE_STATUS=0x3e0b0, dl_active=true, and phylinkup=true.
- fixed: the accepted endpoint config identity hardware proof reached the
  link-up precondition, wrote the accepted EXT_CFG_INDEX=0x00100000 selector,
  and read EXT_CFG_DATA + 0 as 0xffffffff.

## Selected Discriminator

The accepted source contract selects one read-only preflight sequence:

~~~text
target: pcie2-bridge-misc-ctrl-preflight-read
status read: 32-bit little-endian load from 0x1000124068
preflight read: 32-bit little-endian load from 0x1000124008
decoded preflight bits: SCB_ACCESS_EN, CFG_READ_UR_MODE, RCB_MPS_MODE,
RCB_64B_MODE, MAX_BURST_SIZE
~~~

This is qualitatively different from the accepted endpoint config identity
attempt. It does not write EXT_CFG_INDEX, does not read or write EXT_CFG_DATA,
and does not probe endpoint config offsets or BARs.

## Expected Separations

- pcie2-bridge-preflight-ready: the host link remains visible/up and
  source-defined preflight bits are set in PCIE_MISC_MISC_CTRL.
- pcie2-bridge-preflight-incomplete: the host link/preflight register is
  visible but one of the source-defined preflight bits is clear.
- pcie2-bridge-preflight-sentinel: PCIE_MISC_MISC_CTRL returns 0xdeaddead,
  0xffffffff, or 0x00000000, so same-shaped bridge/config claims remain
  blocked.
- pcie2-bridge-preflight-link-down-skip: the host status read no longer
  satisfies the accepted link-up precondition; no config retry or bridge claim
  is accepted.

## Deferred Or Rejected

- deferred: outbound-window snapshots, root-complex class snapshots, BAR
  discovery, bridge setup, endpoint config retries with different target
  selection, PERST/link-control, interrupt delivery, DMA/cache, and broad RP1
  mapping require later supervisor-planned tasks.
- rejected: repeating the same endpoint config identity hardware run is not
  accepted as progress for this task.
- not-an-issue: no restore operation is required because the selected sequence
  is read-only.
