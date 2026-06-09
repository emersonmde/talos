# Phase 11 RP1 Observed GPIO14 Ownership/Route Closeout

Task id: phase11-rp1-observed-gpio-ownership-route-closeout-20260608

Status: accepted

Classification: observed-gpio14-ownership-route-preflight-non-gpio-blocker-frontier-closed

## Goal

Close out the observed-aperture GPIO14 ownership/route preflight chain and
decide whether another Milestone 11.2 task is mechanically justified.

## Scope

- Reconciled the accepted source contract, local/static core, no-MMIO Pi 5
  control proof, real Pi 5 read-only preflight proof, restore evidence,
  accepted claims, rejected claims, retained risks, and same-shaped retry
  policy.
- Updated project and roadmap docs for the accepted frontier.
- Set the next action to supervisor planning because no worker-owned task
  remains mechanically unblocked after this closeout.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime source changes, GPIO/RIO/pad/INTE/CTRL writes, IRQRESET, interrupt
unmasking, IAR/EOIR acknowledgement, ISR/handler install, event generation,
interrupt delivery, endpoint config retry, bridge setup write, clock/reset
write, DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3,
phase transition, or capture/staging relaxation.

## Findings And Disposition

- fixed: reconciled the accepted source contract
  phase11-rp1-observed-gpio-ownership-route-source-contract-v1 as a read-only
  observed-aperture preflight, not a write-backed ownership transition.
- fixed: retained the local/static core evidence that the real candidate uses
  only the accepted read set and the paired control constructs no forbidden
  RP1/GIC/PCIe/MIP/GPIO/RIO/pad/clock/reset MMIO addresses.
- fixed: retained the Pi 5 no-MMIO control proof as visible under decisive V3,
  run-unique, boot-staging, TFTP, final identity, and restore gates; no
  GPIO/RP1/GIC/PCIe hardware behavior is accepted from that control.
- fixed: retained the real Pi 5 proof as decisive read-only preflight
  visibility with selected tree
  e6ded87c576967c770223930463864fc081443467d6e00fbe108f29fa9e33fd2,
  two 50,496-byte da591740/kernel_2712.img fetches, final selected-tree
  identity, V3 freshness, boot-staging identity, marker-visible output, and
  restore to baseline tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: classified the accepted real preflight as a non-GPIO-function
  blocker: GPIO14 FUNCSEL=4 / uart0, GPIO14 input sampled high, IO_BANK0
  INTE/INTS clear, INTID160 not enabled/pending/active, and HPPIR spurious
  1023.
- deferred: any GPIO14 function change, GPIO/RIO/pad/INTE/CTRL writes,
  event-source generation, IRQRESET acknowledgement, interrupt unmasking,
  interrupt pending/delivery proof, handler ownership, and restore-after-write
  semantics require supervisor planning as a different feature slice.
- not-an-issue: the source-expected 0x1f GPIO ownership/route blockers remain
  retained context only; this chain was explicitly the observed 0x1c
  aperture and closed on a different, decisive non-GPIO-function blocker.
- not-an-issue: the saturated serial cursor did not weaken acceptance because
  the accepted V3 and boot-staging checkers passed with marker absence before
  power, marker visibility after power, expected TFTP bytes, final identity,
  and restore proof.

No findings were removed.

## Accepted Claims

- The observed 0x1c GPIO14 ownership/route preflight report is visible on Pi 5
  under the accepted capture/staging gates.
- GPIO14 currently reports FUNCSEL=4 / uart0 in this preflight and is not a
  GPIO-owned event-generation target.
- The selected read-only parent-route snapshot reports INTID160 not enabled,
  pending, or active, with HPPIR reporting spurious 1023.
- The accepted frontier is limited to the source contract, local/static
  implementation, no-MMIO control proof, and real read-only preflight
  classification.

## Rejected Claims And Retained Risks

This closeout does not accept GPIO ownership, event generation readiness,
interrupt pending generation, interrupt delivery, IAR/EOIR acknowledgement,
handler ownership, broad RP1 mapping, GPIO/RIO/pad/INTE/CTRL writes,
parent-route masking writes, clock/reset programming, DMA/cache, networking,
SSH, Milestone 11.3, or a phase transition.

Same-shaped observed-aperture GPIO14 ownership/route preflight reruns are not
progress unless future supervisor planning supplies materially different
acceptance criteria or a new discriminator. The next progress item must decide
how to handle GPIO14 being muxed to UART0 before any write-backed GPIO event
or interrupt-delivery work.

## Evidence

- Source contract task record:
  tasks/2026-06-08-phase11-rp1-observed-gpio-ownership-route-source-contract.md.
- Local/static core task record:
  tasks/2026-06-08-phase11-rp1-observed-gpio-ownership-route-core.md.
- No-MMIO control task record:
  tasks/2026-06-08-phase11-rp1-observed-gpio-ownership-route-control-pi5.md.
- Real preflight task record:
  tasks/2026-06-08-phase11-rp1-observed-gpio-ownership-route-pi5.md.
- Closeout evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-closeout/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-closeout/classification.json.

## Validation

- static inspection: source/core/control/real task records and evidence maps
  inspected.
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
