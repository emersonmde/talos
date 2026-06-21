# Phase 12.1 RP1 Ethernet BCM54213PE Lifecycle Ownership Pi 5 Proof

Task id: phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-pi5-proof-20260621

Status: accepted

Classification: bcm54213pe-lifecycle-powerdown-exit-no-change-link-not-ready

Evidence level: static archive/image review, lab-controller API selected-tree
and restore evidence, same-power-cycle TFTP evidence, serial hardware
boot/output, serial freshness guard v1 replay, identity join/run-unique replay,
boot staging identity replay, focused Rust tests, task-owned JSON validation,
docs build, and diff checks.

## Goal

Run the serialized Pi 5 proof for the accepted BCM54213PE lifecycle ownership
powerdown-exit sequence and decide only whether the selected BMCR_PDOWN exit
gate produces a link-ready terminal, a link-not-ready terminal, or a named
blocker.

## Scope Performed

- Acquired hardwareTestLock before lab archive publication and released it only
  after restore proof.
- Added task-specific image, boot-tree, archive, and review wrappers for the
  lifecycle powerdown-exit candidate/control pair.
- Built and reviewed run-unique control and candidate archives.
- Ran the no-MDIO/no-Ethernet control, then the candidate, under selected-tree
  boot identity.
- Retained selected-tree identity, same-power-cycle TFTP byte evidence, serial
  nonce freshness, final pre-restore identity, restore proof, and final lab
  status.

## Findings

- fixed: control archive retained SHA-256
  030f4b3e401c16299067145b635fa518aecf8cb363914c86c8a58e1d2edd4f93, kernel
  SHA-256 9cb11ffc97d5804e87d49d68463e7e85ded83acd1b1702efee815632d48fe303,
  a 50,344-byte kernel_2712.img, selected tree
  470ddbba0db7d535d59ac0e53b7a3a88a1be1fc33dfaea5e8e70f2c0ce646b46, two
  matching da591740/kernel_2712.img TFTP serves, 19 fresh serial nonce
  occurrences, and restore to baseline.
- fixed: control proved the no-MDIO/no-Ethernet shape with classification
  no-mdio-no-ethernet-bcm54213pe-lifecycle-ownership-control and no MDIO, MAN,
  MACB, GPIO32/PHY, interrupt, packet, networking, sockets, SSH, or phase
  target construction.
- fixed: candidate archive retained SHA-256
  dec1c5645eeb7cc55659c01db828341a2811e19b2ec986473e528d8dbe32190f, kernel
  SHA-256 988d4b6157bc1cbe9b7e5a9f67bc38868f80776364bfa00edb7e8146f1b89241,
  a 52,584-byte kernel_2712.img, selected tree
  ea74cb68f07eb69fadb5115c0e1cd65b770e3f2de6b07be56a1df14fc9583784, two
  matching da591740/kernel_2712.img TFTP serves, 17 fresh serial nonce
  occurrences, and restore to baseline.
- fixed: candidate performed the selected BMCR_PDOWN gate without widening the
  hardware surface. It observed ncr-before/ncr-after 0x10, pre-BMCR 0x1000,
  bmcr-pdown-pre-set=false, bmcr-write-performed=false, post-BMCR 0x1000,
  double-sampled BMSR 0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000,
  MII_CTRL1000 0x0200, MII_STAT1000 0x0000, passive MACB_NSR 0x00000006, and
  passive MACB_NSR_LINK=false.
- fixed: candidate classified
  bcm54213pe-lifecycle-powerdown-exit-no-change-link-not-ready because BMCR
  PDOWN was already clear, no clear write was issued, BMSR link status and
  autoneg complete were false, and passive MACB_NSR_LINK was false.
- fixed: serial freshness guard v1, identity join/run-unique, boot staging
  identity, and TFTP stability replay accepted both retained runs.
- rejected: the candidate does not prove link-ready, autoneg-complete,
  packet-readiness, live packet I/O, ping/hardware reachability, Ethernet
  driver readiness, networking, sockets, SSH, Phase 12.2, or a phase
  transition.
- rejected: APD, EEE, IDDQ/TOP_MISC, soft reset, interrupt ownership,
  config_init replay, GPIO32 reset action, MAC/phylink configuration, link
  forcing, and broad PHY/MAC ownership remain unaccepted.
- not-an-issue: BMCR 0x1000 differs from the source-core accepted-context
  example 0x1200 but still has BMCR_PDOWN clear; the selected fail-closed
  semantics therefore required no write and produced the allowed no-change
  link-not-ready terminal.
- removed: no cleanup reasserted BMCR_PDOWN; the lab boot state was restored
  from the pre-run snapshot instead.

## Control

The control replay is decisive for the no-MDIO/no-Ethernet paired shape. It
retained selected-tree identity through /boot/files, two matching
da591740/kernel_2712.img TFTP serves, fresh serial nonce occurrences, no MDIO
or MACB target construction, and post-run restore to the baseline boot tree.

## Candidate

The candidate replay is decisive for the selected BMCR_PDOWN gate. It
constructed only the accepted PHY1/BCM54213PE lifecycle surface, observed
BMCR_PDOWN already clear, skipped the conditional clear write, sampled the
post-status registers, and stopped at the allowed no-change link-not-ready
terminal. This is not link readiness and not packet readiness.

## Evidence

- Classification:
  tasks/evidence/2026-06-21-phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-21-phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-pi5-proof/evidence-map.json.
- Hardware run summary:
  tasks/evidence/2026-06-21-phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-pi5-proof/hardware-run-summary.json.
- Control run:
  tasks/evidence/2026-06-21-phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-pi5-proof/control-run/.
- Candidate run:
  tasks/evidence/2026-06-21-phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-pi5-proof/candidate-run/.
- Restore proof:
  tasks/evidence/2026-06-21-phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-pi5-proof/final-post-restore-boot-files.json.

## Validation

- static archive/image review: task-specific control/candidate review wrappers
  passed and rejected forbidden strings.
- lab-controller API: POST /boot/snapshot, PUT /boot/archive, power-cycle via
  capture helper, GET /boot/files, POST /boot/restore, and final GET
  /boot/files evidence retained selected-tree identity, final identity, and
  restore proof.
- same-power-cycle TFTP evidence: control retained two matching 50,344-byte
  da591740/kernel_2712.img serves; candidate retained two matching 52,584-byte
  serves.
- serial hardware boot/output: control retained the no-MDIO/no-Ethernet marker;
  candidate retained the lifecycle marker and the no-change link-not-ready
  runtime facts.
- serial freshness guard v1 replay: both bundles passed.
- identity join/run-unique replay: both bundles passed.
- boot staging identity replay: both bundles passed.
- cargo fmt --all -- --check: pass.
- focused Rust tests: cargo -Zjson-target-spec test --quiet
  rp1_ethernet_bcm54213pe_lifecycle_ownership_source_core passed with the
  configured QEMU path.
- sh -n on touched shell scripts: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass with the pre-existing large search-index warning.
- git diff --cached --check: pass before commit.

## Acceptance Check

- Hardware evidence is serialized under hardwareTestLock and includes post-run
  baseline restore proof: satisfied.
- Candidate/control selected-tree identity, TFTP byte serves, serial
  freshness, and final identity are decisive: satisfied.
- Terminal classification is
  bcm54213pe-lifecycle-powerdown-exit-no-change-link-not-ready, one of the
  allowed terminal classifications: satisfied.
- Rejected packet I/O, networking, SSH, Phase 12.2, and phase-transition claims
  remain explicit: satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-lifecycle-ownership-closeout-20260621 on the
next worker wake if dependencies remain satisfied and hardwareTestLock remains
unlocked/restored. Do not start packet I/O, networking, SSH, Phase 12.2, or a
phase transition from this proof.
