# Phase 12 RP1 Ethernet PHY1 Link-Not-Ready Recovery Source Checkpoint

Task id: phase12-rp1-ethernet-phy1-link-not-ready-recovery-source-checkpoint-20260614

Status: accepted

Classification: rp1-ethernet-phy1-link-not-ready-recovery-source-checkpoint-accepted

Evidence level: static/task evidence inspection and public source excerpt
inspection.

## Goal

Select the smallest source-backed and evidence-backed recovery or
discriminator step after the accepted PHY1 link-not-ready result, without
starting write-capable PHY, GPIO32, MACB, packet, networking, SSH, or phase
transition work.

## Scope Performed

- Inspected the accepted PHY1 BMSR double-sample proof and closeout evidence.
- Reconciled the retained PHY1 status diagnostic and corrected-target MDIO
  register-vector frontier.
- Reconciled prior GPIO32 PHY-reset write/restore, GPIO32 event-state, and
  GPIO32 event-clear blockers.
- Rechecked retained clock/reset prerequisite closeout boundaries.
- Retained a narrow Raspberry Pi Linux MACB source excerpt for the read-only
  MACB NSR_LINK comparator.
- Selected exactly one next objective task id for supervisor planning.

## Findings

- fixed: the accepted PHY1 link-not-ready result is explained by BMCR 0x1000,
  first BMSR 0x7949, second BMSR 0x7949, ANAR 0x01e1, and ANLPAR 0x0000.
  BMCR reset, loopback, and autoneg-restart are false; the second BMSR sample
  has BMSR_LSTATUS=false and BMSR_ANEGCOMPLETE=false.
- fixed: the accepted corrected-target MDIO boundary is sufficient only for
  selected read transactions through the MACB MAN path. It does not accept
  broad MDIO/PHY ownership or PHY configuration writes.
- fixed: Linux v6.12 generic PHY handling treats BMSR link status as
  latched-low and keeps link down when autonegotiation is enabled but
  autoneg-complete is false, matching the accepted double-sample result.
- fixed: prior GPIO32 PHY-reset write/restore v2 remains a no-write frontier.
  The candidate tripped the unexpected event-state guard before GPIO32/RIO/pad
  writes, so it did not prove GPIO32 ownership or PHY reset ownership.
- fixed: prior GPIO32 event-clear remains a persistent-or-firmware-owned
  frontier. The guarded IRQRESET write preserved invariants but event bits
  persisted, so same-shaped event-clear and write/restore retries remain
  closed.
- fixed: retained Raspberry Pi Linux MACB source defines MACB_NSR at offset
  0x0008, MACB_NSR_LINK as bit 0, and macb_get_pcs_fixed_state() as a
  read-only NSR_LINK to phylink state mapping. The previously accepted
  observed-window rp1_eth base makes the read address 0x1c00100008.
- deferred: PHY reset ownership, GPIO32 event-state/source-clearance,
  GPIO32/RIO/pad writes, BMCR/autonegotiation write/readback/restore, PHY
  configuration writes, link forcing, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition.
- not-an-issue: a same-shaped BMSR/link-readiness retry is rejected. It would
  only reread the already accepted BMCR/BMSR state unless it had a distinct
  discriminator; the distinct read-only discriminator available now is MACB
  NSR_LINK, not another PHY BMSR sample.
- removed: no source, helper, task, or evidence files were removed.

## Reconciliation

The accepted PHY1 register vector and status diagnostic established a visible
corrected-target Clause 22 PHY1 path with BMCR 0x1000, BMSR 0x7949, PHYSID1
0x600d, PHYSID2 0x84a2, ANAR 0x01e1, and ANLPAR 0x0000. The accepted
double-sample link-readiness proof then read BMCR 0x1000 and BMSR 0x7949 twice
under capture-chain-v4, boot-staging identity, same-power-cycle TFTP byte
agreement, final pre-restore identity, serial freshness, evidence-consistency
guard, and restore evidence. Since the second BMSR sample has link-status and
autoneg-complete both clear while BMCR autoneg-enable is set and
autoneg-restart is clear, the accepted register-state classification is
link-not-ready.

That evidence does not by itself choose a recovery write. The prior GPIO32
path has two accepted blockers: GPIO32 PHY-reset write/restore v2 stopped
before writes because unexpected event bits were present, and the GPIO32
event-clear proof showed persistent or firmware-owned event bits after the
guarded clear while preserving CTRL, RIO1 OUT/OE/IN, and pad invariants. A
PHY reset path therefore still needs a separate supervisor-planned ownership
or source-clearance task.

BMCR/autonegotiation writes would be qualitatively new PHY configuration
writes and would mutate link negotiation state. The accepted read-only MDIO
frontier does not yet justify that ownership jump. A physical-link/operator
blocker may become appropriate if the MAC and PHY both report link down, but
this checkpoint still has one passive hardware discriminator before making
that external assumption.

## Selected Next Task

Selected next objective task id:
phase12-rp1-ethernet-macb-nsr-link-readonly-source-contract-20260614.

Allowed operations for the selected task:

- Source/docs/evidence contract work only.
- Use retained RP1 observed-window rp1_eth base 0x1c00100000 plus MACB_NSR
  offset 0x0008, producing read-only target 0x1c00100008.
- Define a future candidate that performs only one or more volatile 32-bit
  reads of MACB_NSR and decodes bit 0 as NSR_LINK.
- Define a paired no-MMIO/no-Ethernet control.
- Require capture-chain-v4, boot-staging identity, same-power-cycle TFTP byte
  agreement, serial freshness, final pre-restore identity, restore evidence,
  and task-owned classification/evidence JSON for any later Pi 5 proof.

Non-goals for the selected task:

- No runtime implementation in this checkpoint.
- No hardware action until a later explicitly queued proof task.
- No MACB writes, NCR writes, NCFGR writes, MAN writes, DMA/descriptors,
  interrupts, packets, networking, sockets, SSH, Phase 12.2, or phase
  transition.
- No PHY configuration write, BMCR write, autonegotiation restart, link
  forcing, GPIO32/PHY reset action, GPIO/RIO/pad write, or same-shaped BMSR
  retry.

Validation gates for the selected task:

- static/source evidence inspection.
- jq empty on task-owned JSON evidence.
- git diff --check.
- mdbook build if docs/src files are touched.
- git diff --cached --check before commit.

Evidence requirements for the selected task:

- Retained MACB register-definition source for MACB_NSR and MACB_NSR_LINK.
- Retained or newly fetched Raspberry Pi Linux source excerpt for
  macb_get_pcs_fixed_state().
- Accepted PHY1 link-not-ready proof and closeout evidence.
- Accepted corrected-target MDIO/vector frontier.
- Accepted GPIO32 no-write and event-clear blockers.

## Rejected Follow-Ups

- Same-shaped BMSR/link-readiness retry: rejected as not progress because the
  accepted proof already double-sampled BMSR and classified link-not-ready.
- GPIO32/PHY reset path: deferred until a source-clearance or ownership path
  accounts for the unexpected event-state and persistent/firmware-owned
  event-clear blockers.
- BMCR/autonegotiation write/readback/restore: deferred because it would be a
  new PHY configuration write and the current accepted frontier is read-only.
- Physical-link/operator blocker: deferred until the passive MACB NSR_LINK
  comparator either agrees with PHY link-down or reveals a MAC/PHY mismatch.
- Packet I/O/networking/SSH: rejected; no link, DMA, descriptor, interrupt,
  socket, or userspace-networking boundary is accepted.

## Evidence

- BMSR double-sample proof classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/classification.json.
- BMSR double-sample closeout classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-closeout/classification.json.
- PHY1 status diagnostic classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/classification.json.
- Corrected-target MDIO register-vector classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-mdio-register-vector-pi5-proof-v4-after-root-recovery/classification.json.
- GPIO32 PHY-reset write/restore v2 closeout classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-proof-closeout/classification.json.
- GPIO32 event-clear closeout classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-proof-closeout/classification.json.
- Clock/reset prerequisite closeout classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-clock-reset-prereq-closeout/classification.json.
- Generic PHY source excerpt:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-link-readiness-source-contract/source/linux-v6.12-phy_device-genphy_update_link-excerpt.txt.
- MACB NSR_LINK source excerpt:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-link-not-ready-recovery-source-checkpoint/source/linux-rpi-6.12-macb-nsr-link-excerpt.txt.
- Checkpoint classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-link-not-ready-recovery-source-checkpoint/classification.json.
- Checkpoint evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-link-not-ready-recovery-source-checkpoint/evidence-map.json.

## Validation

- static/task evidence inspection: accepted task records, classification JSON,
  retained source excerpts, Phase 12 docs, roadmap, and git history inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list with dispositions: satisfied.
- BMCR 0x1000, BMSR first/second 0x7949, ANAR 0x01e1, ANLPAR 0x0000, and
  corrected-target MDIO boundary reconciled: satisfied.
- GPIO32 no-write and persistent/firmware-owned event-clear blockers accounted
  for before selecting any reset or GPIO32 follow-up: satisfied.
- Same-shaped BMSR/link-readiness retries rejected: satisfied.
- At most one next objective task id selected with allowed operations,
  non-goals, validation gates, and evidence requirements: satisfied.
- No packet I/O, network stack, sockets, SSH, Phase 12.2, phase transition, or
  fake/kernel-backed networking behavior accepted or implied: satisfied.
- Accepted work committed before follow-up starts: satisfied by this task's
  commit.

## Next Action

Set planningNeeded=true for supervisor creation or promotion of
phase12-rp1-ethernet-macb-nsr-link-readonly-source-contract-20260614. Do not
start the follow-up from this checkpoint unless it is explicitly queued with
acceptance criteria and gates.
