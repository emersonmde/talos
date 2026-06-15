# Phase 12 RP1 Ethernet Post-Physical-Precondition Link Status Closeout

Task id: phase12-rp1-ethernet-post-physical-precondition-link-status-closeout-20260614

Status: accepted

Classification: post-physical-link-status-source-contract-revision-required

Evidence level: static/task evidence inspection and task-owned JSON evidence.
No source/runtime code change, hardwareTestLock acquisition, boot archive
publication, lab mutation, hardware action, packet I/O, networking, SSH, Phase
12.2 work, or phase transition was performed.

## Goal

Close out the post-physical-precondition link-status proof/blocker and
reconcile whether the recorded Pi 5 runtime status can drive follow-up
planning.

## Scope Performed

- Inspected the accepted post-physical-precondition source contract.
- Inspected the committed Pi 5 proof/blocker task record and classification
  evidence.
- Reconciled the proof's decisive capture/staging evidence with the failed
  source-contract gate.
- Preserved rejected claims and selected no implementation follow-up.

## Findings

- fixed: the proof task has committed candidate/control selected-tree identity,
  same-power-cycle TFTP byte agreement, serial marker freshness, final
  pre-restore identity, restore evidence, capture-chain-v4 replay, and
  boot-staging identity replay.
- fixed: the candidate runtime report reached the intended status sample and
  recorded BMCR 0x1000, BMSR 0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000,
  MACB_NSR 0x00000006, BMSR link-status false, BMSR autoneg-complete false,
  ANLPAR nonzero false, and MACB_NSR_LINK false.
- fixed: the paired control constructed no MDIO/MAN/MACB target and performed
  no volatile Ethernet access.
- deferred: the runtime candidate result remains evidence, but it is not an
  accepted planning frontier because the accepted source contract forbids MAN
  writes while the implemented corrected-target PHY1 reads require MACB MAN
  read-command transactions.
- deferred: supervisor planning is required to define a revised source
  contract or alternate read-only discriminator that explicitly decides how
  MACB MAN read-command writes are bounded, counted, and reported.
- removed: no obsolete source, task, evidence, helper, or docs files were
  removed.
- not-an-issue: no hardware restore action was needed in this closeout because
  the proof task had already restored the baseline tree before releasing the
  hardwareTestLock.

## Reconciliation

The source contract selected a combined read-only status sample: corrected-
target PHY1 BMCR, double-sampled BMSR, ANAR, ANLPAR, plus passive MACB_NSR.
It also stated that the future candidate must not write MAN and retained
macb_write_count=0.

The proof task showed that capture identity and hardware evidence were
decisive, but the candidate reached PHY1 over MACB MAN read-command frames.
Those frames are not PHY configuration writes, BMCR writes, GPIO32 writes,
packet I/O, DMA work, or link forcing, but they are still writes to the MACB
MAN register. Accepting the runtime not-ready result under the existing
contract would silently relax the contract after the hardware run.

This closeout therefore keeps the accepted classification at
post-physical-link-status-source-contract-revision-required. The runtime
post-physical-link-status-phy-not-ready report is retained as non-frontier
evidence only. A future task must be planned explicitly with one of these
bounded choices before another proof uses the status result:

- revise the source contract to allow and count MACB MAN read-command writes
  as the bounded mechanism for Clause 22 PHY reads while still forbidding PHY
  writes, MAC configuration writes, GPIO32/PHY reset action, packet I/O, DMA,
  networking, SSH, and phase transition claims;
- or select a different discriminator that avoids MAN transactions entirely;
- or pause this Phase 12.1 path if neither boundary is acceptable.

## Evidence

- Prior proof task:
  tasks/2026-06-15-phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof.md.
- Prior proof classification:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof/classification.json.
- Prior proof evidence map:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-precondition-link-status-pi5-proof/evidence-map.json.
- Source contract:
  tasks/2026-06-14-phase12-rp1-ethernet-post-physical-precondition-link-status-source-contract.md.
- Closeout classification:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-precondition-link-status-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-precondition-link-status-closeout/evidence-map.json.

## Validation

- static/task evidence inspection: inspected the accepted source contract,
  proof/blocker task record, proof classification, and proof evidence map.
- JSON validation: jq empty on closeout task-owned JSON evidence.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Closeout records findings with disposition: satisfied.
- Closeout reconciles proof/blocker evidence without expanding acceptance
  beyond the selected read-only status proof: satisfied by rejecting the
  runtime result as a frontier until the MAN read-command boundary is
  replanned.
- NextAction either selects one bounded follow-up or sets planningNeeded with
  a concrete reason: satisfied by setting planningNeeded=true for source-
  contract revision planning.
- Accepted closeout committed before follow-up starts: satisfied after commit.

## Next Action

Supervisor planning is required before any fresh link-status proof,
source-contract revision, MAN read-command counting change, PHY configuration,
GPIO32/PHY reset action, packet I/O, networking, SSH, Phase 12.2, or phase
transition. No worker task is mechanically unblocked after this closeout.
