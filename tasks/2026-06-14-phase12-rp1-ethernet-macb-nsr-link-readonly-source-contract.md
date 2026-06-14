# Phase 12 RP1 Ethernet MACB NSR_LINK Read-Only Source Contract

Task id: phase12-rp1-ethernet-macb-nsr-link-readonly-source-contract-20260614

Status: accepted

Classification: rp1-ethernet-macb-nsr-link-readonly-source-contract-accepted

Evidence level: static/source evidence inspection and task-owned JSON
evidence. No runtime implementation, Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, MACB write, PHY configuration
write, autonegotiation restart, link forcing, GPIO32/PHY reset action, packet
I/O, networking, SSH, Phase 12.2, or phase transition was performed.

## Goal

Define the smallest passive MAC-side discriminator after the accepted PHY1
link-not-ready result, without authorizing implementation or hardware proof.

## Scope Performed

- Consumed the accepted PHY1 link-not-ready recovery checkpoint at commit
  3ff94737e1e7267ea3f91e618d58a38da3ab1bf5.
- Reconciled the accepted PHY1 register frontier: BMCR 0x1000, BMSR
  first/second 0x7949, ANAR 0x01e1, ANLPAR 0x0000, corrected-target MDIO read
  boundary, and no-MDIO/no-Ethernet control behavior.
- Reconciled the accepted GPIO32 PHY-reset write/restore no-write frontier and
  GPIO32 event-clear persistent-or-firmware-owned frontier.
- Retained Raspberry Pi Linux MACB source evidence for MACB_NSR,
  MACB_NSR_LINK, and macb_get_pcs_fixed_state().
- Selected one objective follow-up task id for supervisor planning.

## Non-Goals

No Rust implementation, no source report surface change, no hardware action,
no lab mutation, no boot archive publication, no hardwareTestLock acquisition,
no MACB NCR/NCFGR/MAN write, no MACB write of any kind, no PHY configuration
write, no BMCR write, no autonegotiation restart, no link forcing, no
GPIO32/PHY reset action, no GPIO/RIO/pad write, no DMA/descriptors, no packet
I/O, no networking, no sockets, no SSH, no Phase 12.2 work, and no phase
transition.

## Reconciled Inputs

- tasks/2026-06-14-phase12-rp1-ethernet-phy1-link-not-ready-recovery-source-checkpoint.md.
- tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-link-not-ready-recovery-source-checkpoint/classification.json.
- tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-link-not-ready-recovery-source-checkpoint/evidence-map.json.
- tasks/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof.md.
- tasks/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-closeout.md.
- tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/classification.json.
- tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/classification.json.
- tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-proof-closeout/classification.json.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-proof-closeout/classification.json.
- tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-source-contract/source/linux-rpi-6.12-macb-nsr-link-excerpt.txt.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Source Facts

- Raspberry Pi Linux rpi-6.12.y macb.h defines MACB_NSR at offset 0x0008 as
  Network Status.
- The same source defines MACB_NSR_LINK_OFFSET as 0 and MACB_NSR_LINK_SIZE as
  1, making NSR_LINK bit 0.
- Raspberry Pi Linux rpi-6.12.y macb_main.c implements
  macb_get_pcs_fixed_state() by reading MACB NSR and mapping
  MACB_BIT(NSR_LINK) to state->link.
- The accepted observed-window rp1_eth base is 0x1c00100000, so the future
  read-only target address is 0x1c00100008.
- The accepted PHY1 state is link-not-ready at the PHY register frontier:
  BMCR 0x1000, BMSR first 0x7949, BMSR second 0x7949, ANAR 0x01e1, and ANLPAR
  0x0000. BMCR reset, loopback, and autoneg-restart are false; second-sample
  BMSR_LSTATUS and BMSR_ANEGCOMPLETE are false.
- Prior GPIO32 evidence does not authorize reset work: write/restore v2
  stopped before GPIO32/RIO/pad writes because unexpected event-state was
  present, and the event-clear proof classified the event bits as persistent
  or firmware-owned after a guarded clear.

## Selected Contract

Accepted contract id:
phase12-rp1-ethernet-macb-nsr-link-readonly-contract-v1.

~~~text
selected discriminator: rp1-ethernet-macb-nsr-link-readonly
controller: rp1_eth / raspberrypi,rp1-gem / cdns,macb
observed-window base: 0x1c00100000
MACB_NSR offset: 0x0008
MACB_NSR_LINK bit: 0
MACB_NSR target: 0x1c00100008
future candidate operation: one or more volatile 32-bit reads of MACB_NSR
future candidate decode:
  nsr_link = (macb_nsr & 0x00000001) != 0
paired control: same reporting path, no MMIO target construction, no Ethernet volatile load/store
MACB write permission: forbidden
PHY configuration write permission: forbidden
GPIO32/PHY reset permission: forbidden
packet I/O permission: forbidden
~~~

The future candidate may read only MACB_NSR at 0x1c00100008 and decode bit 0
as NSR_LINK. It may retain the accepted input frontier and the observed
MACB_MID context as source/evidence context, but it must not write MACB NCR,
NCFGR, MAN, or any other MACB register. It must not perform PHY MDIO
transactions, PHY configuration writes, BMCR writes, GPIO32/PHY reset actions,
DMA/descriptors, interrupts, or packet I/O.

## Operation Order For A Future Candidate

1. Print a candidate start marker, contract id, and accepted input frontier.
2. Print the accepted PHY1 link-not-ready frontier: BMCR 0x1000, BMSR
   first/second 0x7949, ANAR 0x01e1, ANLPAR 0x0000, and corrected-target MDIO
   read boundary.
3. Print retained GPIO32 no-write and persistent-or-firmware-owned event-clear
   blocker labels.
4. Optionally read observed-window MACB_MID at 0x1c001000fc as context only if
   the future implementation already has the existing read-only helper.
5. Volatile-read MACB_NSR at 0x1c00100008 as a 32-bit value.
6. Decode bit 0 as nsr_link.
7. Classify only the MAC-side link comparator result; do not infer link
   recovery, PHY ownership, MAC configuration, packet I/O, networking, or SSH.

## Allowed Future Proof Classifications

- macb-nsr-link-readonly-link-set: decisive candidate/control identity,
  capture, TFTP, serial, and restore evidence, with MACB_NSR bit 0 set.
- macb-nsr-link-readonly-link-clear: decisive candidate/control identity,
  capture, TFTP, serial, and restore evidence, with MACB_NSR bit 0 clear.
- macb-nsr-link-readonly-precondition-blocker: source contract, observed-window
  identity, or read target precondition is not satisfied before accepting
  NSR_LINK.
- macb-nsr-link-readonly-capture-blocker: candidate/control identity, serial,
  TFTP, capture-chain, or restore evidence is not precise enough to classify.
- no-mmio-no-ethernet-macb-nsr-link-control: paired control output from the
  same reporting path with no MMIO target construction and no Ethernet
  volatile load/store.

## Paired Control Boundary

The paired no-MMIO/no-Ethernet control must preserve the same
capture/reporting path while:

- constructing no MACB_NSR target address;
- performing no volatile Ethernet MMIO load/store;
- performing no MDIO target construction, MAN frame construction, PHY read, or
  PHY write;
- withholding candidate-only MACB_NSR address, raw NSR value, NSR_LINK decode,
  and result-valid fields;
- retaining contract id, rejected-claim labels, retained-risk labels, and
  control classification;
- classifying only as no-mmio-no-ethernet-macb-nsr-link-control.

## Evidence Gates For A Future Hardware Proof

- inconclusive-run triage before code changes: candidate identity, fresh serial
  cursor, TFTP delta, known-good control when appropriate, then candidate
  rerun.
- cargo fmt --all -- --check.
- cargo -Zjson-target-spec test --quiet.
- targeted QEMU/substitute gate only if an existing non-hardware harness covers
  the report path; QEMU is not Pi 5 hardware evidence.
- static archive/image review before hardware publication.
- serialized Pi 5 candidate/control proof with hardwareTestLock.
- lab-controller API candidate identity, fresh serial cursor, TFTP delta,
  final pre-restore identity, snapshot/restore, and post-restore identity
  evidence.
- capture-chain-v4 replay for candidate and control.
- boot-staging identity gate for candidate and control.
- same-power-cycle TFTP byte agreement for candidate and control.
- evidence-consistency guard over task markdown, classification, capture, and
  evidence map outputs.
- jq empty on task-owned JSON evidence.
- git diff --check.
- mdbook build if docs/src files are touched.
- git diff --cached --check before commit.

## Same-Shaped Retry Policy

Same-shaped BMSR/link-readiness retries remain closed. The accepted BMSR
double-sample proof already classified the PHY register-state link frontier as
link-not-ready. A future hardware proof must instead use the passive MACB_NSR
comparator above, or stop with a precise source-contract/capture/staging
blocker.

## Findings

- fixed: selected one exact passive MAC-side discriminator from the accepted
  PHY1 link-not-ready frontier: read-only MACB_NSR bit 0 at 0x1c00100008.
- fixed: retained source evidence for MACB_NSR offset 0x0008, NSR_LINK bit 0,
  and macb_get_pcs_fixed_state() mapping NSR_LINK to link state.
- fixed: accounted for accepted PHY1 BMCR 0x1000, BMSR first/second 0x7949,
  ANAR 0x01e1, ANLPAR 0x0000, corrected-target MDIO read boundary, and paired
  no-MDIO/no-Ethernet controls.
- fixed: preserved the GPIO32 no-write and persistent-or-firmware-owned
  event-clear blockers before any reset or GPIO32 follow-up.
- fixed: defined future candidate operation, paired no-MMIO/no-Ethernet
  control behavior, classifications, evidence gates, and inconclusive-run
  triage.
- deferred: implementing the future proof, hardware publication, MACB_NSR
  runtime evidence, PHY configuration, autonegotiation restart, link forcing,
  PHY reset/GPIO32 ownership, Ethernet driver behavior, interrupts,
  DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition remain future explicit tasks.
- not-an-issue: no hardwareTestLock was acquired because this task is
  source/docs/evidence only.
- removed: no obsolete implementation, docs, or evidence was removed.

## Rejected Claims And Retained Risks

Rejected claims:

- runtime MACB_NSR evidence;
- link recovery;
- link usability beyond the future MACB_NSR comparator bit;
- PHY configuration writes;
- BMCR writes;
- autonegotiation restart;
- link forcing;
- PHY reset or GPIO32 ownership;
- broad MDIO/PHY ownership;
- MACB write ownership;
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

- A future MACB_NSR proof can prove only the MAC-side NSR_LINK comparator at
  the selected instant.
- If MACB_NSR_LINK is clear and PHY1 BMSR remains link-not-ready, a later
  supervisor-planned physical-link/operator or reset-ownership decision may
  be needed, but this source contract does not make that decision.
- If MACB_NSR_LINK is set while PHY1 BMSR is link-not-ready, a later task must
  reconcile MAC/PHY disagreement before any packet or network stack work.
- GPIO32/PHY reset ownership and PHY configuration writes remain separate
  unaccepted frontiers.

## Selected Next Task

Selected future task id for supervisor planning:
phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof-20260614.

This source contract does not create, queue, promote, implement, or run that
future proof. Supervisor planning must add an explicit task with concrete
scope, non-goals, dependencies, acceptance criteria, validation gates, docs
requirements, and evidence requirements before implementation or hardware
work starts.

## Evidence

- Classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-source-contract/classification.json.
- Evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-source-contract/evidence-map.json.
- MACB NSR_LINK source excerpt:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-source-contract/source/linux-rpi-6.12-macb-nsr-link-excerpt.txt.
- Input checkpoint:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-link-not-ready-recovery-source-checkpoint.md.
- Accepted BMSR proof:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof.md.
- Accepted BMSR closeout:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-closeout.md.
- Accepted GPIO32 no-write closeout:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-proof-closeout/classification.json.
- Accepted GPIO32 event-clear closeout:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-proof-closeout/classification.json.

## Validation

- static/source evidence inspection: accepted checkpoint, BMSR proof and
  closeout, GPIO32 blocker evidence, retained source excerpt, Phase 12 docs,
  roadmap, and git history inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list with dispositions: satisfied.
- MACB_NSR offset 0x0008, MACB_NSR_LINK bit 0, observed rp1_eth base
  0x1c00100000, and read-only target address 0x1c00100008 identified:
  satisfied.
- Public source evidence for MACB_NSR/MACB_NSR_LINK and
  macb_get_pcs_fixed_state() retained and cited: satisfied.
- Accepted PHY1 BMCR/BMSR/ANAR/ANLPAR evidence, corrected-target MDIO read
  boundary, and GPIO32 blockers accounted for: satisfied.
- Future candidate and paired control boundaries defined as read-only
  MACB_NSR/no-MMIO-no-Ethernet with no writes or packets: satisfied.
- Future Pi 5 proof gates, including inconclusive-run triage, capture-chain-v4,
  boot-staging identity, same-power-cycle TFTP byte agreement, restore
  evidence, and task-owned JSON evidence specified: satisfied.
- At most one objective follow-up task id selected: satisfied.
- No MACB write ownership, PHY configuration, PHY reset/GPIO32 action, packet
  I/O, networking, sockets, SSH, Phase 12.2, or phase transition accepted or
  implied: satisfied.
- Accepted work committed before follow-up starts: satisfied by this task's
  commit.

## Next Action

Set planningNeeded=true for supervisor creation or revision of
phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof-20260614. Do not start
the follow-up from this contract unless it is explicitly queued with
acceptance criteria and gates.
