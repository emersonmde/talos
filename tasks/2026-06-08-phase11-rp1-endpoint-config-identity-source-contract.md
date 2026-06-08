# Phase 11 RP1 Endpoint Config Identity Source Contract

Task id: phase11-rp1-endpoint-config-identity-source-contract-20260608

Status: accepted

Classification: accepted-source-contract

## Goal

Decide the smallest source-backed RP1 endpoint config identity-read
discriminator now that the PCIe2 host-link status register is visible and
link-up on Pi 5.

## Scope

- Reviewed the accepted PCIe2 host-link status closeout, retained RP1
  SYSINFO/clock-window sentinel context, Raspberry Pi Linux BCM2712/RP1
  device-tree sources, the Broadcom STB PCIe config-access source, and
  current Talos RP1/PCIe constants.
- Selected one bounded endpoint config identity-read sequence:
  `rp1-endpoint-config-vendor-device-read`.
- Defined the exact controller/config registers, index-write/data-read
  ordering, bus/device/function/offset, width, expected invariants, report
  fields, classifications, paired control requirements, and forbidden
  operations.
- Separated the controller `EXT_CFG_INDEX` target-selection write from
  endpoint config mutation and BAR/bridge setup.
- Updated roadmap and Phase 11 RP1/PCIe map contract docs for the accepted
  source-contract frontier.
- Recorded findings with disposition.

## Non-Goals

No runtime implementation, Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, endpoint configuration mutation, BAR
programming, bridge setup, PERST/link-control changes, MSI/MIP/GIC operation,
RP1 peripheral/SYSINFO/clock/GPIO retry, clock/reset writes, GPIO/RIO/pad
writes, event generation, interrupt delivery, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, or phase transition.

## Findings

- fixed: selected a single config identity read at PCI domain 2, bus 1, device
  0, function 0, offset 0 because the accepted host-link proof satisfied the
  retained Broadcom STB link-up precondition for endpoint config access.
- fixed: limited the only write to the BCM2712 PCIe2 controller
  `EXT_CFG_INDEX` register at `0x1000129000` with value `0x00100000`; source
  inspection shows this selects the external config target before the
  read-only `EXT_CFG_DATA` access and does not write endpoint config space.
- fixed: selected a single 32-bit read from `EXT_CFG_DATA + 0` at
  `0x1000128000`, decoding the PCI vendor/device dword as vendor `0x1de4` and
  device `0x0001` for the RP1 PCIe 2.0 South Bridge.
- fixed: required a paired no-MMIO/no-RP1/no-GIC control that preserves output
  shape and classification vocabulary while constructing no forbidden MMIO
  address.
- deferred: BAR discovery, bridge setup, endpoint config writes, bus
  mastering, MSI/MSI-X programming, interrupt delivery, DMA/cache, and broad
  RP1 mapping remain outside this source contract.
- not-an-issue: no restore operation is required for the accepted sequence
  because the only selected write changes a controller config-index selector
  and no selected operation mutates endpoint configuration or bridge state.

No findings were removed.

## Accepted Source Contract

Contract id:
phase11-rp1-endpoint-config-identity-source-contract-v1

~~~text
target: rp1-endpoint-config-vendor-device-read
operation: bounded BCM2712 PCIe2 external config identity read
pcie2 controller base: 0x1000120000
precondition register: PCIE_MISC_PCIE_STATUS at offset 0x4068
index register: EXT_CFG_INDEX at offset 0x9000
data register: EXT_CFG_DATA at offset 0x8000
bus/device/function/offset: 0002:01:00.0 offset 0x0
index value: 0x00100000
data address: 0x1000128000
width: 32
expected vendor/device: 0x1de4:0x0001
~~~

Allowed sequence:

1. Read `PCIE_MISC_PCIE_STATUS` at `0x1000124068` and decode `DL_ACTIVE`
   (`0x20`) and `PHYLINKUP` (`0x10`).
2. If either link-up bit is clear, report the link-down classification and do
   not perform the index write or config-data read.
3. If both link-up bits are set, write exactly `0x00100000` to
   `EXT_CFG_INDEX` at `0x1000129000`.
4. Read exactly one 32-bit little-endian dword from `EXT_CFG_DATA + 0` at
   `0x1000128000`.
5. Decode vendor ID from bits 15:0 and device ID from bits 31:16.

No other config offsets are selected. No `EXT_CFG_DATA` write, BAR read/write,
bridge setup, PERST/link-control change, MSI/MIP/GIC operation, RP1
peripheral/SYSINFO/clock/GPIO/GIC operation, DMA/cache operation, or restore
operation is selected.

## Source Reconciliation

- Linux `bcm2712.dtsi` declares pcie2 as compatible `brcm,bcm2712-pcie`,
  register base `0x10_0012_0000`, PCI domain 2, and a non-prefetchable PCIe
  window that maps PCIe address `0x00_0000_0000` to CPU physical
  `0x1f_0000_0000`.
- Linux `bcm2712-rpi-5-b.dts` sets `rp1_target: &pcie2`, marks pcie2 okay,
  includes `rp1.dtsi`, and maps RP1 bus `0xc0_4000_0000..0xc0_4040_ffff`
  into pcie2 PCI address `0x00_0000_0000`.
- The accepted PCIe2 host-link proof reported `PCIE_MISC_PCIE_STATUS`
  raw `0x3e0b0`, `dl_active=true`, and `phylinkup=true`, satisfying the
  retained Broadcom STB config-access link-up gate.
- Retained Broadcom STB PCIe source maps non-root-bus config access by writing
  `PCIE_ECAM_OFFSET(bus->number, devfn, 0)` to `EXT_CFG_INDEX`, then reading
  from `EXT_CFG_DATA + PCIE_ECAM_REG(where)`.
- The retained BCM2712 offset table uses `EXT_CFG_INDEX=0x9000` and
  `EXT_CFG_DATA=0x8000`, making the pcie2 controller addresses
  `0x1000129000` and `0x1000128000`.
- For the direct RP1 endpoint identity target, bus 1, device 0, function 0,
  offset 0 yields `PCIE_ECAM_OFFSET(1, 0, 0) = 0x00100000`.
- Linux mainline RP1 PCI source binds the RP1 misc driver to
  `PCI_DEVICE(PCI_VENDOR_ID_RPI, PCI_DEVICE_ID_RPI_RP1_C0)`, and
  `pci_ids.h` defines those IDs as vendor `0x1de4` and device `0x0001`.

## Expected Invariants

- The precondition status read should not be `0xdeaddead` on the accepted
  hardware path.
- `dl_active` and `phylinkup` must both be true before the selected
  `EXT_CFG_INDEX` write and `EXT_CFG_DATA` read occur.
- The raw config dword at offset 0 should decode as vendor `0x1de4` and
  device `0x0001` for the RP1 PCIe 2.0 South Bridge.
- `raw_config_is_all_ones`, `raw_config_is_zero`, and `raw_config_is_deaddead`
  classify invalid/sentinel returns without accepting endpoint ownership.
- A matching RP1 vendor/device dword classifies only endpoint config identity
  visibility, not broad RP1 mapping, BAR ownership, endpoint programming, or
  interrupt delivery.

## Report Fields

- contract id and target name.
- pcie2 controller base and PCI domain.
- precondition status address, raw status, `dl_active`, `phylinkup`, and
  `status_is_deaddead`.
- config BDF, config offset, index register address, index value, data
  register address, and width.
- raw config dword, vendor ID, device ID, expected vendor/device, and match
  boolean.
- raw-config sentinel booleans for all-ones, zero, and `0xdeaddead`.
- terminal classification.

Accepted classifications:

- `rp1-endpoint-config-id-visible`
- `rp1-endpoint-config-id-unexpected`
- `rp1-endpoint-config-id-all-ones`
- `rp1-endpoint-config-id-zero`
- `rp1-endpoint-config-id-sentinel`
- `rp1-endpoint-config-link-down-skip`
- `rp1-endpoint-config-id-inconclusive-capture`
- `no-mmio-rp1-endpoint-config-id-control-visible`
- `staging/build-blocker`

The paired no-MMIO/no-RP1/no-GIC control must preserve the same output shape
and classification vocabulary while constructing no BCM2712 PCIe, RP1
peripheral/SYSINFO/clock/GPIO/MSI-X, MIP, GIC, or DMA MMIO address and
performing no volatile load/store to those paths.

## Forbidden Operations

- Any `EXT_CFG_DATA` write, endpoint config write, BAR programming, BAR
  discovery beyond the single offset-0 identity read, bridge setup,
  CPU-to-PCIe window programming, PERST, link-control, MSI target/data, MIP,
  or GIC write.
- Config reads from any BDF other than 0002:01:00.0 or any offset other than
  `0x0` in this contract.
- Treating an identity match as endpoint ownership, broad RP1 mapping, BAR
  ownership, interrupt delivery, DMA/cache readiness, or storage/networking
  progress.
- Same-shaped PCIe2 host-link status, RP1 peripheral, SYSINFO, clock, GPIO,
  MSI-X CFG, GIC-visible, or clock-window hardware reruns as progress for this
  contract.
- RP1 clock/reset writes, reset-controller writes, GPIO/RIO/pad writes, event
  generation, interrupt enablement or delivery, GIC acknowledgement, ISR
  installation, storage, generated-root, networking, SSH, Milestone 11.3, or
  phase transition.

## Accepted Claims

This task accepts only the source contract for a bounded BCM2712 PCIe2
external config identity read of the direct RP1 endpoint and the paired
no-MMIO/no-RP1/no-GIC control requirement. It does not accept runtime
behavior, hardware behavior, broad RP1 mapping, endpoint ownership, endpoint
configuration mutation, BAR programming, bridge setup, interrupt delivery,
DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3, or phase
transition.

## Evidence

- Source notes:
  tasks/evidence/2026-06-08-phase11-rp1-endpoint-config-identity-source-contract/source-reference-notes.md.
- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-endpoint-config-identity-source-contract/evidence-map.json.
- Retained Broadcom STB PCIe source:
  tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-source-contract/pcie-brcmstb.c.
- Retained Raspberry Pi device-tree sources:
  tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/.
- Updated contract docs:
  docs/src/project/phase11-rp1-pcie-map-contract.md.
- Updated roadmap:
  docs/src/roadmap.md.

## Validation

- Static inspection: accepted PCIe2 host-link closeout, retained Raspberry Pi
  Linux device-tree sources, retained Broadcom STB PCIe config-access source,
  Linux mainline RP1 PCI ID source, current Talos RP1/PCIe constants, roadmap,
  and RP1/PCIe map contract inspected.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

Accepted as accepted-source-contract.

Next mechanically unblocked task:
phase11-rp1-endpoint-config-identity-core-20260608. Implement only the
accepted bounded endpoint config identity read and paired no-MMIO/no-RP1/no-GIC
control; do not acquire hardwareTestLock for the core task.
