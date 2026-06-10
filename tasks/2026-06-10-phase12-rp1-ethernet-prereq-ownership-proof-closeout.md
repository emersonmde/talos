# Phase 12 RP1 Ethernet Prerequisite Ownership Proof Closeout

Task: phase12-rp1-ethernet-prereq-ownership-proof-closeout-20260610

Status: accepted

Classification: rp1-ethernet-prereq-ownership-report-visibility-frontier-closed

Evidence level: static inspection of accepted Pi 5 proof task record,
classification/evidence JSON, capture summaries, Phase 12 docs, roadmap, and
git history. No Pi 5 hardware run was performed by this closeout.

## Goal

Close out the prerequisite ownership report visibility proof without accepting
runtime prerequisite ownership or Ethernet driver readiness by implication.

## Scope

- Consumed the accepted serialized Pi 5 proof from commits
  01af4b5ab78b5af836ec4527b1de783f525548db and
  cfc4ed93cd0a3d47ee6b0fc8a26c2b3be45f0960.
- Reconciled candidate/control capture-chain-v4 identity, serial output,
  TFTP, archive digests, final identity, restore evidence, and rejected claims.
- Closed same-shaped prerequisite report visibility hardware retries for this
  candidate/control pair.
- Updated Phase 12 documentation and roadmap status to record the accepted
  report-visibility frontier and retained boundaries.
- Requested supervisor planning for the next explicit Phase 12.1 slice because
  this closeout does not own a new source contract, runtime prerequisite
  implementation, RP1 MMIO writes, DMA, descriptors, interrupts, packet I/O,
  networking, sockets, SSH, Phase 12.2, or a phase transition.

## Findings

- fixed: reconciled the candidate/control capture-chain-v4 proof as decisive
  for the accepted report visibility/control pair.
- fixed: recorded the candidate accepted boundary: the report path printed
  context-only observed-window MACB_MID identity, RP1_INT_ETH, pclk/hclk/
  tsu_clk/tx_clk, RGMII-ID phy1, GPIO32 PHY reset metadata, PHY/MDIO policy,
  DMA/descriptor dependency policy, rejected claims, and classification
  rp1-ethernet-prereq-ownership-report-visible.
- fixed: preserved the paired no-ownership/no-Ethernet control result as proof
  of the same report path while withholding candidate-only prerequisite facts.
- fixed: recorded that the final lab boot tree was restored to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 and
  hardwareTestLock was released before the proof was accepted.
- fixed: closed same-shaped prerequisite report visibility hardware retries.
  Repeating this candidate/control pair would not prove clock/reset, GPIO/PHY,
  MDIO, interrupt, DMA, descriptor, packet, or network ownership.
- deferred: actual clock/reset, GPIO32/PHY reset, MDIO/PHY,
  interrupt-completion, DMA, descriptor-ring, packet I/O, networking, sockets,
  SSH, Phase 12.2, and phase transition work remain future supervisor-planned
  tasks.
- not-an-issue: no hardwareTestLock was acquired because this closeout is a
  static checkpoint over already accepted hardware evidence.

No findings were removed.

## Accepted Boundary

The accepted frontier is prerequisite ownership report visibility/control
output only:

- candidate/control evidence joined selected-tree identity, expected TFTP fetch
  bytes, run-unique serial marker freshness, final pre-restore identity, and
  restore proof;
- candidate serial retained the accepted prerequisite report fields and
  classified rp1-ethernet-prereq-ownership-report-visible;
- paired control retained the same reporting/capture path while withholding
  candidate-only prerequisite facts and classifying
  no-ownership-no-ethernet-rp1-ethernet-prereq-control;
- lab boot state was restored and hardwareTestLock was released before the
  proof was accepted.

This accepts only that the already constructed prerequisite report is visible
on Pi 5 serial under the bounded proof conditions. It does not accept runtime
ownership of clocks/resets, GPIO32/PHY reset, MDIO/PHY, interrupts, DMA,
descriptor rings, or packets.

## Same-Shaped Retry Policy

Same-shaped prerequisite report visibility candidate/control hardware retries
are closed. A future Phase 12.1 task must supply different explicit scope and
acceptance criteria, such as a source-backed and implementation-backed
clock/reset, PHY/MDIO, interrupt, DMA, descriptor, or packet-path prerequisite
slice. This closeout does not select or create such a task.

## Rejected Claims And Retained Risks

Rejected claims:

- Ethernet driver readiness;
- broad Ethernet MMIO readiness;
- RP1 MMIO writes;
- clock/reset writes or ownership;
- GPIO32 ownership or PHY reset assertion/deassertion;
- MDIO transactions or PHY ownership;
- interrupt delivery, handler ownership, or completion;
- DMA, descriptor rings, channel ownership, or transfer completion;
- packet I/O, networking, sockets, SSH;
- Phase 12.2 work or phase transition.

Retained risks:

- Report visibility is not prerequisite ownership.
- The accepted observed-window MACB_MID identity remains context only for this
  report.
- Clock/reset and PHY/MDIO ownership remain unaccepted.
- DMA/cache/descriptors and interrupt completion remain unaccepted for
  Ethernet.
- The next Phase 12.1 step requires supervisor planning before any driver or
  packet behavior is implemented.

## Evidence

- Accepted proof task:
  tasks/2026-06-10-phase12-rp1-ethernet-prereq-ownership-pi5-proof.md.
- Proof classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-prereq-ownership-pi5-proof/classification.json.
- Proof evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-prereq-ownership-pi5-proof/evidence-map.json.
- Proof capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-prereq-ownership-pi5-proof/capture-summary.json.
- Closeout classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-prereq-ownership-proof-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-prereq-ownership-proof-closeout/evidence-map.json.
- Project docs:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap:
  docs/src/roadmap.md.

## Validation

- static inspection: reviewed accepted proof task record, proof
  classification/evidence JSON, capture summary, Phase 12 docs, roadmap, and
  git history.
- JSON checks: jq empty on proof and closeout classification/evidence JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles proof evidence without expanding acceptance to Ethernet
  driver readiness, packet I/O, live DMA, descriptor rings, interrupts,
  networking, sockets, SSH, Phase 12.2, or phase transition: satisfied.
- Checkpoint states whether same-shaped prerequisite hardware retries are
  closed, blocked, or require a different discriminator: satisfied; closed for
  this report visibility/control pair.
- NextAction selects one bounded follow-up only if mechanically objective;
  otherwise planningNeeded=true with a concrete blocker reason: satisfied;
  supervisor planning is required for the next explicit Phase 12.1 slice.
- Accepted checkpoint is committed before any follow-up starts: satisfied by
  the commit recorded in supervisor state after this task.

## Next Action

Supervisor planning required. The next Phase 12.1 task must be explicitly
scoped with acceptance criteria before any runtime prerequisite ownership,
Ethernet driver implementation, RP1 MMIO writes, DMA, descriptor rings,
interrupts, packet I/O, networking, sockets, SSH, Phase 12.2, or phase
transition work starts.
