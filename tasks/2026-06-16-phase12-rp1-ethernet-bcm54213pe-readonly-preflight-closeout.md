# Phase 12 RP1 Ethernet BCM54213PE Read-Only Preflight Closeout

Task id: phase12-rp1-ethernet-bcm54213pe-readonly-preflight-closeout-20260616

Status: accepted

Classification:
bcm54213pe-readonly-preflight-frontier-closed-hardware-proof-planning-required

Evidence level: static/task evidence inspection, accepted source-contract
review, accepted local/static report-core review, JSON evidence validation,
docs build, and diff checks. No Pi 5 hardware run, boot archive publication,
lab mutation, hardwareTestLock acquisition, power-cycle, TFTP/serial capture,
volatile Ethernet access, GPIO32 event clear/reset recovery, BMCR write,
Broadcom shadow/MMD/aux access, interrupt ownership, PHY/MAC configuration,
packet I/O, networking, SSH, Phase 12.2, or phase transition was performed.

## Goal

Close out the BCM54213PE read-only preflight report frontier, reconcile the
accepted source/static and report-core evidence, and record the next explicit
supervisor-planned boundary without authorizing hardware directly.

## Scope Performed

- Inspected the accepted read-only preflight source contract, classification
  JSON, source excerpts, and evidence map.
- Inspected the accepted local/static report-core task, classification JSON,
  evidence map, and implemented Rust report/validator boundary.
- Reconciled accepted, deferred, blocked, and rejected claims against the
  accepted phy-not-ready status, accepted physical-link precondition, GPIO32
  persistent-event-state blocker, and MACB/phylink boundary.
- Updated Phase 12 project docs and roadmap with the closed frontier and
  required planning preconditions for any later hardware proof.
- Set supervisor planning as the next action because no explicit queued
  hardware-proof or write/restore task exists after this closeout.

## Findings

- fixed: the selected read-only preflight target set is closed to exactly PHY1
  MII_CTRL1000 0x09 and PHY1 MII_STAT1000 0x0a.
- fixed: the local/static report core encodes candidate metadata and paired
  no-MDIO/no-Ethernet control metadata without runtime volatile access.
- fixed: report-core validators reject forbidden hardware proof, volatile
  access, write/restore, GPIO32 action, BMCR/PHY writes, Broadcom
  shadow/MMD/aux access, interrupt surfaces, PHY/MAC configuration,
  link-readiness, packet/networking/SSH/Phase 12.2, and phase-transition
  claims.
- selected: the read-only preflight frontier is sufficient for supervisor
  planning of a later explicit candidate/control hardware-proof contract for
  MII_CTRL1000/MII_STAT1000 only, with full hardware evidence preconditions
  named below.
- deferred: the later hardware proof itself remains deferred and is not
  authorized by this closeout.
- deferred: MII_BCM54XX_ECR and MII_BCM54XX_IMR remain interrupt-context
  surfaces that need separate planning before any interrupt ownership claim.
- blocked: Broadcom AUX/shadow and Clause 45/MMD EEE surfaces remain blocked
  from the pure read-only set because retained Linux source requires selector
  writes before reading.
- blocked: GPIO32/ETH_RST_N reset recovery remains blocked by accepted
  persistent or firmware-owned GPIO32 event-state evidence.
- rejected: same-shaped BMCR/BMSR/ANAR/ANLPAR/MACB_NSR status retries, BMCR
  autoneg restart, GPIO32 event clear/reset recovery, broad PHY/MAC
  configuration, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition remain rejected.
- removed: no stale source, helper, task, or evidence files were removed.
- not-an-issue: the accepted physical Ethernet link precondition remains
  accepted and was not re-opened.

## Reconciliation

The source contract accepted only two pure Clause 22 read targets for a
local/static report core:

- PHY1 MII_CTRL1000 0x09.
- PHY1 MII_STAT1000 0x0a.

The report core accepted only local/static report metadata and validators. The
candidate report may carry future MAN read-frame metadata for the selected
targets, but it performs no volatile Ethernet access in this task. The paired
control report constructs no MDIO target, no MAN frame, no MACB target, no
GPIO target, and no RP1 Ethernet target facts.

The accepted post-physical status remains phy-not-ready: BMCR 0x1000, BMSR
0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000, MACB_NSR 0x00000006, BMSR link
false, autoneg complete false, ANLPAR nonzero false, and MACB_NSR_LINK false.
The BCM54213PE mapping remains PHYSID1 0x600d / PHYSID2 0x84a2.

The read-only preflight frontier does not prove link readiness or Ethernet
driver behavior. MII_CTRL1000 and MII_STAT1000 can only provide a narrower
future PHY1 gigabit master/slave status snapshot, and any hardware use of that
snapshot must be explicitly planned as a candidate/control proof.

## Frontier

Closed frontier:
bcm54213pe-readonly-preflight-frontier-closed.

Accepted: the source/static and local/static report boundary for exactly PHY1
MII_CTRL1000 0x09 and PHY1 MII_STAT1000 0x0a, including the paired
no-MDIO/no-Ethernet control shape and validators rejecting forbidden claims.

Not accepted: hardware proof, GPIO32/PHY reset ownership, BMCR/autoneg retry,
Broadcom shadow/MMD/aux access, interrupt ownership, PHY/MAC configuration,
link readiness, Ethernet driver readiness, packet I/O, networking, sockets,
SSH, Phase 12.2, or a phase transition.

## Next Boundary

Supervisor planning is required before any follow-up task starts. The selected
next boundary is a later explicit hardware-proof contract for the already
closed read-only target set, unless the supervisor chooses a separate
source/static write-restore contract or an explicit pause.

A later hardware-proof contract must name at least these preconditions:

- candidate identity proof before power.
- fresh serial cursor before power.
- same-power-cycle TFTP delta for candidate and control.
- known-good no-MDIO/no-Ethernet control.
- candidate rerun if the first candidate evidence is inconclusive.
- hardwareTestLock acquisition and release.
- restore proof to the pre-run baseline.
- post-run evidence review that rejects packet I/O, networking, SSH, Phase
  12.2, phase transition, link readiness, GPIO32 reset ownership, BMCR writes,
  Broadcom shadow/MMD/aux access, interrupt ownership, and broad PHY/MAC
  configuration.

No mechanically unblocked queued task exists after this closeout, so
planningNeeded is set for supervisor selection of that explicit next boundary.

## Evidence

- Source contract task:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract.md.
- Source contract classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract/classification.json.
- Source contract evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-source-contract/evidence-map.json.
- Report-core task:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-report-core.md.
- Report-core classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-report-core/classification.json.
- Report-core evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-report-core/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-closeout/evidence-map.json.

## Validation

- static/task evidence inspection: source contract task/classification/evidence
  map, report-core task/classification/evidence map, docs, roadmap, and git
  history inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Closeout reconciles accepted work, deferred work, blocked work, rejected
  claims, docs, validation, and risks: satisfied.
- Closeout states the next boundary: satisfied; a later explicit hardware-proof
  contract is selected for supervisor planning, with separate write/restore or
  pause still available to supervisor.
- Required hardware-proof preconditions are named: satisfied.
- Packet I/O, networking, SSH, Phase 12.2, and phase transition are not
  accepted: satisfied.

## Next Action

Set planningNeeded=true for supervisor planning of the next explicit Phase
12.1 boundary. Do not start hardware, GPIO32 event clear/reset recovery, BMCR
write, Broadcom shadow/MMD/aux access, interrupt ownership, PHY/MAC
configuration, packet I/O, networking, SSH, Phase 12.2, or phase transition
from this closeout.
