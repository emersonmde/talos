# Phase 12 RP1 Ethernet GEM MID Blocker Reconciliation Closeout

Task: phase12-rp1-ethernet-gem-mid-blocker-reconciliation-closeout-20260610

Status: accepted

Classification: rp1-ethernet-gem-mid-blocker-reconciliation-closeout-accepted

Evidence level: static inspection of the accepted GEM MID blocker
reconciliation task record, classification JSON, evidence map, project docs,
and git history.

## Goal

Close the accepted GEM MID blocker reconciliation and select or block the next
mechanically justified discriminator without running hardware or drifting into
Ethernet implementation work.

## Scope

- Consumed accepted
  phase12-rp1-ethernet-gem-mid-blocker-reconciliation-20260610 evidence and
  commit.
- Reconciled the accepted classification, retained risks, rejected claims, and
  selected follow-up.
- Selected exactly one follow-up discriminator core with explicit local/static
  report shape.
- Preserved non-goals against Pi 5 hardware, Ethernet driver behavior, packet
  I/O, DMA, interrupts, networking, sockets, SSH, Phase 12.2 work, and phase
  transition claims.
- Recorded findings with disposition.

## Non-Goals

No new source inventory beyond the accepted reconciliation evidence, Pi 5
hardware run, boot archive publication, hardwareTestLock acquisition, Ethernet
driver implementation, packet I/O, DMA, descriptor rings, interrupt delivery,
clock/reset writes, PHY reset, networking, sockets, SSH, Phase 12.2 work, or
phase transition.

## Reconciliation

The accepted reconciliation classifies the GEM MID result as
`rp1-ethernet-gem-mid-retained-0x1f-window-sentinel`. It preserves the source
translation for `MACB_MID` at `0x1f001000fc`, but treats the accepted
`raw=0xdeaddead` Pi 5 result as the same qualitative boundary as prior
translated `0x1f` RP1 aperture reads, not as live GEM visibility.

The accepted evidence also keeps observed-aperture RP1 reads qualitatively
separate from the translated `0x1f` sentinel. Phase 11 accepted observed
`SYSINFO_CHIP_ID` at `0x1c00000000` returning `0x20001927`, selected observed
clock/reset fields that were not `0xdeaddead`, and visible observed
GPIO/RIO/pad/source fields. Those boundaries make a same-run observed
positive-control plus translated GEM MID read the smallest changed
discriminator.

No contradiction was found in the accepted reconciliation that would supersede
the queued discriminator core or require Matthew/supervisor input. The next
hardware action remains blocked until a later explicitly gated Pi 5 proof.

## Selected Follow-Up

Selected:
`phase12-rp1-ethernet-gem-mid-decode-discriminator-core-20260610`.

Required discriminator shape for the local/static core:

- Candidate report: one same-run read-only observed RP1 positive-control load
  of `SYSINFO_CHIP_ID` at `0x1c00000000` plus one read-only `MACB_MID` load at
  `0x1f001000fc`.
- Expected candidate classifications:
  `observed-rp1-positive-control-gem-mid-0x1f-window-sentinel`,
  `observed-rp1-positive-control-and-gem-mid-visible`,
  `observed-rp1-positive-control-sentinel`, and `staging/build-blocker`.
- Paired control: same reporting path with no observed RP1 MMIO target and no
  Ethernet MMIO target, carrying explicit no-MMIO/no-Ethernet control
  classification.
- Rejections: Ethernet driver readiness, broad Ethernet MMIO readiness,
  RP1 MMIO/DMA programming, descriptor rings, interrupt completion,
  clock/reset ownership, PHY reset ownership, packet I/O, networking, sockets,
  SSH, Phase 12.2, and phase transition claims.

This selection is local/static only. It does not authorize a hardware run or
any Ethernet behavior.

## Findings

- fixed: closed the accepted reconciliation as a retained `0x1f` translated
  RP1-window sentinel boundary rather than a live GEM identity read.
- fixed: selected the already queued local/static decode discriminator core as
  the next mechanically justified task.
- fixed: made the discriminator shape explicit: observed `0x1c` RP1
  `SYSINFO_CHIP_ID` positive control plus translated `0x1f001000fc`
  `MACB_MID`, with a paired no-MMIO/no-Ethernet control.
- deferred: Pi 5 proof, bridge/window enablement source, Ethernet-local
  clock/reset dependency, live GEM visibility, broad Ethernet MMIO readiness,
  PHY reset/MDIO ownership, packet I/O, networking, sockets, SSH, Phase 12.2,
  and phase transition work.
- not-an-issue: no hardwareTestLock is acquired because this closeout is a
  source/local checkpoint only.

No findings were removed.

## Rejected Claims And Retained Risks

This closeout does not accept live GEM visibility, broad Ethernet MMIO
readiness, Ethernet driver readiness, RP1 MMIO/DMA programming, descriptor
rings, transfer completion, interrupt completion, clock/reset ownership, PHY
reset ownership, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase
transition.

Retained risks:

- the source of the translated `0x1f` RP1-window sentinel remains unproven;
- PCIe/RP1 bridge or window enablement remains unaccepted;
- Ethernet clock/reset dependency remains unaccepted;
- no live GEM identity, broad Ethernet MMIO readiness, PHY reset ownership, or
  packet I/O has been accepted.

## Evidence

- Accepted reconciliation task record:
  `tasks/2026-06-10-phase12-rp1-ethernet-gem-mid-blocker-reconciliation.md`.
- Accepted reconciliation classification:
  `tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-blocker-reconciliation/classification.json`.
- Accepted reconciliation evidence map:
  `tasks/evidence/2026-06-10-phase12-rp1-ethernet-gem-mid-blocker-reconciliation/evidence-map.json`.
- Phase 12 project doc:
  `docs/src/project/phase12-networking-ssh.md`.
- Current git history:
  `672321e7 Reconcile GEM MID window sentinel`.

## Validation

- static inspection: reviewed the accepted reconciliation task record,
  classification JSON, evidence map, Phase 12 project doc, and recent git
  history.
- diff checks: `git diff --check` and `git diff --cached --check` passed.
- documentation build: not run because no `docs/src` files were touched.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Closeout states whether
  `phase12-rp1-ethernet-gem-mid-decode-discriminator-core-20260610` is
  selected, blocked, or superseded: satisfied by selected.
- If selected, nextAction explicitly names the required discriminator shape
  and cites accepted reconciliation evidence: satisfied.
- If blocked, nextAction names missing source/human/supervisor input and
  prevents hardware reruns: not applicable because the core is selected.
- No driver, packet, DMA, interrupt, networking, sockets, SSH, Phase 12.2, or
  phase transition claim is accepted: satisfied.
- Accepted closeout is committed before the discriminator core starts:
  satisfied by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
`phase12-rp1-ethernet-gem-mid-decode-discriminator-core-20260610` on the next
worker wake. It must implement only the local/static same-run observed RP1
`SYSINFO_CHIP_ID` positive-control plus translated `MACB_MID` report shape,
with a paired no-MMIO/no-Ethernet control. It must not run hardware, acquire
hardwareTestLock, implement Ethernet behavior, program RP1 MMIO/DMA, create
descriptor rings, claim interrupts/clock/reset/PHY ownership, perform packet
I/O, add networking/sockets/SSH, start Phase 12.2, or claim a phase
transition.
