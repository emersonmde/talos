# Phase 12 RP1 Ethernet Path ADR

Task: phase12-rp1-ethernet-path-adr-20260609

Status: accepted

Evidence level: static inspection of accepted Phase 12.1 source inventory,
retained Raspberry Pi Linux source excerpts, roadmap/project docs, and Phase
11 closeout evidence.

## Goal

Record the chosen initial RP1 Ethernet path, or a precise blocker, before any
Ethernet implementation or hardware diagnostic work starts.

## Scope

- Consume the accepted RP1 Ethernet source inventory and its selected
  nextAction for this ADR/design-note task.
- Decide between direct Cadence GEM work, no_std driver reuse, or a simpler
  staged transport using only accepted source and Talos project evidence.
- Name the next smallest follow-up only if it is mechanically objective from
  the accepted decision.
- Preserve non-goals against Ethernet implementation, hardware runs, live DMA,
  descriptor rings, packet I/O, networking, sockets, SSH, and Phase 12.2 work.
- Record findings with disposition.

## Non-Goals

- No Ethernet driver, packet TX/RX, network stack, sockets, SSH, hardware run,
  boot archive publication, hardwareTestLock acquisition, RP1 MMIO/DMA
  programming, descriptor-ring construction, PHY reset, clock/reset writes,
  interrupt enablement, or Phase 12.2 implementation.

## Retained Inputs

- tasks/2026-06-09-phase12-rp1-ethernet-source-inventory.md
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/classification.json
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/evidence-map.json
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-bcm2712-rpi-5-b.dts
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-cdns-macb.yaml
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c
- tasks/2026-06-09-phase11-rp1-hardware-substrate-closeout.md
- docs/src/project/phase12-networking-ssh.md
- docs/src/roadmap.md

## Decision

Select a direct RP1 Cadence GEM path, but stage it through hardware-substrate
proofs before any driver or packet behavior.

The selected path is:

1. Keep RP1 Ethernet as the target hardware path for Phase 12.1 rather than
   switching to a non-RP1/simple transport.
2. Do not start a Cadence GEM driver from the Linux MACB runtime shape yet.
   The Linux path is useful source evidence, but it depends on DMA descriptor
   rings, packet buffers, clocks, MDIO/PHY reset, phylink, interrupts, and
   completion handling that Talos has not accepted.
3. Do not pick a no_std GEM/MACB driver as the first implementation. Reuse can
   be reconsidered after Talos proves the hardware access and driver substrate
   boundaries, because importing driver logic before those boundaries would
   hide the same unaccepted MMIO, DMA, interrupt, clock, and PHY assumptions.
4. Treat live RP1 Ethernet MMIO as the first precise blocker. The accepted
   inventory records the source register path
   `RP1 0xc0_40100000 -> CPU 0x1f_0010_0000`, but accepted Phase 11 evidence
   does not prove broad RP1 Ethernet endpoint MMIO readiness.
5. Require a supervisor-planned source contract for a harmless
   `rp1_eth`/GEM visibility diagnostic before any implementation. That
   follow-up must choose an exact read-only register from retained or newly
   retained Cadence/RP1 sources, use a paired no-Ethernet/no-MMIO control, and
   explicitly reject descriptor rings, DMA, interrupts, PHY reset, packet I/O,
   networking, sockets, and SSH.

This ADR therefore chooses the hardware-first direct GEM path and blocks
implementation until a new explicit task defines the smallest Ethernet-MMIO
source contract. No queued worker task is mechanically unblocked by this ADR.

## Rationale

- `rp1_eth` is source-identified as `raspberrypi,rp1-gem` / `cdns,macb` with
  a specific RP1 bus register window, interrupt, clocks, RGMII PHY mode, and
  PHY reset GPIO. That is enough to choose the hardware target.
- The accepted Phase 11 closeout is only a substrate/research frontier. It
  retains blockers for endpoint config identity, bridge/outbound setup, live
  RP1 MMIO breadth, clock/reset ownership, GPIO/event ownership, descriptor
  rings, DMA/channel ownership, transfer completion, and interrupt completion.
- The Linux MACB/GEM driver shape argues against pretending that packet I/O is
  a small next step: even basic bring-up pulls in descriptor allocation,
  DMA-safe buffers, ring base programming, link-speed clock changes,
  MDIO/PHY reset, and interrupt/NAPI completion paths.
- A simpler non-RP1 transport would be easier to demo but would not retire the
  Phase 12.1 RP1 Ethernet unknowns. It can remain a fallback only if Matthew
  or the supervisor later changes the milestone objective.

## Alternatives Considered

- Implement a direct Cadence GEM driver now: rejected. It would rely on
  unaccepted RP1 Ethernet MMIO, descriptor-ring ownership, DMA/cache policy,
  interrupts, PHY reset, and clocks.
- Import or adapt a no_std Cadence GEM/MACB driver now: deferred. It may help
  later with register vocabulary or driver structure, but it cannot make the
  substrate assumptions true.
- Stage networking through a simpler non-RP1 transport: deferred. It may be a
  useful development-access fallback later, but it would bypass the explicit
  RP1 Ethernet research spike.
- Start with a packet-less read-only GEM visibility diagnostic: selected as
  the next kind of work, but not started here because no explicit queued task
  currently defines the exact register, evidence gates, or hardware contract.

## Rejected Claims

- Ethernet driver readiness.
- Live RP1 Ethernet MMIO readiness.
- RP1 MMIO/DMA programming.
- Descriptor-ring construction or ownership.
- DMA channel ownership, transfer completion, or interrupt completion.
- Clock/reset ownership or PHY reset ownership.
- Packet TX/RX, network stack, sockets, SSH, or Phase 12.2 implementation.
- Treating an imported driver or Linux source excerpt as accepted Talos runtime
  behavior.

## Findings

- fixed: selected the direct RP1 Cadence GEM hardware path as the Phase 12.1
  target, staged behind substrate proofs.
- fixed: rejected immediate driver implementation because source evidence ties
  MACB/GEM bring-up to unaccepted MMIO, DMA, descriptor, interrupt, clock, and
  PHY assumptions.
- fixed: rejected no_std driver reuse as the first implementation boundary
  while retaining it as a later structure/register-vocabulary option.
- fixed: rejected a simpler non-RP1 transport as the first path because it
  would not answer the RP1 Ethernet milestone.
- deferred: exact read-only GEM diagnostic register selection and task
  decomposition require supervisor planning with explicit acceptance criteria.
- deferred: all live MMIO, DMA, descriptor-ring, interrupt, PHY reset, packet
  I/O, networking, sockets, SSH, and Phase 12.2 work.
- not-an-issue: choosing a hardware-first direct GEM path is not an
  implementation claim; this ADR preserves the accepted non-goals.

No findings were removed.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- ADR/design note records the chosen Ethernet path or precise blocker, with
  tradeoffs grounded in accepted inventory: satisfied.
- ADR names the next smallest implementation or diagnostic task only if
  mechanically objective from the accepted decision: satisfied by declining to
  name a worker-owned task and requiring supervisor planning for an explicit
  source-contract diagnostic.
- ADR rejects Ethernet readiness, packet I/O, network stack, sockets, SSH,
  live DMA, descriptor rings, and Phase 12.2 work by implication: satisfied.
- Accepted ADR/design note is committed before any implementation or hardware
  diagnostic starts: satisfied by the task commit recorded in supervisor
  state after this task.

## Next Action

Set planningNeeded=true for supervisor planning of a bounded Phase 12.1 RP1
Ethernet MMIO source-contract diagnostic. No worker-owned follow-up task is
currently mechanically unblocked. The future task must be explicit about exact
register target, paired control, validation gates, evidence requirements, and
non-goals before any hardware or implementation work starts.
