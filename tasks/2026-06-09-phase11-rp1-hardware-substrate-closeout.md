# Phase 11 RP1 Hardware Substrate Closeout

Task id: phase11-rp1-hardware-substrate-closeout-20260609

Status: accepted

Classification: rp1-hardware-substrate-phase11-frontier-closed

## Goal

Close Phase 11 as a hardware-substrate/research frontier before source-only
Phase 12.1 Ethernet research, without accepting live DMA, RP1 driver
ownership, Ethernet readiness, networking, SSH, or Phase 12 implementation by
implication.

## Scope

- Reconciled accepted Phase 11 evidence for RP1/PCIe mapping, PCIe2 host-link
  and endpoint/bridge blockers, interrupt/clock/GPIO Milestone 11.2, and
  DMA/cache Milestone 11.3.
- Recorded findings with disposition.
- Updated project and roadmap docs for the accepted Phase 11 frontier and
  retained risks only.
- Selected the exact next source-only task:
  phase12-rp1-ethernet-source-inventory-20260609.

## Non-Goals

No runtime source changes, Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, RP1 MMIO writes, DMA channel programming,
descriptor-ring construction, interrupt enablement or delivery,
Ethernet/storage driver work, network stack work, sockets, SSH, or Phase 12
implementation.

## Reconciliation

The Phase 11 checkpoint closes as
rp1-hardware-substrate-phase11-frontier-closed.

- RP1/PCIe mapping and host-link frontier:
  retained Phase 11 task records and project docs accept a source-backed RP1
  address-translation contract, read-only RP1/observed-aperture visibility
  diagnostics, and PCIe2 host-link visibility. They also retain endpoint config
  all-ones and bridge setup incomplete blockers. These are substrate facts and
  blockers, not broad RP1 ownership, general PCIe enumeration, endpoint
  configuration readiness, or driver readiness.
- Milestone 11.2 interrupt/clock/GPIO frontier:
  phase11-rp1-irq-clock-gpio-milestone-closeout-20260609 accepts source-backed
  interrupt-route documentation and minimal serial-captured blockers: GPIO14 is
  muxed to UART0, GPIO16 is not in an accepted GPIO function, INTID 160 status
  is only clear/spurious read-only status, and the observed clock dependency
  snapshot reports selected system-clock enable bits false. This does not
  accept GPIO ownership, event generation, interrupt delivery, handler
  ownership, or clock/reset ownership.
- Milestone 11.3 DMA/cache frontier:
  phase11-rp1-dma-cache-milestone-11-3-closeout-20260609 accepts documented
  DMA buffer ownership and cache-maintenance rules through the source
  inventory, substrate/core, sync plan, maintenance sequence, and
  architecture-gated executor chain. The small DMA or driver-adjacent
  diagnostic requirement is satisfied only by the accepted local/static plan
  and run-unique Pi 5 candidate/control visibility of the report path. This
  does not accept live DMA, channel ownership, descriptor rings, transfer
  completion, interrupt completion, hardware/device completion, Ethernet or
  storage readiness, networking, SSH, or Phase 12 work.

## Findings And Disposition

- fixed: reconciled the RP1/PCIe mapping history as a source-backed and
  visibility-backed substrate frontier with explicit endpoint/bridge blockers,
  not a broad RP1 mapping or driver ownership claim.
- fixed: retained Milestone 11.2 as an accepted blocker checkpoint with serial
  evidence, not a GPIO ownership, interrupt-delivery, or clock/reset ownership
  frontier.
- fixed: retained Milestone 11.3 as accepted only at documented/local-static
  and visibility/control boundaries, not as live DMA or Ethernet readiness.
- fixed: updated roadmap and project-contract wording so the accepted next
  step is source-only Phase 12.1 Ethernet source inventory, not
  descriptor-ring/channel-ownership, live DMA, networking, SSH, or Phase 12
  implementation.
- deferred: descriptor-ring layout/ownership, RP1 DMA channel ownership,
  transfer and interrupt completion, clock/reset ownership, GPIO/event
  ownership, Ethernet/storage driver readiness, packet I/O, network stack,
  sockets, SSH, and any hardware phase-transition claims remain future work.
- not-an-issue: selecting Phase 12.1 source inventory is not an implementation
  transition; the queued task is static research only and carries explicit
  non-goals against code, hardware, packet I/O, networking, and SSH.

No findings were removed.

## Accepted Claims

- Phase 11 has closed as a substrate/research frontier sufficient to begin
  source-only Phase 12.1 Ethernet inventory.
- RP1/PCIe address/path, host-link visibility, and retained endpoint/bridge
  blockers are documented well enough for source inventory to cite them as
  prerequisites and risks.
- Milestone 11.2 has accepted source-backed interrupt-route documentation and
  serial-captured interrupt/clock/GPIO blockers.
- Milestone 11.3 has accepted documented DMA buffer ownership and
  cache-maintenance rules plus a local/static and visibility-backed
  driver-adjacent diagnostic path.

## Rejected Claims And Retained Risks

This checkpoint does not accept live DMA, RP1 MMIO/DMA programming,
descriptor-ring construction or ownership, RP1 DMA channel ownership, transfer
completion, interrupt completion, clock/reset ownership, GPIO/event ownership,
Ethernet/storage readiness, packet I/O, networking, sockets, SSH, or Phase 12
implementation by implication.

Retained risks:

- endpoint config identity remains all-ones and bridge setup remains
  incomplete at the accepted Phase 11 frontier;
- no live RP1 DMA transfer, descriptor ring, transfer completion, or interrupt
  completion has been accepted;
- GPIO/event ownership and interrupt delivery remain blocked behind accepted
  function/clock/status evidence;
- clock/reset ownership and reset-controller ownership remain unaccepted;
- Phase 12.1 must start with source-only Ethernet inventory before any
  implementation or hardware diagnostic.

## Evidence

- Milestone 11.2 closeout:
  tasks/2026-06-09-phase11-rp1-irq-clock-gpio-milestone-closeout.md.
- Milestone 11.3 closeout:
  tasks/2026-06-09-phase11-rp1-dma-cache-milestone-11-3-closeout.md.
- Project contract:
  docs/src/project/phase11-rp1-pcie-map-contract.md.
- Roadmap:
  docs/src/roadmap.md.
- Evidence map:
  tasks/evidence/2026-06-09-phase11-rp1-hardware-substrate-closeout/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-09-phase11-rp1-hardware-substrate-closeout/classification.json.

## Validation

- static inspection: accepted Phase 11 task records, evidence
  maps/classification JSON, roadmap, project contract, and recent git history.
- JSON checks: jq empty on task-owned evidence-map/classification JSON.
- diff checks: git diff --check and git diff --cached --check.
- documentation build: /home/node/.cargo/bin/mdbook build.

## Acceptance

- Task record lists findings with disposition: satisfied.
- Checkpoint cites accepted Phase 11 evidence for RP1/PCIe mapping,
  interrupt/clock/GPIO frontiers, and DMA/cache Milestone 11.3 frontier:
  satisfied.
- Checkpoint states whether Phase 11 is closed for source-only Phase 12.1
  Ethernet research and names the exact next task id: satisfied.
- Checkpoint rejects live DMA, RP1 MMIO/DMA programming,
  descriptor-ring/channel ownership, transfer completion, interrupt
  completion, clock/reset ownership, GPIO/event ownership, Ethernet/storage
  readiness, networking, SSH, and Phase 12 implementation by implication:
  satisfied.
- Accepted closeout is committed before any Phase 12 task starts: satisfied by
  the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote phase12-rp1-ethernet-source-inventory-20260609 on the
next worker wake only if this closeout is accepted and committed. That task is
source-only research; it must not implement an Ethernet driver, run hardware,
program RP1 MMIO/DMA, create descriptor rings, perform packet I/O, build a
network stack, open sockets, or add SSH.
