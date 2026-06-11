# Phase 12 RP1 Ethernet GPIO32 Event-Clear Proof Closeout

Task id: phase12-rp1-ethernet-gpio32-event-clear-proof-closeout-20260611

Status: accepted

Classification:
rp1-ethernet-gpio32-event-clear-persistent-or-firmware-owned-frontier-closed

Evidence level: static inspection of accepted Pi 5 proof task record,
classification/evidence JSON, capture summary, project docs, roadmap, and git
history. No Pi 5 hardware run, boot archive publication, hardwareTestLock
acquisition, GPIO/RIO/pad/MMIO write, event clear, GPIO32 write/restore retry,
PHY reset assertion/deassertion, MDIO/PHY work, Ethernet driver behavior,
packet I/O, networking, sockets, SSH, Phase 12.2, or phase transition was
performed by this closeout.

## Goal

Close out the accepted GPIO32 event-clear Pi 5 proof without expanding
acceptance beyond event-clear discrimination and invariant preservation.

## Scope Performed

- Consumed the accepted Pi 5 event-clear proof from commit 8a26bbbe.
- Reconciled the candidate/control hardware evidence against the event-clear
  source contract, guard core, guard closeout, Phase 12 docs, and roadmap.
- Recorded the accepted hardware result as a persistent/firmware-owned event
  blocker: candidate pre-read STATUS 0x0abe3300, CTRL 0x85, RIO1 OUT/OE
  0x10, RIO1 IN 0x12, pad 0x56, event bits 0x0ab00000; then wrote only
  GPIO32 CTRL SET IRQRESET value 0x10000000; post-readback preserved
  CTRL/RIO/pad invariants but retained event bits 0x08800000.
- Preserved the paired no-GPIO/no-Ethernet control-rerun2 as proof of the
  same capture-chain-v4 reporting path while withholding GPIO32/RIO/pad/MMIO
  target construction and performing no event clear.
- Closed same-shaped event-clear hardware retries for this candidate/control
  pair because repeating the same source-backed IRQRESET write would not prove
  GPIO32 ownership, PHY reset ownership, or whether firmware reasserts or owns
  the latched event state.
- Requested supervisor planning for any future GPIO32 ownership or Ethernet
  follow-up because this closeout does not make a bounded ownership task
  mechanically objective.

## Findings

- fixed: reconciled the accepted candidate/control capture-chain-v4 evidence
  as decisive for the GPIO32 event-clear discriminator boundary.
- fixed: recorded that the candidate performed only the accepted GPIO32 CTRL
  SET IRQRESET write value 0x10000000 and preserved CTRL/RIO/pad/no-output
  invariants.
- fixed: recorded the accepted blocker: event bits persisted as 0x08800000
  after the guarded clear attempt, so event clearing did not prove ownership or
  source-state clearance.
- fixed: preserved control-rerun2 as the accepted paired control for the same
  capture/reporting path with no GPIO32/RIO/pad/MMIO target construction and
  no event clear.
- fixed: closed same-shaped event-clear hardware retries for this
  candidate/control pair.
- deferred: GPIO32 ownership, PHY reset ownership, GPIO32 write/restore retry,
  MDIO/PHY, Ethernet driver behavior, interrupt completion, DMA/descriptors,
  packet I/O, networking, sockets, SSH, Phase 12.2, and phase transition
  remain unaccepted.
- not-an-issue: no hardwareTestLock was acquired because this closeout is a
  static checkpoint over already accepted hardware evidence.
- removed: no obsolete implementation, docs, or task evidence was removed.

## Accepted Boundary

The accepted frontier is only GPIO32 event-clear discrimination with invariant
preservation:

- candidate/control evidence joined selected-tree identity, expected TFTP
  fetch bytes, run-unique serial marker freshness, final pre-restore identity,
  restore proof, and task-owned JSON;
- candidate pre-state matched the accepted source contract with STATUS
  0x0abe3300, CTRL 0x85, RIO1 OUT/OE 0x10, RIO1 IN 0x12, pad 0x56, and event
  bits 0x0ab00000;
- candidate performed only GPIO32 CTRL SET IRQRESET write value 0x10000000;
- candidate post-state preserved CTRL 0x85, RIO1 OUT/OE 0x10, RIO1 IN 0x12,
  and pad 0x56 while retaining event bits 0x08800000;
- paired control-rerun2 classified as
  no-gpio-no-ethernet-rp1-ethernet-gpio32-event-clear-control while retaining
  the same capture-chain-v4 reporting path;
- lab boot state was restored and hardwareTestLock was released before the
  proof was accepted.

This accepts only that the accepted event-clear attempt leaves GPIO32 event
bits persistent or firmware-owned while preserving no-reset/no-output
invariants. It does not accept GPIO32 ownership, PHY reset
assertion/deassertion, GPIO32 write/restore retry or success, MDIO/PHY
ownership, Ethernet driver behavior, interrupt completion, DMA/descriptors,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

## Same-Shaped Retry Policy

Same-shaped event-clear candidate/control hardware retries are closed for this
candidate/control pair. A future Phase 12.1 GPIO32 task needs supervisor
planning with a qualitatively different discriminator or explicit ownership
contract before any GPIO32 write/restore retry, PHY reset, MDIO/PHY, Ethernet
driver, packet I/O, networking, socket, SSH, Phase 12.2, or phase-transition
work starts.

## Rejected Claims And Retained Risks

Rejected claims:

- GPIO32 ownership;
- PHY reset assertion or deassertion;
- GPIO32 write/restore retry or success;
- MDIO/PHY ownership;
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

- Event bits remain persistent or firmware-owned after the accepted clear
  attempt.
- The proof does not decide whether firmware reasserts the event state, whether
  the event is harmless, or whether it can be ignored safely.
- GPIO32 / ETH_RST_N ownership remains unproven.
- The next follow-up requires supervisor planning before any ownership,
  Ethernet driver, or packet behavior is implemented.

## Evidence

- Accepted proof task:
  tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-pi5-proof.md.
- Proof classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-pi5-proof/classification.json.
- Proof evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-pi5-proof/evidence-map.json.
- Proof capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-pi5-proof/capture-summary.json.
- Closeout classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-proof-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-proof-closeout/evidence-map.json.
- Project docs:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap:
  docs/src/roadmap.md.

## Validation

- static inspection: accepted event-clear proof task record, proof
  classification/evidence JSON, capture summary, Phase 12 docs, roadmap, and
  git history reviewed.
- JSON checks: jq empty on proof and closeout classification/evidence JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles proof/blocker evidence without expanding acceptance
  beyond event-clear discrimination and invariant preservation: satisfied.
- Same-shaped event-clear retry policy is explicit: satisfied; closed for this
  candidate/control pair.
- NextAction selects one bounded follow-up only if mechanically objective;
  otherwise planningNeeded=true with a concrete blocker reason: satisfied;
  supervisor planning is required because the proof is persistent or
  firmware-owned and no GPIO32 ownership follow-up is mechanically objective.
- Accepted checkpoint is committed before any follow-up starts: satisfied by
  the commit recorded in supervisor state after this task.

## Next Action

Supervisor planning required. The next Phase 12.1 task must be explicitly
scoped with acceptance criteria before any GPIO32 ownership, GPIO32
write/restore retry, PHY reset assertion/deassertion, MDIO/PHY ownership,
Ethernet driver implementation, DMA/descriptors, interrupts, packet I/O,
networking, sockets, SSH, Phase 12.2, or phase transition work starts.
