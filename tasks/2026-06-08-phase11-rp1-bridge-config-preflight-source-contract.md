# Phase 11 RP1 Bridge/Config Preflight Source Contract

Task id: phase11-rp1-bridge-config-preflight-source-contract-20260608

Status: accepted

Classification: accepted-source-contract

## Goal

Select the smallest source-backed bridge/config-preflight discriminator that
can explain or move past the accepted RP1 endpoint config identity all-ones
result without accepting broad RP1 mapping or PCIe ownership.

## Scope

- Reviewed retained Raspberry Pi Linux BCM2712/RP1 device-tree sources,
  retained Broadcom STB PCIe host-driver setup/config-access source, accepted
  PCIe2 host-link status evidence, accepted endpoint config identity evidence,
  and current Talos RP1/PCIe constants.
- Selected one read-only host-controller preflight discriminator:
  pcie2-bridge-misc-ctrl-preflight-read.
- Defined exact allowed reads, source offsets, output markers,
  classifications, paired-control constraints, and forbidden operations.
- Preserved the accepted boundary: the endpoint config all-ones result is not
  retried in the same shape, broad RP1 mapping and endpoint ownership remain
  unaccepted, and no phase transition is implied.
- Updated roadmap and Phase 11 RP1/PCIe map contract docs for the accepted
  source-contract frontier.
- Recorded findings with disposition.

## Non-Goals

No runtime implementation, hardware run, boot archive publication,
hardwareTestLock acquisition, endpoint configuration mutation, BAR discovery
or programming, bridge setup writes, CPU-to-PCIe window programming,
PERST/link-control change, MSI/MIP/GIC operations, interrupt enablement or
delivery, GIC acknowledgement, ISR installation, RP1 peripheral/SYSINFO/
clock/GPIO retry, RP1 clock/reset writes, GPIO/RIO/pad writes, event
generation, DMA/cache, storage, generated-root, networking, SSH, Milestone
11.3, or phase transition.

## Findings And Disposition

- fixed: selected a qualitatively different discriminator from the accepted
  endpoint config identity attempt: a read-only snapshot of the BCM2712 pcie2
  PCIE_MISC_MISC_CTRL setup/preflight register, paired with the already
  accepted host-link status read for context.
- fixed: tied the selected register to retained Broadcom STB PCIe setup
  source. brcm_pcie_setup() reads and writes PCIE_MISC_MISC_CTRL to set
  SCB_ACCESS_EN, CFG_READ_UR_MODE, max burst size, RCB_MPS_MODE, and
  RCB_64B_MODE before inbound-window setup, root-complex class setup, and
  outbound-window programming.
- fixed: treated PCIE_MISC_MISC_CTRL as a preflight state discriminator, not
  as proof of endpoint ownership, broad RP1 mapping, or working endpoint
  config access. A visible ready-shaped value only justifies later supervisor
  planning; it does not accept bridge setup or BAR work.
- fixed: avoided any EXT_CFG_INDEX write, EXT_CFG_DATA read, endpoint config
  offset, BAR access, outbound-window programming, PERST/link-control,
  MSI/MIP/GIC, RP1 peripheral, or DMA/cache operation in this source contract.
- fixed: required a paired no-MMIO/no-PCIe/no-RP1/no-GIC control that
  preserves output shape and classification vocabulary while constructing no
  forbidden MMIO address.
- deferred: outbound-window register snapshots, root-complex class snapshots,
  BAR discovery, bridge setup, endpoint config retries with different BDF or
  offset, PERST/link-control, interrupt delivery, DMA/cache, and broad RP1
  mapping require later supervisor-planned tasks if this discriminator
  justifies them.
- not-an-issue: no restore operation is required because the selected
  discriminator is read-only and does not mutate controller, endpoint, bridge,
  RP1, GIC, or DMA state.

No findings were removed.

## Accepted Source Contract

Contract id:
phase11-rp1-bridge-config-preflight-source-contract-v1

~~~text
target: pcie2-bridge-misc-ctrl-preflight-read
operation: read-only BCM2712 PCIe2 bridge/config preflight snapshot
pcie2 controller base: 0x1000120000
status register: PCIE_MISC_PCIE_STATUS at offset 0x4068
preflight register: PCIE_MISC_MISC_CTRL at offset 0x4008
status address: 0x1000124068
preflight address: 0x1000124008
width: 32
~~~

Allowed sequence:

1. Read PCIE_MISC_PCIE_STATUS at 0x1000124068 and decode DL_ACTIVE (0x20),
   PHYLINKUP (0x10), PCIE_PORT (0x80), and LINK_IN_L23 (0x40).
2. Read exactly one 32-bit little-endian dword from PCIE_MISC_MISC_CTRL at
   0x1000124008.
3. Decode SCB_ACCESS_EN (0x1000), CFG_READ_UR_MODE (0x2000), RCB_MPS_MODE
   (0x400), RCB_64B_MODE (0x80), and MAX_BURST_SIZE (0x300000).

No other MMIO load is selected. No MMIO store is selected.

## Source Reconciliation

- bcm2712.dtsi declares pcie2 as compatible brcm,bcm2712-pcie with
  controller register base 0x10_0012_0000, PCI domain 2, and a
  non-prefetchable downstream PCIe window mapping PCIe 0x00_0000_0000 to CPU
  physical 0x1f_0000_0000.
- bcm2712-rpi-5-b.dts binds rp1_target to &pcie2, sets pcie2 status = okay,
  and maps RP1 bus 0xc0_4000_0000..0xc0_4040_ffff into pcie2 PCI address
  0x00_0000_0000.
- The accepted PCIe2 host-link status Pi 5 proof reported
  PCIE_MISC_PCIE_STATUS=0x3e0b0, dl_active=true, and phylinkup=true,
  separating visible host/link state from retained RP1-window sentinel reads.
- The accepted endpoint config identity Pi 5 proof reached that same link-up
  precondition but EXT_CFG_DATA + 0 for BDF 0002:01:00.0 returned
  0xffffffff.
- Retained pcie-brcmstb.c defines PCIE_MISC_MISC_CTRL at host-controller
  offset 0x4008; brcm_pcie_setup() sets SCB_ACCESS_EN, CFG_READ_UR_MODE, max
  burst size, RCB_MPS_MODE, and RCB_64B_MODE in this register before
  inbound-window setup, root-complex class-code setup, and outbound-window
  programming.
- The same retained driver keeps endpoint config access behind the separate
  link-up predicate and uses EXT_CFG_INDEX plus EXT_CFG_DATA; this source
  contract does not repeat that path.

## Report Fields

- contract id and target name.
- pcie2 controller base, register names, source offsets, CPU physical
  addresses, and widths.
- raw PCIE_MISC_PCIE_STATUS, decoded pcie_port, dl_active, phylinkup,
  link_in_l23, and status_is_deaddead.
- raw PCIE_MISC_MISC_CTRL, decoded scb_access_en, cfg_read_ur_mode,
  rcb_mps_mode, rcb_64b_mode, max_burst_size, and misc_ctrl_is_sentinel.
- retained endpoint config identity classification
  rp1-endpoint-config-id-all-ones.
- terminal classification.

Accepted classifications:

- pcie2-bridge-preflight-ready
- pcie2-bridge-preflight-incomplete
- pcie2-bridge-preflight-sentinel
- pcie2-bridge-preflight-link-down-skip
- pcie2-bridge-preflight-inconclusive-capture
- no-mmio-pcie2-bridge-preflight-control-visible
- staging/build-blocker

Classification rules:

- pcie2-bridge-preflight-ready: status is non-sentinel, dl_active=true,
  phylinkup=true, PCIE_MISC_MISC_CTRL is non-sentinel, and both SCB_ACCESS_EN
  and CFG_READ_UR_MODE are set.
- pcie2-bridge-preflight-incomplete: status is visible and the preflight
  register is visible, but either SCB_ACCESS_EN or CFG_READ_UR_MODE is clear.
- pcie2-bridge-preflight-sentinel: PCIE_MISC_MISC_CTRL reads as 0xdeaddead,
  0xffffffff, or 0x00000000.
- pcie2-bridge-preflight-link-down-skip: status is visible but either
  dl_active or phylinkup is false; the preflight register value may be
  reported, but no config retry or bridge claim is accepted.

## Paired Control Constraints

The paired control must preserve the same output shape, field names, and
classification vocabulary while constructing no BCM2712 PCIe, RP1
peripheral/SYSINFO/clock/GPIO/MSI-X, MIP, GIC, DMA, or other MMIO address and
performing no volatile load or store. It must classify as
no-mmio-pcie2-bridge-preflight-control-visible.

## Forbidden Operations

- Same-shaped endpoint config identity hardware reruns.
- EXT_CFG_INDEX writes, EXT_CFG_DATA reads or writes, endpoint config offset
  probing, BAR discovery, BAR programming, bridge setup writes, CPU-to-PCIe
  window programming, PERST/link-control changes, bus mastering, or DMA/cache
  operations.
- MSI/MIP/GIC operations, interrupt enablement or delivery, GIC
  acknowledgement, ISR installation, RP1 peripheral/SYSINFO/clock/GPIO/GIC
  retries, RP1 clock/reset writes, GPIO/RIO/pad writes, event generation,
  storage, generated-root, networking, SSH, Milestone 11.3, or phase
  transition.

## Accepted Claims

This task accepts only the source contract for a read-only BCM2712 PCIe2
bridge/config preflight discriminator and the paired no-MMIO/no-PCIe/no-RP1/
no-GIC control requirement. It does not accept runtime behavior, hardware
behavior, expected RP1 vendor/device visibility, endpoint ownership, broad RP1
mapping, endpoint configuration mutation, bridge setup, BAR discovery or
programming, interrupt delivery, DMA/cache, networking, SSH, Milestone 11.3,
or phase transition.

## Evidence

- Source notes:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-config-preflight-source-contract/source-reference-notes.md.
- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-config-preflight-source-contract/evidence-map.json.
- Retained Broadcom STB PCIe source:
  tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-source-contract/pcie-brcmstb.c.
- Retained Raspberry Pi device-tree sources:
  tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/.
- Updated contract docs:
  docs/src/project/phase11-rp1-pcie-map-contract.md.
- Updated roadmap:
  docs/src/roadmap.md.

## Validation

- Static inspection: accepted endpoint config identity closeout, accepted
  PCIe2 host-link closeout, retained Raspberry Pi Linux device-tree sources,
  retained Broadcom STB PCIe setup/config-access source, current Talos
  RP1/PCIe constants, roadmap, and RP1/PCIe map contract inspected.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

Accepted as accepted-source-contract.

Next mechanically unblocked task:
phase11-rp1-bridge-config-preflight-core-20260608. Implement only the accepted
read-only bridge/config preflight discriminator and paired no-MMIO/no-PCIe/
no-RP1/no-GIC control; do not acquire hardwareTestLock for the core task.
