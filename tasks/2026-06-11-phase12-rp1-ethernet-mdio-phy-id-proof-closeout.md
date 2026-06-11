# Phase 12 RP1 Ethernet MDIO PHY-ID Proof Closeout

Task id: phase12-rp1-ethernet-mdio-phy-id-proof-closeout-20260611

Status: accepted

Classification: rp1-ethernet-mdio-phy-id-ncr-mpe-clear-frontier-closed

Evidence level: static inspection of the accepted MDIO PHY-ID Pi 5 proof task
record, classification/evidence JSON, capture summary, Phase 12 docs, roadmap,
and git history. No Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, runtime MDIO transaction, NCR.MPE write,
GPIO32/PHY reset write, Ethernet driver behavior, interrupt handling,
DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or phase
transition was performed by this closeout.

## Goal

Close out the accepted MDIO PHY-ID Pi 5 proof without expanding acceptance
beyond the guarded Clause 22 PHY-ID discriminator boundary.

## Scope Performed

- Consumed the accepted serialized Pi 5 proof from commit
  508a1e1d2f0200c8e78581c9e600f3c7f310393e.
- Reconciled the candidate/control capture-chain-v4 evidence against the
  accepted MDIO PHY-ID source contract, guard core, guard closeout, Phase 12
  docs, roadmap, and git history.
- Recorded the accepted blocker: candidate observed MACB_MID context 0x70109
  at 0x1c001000fc and NCR 0x20001927 at 0x1c00000000, but NCR.MPE bit 4 was
  clear, so the no-write source contract correctly prevented MAN writes.
- Preserved the paired no-MDIO/no-Ethernet control as proof of the same
  capture-chain-v4 reporting path while constructing no MDIO target or MAN
  frame and performing no MDIO transaction.
- Closed same-shaped MDIO PHY-ID hardware retries for this candidate/control
  pair because repeating the same no-write discriminator cannot make NCR.MPE
  set, prove PHY-ID visibility, or grant NCR.MPE write ownership.
- Requested supervisor planning for any future MPE-enablement, PHY reset,
  MDIO/PHY ownership, or Ethernet follow-up because this closeout does not
  make a bounded ownership task mechanically objective.

## Findings

- fixed: reconciled the accepted candidate/control capture-chain-v4 evidence
  as decisive for the current MDIO PHY-ID discriminator boundary.
- fixed: recorded that the candidate performed no NCR, MAN, GPIO32, or PHY
  reset write because NCR.MPE was clear before the first permitted MAN write.
- fixed: recorded the accepted source-contract blocker:
  mdio-phy1-physid-source-contract-violated-blocker.
- fixed: preserved candidate fields proving the no-write boundary:
  ncr-before=0x20001927, ncr-mpe-precondition-met=false,
  ncr-mpe-write-performed=false, man-writes-performed=false,
  claims-runtime-mdio-transaction=false, and touched-fields=none.
- fixed: preserved the paired no-MDIO/no-Ethernet control result with no MDIO
  target construction, no MAN frame construction, no MAN writes, no runtime
  MDIO transaction, and touched-fields=none.
- fixed: closed same-shaped MDIO PHY-ID hardware retries for this
  candidate/control pair.
- deferred: visible PHY-ID reads, NCR.MPE enablement or write authority,
  broad MDIO/PHY ownership, PHY reset ownership, Ethernet driver behavior,
  interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition remain unaccepted future
  supervisor-planned work.
- not-an-issue: no hardwareTestLock was acquired because this closeout is a
  static checkpoint over already accepted hardware evidence.
- removed: no obsolete implementation, docs, or task evidence was removed.

## Accepted Boundary

The accepted frontier is only a guarded MDIO PHY-ID discriminator blocked by
the no-write NCR.MPE precondition:

- candidate/control evidence joined selected-tree identity, expected TFTP
  fetch bytes, run-unique serial marker freshness, final pre-restore identity,
  restore proof, and task-owned JSON;
- candidate retained observed-window MACB_MID context raw value 0x70109 at
  0x1c001000fc;
- candidate retained NCR before value 0x20001927 at 0x1c00000000;
- candidate observed NCR.MPE bit 4 clear and therefore performed no NCR.MPE
  write, no MAN write, no MAN restore write, no GPIO32 write, and no PHY reset
  write;
- candidate classified as mdio-phy1-physid-source-contract-violated-blocker;
- paired control classified as
  no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-control while constructing no
  MDIO target or MAN frame and performing no MDIO transaction;
- lab boot state was restored and hardwareTestLock was released before the
  proof was accepted.

This accepts only that the current guarded MDIO PHY-ID candidate/control pair
reaches the no-write NCR.MPE-clear blocker with capture-chain-v4 evidence. It
does not accept visible PHY-ID reads, runtime MDIO transaction success,
NCR.MPE write ownership, broad MDIO/PHY ownership, PHY reset ownership,
Ethernet driver behavior, interrupt completion, DMA/descriptors, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

## Same-Shaped Retry Policy

Same-shaped MDIO PHY-ID candidate/control hardware retries are closed for this
candidate/control pair. A future Phase 12.1 MDIO/PHY task needs supervisor
planning with a qualitatively different discriminator or explicit source-backed
ownership contract, such as an NCR.MPE enablement/restore boundary or a
different prerequisite that can make the first MAN write safe. This closeout
does not select or create that task.

## Rejected Claims And Retained Risks

Rejected claims:

- visible PHY-ID read evidence;
- runtime MDIO transaction success;
- NCR.MPE write ownership or write permission;
- broad MDIO/PHY ownership;
- PHY reset ownership;
- Ethernet driver readiness;
- interrupt completion;
- DMA/descriptors;
- packet I/O;
- networking;
- sockets;
- SSH;
- Phase 12.2;
- phase transition.

Retained risks:

- NCR.MPE is clear under the accepted candidate setup, so the selected MAN
  reads cannot safely execute under the current source contract.
- The proof does not decide whether firmware, reset sequencing, clock/reset
  state, PHY reset, or another MACB/GEM prerequisite should enable NCR.MPE.
- GPIO32 / ETH_RST_N ownership remains unproven.
- Future MDIO/PHY progress requires supervisor planning before any MPE write,
  PHY reset, Ethernet driver, or packet behavior is implemented.

## Evidence

- Accepted proof task:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-pi5-proof.md.
- Proof classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-pi5-proof/classification.json.
- Proof evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-pi5-proof/evidence-map.json.
- Proof capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-pi5-proof/capture-summary.json.
- Closeout classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-proof-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-proof-closeout/evidence-map.json.
- Project docs:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap:
  docs/src/roadmap.md.

## Validation

- static inspection: accepted MDIO PHY-ID proof task record, proof
  classification/evidence JSON, capture summary, Phase 12 docs, roadmap, and
  git history reviewed.
- JSON checks: jq empty on proof and closeout classification/evidence JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles proof/blocker evidence without expanding acceptance
  beyond the MDIO/PHY-ID discriminator boundary: satisfied.
- Same-shaped retry policy is explicit: satisfied; closed for this
  candidate/control pair.
- NextAction selects one bounded follow-up only if mechanically objective;
  otherwise planningNeeded=true with a concrete blocker reason: satisfied;
  supervisor planning is required because NCR.MPE is clear and no MPE
  enablement or MDIO/PHY ownership follow-up is mechanically objective.
- Accepted checkpoint is committed before any follow-up starts: satisfied by
  the commit recorded in supervisor state after this task.

## Next Action

Supervisor planning required. The next Phase 12.1 task must be explicitly
scoped with acceptance criteria before any NCR.MPE write/restore authority,
PHY reset assertion/deassertion, MDIO/PHY ownership, Ethernet driver
implementation, DMA/descriptors, interrupts, packet I/O, networking, sockets,
SSH, Phase 12.2, or phase transition work starts.
