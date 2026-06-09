# Phase 11 RP1 Clock/Reset Dependency Closeout

Task id: phase11-rp1-clock-reset-dependency-closeout-20260609

Status: accepted

Classification: clock-reset-dependency-preflight-system-clock-blocker-frontier-closed

## Goal

Close out the read-only RP1 clock/reset dependency preflight chain without
accepting clock/reset ownership, GPIO ownership, event generation, interrupt
delivery, broad RP1 mapping, or a phase transition by implication.

## Scope

- Reconciled the accepted source contract, local/static core, no-MMIO Pi 5
  control proof, real Pi 5 read-only proof, restore evidence, accepted claims,
  rejected claims, retained risks, and same-shaped retry policy.
- Updated project and roadmap docs for the accepted frontier.
- Set the next action to supervisor planning because no worker-owned task
  remains mechanically unblocked after this closeout.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime source changes, clock/reset writes, GPIO function changes,
GPIO/RIO/pad/INTE/CTRL writes, IRQRESET, interrupt unmasking, IAR/EOIR
acknowledgement, ISR/handler install, event generation, interrupt delivery,
endpoint config retry, bridge setup write, DMA/cache, networking, SSH,
Milestone 11.3, phase transition, or capture/staging relaxation.

## Reconciliation

The chain closes as
clock-reset-dependency-preflight-system-clock-blocker-frontier-closed.

- Source contract:
  phase11-rp1-clock-reset-dependency-source-contract-20260609 accepted
  phase11-rp1-clock-reset-dependency-source-contract-v1 as a read-only
  observed-aperture SYSINFO and clock-manager dependency preflight. The
  selected loads are SYSINFO_CHIP_ID/SYSINFO_PLATFORM at
  0x1c00000000/0x1c00000004, PLL_SYS_CS at 0x1c00020000,
  CLK_SYS_CTRL/DIV_INT/SEL at
  0x1c00018014/0x1c00018018/0x1c00018020, CLK_SLOW_SYS_CTRL at
  0x1c00018024, and CLK_UART_CTRL/DIV_INT/SEL at
  0x1c00018054/0x1c00018058/0x1c00018060. No reset-controller read was
  selected because retained Linux source exposes reset_control_reset, not a
  bounded safe read-only reset-status register.
- Local/static core:
  phase11-rp1-clock-reset-dependency-core-20260609 accepted real and paired
  control candidates for only that read set and output shape. The real
  candidate performs no clock/reset writes, GPIO writes, event generation, or
  interrupt work; the control constructs no RP1, GPIO, clock/reset, PCIe/MIP,
  GIC, DMA, or other forbidden MMIO address.
- Control proof:
  phase11-rp1-clock-reset-dependency-control-pi5-20260609 accepted
  no-mmio-clock-reset-dependency-control-visible with selected tree
  3f48e70435914a0ca3deb160c517a32205643c3fbd9547d407387895ae417aba,
  two 48,640-byte da591740/kernel_2712.img fetches, nonce-visible serial
  output after power, boot-staging checker success, final selected-tree
  identity, and restore to the baseline tree.
- Real preflight:
  phase11-rp1-clock-reset-dependency-pi5-20260609 accepted the real read-only
  preflight as observed-clock-reset-dependency-blocked-system-clock-disabled.
  The run used selected tree
  ef7b62b81d097a52bda724d2173c982fa512e2b6541541514abebd6d8db1422f,
  retained two 49,496-byte da591740/kernel_2712.img fetches, passed V3 and
  boot-staging identity checks, retained marker-visible output, and restored to
  the baseline tree. The visible result reported chip-id 0x20001927,
  platform 0x2, PLL_SYS_CS 0x80000001, CLK_SYS_CTRL 0x2, CLK_SLOW_SYS_CTRL
  0x0, CLK_UART_CTRL 0x10000840, chip-id-matches-expected=true,
  pll-sys-locked=true, clk-sys-enabled=false, clk-slow-sys-enabled=false,
  clk-uart-enabled=true, and no selected clock returned the 0xdead_dead
  sentinel.

## Findings And Disposition

- fixed: reconciled the accepted source contract as a read-only observed 0x1c
  aperture dependency preflight, not a clock/reset ownership or write contract.
- fixed: retained the local/static evidence that the real candidate uses only
  the accepted SYSINFO/clock-manager read set and the paired control constructs
  no forbidden MMIO addresses.
- fixed: retained the no-MMIO control proof as visible under decisive
  run-unique, boot-staging, TFTP, final identity, and restore gates; no
  RP1/clock/reset/GPIO/GIC/PCIe hardware behavior is accepted from that
  control.
- fixed: retained the real Pi 5 proof as decisive read-only dependency
  visibility with V3 freshness, expected TFTP bytes, final identity,
  marker-visible output, and restore proof.
- fixed: classified the accepted read-only snapshot as blocked on selected
  system-clock enable state: chip identity matched, PLL_SYS was locked,
  CLK_UART was enabled, no selected clock read returned the 0xdead_dead
  sentinel, but CLK_SYS and CLK_SLOW_SYS enable bits were false.
- deferred: any clock/reset write, reset-controller ownership, GPIO function
  change, write-backed event setup, interrupt delivery, DMA/cache, networking,
  SSH, Milestone 11.3, phase transition, or next Milestone 11.2 direction
  requires supervisor planning as a different feature slice.
- not-an-issue: the retained 0x1f SYSINFO/clock sentinel evidence remains
  comparator context only; this chain used the observed 0x1c aperture and
  closed on a source-backed clock-manager dependency snapshot.
- not-an-issue: saturated serial did not weaken acceptance because the
  accepted V3/run-unique and boot-staging gates proved current-run visibility,
  expected TFTP bytes, final identity, and restore proof.

No findings were removed.

## Accepted Claims

- The observed 0x1c RP1 SYSINFO and selected clock-manager dependency
  preflight report is visible on Pi 5 under the accepted capture/staging gates.
- SYSINFO_CHIP_ID matched 0x20001927 and selected clock-manager reads did not
  return the 0xdead_dead sentinel in the accepted snapshot.
- The accepted read-only snapshot classified the current dependency state as
  observed-clock-reset-dependency-blocked-system-clock-disabled because
  clk-sys-enabled=false and clk-slow-sys-enabled=false while
  pll-sys-locked=true and clk-uart-enabled=true.
- The accepted frontier is limited to the source contract, local/static
  implementation, no-MMIO control proof, and real read-only dependency
  snapshot classification.

## Rejected Claims And Retained Risks

This closeout does not accept clock/reset ownership, reset-controller
ownership, clock/reset writes, GPIO ownership, GPIO function changes, event
generation readiness, interrupt pending generation, interrupt delivery,
IAR/EOIR acknowledgement, handler ownership, broad RP1 mapping, GPIO/RIO/pad/
INTE/CTRL writes, endpoint config retry, bridge setup writes, DMA/cache,
networking, SSH, Milestone 11.3, or a phase transition.

Same-shaped clock/reset dependency preflight reruns are not progress unless
future supervisor planning supplies materially different acceptance criteria or
a new discriminator. The next progress item must decide which Milestone 11.2
feature slice follows this system-clock-disabled blocker before any clock/reset
write, GPIO function change, write-backed event setup, or interrupt-delivery
work.

## Evidence

- Source contract task record:
  tasks/2026-06-09-phase11-rp1-clock-reset-dependency-source-contract.md.
- Local/static core task record:
  tasks/2026-06-09-phase11-rp1-clock-reset-dependency-core.md.
- No-MMIO control task record:
  tasks/2026-06-09-phase11-rp1-clock-reset-dependency-control-pi5.md.
- Real preflight task record:
  tasks/2026-06-09-phase11-rp1-clock-reset-dependency-pi5.md.
- Closeout evidence map:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-closeout/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-closeout/classification.json.

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
frontier before any clock/reset write, GPIO function change, write-backed
event setup, interrupt-delivery attempt, broad ownership claim, Milestone 11.3
work, or phase transition.
