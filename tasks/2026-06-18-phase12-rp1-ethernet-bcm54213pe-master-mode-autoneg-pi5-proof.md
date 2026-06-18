# Phase 12.1 RP1 Ethernet BCM54213PE Master-Mode Autoneg Pi 5 Proof

Task id: phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof-20260618

Status: accepted

Classification: bcm54213pe-master-mode-autoneg-timeout-link-not-ready

Evidence level: static archive/image review, lab-controller API selected-tree
and restore evidence, same-power-cycle TFTP evidence, serial hardware
boot/output, serial freshness guard v1, JSON evidence validation, docs build,
and diff checks.

## Goal

Run the serialized Pi 5 hardware proof for the accepted sequenced BCM54213PE
MII_CTRL1000 master-mode plus BMCR autoneg-restart source contract.

## Scope Performed

- Acquired hardwareTestLock before lab archive publication and retained it
  through paired control/candidate restore evidence.
- Built and statically reviewed the paired no-MDIO/no-Ethernet control archive
  and the MII_CTRL1000 master-mode plus BMCR autoneg-restart candidate archive.
- Ran the control before the candidate, retaining selected-tree identity,
  same-power-cycle TFTP byte evidence, fresh serial nonce evidence, final
  pre-restore identity, and restore proof for both runs.
- Restored the pre-run snapshot
  bcm54213pe-master-mode-autoneg-pre-20260618T1638Z and retained final restored
  boot-tree evidence.

## Findings

- fixed: the control retained the no-MDIO/no-Ethernet shape with classification
  no-mdio-no-ethernet-bcm54213pe-master-mode-autoneg-control, matching
  selected-tree/TFTP bytes, serial nonce freshness, final pre-restore identity,
  and restore proof.
- fixed: the candidate retained matching selected-tree identity
  8b9eddafc3f0210f4be8c2c0f649286e0f92a17f65e0611952b618c89af03b7d,
  two matching 54072-byte da591740/kernel_2712.img TFTP serves, fresh serial
  nonce evidence, final pre-restore identity, and restore proof.
- fixed: the candidate repeated the accepted MII_CTRL1000 sequence with pre-read
  0x0200, write value 0x1a00, readback 0x1a00, and completion flags true.
- fixed: the candidate issued exactly one BMCR autoneg enable/restart write
  0x1200 only after MII_CTRL1000 readback matched.
- fixed: bounded convergence sampling completed eight samples and ended with
  BMCR 0x1000, BMSR 0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000,
  MII_CTRL1000 0x1a00, MII_STAT1000 0x0000, passive MACB_NSR 0x00000006,
  BMSR link false, BMSR autoneg-complete false, and MACB_NSR_LINK false.
- rejected: link-ready, autoneg-complete, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition remain unaccepted.
- rejected: GPIO32/ETH_RST_N reset ownership, interrupts, APD/EEE/lifecycle,
  MAC/phylink configuration, and more same-shaped status/autoneg polling remain
  outside this proof.
- removed: no source, helper, task, evidence, or doc files were removed.

## Evidence

- Classification:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof/evidence-map.json.
- Hardware summary:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof/hardware-run-summary.json.
- Control archive review:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof/control-archive-review.txt.
- Candidate archive review:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof/candidate-archive-review.txt.
- Control run:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof/control-run/.
- Candidate run:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof/candidate-run/.
- Final restore proof:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-pi5-proof/final-post-restore-boot-files.json.

## Validation

- static archive/image review: control and candidate review wrappers passed.
- lab-controller API: POST /boot/snapshot, PUT /boot/archive, POST
  /power/cycle, GET /boot/files, POST /boot/restore, and final GET /boot/files
  evidence retained selected-tree identity, final identity, and restore proof.
- same-power-cycle TFTP evidence: control and candidate retained matching
  da591740/kernel_2712.img byte serves.
- serial hardware boot/output: control retained the no-MDIO/no-Ethernet marker;
  candidate retained the MII_CTRL1000 plus BMCR autoneg-restart marker and
  runtime facts.
- serial freshness guard v1: control and candidate passed.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Acceptance Check

- HardwareTestLock was serialized, restored, and released with task-owned
  evidence: satisfied.
- Candidate/control selected-tree identity, same-power-cycle TFTP byte serves,
  serial freshness, final pre-restore identity, and restore proof are retained:
  satisfied.
- Terminal classification is one of the accepted source-core classifications:
  bcm54213pe-master-mode-autoneg-timeout-link-not-ready.
- Link-ready/autoneg-complete were not accepted because BMSR link,
  BMSR autoneg-complete, and MACB_NSR_LINK were all false at terminal sample.
- The result does not claim packet I/O, networking, SSH, Phase 12.2, or phase
  transition: satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-master-mode-autoneg-closeout-20260618 on a
future worker wake if dependencies remain satisfied, hardwareTestLock remains
unlocked/restored, supervisorIntervention remains inactive, and projects/talos
is clean. Do not start packet I/O, networking, SSH, Phase 12.2, or a phase
transition from this proof.
