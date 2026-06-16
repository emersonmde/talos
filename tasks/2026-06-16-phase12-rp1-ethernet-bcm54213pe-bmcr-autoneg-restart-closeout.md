# Phase 12.1 RP1 Ethernet BCM54213PE BMCR Autoneg Restart Closeout

Task id: phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-closeout-20260616

Status: accepted

Classification:
bcm54213pe-bmcr-autoneg-restart-frontier-closed-post-status-link-not-ready-planning-required

Evidence level: static/task evidence inspection, accepted source checkpoint
review, accepted proof-core review, accepted serialized Pi 5 proof review, JSON
evidence validation, docs build, and diff checks. No new Pi 5 hardware run,
boot archive publication, lab mutation, hardwareTestLock acquisition,
power-cycle, TFTP/serial capture, GPIO32 event clear/reset recovery, Broadcom
shadow/MMD/AUX access, interrupt ownership, PHY/MAC configuration, packet I/O,
networking, SSH, Phase 12.2, or phase transition was performed.

## Goal

Close out the BCM54213PE BMCR/autoneg restart frontier by reconciling the
source checkpoint, local/static proof core, and serialized Pi 5 proof. Record
only the accepted runtime fact and the next explicit supervisor-planning
boundary.

## Scope Performed

- Inspected the accepted link-recovery source checkpoint and its selected
  BMCR/autoneg restart surface.
- Inspected the accepted local/static proof-core contract, candidate/control
  boot scenarios, fail-closed tests, compile-only evidence, and rejected claim
  set.
- Inspected the accepted serialized Pi 5 proof task, classification JSON,
  capture summary, evidence map, selected-tree/TFTP/serial freshness evidence,
  final identity, and restore proof.
- Updated Phase 12 project docs and roadmap with the closed BMCR/autoneg
  restart frontier.
- Set supervisor planning as the next boundary because this closeout does not
  authorize GPIO32/reset recovery, Broadcom selector/config writes, interrupt
  ownership, packet I/O, networking, SSH, Phase 12.2, or a phase transition.

## Findings

- fixed: the source checkpoint selected the thinnest real link-recovery
  surface after read-only BCM54213PE visibility: one corrected-target PHY1
  BMCR autoneg enable/restart write intent, bounded pre/post status reads, and
  a paired no-MDIO/no-Ethernet control.
- fixed: the proof core pinned the exact BMCR write frame to 0x50821200 for
  value 0x1200, rejected extra writes and selector/GPIO32/interrupt/networking
  claims, and passed focused Rust tests plus candidate/control compile-only
  image builds.
- fixed: the no-MDIO/no-Ethernet Pi 5 control retained selected tree
  967662d73a6b078450170d4bca3d31446e29afbe4a5aabcf46c3c1c0ea6b809b, two
  matching 50,192-byte TFTP serves, 20 fresh nonce occurrences, final identity,
  and restore proof.
- fixed: the candidate retained selected tree
  28d2f01e69c584a494bdfc6c5dd3ab82cc9a2ce175abe5a5ac62f6c8709bd15f, two
  matching 53,112-byte TFTP serves, 17 fresh nonce occurrences, final identity,
  and restore proof.
- fixed: the candidate performed exactly one corrected-target PHY1 BMCR write
  frame 0x50821200 for value 0x1200 and sampled the bounded post-status window.
- fixed: post-status evidence remained link-not-ready: post-BMCR 0x1000,
  post-BMSR 0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000, MII_CTRL1000 0x0200,
  MII_STAT1000 0x0000, passive MACB_NSR 0x00000006, BMSR link false,
  BMSR autoneg-complete false, and MACB_NSR_LINK false.
- selected: supervisor planning is the next boundary before any further
  recovery/configuration/networking work. This closeout does not mechanically
  authorize a GPIO32 reset task, Broadcom selector/config task, interrupt task,
  packet task, networking task, SSH task, Phase 12.2, or phase transition.
- deferred: the reason link remains down is still unresolved; plausible future
  boundaries include GPIO32/PHY reset ownership, Broadcom-specific
  configuration, interrupt ownership, physical/partner state, explicit pause,
  or another supervisor-selected Phase 12.1 discriminator.
- rejected: link readiness, Ethernet driver readiness, GPIO32 reset ownership,
  Broadcom selector/config writes, interrupt ownership, packet I/O, networking,
  sockets, SSH, Phase 12.2, and phase transition remain unaccepted.
- removed: generated boot archives remain target/tmp artifacts; no
  task-owned source, script, docs, or evidence files were removed.
- not-an-issue: no candidate rerun or inconclusive-run triage was needed
  because the accepted Pi 5 proof retained internally consistent selected-tree,
  TFTP, serial freshness, final identity, and restore evidence.

## Reconciliation

The checkpoint and proof core turned the accepted BCM54213PE read-only
frontier into one bounded feature attempt: a standard Clause 22 PHY1 BMCR
autoneg enable/restart write followed by status sampling. The control kept the
same boot and freshness shape while constructing no MDIO/MAN/MACB/GPIO32/PHY
or RP1 Ethernet target facts.

The serialized Pi 5 proof satisfied that contract. The candidate wrote exactly
one BMCR restart frame and then sampled the selected status registers under
fresh selected-tree, TFTP, serial, final-identity, and restore evidence. The
accepted runtime fact is therefore not link recovery; it is that the bounded
restart/status path ran and still reported link-not-ready.

## Frontier

Closed frontier:
bcm54213pe-bmcr-autoneg-restart-frontier-closed-post-status-link-not-ready.

Accepted: one corrected-target PHY1 BMCR autoneg enable/restart write intent
with frame 0x50821200 / value 0x1200, followed by bounded post-status sampling
that retained post-BMCR 0x1000, post-BMSR 0x7949/0x7949, ANAR 0x01e1, ANLPAR
0x0000, MII_CTRL1000 0x0200, MII_STAT1000 0x0000, passive MACB_NSR
0x00000006, BMSR link false, BMSR autoneg-complete false, and MACB_NSR_LINK
false.

Deferred: supervisor selection of any later GPIO32/PHY reset ownership,
Broadcom selector/config path, interrupt ownership, PHY/MAC configuration,
physical/partner-state discriminator, packet I/O, networking, sockets, SSH,
Phase 12.2, explicit pause, or other Phase 12.1 boundary.

Not accepted: link readiness, Ethernet driver readiness, packet behavior,
networking, sockets, SSH, Phase 12.2, or a phase transition.

## Next Boundary

Supervisor planning is required before any follow-up task starts. The worker
must not infer a next task from the accepted link-not-ready restart result.

The closeout sets planningNeeded=true for supervisor selection of the next
explicit Phase 12.1 boundary or an explicit pause.

## Evidence

- Link-recovery source checkpoint:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-link-recovery-source-checkpoint.md.
- Source checkpoint classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-link-recovery-source-checkpoint/classification.json.
- BMCR/autoneg proof-core task:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-core.md.
- Proof-core classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-core/classification.json.
- Proof-core evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-core/evidence-map.json.
- BMCR/autoneg Pi 5 proof task:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-pi5-proof.md.
- Pi 5 proof classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-pi5-proof/classification.json.
- Pi 5 proof capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-pi5-proof/capture-summary.json.
- Pi 5 proof evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-pi5-proof/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-bmcr-autoneg-restart-closeout/evidence-map.json.

## Acceptance Check

- Closeout task record reconciles source/core/hardware evidence with findings
  dispositions: satisfied.
- Frontier classification names the accepted runtime fact or the precise first
  failing invariant: satisfied by the accepted post-status-sampled,
  link-not-ready runtime fact.
- Rejected claims remain explicit and no packet I/O/networking/SSH/Phase 12.2
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
clear/reset recovery, Broadcom selector/config writes, interrupt ownership,
PHY/MAC configuration, packet I/O, networking, sockets, SSH, Phase 12.2, or a
phase transition from this closeout.
