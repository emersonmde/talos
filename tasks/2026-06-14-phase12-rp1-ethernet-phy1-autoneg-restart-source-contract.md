# Phase 12 RP1 Ethernet PHY1 Autoneg Restart Source Contract

Task id: phase12-rp1-ethernet-phy1-autoneg-restart-source-contract-20260614

Status: accepted

Classification: rp1-ethernet-phy1-autoneg-restart-source-contract-accepted

Evidence level: static source/docs/evidence inspection, rg/source reference
capture, and task-owned JSON evidence. No runtime implementation, Pi 5
hardware run, boot archive publication, hardwareTestLock acquisition,
GPIO32/RIO/pad write, PHY reset action, MACB write, link forcing, packet I/O,
networking, sockets, SSH, Phase 12.2, or phase transition was performed.

## Goal

Define the smallest source-backed PHY1 autonegotiation-restart recovery
contract after accepted passive PHY-side and MAC-side link-clear evidence.

## Scope Performed

- Reconciled the accepted corrected-target PHY1 vector: BMCR 0x1000, BMSR
  first/second 0x7949, ANAR 0x01e1, ANLPAR 0x0000.
- Reconciled the accepted PHY-side double-sampled link-not-ready result and
  MAC-side MACB_NSR raw 0x6 / NSR_LINK=false result.
- Reconciled the corrected-target MDIO/MAN boundary and retained GPIO32
  no-write/event-clear blockers.
- Retained source-backed BMCR_ANENABLE, BMCR_ANRESTART, and Linux PHY
  autonegotiation-restart semantics.
- Defined exact future candidate/control report fields, preconditions, restore
  expectations, allowed classifications, and rejected claims.
- Selected one objective follow-up task already present in the queue.

## Non-Goals

No Rust implementation, no source report surface change, no hardware action,
no lab mutation, no boot archive publication, no hardwareTestLock acquisition,
no GPIO32/RIO/pad write, no PHY reset assertion/deassertion, no MACB write, no
NCR write, no link forcing, no packet I/O, no DMA/descriptors, no interrupts,
no networking, no sockets, no SSH, no Phase 12.2 work, and no phase
transition.

## Reconciled Inputs

- tasks/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-v4-closeout.md.
- tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-v4-closeout/classification.json.
- tasks/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-closeout.md.
- tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-closeout/classification.json.
- tasks/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-closeout.md.
- tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-closeout/classification.json.
- tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-proof-closeout/classification.json.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-proof-closeout/classification.json.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-uapi-linux-mii.h.
- tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-source-contract/source/linux-v6.12-phy-autoneg-restart-excerpt.txt.
- tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-link-readiness-source-contract/source/linux-v6.12-phy_device-genphy_update_link-excerpt.txt.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Source Facts

- Linux mii.h defines MII_BMCR as register 0x00, BMCR_ANRESTART as 0x0200,
  and BMCR_ANENABLE as 0x1000.
- Linux v6.12 genphy_restart_aneg() calls phy_modify(MII_BMCR, BMCR_ISOLATE,
  BMCR_ANENABLE | BMCR_ANRESTART). This source-backed behavior sets
  autonegotiation enable and restart while clearing isolate in the generic
  Linux path.
- This Talos contract does not claim isolate recovery. Because the accepted
  BMCR is already 0x1000 with isolate clear, the future candidate must
  precondition-check isolate clear and perform exactly one guarded write that
  sets BMCR_ANENABLE and BMCR_ANRESTART while preserving other pre-read BMCR
  bits.
- Linux v6.12 genphy_update_link() treats BMCR_ANRESTART as autoneg being
  started and reports link down until later status reads prove otherwise. A
  restart write is therefore a recovery attempt, not Ethernet readiness.

## Accepted Frontier

The accepted input evidence remains input evidence only:

~~~text
PHY1 BMCR: 0x1000
PHY1 BMSR first sample: 0x7949
PHY1 BMSR second sample: 0x7949
PHY1 ANAR: 0x01e1
PHY1 ANLPAR: 0x0000
MACB_NSR raw: 0x00000006
MACB_NSR_LINK: false
BMCR_ANENABLE: true
BMCR_ANRESTART: false
BMSR_LSTATUS: false
BMSR_ANEGCOMPLETE: false
corrected target: Clause 22 PHY1 MDIO/MAN boundary
retained GPIO32 blockers: write-restore-v2-no-write,
  event-clear-persistent-or-firmware-owned
~~~

This frontier does not prove a physical link partner, PHY reset ownership,
Ethernet readiness, packet I/O, networking, or SSH.

## Selected Contract

Accepted contract id:
phase12-rp1-ethernet-phy1-autoneg-restart-contract-v1.

Future candidate boundary:

~~~text
selected discriminator: rp1-ethernet-phy1-autoneg-restart
target: corrected-target Clause 22 PHY1 only
BMCR register: 0x00
BMCR_ANENABLE: 0x1000
BMCR_ANRESTART: 0x0200
precondition: BMCR_ISOLATE is clear; GPIO32/reset/MACB write ownership is not needed
write count: exactly one guarded BMCR write
write value: pre_bmcr | BMCR_ANENABLE | BMCR_ANRESTART
post-readback: BMCR, double BMSR, ANAR, ANLPAR, passive MACB_NSR_LINK
paired control: same report surface, no MDIO/MAN/MACB target construction,
  no volatile load/store
~~~

## Operation Order For A Future Candidate

1. Print a candidate start marker, contract id, and accepted input frontier.
2. Pre-read PHY1 BMCR, BMSR, ANAR, and ANLPAR through corrected-target Clause
   22 MDIO/MAN operations.
3. Reject the candidate as a precondition blocker before any write if the
   pre-read BMCR is unavailable or shows isolate set.
4. Perform exactly one guarded BMCR write setting BMCR_ANENABLE and
   BMCR_ANRESTART while preserving other pre-read BMCR bits.
5. Bounded-read back BMCR.
6. Bounded double-sample BMSR, using the second sample for link/autoneg
   completion classification.
7. Bounded-read back ANAR and ANLPAR.
8. Read the passive MACB_NSR_LINK comparator only as a post-restart comparator.
9. Classify only the bounded recovery result; do not infer Ethernet readiness.

## Paired Control Boundary

The paired control must preserve the same report surface while:

- constructing no MDIO/MAN/MACB target;
- performing no volatile load/store;
- performing no PHY read, PHY write, BMCR write, BMSR read, ANAR read, ANLPAR
  read, or MACB_NSR read;
- withholding candidate-only target, raw register, decode, touched-field, and
  result-valid fields;
- retaining contract id, accepted frontier labels, rejected-claim labels, and
  control classification;
- classifying only as no-mdio-no-macb-phy1-autoneg-restart-control.

## Future Report Fields

The future candidate report must include:

- contract id and task id;
- accepted input frontier labels and values;
- pre BMCR, BMSR, ANAR, and ANLPAR raw values;
- BMCR write value, write count, and touched bit fields;
- post BMCR raw value;
- post BMSR first and second raw values;
- post ANAR and ANLPAR raw values;
- passive MACB_NSR raw value and NSR_LINK decode;
- booleans for BMCR_ANENABLE, BMCR_ANRESTART, BMSR_LSTATUS,
  BMSR_ANEGCOMPLETE, ANLPAR nonzero, MACB_NSR_LINK, and every rejected claim;
- allowed classification and retained-risk labels.

## Allowed Future Classifications

- phy1-autoneg-restart-link-ready: decisive candidate/control identity,
  capture, TFTP, serial, and restore evidence; exactly one guarded restart
  write; post-readback proves BMSR_LSTATUS and BMSR_ANEGCOMPLETE set.
- phy1-autoneg-restart-link-still-not-ready: decisive evidence and exactly one
  guarded restart write, but post-readback still shows link/autoneg incomplete
  or ANLPAR remains zero.
- phy1-autoneg-restart-physical-or-operator-precondition-blocker: source
  contract and capture are decisive, but the post-restart state still indicates
  a missing physical link partner, cabling, switch, or other operator-side link
  precondition.
- phy1-autoneg-restart-precondition-blocker: pre-read, target, BMCR isolate,
  write-guard, or readback precondition fails before accepting runtime restart
  evidence.
- phy1-autoneg-restart-capture-blocker: candidate/control identity, serial,
  TFTP, capture-chain, or restore evidence is not precise enough to classify.
- no-mdio-no-macb-phy1-autoneg-restart-control: paired control output from the
  same report path with no MDIO/MAN/MACB target construction and no volatile
  load/store.

## Restore And Evidence Expectations

The later hardware proof must restore the lab boot tree and release
hardwareTestLock. This source contract does not claim PHY reset, PHY
configuration restore, MACB restore, or GPIO32 restore ownership. Any future
proof must retain candidate/control archive identity, same-power-cycle TFTP
byte agreement, serial freshness, final pre-restore identity, post-restore
identity, capture-chain-v4 replay, boot-staging identity replay, and
evidence-consistency guard output.

## Findings

- fixed: selected one exact recovery discriminator after passive link-clear
  evidence: corrected-target PHY1 BMCR autoneg-restart with bounded readback.
- fixed: preserved the accepted PHY1 BMCR 0x1000, BMSR 0x7949/0x7949, ANAR
  0x01e1, ANLPAR 0x0000, and MACB_NSR raw 0x6 / NSR_LINK=false frontier as
  input evidence only.
- fixed: cited source-backed BMCR_ANENABLE and BMCR_ANRESTART semantics and
  Linux behavior that treats BMCR_ANRESTART as link down until later status
  reads prove otherwise.
- fixed: defined future candidate operation order, control behavior, report
  fields, classifications, restore expectations, and rejected claims.
- deferred: runtime guard implementation, Pi 5 hardware proof, closeout,
  physical-link/operator handling, PHY reset ownership, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition remain separate
  explicit tasks.
- not-an-issue: no hardwareTestLock was acquired because this task is
  source/docs/evidence only.
- removed: no obsolete implementation, docs, or evidence was removed.

## Rejected Claims And Retained Risks

Rejected claims:

- runtime BMCR write evidence;
- GPIO32/PHY reset ownership;
- MACB writes;
- link forcing;
- packet I/O;
- networking;
- sockets;
- SSH;
- Phase 12.2;
- phase transition.

Retained risks:

- A future autoneg restart can still classify link-still-not-ready if the
  physical link partner, cabling, switch, or reset/power state is not ready.
- BMCR_ANRESTART is a recovery action, not proof of Ethernet readiness.
- GPIO32/PHY reset ownership remains unaccepted.
- MACB_NSR_LINK remains only a comparator, not packet I/O or network stack
  readiness.

## Selected Next Task

Accepted next mechanically unblocked task:
phase12-rp1-ethernet-phy1-autoneg-restart-guard-core-20260614.

The next task is guard-core implementation only. It must not run hardware,
publish boot artifacts, acquire hardwareTestLock, or start networking/SSH work.

## Validation

- static source/docs/evidence inspection: pass.
- rg/source reference capture for BMCR_ANENABLE/BMCR_ANRESTART and Linux PHY
  autoneg restart semantics: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check before commit: pass.
