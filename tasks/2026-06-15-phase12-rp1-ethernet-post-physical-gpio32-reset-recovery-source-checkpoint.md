# Phase 12 RP1 Ethernet Post-Physical GPIO32 Reset-Recovery Source Checkpoint

Task id: phase12-rp1-ethernet-post-physical-gpio32-reset-recovery-source-checkpoint-20260615

Status: accepted

Classification:
post-physical-gpio32-reset-recovery-source-checkpoint-blocked-persistent-event-state

Evidence level: static/source/task evidence inspection, JSON evidence
validation, diff checks, and docs build. No Pi 5 hardware run, boot archive
publication, lab mutation, hardwareTestLock acquisition, power-cycle,
GPIO/RIO/pad/MMIO write, event clear, PHY reset assertion/deassertion, BMCR
write, PHY configuration write, MACB configuration write, packet I/O,
networking, SSH, Phase 12.2, or phase transition was performed.

## Goal

Reconcile the accepted post-physical phy-not-ready status frontier with prior
GPIO32 / ETH_RST_N blockers before selecting any reset-recovery proof.

## Scope Performed

- Inspected the accepted v2 post-physical link-status closeout and task-owned
  classification/evidence map.
- Inspected the accepted GPIO32 write/restore v2 blocked/no-write closeout,
  read-only event-state proof closeout, event-clear proof closeout, and their
  source contracts.
- Reconciled retained RP1 pinctrl source facts for GPIO STATUS event bits and
  IRQRESET event-clear semantics against the accepted hardware evidence.
- Determined whether a future GPIO32 reset-recovery proof can be selected from
  the current evidence boundary.
- Updated Phase 12 docs and roadmap with the resulting blocker.

## Findings

- fixed: the accepted v2 post-physical status frontier remains
  post-physical-link-status-phy-not-ready with BMCR 0x1000, BMSR
  0x7949/0x7949, ANAR 0x01e1, ANLPAR 0x0000, MACB_NSR 0x00000006, BMSR link
  false, autoneg complete false, ANLPAR nonzero false, and MACB_NSR_LINK
  false.
- fixed: the GPIO32 write/restore v2 proof is still a no-write frontier: it
  observed STATUS 0x0abe3300, CTRL 0x85, RIO1 OUT/OE 0x10, RIO1 IN 0x12, pad
  0x56, and event bits 0x0ab00000, then stopped with writes-performed=false.
- fixed: retained RP1 pinctrl source backs STATUS bits 20-27 as raw/filtered
  falling/rising/low/high event bits and IRQRESET as the source-backed event
  clear mechanism, but it does not prove those bits are harmless for ETH_RST_N
  ownership or safe to ignore before a reset-line write/restore proof.
- fixed: the accepted event-clear proof performed only the guarded GPIO32 CTRL
  SET IRQRESET write value 0x10000000 and preserved CTRL/RIO/pad invariants,
  but event bits persisted as 0x08800000, leaving the state persistent or
  firmware-owned.
- blocked: a future GPIO32 reset-recovery proof is not source/evidence
  justified from the current boundary because the required pre-write event
  state remains nonzero after the only accepted clear attempt.
- deferred: a future task may revisit GPIO32 only if it brings a distinct
  source-backed ownership, firmware/event-state, reset-controller, or PHY
  power/strap discriminator with explicit preconditions and evidence gates.
- not-an-issue: the accepted physical Ethernet link precondition remains
  accepted; this checkpoint does not ask Matthew to reconfirm cabling or link
  partner setup.
- removed: no source, helper, task, or evidence files were removed.

## Reconciliation

The accepted v2 post-physical proof established a capture-fresh PHY/MAC status
sample after the confirmed physical-link precondition. The result is still
not ready: the PHY reports BMCR 0x1000, BMSR 0x7949 on both samples, no
autoneg completion, no partner advertisement in ANLPAR, and the MACB_NSR link
bit is clear. That makes reset or recovery planning relevant, but it does not
by itself authorize GPIO32 / ETH_RST_N action.

The existing GPIO32 source contract selected a conservative write/restore
sequence only when pre-write state is safe: no sentinel reads, GPIO function
and override fields are compatible with raw RIO OUT/OE control, pad output is
not disabled, a restore baseline is complete, and event/interrupt state does
not turn the task into an event/interrupt ownership proof. The v2 hardware
attempt hit that event-state guard before any GPIO/RIO/pad write and reported
writes-performed=false.

The follow-up event-state and event-clear tasks made the blocker more precise,
not weaker. Source inspection maps the observed bits to GPIO STATUS event
fields, and the guarded IRQRESET proof shows a narrow clear operation can be
attempted while preserving CTRL/RIO/pad invariants. But the accepted hardware
result retained event bits after the clear attempt. The current evidence
therefore supports only a persistent-or-firmware-owned event-state blocker,
not a safe-ignore rule or reset-line ownership.

## Decision

No GPIO32 reset-recovery Pi 5 proof is selected by this checkpoint.

Reason:

- the current write/restore source contract requires nonzero GPIO32 event
  state to block before writes;
- the accepted event-clear proof did not clear the event state to the
  write/restore precondition;
- no retained Raspberry Pi/RP1 source excerpt proves that the remaining
  source-backed event bits are harmless for ETH_RST_N ownership or can be
  ignored safely;
- selecting a reset-recovery proof now would repeat the same blocked
  GPIO32/RIO/pad write path with weaker preconditions.

This checkpoint sets planningNeeded=true for supervisor selection of a
distinct bounded follow-up or an explicit pause. A safe future plan could
target source-backed firmware/event-state ownership, reset-controller or PHY
power/strap evidence, or a different autoneg/PHY recovery discriminator, but
that task must be explicitly queued with its own acceptance criteria.

## Rejected Claims And Retained Risks

Rejected claims:

- GPIO32 ownership;
- ETH_RST_N reset assertion or deassertion;
- GPIO32 write/restore retry or success;
- safe-ignore treatment for persistent GPIO32 event bits;
- PHY reset ownership;
- PHY configuration writes;
- BMCR writes;
- MACB configuration writes;
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

- PHY power, reset, strap, or partner-autoneg state may still explain the
  accepted phy-not-ready status.
- GPIO32 event bits may be stale, level-reasserted, firmware-owned, or tied to
  another ownership path; the current source/evidence set does not decide it.
- The GPIO32 reset path remains blocked until a future planned task changes
  the evidence boundary without weakening the existing no-write guard.
- Packet I/O and network-stack work remain blocked until link and lower-level
  prerequisites are separately accepted.

## Evidence

- V2 post-physical closeout task:
  tasks/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-closeout.md.
- V2 post-physical closeout classification:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-link-status-v2-closeout/classification.json.
- GPIO32 write/restore v2 closeout:
  tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-proof-closeout.md.
- GPIO32 event-state closeout:
  tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-proof-closeout.md.
- GPIO32 event-clear closeout:
  tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-proof-closeout.md.
- GPIO32 write/restore source contract:
  tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-source-contract.md.
- GPIO32 event-state source contract:
  tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-source-contract.md.
- GPIO32 event-clear source contract:
  tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-source-contract.md.
- Checkpoint classification:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-gpio32-reset-recovery-source-checkpoint/classification.json.
- Checkpoint evidence map:
  tasks/evidence/2026-06-15-phase12-rp1-ethernet-post-physical-gpio32-reset-recovery-source-checkpoint/evidence-map.json.

## Validation

- static/source/task evidence inspection: accepted v2 post-physical closeout,
  GPIO32 write/restore v2 closeout, GPIO32 event-state closeout, GPIO32
  event-clear closeout, retained source contracts, Phase 12 docs, roadmap, and
  git history inspected.
- JSON validation: jq empty on task-owned classification/evidence-map JSON and
  referenced input classification/evidence JSON.
- diff check: git diff --check.
- docs validation: mdbook build because docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Findings list with dispositions: satisfied.
- Accepted post-physical phy-not-ready evidence reconciled with GPIO32
  write/restore, event-state, and event-clear blockers: satisfied.
- Future GPIO32 reset-recovery proof is not selected because source/evidence
  cannot justify it safely: satisfied.
- Precise blocker and planningNeeded reason recorded: satisfied.
- Packet I/O, networking, SSH, Phase 12.2, and phase transition remain
  explicitly rejected: satisfied.
- Accepted checkpoint committed before any follow-up starts: satisfied once
  this task is committed.

## Next Action

Set planningNeeded=true. Supervisor must select a distinct source-grounded
Phase 12.1 follow-up or an explicit pause. Do not start GPIO32 reset-recovery,
event-clear retry, BMCR write, PHY configuration, packet I/O, networking, SSH,
Phase 12.2, or a phase transition without a new explicit queued task.
