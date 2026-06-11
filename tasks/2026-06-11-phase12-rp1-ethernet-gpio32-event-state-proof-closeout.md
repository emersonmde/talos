# Phase 12 RP1 Ethernet GPIO32 Event-State Proof Closeout

Task id: phase12-rp1-ethernet-gpio32-event-state-proof-closeout-20260611

Status: accepted

Classification:
rp1-ethernet-gpio32-event-state-proof-blocked-event-state-frontier-closed

Evidence level: static inspection of accepted Pi 5 proof task record,
classification/evidence JSON, capture summary, project docs, roadmap, and git
history. No Pi 5 hardware run, archive publication, hardwareTestLock
acquisition, event clear, GPIO/RIO/pad/MMIO write, GPIO32 write/restore retry,
MDIO/PHY work, Ethernet driver behavior, packet I/O, networking, sockets, SSH,
Phase 12.2, or phase transition was performed by this closeout.

## Goal

Close out the accepted read-only GPIO32 event-state Pi 5 proof without
expanding acceptance beyond event-state discrimination.

## Scope Performed

- Consumed the accepted read-only Pi 5 proof from commit 832f0153.
- Reconciled candidate/control hardware evidence against the source contract,
  local/static discriminator core, static closeout, Phase 12 docs, and
  roadmap.
- Recorded the accepted hardware result as a read-only blocked-event-state
  discriminator: GPIO32 STATUS 0x0abe3300, CTRL 0x85, RIO1 OUT/OE 0x10,
  RIO1 IN 0x12, pad 0x56, and source-backed event bits 0x0ab00000 with
  writes-performed=false and event-clear-performed=false.
- Preserved the paired no-GPIO/no-Ethernet control as proof of the same
  capture path while withholding GPIO32/RIO/pad/MMIO target facts.
- Closed same-shaped event-state hardware retries for this candidate/control
  pair because repeating the read-only report would not resolve whether event
  bits are stale, clearable, firmware-owned, harmless, or safe to ignore.
- Requested supervisor planning for the next explicit Phase 12.1 slice because
  this closeout does not own event clearing, GPIO32 ownership, PHY reset,
  write/restore retry authority, MDIO/PHY ownership, driver readiness, packet
  I/O, networking, SSH, Phase 12.2, or a phase transition.

## Findings

- fixed: reconciled the accepted candidate/control capture-chain-v4 evidence
  as decisive for the read-only GPIO32 event-state discriminator boundary.
- fixed: recorded the candidate blocked-event-state result with source-backed
  STATUS bits 20-27 and no GPIO/RIO/pad/MMIO writes or event clear.
- fixed: preserved the paired no-GPIO/no-Ethernet control result as proof of
  the capture/reporting path without constructing GPIO32/RIO/pad/MMIO targets.
- fixed: closed same-shaped read-only event-state hardware retries for this
  candidate/control pair.
- deferred: source-backed interpretation or clearance of the event bits remains
  a future supervisor-planned discriminator; this closeout does not authorize
  event clearing or a GPIO32 write/restore retry.
- deferred: GPIO32 ownership, PHY reset assertion/deassertion, MDIO/PHY
  ownership, Ethernet driver readiness, interrupts, DMA/descriptors, packet
  I/O, networking, sockets, SSH, Phase 12.2, and phase transition remain
  unaccepted.
- not-an-issue: no hardwareTestLock was acquired because this closeout is a
  static checkpoint over already accepted hardware evidence.
- removed: no obsolete implementation, docs, or task evidence was removed.

## Accepted Boundary

The accepted frontier is only a read-only GPIO32 event-state discriminator:

- candidate/control evidence joined selected-tree identity, archive digest,
  run-unique serial marker freshness, stable TFTP delta, final pre-restore
  identity, and restore proof;
- candidate reported GPIO32 STATUS 0x0abe3300, CTRL 0x85, RIO1 OUT/OE 0x10,
  RIO1 IN 0x12, pad 0x56, event bits 0x0ab00000, and source decoding
  source-backed-bits-20-27;
- candidate classified as
  rp1-ethernet-gpio32-event-state-blocked-event-state with
  writes-performed=false and event-clear-performed=false;
- paired control classified as
  no-gpio-no-ethernet-rp1-ethernet-gpio32-event-state-control while retaining
  the same capture-chain-v4 reporting path;
- lab boot state was restored and hardwareTestLock was released before the
  proof was accepted.

This accepts only that source-backed GPIO32 event-state bits are visible in the
accepted read-only proof and block the prior write/restore precondition. It
does not accept event clearing, GPIO32 ownership, PHY reset, GPIO32
write/restore retry or success, MDIO/PHY ownership, Ethernet driver behavior,
interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2,
or a phase transition.

## Same-Shaped Retry Policy

Same-shaped read-only event-state candidate/control hardware retries are
closed. A future Phase 12.1 task must be explicitly planned with a
qualitatively different discriminator or source-backed clearance/ownership
contract before any event clear, GPIO/RIO/pad/MMIO write, PHY reset, or
GPIO32 write/restore retry is attempted.

## Rejected Claims And Retained Risks

Rejected claims:

- event clearing authority;
- GPIO/RIO/pad/MMIO writes;
- GPIO32 ownership or PHY reset assertion/deassertion;
- GPIO32 write/restore retry or success;
- MDIO/PHY ownership;
- Ethernet driver readiness;
- interrupt, DMA, descriptor, packet, socket, networking, or SSH readiness;
- Phase 12.2 work or phase transition.

Retained risks:

- Event-bit semantics are source-unresolved: stale, clearable, firmware-owned,
  harmless, and safe-to-ignore interpretations remain unaccepted.
- The read-only proof does not decide a safe event-clear sequence.
- The GPIO32 PHY reset path remains blocked before writes.
- The next Phase 12.1 step requires supervisor planning before any driver or
  packet behavior is implemented.

## Evidence

- Accepted proof task:
  tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-readonly-pi5-proof.md.
- Proof classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-readonly-pi5-proof/classification.json.
- Proof evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-readonly-pi5-proof/evidence-map.json.
- Proof capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-readonly-pi5-proof/capture-summary.json.
- Closeout classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-proof-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-proof-closeout/evidence-map.json.
- Project docs:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap:
  docs/src/roadmap.md.

## Validation

- static inspection: accepted proof task record, proof classification/evidence
  JSON, capture summary, Phase 12 docs, roadmap, and git history reviewed.
- JSON checks: jq empty on proof and closeout classification/evidence JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles proof/blocker evidence without expanding acceptance
  beyond read-only GPIO32 event-state discrimination: satisfied.
- Same-shaped retry policy is explicit: satisfied; closed for this
  candidate/control pair.
- NextAction selects one bounded follow-up only if mechanically objective;
  otherwise planningNeeded=true with a concrete blocker reason: satisfied;
  supervisor planning is required for a qualitatively different Phase 12.1
  event-state/source-clearance or ownership discriminator.
- Accepted checkpoint is committed before any follow-up starts: satisfied by
  the commit recorded in supervisor state after this task.

## Next Action

Supervisor planning required. The next Phase 12.1 task must be explicitly
scoped with acceptance criteria before any event clearing, GPIO/RIO/pad/MMIO
write, GPIO32 write/restore retry, PHY reset assertion/deassertion, MDIO/PHY
ownership, Ethernet driver implementation, DMA/descriptors, interrupts,
packet I/O, networking, sockets, SSH, Phase 12.2, or phase transition work
starts.
