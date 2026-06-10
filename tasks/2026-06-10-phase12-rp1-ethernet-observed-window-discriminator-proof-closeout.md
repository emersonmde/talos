# Phase 12 RP1 Ethernet Observed Window Discriminator Proof Closeout

Task: phase12-rp1-ethernet-observed-window-discriminator-proof-closeout-20260610

Status: accepted

Classification: rp1-ethernet-observed-window-discriminator-proof-frontier-closed

Evidence level: static inspection of accepted Pi 5 proof task record,
classification/evidence JSON, capture summaries, Phase 12 docs, roadmap, and
git history. No Pi 5 hardware run was performed by this closeout.

## Goal

Close out the observed-window discriminator proof and choose the next Phase
12.1 boundary without accepting Ethernet readiness by implication.

## Scope

- Consumed the accepted serialized Pi 5 proof from commit
  205cdeff97a3ec17f5f4efa8eb4ed53498313299.
- Reconciled the observed-window MACB_MID visible-read evidence against the
  prior observed-window contract, local/static core, and proof closeout.
- Closed same-shaped observed-window discriminator hardware retries for this
  candidate/control pair.
- Updated Phase 12 documentation and roadmap status to record the accepted
  read-only identity frontier and retained boundaries.
- Requested supervisor planning for the next explicit Phase 12.1 slice because
  this closeout does not own new source contracts, driver design, RP1 writes,
  DMA, descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase
  12.2, or a phase transition.

## Findings

- fixed: reconciled the candidate/control capture-chain-v4 proof as decisive
  for the accepted observed-window candidate/control pair.
- fixed: recorded the accepted read-only identity result: SYSINFO_CHIP_ID at
  0x1c00000000 returned 0x20001927 and observed-window MACB_MID at
  0x1c001000fc returned raw 0x70109, idnum 0x7, rev 0x109.
- fixed: preserved the paired no-MMIO/no-Ethernet control result as proof of
  the reporting path without constructing MMIO targets.
- fixed: closed same-shaped observed-window hardware retries. Repeating this
  candidate/control pair would not create a new boundary.
- deferred: Ethernet driver readiness, broad Ethernet MMIO readiness,
  source-backed clock/reset/PHY/MDIO ownership, RP1 MMIO writes, DMA,
  descriptor rings, interrupts, packet I/O, networking, sockets, SSH, Phase
  12.2, and phase transition remain future supervisor-planned work.
- not-an-issue: no hardwareTestLock was acquired because this closeout is a
  static checkpoint over already accepted hardware evidence.

No findings were removed.

## Accepted Boundary

The accepted frontier is a read-only observed-window GEM MID identity
discriminator:

- candidate/control evidence joined selected-tree identity, expected TFTP fetch
  bytes, run-unique serial marker freshness, final pre-restore identity, and
  restore proof;
- candidate SYSINFO_CHIP_ID positive control at 0x1c00000000 returned
  0x20001927;
- candidate observed-window MACB_MID at 0x1c001000fc returned raw 0x70109,
  idnum 0x7, rev 0x109;
- paired control retained the same reporting/capture path while constructing no
  MMIO targets;
- lab boot state was restored and hardwareTestLock was released before the
  proof was accepted.

This accepts only that the observed-window MACB_MID read is visible under the
bounded proof conditions. It does not accept broad Ethernet MMIO readiness or
driver readiness.

## Same-Shaped Retry Policy

Same-shaped observed-window candidate/control hardware retries are closed. A
future Phase 12.1 task must supply different explicit scope and acceptance
criteria, such as a source-backed ownership contract for clock/reset/PHY/MDIO
or another supervisor-planned prerequisite. This closeout does not select or
create such a task.

## Rejected Claims And Retained Risks

Rejected claims:

- Ethernet driver readiness;
- broad Ethernet MMIO readiness;
- RP1 MMIO writes;
- DMA and descriptor rings;
- interrupt, transfer, or device completion;
- clock/reset ownership;
- PHY reset or MDIO ownership;
- packet I/O, networking, sockets, SSH;
- Phase 12.2 work or phase transition.

Retained risks:

- MACB_MID visibility is only read-only identity evidence, not a packet path.
- Clock/reset and PHY/MDIO ownership remain unaccepted.
- DMA/cache/descriptors and interrupt completion remain unaccepted for
  Ethernet.
- The next Phase 12.1 step requires supervisor planning before any driver or
  packet behavior is implemented.

## Evidence

- Accepted proof task:
  tasks/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-pi5-proof.md.
- Proof classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-pi5-proof/classification.json.
- Proof evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-pi5-proof/evidence-map.json.
- Proof capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-pi5-proof/capture-summary.json.
- Closeout classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-proof-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-proof-closeout/evidence-map.json.
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
- Checkpoint states whether same-shaped observed-window hardware retries are
  closed, blocked, or require a different discriminator: satisfied; closed for
  this candidate/control pair.
- NextAction selects one bounded follow-up only if mechanically objective;
  otherwise planningNeeded=true with a concrete blocker reason: satisfied;
  supervisor planning is required for the next explicit Phase 12.1 slice.
- Accepted checkpoint is committed before any follow-up starts: satisfied by
  the commit recorded in supervisor state after this task.

## Next Action

Supervisor planning required. The next Phase 12.1 task must be explicitly
scoped with acceptance criteria before any Ethernet driver implementation,
RP1 MMIO writes, DMA, descriptor rings, interrupts, packet I/O, networking,
sockets, SSH, Phase 12.2, or phase transition work starts.
