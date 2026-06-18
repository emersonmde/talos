# Phase 12.1 RP1 Ethernet BCM54213PE Master-Mode Autoneg Closeout

Task id:
phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-closeout-20260618

Status: accepted

Classification:
bcm54213pe-master-mode-autoneg-frontier-paused-link-not-ready-planning-required

Evidence level: static/task evidence review, task-owned JSON evidence, docs
build, and diff checks. No runtime code change, Pi 5 hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, power-cycle,
TFTP/serial capture, GPIO32/PHY reset action, interrupt write, MACB
configuration write, packet I/O, networking, sockets, SSH, Phase 12.2, or
phase transition was performed by this closeout.

## Goal

Reconcile the accepted BCM54213PE MII_CTRL1000 master-mode plus BMCR autoneg
restart source contract and Pi 5 proof with the Phase 12.1 frontier, preserve
the accepted evidence, and determine whether link-ready packet-readiness is
mechanically unblocked.

## Scope Performed

- Reviewed the accepted master-mode-autoneg source-contract core and serialized
  Pi 5 proof evidence.
- Recorded the terminal classification and findings with dispositions.
- Preserved accepted claims for MII_CTRL1000 master-mode write/readback, exactly
  one BMCR autoneg enable/restart write after readback, and bounded convergence
  sampling.
- Preserved rejected claims for link-ready, autoneg-complete, GPIO32/PHY reset,
  interrupts, APD/EEE/lifecycle, MAC/phylink, packet I/O, networking, sockets,
  SSH, Phase 12.2, phase transition, and same-shaped status/autoneg polling.
- Updated Phase 12 documentation and the roadmap because the visible frontier
  changed from a selected hardware proof to a paused hardware-observed
  timeout/link-not-ready boundary.
- Selected no next worker task. planningNeeded=true is required because the
  accepted evidence does not mechanically unblock packet-readiness or define the
  next distinct feature-led Ethernet step.

## Findings

- fixed: the accepted hardware frontier now includes a serialized Pi 5
  candidate/control proof for the selected BCM54213PE MII_CTRL1000 master-mode
  plus BMCR autoneg-restart discriminator.
- fixed: the paired control retained the no-MDIO/no-Ethernet classification
  no-mdio-no-ethernet-bcm54213pe-master-mode-autoneg-control with selected-tree,
  TFTP, serial nonce, final pre-restore identity, and restore evidence.
- fixed: the decisive candidate retained selected-tree identity
  8b9eddafc3f0210f4be8c2c0f649286e0f92a17f65e0611952b618c89af03b7d,
  same-power-cycle 54072-byte TFTP serves, fresh serial nonce evidence, final
  pre-restore identity, and restore proof.
- fixed: the decisive candidate reported PHY1 MII_CTRL1000 pre-read 0x0200,
  write value 0x1a00, and readback 0x1a00, with read/write/readback completion
  fields true.
- fixed: the decisive candidate issued exactly one BMCR autoneg enable/restart
  write 0x1200 after the MII_CTRL1000 readback matched.
- fixed: bounded convergence sampling produced terminal BMCR 0x1000, BMSR
  0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000, MII_CTRL1000 0x1a00,
  MII_STAT1000 0x0000, passive MACB_NSR 0x00000006, BMSR link false,
  BMSR autoneg-complete false, and MACB_NSR_LINK false.
- removed: no source, helper, docs, task, or evidence files were removed.
- deferred: link-ready and autoneg-complete remain unaccepted and require a
  future supervisor-planned discriminator or feature boundary.
- deferred: GPIO32/ETH_RST_N reset ownership, interrupts,
  APD/EEE/lifecycle, MAC/phylink, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition remain outside the accepted frontier.
- rejected: the link-ready packet-readiness checkpoint is not mechanically
  unblocked because direct BMSR/autoneg and MACB_NSR link evidence stayed false.
- rejected: another same-shaped timeout/status/autoneg poll/capture retry is not
  progress without a new accepted discriminator.
- not-an-issue: this closeout did not acquire hardwareTestLock or run
  inconclusive-run triage because it performed only static reconciliation over
  already accepted proof evidence.

## Reconciliation

Input frontier:
bcm54213pe-master-mode-autoneg-timeout-link-not-ready.

Accepted closeout frontier:
bcm54213pe-master-mode-autoneg-frontier-paused-link-not-ready-planning-required.

Selected discriminator:
bcm54213pe-phy1-mii-ctrl1000-master-mode-plus-bmcr-autoneg-restart.

Selected next task: null.

Planning needed: true.

Planning reason: The selected BCM54213PE proof establishes that MII_CTRL1000
master-mode write/readback and one BMCR autoneg restart executed, but direct
terminal evidence still shows BMSR link false, BMSR autoneg-complete false, and
MACB_NSR_LINK false. The proof does not accept packet-readiness, networking,
SSH, Phase 12.2, or any exact next discriminator. Supervisor planning is
required before further Phase 12.1 Ethernet work.

## Evidence

- Source-contract core:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-source-contract-core.md.
- Pi 5 proof:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof.md.
- Pi 5 proof classification:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof/classification.json.
- Pi 5 proof evidence map:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof/evidence-map.json.
- Pi 5 proof hardware summary:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof/hardware-run-summary.json.
- Candidate capture summary:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof/candidate-run/capture-invariant-summary.json.
- Control capture summary:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof/control-run/capture-invariant-summary.json.
- Closeout classification:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-closeout/evidence-map.json.

## Acceptance Check

- Closeout records the terminal classification and findings with dispositions:
  satisfied.
- Accepted and rejected claims are explicit, especially link-ready,
  autoneg-complete, packet I/O, networking, SSH, Phase 12.2, and phase
  transition: satisfied.
- Link-ready/autoneg-complete are not accepted, so planningNeeded=true and no
  packet/networking/SSH task is mechanically unblocked: satisfied.
- No runtime implementation, hardware run, lab mutation, boot publication,
  packet I/O, networking, SSH, Phase 12.2, or phase transition was performed:
  satisfied.

## Validation

- static/task evidence review: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Supervisor planning is required before any future Phase 12.1 Ethernet task. Do
not promote the link-ready packet-readiness checkpoint, packet I/O, networking,
sockets, SSH, Phase 12.2, phase transition, GPIO32/PHY reset action,
interrupt/APD/EEE/lifecycle work, MAC/phylink work, or same-shaped
timeout/status/autoneg poll/capture retries from this closeout.
