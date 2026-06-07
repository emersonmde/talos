# Phase 11 RP1 GPIO Ownership/Restore Closeout

Task id: phase11-rp1-gpio-ownership-restore-closeout-20260607

Status: accepted

## Goal

Close out the GPIO ownership/restore source/core/control/real diagnostic chain
without implying GPIO ownership, event generation, interrupt delivery, handler
ownership, or a phase transition.

## Scope

- Reconciled the accepted source contract, local/static core,
  no-MMIO/no-RP1/no-GIC control proof, and real Pi 5 preflight proof.
- Recorded findings with disposition.
- Named accepted and unaccepted claims for GPIO14 ownership/route preflight
  visibility, the non-GPIO-function blocker, capture and restore hygiene,
  event-generation readiness, and the next Milestone 11.2 step.
- Updated only roadmap/project contract docs for the accepted frontier.
- Set the next action to supervisor planning; no worker-owned follow-up task is
  created here.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime source changes, GPIO CTRL writes, IO_BANK0 INTE writes, IRQRESET
acknowledgement, RIO or pad writes, parent-route masking writes, GPIO event
generation, interrupt pending generation beyond the read-only snapshot,
interrupt enablement or delivery, GIC IAR/EOIR acknowledgement, ISR
installation, broad GPIO driver ownership, clock/reset programming, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe enumeration, Milestone
11.3, or phase transition.

## Reconciliation

The chain closes as gpio14-ownership-preflight-blocked-frontier-closed.

- Source contract:
  phase11-rp1-gpio-ownership-restore-source-contract-20260607 accepted
  phase11-rp1-gpio-ownership-restore-source-contract-v1, selecting a
  read-only GPIO14 ownership/route preflight before any event-generation
  retry. The selected reads are GPIO14 STATUS/CTRL, IO_BANK0 INTE/INTS, RIO0
  OUT/OE/IN, GPIO14 pad control, and the accepted INTID 160 GIC route status
  registers. No writes are allowed by the contract.
- Local/static core:
  phase11-rp1-gpio-ownership-restore-core-20260607 accepted a real candidate
  with only the contracted read-only preflight reads and a paired control
  candidate that constructs no forbidden RP1/GIC/MSI-X/PCIe/MIP/GPIO/pads/RIO
  or clock/reset MMIO path.
- Control proof:
  phase11-rp1-gpio-ownership-restore-control-pi5-20260607 accepted the
  no-MMIO/no-RP1/no-GIC output shape as visible on Pi 5 after v2 identity join,
  556 control markers, two 48,368-byte candidate TFTP fetches, stable
  pre-restore TFTP evidence, final selected-tree identity, and restore proof.
- Real proof:
  phase11-rp1-gpio-ownership-restore-pi5-20260607 accepted
  gpio14-ownership-preflight-blocked-non-gpio-function. The decisive rerun
  passed v2 identity join for tree
  91372af6aeecc90b47b57d6d3f1caf46ee5b20f47ec392977fdae2674ac0112f,
  retained two 50,056-byte da591740/kernel_2712.img fetches, final
  selected-tree identity, restore proof, and 93
  TALOS: rp1-gpio14-ownership-route-preflight-result records. The visible
  result reported GPIO14 fsel 13 / func-name unknown, RIO GPIO14 out/oe/in
  true, pad input disabled and output disabled, INTID160 enabled=false,
  pending=false, active=false, HPPIR INTID 1023, and
  classification=gpio14-ownership-preflight-blocked-non-gpio-function.

## Findings And Disposition

- fixed: closed the source/core/control/real proof chain as a read-only
  GPIO14 ownership/route preflight frontier with a retained blocker.
- fixed: retained the paired no-MMIO/no-RP1/no-GIC control proof as satisfied
  before accepting the real Pi 5 preflight proof.
- fixed: retained the rejected first real run, known-good control, and decisive
  candidate rerun as capture hygiene evidence; acceptance uses only the
  decisive identity-joined rerun.
- fixed: updated docs to record the accepted boundary and retained blocker.
- deferred: any event-generation or pending-generation retry requires new
  supervisor planning around GPIO14 ownership/function selection,
  parent-route masking, deterministic event source, partial-write recovery, and
  restore semantics.
- not-an-issue: the raw 0xdeaddead snapshot values remain accepted only as
  visible read-only diagnostic data at the contracted addresses; they are not
  treated as GPIO ownership, event generation, interrupt delivery, or handler
  ownership.

No findings were removed in this closeout task.

## Accepted Claims

Accepted frontier: gpio14-ownership-preflight-blocked-frontier-closed.

The accepted boundary is limited to the source-backed GPIO14 ownership/route
preflight register identity, the selected read-only preflight snapshot, the
local real/control candidate split, the no-MMIO/no-RP1/no-GIC control output
proof, and the real Pi 5 visibility proof that blocks later GPIO14
event-generation work because GPIO14 reported fsel 13 / unknown function.

## Retained Risks And Unaccepted Claims

The closeout does not accept GPIO ownership, GPIO event generation, interrupt
pending generation beyond the read-only snapshot, interrupt enablement,
interrupt delivery, IAR/EOIR acknowledgement, ISR/handler ownership,
Talos-owned GPIO state, GPIO CTRL/INTE/RIO/pad writes, parent-route masking
writes, clock/reset programming, DMA/cache behavior, storage, generated-root,
networking, SSH, broader PCIe enumeration, Milestone 11.3, or a phase
transition.

The GPIO14 fsel 13 / unknown-function result is retained as a blocker for a
same-shaped event-latch or event-generation hardware retry. A future task must
use a different discriminator or explicit supervisor-planned ownership,
masking, deterministic event-source, and restore acceptance criteria.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-closeout/evidence-map.json.
- Source contract evidence:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-source-contract/evidence-map.json.
- Local/static core evidence:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-core/evidence-map.json.
- Control proof evidence:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-control-pi5/evidence-map.json.
- Real proof evidence:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-pi5/evidence-map.json.

## Validation

- Static inspection: source contract, core, no-MMIO/no-RP1/no-GIC control
  proof, real proof, and evidence maps inspected.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

Accepted as gpio14-ownership-preflight-blocked-frontier-closed.

No explicit worker-owned task remains in this queue. Supervisor planning is
required for the next Milestone 11.2 feature slice. Same-shaped GPIO
ownership/route preflight, event-latch, or event-generation hardware reruns
are not progress unless a future supervisor task supplies a different
discriminator or new acceptance criteria.
