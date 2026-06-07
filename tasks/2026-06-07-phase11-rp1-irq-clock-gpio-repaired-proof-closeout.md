# Phase 11 RP1 IRQ/Clock/GPIO Repaired Proof Closeout

Task id: phase11-rp1-irq-clock-gpio-repaired-proof-closeout-20260607

Status: accepted

## Goal

Close out the repaired capture/control/real GPIO14 STATUS proof chain and
decide whether Milestone 11.2 can move to interrupt-routing contract work or
remains blocked.

## Scope

- Reconciled the accepted Milestone 11.2 source contract, GPIO14 STATUS
  diagnostic core, repaired capture identity replay, repaired no-MMIO control
  proof, and real GPIO14 STATUS repaired proof.
- Recorded accepted and unaccepted claims for GPIO/status behavior,
  interrupt routing, clock/reset assumptions, capture/restore hygiene, and the
  next Milestone 11.2 step.
- Updated roadmap and project contract docs for the accepted closeout frontier.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime source changes, GPIO/pin-control writes, pad writes, clock/reset
programming, interrupt enablement or handling, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3, or
phase transition.

## Classification

Accepted as gpio14-status-read-frontier-closed.

The repaired chain accepts one read-only RP1 GPIO14 STATUS diagnostic boundary:
the source contract phase11-rp1-irq-clock-gpio-contract-v1, target
rp1-gpio14-status-read, CPU physical address 0x1f000d0070, one 32-bit
volatile load, and the Pi 5 repaired proof rerun that tied selected candidate
identity, two 46,336-byte TFTP fetches, final selected-tree identity, visible
serial result markers, and restore proof. The accepted result lines reported
raw 0xdeaddead with classification=diagnostic-result-visible.

The capture identity repair did not relax acceptance criteria. It added
retained-evidence replay coverage showing that compromised marker-visible and
candidate-rerun evidence remains rejected when serial freshness, TFTP identity,
or final selected-tree identity do not join, while the repaired no-MMIO control
and repaired real proof pass the same v2 identity boundary.

This closeout does not accept GPIO ownership, pin-control, pad writes,
clock/reset programming, interrupt enablement or delivery, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3, or
phase transition.

## Findings And Disposition

- fixed: reconciled the source contract, no-MMIO control, repaired identity
  replay, real GPIO14 STATUS proof, and retained compromised-run evidence.
- fixed: recorded the accepted frontier as the read-only GPIO14 STATUS
  diagnostic boundary with v2 identity-joined Pi 5 evidence.
- fixed: updated roadmap and project contract docs so the former pending
  closeout language now names the accepted closeout boundary.
- not-an-issue: the earlier marker-visible GPIO14 run remains useful evidence,
  but it is still rejected by the v2 identity join because serial freshness
  was not clean.
- not-an-issue: the accepted raw value 0xdeaddead is treated as the observed
  result of this diagnostic, not as proof of GPIO ownership or pinmux state.
- deferred: interrupt-routing source contract work remains supervisor-planned
  future Milestone 11.2 work.
- deferred: Talos-owned GPIO/pin-control, pad writes, clock/reset programming,
  interrupt enablement/delivery, DMA/cache, storage, generated-root,
  networking, SSH, broader PCIe, Milestone 11.3, and phase transition remain
  unaccepted.

## Accepted Claims

- phase11-rp1-irq-clock-gpio-contract-v1 is the accepted Milestone 11.2 source
  contract for this slice.
- The paired no-MMIO GPIO14 control was visible on Pi 5 through the repaired
  capture identity path.
- The real GPIO14 STATUS repaired proof accepted one read-only 32-bit load
  from 0x1f000d0070 and repeated visible diagnostic result output tied to the
  selected candidate.
- The repaired capture/identity path keeps the existing v2 rejection rules for
  non-fresh serial, mismatched TFTP bytes, and final selected-tree mismatch.
- Hardware lock and restore hygiene were retained for the hardware proof tasks;
  the lab was restored to the pre-run known-good tree.

## Retained Risks

- GPIO14 may still be firmware-owned or muxed for UART0 TXD; this closeout
  does not claim Talos GPIO ownership.
- Interrupt routing, MSI-X/GIC delivery, clock/reset dependencies, and pad
  programming remain source-contract work, not implemented behavior.
- The observed raw value is diagnostic evidence only; no driver behavior or
  interrupt behavior is inferred from it.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-repaired-proof-closeout/evidence-map.json.
- Source contract:
  tasks/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract.md.
- Diagnostic core:
  tasks/2026-06-07-phase11-rp1-gpio-status-diagnostic-core.md.
- Capture identity repair:
  tasks/2026-06-07-phase11-pi5-capture-identity-join-repair-core.md.
- Repaired no-MMIO control:
  tasks/2026-06-07-phase11-pi5-capture-identity-repaired-control-pi5.md.
- Repaired real proof:
  tasks/2026-06-07-phase11-rp1-gpio-status-repaired-proof-pi5.md.

## Validation

- static inspection of capture repair, no-MMIO control, real proof, source
  contract, and evidence maps: passed.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as gpio14-status-read-frontier-closed. No explicit worker-owned queued
task remains after this closeout; supervisor planning is required for the next
Milestone 11.2 interrupt-routing source contract. Same-shaped GPIO14 STATUS
hardware reruns are blocked unless a future supervisor task supplies a
different discriminator or new acceptance criteria.
