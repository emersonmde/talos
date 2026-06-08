# Phase 11 RP1 Observed Aperture Closeout

Task id: phase11-rp1-observed-aperture-closeout-20260608

Status: accepted

Classification: observed-aperture-rp1-uart0-fr-visible-frontier-closed

## Goal

Close out the observed-aperture discriminator chain and record the accepted
frontier without implying endpoint ownership, broad RP1 mapping, UART
ownership, interrupt delivery, GPIO/clock work, Milestone 11.3, or a phase
transition.

## Scope

- Reconciled the accepted source contract, local/static core, paired
  no-MMIO/no-PCIe/no-RP1/no-GIC control proof, real Pi 5 proof, restore
  evidence, and retained risks.
- Recorded findings with disposition.
- Named accepted and unaccepted claims for the observed aperture, live RP1
  identity, endpoint ownership, broad RP1 mapping, interrupt delivery,
  GPIO/clock ownership, DMA/cache, storage, generated-root, networking, SSH,
  Milestone 11.3, and phase transition.
- Updated roadmap and RP1/PCIe map contract docs for the closeout frontier.
- Set nextAction to supervisor planning rather than creating a worker-owned
  follow-up task.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime source changes, endpoint config retry, BAR discovery or programming,
bridge setup writes, PERST/link-control changes, GPIO/pad/clock/reset writes,
interrupt enablement or delivery, GIC acknowledgement, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, or phase transition.

## Findings And Disposition

- fixed: reconciled the accepted source contract boundary. The only real
  selected operation was one read-only 32-bit volatile load from observed CPU
  physical address 0x1c00030018, the RP1 UART0 PL011 flag-register offset
  selected from retained source and first-light evidence after the bridge/setup
  0x1f mismatch.
- fixed: reconciled the local/static core evidence. The real archive retained
  the accepted report shape and classification vocabulary, and the paired
  control retained the same output shape without constructing forbidden
  BCM2712 PCIe, RP1, MIP, GIC, GPIO, clock/reset, DMA, or other MMIO
  addresses.
- fixed: reconciled the control proof as
  no-mmio-observed-aperture-control-visible. The accepted unchanged rerun
  passed capture-transaction-v2, retained two 47,344-byte candidate fetches,
  72 control markers, and restore to the pre-run tree.
- fixed: reconciled the real Pi 5 proof as
  observed-aperture-rp1-uart0-fr-visible. The accepted unchanged rerun after
  known-good-control triage passed capture-transaction-v2, retained two
  47,664-byte candidate fetches, 69 result markers, raw=0x187,
  raw-is-pl011-fr-shaped=true, and restore to the pre-run tree.
- fixed: recorded the accepted claim precisely. The 0x1c00030018 observed
  aperture returned a non-sentinel, non-zero, non-all-ones PL011-FR-shaped
  value on Pi 5, but this is not endpoint ownership, UART ownership, or a
  broad RP1 mapping claim.
- fixed: made the same-shaped rerun policy explicit. Same-shaped endpoint
  config identity, bridge/setup-state, 0x1f RP1 peripheral, and 0x1c observed
  aperture hardware reruns are not progress unless a future supervisor task
  supplies a different discriminator or new acceptance criteria.
- deferred: supervisor planning must choose any later endpoint visibility
  retry, bridge/BAR/setup discriminator, live UART ownership discriminator,
  interrupt-delivery work, GPIO/clock retry, or blocker-driven alternate
  Milestone 11.2 slice. This closeout creates no worker-owned follow-up task.
- deferred: live RP1 ownership, endpoint ownership, broad RP1 mapping, UART
  ownership, interrupt delivery, GPIO/clock ownership, DMA/cache, storage,
  generated-root, networking, SSH, Milestone 11.3, and phase transition remain
  outside this accepted frontier.
- not-an-issue: raw=0x187 is PL011-FR-shaped under the local mask, but the
  accepted contract intentionally treats that as observed-aperture visibility
  only; ownership and interrupt behavior require separate source contracts and
  hardware proofs.

No findings were removed.

## Closeout Classification

Accepted as observed-aperture-rp1-uart0-fr-visible-frontier-closed.

The accepted frontier is limited to the source/evidence-backed
observed-aperture discriminator, the paired no-MMIO/no-PCIe/no-RP1/no-GIC
control proof, and the real Pi 5 visible result under identity-joined hardware
evidence. The selected real operation was one 32-bit volatile read from
observed CPU physical address 0x1c00030018.

The accepted hardware result proves that the selected observed aperture is
visible on Pi 5 for this one read. The retained value is raw=0x187, with
raw-is-deaddead=false, raw-is-all-ones=false, raw-is-zero=false, and
raw-is-pl011-fr-shaped=true. The accepted evidence chain ties the selected
candidate artifact, two 47,664-byte TFTP fetches, serial result markers,
final selected-tree identity, and restore proof into one capture transaction.

This does not accept endpoint ownership, broad RP1 mapping, UART ownership,
interrupt delivery, GPIO/clock ownership, DMA/cache, storage, generated-root,
networking, SSH, Milestone 11.3, or a phase transition.

Same-shaped endpoint config identity, same-shaped bridge/setup-state,
same-shaped 0x1f RP1 peripheral, and same-shaped 0x1c observed-aperture
hardware reruns remain closed unless a future supervisor task supplies a
different discriminator or new acceptance criteria. Supervisor planning is
required for the next Milestone 11.2 frontier.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-closeout/evidence-map.json.
- Static reconciliation:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-closeout/static-reconciliation.md.
- Source contract:
  tasks/2026-06-08-phase11-rp1-observed-aperture-source-contract.md.
- Local/static core:
  tasks/2026-06-08-phase11-rp1-observed-aperture-core.md.
- Control Pi 5 proof:
  tasks/2026-06-08-phase11-rp1-observed-aperture-control-pi5.md.
- Real Pi 5 proof:
  tasks/2026-06-08-phase11-rp1-observed-aperture-pi5.md.
- Classification records:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-control-pi5/classification.json,
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-pi5/classification.json.

## Validation

- static inspection: source contract, local/static core, control proof, real
  proof, restore evidence, and evidence maps inspected.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as observed-aperture-rp1-uart0-fr-visible-frontier-closed.

Next action: no worker-owned follow-up task is created by this closeout.
Set planningNeeded=true for supervisor planning of the next Milestone 11.2
frontier if work should continue.
