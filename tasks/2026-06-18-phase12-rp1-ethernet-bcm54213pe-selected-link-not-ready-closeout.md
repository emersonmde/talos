# Phase 12.1 RP1 Ethernet BCM54213PE Selected Link-Not-Ready Closeout

Task id:
phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-closeout-20260618

Status: accepted

Classification:
bcm54213pe-master-mode-write-readback-frontier-paused-planning-required

Evidence level: static/task evidence review, task-owned JSON evidence, docs
build, and diff checks. No runtime code change, Pi 5 hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, power-cycle,
TFTP/serial capture, GPIO32/PHY reset action, interrupt write, MACB
configuration write, packet I/O, networking, sockets, SSH, Phase 12.2, or
phase transition was performed by this closeout.

## Goal

Reconcile the accepted BCM54213PE MII_CTRL1000 master-mode source contract and
Pi 5 proof with the Phase 12.1 link-not-ready frontier, preserve the accepted
evidence, and determine whether any next feature step is mechanically unblocked.

## Scope Performed

- Reviewed the accepted discriminator selection, local/static source-contract
  core, and serialized Pi 5 proof evidence.
- Recorded the selected discriminator result and findings with dispositions.
- Preserved rejected claims for link-ready, autoneg-complete, GPIO32/PHY reset,
  interrupts, APD/EEE/lifecycle, MAC/phylink, packet I/O, networking, sockets,
  SSH, Phase 12.2, and phase transition.
- Updated Phase 12 documentation and the roadmap because the visible frontier
  changed from a selected source contract to a hardware-observed
  MII_CTRL1000 write/readback boundary.
- Selected no next worker task. planningNeeded=true is required because the
  accepted evidence does not mechanically define the next distinct
  feature-led Ethernet step.

## Findings

- fixed: the accepted hardware frontier now includes a serialized Pi 5
  candidate/control proof for the selected BCM54213PE MII_CTRL1000 master-mode
  discriminator.
- fixed: the decisive candidate rerun retained selected-tree identity
  515684b45744c6c89847652c1b34d643a850094d4da3101207fa3b4462d00784,
  same-power-cycle TFTP byte agreement, fresh serial nonce evidence, final
  pre-restore identity, and restore proof.
- fixed: the decisive candidate reported PHY1 MII_CTRL1000 pre-read 0x0200,
  write value 0x1a00, and readback 0x1a00, with the selected write/readback
  completion fields true.
- fixed: the paired control retained the no-MDIO/no-Ethernet classification
  no-mdio-no-ethernet-bcm54213pe-mii-ctrl1000-master-mode-control.
- removed: no source, helper, docs, task, or evidence files were removed.
- deferred: link-ready and autoneg-complete remain unaccepted and require a
  future supervisor-planned discriminator or feature boundary.
- deferred: GPIO32/ETH_RST_N reset ownership, interrupts,
  APD/EEE/lifecycle, MAC/phylink, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition remain outside the accepted frontier.
- rejected: packet-readiness, networking, SSH, Phase 12.2, and phase transition
  are not mechanically unblocked by MII_CTRL1000 write/readback visibility.
- rejected: another same-shaped timeout/status/restart/poll/capture retry is
  not progress without a new accepted discriminator.
- not-an-issue: this closeout did not acquire hardwareTestLock or run
  inconclusive-run triage because it performed only static reconciliation over
  already accepted proof evidence.

## Reconciliation

Input frontier:
mii-ctrl1000-master-mode-write-readback-visible.

Accepted closeout frontier:
bcm54213pe-master-mode-write-readback-frontier-paused-planning-required.

Selected discriminator:
bcm54213pe-phy1-mii-ctrl1000-master-mode-gate-source-contract.

Selected next task: null.

Planning needed: true.

Planning reason: The selected BCM54213PE MII_CTRL1000 proof establishes a
feature-relevant PHY configuration write/readback boundary, but it does not
accept link-ready, autoneg-complete, packet I/O, networking, SSH, Phase 12.2,
or any exact next discriminator. The next Phase 12.1 Ethernet task requires
supervisor planning over the retained evidence instead of worker promotion.

## Evidence

- Discriminator selection:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-link-not-ready-discriminator-selection.md.
- Source-contract core:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-source-contract-core.md.
- Pi 5 proof:
  tasks/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof.md.
- Pi 5 proof classification:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof/classification.json.
- Pi 5 proof evidence map:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof/evidence-map.json.
- Pi 5 proof hardware summary:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-pi5-proof/hardware-run-summary.json.
- Closeout classification:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-18-phase12-rp1-ethernet-bcm54213pe-selected-link-not-ready-closeout/evidence-map.json.

## Acceptance Check

- Closeout records the selected discriminator result and findings with
  dispositions: satisfied.
- Accepted and rejected claims are explicit, especially link-ready,
  autoneg-complete, packet I/O, networking, SSH, Phase 12.2, and phase
  transition: satisfied.
- No next task is selected; planningNeeded=true and no queued hardware or
  networking task is mechanically unblocked: satisfied.
- No runtime implementation, hardware run, lab mutation, boot publication,
  packet I/O, networking, SSH, Phase 12.2, or phase transition was performed:
  satisfied.

## Validation

- static/task evidence review: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Supervisor planning is required before any future Phase 12.1 Ethernet task. Do
not promote packet I/O, networking, sockets, SSH, Phase 12.2, phase transition,
GPIO32/PHY reset action, interrupt/APD/EEE/lifecycle work, MAC/phylink work, or
same-shaped timeout/status/restart/poll/capture retries from this closeout.
