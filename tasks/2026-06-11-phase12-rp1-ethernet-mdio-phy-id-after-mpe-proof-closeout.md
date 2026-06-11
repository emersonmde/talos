# Phase 12 RP1 Ethernet MDIO PHY-ID After-MPE Proof Closeout

Task id: phase12-rp1-ethernet-mdio-phy-id-after-mpe-proof-closeout-20260611

Status: accepted

Classification: rp1-ethernet-mdio-phy-id-after-mpe-visible-frontier-closed

Evidence level: static inspection of the accepted corrected-target after-MPE
MDIO PHY-ID Pi 5 proof task record, classification/evidence JSON, capture
summary, Phase 12 docs, roadmap, and git history. No Pi 5 hardware run, boot
archive publication, hardwareTestLock acquisition, runtime MDIO transaction,
NCR write, MAN write, PHY-ID read, PHY reset/GPIO32 action, Ethernet driver
behavior, interrupt handling, DMA/descriptors, packet I/O, networking,
sockets, SSH, Phase 12.2, or phase transition was performed by this closeout.

## Goal

Close out the accepted corrected-target after-MPE MDIO PHY-ID proof without
expanding acceptance beyond the selected MAN transaction and MAN.DATA return
boundary.

## Scope Performed

- Consumed the accepted serialized Pi 5 proof from commit
  eed28a2bb185d55f58180626dc4718af0fc6d54a.
- Reconciled candidate/control capture-chain-v4 evidence against the accepted
  after-MPE source contract, guard core, guard closeout, Phase 12 docs,
  roadmap, and git history.
- Recorded the accepted visible-read boundary: candidate observed corrected
  NCR 0x10 at 0x1c00100000, performed no NCR write, wrote only the accepted
  PHYSID1/PHYSID2 MAN frames after the MPE gate passed, and extracted MAN.DATA
  physid1=0xffff and physid2=0xffff.
- Preserved the paired no-MDIO/no-Ethernet control as proof of the same
  capture-chain-v4 reporting path while constructing no MDIO target or MAN
  frame and performing no volatile load/store.
- Closed same-shaped corrected-target after-MPE PHY-ID hardware retries for
  this candidate/control pair because the accepted proof already establishes
  the bounded 0xffff/0xffff MAN.DATA return result and repetition cannot prove
  PHY responsiveness, PHY reset ownership, link state, Ethernet driver
  behavior, or broader MDIO/PHY ownership.
- Required supervisor planning for any follow-up because no explicit queued
  task remains after this closeout.

## Findings

- fixed: reconciled the accepted candidate/control capture-chain-v4 evidence
  as decisive for the current corrected-target after-MPE PHY-ID boundary.
- fixed: recorded that candidate observed MACB_MID context 0x70109 at
  0x1c001000fc and corrected NCR 0x10 at 0x1c00100000.
- fixed: recorded that candidate performed no NCR write and wrote only the
  accepted MAN frames 0x600a0000 and 0x600e0000 after corrected NCR.MPE bit 4
  was already set.
- fixed: recorded the accepted MAN.DATA result: physid1=0xffff and
  physid2=0xffff, with both valid flags true.
- fixed: preserved the paired control result with no MDIO target construction,
  no MAN frame construction, no volatile load/store, no MAN writes, no runtime
  MDIO transaction, and touched-fields=none.
- fixed: closed same-shaped after-MPE PHY-ID hardware retries for this
  candidate/control pair.
- deferred: PHY responsiveness, PHY reset/GPIO32 ownership, broad MDIO/PHY
  ownership, link state, Ethernet driver behavior, interrupts,
  DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition remain unaccepted future supervisor-planned work.
- not-an-issue: no hardwareTestLock was acquired because this closeout is a
  static checkpoint over already accepted hardware evidence.
- removed: no obsolete implementation, docs, or task evidence was removed.

## Accepted Boundary

The accepted frontier is exactly the selected corrected-target after-MPE MAN
transaction and MAN.DATA return proof:

- candidate/control evidence joined selected-tree identity, expected TFTP
  fetch bytes, run-unique serial marker freshness, final pre-restore identity,
  restore proof, and task-owned JSON;
- candidate retained observed-window MACB_MID context raw value 0x70109 at
  0x1c001000fc;
- candidate retained corrected NCR target 0x1c00100000 with NCR.MPE bit 4
  already set as 0x10 and performed no NCR write;
- candidate wrote only the accepted Clause 22 PHY1 PHYSID1/PHYSID2 MAN frames:
  0x600a0000 and 0x600e0000;
- candidate polled corrected NSR.IDLE and extracted MAN.DATA physid1=0xffff
  and physid2=0xffff;
- paired control classified as
  no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-after-mpe-control while
  constructing no MDIO target or MAN frame and performing no volatile
  load/store;
- lab boot state was restored and hardwareTestLock was released before the
  proof was accepted.

This accepts only that Talos can execute the selected corrected-target MAN
sequence after the accepted NCR.MPE boundary and observe MAN.DATA 0xffff/0xffff
under capture-chain-v4. It does not accept PHY responsiveness, link state,
PHY reset/GPIO32 ownership, broad MDIO/PHY ownership, Ethernet driver
behavior, interrupt completion, DMA/descriptors, packet I/O, networking,
sockets, SSH, Phase 12.2, or a phase transition.

## Same-Shaped Retry Policy

Same-shaped corrected-target after-MPE PHY-ID candidate/control hardware
retries are closed for this candidate/control pair. A repeated proof would
only reconfirm the selected MAN transaction and 0xffff/0xffff MAN.DATA return
boundary; it would not prove that a PHY responded, that ETH_RST_N is owned,
that link is possible, or that Ethernet packet I/O is ready.

Future Phase 12.1 progress requires supervisor planning with a qualitatively
different discriminator and explicit acceptance criteria. Examples include a
source-backed PHY reset prerequisite, a post-reset MDIO discriminator, a MAC
configuration prerequisite, or another bounded ownership task. This closeout
does not select or create that task.

## Rejected Claims And Retained Risks

Rejected claims:

- PHY responsiveness;
- link state;
- PHY reset/GPIO32 ownership;
- broad MDIO/PHY ownership;
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

- The 0xffff/0xffff MAN.DATA result is a visible return value for the selected
  MAN transactions, not proof of a responsive PHY or usable link.
- GPIO32 / ETH_RST_N ownership remains unproven.
- The proof does not establish whether PHY reset, strap state, clock/reset
  sequencing, MAC configuration, or another prerequisite is needed before a
  meaningful PHY response can be expected.
- Future MDIO/PHY or Ethernet progress needs a newly planned bounded task
  before any PHY reset, Ethernet driver, packet behavior, networking, sockets,
  SSH, Phase 12.2, or phase transition work starts.

## Evidence

- Accepted proof task:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof.md.
- Proof classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof/classification.json.
- Proof evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof/evidence-map.json.
- Proof capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof/capture-summary.json.
- Closeout classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-proof-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-proof-closeout/evidence-map.json.
- Project docs:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap:
  docs/src/roadmap.md.

## Validation

- static inspection: accepted after-MPE MDIO PHY-ID proof task record, proof
  classification/evidence JSON, capture summary, Phase 12 docs, roadmap, and
  git history reviewed.
- JSON checks: jq empty on proof and closeout classification/evidence JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles proof evidence without expanding beyond the
  corrected-target after-MPE PHY-ID boundary: satisfied.
- Same-shaped retry policy is explicit: satisfied; closed for this
  candidate/control pair.
- NextAction selects one bounded follow-up only if mechanically objective;
  otherwise planningNeeded=true with a concrete blocker reason: satisfied;
  supervisor planning is required because no explicit queued follow-up remains
  after this closeout and the accepted 0xffff/0xffff result does not make PHY
  reset, broad MDIO/PHY ownership, Ethernet driver, packet I/O, networking, or
  Phase 12.2 work mechanically objective.
- Accepted checkpoint is committed before any follow-up starts: satisfied by
  the commit recorded in supervisor state after this task.

## Next Action

Supervisor planning required. The next Phase 12.1 task must be explicitly
scoped with acceptance criteria before any PHY reset assertion/deassertion,
post-reset MDIO discriminator, broad MDIO/PHY ownership, Ethernet driver
implementation, DMA/descriptors, interrupts, packet I/O, networking, sockets,
SSH, Phase 12.2, or phase transition work starts.
