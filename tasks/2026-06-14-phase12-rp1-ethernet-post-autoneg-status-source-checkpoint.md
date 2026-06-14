# Phase 12 RP1 Ethernet Post-Autoneg Status Source Checkpoint

Task id: phase12-rp1-ethernet-post-autoneg-status-source-checkpoint-20260614

Status: accepted

Classification: rp1-ethernet-post-autoneg-status-source-checkpoint-no-safe-readonly-follow-up

Evidence level: static/task evidence inspection and task-owned JSON evidence.
No runtime implementation, hardware action, hardwareTestLock acquisition, lab
mutation, boot archive publication, PHY configuration, GPIO32/PHY reset action,
MACB write, packet I/O, networking, SSH, Phase 12.2, or phase transition was
performed.

## Goal

Reconcile the accepted guarded PHY1 BMCR autoneg-restart v2 proof and decide
whether one objective future read-only post-autoneg status proof remains safe,
or whether supervisor planning is required before the next Phase 12.1 task.

## Scope Performed

- Inspected the accepted v2 proof, v2 closeout, classification JSON, evidence
  map, Phase 12 docs, roadmap, and prior source contracts for PHY1 status,
  BMSR double-sample link readiness, MACB_NSR_LINK, and PHY1 autoneg restart.
- Compared the v2 post-restart runtime fields against prior accepted passive
  PHY-side and MAC-side status evidence.
- Preserved rejected claims for GPIO32/PHY reset ownership, packet I/O,
  networking, SSH, Phase 12.2, and phase transition.
- Recorded that no new mechanically safe read-only post-autoneg status proof is
  selected from this checkpoint.

## Non-Goals

No hardware run, no boot publication, no lab mutation, no source/runtime code
change, no helper rewrite, no BMCR write, no PHY configuration write, no
GPIO32/PHY reset action, no MACB write, no packet I/O, no networking, no SSH,
no Phase 12.2 work, and no phase transition.

## Findings

- fixed: the accepted v2 closeout dependency is satisfied by commit
  1852f57b8da7fad6e7e20e7429cb0f6f912fa205 with selected_next_task equal to
  this checkpoint.
- fixed: the accepted v2 proof already includes the bounded post-write status
  source set available under the current contract: post-BMCR 0x1000,
  post-BMSR 0x7949/0x7949, post-ANAR 0x01e1, post-ANLPAR 0x0000, and passive
  MACB_NSR raw 0x00000006 / NSR_LINK=false.
- fixed: prior read-only PHY1 status, BMSR double-sample, and MACB_NSR_LINK
  proofs already established the same link-not-ready PHY/MAC frontier before
  the restart attempt; the v2 proof confirms the guarded restart did not move
  those sampled status sources to link-ready.
- fixed: another immediate read-only proof over BMCR, BMSR, ANAR, ANLPAR, or
  MACB_NSR would be same-shaped under the accepted contracts unless a future
  supervisor task adds a distinct source-backed discriminator, timing model, or
  external precondition.
- deferred: physical carrier, partner autonegotiation, reset/strap state, PHY
  power, operator cabling, and any future delayed/settled status strategy
  require supervisor planning with explicit acceptance gates before worker
  execution.
- not-an-issue: no hardwareTestLock was acquired because this checkpoint is
  static evidence and task-record work only.
- removed: no source, helper, task, or evidence files were removed.

## Reconciliation

The v2 proof accepted a capture-fresh candidate/control pair. The candidate
reached the guarded corrected-target PHY1 BMCR discriminator with NCR.MPE set,
BMCR isolate clear, and exactly one BMCR write intent value 0x1200. Its
post-write readback was BMCR 0x1000, BMSR 0x7949 on both samples, ANAR 0x01e1,
ANLPAR 0x0000, and passive MACB_NSR_LINK=false. The paired control constructed
no MDIO/MAN/MACB target and performed no volatile Ethernet access.

Those post-write values are already the complete bounded status set named by
the accepted autoneg-restart source contract and guard core. They also agree
with the earlier accepted PHY1 and MAC-side link-not-ready evidence. Repeating
the same read-only status sources would not create a new discriminator from
the current evidence frontier; it would only re-sample the same BMCR/BMSR/
ANAR/ANLPAR/MACB_NSR sources without a planned new timing, source contract, or
precondition.

The checkpoint therefore selects no future read-only status proof. Supervisor
planning is required before any next task so the program can choose a genuinely
objective discriminator, an operator/physical precondition path, or a pause in
this Phase 12.1 hardware branch.

## Rejected Follow-Ups

- Same-shaped immediate BMCR/BMSR/ANAR/ANLPAR status read: rejected because
  the accepted v2 proof already retained those post-write values.
- Same-shaped passive MACB_NSR_LINK read: rejected because v2 already retained
  passive MACB_NSR raw 0x00000006 / NSR_LINK=false after the restart attempt,
  and the earlier MACB_NSR proof already accepted the same link-clear
  comparator.
- Fresh BMCR autoneg-restart retry: rejected from this checkpoint because it
  would be another PHY configuration write, not a read-only status proof.
- Delayed/settled post-autoneg status proof: deferred to supervisor planning;
  it would need an explicit source-backed timing model, accepted operations,
  evidence gates, and retained risks before worker promotion.
- GPIO32/PHY reset path: deferred because prior GPIO32 write/restore and
  event-clear evidence still block reset ownership.
- Physical-link/operator diagnosis: deferred to supervisor planning; this
  checkpoint does not itself prove cabling, switch partner state, PHY power, or
  reset/strap cause.
- Packet I/O/networking/SSH: rejected; no link, DMA, descriptor, interrupt,
  socket, userspace-networking, or Phase 12.2 boundary is accepted.

## Evidence

- V2 closeout task:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-closeout.md.
- V2 closeout classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-closeout/classification.json.
- V2 closeout evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-closeout/evidence-map.json.
- V2 proof classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-v2-after-capture-recovery-pi5-proof/classification.json.
- Autoneg restart source contract:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-source-contract.md.
- Autoneg restart guard core:
  tasks/2026-06-14-phase12-rp1-ethernet-phy1-autoneg-restart-guard-core.md.
- Prior PHY1 status diagnostic classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-status-diagnostic-pi5-proof/classification.json.
- Prior BMSR double-sample proof classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-phy1-bmsr-double-sample-link-readiness-pi5-proof/classification.json.
- Prior MACB_NSR_LINK proof classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-macb-nsr-link-readonly-pi5-proof/classification.json.
- Checkpoint classification:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-post-autoneg-status-source-checkpoint/classification.json.
- Checkpoint evidence map:
  tasks/evidence/2026-06-14-phase12-rp1-ethernet-post-autoneg-status-source-checkpoint/evidence-map.json.

## Validation

- static/task evidence inspection: accepted v2 closeout, v2 proof
  classification/evidence map, source contract, guard core, prior PHY1 status,
  BMSR double-sample, MACB_NSR_LINK classifications, Phase 12 docs, roadmap,
  and git history inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Checkpoint selects one exact future read-only status proof or records why no
  objective post-autoneg status follow-up is safe: satisfied; no safe
  same-frontier read-only follow-up is selected.
- Checkpoint dependencies cite accepted v2 closeout classification and commit:
  satisfied.
- Rejected claims for GPIO32/PHY reset ownership, packet I/O, networking, SSH,
  Phase 12.2, and phase transition remain explicit: satisfied.

## Next Action

Set planningNeeded=true for supervisor selection of the next bounded Phase 12.1
task. Do not start fresh BMCR writes, delayed status hardware runs, GPIO32/PHY
reset work, packet I/O, networking, SSH, Phase 12.2, or a phase transition
from this checkpoint without an explicit queued task.
