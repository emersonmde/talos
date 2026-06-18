# Phase 12.1 RP1 Ethernet BCM54213PE Master-Mode Autoneg Source Contract Core

Task id: phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-source-contract-core-20260618

Status: accepted

Classification:
bcm54213pe-master-mode-autoneg-source-contract-core-local-static

Evidence level: static/source/task evidence inspection, focused Rust unit tests,
full Rust unit tests through QEMU, JSON evidence validation, docs build, and diff
checks. No Pi 5 hardware run, hardwareTestLock acquisition, boot archive
publication, lab mutation, power-cycle, TFTP/serial capture, runtime Ethernet
behavior, packet I/O, networking, SSH, Phase 12.2, or phase transition was
performed.

## Goal

Implement the local/static source contract and deterministic candidate/control
surface for the sequenced BCM54213PE MII_CTRL1000 master-mode plus BMCR autoneg
restart discriminator selected after the accepted MII_CTRL1000 write/readback
frontier.

## Scope Performed

- Added a BCM54213PE master-mode-autoneg source-contract core to
  src/rp1_ethernet.rs using the existing Phase 12 static evidence pattern.
- Bound the contract to the accepted closeout commit
  e01744a814987c725ebd6158de5fa570c229403a and accepted Pi 5 proof commit
  7f029dc3fbb38032e396cc01b438ab999ace8ecd.
- Modeled the future candidate as PHY1 MII_CTRL1000 read/modify/write/readback
  first, with accepted pre-read 0x0200, expected write/readback 0x1a00, then one
  BMCR autoneg enable plus restart write frame 0x50821200, then bounded
  BMCR/BMSR/ANAR/ANLPAR/MII_CTRL1000/MII_STAT1000/passive-MACB_NSR sampling.
- Modeled the paired no-MDIO/no-Ethernet control with no MDIO, MAN, MACB,
  GPIO32/PHY, interrupt, packet, networking, or SSH target construction.
- Added focused validators/tests rejecting same-shaped status-only polling, bare
  BMCR restart retry, marker/capture-only retry, GPIO32 reset, interrupt,
  APD/EEE/lifecycle, MAC/phylink, packet I/O, networking, SSH, Phase 12.2, and
  phase-transition overclaims.
- Updated Phase 12 documentation and the roadmap because the selected next
  boundary changed to the serialized Pi 5 proof task.

## Findings

- fixed: the selected follow-up is now represented by deterministic local/static
  source-contract evidence instead of a planning note only.
- fixed: the future hardware proof boundary is qualitatively distinct from
  rejected same-shaped status polling and bare BMCR restart retries because it
  requires the accepted MII_CTRL1000 write/readback prerequisite before restart
  and convergence sampling.
- fixed: the candidate and control report surfaces now encode exact selected
  operations, terminal classifications, and forbidden claim rejection.
- deferred: Pi 5 hardware evidence belongs to
  phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof-20260618.
- rejected: link-ready and autoneg-complete are not accepted by this local/static
  source contract; they require direct later hardware observation.
- removed: no source, helper, task, evidence, or doc files were removed.
- not-an-issue: no boot scenario runtime, archive wrapper, or lab action was
  added in this source/static task.

## Accepted Contract

Selected discriminator:
bcm54213pe-phy1-mii-ctrl1000-master-mode-plus-bmcr-autoneg-restart.

Candidate surface for a later hardware proof:

- emit selected-tree/TFTP/serial/final-identity/restore evidence;
- pre-read PHY1 MII_CTRL1000 0x09;
- write/readback PHY1 MII_CTRL1000 with accepted pre-value 0x0200, mask 0x1800,
  expected write value 0x1a00, and expected write frame 0x50a61a00;
- perform exactly one PHY1 BMCR autoneg enable plus restart write frame
  0x50821200 only after MII_CTRL1000 readback matches;
- run bounded convergence sampling over BMCR, double-sampled BMSR, ANAR, ANLPAR,
  MII_CTRL1000, MII_STAT1000, and passive MACB_NSR_LINK context;
- accept link-ready/autoneg-complete only if the later Pi 5 proof directly
  observes them.

Paired control surface:

- no MDIO target construction;
- no MAN frame construction;
- no MACB target construction;
- no GPIO32/ETH_RST_N/PHY target construction;
- no interrupt, APD/EEE/lifecycle, packet, networking, SSH, Phase 12.2, or
  phase-transition claim.

Future hardware terminal classifications:

- bcm54213pe-master-mode-autoneg-link-ready;
- bcm54213pe-master-mode-autoneg-timeout-link-not-ready;
- bcm54213pe-master-mode-autoneg-precondition-blocker;
- bcm54213pe-master-mode-autoneg-master-mode-readback-mismatch;
- bcm54213pe-master-mode-autoneg-bmcr-restart-blocker;
- bcm54213pe-master-mode-autoneg-capture-blocker;
- no-mdio-no-ethernet-bcm54213pe-master-mode-autoneg-control.

## Rejected Claims And Retained Risks

Rejected claims:

- link-ready from local/static evidence;
- autoneg-complete from local/static evidence;
- same-shaped status-only polling retry;
- bare BMCR autoneg restart retry without accepted MII_CTRL1000 write/readback;
- marker/capture-only retry;
- GPIO32/ETH_RST_N reset ownership;
- interrupt ownership;
- APD/EEE/lifecycle ownership;
- MAC/phylink ownership;
- packet I/O;
- networking;
- sockets;
- SSH;
- Phase 12.2;
- phase transition.

Retained risks:

- This source contract only encodes a future sequenced PHY configuration and
  convergence-sampling boundary; it does not perform or prove hardware behavior.
- The later Pi 5 proof must serialize hardwareTestLock and retain selected-tree,
  TFTP, serial, final identity, restore, paired control, and post-hardware
  review evidence.
- GPIO32/ETH_RST_N reset ownership, interrupt ownership, APD/EEE/lifecycle,
  MAC/phylink, packet I/O, networking, sockets, SSH, and Phase 12.2 remain
  unaccepted.

## Evidence

- Task classification:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-source-contract-core/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-source-contract-core/evidence-map.json.
- Local/static validator output:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-source-contract-core/validator-output.txt.
- Accepted MII_CTRL1000 closeout:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-closeout.md.
- Accepted MII_CTRL1000 Pi 5 proof:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof.md.

## Validation

- static/source/task evidence inspection.
- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test --quiet
  rp1_ethernet_bcm54213pe_master_mode_autoneg_source_contract.
- cargo -Zjson-target-spec test --quiet with documented QEMU PATH.
- jq empty on task-owned JSON evidence.
- git diff --check.
- mdbook build.
- git diff --cached --check before commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof-20260618 on the
next worker wake if dependencies remain satisfied, hardwareTestLock remains
unlocked/restored, supervisorIntervention is inactive, and projects/talos is
clean. Do not start packet I/O, networking, SSH, Phase 12.2, or
phase-transition work from this source-contract core.
