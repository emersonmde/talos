# Phase 11 RP1 Bridge/Setup Source Contract

Task id: phase11-rp1-bridge-setup-source-contract-20260608

Status: accepted

Classification: accepted-source-contract

## Goal

Select the smallest source-backed bridge/setup discriminator after the
accepted bridge/config preflight ready result, without accepting endpoint
ownership, broad RP1 mapping, BAR discovery/programming, or a phase
transition.

## Scope

- Reviewed retained Raspberry Pi Linux BCM2712/RP1 device-tree sources,
  retained Broadcom STB PCIe host-driver setup/config-access source, accepted
  endpoint config identity all-ones evidence, and accepted bridge/config
  preflight ready closeout evidence.
- Selected one read-only bridge setup-state discriminator:
  pcie2-bridge-setup-state-read.
- Defined exact allowed reads, source offsets, bit masks, ordering,
  classifications, paired-control constraints, and forbidden operations.
- Preserved the accepted boundary: no endpoint config retry, no endpoint
  ownership claim, no broad RP1 mapping claim, no BAR work, no runtime writes,
  and no phase transition are implied.
- Updated roadmap and Phase 11 RP1/PCIe map contract docs for the accepted
  source-contract frontier.
- Recorded findings with disposition.

## Non-Goals

No runtime implementation, hardware run, boot archive publication,
hardwareTestLock acquisition, endpoint configuration access or mutation, BAR
discovery or programming, bridge setup writes, CPU-to-PCIe window programming,
inbound-window programming, PERST/link-control change, root-complex class
write, MSI/MIP/GIC operations, interrupt enablement or delivery, GIC
acknowledgement, ISR installation, RP1 peripheral/SYSINFO/clock/GPIO retry,
RP1 clock/reset writes, GPIO/RIO/pad writes, event generation, DMA/cache,
storage, generated-root, networking, SSH, Milestone 11.3, or phase transition.

## Findings And Disposition

- fixed: selected a read-only setup-state discriminator rather than a
  reversible/idempotent write. The accepted bridge/config preflight already
  showed SCB_ACCESS_EN and CFG_READ_UR_MODE set, so the next smaller source
  question is whether later bridge setup state is visible before any mutation.
- fixed: tied the selected registers to retained Broadcom STB PCIe setup
  source. brcm_pcie_setup() writes root-complex class code 0x060400 through
  PCIE_RC_CFG_PRIV1_ID_VAL3 and programs outbound window 0 with
  brcm_pcie_set_outbound_win() from the Linux host bridge windows.
- fixed: tied expected pcie2 window shape to retained BCM2712/RP1
  device-tree sources. bcm2712.dtsi defines pcie2 non-prefetchable PCIe
  address 0x00_0000_0000 at CPU physical 0x1f_0000_0000, and
  bcm2712-rpi-5-b.dts maps RP1 bus 0xc0_4000_0000 through that pcie2 window.
- fixed: classified the selected reads as setup-state evidence only. A visible
  root-complex class and outbound window shape may justify later supervisor
  planning for a different endpoint visibility or narrower bridge/BAR slice,
  but it is not proof that RP1 responds, that endpoint config should be
  retried, or that Talos owns PCIe setup.
- fixed: required a paired no-MMIO/no-PCIe/no-RP1/no-GIC control that
  preserves output shape and classification vocabulary while constructing no
  forbidden MMIO address.
- deferred: endpoint visibility retry, endpoint ownership, BAR discovery,
  BAR programming, bridge setup writes, PERST/link-control, interrupt
  delivery, DMA/cache, and broad RP1 mapping require later supervisor-planned
  tasks if this discriminator justifies them.
- not-an-issue: no restore/quarantine operation is required because the
  selected discriminator is read-only and does not mutate controller,
  endpoint, bridge, RP1, GIC, or DMA state.

No findings were removed.

## Accepted Source Contract

Contract id:
phase11-rp1-bridge-setup-source-contract-v1

~~~text
target: pcie2-bridge-setup-state-read
operation: read-only BCM2712 PCIe2 bridge setup-state snapshot
pcie2 controller base: 0x1000120000
status register: PCIE_MISC_PCIE_STATUS at offset 0x4068
preflight register: PCIE_MISC_MISC_CTRL at offset 0x4008
root-complex id/class register: PCIE_RC_CFG_PRIV1_ID_VAL3 at offset 0x043c
outbound win0 pcie low: PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LO at offset 0x400c
outbound win0 pcie high: PCIE_MISC_CPU_2_PCIE_MEM_WIN0_HI at offset 0x4010
outbound win0 CPU base/limit low: PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_LIMIT at offset 0x4070
outbound win0 CPU base high: PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_HI at offset 0x4080
outbound win0 CPU limit high: PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LIMIT_HI at offset 0x4084
width: 32 for every selected read
~~~

Allowed sequence:

1. Read PCIE_MISC_PCIE_STATUS at 0x1000124068 and decode DL_ACTIVE (0x20),
   PHYLINKUP (0x10), PCIE_PORT (0x80), and LINK_IN_L23 (0x40).
2. Read PCIE_MISC_MISC_CTRL at 0x1000124008 and decode SCB_ACCESS_EN
   (0x1000) and CFG_READ_UR_MODE (0x2000) as the accepted preflight
   predicate.
3. Read PCIE_RC_CFG_PRIV1_ID_VAL3 at 0x100012043c and decode class code as
   raw & 0x00ff_ffff. The expected source setup class is 0x060400
   (PCIe-to-PCIe bridge).
4. Read outbound window 0 registers:
   - PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LO at 0x100012400c.
   - PCIE_MISC_CPU_2_PCIE_MEM_WIN0_HI at 0x1000124010.
   - PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_LIMIT at 0x1000124070.
   - PCIE_MISC_CPU_2_PCIE_MEM_WIN0_BASE_HI at 0x1000124080.
   - PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LIMIT_HI at 0x1000124084.
5. Decode the source-expected pcie2 non-prefetchable window shape:
   - pcie address low dword = 0x00000000.
   - pcie address high dword = 0x00000000.
   - base_limit base field mask 0x0000fff0, expected field value
     0x00000000 for CPU base 0x1f_0000_0000.
   - base_limit limit field mask 0xfff00000, expected field value
     0xfff00000 for the retained nearly-4GiB non-prefetchable pcie2 window.
   - base_hi mask 0x000000ff, expected value 0x0000001f.
   - limit_hi mask 0x000000ff, expected value 0x0000001f.

No other MMIO load is selected. No MMIO store is selected.

## Source Reconciliation

- bcm2712.dtsi declares pcie2 as compatible brcm,bcm2712-pcie with
  controller register base 0x10_0012_0000, PCI domain 2, reset names
  rescal/bridge, MIP0 MSI parent, and a non-prefetchable downstream PCIe
  window mapping PCIe 0x00_0000_0000 to CPU physical 0x1f_0000_0000.
- bcm2712-rpi-5-b.dts binds rp1_target to &pcie2, sets pcie2 status = okay,
  and maps RP1 bus 0xc0_4000_0000..0xc0_4040_ffff into pcie2 PCI address
  0x00_0000_0000.
- The accepted endpoint config identity Pi 5 proof reached the PCIe2 link-up
  precondition but EXT_CFG_DATA + 0 for BDF 0002:01:00.0 returned
  0xffffffff.
- The accepted bridge/config preflight Pi 5 proof reached the same link-up
  precondition and reported PCIE_MISC_MISC_CTRL=0xa8003000 with
  SCB_ACCESS_EN=true and CFG_READ_UR_MODE=true.
- Retained pcie-brcmstb.c brcm_pcie_setup() writes root-complex class code
  0x060400 through PCIE_RC_CFG_PRIV1_ID_VAL3 after MISC_CTRL setup and before
  iterating the host bridge windows.
- Retained pcie-brcmstb.c brcm_pcie_set_outbound_win() programs
  PCIE_MISC_CPU_2_PCIE_MEM_WIN0_LO/HI and BASE_LIMIT/BASE_HI/LIMIT_HI from
  the CPU start, PCIe start, and size of the first IORESOURCE_MEM window.
- This source contract reads only the already-programmed state. It does not
  write root-complex class, reprogram CPU-to-PCIe windows, touch inbound
  windows, access EXT_CFG_INDEX/EXT_CFG_DATA, discover/program BARs, or claim
  RP1 endpoint visibility.

## Report Fields

- contract id and target name.
- pcie2 controller base, register names, source offsets, CPU physical
  addresses, and widths.
- raw PCIE_MISC_PCIE_STATUS, decoded pcie_port, dl_active, phylinkup,
  link_in_l23, and status_is_deaddead.
- raw PCIE_MISC_MISC_CTRL, decoded scb_access_en, cfg_read_ur_mode, and
  misc_ctrl_is_sentinel.
- raw PCIE_RC_CFG_PRIV1_ID_VAL3, decoded class_code, and
  class_code_is_pcie_bridge.
- raw outbound window 0 LO, HI, BASE_LIMIT, BASE_HI, and LIMIT_HI values.
- decoded pcie_base_is_zero, cpu_base_low_matches, cpu_limit_low_matches,
  cpu_base_high_matches, cpu_limit_high_matches, and outbound_window0_matches.
- retained endpoint config identity classification
  rp1-endpoint-config-id-all-ones.
- terminal classification.

Accepted classifications:

- pcie2-bridge-setup-state-visible
- pcie2-bridge-setup-state-incomplete
- pcie2-bridge-setup-state-sentinel
- pcie2-bridge-setup-state-link-down-skip
- pcie2-bridge-setup-state-inconclusive-capture
- no-mmio-pcie2-bridge-setup-state-control-visible
- staging/build-blocker

Classification rules:

- pcie2-bridge-setup-state-visible: status is non-sentinel, dl_active=true,
  phylinkup=true, PCIE_MISC_MISC_CTRL is non-sentinel with SCB_ACCESS_EN and
  CFG_READ_UR_MODE set, class_code is 0x060400, and outbound window 0 matches
  the source-expected PCIe 0 -> CPU 0x1f_0000_0000 window shape.
- pcie2-bridge-setup-state-incomplete: status and selected setup-state
  registers are visible, but one or more of SCB_ACCESS_EN, CFG_READ_UR_MODE,
  class code 0x060400, or outbound window 0 expected fields is missing.
- pcie2-bridge-setup-state-sentinel: any selected setup-state register needed
  for terminal classification reads as 0xdeaddead, 0xffffffff, or 0x00000000
  in a position where the source-expected value is not zero.
- pcie2-bridge-setup-state-link-down-skip: status is visible but either
  dl_active or phylinkup is false; setup-state values may be reported, but no
  endpoint config retry or bridge claim is accepted.
- pcie2-bridge-setup-state-inconclusive-capture: artifact identity, TFTP
  evidence, serial window, or restore evidence prevents a decisive hardware
  classification.
- no-mmio-pcie2-bridge-setup-state-control-visible: paired control output
  shape is visible without constructing forbidden MMIO addresses.
- staging/build-blocker: build, archive review, or staging prevents a
  candidate from reaching hardware evidence.

## Paired Control Constraints

The paired control must preserve the same output shape, field names, and
classification vocabulary while constructing no BCM2712 PCIe, RP1
peripheral/SYSINFO/clock/GPIO/MSI-X, MIP, GIC, DMA, or other MMIO address and
performing no volatile load or store. It must classify as
no-mmio-pcie2-bridge-setup-state-control-visible.

## Forbidden Operations

- Same-shaped endpoint config identity hardware reruns.
- EXT_CFG_INDEX writes, EXT_CFG_DATA reads or writes, endpoint config offset
  probing, BAR discovery, BAR programming, bridge setup writes,
  CPU-to-PCIe window programming, inbound-window programming,
  PERST/link-control changes, root-complex class writes, bus mastering, or
  DMA/cache operations.
- MSI/MIP/GIC operations, interrupt enablement or delivery, GIC
  acknowledgement, ISR installation, RP1 peripheral/SYSINFO/clock/GPIO/GIC
  retries, RP1 clock/reset writes, GPIO/RIO/pad writes, event generation,
  storage, generated-root, networking, SSH, Milestone 11.3, or phase
  transition.

## Restore And Quarantine

No restore/quarantine action is selected or required by this source contract
because it is read-only. Hardware tasks that implement this contract still
must restore the published boot tree after the serialized Pi 5 run.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-source-contract/evidence-map.json.
- Retained device-tree sources:
  tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712.dtsi,
  tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi-5-b.dts,
  tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/rp1.dtsi.
- Retained Broadcom STB PCIe source:
  tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-source-contract/pcie-brcmstb.c.
- Accepted bridge/config preflight closeout:
  tasks/2026-06-08-phase11-rp1-bridge-config-preflight-closeout.md.

## Validation

- static inspection: retained Broadcom STB PCIe source, BCM2712/RP1
  device-tree sources, accepted endpoint/config and bridge/preflight task
  records, current Talos RP1/PCIe constants, roadmap, and project contract
  docs inspected.
- jq evidence map check: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted source contract:
phase11-rp1-bridge-setup-source-contract-v1.

Next action: promote phase11-rp1-bridge-setup-core-20260608 only after this
accepted source contract is committed and no supervisor intervention is active.
