# Phase 12.1 RP1 Ethernet BCM54213PE Selected Link-Not-Ready Source Contract Core

Task id: phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-source-contract-core-20260618

Status: accepted

Classification:
bcm54213pe-mii-ctrl1000-master-mode-source-contract-core-local-static

Evidence level: static/source/task evidence inspection, focused Rust unit
tests, full Rust unit tests through QEMU, JSON evidence validation, docs build,
and diff checks. No Pi 5 hardware run, hardwareTestLock acquisition, boot archive
publication, lab mutation, power-cycle, TFTP/serial capture, runtime Ethernet
behavior, packet I/O, networking, SSH, Phase 12.2, or phase transition was
performed.

## Goal

Implement the local/static source contract and deterministic candidate/control
surface for the BCM54213PE MII_CTRL1000 master-mode gate selected by
phase12-rp1-ethernet-bcm54213pe-link-not-ready-discriminator-selection-20260618.

## Scope Performed

- Added a BCM54213PE MII_CTRL1000 master-mode source-contract core to
  src/rp1_ethernet.rs using the existing Phase 12 static evidence pattern.
- Encoded the selected discriminator
  bcm54213pe-phy1-mii-ctrl1000-master-mode-gate-source-contract.
- Bound the contract to the accepted Linux source owners:
  bcm54xx_config_init -> bcm54213pe_config_init -> bcm54210e_config_init,
  PHY_BRCM_EN_MASTER_MODE, CTL1000_AS_MASTER, and CTL1000_ENABLE_MASTER.
- Encoded the deterministic future candidate surface: PHY1 MII_CTRL1000 0x09
  pre-read, MAN read frame 0x60a60000, one modeled read/modify/write intent
  using write prefix 0x50a60000, write mask 0x1800, accepted pre-value 0x0200,
  expected write value 0x1a00, expected write frame 0x50a61a00, and required
  post-write readback/restore-or-rollback expectations.
- Encoded the paired no-MDIO/no-Ethernet control surface and validator
  rejection paths for runtime action, same-shaped status/restart/poll retries,
  BMCR restart, RGMII delay retry, GPIO32 reset, interrupts,
  APD/EEE/lifecycle, MAC/phylink, packet I/O, networking, SSH, Phase 12.2, and
  phase transition.
- Added focused tests for accepted candidate/control shape and drift rejection.

## Findings

- fixed: the selected discriminator is now represented by deterministic local
  source-contract evidence instead of remaining only a selection record.
- fixed: the future hardware proof boundary has explicit candidate/control
  report surfaces and terminal classifications.
- fixed: the validator rejects unselected candidate families and forbidden
  overclaims from the accepted selection task.
- deferred: Pi 5 hardware evidence belongs to
  phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof-20260618.
- removed: no source, helper, task, evidence, or doc files were removed.
- not-an-issue: no runtime boot scenario was added in this source-contract
  task; runtime code and image/archive review belong to the follow-up Pi 5 proof
  task.

## Accepted Contract

Selected discriminator:
bcm54213pe-phy1-mii-ctrl1000-master-mode-gate-source-contract.

Source owners:

- linux-rpi-6.12 Broadcom PHY driver
  bcm54xx_config_init -> bcm54213pe_config_init -> bcm54210e_config_init;
- PHY_BRCM_EN_MASTER_MODE gate;
- PHY1 MII_CTRL1000 CTL1000_AS_MASTER and CTL1000_ENABLE_MASTER write path.

Candidate surface for a later hardware proof:

- emit selected-tree/TFTP/serial/final-identity/restore evidence;
- pre-read PHY1 MII_CTRL1000 0x09;
- model exactly one PHY1 MII_CTRL1000 read/modify/write setting
  CTL1000_AS_MASTER 0x0800 and CTL1000_ENABLE_MASTER 0x1000;
- preserve accepted pre-read value 0x0200 and expected write value 0x1a00 in
  the report contract;
- require post-write readback of the selected mask;
- record restore or rollback expectations;
- withhold link-ready acceptance until a later proof observes it directly.

Paired control surface:

- no MDIO target construction;
- no MAN frame construction;
- no MACB target construction;
- no GPIO32/ETH_RST_N/PHY target construction;
- no interrupt, APD/EEE/lifecycle, packet, networking, SSH, Phase 12.2, or
  phase-transition claim.

Future hardware terminal classifications:

- mii-ctrl1000-master-mode-write-readback-visible;
- mii-ctrl1000-master-mode-precondition-blocker;
- mii-ctrl1000-master-mode-readback-mismatch;
- mii-ctrl1000-master-mode-capture-blocker;
- no-mdio-no-ethernet-bcm54213pe-mii-ctrl1000-master-mode-control.

## Rejected Claims And Retained Risks

Rejected claims:

- link-ready;
- autoneg-complete;
- Ethernet driver readiness;
- same-shaped status/restart/poll/capture retry;
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

- This source contract encodes future MII_CTRL1000 master-mode write intent only;
  it does not perform or prove hardware behavior.
- The later Pi 5 proof must serialize hardwareTestLock and retain selected-tree,
  TFTP, serial, final identity, restore, paired control, and post-hardware
  review evidence.
- GPIO32/ETH_RST_N reset ownership, interrupt ownership, APD/EEE/lifecycle,
  MAC/phylink, packet I/O, networking, sockets, SSH, and Phase 12.2 remain
  unaccepted.

## Evidence

- Task classification:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-source-contract-core/classification.json.
- Task evidence map:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-source-contract-core/evidence-map.json.
- Local/static validator output:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-source-contract-core/validator-output.txt.
- Accepted selection:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-link-not-ready-discriminator-selection.md.
- Retained source evidence:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-bcm54213pe-config-init-source-contract/source/linux-rpi-6.12-broadcom-bcm54213pe-config-contract-excerpt.txt.

## Validation

- static/source/task evidence inspection.
- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test --quiet
  rp1_ethernet_bcm54213pe_master_mode_source_contract.
- cargo -Zjson-target-spec test --quiet with documented QEMU PATH.
- jq empty on task-owned JSON evidence.
- git diff --check.
- mdbook build.
- git diff --cached --check before commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof-20260618 on
the next worker wake if dependencies remain satisfied, hardwareTestLock remains
unlocked/restored, supervisorIntervention is inactive, and projects/talos is
clean. Do not start packet I/O, networking, SSH, Phase 12.2, or
phase-transition work from this source-contract core.
