# Phase 12.1 RP1 Ethernet BCM54213PE Read-Only Preflight V2 Closeout

Task id: phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-closeout-20260616

Status: accepted

Classification:
bcm54213pe-readonly-preflight-v2-frontier-closed-read-values-accepted-planning-required

Evidence level: static/task evidence inspection, accepted v2 proof-core review,
accepted serialized Pi 5 proof review, JSON evidence validation, docs build,
and diff checks. No new Pi 5 hardware run, boot archive publication, lab
mutation, hardwareTestLock acquisition, power-cycle, TFTP/serial capture,
GPIO32 event clear/reset recovery, BMCR/autoneg write, Broadcom shadow/MMD/aux
access, interrupt ownership, PHY/MAC configuration, packet I/O, networking,
SSH, Phase 12.2, or phase transition was performed.

## Goal

Close out the BCM54213PE read-only preflight v2 frontier, reconcile the
accepted local/static proof core with serialized Pi 5 evidence, and record the
next explicit supervisor-planned boundary without authorizing write,
configuration, link recovery, packet I/O, networking, SSH, or a phase
transition.

## Scope Performed

- Inspected the accepted v2 proof-core task, classification JSON, evidence map,
  selected target set, serial freshness contract, candidate/control marker
  boundaries, and rejected claim set.
- Inspected the accepted v2 Pi 5 proof task, classification JSON, capture
  summary, evidence map, retained control/candidate evidence, TFTP identity,
  serial freshness guard result, final identity, and restore proof summary.
- Reconciled findings against the closed serial freshness frontier and the
  prior v2 hardware-proof blocker.
- Updated Phase 12 project docs and roadmap with the closed read-only v2
  frontier and next boundary.
- Set supervisor planning as the next action because this closeout does not
  authorize link recovery, writes, configuration, networking, or Phase 12.2.

## Findings

- fixed: the v2 proof-core selected exactly PHY1 MII_CTRL1000 0x09 and
  MII_STAT1000 0x0a, separated the candidate pre-MDIO marker from the post-read
  values marker, and retained the cursor-nonce-post-power-freshness-v1 fields.
- fixed: the no-MDIO/no-Ethernet control retained selected-tree identity, two
  matching 50,856-byte same-power-cycle TFTP serves, 17 fresh serial
  marker/nonce occurrences, final identity, and restore proof while
  constructing no Ethernet or MDIO target facts.
- fixed: the candidate retained selected-tree identity, two matching
  52,056-byte same-power-cycle TFTP serves, 17 fresh serial marker/nonce
  occurrences, final identity, and restore proof, then reached the post-read
  marker.
- fixed: the accepted candidate read-only values are PHY1 MII_CTRL1000 0x09
  raw 0x0200 valid and PHY1 MII_STAT1000 0x0a raw 0x0000 valid, with completed
  register count 2.
- fixed: decoded MII_CTRL1000 advertises 1000baseT full-duplex capability and
  does not advertise 1000baseT half-duplex; decoded MII_STAT1000 reports local
  and remote receiver OK false and link-partner 1000 full/half false.
- selected: the next boundary is supervisor planning for any follow-up source
  contract or hardware task; this closeout does not mechanically authorize
  GPIO32/reset recovery, BMCR/autoneg writes, Broadcom selector access,
  interrupt ownership, PHY/MAC configuration, packet I/O, networking, SSH,
  Phase 12.2, or a phase transition.
- deferred: any write/configuration/link-recovery task, including BMCR/autoneg,
  GPIO32/PHY reset ownership, Broadcom shadow/MMD/aux access, interrupt
  handling, packet I/O, or network stack work, requires a separate
  supervisor-planned task with explicit gates.
- rejected: link readiness, Ethernet driver readiness, packet behavior,
  networking, sockets, SSH, Phase 12.2, and phase transition remain rejected.
- removed: no task-owned source, script, docs, or evidence files were removed.
- not-an-issue: no candidate rerun was needed because selected-tree,
  same-power-cycle TFTP, serial freshness, final identity, and restore evidence
  were internally consistent.

## Reconciliation

The accepted v2 proof core closed the local/static surface to two read-only
Clause 22 PHY1 registers and a paired control. The candidate had to prove
fresh pre-MDIO entry separately from post-read completion, and the control had
to retain the same freshness shape while constructing no MDIO, MAN, MACB,
GPIO32/PHY, or RP1 Ethernet target facts.

The serialized Pi 5 proof satisfied that contract. The control showed the
fresh no-MDIO/no-Ethernet shape. The candidate preserved selected-tree
identity, matching same-power-cycle TFTP byte serves, cursor-nonce serial
freshness, final pre-restore identity, and restore proof, then emitted the
post-read values marker with both selected registers complete.

This accepts a narrow read-only visibility fact: on the retained Pi 5 run, PHY1
MII_CTRL1000 0x09 read as 0x0200 and PHY1 MII_STAT1000 0x0a read as 0x0000
under the v2 freshness and identity contract. It does not establish that the
PHY link is ready, that autonegotiation can be restarted safely, that GPIO32 or
PHY reset is owned by Talos, that Broadcom selector spaces are safe to access,
or that packet I/O/networking can begin.

## Frontier

Closed frontier:
bcm54213pe-readonly-preflight-v2-frontier-closed-read-values-accepted.

Accepted: bounded read-only visibility of PHY1 MII_CTRL1000 0x09 raw 0x0200
valid and PHY1 MII_STAT1000 0x0a raw 0x0000 valid, joined with accepted
selected-tree identity, same-power-cycle TFTP byte serves, cursor-nonce serial
freshness guard replay, final identity, restore proof, and a no-MDIO/no-Ethernet
control.

Deferred: supervisor selection of any later source contract or hardware task
for GPIO32/PHY reset ownership, BMCR/autoneg, Broadcom shadow/MMD/aux access,
interrupt ownership, PHY/MAC configuration, packet I/O, networking, SSH,
Phase 12.2, explicit pause, or other Phase 12.1 boundary.

Not accepted: link readiness, Ethernet driver readiness, packet behavior,
networking, sockets, SSH, Phase 12.2, or a phase transition.

## Next Boundary

Supervisor planning is required before any follow-up task starts. No
write/configuration/link-recovery/networking task is authorized by this
closeout, and the worker must not infer the next direction from the accepted
read-only register values alone.

The closeout sets planningNeeded=true for supervisor selection of the next
explicit Phase 12.1 boundary or an explicit pause.

## Evidence

- V2 proof-core task:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-proof-core.md.
- V2 proof-core classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-proof-core/classification.json.
- V2 proof-core evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-proof-core/evidence-map.json.
- V2 Pi 5 proof task:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-pi5-proof.md.
- V2 Pi 5 proof classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-pi5-proof/classification.json.
- V2 Pi 5 proof capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-pi5-proof/capture-summary.json.
- V2 Pi 5 proof evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-pi5-proof/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-readonly-preflight-v2-closeout/evidence-map.json.

## Acceptance Check

- Closeout task record reconciles proof-core and hardware evidence with
  findings dispositions: satisfied.
- Frontier classification names accepted register values or precise first
  failing invariant: satisfied by accepted MII_CTRL1000 0x0200 and MII_STAT1000
  0x0000 read-only visibility.
- Rejected claims remain explicit and no future write/configuration/networking
  work is authorized without a separate supervisor-planned task: satisfied.
- Next boundary is explicit: satisfied by planningNeeded=true for supervisor
  selection.
- Docs/evidence/state updates are committed before any next task starts:
  satisfied once this task is committed.

## Validation

- static/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Supervisor planning required before any follow-up. Do not start GPIO32 event
clear/reset recovery, BMCR/autoneg writes, Broadcom shadow/MMD/aux access,
interrupt ownership, PHY/MAC configuration, packet I/O, networking, sockets,
SSH, Phase 12.2, or a phase transition from this closeout.
