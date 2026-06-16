# Phase 12.1 RP1 Ethernet BCM54213PE Autoneg Convergence Closeout

Task id: phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-closeout-20260616

Status: accepted

Classification:
bcm54213pe-autoneg-convergence-frontier-closed-timeout-link-not-ready

Evidence level: static/task evidence inspection, accepted BMCR/autoneg restart
closeout review, accepted convergence proof-core review, accepted serialized
Pi 5 convergence proof review, JSON evidence validation, docs build, and diff
checks. No new Pi 5 hardware run, boot archive publication, lab mutation,
hardwareTestLock acquisition, power-cycle, TFTP/serial capture, GPIO32
reset/config write, Broadcom selector write, interrupt ownership, packet I/O,
networking, SSH, Phase 12.2, or phase transition was performed.

## Goal

Close out the BCM54213PE autoneg convergence frontier by reconciling the
accepted BMCR restart closeout, local/static convergence core, and serialized
Pi 5 proof. Record the accepted timeout/link-not-ready runtime fact and expose
only the dependency-matched timeout source-checkpoint follow-up.

## Scope Performed

- Inspected the accepted BMCR/autoneg restart closeout and its retained
  post-status link-not-ready fact.
- Inspected the accepted local/static convergence core contract, candidate and
  control boot scenarios, fail-closed tests, compile evidence, and rejected
  claim set.
- Inspected the accepted serialized Pi 5 convergence proof task,
  classification JSON, capture summary, evidence map, selected-tree/TFTP
  evidence, serial freshness evidence, final identity, and restore proof.
- Updated Phase 12 project docs and roadmap with the closed convergence
  timeout frontier and selected timeout source-checkpoint boundary.
- Preserved all rejected reset/config/interrupt/packet/networking/SSH/Phase
  12.2 claims.

## Findings

- fixed: the prior BMCR/autoneg restart closeout accepted exactly one
  corrected-target PHY1 BMCR write frame 0x50821200 for value 0x1200 followed
  by immediate post-status samples that remained link-not-ready.
- fixed: the convergence core extended that real feature path only by adding a
  bounded eight-sample convergence poll after the same single accepted BMCR
  restart write; focused tests and validators reject extra writes, selector or
  GPIO32 access, interrupt ownership, packet/networking claims, and phase
  transition claims.
- fixed: the paired no-MDIO/no-Ethernet Pi 5 control retained selected tree
  c0cba209ffcf845d644f4e7461e3305aaed0fc6d5bb0edf2d798bb57f331e17b, two
  matching 49,712-byte TFTP serves, 44 fresh serial nonce occurrences, final
  identity, and restore proof while constructing no target facts.
- fixed: the candidate retained selected tree
  a932c281bd02341694a1440eb1316b6ea6c582c814e1add9f1fef5e2727bafa4, two
  matching 52,248-byte TFTP serves, 39 fresh serial nonce occurrences, final
  identity, and restore proof.
- fixed: the candidate performed exactly one corrected-target PHY1 BMCR write
  frame 0x50821200 for value 0x1200, then completed the accepted bounded
  eight-sample convergence poll schedule.
- fixed: the terminal poll remained link-not-ready: BMCR 0x1000, BMSR
  0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000, MII_CTRL1000 0x0200,
  MII_STAT1000 0x0000, passive MACB_NSR 0x00000006, BMSR link false,
  BMSR autoneg-complete false, MACB_NSR_LINK false, and link-ready-terminal
  false.
- fixed: the local/static terminal label mismatch found by the first hardware
  run was corrected before the accepted rerun; the accepted terminal
  classification is
  bcm54213pe-autoneg-convergence-timeout-link-not-ready.
- selected: the dependency-matched next boundary is only
  phase12-rp1-ethernet-bcm54213pe-post-convergence-timeout-source-checkpoint-20260616.
  The link-ready packet-readiness checkpoint remains dependency-blocked.
- deferred: GPIO32/PHY reset ownership, Broadcom selector/config writes,
  interrupt ownership, PHY/MAC configuration, physical or link-partner state,
  packet I/O, networking, sockets, SSH, Phase 12.2, and any alternate Phase
  12.1 boundary require a separate accepted source checkpoint or supervisor
  plan.
- rejected: this closeout does not accept link readiness, Ethernet driver
  readiness, packet transport, networking, sockets, SSH, Phase 12.2, or a
  phase transition.
- removed: generated boot archives remain target/tmp artifacts; no task-owned
  source, script, docs, or evidence files were removed.
- not-an-issue: no new hardware run or inconclusive-run triage was needed in
  this closeout because the accepted Pi 5 proof retained decisive selected-tree,
  TFTP, serial freshness, final identity, and restore evidence.

## Reconciliation

The accepted convergence proof is a thin real feature attempt, not a diagnostic
substitute. It reuses the previously accepted corrected-target PHY1 BMCR
restart write and adds only a bounded wait/poll window for link/autoneg
convergence. The control preserved the same capture and freshness shape while
constructing no MDIO/MAN/MACB/GPIO32/PHY/RP1 Ethernet target facts.

The serialized Pi 5 proof satisfied that contract and timed out. The accepted
runtime fact is therefore that the selected restart-plus-convergence path ran
with exactly one BMCR restart write and eight bounded poll samples, and the
terminal sample still reported link-not-ready/autoneg-incomplete.

## Frontier

Closed frontier:
bcm54213pe-autoneg-convergence-frontier-closed-timeout-link-not-ready.

Accepted: one corrected-target PHY1 BMCR autoneg enable/restart write frame
0x50821200 / value 0x1200, followed by eight bounded convergence poll samples
under selected-tree identity, same-power-cycle TFTP byte serves, cursor-nonce
serial freshness, final identity, restore proof, and paired no-MDIO/no-Ethernet
control evidence.

Accepted terminal sample: BMCR 0x1000, BMSR 0x7949/0x7949, ANAR 0x01e1,
ANLPAR 0x0000, MII_CTRL1000 0x0200, MII_STAT1000 0x0000, passive MACB_NSR
0x00000006, BMSR link false, BMSR autoneg-complete false, MACB_NSR_LINK false,
and link-ready-terminal false.

Deferred: source-grounded selection of any reset/config/partner-state
follow-up, GPIO32/PHY reset ownership, Broadcom selector/config path,
interrupt ownership, PHY/MAC configuration, packet I/O, networking, sockets,
SSH, Phase 12.2, explicit pause, or other Phase 12.1 boundary.

Not accepted: link readiness, Ethernet driver readiness, packet behavior,
networking, sockets, SSH, Phase 12.2, or a phase transition.

## Next Boundary

The timeout source checkpoint is mechanically dependency-satisfied for the next
worker wake after this closeout is committed:
phase12-rp1-ethernet-bcm54213pe-post-convergence-timeout-source-checkpoint-20260616.

The worker must not promote the link-ready packet-readiness checkpoint unless a
future accepted task records a link-ready/autoneg-complete convergence
frontier. This closeout authorizes no hardware action by itself.

## Evidence

- BMCR/autoneg restart closeout:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-closeout.md.
- Convergence core:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-core.md.
- Convergence core classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-core/classification.json.
- Convergence core evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-core/evidence-map.json.
- Convergence Pi 5 proof:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-pi5-proof.md.
- Pi 5 proof classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-pi5-proof/classification.json.
- Pi 5 proof capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-pi5-proof/capture-summary.json.
- Pi 5 proof evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-pi5-proof/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-autoneg-convergence-closeout/evidence-map.json.

## Acceptance Check

- Closeout task record reconciles source/core/hardware evidence with findings
  dispositions: satisfied.
- Frontier classification names the accepted runtime fact or precise first
  failing invariant: satisfied by timeout/link-not-ready after the accepted
  convergence poll window.
- Timeout/link-not-ready unblocks only the timeout source-checkpoint task:
  satisfied.
- Link-ready packet/networking/SSH follow-up remains blocked: satisfied.
- Unexpected or inconclusive outcome handling is not needed because the
  accepted proof is decisive.
- Docs/evidence/state updates are committed before any next task starts:
  satisfied once this task is committed.

## Validation

- static/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Mechanically promote
phase12-rp1-ethernet-bcm54213pe-post-convergence-timeout-source-checkpoint-20260616
on the next worker wake if dependencies remain satisfied, hardwareTestLock
remains unlocked/restored, supervisorIntervention remains inactive, and
projects/talos remains clean. Do not start link-ready packet-readiness,
GPIO32/reset, Broadcom selector/config, interrupt, packet I/O, networking, SSH,
Phase 12.2, or a phase transition from this closeout.
