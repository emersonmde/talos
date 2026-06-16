# Phase 12 RP1 Ethernet BCM54213PE Config-Init Source Contract

Task id: phase12-rp1-ethernet-bcm54213pe-config-init-source-contract-20260615

Status: accepted

Classification:
bcm54213pe-config-init-source-contract-readonly-preflight-contract-selected

Evidence level: static/source/task evidence inspection, retained public source
excerpts with SHA-256 checksums, JSON evidence validation, diff checks, and
docs build. No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, power-cycle, TFTP/serial capture, GPIO/RIO/pad
MMIO write, GPIO32 event clear, PHY reset assertion/deassertion, BMCR write,
Broadcom shadow/MMD/aux write, PHY configuration write, MACB configuration
write, packet I/O, networking, SSH, Phase 12.2, or phase transition was
performed.

## Goal

Turn the accepted exact BCM54213PE source inventory into a bounded source/static
contract for the minimal Linux-backed config_init and read_status surfaces Talos
may later own, without authorizing hardware or PHY/MAC writes.

## Scope Performed

- Inspected the accepted Broadcom PHY ID/driver source inventory, accepted
  post-physical phy-not-ready status, accepted GPIO32 persistent-event-state
  blocker, accepted PHY power/strap checkpoint, and retained Raspberry Pi Linux
  devicetree/MACB excerpts.
- Retained Raspberry Pi Linux rpi-6.12.y excerpts for BCM54213PE config_init,
  Broadcom register definitions, APD/powerdown, EEE, interrupt acknowledgement
  and configuration, suspend/resume, generic autoneg/read_status dependencies,
  and PHY-state dispatch.
- Inventoried the Linux helper paths and register surfaces that are relevant to
  BCM54213PE config_init, read_status/aneg, APD, EEE broken-mode handling,
  RGMII delay, interrupt handling, and suspend/resume.
- Split read-only candidates from write targets and classified each potential
  future action as selected, deferred, blocked, rejected, or already sampled.
- Updated Phase 12 docs and roadmap with the accepted source/static contract
  boundary and selected source/static follow-up.

## Findings

- fixed: the BCM54213PE Linux driver-table entry uses
  config_init=bcm54xx_config_init, config_intr=bcm_phy_config_intr,
  handle_interrupt=bcm_phy_handle_interrupt, suspend=bcm54xx_suspend, and
  resume=bcm54xx_resume.
- fixed: the BCM54213PE model-specific config path is
  bcm54xx_config_init -> bcm54213pe_config_init -> bcm54210e_config_init ->
  bcm54xx_config_clock_delay.
- fixed: for the retained Pi 5 phy-mode = rgmii-id context, Linux's RGMII delay
  path would write Broadcom AUX/shadow state to enable RXC-RXD skew and GTXCLK
  internal TX delay; this is a write/restore surface, not a current hardware
  discriminator.
- fixed: Linux read_status/aneg dependencies add MII_STAT1000 and MII_CTRL1000
  context around the already accepted BMCR/BMSR/ANAR/ANLPAR status surface, but
  the current accepted frontier already reports no link, no autoneg completion,
  and ANLPAR 0x0000.
- fixed: Broadcom interrupt support involves MII_BCM54XX_ISR reads,
  MII_BCM54XX_IMR writes, MII_BCM54XX_ECR writes, and phy_trigger_machine()
  from the handler; ISR reads may acknowledge pending interrupts and therefore
  require source/static side-effect classification before hardware.
- fixed: APD/powerdown and EEE handling use Broadcom shadow registers and
  Clause 45/MMD EEE registers; they are relevant source facts but not selected
  as writes or runtime behavior here.
- fixed: suspend/resume includes BMCR power-down, IDDQ, genphy_resume(),
  optional soft reset, and bcm54xx_config_init re-entry; these lifecycle paths
  are too broad for the current phy-not-ready discriminator.
- blocked: direct GPIO32 reset recovery remains blocked by persistent or
  firmware-owned GPIO32 event state; this contract does not weaken the
  accepted no-write boundary.
- blocked: no direct Pi 5 hardware proof is authorized by this contract because
  the smallest potentially distinct read surfaces still need a source/static
  access contract for side effects, selector mechanics, and inconclusive
  classification.
- deferred: RGMII delay, APD, EEE, interrupt mask/control, BMCR lifecycle, LED,
  WOL, PTP, and Broadcom shadow/MMD writes require future write/restore
  contracts if they are ever considered.
- rejected: link readiness, GPIO32 ownership/reset, BMCR autoneg retry, broad
  PHY/MAC configuration, packet I/O, networking, SSH, Phase 12.2, and phase
  transition remain rejected.
- not-an-issue: the accepted physical Ethernet link precondition remains
  accepted and is not re-asked.
- removed: no source, helper, task, or evidence files were removed.

## Helper And Register Contract

The accepted exact PHY model remains Broadcom BCM54213PE from PHYSID1 0x600d
and PHYSID2 0x84a2, combined as 0x600d84a2. The Linux driver-table match is
exact, with phy_id_mask = 0xffffffff.

The relevant Linux helper paths are:

- config_init: bcm54xx_config_init -> case PHY_ID_BCM54213PE ->
  bcm54213pe_config_init -> bcm54210e_config_init ->
  bcm54xx_config_clock_delay.
- read_status: phy_check_link_status -> phy_read_status ->
  genphy_read_status -> genphy_update_link -> genphy_read_master_slave /
  genphy_read_lpa -> phy_resolve_aneg_linkmode when autoneg_complete is true.
- aneg config: phy_config_aneg -> genphy_config_aneg ->
  __genphy_config_aneg -> genphy_c45_an_config_eee_aneg ->
  genphy_setup_master_slave -> genphy_config_advert ->
  genphy_check_and_restart_aneg.
- APD/powerdown: brcm,powerdown-enable -> PHY_BRCM_AUTO_PWRDWN_ENABLE ->
  bcm_phy_enable_apd -> BCM54XX_SHD_SCR3 / BCM54XX_SHD_APD.
- EEE broken-mode: of_set_phy_eee_broken -> genphy_config_eee_advert ->
  MDIO_AN_EEE_ADV, plus Broadcom BRCM_CL45VEN_EEE_CONTROL and
  BCM_CL45VEN_EEE_ADV helper context.
- RGMII delay: bcm54xx_config_clock_delay over MII_BCM54XX_AUX_CTL shadow
  MII_BCM54XX_AUXCTL_SHDWSEL_MISC and BCM54810_SHD_CLK_CTL.
- interrupts: bcm_phy_ack_intr, bcm_phy_config_intr, and
  bcm_phy_handle_interrupt over MII_BCM54XX_ISR, MII_BCM54XX_IMR, and
  MII_BCM54XX_ECR.
- suspend/resume: bcm54xx_suspend, BMCR_PDOWN, bcm54xx_iddq_set,
  bcm54xx_resume, genphy_resume, optional genphy_soft_reset, and
  bcm54xx_config_init re-entry.

## Read-Only Candidates

Read-only candidates are not hardware authorization. They are the only surfaces
that a future source/static follow-up may consider.

- MII_BMCR 0x00 and MII_BMSR 0x01 are already sampled in the accepted v2
  phy-not-ready frontier. They may be baseline context only.
- MII_LPA 0x05 is already represented by accepted ANLPAR 0x0000.
- MII_STAT1000 0x0a is source-backed by genphy_read_lpa and
  genphy_read_master_slave. It is potentially distinct only if a future
  contract defines when reading gigabit/master-slave partner state is useful
  despite the current no-autoneg-complete frontier.
- MII_CTRL1000 0x09 is read in generic error paths but is also adjacent to the
  BCM54210E/BCM54213PE master-mode write path. A future contract may permit a
  read-only baseline only if CTL1000_AS_MASTER and CTL1000_ENABLE_MASTER writes
  remain forbidden.
- MII_BCM54XX_ISR 0x1a is source-backed as interrupt status, but Linux uses ISR
  reads to acknowledge pending interrupts. A future proof must classify this as
  read-with-side-effect unless source evidence proves otherwise.
- MII_BCM54XX_ECR 0x10 and MII_BCM54XX_IMR 0x1b may be interrupt context reads
  only; their mask/unmask writes remain deferred.
- Broadcom AUX/shadow reads for MII_BCM54XX_AUXCTL_SHDWSEL_MISC and
  BCM54810_SHD_CLK_CTL are potential RGMII-delay context only if a future
  contract proves Talos can read them without setting WREN or changing selector
  state.
- BRCM_CL45VEN_EEE_CONTROL and BCM_CL45VEN_EEE_ADV are deferred Clause 45/MMD
  read-only candidates. No MMD write is selected.

## Write Targets

The following source-backed Linux writes are explicitly not authorized:

- MII_BCM54XX_IMR 0x1b writes from bcm54xx_config_init.
- MII_BCM54XX_ECR 0x10 ECR_IM writes from bcm_phy_config_intr.
- Broadcom AUX/shadow writes from bcm54xx_config_clock_delay, including
  RXC-RXD skew and GTXCLK internal TX delay.
- MII_CTRL1000 0x09 master-mode writes from bcm54210e_config_init.
- BCM54XX_SHD_SCR3 and BCM54XX_SHD_APD writes from bcm_phy_enable_apd.
- MDIO_AN_EEE_ADV, BRCM_CL45VEN_EEE_CONTROL, and BCM_CL45VEN_EEE_ADV writes
  from generic and Broadcom EEE helpers.
- MII_BMCR 0x00 writes from suspend/resume, soft reset, forced mode, and
  autoneg restart paths.
- Broadcom LED, WOL, PTP, and expansion/shadow writes in bcm54xx_config_init.

Any future write work needs a separate explicit source contract with baseline,
write/restore, rollback/reset, hardware lock, evidence, and rejected-claim
rules. The current task selects none of it.

## Reconciliation

The accepted post-physical status remains phy-not-ready: BMCR 0x1000, BMSR
0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000, MACB_NSR 0x00000006, BMSR link
false, autoneg complete false, ANLPAR nonzero false, and MACB_NSR_LINK false.
Another BMCR/BMSR/ANAR/ANLPAR/MACB_NSR sample would repeat that accepted
shape. Another BMCR autoneg-restart attempt would repeat the accepted v2
link-not-ready recovery shape.

The accepted physical Ethernet link precondition remains accepted and is not a
question for this contract. The relevant uncertainty is now source/ownership
specific: which BCM54213PE register surfaces can Talos safely read or later
restore after writes without crossing into GPIO32 reset, broad PHY
configuration, or MACB/phylink behavior.

The GPIO32 blocker remains decisive for reset recovery. GPIO32 write/restore v2
stopped before GPIO/RIO/pad writes with event bits 0x0ab00000, and the accepted
GPIO32 CTRL SET IRQRESET clear attempt left event bits 0x08800000. Exact
BCM54213PE identification does not prove those bits harmless and does not
authorize ETH_RST_N reset action.

MACB/phylink remains a boundary, not a selected runtime path. Linux phylink and
MACB configuration would involve MAC-side state and possible MACB writes beyond
this static PHY contract. No packet I/O, networking, sockets, SSH, Phase 12.2,
or phase transition is accepted.

## Future Discriminator Decision

No direct hardware proof is selected.

The selected follow-up is only a future source/static contract for a
BCM54213PE read-only preflight. That future task, if the supervisor queues it,
must decide whether MII_STAT1000, MII_CTRL1000, interrupt context reads, EEE
MMD reads, or Broadcom shadow/AUX reads can form a safe, source-backed,
read-only discriminator distinct from the accepted status sample. It must also
classify read side effects, especially ISR acknowledgement and shadow selector
mechanics, before any Pi 5 proof can be proposed.

Minimum preconditions for that future task:

- preserve the accepted BCM54213PE exact ID and phy-not-ready status frontier;
- preserve the accepted physical-link precondition without re-asking Matthew;
- preserve the GPIO32 persistent-event-state blocker and forbid
  GPIO32/RIO/pad/MMIO writes;
- classify every target as pure read-only, read-with-side-effect,
  write/restore, blocked, or rejected;
- define hardware evidence, restore/rollback rules, and inconclusive-run triage
  before any later Pi 5 proof.

## Rejected Claims And Retained Risks

Rejected claims:

- link readiness;
- GPIO32 ownership;
- GPIO32 reset recovery;
- ETH_RST_N reset assertion or deassertion;
- GPIO32 event-clear retry;
- GPIO32 write/restore retry or success;
- BMCR write or autoneg-restart retry;
- broad PHY configuration;
- MACB configuration;
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

- The PHY may require Broadcom-specific RGMII delay, APD, EEE, interrupt, or
  power lifecycle configuration before link can come up, but this task does not
  choose those writes.
- Some apparently read-only Broadcom surfaces may have side effects or require
  selector writes that are not yet modeled in Talos.
- GPIO32 event bits may still independently block any reset-based recovery.
- Packet I/O and network-stack work remain blocked until link and lower-level
  ownership prerequisites are separately accepted.

## Evidence

- Task classification:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract/evidence-map.json.
- Source excerpt checksums:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract/source/sha256sums.txt.
- BCM54213PE config source excerpt:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract/source/linux-rpi-6.12-broadcom-bcm54213pe-config-contract-excerpt.txt.
- Broadcom register definition excerpt:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract/source/linux-rpi-6.12-brcmphy-register-contract-excerpt.txt.
- Broadcom helper source excerpt:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract/source/linux-rpi-6.12-bcm-phy-lib-contract-excerpt.txt.
- Generic PHY aneg/read_status excerpts:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract/source/linux-rpi-6.12-phy-device-aneg-status-contract-excerpt.txt,
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract/source/linux-rpi-6.12-phy-device-read-status-register-contract-excerpt.txt,
  and
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract/source/linux-rpi-6.12-phy-dispatch-contract-excerpt.txt.
- Accepted Broadcom PHY ID/driver inventory:
  tasks/2026-06-15-phase12-rp1-ethernet-broadcom-phy-id-driver-source-inventory.md.
- Accepted PHY power/strap checkpoint:
  tasks/2026-06-15-phase12-rp1-ethernet-phy-power-strap-source-checkpoint.md.
- Accepted post-physical link-status v2 closeout:
  tasks/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-closeout.md.
- Accepted GPIO32 reset-recovery source checkpoint:
  tasks/2026-06-15-phase12-rp1-ethernet-post-physical-gpio32-reset-recovery-source-checkpoint.md.
- Retained Raspberry Pi Linux devicetree/MACB excerpts:
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-bcm2712-rpi-5-b.dts,
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi, and
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c.

## Validation

- static/source/task evidence inspection: accepted Broadcom PHY ID/driver
  inventory, accepted PHY power/strap checkpoint, accepted post-physical v2
  status closeout, accepted GPIO32 reset-recovery checkpoint, retained
  Raspberry Pi Linux devicetree/MACB source excerpts, and newly retained Linux
  Broadcom/PHY source excerpts inspected.
- source checksums: sha256sum -c on the task-owned source/sha256sums.txt.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list with dispositions: satisfied.
- Source excerpts cite repository/ref/path provenance and task-owned SHA-256
  checksums: satisfied.
- Exact Linux helper paths and register surfaces for config_init, read_status,
  autoneg, APD/powerdown, EEE, RGMII delay, interrupts, and suspend/resume are
  named: satisfied.
- Read-only candidates are separated from write targets and restore/blocker
  requirements are identified: satisfied.
- Contract reconciles the accepted phy-not-ready status, GPIO32 blocker,
  physical-link precondition, MACB/phylink boundary, and rejected retry shapes:
  satisfied.
- Future discriminator decision is explicit: no direct hardware proof; selected
  follow-up is only a future source/static BCM54213PE read-only preflight
  contract requiring supervisor planning: satisfied.
- Rejected link readiness, GPIO32 ownership/reset, BMCR retry, broad PHY/MAC
  configuration, packet I/O, networking, SSH, Phase 12.2, and phase transition:
  satisfied.

## Next Action

Set planningNeeded=true for supervisor selection of the next explicit bounded
task or an explicit pause. Do not start hardware, GPIO32 event clear, GPIO32
reset recovery, BMCR write, Broadcom shadow/MMD/aux write, PHY or MACB
configuration, packet I/O, networking, SSH, Phase 12.2, or phase transition
from this contract.
