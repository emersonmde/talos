# Phase 12 RP1 Ethernet Post-Physical-Precondition Link Status Source Contract

Task id: phase12-rp1-ethernet-post-physical-precondition-link-status-source-contract-20260614

Status: accepted

Classification: rp1-ethernet-post-physical-precondition-link-status-source-contract-accepted

Evidence level: static/source/task evidence inspection and task-owned JSON
evidence. No runtime implementation, Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, lab mutation, MDIO write, MACB
write, PHY configuration write, GPIO32/PHY reset action, packet I/O,
networking, SSH, Phase 12.2, or phase transition was performed.

## Goal

Define the smallest read-only post-physical-precondition link-status proof
after the operator confirmed that the Pi 5 Ethernet physical link path is
present.

## Scope Performed

- Promoted this queued worker task after
  phase12-rp1-ethernet-physical-link-precondition-operator-20260614 was
  accepted with physical_link_precondition=confirmed.
- Reconciled the accepted v2 autoneg-restart proof: one guarded PHY1 BMCR
  write intent value 0x1200, post-BMCR 0x1000, post-BMSR 0x7949/0x7949,
  post-ANAR 0x01e1, post-ANLPAR 0x0000, and passive MACB_NSR_LINK=false.
- Reconciled the prior BMSR double-sample and MACB_NSR_LINK read-only
  contracts and proofs.
- Defined one future read-only candidate/control proof that samples the PHY
  and MAC-side status sources after the confirmed physical-link precondition.
- Preserved rejected claims for GPIO32/PHY reset ownership, PHY configuration,
  packet I/O, networking, SSH, Phase 12.2, and phase transition.

## Non-Goals

No source/runtime code change, no hardware run, no lab mutation, no boot
publication, no hardwareTestLock acquisition, no BMCR write, no PHY
configuration write, no MACB write, no GPIO32/RIO/pad write, no PHY reset
assertion/deassertion, no link forcing, no DMA/descriptors, no packet I/O, no
networking, no sockets, no SSH, no Phase 12.2 work, and no phase transition.

## Reconciled Inputs

- tasks/2026-06-14-phase12-rp1-ethernet-post-autoneg-status-source-checkpoint.md.
- tasks/evidence/2026-06-14-phase12-rp1-ethernet-post-autoneg-status-source-checkpoint/classification.json.
- tasks/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-closeout.md.
- tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/classification.json.
- tasks/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-closeout.md.
- tasks/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-closeout.md.
- tasks/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-source-contract.md.
- tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-source-contract/source/linux-rpi-6.12-macb-nsr-link-excerpt.txt.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-uapi-linux-mii.h.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Source Facts

- Linux mii.h defines MII_BMCR as register 0x00, MII_BMSR as register 0x01,
  MII_ADVERTISE/ANAR as register 0x04, and MII_LPA/ANLPAR as register 0x05.
- Linux mii.h defines BMSR_LSTATUS as 0x0004 and BMSR_ANEGCOMPLETE as 0x0020.
- Prior accepted Talos source contracts already use double BMSR sampling
  because Linux genphy_update_link treats link status as latched low.
- Raspberry Pi Linux rpi-6.12.y macb.h defines MACB_NSR as Network Status at
  offset 0x0008, with NSR_LINK at bit 0.
- Raspberry Pi Linux macb_get_pcs_fixed_state() maps MACB_NSR bit 0 to the
  MAC-side link state.
- The accepted observed-window rp1_eth base is 0x1c00100000, so MACB_NSR is
  at 0x1c00100008.

## Accepted Input Frontier

~~~text
physical_link_precondition: confirmed by operator statement
accepted v2 BMCR write intent: 0x1200
post BMCR: 0x1000
post BMSR first sample: 0x7949
post BMSR second sample: 0x7949
post ANAR: 0x01e1
post ANLPAR: 0x0000
post MACB_NSR_LINK: false
prior BMSR double-sample result: link-not-ready
prior MACB_NSR_LINK result: link-clear
retained GPIO32 blockers: write-restore-v2-no-write,
  event-clear-persistent-or-firmware-owned
~~~

This input frontier proves only that the operator-side physical link path is
not an absent-cable blocker for the next read-only status proof. It does not
prove PHY reset ownership, PHY configuration ownership, Ethernet readiness,
packet I/O, networking, or SSH.

## Selected Contract

Accepted contract id:
phase12-rp1-ethernet-post-physical-precondition-link-status-contract-v1.

~~~text
selected discriminator: rp1-ethernet-post-physical-precondition-link-status
precondition: physical Ethernet link/cabling/setup is confirmed
candidate operation class: read-only status sampling only
PHY target: corrected-target Clause 22 PHY1
selected PHY reads:
  BMCR register 0x00
  BMSR register 0x01, first sample
  BMSR register 0x01, second sample
  ANAR register 0x04
  ANLPAR register 0x05
selected MAC read:
  MACB_NSR at 0x1c00100008, decode bit 0 as NSR_LINK
candidate timing:
  one bounded immediate sample window after boot marker and after the
  confirmed physical-link precondition is recorded in task evidence
paired control:
  same report surface, no MDIO/MAN/MACB target construction, no Ethernet
  volatile load/store
writes allowed: none
packets allowed: none
~~~

## Future Candidate Operation Order

1. Print a candidate start marker, contract id, and the confirmed physical-link
   precondition label.
2. Print the accepted v2 post-autoneg frontier and retained GPIO32 blocker
   labels as input evidence.
3. Read corrected-target PHY1 BMCR.
4. Read corrected-target PHY1 BMSR twice; classify BMSR_LSTATUS and
   BMSR_ANEGCOMPLETE from the second sample.
5. Read corrected-target PHY1 ANAR and ANLPAR.
6. Read passive MACB_NSR at 0x1c00100008 and decode bit 0 as NSR_LINK.
7. Classify only the combined read-only status observation relative to the
   confirmed physical-link precondition.

The future candidate must not write BMCR, MAN, NCR, NCFGR, MACB_NSR, GPIO32,
RIO, pads, or any other PHY/MAC/GPIO register. It must not configure PHY state,
force link, reset the PHY, allocate DMA descriptors, transmit or receive
packets, or infer network readiness.

## Paired Control Boundary

The paired control must preserve the same report surface while:

- constructing no MDIO target and no MAN frame;
- constructing no MACB_NSR target address;
- performing no volatile Ethernet MMIO load/store;
- performing no PHY read, PHY write, MACB read, MACB write, GPIO32 access, DMA
  action, or packet I/O;
- withholding candidate-only raw register values, target addresses, decodes,
  touched-field labels, and result-valid fields;
- retaining contract id, physical-link-precondition label, rejected-claim
  labels, retained-risk labels, and control classification;
- classifying only as no-mdio-no-macb-post-physical-link-status-control.

## Future Report Fields

The future candidate report must include:

- contract id and task id;
- physical_link_precondition=confirmed and operator-evidence reference label;
- accepted v2 input frontier labels and values;
- BMCR raw and decoded BMCR_ANENABLE, BMCR_ANRESTART, isolate, reset, and
  loopback booleans;
- BMSR first and second raw values;
- second-sample BMSR_LSTATUS and BMSR_ANEGCOMPLETE booleans;
- ANAR and ANLPAR raw values and ANLPAR nonzero boolean;
- MACB_NSR raw value and NSR_LINK boolean;
- mdio_read_count, mdio_write_count=0, macb_read_count, macb_write_count=0;
- booleans rejecting GPIO32/PHY reset ownership, PHY configuration, link
  forcing, packet I/O, networking, SSH, Phase 12.2, and phase transition;
- allowed classification and retained-risk labels.

## Allowed Future Proof Classifications

- post-physical-link-status-link-ready: decisive candidate/control identity,
  capture, TFTP, serial, and restore evidence; second-sample BMSR_LSTATUS and
  BMSR_ANEGCOMPLETE are set, ANLPAR is nonzero, and MACB_NSR_LINK is set.
- post-physical-link-status-phy-not-ready: decisive evidence, but PHY-side
  second-sample BMSR_LSTATUS or BMSR_ANEGCOMPLETE remains clear, or ANLPAR
  remains zero.
- post-physical-link-status-mac-not-ready: decisive evidence, PHY-side status
  is ready but MACB_NSR_LINK remains clear.
- post-physical-link-status-phy-mac-disagreement: decisive evidence, PHY-side
  and MAC-side status disagree in a way not covered by the two narrower
  not-ready classifications.
- post-physical-link-status-capture-blocker: candidate/control identity,
  serial, TFTP, capture-chain, boot-staging identity, evidence consistency, or
  restore evidence is not precise enough to classify.
- post-physical-link-status-source-precondition-blocker: the source contract,
  physical-link evidence label, corrected-target PHY1 read boundary, or
  MACB_NSR target precondition is missing before accepting runtime evidence.
- no-mdio-no-macb-post-physical-link-status-control: paired control output
  from the same reporting path with no MDIO/MAN/MACB target construction and no
  volatile Ethernet load/store.

## Evidence Gates For The Future Hardware Proof

- source-contract gates from this task.
- static archive/image review before hardware publication.
- serialized Pi 5 candidate/control proof with hardwareTestLock.
- fresh serial marker/cursor evidence.
- stable same-cursor TFTP delta and same-power-cycle TFTP byte agreement.
- capture-chain-v4 replay for candidate and control.
- boot-staging identity gate for candidate and control.
- final pre-restore identity, restore proof, and post-restore identity.
- evidence-consistency guard over task markdown, classification, capture, and
  evidence map outputs.
- inconclusive-run triage before code/helper changes: candidate identity,
  fresh serial cursor, TFTP delta, known-good control, then unchanged
  candidate rerun.
- jq empty on task-owned JSON evidence.
- git diff --check.
- mdbook build if docs/src files are touched.
- git diff --cached --check before commit.

## Findings

- fixed: accepted the operator physical-link confirmation as the precondition
  that makes one post-physical-precondition read-only status proof objective.
- fixed: selected the exact future candidate status sources: PHY1 BMCR,
  double-sampled BMSR, ANAR, ANLPAR, and passive MACB_NSR_LINK.
- fixed: defined the candidate timing as one bounded immediate sample window
  after the boot marker and after the physical-link precondition is recorded in
  task evidence.
- fixed: defined paired control behavior with no MDIO/MAN/MACB target
  construction and no volatile Ethernet load/store.
- fixed: defined allowed classifications, report fields, evidence gates,
  inconclusive-run triage, and rejected claims for the future proof.
- deferred: guard/runtime implementation and the Pi 5 hardware proof remain
  the explicitly queued follow-up task.
- not-an-issue: no hardwareTestLock was acquired because this source contract
  is static task/evidence work only.
- removed: no obsolete source, docs, task records, or helper scripts were
  removed.

## Rejected Claims And Retained Risks

Rejected claims:

- runtime link readiness from this static contract;
- PHY reset ownership;
- GPIO32 ownership;
- PHY configuration writes;
- BMCR writes;
- MACB writes;
- autonegotiation restart;
- link forcing;
- packet I/O;
- DMA/descriptors;
- interrupts;
- networking;
- sockets;
- SSH;
- Phase 12.2;
- phase transition.

Retained risks:

- A future proof can classify only the sampled PHY/MAC status sources at the
  selected instant.
- If the future proof remains link-not-ready despite the confirmed physical
  link path, follow-up planning should focus on DHCP/IP/subnet assumptions,
  PHY power/reset/strap behavior, or kernel network behavior rather than asking
  for another physical cabling confirmation.
- GPIO32/PHY reset ownership remains unaccepted.
- Packet I/O and network stack work remain blocked until an explicit later task
  accepts the necessary link and driver prerequisites.

## Selected Next Task

Selected future task id:
phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof-20260614.

This task is already queued. It may be promoted only after this source contract
is accepted and committed, hardwareTestLock remains unlocked/restored, and no
conflicting uncommitted changes exist.

## Evidence

- Classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-post-physical-precondition-link-status-source-contract/classification.json.
- Evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-post-physical-precondition-link-status-source-contract/evidence-map.json.
- Operator physical-link precondition task:
  memory/talos-supervisor-state.json currentTask/evidence as of
  2026-06-15T13:10:59Z.
- Post-autoneg source checkpoint:
  tasks/2026-06-14-phase12-rp1-ethernet-post-autoneg-status-source-checkpoint.md.
- V2 autoneg proof classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/classification.json.
- BMSR double-sample closeout:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-closeout.md.
- MACB_NSR_LINK source contract:
  tasks/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-source-contract.md.
- MACB_NSR_LINK closeout:
  tasks/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-closeout.md.

## Validation

- static/source/task evidence inspection: accepted physical-link state
  resolution, post-autoneg checkpoint, v2 proof and closeout, BMSR
  double-sample closeout, MACB_NSR_LINK source contract and closeout, Phase 12
  docs, roadmap, and git history inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Contract names the exact future read-only candidate fields and timing after
  confirmed physical-link precondition: satisfied.
- Contract defines paired control behavior that constructs no MDIO/MAN/MACB
  target and performs no volatile Ethernet access: satisfied.
- Contract selects
  phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof-20260614:
  satisfied.
- Rejected claims for GPIO32/PHY reset ownership, PHY configuration, packet
  I/O, networking, SSH, Phase 12.2, and phase transition remain explicit:
  satisfied.

## Next Action

Mechanically promote
phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof-20260614
on the next worker wake if dependencies remain satisfied. Do not start packet
I/O, networking, SSH, Phase 12.2, or a phase transition from this source
contract.
