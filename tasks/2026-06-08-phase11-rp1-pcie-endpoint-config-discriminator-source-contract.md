# Phase 11 RP1 PCIe Endpoint/Config Discriminator Source Contract

Task id: phase11-rp1-pcie-endpoint-config-discriminator-source-contract-20260608

Status: accepted

Classification: accepted-source-contract

## Goal

Define the smallest source-backed read-only discriminator that can tell
whether the repeated 0xdeaddead RP1 peripheral/SYSINFO results are caused by
the RP1 PCIe endpoint/config/decode path rather than another peripheral-local
clock or GPIO condition.

## Scope

- Reviewed the accepted UART0 FR, GPIO/status, interrupt-routing, GIC-visible,
  clock/reset, and SYSINFO/clock-sentinel Milestone 11.1/11.2 frontiers.
- Reviewed retained Raspberry Pi Linux device-tree sources, the Broadcom STB
  PCIe host driver source, and current Talos RP1/PCIe address helpers.
- Selected one read-only host-bridge status discriminator:
  pcie2-host-link-status-read.
- Named the exact allowed address, width, expected invariants, report fields,
  classifications, paired control requirement, and forbidden operations.
- Updated the roadmap and Phase 11 RP1/PCIe map contract for the accepted
  source-contract frontier.

## Non-Goals

No runtime implementation, Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, RP1 peripheral/SYSINFO/clock/GPIO/GIC retry,
PCIe writes, configuration writes, bus mastering, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, or phase transition.

## Findings

- fixed: selected the BCM2712 PCIe2 host-bridge link/status register as a
  qualitatively different discriminator from prior RP1 peripheral, SYSINFO,
  and clock-window reads because it is outside the translated RP1
  0x1f_0000_0000 aperture.
- fixed: used the Broadcom STB PCIe source fact that endpoint config-space
  access is gated by link-up and uses an index write; therefore direct
  endpoint config probing is not accepted in this read-only source contract.
- fixed: tied the selected status bits to the source driver link-up predicate
  before any later endpoint/config/decode work.
- fixed: specified exact allowed operation, report fields, classification
  names, paired control requirements, and forbidden operations.
- deferred: any endpoint config-space read, bridge setup, PERST, MSI, MIP,
  outbound-window programming, BAR discovery, DMA/cache, or bus-mastering work
  requires a later supervisor-planned task with explicit write/restore gates.
- not-an-issue: no restore operation is needed because the accepted
  discriminator is read-only and touches only a host-controller status
  register.

No findings were removed.

## Accepted Source Contract

Contract id:
phase11-rp1-pcie-endpoint-config-discriminator-source-contract-v1

~~~text
target: pcie2-host-link-status-read
operation: read-only BCM2712 PCIe2 host-bridge link/status snapshot
pcie2 controller base: 0x1000120000
status offset: 0x4068
cpu physical address: 0x1000124068
width: 32
~~~

Allowed operation:

| Field | Source offset | CPU physical address | Width | Purpose |
| --- | ---: | ---: | ---: | --- |
| PCIE_MISC_PCIE_STATUS | 0x4068 | 0x1000124068 | 32-bit load | Host-bridge link/decode snapshot before endpoint/config probing |

No other MMIO loads are selected. No writes, index/data config cycles,
PERST/link-control changes, bridge setup, MSI/MIP/GIC operations, RP1
peripheral/SYSINFO/clock/GPIO/GIC operations, DMA/cache operations, or restore
operations are selected.

## Source Reconciliation

- Linux bcm2712.dtsi declares pcie2 as compatible brcm,bcm2712-pcie with
  controller register base 0x10_0012_0000 and non-prefetchable PCIe window
  mapping PCI address 0x00_0000_0000 to CPU physical 0x1f_0000_0000.
- Linux bcm2712-rpi-5-b.dts maps the RP1 peripheral bus
  0xc0_4000_0000..0xc0_4040_ffff into pcie2 PCI address 0x00_0000_0000.
- The accepted SYSINFO/clock-sentinel Pi 5 proof showed RP1 SYSINFO_CHIP_ID,
  SYSINFO_PLATFORM, and CLK_ADC_CTRL all returning 0xdeaddead through that
  translated RP1 aperture.
- Linux pcie-brcmstb.c defines PCIE_MISC_PCIE_STATUS at host-controller offset
  0x4068, with PCIE_PORT bit 0x80, DL_ACTIVE bit 0x20, PHYLINKUP bit 0x10,
  and LINK_IN_L23 bit 0x40.
- The same driver's brcm_pcie_link_up predicate reads PCIE_MISC_PCIE_STATUS
  and treats DL_ACTIVE and PHYLINKUP as the link-up condition.
- The driver explicitly avoids endpoint config-space access when link-up is
  false because such access can cause a CPU abort; endpoint config access also
  uses EXT_CFG_INDEX writes before reading EXT_CFG_DATA. That makes a direct
  endpoint config read unsuitable for this read-only contract.

## Expected Invariants

- pcie_port reports bit 0x80 from PCIE_MISC_PCIE_STATUS.
- dl_active reports bit 0x20 from PCIE_MISC_PCIE_STATUS.
- phylinkup reports bit 0x10 from PCIE_MISC_PCIE_STATUS.
- link_in_l23 reports bit 0x40 from PCIE_MISC_PCIE_STATUS.
- status_is_deaddead reports whether the raw status is 0xdeaddead.
- retained_rp1_window_sentinel is true for the accepted SYSINFO/clock-sentinel
  result where SYSINFO_CHIP_ID, SYSINFO_PLATFORM, and CLK_ADC_CTRL all
  returned 0xdeaddead.
- A non-sentinel host status with dl_active=true and phylinkup=true separates
  visible PCIe2 host/link state from the retained RP1-window sentinel and
  classifies as pcie2-host-link-up-rp1-window-sentinel.
- A non-sentinel host status with either dl_active=false or phylinkup=false
  blocks endpoint/config claims and classifies as
  pcie2-host-status-visible-link-down.
- A 0xdeaddead host status classifies as pcie2-host-status-sentinel and blocks
  same-shaped RP1 endpoint/config/decode probing until a different
  discriminator is planned.

## Report Fields

- contract id and target name.
- pcie2 controller base, register name, source offset, CPU physical address,
  and width.
- raw PCIE_MISC_PCIE_STATUS value.
- pcie_port, dl_active, phylinkup, link_in_l23, and status_is_deaddead.
- retained SYSINFO/clock-sentinel classification and
  retained_rp1_window_sentinel=true.
- terminal classification.

Accepted classifications:

- pcie2-host-link-up-rp1-window-sentinel
- pcie2-host-status-visible-link-down
- pcie2-host-status-sentinel
- pcie2-host-status-inconclusive-capture
- no-mmio-pcie2-host-link-status-control-visible
- staging/build-blocker

The paired no-MMIO/no-RP1/no-GIC control must preserve the same output shape
and classification vocabulary while constructing no BCM2712 PCIe, RP1
peripheral/SYSINFO/clock/GPIO/MSI-X, MIP, or GIC MMIO address and performing
no volatile load/store to those paths.

## Forbidden Operations

- PCIe writes, including EXT_CFG_INDEX, EXT_CFG_DATA, bridge setup,
  CPU-to-PCIe window programming, PERST, link-control, MSI target, MSI data,
  MIP, or GIC writes.
- Endpoint config-space access, BAR discovery, enumeration, bus mastering, or
  DMA/cache operations.
- Same-shaped RP1 peripheral, SYSINFO, clock, GPIO, MSI-X CFG, GIC-visible,
  or clock-window hardware reruns as progress for this contract.
- RP1 clock/reset writes, reset-controller writes, GPIO/RIO/pad writes,
  event generation, interrupt enablement or delivery, GIC acknowledgement,
  ISR installation, storage, generated-root, networking, SSH, Milestone 11.3,
  or phase transition.

## Accepted Claims

This task accepts only the source contract for a read-only BCM2712 PCIe2
host-bridge link/status discriminator and the paired no-MMIO/no-RP1/no-GIC
control requirement. It does not accept runtime behavior, hardware behavior,
live endpoint config access, broad RP1 mapping, endpoint ownership, PCIe
writes, clock/reset ownership, GPIO ownership, event generation, interrupt
delivery, handler ownership, DMA/cache, storage, generated-root, networking,
SSH, Milestone 11.3, or phase transition.

## Evidence

- Source notes:
  tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-source-contract/source-reference-notes.md.
- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-source-contract/evidence-map.json.
- Retained Broadcom STB PCIe source:
  tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-source-contract/pcie-brcmstb.c.
- Updated contract docs:
  docs/src/project/phase11-rp1-pcie-map-contract.md.
- Updated roadmap:
  docs/src/roadmap.md.

## Validation

- Static inspection: accepted Phase 11 task records, roadmap, RP1/PCIe map
  contract, retained Raspberry Pi Linux device-tree files, retained Broadcom
  STB PCIe driver source, and current Talos RP1/PCIe helpers inspected.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

Accepted as accepted-source-contract.

Next mechanically unblocked task:
phase11-rp1-pcie-endpoint-config-discriminator-core-20260608. Implement only
the accepted read-only pcie2-host-link-status discriminator and paired
no-MMIO/no-RP1/no-GIC control; do not acquire hardwareTestLock for the core
task.
