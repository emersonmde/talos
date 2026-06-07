# Task: Phase 11 RP1 GPIO Event-Latch Source Contract

Task ID: phase11-rp1-gpio-event-latch-source-contract-20260607

Status: accepted

Evidence level: static source/doc inspection

## Goal

Define the smallest source-backed GPIO14 event-latch or pending-generation
contract that can prove Talos can deliberately change RP1 IO_BANK0 source
status without accepting interrupt delivery.

## Scope

- Reviewed the accepted GPIO14 STATUS, interrupt-routing, GIC-visible route,
  and GPIO bank source-status frontiers.
- Reused retained Raspberry Pi Linux rpi-6.12.y RP1 pinctrl/MFD evidence.
- Checked whether exactly one source-backed GPIO14 event-latch or
  pending-generation discriminator is safe enough to hand to a local/static
  implementation task.
- Recorded exact source-backed register paths, bit fields, and why no safe
  bounded discriminator is available from retained evidence.
- Updated roadmap/project contract docs for the blocker frontier.

## Non-Goals

No runtime implementation, hardware run, boot archive publication,
hardwareTestLock acquisition, interrupt unmasking, interrupt delivery, GIC
enable writes, GIC IAR/EOIR acknowledgement, ISR installation, broad GPIO
ownership, pin-control or pad writes, clock/reset programming, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe enumeration, Milestone
11.3, or phase transition.

## Findings

- fixed: identified the only retained source-backed GPIO14 event configuration
  path. Linux rp1_irq_set_type clears raw event enables in GPIO14 CTRL, writes
  IRQRESET, then sets selected raw event-enable bits in GPIO14 CTRL.
- fixed: identified the only retained source-backed bank pending/interrupt
  source enable path. Linux rp1_gpio_irq_config writes GPIO14's bank bit to
  IO_BANK0 INTE set/clear aliases and writes GPIO14 CTRL IRQRESET when
  disabling.
- fixed: retained exact GPIO14 addresses for the blocked event path:
  STATUS at 0x1f000d0070, CTRL at 0x1f000d0074, CTRL SET at 0x1f000d2074,
  and CTRL CLR at 0x1f000d3074.
- fixed: retained exact IO_BANK0 source-status addresses for the surrounding
  snapshot: INTE at 0x1f000d011c, INTE SET at 0x1f000d211c, INTE CLR at
  0x1f000d311c, and INTS at 0x1f000d0124.
- fixed: classified the task as source-contract-blocked. The retained sources
  do not justify a safe bounded discriminator that deliberately changes GPIO14
  event/pending state while preserving the task non-goals.
- fixed: blocked the queued local/static core and Pi 5 follow-ups because their
  dependency requires accepted-source-contract, not a blocker.
- deferred: a future supervisor-planned GPIO ownership or interrupt-masking
  task may revisit this once Talos can safely own a pin, mask the parent route,
  and define restore semantics.
- not-an-issue: pre-existing INTE/INTS visibility remains useful as the prior
  frontier, but it is not treated as event generation or as proof that Talos
  can deliberately change pending state.

No findings were removed in this source-contract task.

## Blocked Contract Summary

Classification: source-contract-blocked.

No diagnostic target is accepted for implementation. The retained source path
does not provide a safe no-owner GPIO14 event-latch/pending-generation
discriminator.

The candidate source-backed write paths would require one or more of these
operations:

- GPIO14 CTRL CLR write of RP1_INT_MASK << RP1_GPIO_EVENTS_SHIFT_RAW to clear
  raw event enables.
- GPIO14 CTRL SET write of RP1_GPIO_CTRL_IRQRESET to clear latched events.
- GPIO14 CTRL SET write of one or more raw event-enable bits:
  IRQEN_FALLING, IRQEN_RISING, IRQEN_LOW, or IRQEN_HIGH.
- IO_BANK0 INTE SET or INTE CLR write of GPIO14 mask 0x00004000.

Those writes are not accepted because retained evidence does not prove that:

- GPIO14 can be used as a Talos-owned GPIO without disturbing firmware-owned
  UART0/pin state.
- Event-enable writes alone can generate or clear a deterministic
  source-status transition without changing pinmux, RIO output, pads, or input
  level.
- Bank INTE writes can be performed without interrupt unmasking or parent
  route side effects.
- Cleanup can restore the exact prior event-enable, latch, and bank-enable
  state after a failed or partial hardware run.

## Forbidden Operations

Until a future supervisor-planned task supplies a stronger ownership and
masking contract, the following remain forbidden for this event-latch path:

- GPIO14 CTRL, CTRL SET, or CTRL CLR writes.
- IO_BANK0 INTE SET or INTE CLR writes.
- GPIO IRQRESET acknowledgement writes.
- RIO output/input/enable writes, pad writes, pinmux/function-select writes,
  clock/reset writes, MSI-X/PCIe/MIP/GIC writes, GIC IAR/EOIR reads or writes,
  interrupt unmasking, ISR installation, and hardware execution of any
  event-latch candidate.

Read-only inspection of the previously accepted GPIO14 STATUS, IO_BANK0 INTE,
and IO_BANK0 INTS frontiers remains accepted only under their existing task
contracts.

## No-Write Control Disposition

No no-write/no-RP1/no-GIC control is authorized by this task because there is
no accepted real discriminator to pair it with. If the supervisor replans this
area, the control must preserve the same serial/output shape as the real
candidate while constructing no RP1 GPIO/RIO/pads/clock/reset, MSI-X/PCIe/MIP,
or GIC MMIO address and performing no volatile load or store to those paths.

## Accepted Claims

This task accepts only a source-contract blocker. It records that retained
source evidence identifies the relevant GPIO14 event/latch and bank
source-enable write paths but does not justify a safe implementation contract
for this milestone slice.

It does not accept GPIO event generation, interrupt pending generation beyond
the prior read-only snapshot, interrupt enablement or delivery, GIC
acknowledgement, ISR/handler ownership, broad GPIO ownership, pin-control
behavior, pad writes, clock/reset programming, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3, or
a phase transition.

## Validation

- Static source/doc inspection: retained in
  tasks/evidence/2026-06-07-phase11-rp1-gpio-event-latch-source-contract/source-reference-notes.md.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Result

Accepted as source-contract-blocked. Follow-up implementation, no-write
control, real Pi 5 diagnostic, and closeout tasks remain blocked because the
source-contract dependency was not accepted as accepted-source-contract.

## Follow-Up

Return to supervisor planning. A future task must define GPIO ownership,
parent-route masking, precise restore semantics, and a deterministic
event/pending discriminator before promoting any event-latch implementation.
