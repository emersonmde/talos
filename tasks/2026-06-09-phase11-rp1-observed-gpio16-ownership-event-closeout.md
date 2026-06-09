# Phase 11 RP1 Observed GPIO16 Ownership/Event Closeout

Task id: phase11-rp1-observed-gpio16-ownership-event-closeout-20260609

Status: accepted

Classification: observed-gpio16-ownership-event-preflight-non-gpio-blocker-frontier-closed

## Goal

Close out the observed-aperture GPIO16 ownership/event preflight chain without
implying GPIO ownership, event generation, interrupt delivery, broad RP1
mapping, or a phase transition.

## Scope

- Reconciled the accepted source contract, local/static core, no-MMIO Pi 5
  control retry proof, real Pi 5 read-only preflight proof, restore evidence,
  accepted claims, rejected claims, retained risks, and same-shaped retry
  policy.
- Updated project and roadmap docs for the accepted frontier.
- Set the next action to supervisor planning because no worker-owned task
  remains mechanically unblocked after this closeout.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime source changes, GPIO/RIO/pad/INTE/CTRL writes, IRQRESET, interrupt
unmasking, IAR/EOIR acknowledgement, ISR/handler install, event generation,
interrupt delivery, GPIO14 ownership change, endpoint config retry, bridge
setup write, clock/reset write, DMA/cache, storage, generated-root,
networking, SSH, Milestone 11.3, phase transition, or capture/staging
relaxation.

## Reconciliation

The chain closes as
observed-gpio16-ownership-event-preflight-non-gpio-blocker-frontier-closed.

- Source contract:
  phase11-rp1-observed-gpio16-ownership-event-source-contract-20260609
  accepted
  phase11-rp1-observed-gpio16-ownership-event-source-contract-v1 as a
  read-only observed-aperture GPIO16 ownership/event preflight. The selected
  loads are GPIO16 STATUS/CTRL at 0x1c000d0080/0x1c000d0084, IO_BANK0
  INTE/INTS at 0x1c000d011c/0x1c000d0124, RIO0 OUT/OE/IN at
  0x1c000e0000/0x1c000e0004/0x1c000e0008, GPIO16 pad control at
  0x1c000f0044, and read-only INTID160 GIC route status registers.
- Local/static core:
  phase11-rp1-observed-gpio16-ownership-event-core-20260609 accepted real and
  paired control candidates for only that read set and output shape. The real
  candidate performs no GPIO/RIO/pad/INTE/CTRL writes, no IRQRESET, no event
  generation, and no action/restore sequence; the control constructs no RP1 or
  GIC MMIO address.
- Control proof:
  phase11-rp1-observed-gpio16-ownership-event-control-pi5-retry-20260609
  accepted no-mmio-observed-gpio16-ownership-event-control-visible under the
  repaired run-unique discriminator, with selected tree
  cdb35bef8b7fbd5b68df9c76a58fbb410e20522d46aed6b77319002b0be6bd19,
  two 48,744-byte da591740/kernel_2712.img fetches, nonce-visible serial
  output after power, final selected-tree identity, boot-staging checker
  success, and restore to the baseline tree.
- Real preflight:
  phase11-rp1-observed-gpio16-ownership-event-pi5-20260609 accepted the real
  read-only preflight as
  observed-gpio16-ownership-preflight-blocked-non-gpio-function. The run used
  selected tree 908eadd18fab1ba826d2dba92125649383a4857ed39ea18af125feb721a637c3,
  retained two 50,640-byte da591740/kernel_2712.img fetches, final
  selected-tree identity, V3 and boot-staging checker success,
  marker-visible output, and restore to the baseline tree. The visible result
  reported GPIO16 FUNCSEL=31 / unknown, IO_BANK0 INTE/INTS clear, GPIO16 RIO
  OUT/OE/IN false, pad input disabled, pad output disabled, INTID160 not
  enabled/pending/active, and HPPIR spurious 1023.

## Findings And Disposition

- fixed: reconciled the accepted GPIO16 source contract as a read-only
  observed-aperture preflight, not a write-backed GPIO ownership transition.
- fixed: retained the local/static evidence that the real candidate uses only
  the accepted read set and the paired control constructs no forbidden RP1/GIC
  MMIO addresses.
- fixed: retained the retried no-MMIO control proof as visible under decisive
  run-unique, boot-staging, TFTP, final identity, and restore gates; no
  GPIO/RP1/GIC/PCIe hardware behavior is accepted from that control.
- fixed: retained the real Pi 5 proof as decisive read-only GPIO16 preflight
  visibility with V3 freshness, expected TFTP bytes, final identity,
  marker-visible output, and restore proof.
- fixed: classified GPIO16 as a non-GPIO-function blocker: FUNCSEL=31 /
  unknown, GPIO16 source-enable/source-status clear, RIO GPIO16 bits false,
  pad input disabled, pad output disabled, INTID160 not enabled/pending/active,
  and HPPIR spurious 1023.
- deferred: GPIO16 function changes, GPIO/RIO/pad/INTE/CTRL writes,
  event-source generation, IRQRESET acknowledgement, interrupt unmasking,
  interrupt pending/delivery proof, handler ownership, restore-after-write
  semantics, and any next Milestone 11.2 direction require supervisor
  planning as a different feature slice.
- not-an-issue: the prior source-expected 0x1f GPIO16 fsel 13 blocker remains
  retained context only; this chain used the observed 0x1c aperture and closed
  on a newer FUNCSEL=31 / unknown non-GPIO-function blocker.
- not-an-issue: saturated serial did not weaken acceptance because the
  accepted V3/run-unique and boot-staging gates proved current-run visibility,
  expected TFTP bytes, final identity, and restore proof.

No findings were removed.

## Accepted Claims

- The observed 0x1c GPIO16 ownership/event preflight report is visible on Pi 5
  under the accepted capture/staging gates.
- GPIO16 currently reports FUNCSEL=31 / unknown in this preflight and is not a
  GPIO-owned event-generation target.
- GPIO16 source-enable/source-status are clear in IO_BANK0 INTE/INTS, RIO
  GPIO16 bits are false, pad input is disabled, pad output is disabled, and
  the parent-route snapshot reports INTID160 not enabled, pending, or active,
  with HPPIR reporting spurious 1023.
- The accepted frontier is limited to the source contract, local/static
  implementation, repaired no-MMIO control proof, and real read-only preflight
  classification.

## Rejected Claims And Retained Risks

This closeout does not accept GPIO ownership, event generation readiness,
interrupt pending generation, interrupt delivery, IAR/EOIR acknowledgement,
handler ownership, broad RP1 mapping, GPIO/RIO/pad/INTE/CTRL writes,
parent-route masking writes, clock/reset programming, DMA/cache, networking,
SSH, Milestone 11.3, or a phase transition.

Same-shaped observed-aperture GPIO16 ownership/event preflight reruns are not
progress unless future supervisor planning supplies materially different
acceptance criteria or a new discriminator. The next progress item must decide
which Milestone 11.2 feature slice follows this non-GPIO-function blocker
before any write-backed GPIO event, pin-function change, or interrupt-delivery
work.

## Evidence

- Source contract task record:
  tasks/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-source-contract.md.
- Local/static core task record:
  tasks/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-core.md.
- No-MMIO control retry task record:
  tasks/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-control-pi5-retry.md.
- Real preflight task record:
  tasks/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-pi5.md.
- Closeout evidence map:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-closeout/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-closeout/classification.json.

## Validation

- static inspection: source/core/control-retry/real task records and evidence
  maps inspected.
- jq evidence-map/classification checks: passed.
- git diff --check: passed.
- docs build: mdbook build passed because docs/src files were updated.
- git diff --cached --check: passed before commit.

## Next Action

No worker-owned next task is mechanically unblocked after this closeout. Set
planningNeeded=true for supervisor planning around the next Milestone 11.2
frontier before any GPIO function change, write-backed event setup,
interrupt-delivery attempt, broad ownership claim, Milestone 11.3 work, or
phase transition.
