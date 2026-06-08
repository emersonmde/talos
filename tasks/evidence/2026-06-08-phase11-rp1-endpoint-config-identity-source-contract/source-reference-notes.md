# RP1 Endpoint Config Identity Source Notes

Task:
phase11-rp1-endpoint-config-identity-source-contract-20260608

## Sources Inspected

- tasks/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-closeout.md
- tasks/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-pi5.md
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712.dtsi
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi-5-b.dts
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi.dtsi
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/rp1.dtsi
- tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-source-contract/pcie-brcmstb.c
- src/target/rpi5.rs
- https://raw.githubusercontent.com/torvalds/linux/master/drivers/misc/rp1/rp1_pci.c
- https://raw.githubusercontent.com/torvalds/linux/master/include/linux/pci_ids.h
- https://cateee.net/lkddb/web-lkddb/MISC_RP1.html

## Source Facts

- fixed: bcm2712.dtsi defines pcie2 as compatible `brcm,bcm2712-pcie`,
  register base `0x10_0012_0000`, PCI domain 2, and a non-prefetchable
  downstream PCIe window mapping PCIe `0x00_0000_0000` to CPU physical
  `0x1f_0000_0000`.
- fixed: bcm2712-rpi-5-b.dts binds `rp1_target` to `&pcie2`, sets pcie2
  `status = "okay"`, includes `rp1.dtsi`, and maps RP1 bus
  `0xc0_4000_0000..0xc0_4040_ffff` into pcie2 PCI address
  `0x00_0000_0000`.
- fixed: the accepted PCIe2 host-link proof reported PCIE_MISC_PCIE_STATUS
  raw `0x3e0b0`, `dl_active=true`, and `phylinkup=true`.
- fixed: retained pcie-brcmstb.c gates non-root-bus endpoint config access on
  `brcm_pcie_link_up()` because access without link-up can cause a CPU abort.
- fixed: retained pcie-brcmstb.c writes
  `PCIE_ECAM_OFFSET(bus->number, devfn, 0)` to `EXT_CFG_INDEX` before
  returning `EXT_CFG_DATA + PCIE_ECAM_REG(where)` for the data access.
- fixed: the retained BCM2712 config uses `EXT_CFG_INDEX=0x9000` and
  `EXT_CFG_DATA=0x8000`, so pcie2 controller addresses are
  `0x1000129000` and `0x1000128000`.
- fixed: bus 1, device 0, function 0, offset 0 yields config index
  `0x00100000` and data address `0x1000128000`.
- fixed: Linux mainline rp1_pci.c binds the RP1 misc driver to
  `PCI_DEVICE(PCI_VENDOR_ID_RPI, PCI_DEVICE_ID_RPI_RP1_C0)`.
- fixed: Linux pci_ids.h defines `PCI_VENDOR_ID_RPI` as `0x1de4` and
  `PCI_DEVICE_ID_RPI_RP1_C0` as `0x0001`; LKDDb/pci.ids names that pair
  "RP1 PCIe 2.0 South Bridge".

## Selected Discriminator

The accepted source contract selects exactly one bounded endpoint config
identity-read sequence:

~~~text
target: rp1-endpoint-config-vendor-device-read
precondition: PCIE_MISC_PCIE_STATUS DL_ACTIVE && PHYLINKUP
index write: 0x00100000 -> 0x1000129000
data read: 32-bit little-endian load from 0x1000128000
BDF/offset: 0002:01:00.0 offset 0x0
expected vendor/device: 0x1de4:0x0001
~~~

The selected `EXT_CFG_INDEX` write is a BCM2712 controller target selector for
the following config-data read. It is not an endpoint config-space write,
bridge setup, BAR programming, PERST/link-control operation, or restore-owned
state mutation.

## Expected Separations

- `rp1-endpoint-config-id-visible`: host link is up, the config dword is not
  all ones, zero, or `0xdeaddead`, and vendor/device decode matches
  `0x1de4:0x0001`.
- `rp1-endpoint-config-id-unexpected`: host link is up and the config dword is
  visible but vendor/device does not match the RP1 pair.
- `rp1-endpoint-config-id-all-ones`, `rp1-endpoint-config-id-zero`, and
  `rp1-endpoint-config-id-sentinel`: host link is up but config data returns
  an invalid/sentinel value.
- `rp1-endpoint-config-link-down-skip`: host status is visible but link-up
  bits are not both set, so no index write or config-data read is allowed.

## Deferred Or Rejected

- deferred: BAR reads/writes, endpoint config writes, bridge setup,
  PERST/link-control changes, MSI/MSI-X programming, DMA/cache, bus mastering,
  and broad RP1 mapping require later tasks.
- not-an-issue: no restore operation is needed for the selected source
  contract because the only selected write is the controller config target
  selector and the selected data access is read-only.
- removed: no same-shaped PCIe2 host-link-status or RP1 peripheral/SYSINFO/
  clock-window rerun is accepted as this task's identity discriminator.
