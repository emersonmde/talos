# Phase 11 RP1 IRQ/Clock/GPIO Milestone Closeout

Task id: phase11-rp1-irq-clock-gpio-milestone-closeout-20260609

Status: accepted

Classification: rp1-irq-clock-gpio-milestone-112-blocker-checkpoint-accepted

## Goal

Close out the Milestone 11.2 RP1 interrupt/clock/GPIO frontier without
accepting GPIO ownership, event generation, interrupt delivery, clock/reset
ownership, DMA/cache, networking, SSH, Milestone 11.3 behavior, or a phase
transition by implication.

## Scope

- Reconciled the accepted RP1 interrupt-route, GIC-visible route, GPIO bank
  source-status, GPIO14/GPIO16 ownership/function blockers,
  observed-aperture GPIO16 preflight, clock/reset write-restore history, and
  accepted observed 0x1c clock/reset dependency blocker.
- Recorded findings with disposition.
- Named accepted Milestone 11.2 claims, rejected claims, retained blockers,
  and same-shaped retry policy.
- Updated project and roadmap docs for the accepted checkpoint only.
- Set the next action to the queued DMA/cache source inventory because the
  checkpoint accepts that Milestone 11.2 has captured minimal diagnostic
  blockers with serial evidence.

## Non-Goals

No runtime source changes, hardware run, boot archive publication,
hardwareTestLock acquisition, clock/reset writes, GPIO/RIO/pad/INTE/CTRL
writes, GPIO function changes, event generation, interrupt unmasking,
IAR/EOIR acknowledgement, ISR/handler install, DMA/cache implementation,
networking, SSH, Milestone 11.3 behavior, or phase transition beyond this
explicit checkpoint.

## Reconciliation

The checkpoint closes as
rp1-irq-clock-gpio-milestone-112-blocker-checkpoint-accepted.

- Interrupt route source and status:
  phase11-rp1-interrupt-routing-diagnostic-closeout-20260607 accepted the
  source-backed IO_BANK0 MSI-X vector 0 to MIP0/GIC SPI 128 / INTID 160 route
  identity and selected read-only MSIX_CFG(0) diagnostic boundary.
  phase11-rp1-gic-visible-route-closeout-20260607 accepted the read-only/no-ack
  GIC status snapshot for INTID 160: enable, pending, and active bits were
  clear and HPPIR reported spurious 1023. These are route/status observations,
  not interrupt delivery or handler ownership.
- GPIO bank source-status:
  phase11-rp1-gpio-bank-source-status-closeout-20260607 accepted the
  read-only IO_BANK0 INTE/INTS source-status snapshot visibility. The accepted
  raw values were retained only as diagnostic data and did not prove event
  generation, GPIO ownership, or interrupt delivery.
- GPIO14 ownership/function:
  phase11-rp1-observed-gpio-ownership-route-closeout-20260608 accepted the
  observed-aperture GPIO14 ownership/route preflight as visible and blocked on
  non-GPIO function: GPIO14 reported FUNCSEL=4 / uart0, with INTID 160 not
  enabled, pending, or active and HPPIR spurious.
- GPIO16 ownership/function:
  phase11-rp1-observed-gpio16-ownership-event-closeout-20260609 accepted the
  observed-aperture GPIO16 ownership/event preflight as visible and blocked on
  non-GPIO function: GPIO16 reported FUNCSEL=31 / unknown, IO_BANK0 INTE/INTS
  clear for GPIO16, RIO GPIO16 OUT/OE/IN false, pad input/output disabled, and
  INTID 160 not enabled, pending, or active.
- Clock/reset history:
  Earlier accepted clock-manager work proved source-backed read-only status,
  an idempotent CLK_ADC_CTRL write/restore boundary, a restored enable-toggle
  mismatch blocker, and ADC clock-window sentinel/readback blockers. Those
  boundaries remain evidence about narrow selected clock paths only, not broad
  clock/reset ownership.
- Observed 0x1c clock/reset dependency:
  phase11-rp1-clock-reset-dependency-closeout-20260609 accepted the
  read-only SYSINFO/clock-manager dependency snapshot as visible on Pi 5 and
  classified the current dependency state as system-clock-disabled:
  SYSINFO_CHIP_ID matched 0x20001927, selected clock reads did not return the
  0xdead_dead sentinel, PLL_SYS was locked, CLK_UART was enabled, and
  CLK_SYS/CLK_SLOW_SYS enable bits were false.

## Findings And Disposition

- fixed: reconciled Milestone 11.2 interrupt route evidence as documented
  source-backed route identity plus read-only GIC-visible status, not
  delivered interrupts.
- fixed: reconciled GPIO bank source-status evidence as a visible read-only
  status snapshot, not GPIO ownership or event generation.
- fixed: retained GPIO14 and GPIO16 observed-aperture blockers as decisive
  non-GPIO-function blockers for write-backed event setup.
- fixed: retained the clock/reset dependency chain as a source-backed
  read-only system-clock-disabled blocker before any GPIO function or
  interrupt-delivery work.
- fixed: accepted the roadmap Milestone 11.2 condition as satisfied by
  captured blockers with serial hardware evidence rather than by a working
  write-backed diagnostic.
- deferred: GPIO function changes, GPIO/RIO/pad/INTE/CTRL writes, event
  generation, interrupt pending generation, interrupt delivery,
  IAR/EOIR acknowledgement, ISR/handler ownership, broad RP1 mapping,
  clock/reset ownership, DMA/cache, networking, SSH, and any Milestone 11.3
  implementation remain future work.
- not-an-issue: moving next to the queued DMA/cache source inventory is not a
  phase transition; it is the next explicit Phase 11 task after the accepted
  Milestone 11.2 checkpoint and carries no DMA/cache implementation claim.

No findings were removed.

## Accepted Claims

- RP1 IO_BANK0 interrupt routing to GIC SPI 128 / INTID 160 is documented
  with retained source references and read-only route/status evidence.
- Minimal Milestone 11.2 diagnostics produced captured blocker evidence on
  Pi 5 serial output: GPIO14 is currently muxed to UART0, GPIO16 is not in an
  accepted GPIO function, INTID 160 was visible only as clear/spurious status,
  and the observed clock dependency snapshot showed selected system-clock
  enable bits false.
- The Milestone 11.2 roadmap acceptance condition is satisfied by captured
  blocker evidence with serial/lab-controller/TFTP/restore evidence, not by a
  working write-backed GPIO, status-LED, or interrupt-delivery diagnostic.
- The accepted Milestone 11.2 frontier is limited to source-backed route
  documentation, read-only status/ownership/dependency observations, paired
  no-MMIO controls, real Pi 5 visibility proofs, and recorded blockers.

## Rejected Claims And Retained Risks

This checkpoint does not accept GPIO ownership, event generation readiness,
interrupt pending generation, interrupt delivery, IAR/EOIR acknowledgement,
ISR/handler ownership, broad RP1 mapping, GPIO/RIO/pad/INTE/CTRL writes,
parent-route masking writes, clock/reset ownership, reset-controller
ownership, clock/reset writes, DMA/cache behavior, networking, SSH,
Milestone 11.3 behavior, or a phase transition.

Same-shaped retry policy:

- GPIO14 STATUS/ownership/route reruns are not progress unless future
  supervisor planning supplies a new discriminator or acceptance criteria that
  handles the accepted UART0/FUNCSEL=4 blocker.
- GPIO16 ownership/event reruns are not progress unless future supervisor
  planning supplies a source-backed function/ownership discriminator beyond
  the accepted FUNCSEL=31 / unknown blocker.
- GIC-visible route status reruns are not progress unless future supervisor
  planning supplies a different contract; the accepted clear/spurious snapshot
  must not be reinterpreted as interrupt delivery.
- GPIO bank INTE/INTS source-status reruns are not progress unless future
  supervisor planning supplies an event-generation or ownership contract that
  changes the acceptance surface.
- Observed SYSINFO/clock-manager dependency snapshot reruns are not progress
  unless future supervisor planning supplies materially different acceptance
  criteria or a new discriminator for the accepted system-clock-disabled
  blocker.

## Evidence

- Interrupt-routing closeout:
  tasks/2026-06-07-phase11-rp1-interrupt-routing-diagnostic-closeout.md.
- GIC-visible route closeout:
  tasks/2026-06-07-phase11-rp1-gic-visible-route-closeout.md.
- GPIO bank source-status closeout:
  tasks/2026-06-07-phase11-rp1-gpio-bank-source-status-closeout.md.
- GPIO14 ownership/route closeout:
  tasks/2026-06-08-phase11-rp1-observed-gpio-ownership-route-closeout.md.
- GPIO16 ownership/event closeout:
  tasks/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-closeout.md.
- Clock/reset dependency closeout:
  tasks/2026-06-09-phase11-rp1-clock-reset-dependency-closeout.md.
- Checkpoint evidence map:
  tasks/evidence/2026-06-09-phase11-rp1-irq-clock-gpio-milestone-closeout/evidence-map.json.
- Checkpoint classification:
  tasks/evidence/2026-06-09-phase11-rp1-irq-clock-gpio-milestone-closeout/classification.json.

## Validation

- static inspection: accepted Milestone 11.2 task records and evidence maps
  inspected.
- jq evidence-map/classification checks: passed.
- git diff --check: passed.
- docs build: mdbook build passed because docs/src files were updated.
- git diff --cached --check: passed before commit.

## Next Action

Mechanically promote phase11-rp1-dma-cache-source-inventory-20260609 next.
That task is source inventory only; it must not implement DMA/cache, networking,
SSH, or a DMA-capable driver.
