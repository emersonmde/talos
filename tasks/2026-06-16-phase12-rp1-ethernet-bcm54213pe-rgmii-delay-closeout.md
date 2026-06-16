# Phase 12.1 RP1 Ethernet BCM54213PE RGMII Delay Closeout

Task id: phase12-rp1-ethernet-bcm54213pe-rgmii-delay-closeout-20260616

Status: accepted

Classification:
bcm54213pe-rgmii-delay-frontier-closed-tx-delay-read-capture-blocker

Evidence level: static/task evidence inspection, accepted source-contract
review, accepted proof-core review, accepted serialized Pi 5 proof review,
JSON evidence validation, docs build, and diff checks. No new Pi 5 hardware
run, boot archive publication, lab mutation, hardwareTestLock acquisition,
power-cycle, TFTP/serial capture, runtime code change, GPIO32 reset/config
write, Broadcom uncontracted selector/config write, interrupt ownership,
packet I/O, networking, SSH, Phase 12.2, or phase transition was performed.

## Goal

Close out the BCM54213PE RGMII delay frontier by reconciling the accepted
source contract, local/static proof core, and serialized Pi 5 proof. Record the
precise runtime blocker without broadening acceptance into link readiness,
packet readiness, networking, SSH, or Phase 12.2.

## Scope Performed

- Inspected the accepted RGMII delay source contract and its selected PHY1
  rgmii-id AUX_CTL and SHD clock-delay write/readback boundary.
- Inspected the accepted local/static proof core, validators, candidate/control
  boot scenarios, compile evidence, and rejected claim set.
- Inspected the accepted serialized Pi 5 proof task, classification JSON,
  capture summary, evidence map, selected-tree/TFTP evidence, serial freshness
  evidence, final identity, and restore proof.
- Updated Phase 12 project docs and roadmap with the closed TX delay
  selected-register read capture blocker.
- Preserved all rejected GPIO32/reset, uncontracted Broadcom selector/config,
  interrupt, packet, networking, SSH, Phase 12.2, and phase-transition claims.

## Findings

- fixed: the source contract reduced the Linux-backed BCM54213PE rgmii-id path
  to PHY1 RX delay through AUX_CTL shadow misc RGMII_SKEW_EN, TX delay through
  SHD CLK_CTL GTXCLK_EN, and only then the already accepted BMCR
  restart/convergence discriminator.
- fixed: the proof core implemented that boundary with candidate/control boot
  scenarios and validators that reject MII_CTRL1000 master-mode writes,
  uncontracted Broadcom selector/config access, GPIO32/reset, interrupt
  ownership, packet/networking claims, and phase transition claims.
- fixed: the paired no-MDIO/no-Ethernet Pi 5 control retained selected tree
  8064606a64700931ae0887c2a7d4a0dfb8f899af9f09e7f86c6d8f2ae3b9282c, two
  matching 50,008-byte TFTP serves, fresh serial nonce evidence, final identity,
  and restore proof while constructing no target facts.
- fixed: the candidate retained selected tree
  9d34d9007a837a0f671c0e627fe85c98531d9a1fa5fe60b88b802a350483be58, two
  matching 53,736-byte TFTP serves, fresh serial nonce evidence, final identity,
  and restore proof.
- fixed: the candidate reached exactly one RX delay write/readback attempt on
  PHY1 AUX_CTL and observed ncr-before/ncr-after 0x10, rx-pre-raw 0x71e7,
  rx-write-value 0xf1e7, rx-readback-raw 0x71e7, and
  rx-readback-rgmii-skew-en=true.
- fixed: the first failing runtime layer is the TX delay selected-register read
  after RX delay write/readback matched. The candidate stopped before TX write,
  BMCR restart, and convergence polling.
- fixed: the terminal runtime facts are tx-pre-raw 0x0, tx-write-value 0x0,
  tx-readback-raw 0x0, tx-readback-gtxclk-en=false, rgmii-delay-write-count
  0x1, bmcr-write-count 0x0, bmcr-write-performed false, and classification
  rgmii-delay-capture-blocker.
- selected: the next bounded boundary is a supervisor-planned precise blocker
  follow-up for the TX delay selected-register read layer, or an explicit pause.
  No mechanically unblocked queued follow-up exists after this closeout.
- blocked: the queued link-ready packet-readiness source checkpoint remains
  dependency-blocked because this proof did not reach a link-ready/autoneg
  complete frontier.
- deferred: GPIO32/PHY reset ownership, further Broadcom selector/config
  writes, interrupt ownership, PHY/MAC configuration, packet I/O, networking,
  sockets, SSH, Phase 12.2, and alternate Phase 12.1 boundaries require a
  separate supervisor-planned task.
- rejected: this closeout does not accept link readiness, link-not-ready after
  delay configuration, Ethernet driver readiness, packet transport, networking,
  sockets, SSH, Phase 12.2, or a phase transition.
- removed: no task-owned source, script, docs, or evidence files were removed.
- not-an-issue: no new hardware run or inconclusive-run triage was needed
  because the accepted Pi 5 proof retained decisive selected-tree, TFTP, serial
  freshness, final identity, and restore evidence.

## Reconciliation

The accepted RGMII delay proof is a thin feature attempt. It did not merely add
diagnostics: it attempted the selected BCM54213PE rgmii-id delay configuration
path before the previously accepted BMCR restart/convergence discriminator. The
control preserved the same capture and freshness shape while constructing no
MDIO, MAN, MACB, GPIO32, PHY, interrupt, packet, networking, SSH, or
phase-transition target facts.

The serialized Pi 5 proof satisfied the capture chain and stopped at the TX
delay selected-register read. The accepted runtime fact is therefore only that
the RX delay AUX_CTL read/write/readback path completed with RGMII_SKEW_EN
observed true, and the next selected-register read for TX delay did not
complete. Because TX write, BMCR restart, and convergence polling were not
reached, neither link-ready nor link-not-ready packet-readiness follow-up is
authorized by this closeout.

## Frontier

Closed frontier:
bcm54213pe-rgmii-delay-frontier-closed-tx-delay-read-capture-blocker.

Accepted: source-backed PHY1 RGMII RX delay write/readback reached selected
hardware under selected-tree identity, same-power-cycle TFTP byte serves,
cursor-nonce serial freshness, final identity, restore proof, and paired
no-MDIO/no-Ethernet control evidence.

Accepted blocker: TX delay selected-register read did not complete after RX
delay write/readback matched. The accepted candidate stopped with exactly one
RGMII delay write attempt and zero BMCR restart writes.

Deferred: supervisor selection of a precise TX delay selected-register read
discriminator, explicit pause, or another bounded Phase 12.1 follow-up.
GPIO32/PHY reset ownership, uncontracted Broadcom selector/config writes,
interrupt ownership, PHY/MAC configuration, packet I/O, networking, sockets,
SSH, Phase 12.2, and phase transition remain rejected or deferred.

Not accepted: TX delay write/readback success, BMCR restart after delay
configuration, link readiness, link-not-ready after the full delay path,
Ethernet driver readiness, packet behavior, networking, sockets, SSH, Phase
12.2, or a phase transition.

## Next Boundary

Supervisor planning is required before any follow-up. The only explicit next
boundary selected by this closeout is a precise TX delay selected-register read
blocker follow-up, or an explicit pause. The worker must not promote the queued
link-ready packet-readiness source checkpoint because its dependency on a
link-ready/autoneg-complete frontier is not satisfied.

This closeout authorizes no hardware action by itself.

## Evidence

- RGMII delay source contract:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-source-contract.md.
- Source contract classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-source-contract/classification.json.
- RGMII delay proof core:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-proof-core.md.
- Proof core classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-proof-core/classification.json.
- Proof core evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-proof-core/evidence-map.json.
- RGMII delay Pi 5 proof:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-pi5-proof.md.
- Pi 5 proof classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-pi5-proof/classification.json.
- Pi 5 proof capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-pi5-proof/capture-summary.json.
- Pi 5 proof evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-pi5-proof/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-closeout/evidence-map.json.

## Acceptance Check

- Closeout task record reconciles source/core/hardware evidence with findings
  dispositions: satisfied.
- Next boundary or blocker is explicit and dependency-gated: satisfied by
  supervisor-planned TX delay selected-register read blocker follow-up or
  explicit pause.
- Rejected packet/networking/SSH/Phase 12.2/phase-transition claims remain
  explicit: satisfied.
- Docs/evidence/state updates are committed before any next task starts:
  satisfied once this task is committed.

## Validation

- static/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Stop at supervisor planning after this closeout is accepted and committed. No
explicit queued follow-up is mechanically unblocked. Supervisor should select a
precise TX delay selected-register read blocker follow-up or explicit pause
before any hardware, GPIO32/reset, Broadcom selector/config, interrupt, packet
I/O, networking, SSH, Phase 12.2, or phase-transition work.
