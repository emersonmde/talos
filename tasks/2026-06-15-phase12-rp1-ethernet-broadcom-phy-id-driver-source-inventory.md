# Phase 12 RP1 Ethernet Broadcom PHY ID Driver Source Inventory

Task id: phase12-rp1-ethernet-broadcom-phy-id-driver-source-inventory-20260615

Status: accepted

Classification:
post-physical-broadcom-phy-id-driver-source-inventory-bcm54213pe-source-contract-selected

Evidence level: static/source/task evidence inspection, retained public source
excerpts with SHA-256 checksums, JSON evidence validation, diff checks, and
docs build. No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, power-cycle, TFTP/serial capture, GPIO/RIO/pad
MMIO write, GPIO32 event clear, PHY reset assertion/deassertion, BMCR write,
PHY configuration write, MACB configuration write, packet I/O, networking,
SSH, Phase 12.2, or phase transition was performed.

## Goal

Identify the observed corrected-target PHY from PHYSID1 0x600d / PHYSID2
0x84a2 and gather source-backed Broadcom PHY driver facts that can narrow the
next Phase 12.1 planning step without authorizing a direct hardware retry.

## Scope Performed

- Inspected the accepted post-physical v2 phy-not-ready frontier, GPIO32
  reset-recovery blocker, PHY power/strap checkpoint, retained Raspberry Pi
  devicetree/MACB excerpts, and retained PHY/MDIO evidence maps.
- Retained Raspberry Pi Linux rpi-6.12.y excerpts for Broadcom PHY IDs,
  BCM54213PE driver table matching, BCM54213PE/Broadcom config paths,
  interrupt acknowledgement/handling, APD/EEE helpers, suspend/resume, and
  internal RGMII delay source behavior.
- Mapped the observed Clause 22 ID fields to the exact source-backed model.
- Classified future candidate discriminators against prior same-shaped
  BMCR/BMSR/ANAR/ANLPAR/MACB_NSR sampling, GPIO32 event-clear/write-restore
  attempts, BMCR autoneg-restart retry, and broad MACB/phylink configuration.
- Updated Phase 12 docs and roadmap with the exact PHY model and source/static
  follow-up boundary.

## Findings

- fixed: PHYSID1 0x600d and PHYSID2 0x84a2 combine to 0x600d84a2, which
  Raspberry Pi Linux rpi-6.12.y names PHY_ID_BCM54213PE.
- fixed: the Broadcom driver table matches PHY_ID_BCM54213PE with
  phy_id_mask = 0xffffffff and names the device Broadcom BCM54213PE; this is
  an exact ID match, not only an OUI-family match.
- fixed: the applicable Broadcom OUI-family mask also contains
  PHY_BCM_OUI_4 = 0x600d8400 under PHY_BCM_OUI_MASK = 0xfffffc00, but the
  exact driver-table entry is the controlling model evidence for this task.
- fixed: Raspberry Pi Linux routes BCM54213PE through bcm54xx_config_init(),
  then the BCM54213PE case calls bcm54213pe_config_init(), which reuses the
  BCM54210E path and bcm54xx_config_clock_delay() for RGMII internal delay.
- fixed: with the retained Pi 5 phy-mode = rgmii-id fact, the Linux
  source-backed BCM54213PE path would enable PHY-side RX skew and TX clock
  delay through Broadcom shadow/aux control writes.
- fixed: brcm,powerdown-enable maps to PHY_BRCM_AUTO_PWRDWN_ENABLE, and the
  Broadcom source then adjusts SCR3/APD shadow state; retained devicetree also
  marks eee-broken-1000t and eee-broken-100tx, with Linux PHY core source
  showing EEE broken-mode advertisement handling.
- fixed: Broadcom interrupt support is source-backed as ISR acknowledgement,
  IMR/ECR interrupt configuration, and phy_trigger_machine() from the handler;
  this is useful driver inventory but not a link-readiness discriminator from
  the current Talos frontier.
- fixed: Broadcom suspend/resume source contains BMCR power-down and resume
  reconfiguration paths; those paths are broad lifecycle behavior, not a
  bounded recovery proof for the accepted phy-not-ready sample.
- fixed: the accepted post-physical status remains phy-not-ready with BMCR
  0x1000, BMSR 0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000, MACB_NSR
  0x00000006, BMSR link-status=false, autoneg-complete=false,
  ANLPAR nonzero=false, and MACB_NSR_LINK=false.
- blocked: direct GPIO32 reset recovery remains blocked by persistent or
  firmware-owned GPIO32 event state; exact PHY identification does not weaken
  that no-write boundary.
- deferred: any BCM54213PE register writes, including RGMII delay/APD/EEE/
  interrupt/LED/suspend/resume configuration, require a future explicit source
  contract and are not selected as hardware work here.
- not-an-issue: the accepted physical Ethernet link precondition remains
  accepted and is not re-asked.
- removed: no source, helper, task, or evidence files were removed.

## PHY ID Mapping

The observed corrected-target Clause 22 ID registers are PHYSID1 0x600d and
PHYSID2 0x84a2. Combining them as (PHYSID1 << 16) | PHYSID2 gives 0x600d84a2.

Retained Raspberry Pi Linux source names:

- PHY_ID_BCM54210E as 0x600d84a0;
- PHY_ID_BCM54213PE as 0x600d84a2;
- Broadcom OUI family 4 as 0x600d8400 under mask 0xfffffc00.

The Broadcom driver table then gives PHY_ID_BCM54213PE a phy_id_mask =
0xffffffff and the name Broadcom BCM54213PE. The observed ID therefore maps
exactly to Broadcom BCM54213PE. No unresolved model range is left for the
accepted corrected-target PHY ID.

## Driver Facts

The Linux BCM54213PE path is not merely generic Clause 22 status polling. The
source-backed initialization path includes Broadcom-specific configuration
surfaces:

- bcm54213pe_config_init() reuses bcm54210e_config_init().
- The RGMII delay helper enables or disables Broadcom shadow/aux control bits
  based on the interface mode; for the retained Pi 5 rgmii-id mode, Linux
  enables RXC-RXD skew and GTXCLK internal TX delay.
- brcm,powerdown-enable from devicetree sets PHY_BRCM_AUTO_PWRDWN_ENABLE, and
  the Broadcom helper adjusts SCR3/APD shadow state.
- EEE broken-mode handling is PHY-core source-backed and reconciles the
  retained eee-broken-1000t and eee-broken-100tx properties.
- Interrupt support is source-backed through ISR reads, IMR/ECR configuration,
  and phy_trigger_machine().
- Suspend/resume includes BMCR power-down, IDDQ handling, genphy_resume(), and
  re-running Broadcom config initialization.

These facts are qualitatively different from another passive link-status
sample, GPIO32 event clear, GPIO32 write/restore, or BMCR autoneg-restart
retry. They are not yet safe hardware operations: they include PHY shadow,
MMD, BMCR/lifecycle, interrupt, LED, and possible Broadcom configuration
writes that need a separate source/static contract before Talos can implement
or test anything runtime-facing.

## Candidate Discriminators

- selected: a future source/static contract task,
  phase12-rp1-ethernet-bcm54213pe-config-init-source-contract-20260615, should
  inventory the minimal BCM54213PE init/read-status subset Talos could
  eventually own, split read-only preflight from writes, name exact registers,
  define write/restore or no-write preconditions, and reject hardware proof
  until the contract is accepted.
- deferred: a later read-only Broadcom shadow/EEE/APD/RGMII-delay preflight
  may be considered only if that source contract proves the reads are narrow,
  source-backed, and distinct from prior BMCR/BMSR/ANAR/ANLPAR/MACB_NSR
  sampling.
- blocked: GPIO32 reset recovery remains blocked by the persistent-or-
  firmware-owned GPIO32 event-state frontier.
- rejected: another BMCR/BMSR/ANAR/ANLPAR/MACB_NSR sample repeats the
  accepted post-physical v2 status shape.
- rejected: another BMCR autoneg-restart retry repeats the accepted v2
  link-not-ready recovery shape.
- rejected: direct PHY configuration writes, MACB/phylink configuration,
  packet I/O, networking, SSH, Phase 12.2, and phase transition are outside
  this task and remain forbidden.

## Source Contract Requirements

The selected future source/static contract is not a hardware task. It must
require, at minimum:

- exact BCM54213PE register and helper inventory for Linux config_init,
  read_status/aneg dependency, APD, EEE broken-mode handling, RGMII delay,
  interrupt acknowledgement/configuration, and suspend/resume paths;
- explicit separation of read-only registers from write targets;
- explicit rejection of GPIO32 reset action, BMCR autoneg restart retry,
  broad PHY configuration, MACB configuration, packet I/O, networking, SSH,
  Phase 12.2, and phase transition;
- evidence requirements for retained source excerpts and checksums;
- a decision on whether a later proof can be read-only, must be write/restore,
  or remains blocked.

## Rejected Claims And Retained Risks

Rejected claims:

- link readiness;
- GPIO32 ownership;
- ETH_RST_N reset assertion or deassertion;
- GPIO32 event-clear retry;
- GPIO32 write/restore retry or success;
- BMCR write or autoneg-restart retry;
- PHY configuration write;
- MACB configuration write;
- link forcing;
- reset-controller ownership;
- firmware/event-state ownership;
- interrupt delivery/completion ownership;
- broad MDIO/PHY ownership;
- DMA/descriptors;
- packet I/O;
- networking;
- sockets;
- SSH;
- Phase 12.2;
- phase transition.

Retained risks:

- The PHY may require Broadcom-specific initialization before link can come up,
  but this task does not decide which writes are safe for Talos.
- GPIO32 event bits may still block the reset path independently of PHY model.
- Linux's BCM54213PE path includes multiple write surfaces that can easily
  exceed a thin discriminator if not split by a future source contract.
- Packet I/O and network-stack work remain blocked until link and lower-level
  ownership prerequisites are separately accepted.

## Evidence

- Broadcom ID/RGMII/APD source excerpt:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-broadcom-phy-id-driver-source-inventory/source/linux-rpi-6.12-brcmphy-id-rgmii-apd-excerpt.txt.
- BCM54213PE driver source excerpt:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-broadcom-phy-id-driver-source-inventory/source/linux-rpi-6.12-broadcom-bcm54213pe-driver-excerpt.txt.
- Broadcom interrupt/APD/EEE helper excerpt:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-broadcom-phy-id-driver-source-inventory/source/linux-rpi-6.12-bcm-phy-lib-interrupt-apd-eee-excerpt.txt.
- Linux PHY core EEE/delay excerpt:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-broadcom-phy-id-driver-source-inventory/source/linux-rpi-6.12-phy-device-eee-delay-excerpt.txt.
- Source excerpt checksums:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-broadcom-phy-id-driver-source-inventory/source/sha256sums.txt.
- Inventory classification:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-broadcom-phy-id-driver-source-inventory/classification.json.
- Inventory evidence map:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-broadcom-phy-id-driver-source-inventory/evidence-map.json.
- Accepted PHY power/strap source checkpoint:
  tasks/2026-06-15-phase12-rp1-ethernet-phy-power-strap-source-checkpoint.md.
- Accepted v2 post-physical closeout:
  tasks/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-closeout.md.
- Accepted GPIO32 reset-recovery source checkpoint:
  tasks/2026-06-15-phase12-rp1-ethernet-post-physical-gpio32-reset-recovery-source-checkpoint.md.
- Retained Raspberry Pi Linux devicetree/MACB source excerpts:
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-bcm2712-rpi-5-b.dts,
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi, and
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c.

## Validation

- static/source/task evidence inspection: accepted post-physical v2 closeout,
  GPIO32 reset-recovery checkpoint, PHY power/strap checkpoint, prior PHY1
  status/link/MAC/autoneg closeouts, retained Raspberry Pi Linux source, new
  Broadcom/Linux source excerpts, Phase 12 docs, roadmap, and git history
  inspected.
- source checksums: SHA-256 recorded for newly retained Broadcom/Linux source
  excerpts.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list with dispositions: satisfied.
- Retained/new source excerpts have provenance and task-owned checksums:
  satisfied.
- PHYSID1 0x600d / PHYSID2 0x84a2 maps exactly to Broadcom BCM54213PE:
  satisfied.
- Driver facts reconciled with accepted phy-not-ready status, GPIO32 blocker,
  MACB/phylink boundary, and prior no-distinct-discriminator checkpoint:
  satisfied.
- Candidate discriminators classified as selected, deferred, blocked, or
  rejected: satisfied.
- Selected future task is source/static contract id, not a hardware proof:
  satisfied.
- No hardware, GPIO32, BMCR, PHY/MACB configuration, packet I/O, networking,
  SSH, Phase 12.2, or phase transition was performed: satisfied.

## Next Action

Set planningNeeded=true for supervisor planning of the selected source/static
contract task phase12-rp1-ethernet-bcm54213pe-config-init-source-contract-20260615
or an explicit alternate/pause. Do not start hardware, GPIO32 event clear,
GPIO32 reset recovery, BMCR write, PHY or MACB configuration, packet I/O,
networking, SSH, Phase 12.2, or a phase transition from this inventory.
