# Phase 12 RP1 Ethernet Clock/Reset Prerequisite Closeout

Task id: phase12-rp1-ethernet-clock-reset-prereq-closeout-20260611

Status: accepted

Classification:
rp1-ethernet-clock-reset-prereq-frontier-closed-mdio-phy-id-selected

Evidence level: static inspection of accepted Phase 12 task records,
classification/evidence JSON, Phase 12 docs, roadmap, and git history. No
code implementation, Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, GPIO32 write/restore retry, event-clear retry,
PHY reset assertion/deassertion, MDIO transaction, Ethernet driver behavior,
DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or phase
transition was performed by this closeout.

## Goal

Reconcile the accepted RP1 Ethernet clock/reset prerequisite frontier and
select the next non-GPIO32 ownership prerequisite without expanding Ethernet
behavior.

## Scope Performed

- Consumed the accepted observed-window GEM MID identity proof closeout,
  prerequisite ownership report visibility closeout, CLK_ETH_TSU_CTRL
  idempotent write/restore proof closeout, CLK_ETH_CTRL idempotent
  write/restore proof closeout, GPIO32 write/restore blocked/no-write
  closeout, GPIO32 read-only event-state proof closeout, and GPIO32
  event-clear persistent/firmware-owned proof closeout.
- Reconciled the clock/reset boundary as two Ethernet-private clock-manager
  register proofs only: CLK_ETH_TSU_CTRL and CLK_ETH_CTRL idempotent
  write/readback/restore with paired no-clock-write controls.
- Preserved observed-window MACB_MID identity and prerequisite report
  visibility as read-only/report evidence, not runtime ownership evidence.
- Recorded that GPIO32 / ETH_RST_N ownership remains blocked by accepted
  blocked-no-write, event-state, and event-clear evidence, and that
  same-shaped GPIO32 write/restore and event-clear retries remain closed.
- Selected the already queued
  phase12-rp1-ethernet-mdio-phy-id-source-contract-20260611 as the next
  bounded Phase 12.1 task because it is source/docs contract work for a
  distinct non-GPIO32 prerequisite before packet I/O.

## Findings

- fixed: reconciled observed-window MACB_MID identity as accepted read-only
  identity context at 0x1c001000fc, raw 0x70109, idnum 0x7, rev 0x109.
- fixed: reconciled prerequisite ownership report visibility as accepted
  report/control evidence for rp1_eth clocks, interrupt, RGMII-ID phy1,
  GPIO32 reset metadata, PHY/MDIO policy, and DMA/descriptor dependency
  metadata without accepting runtime prerequisite ownership.
- fixed: reconciled CLK_ETH_TSU_CTRL at 0x1c00018134 as one Ethernet-private
  idempotent write/readback/restore proof with pre/post/restore raw
  0x10000800 and a paired no-clock-write control.
- fixed: reconciled CLK_ETH_CTRL at 0x1c00018064 as one Ethernet-private
  idempotent write/readback/restore proof with pre/post/restore raw
  0x10000800 and a paired no-clock-write control.
- fixed: recorded the accepted combined clock/reset boundary as exactly those
  two Ethernet-private clock control register proofs plus read-only/report
  prerequisite evidence.
- fixed: recorded that GPIO32 write/restore remains blocked/no-write and that
  same-shaped GPIO32 write/restore retries are closed for that
  candidate/control pair.
- fixed: recorded that GPIO32 event-state and event-clear evidence leave
  GPIO32 / ETH_RST_N ownership blocked by persistent or firmware-owned event
  state; same-shaped event-state, event-clear, and write/restore retries stay
  closed.
- fixed: selected the queued MDIO/PHY ID source contract as the next bounded
  non-GPIO32 prerequisite slice, limited to source/docs/evidence contract
  work with no runtime MDIO transaction.
- deferred: broad clock/reset ownership, shared-clock ownership,
  reset-controller ownership, GPIO32/PHY reset ownership, runtime MDIO/PHY
  ownership, Ethernet driver behavior, interrupt completion, DMA/descriptors,
  packet I/O, networking, sockets, SSH, Phase 12.2, and phase transition
  remain unaccepted.
- not-an-issue: no hardwareTestLock was acquired because this closeout is a
  static checkpoint over already accepted evidence.
- removed: no obsolete implementation, docs, or task evidence was removed.

## Accepted Boundary

The accepted frontier is a narrow prerequisite boundary:

- observed-window MACB_MID identity is accepted only as read-only identity
  context;
- prerequisite ownership reports are accepted only as report visibility and
  paired no-ownership/no-Ethernet control output;
- CLK_ETH_TSU_CTRL at 0x1c00018134 is accepted only as an idempotent
  write/readback/restore proof of the observed raw value with paired control;
- CLK_ETH_CTRL at 0x1c00018064 is accepted only as an idempotent
  write/readback/restore proof of the observed raw value with paired control;
- GPIO32 / ETH_RST_N ownership remains blocked by the accepted blocked-no-write
  proof and by event-state/event-clear evidence that kept event bits
  persistent or firmware-owned.

This accepts no broad clock/reset ownership, shared-clock ownership,
reset-controller ownership, GPIO32 or PHY reset ownership, runtime MDIO/PHY
ownership, Ethernet driver readiness, interrupt completion, DMA/descriptors,
packet I/O, networking, sockets, SSH, Phase 12.2, or phase transition.

## Same-Shaped Retry Policy

Same-shaped CLK_ETH_TSU_CTRL and CLK_ETH_CTRL idempotent write/restore retries
remain closed for their accepted candidate/control pairs. Same-shaped GPIO32
write/restore and event-clear retries remain closed after the blocked/no-write
and persistent/firmware-owned evidence. Any future GPIO32 follow-up needs a
qualitatively different supervisor-planned discriminator or ownership contract.

## Next Bounded Task Selection

The next mechanically objective task is
phase12-rp1-ethernet-mdio-phy-id-source-contract-20260611. It is selected only
as source/docs/evidence contract work for a distinct non-GPIO32 prerequisite:
the MDIO/PHY ID path required before packet I/O. The contract task must not
perform runtime MDIO transactions, assert or deassert PHY reset, retry GPIO32
event clear or write/restore, implement Ethernet behavior, program DMA or
descriptors, handle interrupts, perform packet I/O, add networking, sockets,
SSH, Phase 12.2, or create a phase transition.

## Rejected Claims And Retained Risks

Rejected claims:

- broad clock/reset ownership;
- shared-clock ownership;
- reset-controller ownership;
- GPIO32 ownership;
- PHY reset assertion or deassertion;
- GPIO32 write/restore retry or success;
- runtime MDIO/PHY ownership;
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

- The accepted clock proofs are idempotent raw-value restore proofs only.
- GPIO32 / ETH_RST_N ownership remains unproven and blocked by event-state and
  event-clear evidence.
- MDIO/PHY ID remains source-contract work until a later guard and serialized
  Pi 5 proof explicitly accept more.

## Evidence

- Observed-window proof closeout:
  tasks/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-proof-closeout.md.
- Prerequisite report proof closeout:
  tasks/2026-06-10-phase12-rp1-ethernet-prereq-ownership-proof-closeout.md.
- CLK_ETH_TSU_CTRL proof closeout:
  tasks/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-proof-closeout.md.
- CLK_ETH_CTRL proof closeout:
  tasks/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-proof-closeout.md.
- GPIO32 write/restore v2 proof closeout:
  tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-proof-closeout.md.
- GPIO32 event-state proof closeout:
  tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-proof-closeout.md.
- GPIO32 event-clear proof closeout:
  tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-proof-closeout.md.
- Closeout classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-clock-reset-prereq-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-clock-reset-prereq-closeout/evidence-map.json.
- Project docs:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap:
  docs/src/roadmap.md.

## Validation

- static inspection: accepted Phase 12 task records, classification/evidence
  JSON, Phase 12 docs, roadmap, and git history reviewed.
- JSON checks: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles accepted observed-window GEM identity, prerequisite
  report visibility, Ethernet-private clock write/restore proofs, GPIO32
  blocked/no-write proof, read-only event-state proof, and event-clear
  persistent/firmware-owned proof: satisfied.
- Accepted boundary does not expand beyond idempotent CLK_ETH_TSU_CTRL and
  CLK_ETH_CTRL write/readback/restore evidence plus read-only/report
  visibility evidence: satisfied.
- Same-shaped GPIO32 event-clear and GPIO32 write/restore retries remain
  explicitly closed: satisfied.
- NextAction selects
  phase12-rp1-ethernet-mdio-phy-id-source-contract-20260611 only as
  source/docs contract work: satisfied.
- Accepted checkpoint is committed before any follow-up starts: satisfied by
  the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-phy-id-source-contract-20260611 on the next worker
wake if hardwareTestLock remains unlocked and supervisorIntervention.active
remains false. Keep that task limited to source/docs/evidence contract work;
do not perform runtime MDIO transactions, assert/deassert PHY reset, retry
GPIO32 event clear or write/restore, implement Ethernet behavior, program DMA
or descriptors, handle interrupts, perform packet I/O, add networking, sockets,
SSH, Phase 12.2, or create a phase transition.
