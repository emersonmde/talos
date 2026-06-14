# Phase 12 RP1 Ethernet PHY1 Link Readiness Source Contract

Task id: phase12-rp1-ethernet-phy1-link-readiness-source-contract-20260614

Status: accepted

Classification: rp1-ethernet-phy1-link-readiness-source-contract-accepted

Evidence level: static source/task evidence inspection and task-owned JSON
evidence. No runtime implementation, Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, PHY configuration write,
autonegotiation restart, link forcing, GPIO32/PHY reset action, packet I/O,
networking, SSH, Phase 12.2, or phase transition was performed.

## Goal

Name the exact next link-readiness discriminator that follows from the accepted
PHY1 status diagnostic frontier without authorizing implementation or hardware
proof.

## Scope Performed

- Consumed the accepted PHY1 status diagnostic closeout at commit
  c3986efdfce45f5c6c1d76a631225d10cc17b1bf.
- Reconciled the accepted one-sample PHY1 status vector against Linux MII,
  MACB, and generic PHY link-status source facts.
- Selected one read-only next discriminator:
  rp1-ethernet-phy1-bmsr-latch-low-double-sample-link-readiness.
- Defined allowed future candidate operations, paired no-MDIO/no-Ethernet
  control behavior, future proof classifications, forbidden operations, and
  evidence gates.
- Recorded findings with disposition.

## Non-Goals

No source implementation, no Rust code change, no hardware action, no boot
archive publication, no hardwareTestLock acquisition, no NCR write, no PHY
configuration write, no autonegotiation restart, no link forcing, no
GPIO32/PHY reset action, no MACB NSR_LINK proof, no DMA/descriptors, no packet
I/O, no networking, no sockets, no SSH, no Phase 12.2 work, and no phase
transition.

## Reconciled Inputs

- tasks/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof.md.
- tasks/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-closeout.md.
- tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-closeout/classification.json.
- tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-closeout/evidence-map.json.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-uapi-linux-mii.h.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-macb.h.
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c.
- tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-link-readiness-source-contract/source/linux-v6.12-phy_device-genphy_update_link-excerpt.txt.
- src/rp1_ethernet.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Source Facts

- The accepted status diagnostic saw corrected-target PHY1 BMCR 0x1000, BMSR
  0x7949, PHYSID1 0x600d, PHYSID2 0x84a2, ANAR 0x01e1, and ANLPAR 0x0000.
- Linux uapi mii.h defines MII_BMCR as register 0x00 and MII_BMSR as register
  0x01.
- Linux uapi mii.h defines BMCR_ANRESTART 0x0200, BMCR_ANENABLE 0x1000,
  BMSR_LSTATUS 0x0004, BMSR_ANEGCAPABLE 0x0008, and BMSR_ANEGCOMPLETE 0x0020.
- Linux v6.12 genphy_update_link reads BMCR, treats BMCR_ANRESTART as a
  not-ready condition, and handles BMSR link status as latched low by reading
  MII_BMSR again before assigning link and autonegotiation-complete state.
- The retained MACB source has an NSR_LINK bit and an SGMII fixed-state helper,
  but the retained Pi 5 source declares rp1_eth with phy-mode rgmii-id and
  phy-handle phy1. This contract therefore does not select MACB NSR_LINK as
  the next discriminator.
- The accepted corrected-target MAN frame construction already established
  PHY address 1 and MAN read frames for BMCR/BMSR: MII_BMCR 0x00 maps to
  0x60820000 and MII_BMSR 0x01 maps to 0x60860000.

## Selected Contract

Accepted contract id:
phase12-rp1-ethernet-phy1-bmsr-latch-low-double-sample-link-readiness-contract-v1.

~~~text
selected discriminator: rp1-ethernet-phy1-bmsr-latch-low-double-sample-link-readiness
controller: rp1_eth / raspberrypi,rp1-gem / cdns,macb
PHY source identity: phy1 / ethernet-phy@1 / reg 0x1
PHY address: 1
observed-window base: 0x1c00100000
NCR target: 0x1c00100000
NSR target: 0x1c00100008
MAN target: 0x1c00100034
NCR write permission: forbidden
PHY configuration write permission: forbidden
autonegotiation restart permission: forbidden
link forcing permission: forbidden
GPIO32/PHY reset permission: forbidden
selected reads:
  BMCR read: MII_BMCR 0x00, MAN 0x60820000
  BMSR first read: MII_BMSR 0x01, MAN 0x60860000
  BMSR second read: MII_BMSR 0x01, MAN 0x60860000
classification source:
  BMCR_ANRESTART false is required for link-readiness classification
  second BMSR.BMSR_LSTATUS and BMSR_ANEGCOMPLETE classify readiness
paired control: same reporting path, no MDIO target, no MAN frame, no volatile MDIO load/store
~~~

The future candidate may construct only the three selected read-only MAN
frames above. It must not write NCR, restart autonegotiation, force link,
change BMCR, touch GPIO32/ETH_RST_N, configure the PHY, read/write DMA or
descriptor state, or perform packet I/O.

## Operation Order For A Future Candidate

1. Print a candidate start marker, contract id, and accepted input frontier.
2. Read observed-window MACB_MID context at 0x1c001000fc as context only.
3. Read corrected NCR at 0x1c00100000 and require corrected NCR.MPE bit 4 to
   already be set.
4. If corrected NCR.MPE is clear or corrected NCR identity is ambiguous,
   perform no MAN write and classify a precondition blocker.
5. Poll corrected NSR.IDLE bit 2 at 0x1c00100008 until idle or bounded
   timeout.
6. Write corrected MAN at 0x1c00100034 with 0x60820000 for Clause 22 PHY1
   MII_BMCR and read MAN.DATA as bmcr.
7. If bmcr has BMCR_RESET, BMCR_LOOPBACK, or BMCR_ANRESTART set, classify a
   BMCR precondition blocker without BMSR link-readiness acceptance.
8. Poll corrected NSR.IDLE, then write MAN 0x60860000 for first MII_BMSR and
   read MAN.DATA as bmsr_first.
9. Poll corrected NSR.IDLE again, then write MAN 0x60860000 for second
   MII_BMSR and read MAN.DATA as bmsr_second.
10. Classify link-readiness only from bmsr_second BMSR_LSTATUS and
    BMSR_ANEGCOMPLETE, while retaining bmsr_first as the latch-low
    discriminator sample.

## Allowed Future Proof Classifications

- mdio-phy1-bmsr-double-sample-link-ready: corrected-target BMCR preconditions
  are clear, second BMSR has BMSR_LSTATUS and BMSR_ANEGCOMPLETE set, and
  capture-chain-v4/identity evidence is decisive.
- mdio-phy1-bmsr-double-sample-link-not-ready: corrected-target BMCR
  preconditions are clear, second BMSR lacks BMSR_LSTATUS or
  BMSR_ANEGCOMPLETE, and capture-chain-v4/identity evidence is decisive.
- mdio-phy1-bmsr-double-sample-bmcr-precondition-blocker: BMCR reset,
  loopback, autoneg-restart, missing MPE, or ambiguous corrected-target
  identity prevents readiness classification before a BMSR conclusion.
- mdio-phy1-bmsr-double-sample-timeout: initial or post-MAN corrected NSR.IDLE
  polling times out.
- precise-staging-capture-blocker: candidate/control identity, serial, TFTP,
  or restore evidence is not precise enough to classify.
- no-mdio-no-ethernet-rp1-ethernet-phy1-link-readiness-control: paired control
  output from the same reporting path with no MDIO target construction and no
  volatile MDIO load/store.

## Paired Control Boundary

The paired no-MDIO/no-Ethernet control must preserve the same capture/reporting
path while:

- constructing no MDIO target address and no MAN frame;
- performing no NCR, NSR, or MAN volatile load/store for Ethernet MDIO;
- withholding candidate-only PHY address, BMCR/BMSR register numbers, MAN
  frame values, raw sample values, and result-valid fields;
- retaining contract id, rejected-claim labels, retained-risk labels, and
  control classification;
- classifying only as
  no-mdio-no-ethernet-rp1-ethernet-phy1-link-readiness-control.

## Evidence Gates For A Future Hardware Proof

- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test --quiet.
- static archive/image review before hardware publication.
- serialized Pi 5 candidate/control proof with hardwareTestLock.
- lab-controller API candidate identity, fresh serial cursor, TFTP delta,
  final pre-restore identity, snapshot/restore, and post-restore identity
  evidence.
- capture-chain-v4 replay for candidate and control.
- boot-staging identity gate for candidate and control.
- evidence-consistency guard over task markdown, classification, capture, and
  evidence map outputs.
- jq empty on task-owned JSON evidence.
- git diff --check.
- mdbook build if docs/src files are touched.
- git diff --cached --check before commit.

## Same-Shaped Retry Policy

Same-shaped one-sample PHY1 status diagnostic retries remain closed. The next
useful proof must either use the double-sampled BMSR latch-low discriminator
above, including a paired no-MDIO/no-Ethernet control, or stop with a precise
precondition/capture blocker.

## Findings

- fixed: selected one exact next discriminator from the accepted PHY1 status
  frontier: read-only double-sampled BMSR link/autoneg readiness.
- fixed: explained why the accepted one-sample BMSR link-status=false result
  does not by itself settle link readiness because Linux treats BMSR link
  status as latched low and keeps the second BMSR value.
- fixed: preserved the corrected-target MDIO read boundary and named exact
  future BMCR/BMSR register numbers, MAN frames, operation order, paired
  control behavior, classifications, and evidence gates.
- fixed: rejected MACB NSR_LINK as the selected discriminator because retained
  source ties that helper to SGMII fixed-state handling while the Pi 5 rp1_eth
  source uses rgmii-id with phy1.
- deferred: implementing the future proof, hardware publication, visible
  double-sample results, PHY configuration, autonegotiation restart, link
  forcing, PHY reset/GPIO32 ownership, Ethernet driver behavior, interrupts,
  DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition remain future explicit tasks.
- not-an-issue: no hardwareTestLock was acquired because this task is
  source/task evidence only.
- removed: no obsolete implementation, docs, or evidence was removed.

## Rejected Claims And Retained Risks

Rejected claims:

- link readiness proven by this source contract;
- runtime double-sampled BMSR evidence;
- PHY configuration writes;
- autonegotiation restart;
- link forcing;
- PHY reset or GPIO32 ownership;
- MACB NSR_LINK as the selected discriminator;
- broad MDIO/PHY ownership;
- Ethernet driver behavior;
- interrupt delivery/completion;
- DMA/descriptors;
- packet I/O;
- networking;
- sockets;
- SSH;
- Phase 12.2;
- phase transition.

Retained risks:

- A future double-sampled BMSR proof can prove only PHY1 register-state link
  readiness or not-readiness at the selected instant.
- Even link-ready BMSR evidence would not prove MAC configuration, DMA,
  descriptor rings, interrupts, packet I/O, sockets, SSH, or Phase 12.2.
- GPIO32/PHY reset ownership and PHY configuration writes remain separate
  unaccepted frontiers.

## Evidence

- Classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-link-readiness-source-contract/classification.json.
- Evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-link-readiness-source-contract/evidence-map.json.
- Linux genphy_update_link excerpt:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-link-readiness-source-contract/source/linux-v6.12-phy_device-genphy_update_link-excerpt.txt.
- Input closeout:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-closeout.md.
- Linux MII constants:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-uapi-linux-mii.h.
- Linux MACB source:
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c.

## Validation

- static source/task evidence inspection: accepted closeout, status proof,
  Linux MII constants, MACB source, genphy_update_link excerpt, Phase 12 docs,
  roadmap, and task-owned JSON inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list with dispositions: satisfied.
- Exact next discriminator named and justified from the accepted PHY1 status
  frontier: satisfied.
- Allowed operations, forbidden operations, paired control behavior, and future
  evidence gates are explicit: satisfied.
- No-objective-discriminator blocker: not applicable; an objective source-backed
  discriminator exists.
- Contract committed before implementation or hardware proof begins: satisfied
  by the commit for this task.

## Next Action

Supervisor planning is required to add an explicit future proof task before
implementation or hardware. The selected future proof id is
phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof-20260614,
but this worker task does not create, queue, or promote it. Do not start
implementation, hardware proof, PHY configuration, reset/GPIO32 action, packet
I/O, networking, SSH, Phase 12.2, or a phase transition directly from this
source contract.
