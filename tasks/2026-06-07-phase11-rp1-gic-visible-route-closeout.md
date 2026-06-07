# Phase 11 RP1 GIC-Visible Route Closeout

Task id: phase11-rp1-gic-visible-route-closeout-20260607

Status: accepted

## Goal

Close out the GIC-visible route source/core/control/real diagnostic chain
without implying interrupt delivery, handler ownership, or a phase transition.

## Scope

- Reconciled the accepted source contract, local/static core,
  no-MMIO/no-GIC/no-RP1 control proof, and real Pi 5 diagnostic proof.
- Recorded findings with disposition.
- Named accepted and unaccepted claims for GIC-visible route status, capture
  and restore hygiene, interrupt delivery, GPIO ownership, clock/reset
  assumptions, and the next Milestone 11.2 step.
- Updated only roadmap/project contract docs for the accepted frontier.
- Set the next action to supervisor planning for the next Milestone 11.2
  feature slice; no worker-owned follow-up task is created here.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime source changes, GIC enable writes, GIC IAR/EOIR acknowledgement,
interrupt unmasking, ISR installation, GPIO ownership, pin-control or pad
writes, clock/reset programming, broad interrupt delivery/handler ownership,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, or phase transition.

## Reconciliation

The chain closes as gic-visible-route-status-frontier-closed.

- Source contract: phase11-rp1-gic-visible-route-source-contract-20260607
  accepted phase11-rp1-gic-visible-route-source-contract-v1, selecting a
  read-only/no-ack GICv2 status snapshot for the source-predicted RP1 IO_BANK0
  route to GIC SPI 128 / INTID 160. The selected reads are GICD_ISENABLER5 at
  0x10_7fff_9114, GICD_ISPENDR5 at 0x10_7fff_9214, GICD_ISACTIVER5 at
  0x10_7fff_9314, and GICC_HPPIR at 0x10_7fff_a018.
- Local/static core: phase11-rp1-gic-visible-route-diagnostic-core-20260607
  accepted a real candidate with only the contracted read-only/no-ack GICv2
  status reads and a paired control candidate that constructs no forbidden GIC,
  RP1, MSI-X, PCIe, MIP, GPIO, pads, RIO, or clock/reset MMIO path.
- Control proof:
  phase11-rp1-gic-visible-route-no-mmio-control-pi5-20260607 accepted the
  no-MMIO/no-GIC/no-RP1 output shape as visible on Pi 5 after v2 identity join,
  two 47,040-byte candidate TFTP fetches, stable pre-restore TFTP evidence,
  final selected-tree identity, and restore proof.
- Real proof: phase11-rp1-gic-visible-route-diagnostic-pi5-20260607 accepted
  gic-route-status-visible. The decisive rerun passed v2 identity join for
  tree 8ef75b3125c21d7025cff539f5004d7f6911af057c5523ce1610be46deecbbe4,
  retained two 47,816-byte da591740/kernel_2712.img fetches, final
  selected-tree identity, restore proof, and 209
  TALOS: rp1-gic-route-status-result records. The visible result reported
  contract phase11-rp1-gic-visible-route-source-contract-v1, target
  rp1-io-bank0-gic-route-status-read, predicted GIC SPI 128 / INTID 160,
  GICD_ISENABLER5/GICD_ISPENDR5/GICD_ISACTIVER5 raw values of 0x0,
  INTID 160 enabled=false, pending=false, active=false, GICC_HPPIR raw 0x3ff,
  HPPIR INTID 1023, hppir-spurious=true, and
  classification=gic-route-status-visible.

## Findings And Disposition

- fixed: closed the source/core/control/real proof chain as an accepted
  read-only/no-ack GIC-visible route status frontier.
- fixed: retained the paired no-MMIO/no-GIC/no-RP1 control requirement as
  satisfied before accepting the real diagnostic proof.
- fixed: retained the rejected first real run, known-good control, and decisive
  candidate rerun as capture hygiene evidence; acceptance uses only the
  decisive identity-joined rerun.
- fixed: updated docs to record the accepted boundary and retained risks.
- deferred: interrupt pending generation, interrupt delivery, IAR/EOIR
  acknowledgement, ISR/handler ownership, GPIO ownership, pin-control
  behavior, clock/reset programming, DMA/cache, storage, generated-root,
  networking, SSH, broader PCIe enumeration, Milestone 11.3, and phase
  transition remain future work.
- not-an-issue: GICD enable/pending/active bits all clear and HPPIR returning
  1023/spurious is still a valid read-only/no-ack status snapshot; it is not
  treated as proof of delivered interrupts.

No findings were removed in this closeout task.

## Accepted Claims

Accepted frontier: gic-visible-route-status-frontier-closed.

The accepted boundary is limited to the source-backed RP1 IO_BANK0 route
identity, the selected read-only/no-ack GICv2 status snapshot for INTID 160,
the local real/control candidate split, the no-MMIO/no-GIC/no-RP1 control
output proof, and the real Pi 5 visibility proof for the selected diagnostic
status result.

## Retained Risks And Unaccepted Claims

The closeout does not accept interrupt pending generation, interrupt delivery,
IAR/EOIR acknowledgement, ISR/handler ownership, Talos-owned GPIO state,
pin-control behavior, pad writes, clock/reset programming, DMA/cache behavior,
storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, or a phase transition.

The clear INTID 160 enable/pending/active bits and spurious HPPIR result are
retained as the accepted read-only GIC status observation for this boundary,
not as evidence that the source is unrouteable or that Talos owns the interrupt
delivery path.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-closeout/evidence-map.json.
- Source contract evidence:
  tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-source-contract/evidence-map.json.
- Local/static core evidence:
  tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-diagnostic-core/evidence-map.json.
- Control proof evidence:
  tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-no-mmio-control-pi5/evidence-map.json.
- Real proof evidence:
  tasks/evidence/2026-06-07-phase11-rp1-gic-visible-route-diagnostic-pi5/evidence-map.json.

## Validation

- Static inspection: source contract, core, no-MMIO/no-GIC/no-RP1 control
  proof, real proof, and evidence maps inspected.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

Accepted as gic-visible-route-status-frontier-closed.

No explicit worker-owned task remains in this queue. Supervisor planning is
required for the next Milestone 11.2 feature slice. Same-shaped read-only/no-ack
GIC-visible route status hardware reruns are not progress unless a future
supervisor task supplies a different discriminator or new acceptance criteria.
