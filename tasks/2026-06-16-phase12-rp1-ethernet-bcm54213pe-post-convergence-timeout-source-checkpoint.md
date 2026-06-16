# Phase 12.1 RP1 Ethernet BCM54213PE Post-Convergence Timeout Source Checkpoint

Task id: phase12-rp1-ethernet-bcm54213pe-post-convergence-timeout-source-checkpoint-20260616

Status: accepted

Classification:
bcm54213pe-post-convergence-timeout-rgmii-delay-source-contract-selected

Evidence level: static/source/task evidence inspection, JSON evidence
validation, docs build, and diff checks. No runtime code change, Pi 5 hardware
run, boot archive publication, lab mutation, hardwareTestLock acquisition,
power-cycle, TFTP/serial capture, GPIO32 write, Broadcom selector/config
write, interrupt enable, PHY/MAC configuration, packet I/O, networking, SSH,
Phase 12.2, or phase transition was performed.

## Goal

Select the next feature-led boundary after the accepted BCM54213PE autoneg
convergence proof timed out with link-not-ready, using retained source and
evidence only.

## Scope Performed

- Reconciled the accepted convergence closeout timeout frontier and terminal
  register vector.
- Compared GPIO32 reset ownership, Broadcom BCM54213PE config_init paths,
  physical/partner-state evidence, and interrupt/status-only options.
- Selected exactly one next boundary for supervisor planning: a source/static
  BCM54213PE RGMII delay config source contract.
- Preserved rejected packet/networking/SSH/Phase 12.2 and link-ready claims.

## Findings

- fixed: the accepted convergence proof remains decisive evidence for the
  current frontier: exactly one corrected-target PHY1 BMCR autoneg restart
  write frame 0x50821200/value 0x1200 was followed by eight bounded poll
  samples, and the terminal sample remained BMCR 0x1000, BMSR 0x7949/0x7949,
  ANAR 0x01e1, ANLPAR 0x0000, MII_CTRL1000 0x0200, MII_STAT1000 0x0000,
  passive MACB_NSR 0x00000006, BMSR link false, BMSR autoneg-complete false,
  MACB_NSR_LINK false, and link-ready-terminal false.
- fixed: another BMCR/autoneg restart, another convergence poll, or another
  same-shaped BMCR/BMSR/ANAR/ANLPAR/MII_CTRL1000/MII_STAT1000/MACB_NSR sample
  would repeat the accepted timeout shape and is not selected.
- blocked: GPIO32 / ETH_RST_N reset recovery remains blocked by the accepted
  persistent-or-firmware-owned GPIO32 event-state frontier. GPIO32
  write/restore v2 stopped before GPIO/RIO/pad writes with event bits
  0x0ab00000, and the accepted GPIO32 CTRL SET IRQRESET clear attempt left
  event bits 0x08800000. This task does not treat those bits as harmless or
  authorize reset action.
- fixed: the retained BCM54213PE Linux config_init path is now the smallest
  source-backed feature path distinct from the accepted BMCR restart timeout:
  bcm54xx_config_init dispatches BCM54213PE to bcm54213pe_config_init, which
  reuses bcm54210e_config_init and unconditionally calls
  bcm54xx_config_clock_delay.
- selected: the next boundary for supervisor planning is a source/static
  BCM54213PE RGMII delay config source contract. It should decide the exact
  local contract for the rgmii-id delay path before any hardware or write
  proof: MII_BCM54XX_AUX_CTL shadow MII_BCM54XX_AUXCTL_SHDWSEL_MISC with
  MII_BCM54XX_AUXCTL_MISC_WREN and RGMII_SKEW_EN, plus BCM54810_SHD_CLK_CTL
  with BCM54810_SHD_CLK_CTL_GTXCLK_EN.
- deferred: MII_CTRL1000 master-mode configuration is source-backed but gated
  on PHY_BRCM_EN_MASTER_MODE. The accepted runtime value MII_CTRL1000 0x0200
  does not prove that dev_flags bit is selected, so it is not the next
  boundary.
- deferred: interrupt/status-only options are not selected. Linux ISR reads
  acknowledge pending interrupts, IMR/ECR writes configure masking, and the
  handler calls phy_trigger_machine(); this is interrupt ownership and does
  not directly unblock the failed feature path.
- deferred: physical/partner-state evidence remains context, not a new
  operator blocker. The physical-link precondition is accepted; ANLPAR 0x0000
  and MII_STAT1000 0x0000 show no accepted partner ability or receiver-ok
  evidence, but a partner diagnostic would be a diagnostic-only branch unless
  a future supervisor task names the feature it unblocks.
- deferred: APD, EEE, LED, WOL, suspend/resume, BMCR powerdown/soft reset,
  Broadcom expansion register writes, MACB configuration, DMA/descriptors, and
  packet I/O are broader than the selected RGMII delay source contract.
- rejected: this checkpoint does not accept link readiness, Ethernet driver
  readiness, GPIO32/PHY reset ownership, Broadcom selector/config writes,
  interrupt ownership, PHY/MAC configuration, packet I/O, networking, sockets,
  SSH, Phase 12.2, or a phase transition.
- removed: no source, helper, task, or evidence file was removed.
- not-an-issue: no hardwareTestLock was acquired because this checkpoint is
  source/docs/evidence only.

## Selected Boundary

Selected classification:
bcm54213pe-post-convergence-timeout-rgmii-delay-source-contract-selected.

Selected next boundary for supervisor planning:
phase12-rp1-ethernet-bcm54213pe-rgmii-delay-source-contract-20260616.

The selected follow-up should be local/static source-contract work only. Its
job is to decide whether Talos can model a narrow BCM54213PE rgmii-id delay
configuration contract with exact selector/register surfaces, write/restore
requirements, paired no-MDIO/no-Ethernet control, and fail-closed rejected
claims before any Pi 5 proof is considered.

Minimum allowed surface for that future contract:

~~~text
accepted input frontier:
  bcm54213pe-autoneg-convergence-frontier-closed-timeout-link-not-ready
feature path:
  bcm54xx_config_init -> bcm54213pe_config_init ->
  bcm54210e_config_init -> bcm54xx_config_clock_delay
source-backed board mode:
  phy-mode = rgmii-id
candidate selector/registers:
  MII_BCM54XX_AUX_CTL / MII_BCM54XX_AUXCTL_SHDWSEL_MISC
  MII_BCM54XX_AUXCTL_MISC_WREN
  MII_BCM54XX_AUXCTL_SHDWSEL_MISC_RGMII_SKEW_EN
  BCM54810_SHD_CLK_CTL
  BCM54810_SHD_CLK_CTL_GTXCLK_EN
required exclusions:
  GPIO32/ETH_RST_N action
  BMCR restart retry
  PHY_BRCM_EN_MASTER_MODE / MII_CTRL1000 master-mode write
  APD/EEE/LED/WOL/suspend/resume/expansion writes
  interrupt status/mask/control
  MACB configuration
  packet I/O/networking/SSH/Phase 12.2
~~~

No mechanically unblocked taskQueue item exists for this selected boundary in
this worker wake. Supervisor planning is required to add the explicit
dependency-gated follow-up before any implementation or hardware action.

## Reconciliation

The accepted BMCR restart plus convergence proof was the thinnest real feature
path after BCM54213PE register visibility, and it timed out without link
readiness. The next useful feature path should therefore be source-grounded
configuration that Linux applies for this exact PHY and board mode, not another
read-only status sample and not packet/networking work.

The GPIO32 reset path remains blocked by accepted event-state evidence, so
choosing reset now would weaken the process. Interrupt status is also the wrong
next step: ISR reads can acknowledge state and IMR/ECR writes configure
interrupt delivery, while no accepted link-ready or packet path exists for
interrupts to service. Physical/partner state is retained as a risk but does
not supersede the accepted no-reask physical-link precondition.

The BCM54213PE rgmii-id clock-delay branch is narrower than full config_init
and directly source-backed for this board mode. It is still a selector/write
surface, so this checkpoint selects only a future source/static contract, not a
runtime write or hardware proof.

## Rejected Claims And Retained Risks

Rejected claims:

- link readiness or Ethernet driver readiness;
- GPIO32 ownership, event clear, or PHY reset action;
- BMCR/autoneg retry or link forcing;
- immediate Broadcom selector/config write;
- MII_CTRL1000 master-mode write;
- APD, EEE, LED, WOL, suspend/resume, or expansion-register writes;
- interrupt ownership, acknowledgement, masking, delivery, or completion;
- MACB configuration;
- DMA/descriptors;
- packet I/O;
- networking;
- sockets;
- SSH;
- Phase 12.2;
- phase transition.

Retained risks:

- The PHY may still need RGMII delay configuration before link can converge,
  but the exact selector mechanics and restore semantics are not yet accepted.
- GPIO32 event-state may still independently block any reset-based recovery.
- The partner-state registers still report no partner ability or receiver-ok
  evidence after convergence timeout, but this checkpoint does not create a
  new Matthew/operator physical-link blocker.
- A later hardware proof, if planned, must still capture selected-tree
  identity, same-power-cycle TFTP, cursor-nonce serial freshness, final
  identity, restore proof, and paired control evidence.

## Evidence

- Accepted convergence closeout:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-closeout.md.
- Convergence closeout classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-closeout/classification.json.
- Convergence Pi 5 proof classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-pi5-proof/classification.json.
- Convergence Pi 5 capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-pi5-proof/capture-summary.json.
- Retained GPIO32 reset-recovery checkpoint:
  tasks/2026-06-15-phase12-rp1-ethernet-post-physical-gpio32-reset-recovery-source-checkpoint.md.
- Retained BCM54213PE config-init source contract:
  tasks/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract.md.
- Retained Broadcom PHY ID/driver source inventory:
  tasks/2026-06-15-phase12-rp1-ethernet-broadcom-phy-id-driver-source-inventory.md.
- Task classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-post-convergence-timeout-source-checkpoint/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-post-convergence-timeout-source-checkpoint/evidence-map.json.

## Acceptance Check

- Findings are recorded with disposition: satisfied.
- One next boundary is selected with explicit non-goals and dependency-gated
  follow-up requirements: satisfied by the selected RGMII delay source
  contract handoff for supervisor planning.
- Rejected packet/networking/SSH/Phase 12.2 claims remain explicit: satisfied.

## Validation

- static/source/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Set planningNeeded=true for supervisor planning of
phase12-rp1-ethernet-bcm54213pe-rgmii-delay-source-contract-20260616 or an
explicit pause. Do not promote the link-ready packet-readiness checkpoint, and
do not start hardware, GPIO32/reset, Broadcom selector/config writes,
interrupts, packet I/O, networking, SSH, Phase 12.2, or a phase transition
from this checkpoint.
