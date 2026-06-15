# Phase 12 RP1 Ethernet Post-Physical Link Status V2 Closeout

Task id: phase12-rp1-ethernet-post-physical-link-status-v2-closeout-20260615

Status: accepted

Classification: post-physical-link-status-v2-closeout-phy-not-ready-frontier

Evidence level: static/task evidence inspection, JSON evidence validation,
diff checks, and docs build.

## Goal

Close out the accepted v2 post-physical link-status proof without expanding
beyond the accepted MAN read-command accounting boundary.

## Scope Performed

- Inspected the accepted v2 proof task record, classification JSON, evidence
  map, and retained candidate/control evidence summary.
- Reconciled the bounded post-physical link-status sample against the accepted
  v2 source/report boundary.
- Recorded retained risks and rejected claims before requesting supervisor
  planning for any follow-up.
- Updated the Phase 12 roadmap/project docs with the accepted closeout frontier.

## Findings

- fixed: the v2 Pi 5 proof is accepted as the current bounded status frontier,
  not as packet I/O, Ethernet readiness, or driver readiness.
- fixed: candidate/control capture-chain-v4, boot-staging identity,
  same-power-cycle TFTP byte agreement, serial freshness, final pre-restore
  identity, and restore proof are decisive for the status sample.
- fixed: the candidate status sample remains
  post-physical-link-status-phy-not-ready with BMCR 0x1000, BMSR
  0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000, MACB_NSR 0x00000006, BMSR link
  false, autoneg complete false, ANLPAR nonzero false, and MACB_NSR_LINK false.
- fixed: the accepted v2 accounting boundary permits exactly the five selected
  Clause 22 PHY1 MAN read-command stores and separates them from forbidden PHY
  configuration, BMCR, MAC configuration, GPIO32/PHY reset, DMA, and packet
  actions.
- fixed: the paired control constructed no MDIO/MAN/MACB target and performed
  no volatile Ethernet access.
- deferred: source-grounded follow-up planning is required for PHY
  power/reset/strap/autoneg status recovery or an explicit pause.
- not-an-issue: the already accepted physical link precondition remains
  accepted and must not be re-asked as the next unblocker.
- not-an-issue: the v2 proof's phy-not-ready result is useful accepted status
  evidence even though it does not authorize runtime recovery writes.

## Reconciliation

The accepted proof resolves the earlier v1 source-contract blocker by making
the MAN read-command stores explicit and bounded. It accepts only the immediate
PHY/MAC status sample after the confirmed physical-link precondition. The
sample shows the PHY and MAC link surfaces still not ready, with no evidence of
ANLPAR partner advertisement and no MACB_NSR link bit.

Because the accepted result remains not ready, the next step is not Phase 12.2,
packet I/O, networking, or SSH. It is also not another request for Matthew to
confirm the physical cable/link-partner precondition. The next worker-owned task
must be supervisor-planned with explicit source/evidence scope for PHY
power/reset/strap/autoneg status recovery, or the supervisor should record a
pause if no safe bounded discriminator exists.

## Rejected Claims And Retained Risks

Rejected claims:

- Ethernet driver readiness;
- link readiness;
- PHY reset ownership;
- GPIO32 ownership or action;
- PHY configuration writes;
- BMCR writes;
- MAC configuration writes;
- link forcing;
- DMA/descriptors;
- packet I/O;
- interrupts;
- networking;
- sockets;
- SSH;
- Phase 12.2;
- phase transition.

Retained risks:

- The accepted status is only a single bounded sample at the selected instant.
- PHY power/reset/strap/autoneg recovery remains unresolved.
- GPIO32/PHY reset ownership remains unaccepted despite the not-ready status.
- Packet I/O and network stack work remain blocked until later explicit tasks
  accept their prerequisites.

## Evidence

- V2 proof task:
  tasks/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof.md.
- V2 proof classification:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof/classification.json.
- V2 proof evidence map:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-pi5-proof/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-closeout/evidence-map.json.

## Validation

- static/task evidence inspection: accepted v2 proof task, classification JSON,
  and evidence map inspected.
- JSON validation: jq empty on closeout classification/evidence-map JSON and
  referenced v2 proof JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Closeout records findings with disposition: satisfied.
- Closeout accepts the v2 link-status frontier while preserving the precise
  phy-not-ready result and rejected claims: satisfied.
- Follow-up planning focuses on source-grounded PHY power/reset/strap/autoneg
  status recovery or pause, not Matthew reconfirmation of the accepted physical
  link precondition: satisfied.
- Packet I/O, networking, SSH, Phase 12.2, and phase transition remain
  explicitly rejected: satisfied.
- Accepted closeout committed before any follow-up starts: satisfied once this
  task is committed.

## Next Action

Supervisor planning required before any fresh discriminator. The next plan must
select one bounded source-grounded PHY power/reset/strap/autoneg status recovery
task or an explicit pause; it must not ask Matthew to reconfirm the already
accepted physical link precondition and must not start packet I/O, networking,
SSH, Phase 12.2, or a phase transition.
