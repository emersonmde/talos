# Phase 12 RP1 Ethernet BCM54213PE RGMII Delay Source Contract

Task id: phase12-rp1-ethernet-bcm54213pe-rgmii-delay-source-contract-20260616

Status: accepted

Classification:
bcm54213pe-rgmii-delay-source-contract-proof-core-selected

Evidence level: static/source/task evidence inspection, JSON evidence
validation, diff checks, and docs build. No Pi 5 hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, power-cycle,
TFTP/serial capture, GPIO32 event clear, PHY reset assertion/deassertion,
interrupt enable, packet I/O, networking, SSH, Phase 12.2, or phase transition
was performed.

## Goal

Convert the accepted BCM54213PE post-convergence timeout checkpoint into a
bounded source/static contract for the Linux-backed rgmii-id clock-delay path,
and select exactly one next implementation boundary if the write/readback
surface is mechanically specific enough.

## Scope Performed

- Inspected the accepted convergence timeout, accepted timeout source
  checkpoint, retained BCM54213PE config_init source, retained Broadcom AUX and
  shadow helper source, accepted GPIO32 reset blocker, and existing Clause 22
  MAN frame conventions.
- Reduced the RGMII delay surface to the Linux BCM54213PE path:
  bcm54xx_config_init -> bcm54213pe_config_init -> bcm54210e_config_init ->
  bcm54xx_config_clock_delay.
- Selected one next local/static proof-core boundary for candidate/control code
  that configures only BCM54213PE PHY1 rgmii-id RX and TX internal delays, reads
  the selected delay state back, then reuses the already accepted BMCR restart
  and bounded convergence-poll status discriminator.
- Rejected hardware acceptance, link-ready acceptance, packet/networking/SSH
  progress, GPIO32 reset, interrupts, MII_CTRL1000 master-mode writes, and broad
  PHY/MAC configuration from this static task.

## Findings

- fixed: the source-backed BCM54213PE config path reaches
  bcm54xx_config_clock_delay through bcm54213pe_config_init and
  bcm54210e_config_init.
- fixed: the retained Pi 5 devicetree context is phy-mode=rgmii-id, PHY address
  1, and PHY node ethernet-phy@1.
- fixed: for rgmii-id, Linux enables both the RXC-RXD skew bit in the Broadcom
  AUX misc shadow and the GTXCLK internal TX delay bit in shadow register 0x03.
- fixed: the next proof core can be bounded to two read-modify-write/readback
  delay surfaces plus the already accepted BMCR restart/convergence poll.
- deferred: actual Pi 5 hardware execution, boot publication, same-power-cycle
  TFTP/serial proof, and post-run restore stay deferred to the queued Pi 5 proof
  task after proof-core acceptance.
- deferred: MII_CTRL1000 master-mode writes stay deferred because Linux gates
  them on PHY_BRCM_EN_MASTER_MODE and the accepted Pi 5 context does not select
  that flag.
- deferred: interrupt ISR/IMR/ECR work remains deferred because ISR reads can
  acknowledge pending interrupts and mask/unmask writes require interrupt
  ownership.
- blocked: direct GPIO32/ETH_RST_N reset recovery remains blocked by accepted
  persistent-or-firmware-owned GPIO32 event-state evidence.
- rejected: APD, EEE, LED, WOL, suspend/resume, soft reset, MMD/expansion
  configuration, MACB configuration, DMA/descriptors, packet I/O, networking,
  sockets, SSH, Phase 12.2, and phase transition remain out of scope.
- not-an-issue: the accepted physical Ethernet link precondition remains
  accepted and was not re-asked.
- removed: no source, helper, task, or evidence files were removed.

## Selected Boundary

The selected next implementation boundary is:

phase12-rp1-ethernet-bcm54213pe-rgmii-delay-proof-core-20260616.

That proof core may add a candidate/control implementation and validators for
the following exact contract only.

Allowed target:

- PHY address: Clause 22 PHY1.
- PHY model context: Broadcom BCM54213PE, PHYSID1 0x600d, PHYSID2 0x84a2.
- Interface context: rgmii-id from the accepted RP1 Ethernet devicetree facts.
- MAC/MDIO path: existing RP1 MACB MAN Clause 22 command path only.

Allowed RX delay operation:

- Read selector path: write MII_BCM54XX_AUX_CTL 0x18 with
  MII_BCM54XX_AUXCTL_SHDWSEL_MASK | (MII_BCM54XX_AUXCTL_SHDWSEL_MISC << 12),
  which is 0x7007 for shadow 0x07, then read MII_BCM54XX_AUX_CTL 0x18.
- Write path: write MII_BCM54XX_AUX_CTL 0x18 with
  MII_BCM54XX_AUXCTL_SHDWSEL_MISC | pre_read |
  MII_BCM54XX_AUXCTL_MISC_WREN | MII_BCM54XX_AUXCTL_SHDWSEL_MISC_RGMII_SKEW_EN.
- Required masks: preserve non-target bits; require RGMII_SKEW_EN 0x0100 set
  in readback after a repeated selector write/read. WREN 0x8000 is write
  enabling context, not a required persistent readback bit.
- MAN frame prefixes: AUX_CTL read frame 0x60e20000; AUX_CTL write frame prefix
  0x50e20000.

Allowed TX delay operation:

- Read selector path: write MII_BCM54XX_SHD 0x1c with
  MII_BCM54XX_SHD_VAL(BCM54810_SHD_CLK_CTL), which is 0x0c00 for shadow 0x03,
  then read MII_BCM54XX_SHD 0x1c and keep MII_BCM54XX_SHD_DATA(raw).
- Write path: write MII_BCM54XX_SHD 0x1c with MII_BCM54XX_SHD_WRITE |
  MII_BCM54XX_SHD_VAL(BCM54810_SHD_CLK_CTL) |
  MII_BCM54XX_SHD_DATA(pre_read | BCM54810_SHD_CLK_CTL_GTXCLK_EN).
- Required masks: preserve shadow data bits outside GTXCLK_EN; require
  GTXCLK_EN 0x0200 set in readback.
- MAN frame prefixes: SHD read frame 0x60f20000; SHD write frame prefix
  0x50f20000.

Required operation order:

1. Emit a candidate/control contract marker and capture nonce.
2. Candidate only: perform the RX delay read-modify-write/readback sequence.
3. Candidate only: perform the TX delay read-modify-write/readback sequence.
4. If either delay readback mismatches, stop before BMCR restart and classify as
   rgmii-delay-readback-mismatch.
5. If both delay readbacks match, perform exactly one accepted BMCR autoneg
   restart write using the existing value 0x1200 and write frame 0x50821200.
6. Poll the already accepted bounded convergence status vector only:
   BMCR/BMSR-first/BMSR-second/ANAR/ANLPAR/MII_CTRL1000/MII_STAT1000/passive
   MACB_NSR_LINK.
7. Control must emit the same contract/rejection shape without constructing
   MDIO/MAN, Ethernet, MACB, GPIO32, interrupt, packet, networking, or SSH
   targets.

Allowed terminal classifications for later hardware proof are limited to:

- rgmii-delay-link-ready-frontier.
- rgmii-delay-timeout-link-not-ready.
- rgmii-delay-readback-mismatch.
- rgmii-delay-precondition-blocker.
- rgmii-delay-capture-blocker.
- no-mdio-no-ethernet-control.

## Rejected Claims And Retained Risks

This source/static contract does not prove link readiness. It only makes the
next implementation boundary mechanically specific enough to build and validate
candidate/control code before a separate serialized Pi 5 proof.

Rejected claims:

- hardware success;
- link readiness;
- packet I/O;
- networking;
- sockets;
- SSH;
- Phase 12.2;
- phase transition;
- GPIO32/PHY reset ownership;
- interrupt ownership or delivery;
- broad PHY/MAC configuration;
- MII_CTRL1000 master-mode writes;
- APD/EEE/LED/WOL/suspend-resume behavior.

Retained risks:

- RGMII delay write/readback may still leave link not ready.
- Broadcom AUX/shadow selector writes are real PHY writes and need paired
  control plus post-run restore evidence before any hardware acceptance.
- The accepted convergence timeout remains the only runtime fact until the
  queued Pi 5 proof accepts or blocks with fresh evidence.

## Evidence

- Task classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-source-contract/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-source-contract/evidence-map.json.
- Accepted timeout checkpoint:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-post-convergence-timeout-source-checkpoint.md.
- Retained source citations:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract/source/linux-rpi-6.12-broadcom-bcm54213pe-config-contract-excerpt.txt,
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract/source/linux-rpi-6.12-bcm-phy-lib-selector-read-contract-excerpt.txt,
  and Raspberry Pi Linux rpi-6.12.y include/linux/brcmphy.h and
  drivers/net/phy/bcm-phy-lib.c for the shadow helper constants.

## Validation

- static/source/task evidence inspection.
- jq empty on task-owned JSON evidence.
- git diff --check.
- mdbook build.
- git diff --cached --check before commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-rgmii-delay-proof-core-20260616 on the next
worker wake if dependencies remain satisfied. Do not run hardware or start
GPIO32 reset, interrupt, packet I/O, networking, SSH, Phase 12.2, or
phase-transition work from this source/static task.
