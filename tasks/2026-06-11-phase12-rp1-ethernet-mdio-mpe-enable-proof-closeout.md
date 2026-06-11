# Phase 12 RP1 Ethernet MDIO MPE Enable Proof Closeout

Task id: phase12-rp1-ethernet-mdio-mpe-enable-proof-closeout-20260611

Status: accepted

Classification: rp1-ethernet-mdio-mpe-enable-ownership-frontier-closed

Evidence level: static inspection of the accepted NCR.MPE Pi 5 proof task
record, classification/evidence JSON, capture summary, Phase 12 docs, roadmap,
and git history. No Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, MAN write, PHY-ID read, Ethernet driver behavior,
interrupt handling, DMA/descriptors, packet I/O, networking, sockets, SSH,
Phase 12.2, or phase transition was performed by this closeout.

## Goal

Close out the accepted NCR.MPE ownership proof without expanding acceptance
beyond the guarded NCR.MPE set/readback/restore boundary.

## Scope Performed

- Consumed the accepted serialized Pi 5 proof from commit
  1a395c7b51ccb1f809932971bf9906ef818a9498.
- Reconciled candidate-rerun4 and control-rerun5 capture-chain-v4 evidence
  against the accepted NCR.MPE source contract, guard core, guard closeout,
  Phase 12 docs, roadmap, and git history.
- Recorded the accepted ownership boundary: candidate observed MACB_MID
  context 0x70109 at 0x1c001000fc and NCR 0x10 at 0x1c00100000, performed
  only the accepted set/readback/restore sequence, and restored the exact
  pre-run NCR value.
- Preserved the paired no-MDIO/no-Ethernet control as proof of the same
  capture-chain-v4 reporting path while constructing no NCR/MPE target and
  performing no volatile load/store.
- Closed same-shaped NCR.MPE set/readback/restore hardware retries for this
  candidate/control pair because the accepted proof already establishes the
  bounded already-set/restored result and cannot by repetition prove MAN
  transactions, PHY-ID visibility, PHY reset, or broader MDIO/PHY ownership.
- Required supervisor planning for any follow-up because no explicit queued
  task remains after this closeout.

## Findings

- fixed: reconciled the accepted candidate/control capture-chain-v4 evidence
  as decisive for the current NCR.MPE set/readback/restore boundary.
- fixed: recorded that candidate-rerun4 observed NCR.MPE already set:
  pre_raw=0x10, write_value=0x10, post_raw=0x10, restore_raw=0x10,
  restore_eq_pre=true.
- fixed: recorded that the candidate touched only MACB/GEM_NCR.MPE and
  performed no MAN write, no PHY-ID read, no GPIO32/PHY reset write, no
  Ethernet packet I/O, and no DMA/descriptor work.
- fixed: preserved the paired control result with no NCR/MPE target
  construction, no write intent, no volatile load/store, no MAN write, no
  PHY-ID read, and touched-fields=none.
- fixed: closed same-shaped NCR.MPE hardware retries for this
  candidate/control pair.
- deferred: visible PHY-ID reads, runtime MDIO transactions, broad MDIO/PHY
  ownership, PHY reset/GPIO32 ownership, Ethernet driver behavior,
  interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition remain unaccepted future
  supervisor-planned work.
- not-an-issue: no hardwareTestLock was acquired because this closeout is a
  static checkpoint over already accepted hardware evidence.
- removed: no obsolete implementation, docs, or task evidence was removed.

## Accepted Boundary

The accepted frontier is exactly the NCR.MPE set/readback/restore ownership
proof:

- candidate/control evidence joined selected-tree identity, expected TFTP
  fetch bytes, run-unique serial marker freshness, final pre-restore identity,
  restore proof, and task-owned JSON;
- candidate retained observed-window MACB_MID context raw value 0x70109 at
  0x1c001000fc;
- candidate retained NCR target 0x1c00100000 with MPE bit 4 already set;
- candidate performed the accepted NCR.MPE write/readback/restore sequence:
  pre_raw 0x10, write_value 0x10, post_raw 0x10, restore_raw 0x10, and
  restore_eq_pre=true;
- candidate touched only MACB/GEM_NCR.MPE and performed no MAN write, no
  PHY-ID read, no PHY reset/GPIO32 write, no interrupt handling, no
  DMA/descriptor work, and no packet I/O;
- paired control classified as
  no-mdio-no-ethernet-rp1-ethernet-mdio-mpe-enable-control while constructing
  no NCR/MPE target and performing no volatile load/store;
- lab boot state was restored and hardwareTestLock was released before the
  proof was accepted.

This accepts only that Talos can perform the bounded NCR.MPE
set/readback/restore sequence when the bit is already set in the observed
window. It does not accept MAN transactions, visible PHY-ID reads, runtime
MDIO transaction success, broad MDIO/PHY ownership, PHY reset/GPIO32
ownership, Ethernet driver behavior, interrupt completion, DMA/descriptors,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

## Same-Shaped Retry Policy

Same-shaped NCR.MPE set/readback/restore candidate/control hardware retries
are closed for this candidate/control pair. A repeated proof would only
reconfirm the already-set/restored NCR.MPE boundary and would not prove MAN
transaction safety, PHY-ID visibility, PHY reset ownership, broad MDIO/PHY
ownership, Ethernet driver behavior, or packet I/O.

Future MDIO/PHY progress requires supervisor planning with a qualitatively
different discriminator and explicit acceptance criteria. Examples include a
source-backed MAN/PHY-ID retry that consumes the accepted NCR.MPE ownership
frontier, a separate PHY reset prerequisite, or another bounded prerequisite
task. This closeout does not select or create that task.

## Rejected Claims And Retained Risks

Rejected claims:

- MAN transaction success;
- visible PHY-ID read evidence;
- runtime MDIO transaction success;
- broad MDIO/PHY ownership;
- PHY reset/GPIO32 ownership;
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

- NCR.MPE was already set in the accepted candidate run, so this proof does
  not explain which firmware, reset, clock, PHY, or prior initialization state
  established it.
- The proof does not execute MAN reads or establish PHY-ID visibility.
- GPIO32 / ETH_RST_N ownership remains unproven.
- Future MDIO/PHY progress needs a newly planned bounded task before any MAN
  transaction, PHY reset, Ethernet driver, or packet behavior is implemented.

## Evidence

- Accepted proof task:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-pi5-proof.md.
- Proof classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-pi5-proof/classification.json.
- Proof evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-pi5-proof/evidence-map.json.
- Proof capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-pi5-proof/capture-summary.json.
- Closeout classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-proof-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-proof-closeout/evidence-map.json.
- Project docs:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap:
  docs/src/roadmap.md.

## Validation

- static inspection: accepted NCR.MPE proof task record, proof
  classification/evidence JSON, capture summary, Phase 12 docs, roadmap, and
  git history reviewed.
- JSON checks: jq empty on proof and closeout classification/evidence JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles proof evidence without expanding beyond the NCR.MPE
  ownership boundary: satisfied.
- Same-shaped retry policy is explicit: satisfied; closed for this
  candidate/control pair.
- NextAction selects one bounded follow-up only if mechanically objective;
  otherwise planningNeeded=true with a concrete blocker reason: satisfied;
  supervisor planning is required because no explicit queued follow-up remains
  after this closeout.
- Accepted checkpoint is committed before any follow-up starts: satisfied by
  the commit recorded in supervisor state after this task.

## Next Action

Supervisor planning required. The next Phase 12.1 task must be explicitly
scoped with acceptance criteria before any MAN transaction, PHY-ID retry,
PHY reset assertion/deassertion, broad MDIO/PHY ownership, Ethernet driver
implementation, DMA/descriptors, interrupts, packet I/O, networking, sockets,
SSH, Phase 12.2, or phase transition work starts.
