# Phase 11 RP1 GPIO Bank Source-Status Closeout

Task id: phase11-rp1-gpio-bank-source-status-closeout-20260607

Status: accepted

## Goal

Close out the GPIO bank source-status source/core/control/real diagnostic chain
without implying GPIO event generation, interrupt delivery, handler ownership,
or a phase transition.

## Scope

- Reconciled the accepted source contract, local/static core,
  no-MMIO/no-RP1/no-GIC control proof, and real Pi 5 diagnostic proof.
- Recorded findings with disposition.
- Named accepted and unaccepted claims for GPIO bank source-status visibility,
  capture and restore hygiene, interrupt delivery, GPIO ownership, clock/reset
  assumptions, and the next Milestone 11.2 step.
- Updated only roadmap/project contract docs for the accepted frontier.
- Set the next action to supervisor planning for the next Milestone 11.2
  feature slice; no worker-owned follow-up task is created here.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime source changes, GPIO event generation, GPIO interrupt enablement,
GPIO CTRL or IRQRESET writes, MSI-X enable/IACK writes, PCIe MIP or GIC
writes, GIC IAR/EOIR acknowledgement, interrupt unmasking, ISR installation,
GPIO ownership, pin-control or pad writes, clock/reset programming, broad
interrupt delivery/handler ownership, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe enumeration, Milestone 11.3, or phase
transition.

## Reconciliation

The chain closes as gpio-bank-source-status-frontier-closed.

- Source contract:
  phase11-rp1-gpio-bank-source-status-contract-20260607 accepted
  phase11-rp1-gpio-bank-source-status-contract-v1, selecting a
  read-only/non-destructive IO_BANK0 source-status snapshot. The selected
  reads are IO_BANK0 INTS at CPU physical 0x1f000d0124 and companion IO_BANK0
  INTE at CPU physical 0x1f000d011c, both 32-bit volatile loads. Bank0 covers
  GPIO0 through GPIO27; GPIO14 is mask 0x00004000.
- Local/static core:
  phase11-rp1-gpio-bank-source-status-core-20260607 accepted a real candidate
  with only the contracted INTE/INTS reads and a paired control candidate that
  constructs no forbidden RP1 GPIO/RIO/pads/clock/reset, MSI-X/PCIe/MIP, or
  GIC MMIO path.
- Control proof:
  phase11-rp1-gpio-bank-source-status-control-pi5-20260607 accepted the
  no-MMIO/no-RP1/no-GIC output shape as visible on Pi 5 after v2 identity join,
  two 46,832-byte candidate TFTP fetches, stable pre-restore TFTP evidence,
  final selected-tree identity, and restore proof.
- Real proof:
  phase11-rp1-gpio-bank-source-status-pi5-20260607 accepted
  gpio-bank-source-status-visible. The decisive rerun passed v2 identity join
  for tree 84ee89db45d5298e49f44c74e6a18b9c07ce2c146879f677aceace6ad252ea0f,
  retained two 46,904-byte da591740/kernel_2712.img fetches, final
  selected-tree identity, restore proof, and 269
  TALOS: rp1-gpio-bank-source-status-result records. The visible result
  reported contract phase11-rp1-gpio-bank-source-status-contract-v1, target
  rp1-io-bank0-source-status-read, source hwirq 0, bank0 GPIO0..GPIO27,
  IO_BANK0 INTE 0x1f000d011c raw 0xdeaddead, IO_BANK0 INTS 0x1f000d0124 raw
  0xdeaddead, gpio14-enabled=true, gpio14-source-status=true,
  source-status-nonzero=true, and classification=gpio-bank-source-status-visible.

## Findings And Disposition

- fixed: closed the source/core/control/real proof chain as an accepted
  read-only GPIO bank source-status frontier.
- fixed: retained the paired no-MMIO/no-RP1/no-GIC control requirement as
  satisfied before accepting the real diagnostic proof.
- fixed: retained the rejected first real run, known-good control, and decisive
  candidate rerun as capture hygiene evidence; acceptance uses only the
  decisive identity-joined rerun.
- fixed: updated docs to record the accepted boundary and retained risks.
- deferred: GPIO event generation, interrupt pending generation beyond the
  read-only snapshot, interrupt enablement, interrupt delivery, IAR/EOIR
  acknowledgement, ISR/handler ownership, GPIO ownership, pin-control behavior,
  clock/reset programming, DMA/cache, storage, generated-root, networking, SSH,
  broader PCIe enumeration, Milestone 11.3, and phase transition remain future
  work.
- not-an-issue: raw 0xdeaddead INTE/INTS values are accepted only as visible
  read-only diagnostic snapshot data at the contracted addresses; they are not
  treated as GPIO event generation, interrupt delivery, GPIO ownership, or
  handler ownership.

No findings were removed in this closeout task.

## Accepted Claims

Accepted frontier: gpio-bank-source-status-frontier-closed.

The accepted boundary is limited to the source-backed RP1 IO_BANK0 INTE/INTS
register identity, the selected read-only/non-destructive source-status
snapshot, the local real/control candidate split, the no-MMIO/no-RP1/no-GIC
control output proof, and the real Pi 5 visibility proof for the selected
diagnostic status result.

## Retained Risks And Unaccepted Claims

The closeout does not accept GPIO event generation, interrupt pending
generation beyond the read-only snapshot, interrupt enablement, interrupt
delivery, IAR/EOIR acknowledgement, ISR/handler ownership, Talos-owned GPIO
state, pin-control behavior, pad writes, clock/reset programming, DMA/cache
behavior, storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, or a phase transition.

The raw INTE/INTS values are retained as the accepted read-only source-status
observation for this boundary, not as evidence that Talos owns GPIO state or
the interrupt delivery path.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-closeout/evidence-map.json.
- Source contract evidence:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-contract/evidence-map.json.
- Local/static core evidence:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-core/evidence-map.json.
- Control proof evidence:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-control-pi5/evidence-map.json.
- Real proof evidence:
  tasks/evidence/2026-06-07-phase11-rp1-gpio-bank-source-status-pi5/evidence-map.json.

## Validation

- Static inspection: source contract, core, no-MMIO/no-RP1/no-GIC control
  proof, real proof, and evidence maps inspected.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

Accepted as gpio-bank-source-status-frontier-closed.

No explicit worker-owned task remains in this queue. Supervisor planning is
required for the next Milestone 11.2 feature slice. Same-shaped read-only GPIO
bank source-status hardware reruns are not progress unless a future supervisor
task supplies a different discriminator or new acceptance criteria.
