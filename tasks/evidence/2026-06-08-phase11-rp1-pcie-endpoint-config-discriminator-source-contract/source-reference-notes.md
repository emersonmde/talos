# RP1 PCIe Endpoint/Config Discriminator Source Notes

Task:
phase11-rp1-pcie-endpoint-config-discriminator-source-contract-20260608

## Sources Inspected

- tasks/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-closeout.md
- tasks/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-pi5.md
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712.dtsi
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi-5-b.dts
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/rp1.dtsi
- tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-source-contract/pcie-brcmstb.c
- src/target/rpi5.rs

## Source Facts

- fixed: bcm2712.dtsi defines pcie2 as compatible brcm,bcm2712-pcie with
  register base 0x10_0012_0000 and a non-prefetchable PCIe window that maps
  PCIe address 0x00_0000_0000 to CPU physical 0x1f_0000_0000.
- fixed: bcm2712-rpi-5-b.dts maps RP1 bus 0xc0_4000_0000 into that pcie2
  window, so prior RP1 peripheral/SYSINFO reads exercised the downstream
  0x1f_0000_0000 translated aperture rather than the pcie2 host-controller
  register block itself.
- fixed: the accepted SYSINFO/clock-sentinel proof classified the RP1 aperture
  as rp1-sysinfo-and-clock-window-sentinel: SYSINFO_CHIP_ID,
  SYSINFO_PLATFORM, and CLK_ADC_CTRL all returned 0xdeaddead.
- fixed: pcie-brcmstb.c defines PCIE_MISC_PCIE_STATUS at offset 0x4068. For
  pcie2 this is CPU physical 0x10_0012_4068.
- fixed: pcie-brcmstb.c defines status bits PCIE_PORT=0x80,
  DL_ACTIVE=0x20, PHYLINKUP=0x10, and LINK_IN_L23=0x40.
- fixed: pcie-brcmstb.c brcm_pcie_link_up() reads PCIE_MISC_PCIE_STATUS and
  requires DL_ACTIVE and PHYLINKUP.
- fixed: pcie-brcmstb.c documents that endpoint config-space access without
  link-up can cause a CPU abort, and its endpoint config path writes
  EXT_CFG_INDEX before reading EXT_CFG_DATA. That makes direct endpoint config
  probing unsuitable for this read-only source contract.

## Selected Discriminator

The accepted source contract selects exactly one read-only discriminator:

~~~text
target: pcie2-host-link-status-read
address: 0x1000124068
width: 32-bit volatile little-endian load
source register: BCM2712 pcie2 PCIE_MISC_PCIE_STATUS
~~~

The discriminator is qualitatively different from prior RP1
peripheral/SYSINFO/clock reads because it reads the BCM2712 PCIe2
host-controller status register at 0x10_0012_4068, not the downstream RP1
PCIe window at 0x1f_0000_0000.

## Expected Separations

- pcie2-host-link-up-rp1-window-sentinel: status is not 0xdeaddead, both
  DL_ACTIVE and PHYLINKUP are true, and retained context remains
  rp1-sysinfo-and-clock-window-sentinel. This separates visible pcie2
  host/link state from the retained RP1-window sentinel.
- pcie2-host-status-visible-link-down: status is visible but link-up bits are
  not both set. This blocks endpoint/config claims until a later task explains
  link state.
- pcie2-host-status-sentinel: status itself is 0xdeaddead. This blocks
  same-shaped endpoint/config/decode probing because the host-controller status
  path did not separate from the retained sentinel.

## Deferred Or Rejected

- deferred: endpoint config-space reads, BAR discovery, enumeration, bridge
  setup, PERST, MSI/MIP programming, DMA/cache, and bus mastering all require
  later tasks because source inspection shows config access is not purely
  read-only and is unsafe when link-up is false.
- not-an-issue: no restore operation is needed for the selected status read.
- removed: no same-shaped RP1 peripheral/SYSINFO/clock retry is accepted as a
  discriminator for this task.
