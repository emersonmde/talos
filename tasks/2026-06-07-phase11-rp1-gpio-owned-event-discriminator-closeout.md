# Phase 11 RP1 GPIO Owned Event Discriminator Closeout

Task id: phase11-rp1-gpio-owned-event-discriminator-closeout-20260607

Status: accepted

## Goal

Close out the Talos-owned RP1 GPIO16 event/pending discriminator chain without
implying interrupt delivery, handler ownership, broad GPIO ownership, or a
phase transition.

## Scope

- Reconciled the accepted source contract, local/static core,
  no-MMIO/no-RP1/no-GIC control proof, and real Pi 5 blocker proof.
- Recorded findings with disposition.
- Named accepted and unaccepted claims for GPIO16 ownership/function preflight,
  event/pending generation, source status, restore semantics, parent-route
  containment, interrupt delivery, handler ownership, clock/reset assumptions,
  and the next Milestone 11.2 step.
- Updated only roadmap/project contract docs for the accepted blocker
  frontier.
- Set the next action to supervisor planning; no worker-owned follow-up task is
  created here.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime source changes, new GPIO writes, interrupt delivery, GIC IAR/EOIR
acknowledgement, ISR installation, broad GPIO driver ownership, clock/reset
programming, DMA/cache, storage, generated-root, networking, SSH, broader PCIe
enumeration, Milestone 11.3, or phase transition.

## Reconciliation

The chain closes as gpio16-owned-event-preflight-blocked-frontier-closed.

- Source contract:
  phase11-rp1-gpio-owned-event-discriminator-source-contract-20260607 accepted
  phase11-rp1-gpio-owned-event-discriminator-source-contract-v1, selecting a
  source-backed GPIO16 level-high event/source-status discriminator after the
  GPIO14 non-GPIO-function blocker. The contract named GPIO16 STATUS/CTRL,
  IO_BANK0 INTE/INTS, RIO0 OUT/OE/IN, GPIO16 pad control, and INTID 160
  GIC-visible status reads, with bounded GPIO16 pad/CTRL/RIO/event-enable/
  IRQRESET/IO_BANK0-INTE bit-16 writes allowed only after parent-route
  containment and with exact restore semantics.
- Local/static core:
  phase11-rp1-gpio-owned-event-discriminator-core-20260607 accepted a real
  candidate implementing only the contracted GPIO16 discriminator and restore
  path, plus a paired no-MMIO/no-RP1/no-GIC control candidate that constructs
  no forbidden RP1/GIC/MSI-X/PCIe/MIP/GPIO/RIO/pad/clock/reset MMIO path.
- Control proof:
  phase11-rp1-gpio-owned-event-discriminator-control-pi5-20260607 accepted the
  no-MMIO/no-RP1/no-GIC control output shape as visible on Pi 5 after v2
  identity join, 40 control markers, two 49,480-byte candidate TFTP fetches,
  stable pre-restore TFTP evidence, final selected-tree identity, and restore
  proof.
- Real proof:
  phase11-rp1-gpio-owned-event-discriminator-pi5-20260607 accepted
  gpio16-owned-event-preflight-blocked-pin-function. The decisive rerun passed
  v2 identity join for tree
  348b127402b41ca3115ed09aa2e55cc2dce837dc04a7e4770f0143bd17e4c61c,
  retained two 52,056-byte da591740/kernel_2712.img fetches, final
  selected-tree identity, restore proof, and 38
  TALOS: rp1-gpio16-owned-event-discriminator-result records. The visible
  result reported GPIO16 fsel 13 / unknown function, action-skipped=true, and
  classification=gpio16-owned-event-preflight-blocked-pin-function.

## Findings And Disposition

- fixed: closed the source/core/control/real proof chain as a GPIO16
  event-discriminator blocker frontier with retained identity-joined Pi 5
  evidence.
- fixed: retained the paired no-MMIO/no-RP1/no-GIC control proof as satisfied
  before accepting the real Pi 5 blocker proof.
- fixed: retained the rejected first real run, known-good control, and
  decisive candidate rerun as capture hygiene evidence; acceptance uses only
  the decisive identity-joined rerun.
- fixed: updated docs to record the accepted boundary and retained blocker.
- deferred: any GPIO event/pending-generation retry requires new supervisor
  planning around a different discriminator or source-backed ownership/function
  strategy that can avoid the observed GPIO16 fsel 13 / unknown-function
  blocker.
- not-an-issue: the accepted result is a contract blocker classification, not
  interrupt-delivery evidence or proof that Talos owns GPIO16.

No findings were removed in this closeout task.

## Accepted Claims

Accepted frontier: gpio16-owned-event-preflight-blocked-frontier-closed.

The accepted boundary is limited to the source-backed GPIO16 discriminator
contract, the local real/control candidate split, the paired no-MMIO/no-RP1/
no-GIC control output proof, and the real Pi 5 visibility proof that blocks
later GPIO16 event-generation work because GPIO16 reported fsel 13 / unknown
function before any accepted action writes.

## Retained Risks And Unaccepted Claims

The closeout does not accept GPIO16 event generation, interrupt pending
generation, interrupt enablement, interrupt delivery, GIC IAR/EOIR
acknowledgement, ISR/handler ownership, broad GPIO driver ownership,
Talos-owned GPIO state, GPIO16 action writes on hardware, clock/reset
programming, DMA/cache behavior, storage, generated-root, networking, SSH,
broader PCIe enumeration, Milestone 11.3, or a phase transition.

The GPIO16 fsel 13 / unknown-function result blocks same-shaped GPIO16
event-generation hardware reruns. A future task must choose a different
discriminator or supply new source-backed ownership/function acceptance
criteria before reopening GPIO event/pending generation.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-closeout/evidence-map.json.
- Source contract evidence:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-source-contract/evidence-map.json.
- Local/static core evidence:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-core/evidence-map.json.
- Control proof evidence:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-control-pi5/evidence-map.json.
- Real proof evidence:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-pi5/evidence-map.json.

## Validation

- Static inspection: source contract, core, no-MMIO/no-RP1/no-GIC control
  proof, real proof, and evidence maps inspected.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

Accepted as gpio16-owned-event-preflight-blocked-frontier-closed.

No explicit worker-owned task remains in this queue. Supervisor planning is
required for the next Milestone 11.2 feature slice. Same-shaped GPIO16
event-discriminator hardware reruns are not progress unless a future
supervisor task supplies a different discriminator or new acceptance criteria.
