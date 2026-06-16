# Phase 12.1 RP1 Ethernet BCM54213PE RGMII Delay TX-Order Closeout

Task id: phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-closeout-20260616

Status: accepted

Classification:
bcm54213pe-rgmii-delay-tx-order-frontier-closed-timeout-link-not-ready

Evidence level: static/task evidence inspection, accepted source-correction
review, accepted proof-core review, accepted serialized Pi 5 proof review,
JSON evidence validation, docs build, and diff checks. No new Pi 5 hardware
run, boot archive publication, lab mutation, hardwareTestLock acquisition,
power-cycle, TFTP/serial capture, runtime code change, GPIO32 reset/config
write, interrupt ownership, packet I/O, networking, SSH, Phase 12.2, or phase
transition was performed.

## Goal

Close out the corrected BCM54213PE RGMII delay TX-order frontier by reconciling
the accepted source correction, local/static proof core, and serialized Pi 5
proof. Record the exact timeout/link-not-ready boundary without converting it
into packet-readiness, networking, SSH, or Phase 12.2 progress.

## Scope Performed

- Inspected the accepted TX-order source correction and its corrected
  interpretation of the earlier RX-to-TX source-control-flow blocker.
- Inspected the accepted TX-order local/static proof core, validators,
  candidate/control boot scenarios, compile evidence, allowed classifications,
  and rejected claim set.
- Inspected the accepted serialized Pi 5 proof task, classification JSON,
  evidence map, capture summary, selected-tree/TFTP evidence, serial freshness
  evidence, final identity, and restore proof.
- Updated Phase 12 project docs and roadmap with the closed timeout/link-not-ready
  frontier.
- Preserved all rejected GPIO32/reset, interrupt, packet, networking, SSH,
  Phase 12.2, and phase-transition claims.

## Findings

- fixed: the TX-order source correction selected a real feature path rather than
  another same-shaped diagnostic: RX success must advance to TX selector/read,
  optional TX write/readback, BMCR restart, and bounded convergence polling.
- fixed: the local/static proof core implemented the corrected stage accounting
  and retained validators for operation order, selected surfaces, readback
  masks, allowed classifications, and rejected packet/networking/phase claims.
- fixed: the paired no-MDIO/no-Ethernet Pi 5 control retained selected tree
  b1cbdd8e46cbff13b804ade11087f43e3bead258d1c8731352d7354ff0c0d2a4, two
  matching 50,656-byte TFTP serves, fresh serial nonce evidence, final identity,
  and restore proof while constructing no target facts.
- fixed: the corrected candidate retained selected tree
  57139d9354cda7f2cde0be128cc9d4216fc99df9f9bf673e541cb6dd0d1be6e1, two
  matching 55,040-byte TFTP serves, fresh serial nonce evidence, final identity,
  and restore proof.
- fixed: the candidate reached RX selected read/write/readback and reported
  rx-readback-rgmii-skew-en=true.
- fixed: the candidate reached TX selector/write/readback accounting, observed
  tx-pre-raw 0x0e00 and tx-readback-raw 0x0e00, and reported
  tx-selected-read-completed=true, tx-readback-completed=true, and
  tx-readback-gtxclk-en=true.
- fixed: the accepted skip policy was exercised: the candidate skipped a
  redundant TX write because GTXCLK_EN was already enabled, recording
  tx-delay-write-skipped-already-enabled=true and
  tx-delay-write-completed=false.
- fixed: after RX/TX delay criteria were satisfied, the candidate executed
  exactly one BMCR restart write and then completed eight bounded convergence
  samples.
- fixed: the terminal runtime layer is timeout/link-not-ready after the corrected
  delay path: poll-bmsr-link-status=false, poll-bmsr-autoneg-complete=false,
  passive-macb-nsr-link=false, and link-ready-terminal=false.
- blocked: the existing link-ready packet-readiness checkpoint remains
  dependency-blocked because the closeout did not accept a link-ready or
  autoneg-complete frontier.
- deferred: supervisor planning is required for any next bounded Phase 12.1
  blocker or pause, such as partner/link-state analysis, GPIO32/PHY reset
  ownership, PHY/MAC configuration, or another explicitly scoped discriminator.
- rejected: this closeout does not accept link readiness, packet transport,
  Ethernet driver readiness, networking, sockets, SSH, Phase 12.2, or a phase
  transition.
- removed: no task-owned source, script, docs, or evidence files were removed.
- not-an-issue: no new hardware run or inconclusive-run triage was needed
  because the accepted Pi 5 proof retained decisive selected-tree, TFTP, serial
  freshness, final identity, restore, and runtime classification evidence.

## Reconciliation

The accepted TX-order proof is a thin feature attempt. It exercised the corrected
BCM54213PE rgmii-id delay path through RX delay read/write/readback, TX selected
read/readback, the accepted already-enabled TX skip policy, BMCR restart, and
bounded convergence polling. The paired control preserved the same capture and
freshness shape while constructing no MDIO, MAN, MACB, GPIO32, PHY, interrupt,
packet, networking, SSH, or phase-transition target facts.

The terminal Pi 5 fact is not a capture blocker and not link readiness. It is a
bounded timeout after the corrected delay criteria and exactly one BMCR restart:
the sampled BMSR link-status bit, BMSR autoneg-complete bit, passive MACB_NSR
link bit, and link-ready terminal all remained false. Because link-ready or
autoneg-complete was not accepted, packet-readiness remains blocked.

## Frontier

Closed frontier:
bcm54213pe-rgmii-delay-tx-order-frontier-closed-timeout-link-not-ready.

Accepted: selected-tree identity, same-power-cycle TFTP byte serves,
cursor-nonce serial freshness, final identity, restore proof, paired
no-MDIO/no-Ethernet control evidence, RX delay read/write/readback, TX selected
read/readback with GTXCLK_EN already set, skip of redundant TX write under the
accepted policy, exactly one BMCR restart write, and bounded convergence samples
ending link-not-ready.

Blocked: the queued link-ready packet-readiness source checkpoint remains
dependency-gated. Its dependency on an accepted link-ready/autoneg-complete
frontier is not satisfied by timeout-link-not-ready evidence.

Deferred: supervisor selection of the next bounded Phase 12.1 blocker or
explicit pause. GPIO32/PHY reset ownership, PHY/MAC configuration, interrupt
ownership, DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2,
and phase transition remain rejected or deferred.

Not accepted: link readiness, autoneg complete, packet behavior, Ethernet driver
readiness, networking, sockets, SSH, Phase 12.2, or a phase transition.

## Next Boundary

Supervisor planning is required before any follow-up. The worker must not
promote the queued link-ready packet-readiness source checkpoint because its
dependency on a link-ready/autoneg-complete frontier is not satisfied.

This closeout authorizes no hardware action by itself.

## Evidence

- TX-order source correction:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-source-correction.md.
- Source correction classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-source-correction/classification.json.
- TX-order proof core:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-proof-core.md.
- Proof core classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-proof-core/classification.json.
- Proof core evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-proof-core/evidence-map.json.
- TX-order Pi 5 proof:
  tasks/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-pi5-proof.md.
- Pi 5 proof classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-pi5-proof/classification.json.
- Pi 5 proof capture summary:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-pi5-proof/capture-summary.json.
- Pi 5 proof evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-pi5-proof/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-16-phase12-rp1-ethernet-bcm54213pe-rgmii-delay-tx-order-closeout/evidence-map.json.

## Acceptance Check

- Closeout classification is explicit and matches retained Pi 5 proof evidence:
  satisfied.
- Link-ready/autoneg-complete was not accepted, so the link-ready
  packet-readiness checkpoint remains blocked: satisfied.
- planningNeeded is set for supervisor selection of the next bounded blocker or
  explicit pause: satisfied once state is updated.
- Packet I/O, networking, sockets, SSH, Phase 12.2, and phase transition remain
  rejected: satisfied.

## Validation

- static/task evidence inspection: pass.
- jq empty on task-owned JSON evidence: pass.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass before commit.

## Next Action

Stop at supervisor planning after this closeout is accepted and committed. No
explicit queued follow-up is mechanically unblocked. Supervisor should select a
bounded Phase 12.1 timeout/link-not-ready follow-up or explicit pause before any
hardware, GPIO32/reset, PHY/MAC configuration, interrupt, packet I/O,
networking, SSH, Phase 12.2, or phase-transition work.
