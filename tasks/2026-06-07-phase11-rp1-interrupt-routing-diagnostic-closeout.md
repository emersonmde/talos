# Phase 11 RP1 Interrupt-Routing Diagnostic Closeout

Task id: phase11-rp1-interrupt-routing-diagnostic-closeout-20260607

Status: accepted

## Goal

Close out the interrupt-routing source/core/control/real diagnostic chain and
decide the next Milestone 11.2 frontier without implying a phase transition.

## Scope

- Reconciled the accepted source contract, local/static core, no-MMIO/no-enable
  control proof, and real Pi 5 diagnostic proof.
- Recorded findings with disposition.
- Named accepted and unaccepted claims for interrupt routing, GPIO ownership,
  pin-control, clock/reset assumptions, capture/restore hygiene, and the next
  Milestone 11.2 step.
- Updated only roadmap/project contract docs for the accepted frontier.
- Set the next action to supervisor planning for the next Milestone 11.2
  feature slice; no worker-owned follow-up task is created here.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime source changes, GPIO ownership, pin-control or pad writes, clock/reset
programming, broad interrupt delivery/handler ownership, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3, or
phase transition.

## Reconciliation

The chain closes as interrupt-routing-msix-cfg-read-frontier-closed.

- Source contract: phase11-rp1-interrupt-routing-source-contract-20260607
  accepted phase11-rp1-interrupt-routing-source-contract-v1, selecting a
  read-only/no-enable 32-bit load from RP1 MSIX_CFG(0) at CPU physical
  0x1f00108008. Source inspection predicts RP1 hwirq 0 through PCI MSI-X
  vector 0 and MIP0 MSI vector 0 to GIC SPI 128 / INTID 160.
- Local/static core: phase11-rp1-interrupt-routing-diagnostic-core-20260607
  accepted a real candidate with exactly one contracted volatile load and a
  paired control candidate that constructs no forbidden RP1 interrupt, GPIO,
  pads, RIO, clock/reset, MSI-X, PCIe config, MIP, or GIC MMIO path.
- Control proof:
  phase11-rp1-interrupt-routing-no-mmio-control-pi5-20260607 accepted the
  no-MMIO/no-enable output shape as visible on Pi 5 after v2 identity join,
  two 46,520-byte candidate TFTP fetches, stable pre-restore TFTP evidence,
  final selected-tree identity, and restore proof.
- Real proof: phase11-rp1-interrupt-routing-diagnostic-pi5-20260607
  accepted routing-msix-cfg-visible. The decisive rerun passed v2 identity
  join for tree
  63800845c9837b3d57153051583b269070b028412bcd57ea9c55a5f9e56a2304,
  retained two 46,648-byte da591740/kernel_2712.img fetches, final
  selected-tree identity, restore proof, and 970
  TALOS: rp1-interrupt-routing-result records carrying contract
  phase11-rp1-interrupt-routing-source-contract-v1, target
  rp1-io-bank0-msix-cfg-read, address 0x1f00108008, raw 0xdeaddead,
  and classification=routing-msix-cfg-visible.

## Findings And Disposition

- fixed: closed the source/core/control/real proof chain as an accepted
  read-only/no-enable RP1 MSIX_CFG(0) visibility frontier.
- fixed: retained the paired control requirement as satisfied before accepting
  the real diagnostic proof.
- fixed: retained the rejected first real run and known-good control as
  capture hygiene evidence; acceptance uses only the decisive identity-joined
  rerun.
- fixed: updated docs to record the accepted boundary and retained risks.
- deferred: interrupt enablement/delivery, handler ownership, GPIO ownership,
  pin-control behavior, pad writes, clock/reset programming, DMA/cache,
  storage, generated-root, networking, SSH, broader PCIe enumeration,
  Milestone 11.3, and phase transition remain future work.
- not-an-issue: raw 0xdeaddead is an accepted observed diagnostic result for
  this boundary only; it is not treated as proof that interrupts are enabled,
  routed to a handler, or owned by Talos.

No findings were removed in this closeout task.

## Accepted Claims

Accepted frontier: interrupt-routing-msix-cfg-read-frontier-closed.

The accepted boundary is limited to the source-backed RP1 IO_BANK0 interrupt
identity, the selected read-only/no-enable MSIX_CFG(0) diagnostic at
0x1f00108008, the local real/control candidate split, the no-MMIO/no-enable
control output proof, and the real Pi 5 visibility proof for the selected
diagnostic result.

## Retained Risks And Unaccepted Claims

The closeout does not accept interrupt delivery, ISR/handler ownership,
Talos-owned GPIO state, pin-control behavior, pad writes, clock/reset
programming, DMA/cache behavior, storage, generated-root, networking, SSH,
broader PCIe enumeration, Milestone 11.3, or a phase transition.

The source-predicted hwirq/MSI-X/GIC route is retained as the source reference
for the next Milestone 11.2 slice, not as proof of delivered interrupts.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-diagnostic-closeout/evidence-map.json.
- Source contract evidence:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-source-contract/evidence-map.json.
- Local/static core evidence:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-diagnostic-core/evidence-map.json.
- Control proof evidence:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-no-mmio-control-pi5/evidence-map.json.
- Real proof evidence:
  tasks/evidence/2026-06-07-phase11-rp1-interrupt-routing-diagnostic-pi5/evidence-map.json.

## Validation

- Static inspection: source contract, core, no-MMIO/no-enable control proof,
  real proof, and evidence maps inspected.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

Accepted as interrupt-routing-msix-cfg-read-frontier-closed.

No explicit worker-owned task remains in this queue. Supervisor planning is
required for the next Milestone 11.2 feature slice. Same-shaped
read-only/no-enable MSIX_CFG(0) hardware reruns are not progress unless a
future supervisor task supplies a different discriminator or new acceptance
criteria.
