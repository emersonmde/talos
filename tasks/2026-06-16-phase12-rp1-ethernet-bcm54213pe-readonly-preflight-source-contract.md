# Phase 12 RP1 Ethernet BCM54213PE Read-Only Preflight Source Contract

Task id: phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract-20260616

Status: accepted

Classification:
bcm54213pe-readonly-preflight-source-contract-report-core-selected

Evidence level: static/source/task evidence inspection, retained public source
excerpts with SHA-256 checksums, JSON evidence validation, diff checks, and
docs build. No Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, power-cycle, TFTP/serial capture, GPIO/RIO/pad
MMIO write, GPIO32 event clear, PHY reset assertion/deassertion, BMCR write,
Broadcom shadow/MMD/aux write, PHY configuration write, MACB configuration
write, packet I/O, networking, SSH, Phase 12.2, or phase transition was
performed.

## Goal

Convert the accepted BCM54213PE config-init source contract into a source/static
read-only preflight contract that decides whether any BCM54213PE register reads
are safe, distinct, and useful before a later hardware proof.

## Scope Performed

- Inspected the accepted BCM54213PE config-init source contract, retained
  Broadcom and generic PHY source excerpts, accepted phy-not-ready status,
  accepted GPIO32 persistent-event-state blocker, and prior rejected retry
  shapes.
- Retained additional Raspberry Pi Linux rpi-6.12.y excerpts for generic MII
  register definitions, Clause 45/MMD indirect read selector mechanics, and
  Broadcom AUX/shadow selector-read mechanics.
- Classified MII_STAT1000, MII_CTRL1000, MII_BCM54XX_ISR, MII_BCM54XX_ECR,
  MII_BCM54XX_IMR, Broadcom AUX/shadow delay context, Clause 45/MMD EEE
  candidates, and already sampled status surfaces.
- Selected only the future local/static report-core boundary for
  MII_CTRL1000 and MII_STAT1000. No hardware proof is selected by this task.

## Findings

- fixed: the candidate set is split into pure Clause 22 reads, read-with-side
  effect, selector-write-required, write-adjacent, blocked, rejected, and
  already-sampled surfaces.
- selected: MII_CTRL1000 0x09 and MII_STAT1000 0x0a are the only selected
  targets for the queued local/static report-core follow-up.
- already-sampled: BMCR, BMSR, ANAR/ANLPAR, and MACB_NSR are accepted
  phy-not-ready context only; repeating that same shape is rejected.
- rejected: MII_BCM54XX_ISR is not selected because retained Linux source uses
  ISR reads as interrupt acknowledgement/clear-pending.
- deferred: MII_BCM54XX_ECR and MII_BCM54XX_IMR reads are interrupt context
  only; ECR/IMR writes remain forbidden pending a separate write/restore
  contract.
- blocked: Broadcom AUX/shadow delay reads and Clause 45/MMD EEE reads require
  selector writes before reading, so they are not pure read-only targets.
- blocked: direct GPIO32 reset recovery remains blocked by persistent or
  firmware-owned GPIO32 event state; this contract does not weaken the
  accepted no-write boundary.
- rejected: link readiness, GPIO32/PHY reset ownership, BMCR autoneg retry,
  same-shaped BMCR/BMSR/ANAR/ANLPAR/MACB_NSR retry, broad PHY/MAC
  configuration, packet I/O, networking, SSH, Phase 12.2, and phase transition
  remain rejected.
- not-an-issue: the accepted physical Ethernet link precondition remains
  accepted and was not re-asked.
- removed: no source, helper, task, or evidence files were removed.

## Read-Surface Classification

MII_STAT1000 0x0a is a pure Clause 22 read in retained Linux source. Linux
uses it in genphy_read_master_slave() and, after autonegotiation completion,
genphy_read_lpa(). It can expose 1000BASE-T master/slave and link-partner
status context that is materially different from the accepted BMCR/BMSR/
ANAR/ANLPAR/MACB_NSR sample. It cannot prove link readiness while the accepted
BMSR autoneg-complete bit remains false. It is selected for the local/static
report-core follow-up.

MII_CTRL1000 0x09 is a pure Clause 22 read with write-adjacent risk. Retained
Linux source reads it in genphy_read_master_slave() and as interpretation
context for MII_STAT1000 master/slave failure, while the BCM54210E/BCM54213PE
config path may write CTL1000_AS_MASTER and CTL1000_ENABLE_MASTER when
PHY_BRCM_EN_MASTER_MODE is enabled. The read is selected as report context;
all CTRL1000 writes remain forbidden.

MII_BCM54XX_ISR 0x1a is read-with-side-effect. Linux's bcm_phy_ack_intr()
labels the read as clearing pending interrupts, and bcm_phy_handle_interrupt()
also reads ISR before applying IMR mask context. It is rejected from the
read-only preflight report core.

MII_BCM54XX_ECR 0x10 and MII_BCM54XX_IMR 0x1b have read paths in retained
interrupt source, but their value is interrupt configuration context rather
than a distinct phy-not-ready discriminator. They are deferred; mask/unmask
writes remain forbidden.

Broadcom AUX/shadow delay context is blocked from the read-only target set.
Retained Linux source shows bcm54xx_auxctl_read() writes MII_BCM54XX_AUX_CTL
selector bits before reading, bcm_phy_read_misc() writes AUX and expansion
selectors, and bcm54xx_config_clock_delay() writes RGMII delay state. That
surface needs a separate selector/write-restore contract before hardware.

Clause 45/MMD EEE candidates are blocked from the read-only target set. For a
Clause 22 PHY, retained Linux mmd_phy_read() writes MII_MMD_CTRL, MII_MMD_DATA,
and then MII_MMD_CTRL with MII_MMD_CTRL_NOINCR before reading MII_MMD_DATA.
EEE state remains source-relevant but not a pure read-only discriminator.

## Future Discriminator Decision

The selected queued follow-up is
phase12-rp1-ethernet-bcm54213pe-readonly-preflight-report-core-20260616.

That task may only build the local/static report-core boundary for:

- MII_CTRL1000 0x09 on PHY1.
- MII_STAT1000 0x0a on PHY1.

The selected report core must keep the accepted BMCR/BMSR/ANAR/ANLPAR/
MACB_NSR phy-not-ready status as context, not as new proof. A later hardware
proof would still need a separate explicit task with hardwareTestLock,
same-power-cycle identity evidence, a paired no-MDIO/no-Ethernet control,
TFTP/serial evidence, restore rules, and inconclusive-run triage. This task
does not authorize that proof.

## Reconciliation

The accepted post-physical status remains phy-not-ready: BMCR 0x1000, BMSR
0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000, MACB_NSR 0x00000006, BMSR link
false, autoneg complete false, ANLPAR nonzero false, and MACB_NSR_LINK false.
Same-shaped status retries are rejected.

The accepted physical Ethernet link precondition remains accepted. The GPIO32
blocker remains decisive for reset recovery: prior accepted GPIO32 evidence
does not prove ETH_RST_N ownership or permit treating persistent event bits as
harmless.

MACB/phylink remains a boundary. No packet I/O, networking, sockets, SSH,
Phase 12.2, or phase transition is accepted.

## Evidence

- Task classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract/evidence-map.json.
- Source excerpt checksums:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract/source/sha256sums.txt.

## Validation

- static/source/task evidence inspection.
- jq empty on task-owned JSON evidence.
- sha256sum -c on retained source excerpts.
- git diff --check.
- mdbook build.
- git diff --cached --check before commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-readonly-preflight-report-core-20260616 on the
next worker wake if dependencies remain satisfied. Do not start hardware,
GPIO32 event clear/reset recovery, BMCR write, Broadcom shadow/MMD/aux access,
PHY/MAC configuration, packet I/O, networking, SSH, Phase 12.2, or phase
transition.
