# Phase 11 RP1 Clock/Reset Write/Restore Closeout

Task id: phase11-rp1-clock-reset-write-restore-closeout-20260607

Status: accepted

## Goal

Close out the reversible RP1 clock/reset write/readback/restore chain without
implying broad clock/reset ownership, GPIO ownership, interrupt delivery,
handler ownership, or a phase transition.

## Scope

- Reconciled the accepted source contract, local/static core,
  no-MMIO/no-RP1/no-GIC control proof, real Pi 5 proof, restore evidence, and
  evidence maps.
- Recorded findings with disposition.
- Named accepted and unaccepted claims for the CLK_ADC_CTRL idempotent
  write/readback/restore boundary, clock/reset ownership, GPIO ownership, event
  generation, interrupt delivery, handler ownership, DMA/cache, storage,
  generated-root, networking, SSH, broader PCIe, and the next Milestone 11.2
  step.
- Updated only roadmap/project contract docs for the accepted frontier.
- Set planningNeeded=true for supervisor planning; no worker-owned follow-up
  task is created here.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime source changes, additional clock/reset writes, GPIO/RIO/pad writes,
event generation, interrupt enablement or delivery, GIC IAR/EOIR
acknowledgement, ISR installation, broad clock/reset driver ownership,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, or phase transition.

## Reconciliation

The chain closes as rp1-clock-adc-ctrl-write-restore-frontier-closed.

- Source contract:
  phase11-rp1-clock-reset-write-restore-source-contract-20260607 accepted
  phase11-rp1-clock-reset-write-restore-source-contract-v1, selecting a
  bounded idempotent CLK_ADC_CTRL write/readback/restore proof at CPU
  physical 0x1f00018144. The only real-candidate operations are pre-read,
  write back the pre-read raw value, post-read, restore-write the pre-read raw
  value, and restore-read. The accepted unchanged fields are the full raw
  value, enable bit, auxsrc bits, and source bits.
- Local/static core:
  phase11-rp1-clock-reset-write-restore-core-20260607 accepted a real
  candidate implementing only the contracted CLK_ADC_CTRL sequence plus a
  paired no-MMIO/no-RP1/no-GIC control candidate with the same output shape and
  no forbidden RP1 clock/reset, GPIO/RIO/pads, MSI-X/PCIe/MIP, or GIC MMIO
  construction.
- Control proof:
  phase11-rp1-clock-reset-write-restore-control-pi5-20260607 accepted the
  no-MMIO/no-RP1/no-GIC control output shape as visible on Pi 5 after v2
  identity join, two 46,888-byte candidate TFTP fetches, 108 control markers,
  final selected-tree identity, and restore proof.
- Real proof:
  phase11-rp1-clock-reset-write-restore-pi5-20260607 accepted
  rp1-clock-adc-ctrl-idempotent-write-restored. The decisive rerun passed v2
  identity join for tree
  3ea80fee925c554e0e65141bbd18174ab661b3e5ac6a73b82d7c130ca7adb709,
  retained two 47,232-byte da591740/kernel_2712.img fetches, final
  selected-tree identity, restore proof, and 102
  TALOS: rp1-clock-adc-ctrl-write-restore-result records. The visible result
  reported pre-raw=0xdeaddead, post-raw=0xdeaddead,
  restore-raw=0xdeaddead, post-eq-pre=true, and restore-eq-pre=true. The lab
  was finally restored to the original pre-run tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: closed the source/core/control/real proof chain as an accepted
  reversible CLK_ADC_CTRL idempotent write/readback/restore frontier with
  retained identity-joined Pi 5 evidence.
- fixed: retained the paired no-MMIO/no-RP1/no-GIC control proof as satisfied
  before accepting the real diagnostic proof.
- fixed: retained the rejected first real run, known-good control, decisive
  candidate rerun, identity join, stable TFTP evidence, and restore proof as
  capture hygiene evidence; acceptance uses only the decisive identity-joined
  rerun.
- fixed: updated roadmap and project contract docs for the accepted
  reversible clock-manager write/restore boundary.
- deferred: later GPIO ownership retry, interrupt-delivery work, broader
  clock/reset ownership, non-idempotent clock programming, reset-controller
  writes, event generation, handler ownership, DMA/cache, storage,
  generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3,
  and phase transition require supervisor planning with new acceptance
  criteria.
- not-an-issue: the observed 0xdeaddead raw value is accepted only as the raw
  value preserved by this selected idempotent write/restore proof; it is not
  interpreted as broad RP1 clock/reset state.

No findings were removed in this closeout task.

## Accepted Claims

Accepted frontier: rp1-clock-adc-ctrl-write-restore-frontier-closed.

The accepted boundary is limited to the source-backed CLK_ADC_CTRL idempotent
write/readback/restore contract, the local real/control candidate split, the
paired no-MMIO/no-RP1/no-GIC control output proof, and the real Pi 5 visibility
proof that the selected write-back and restore-read returned the pre-read raw
value for this run.

This is the first accepted reversible RP1 clock-manager write/restore boundary.
It may be used by a future supervisor-planned GPIO ownership retry or
interrupt-delivery slice only as evidence that this selected idempotent
clock-manager write/restore discipline worked; it does not license additional
clock/reset writes by implication.

## Retained Risks And Unaccepted Claims

The closeout does not accept broad RP1 clock/reset ownership, non-idempotent
clock enable/disable/divider/source/PLL programming, reset-controller writes,
GPIO ownership, GPIO event generation, interrupt pending generation,
interrupt delivery, GIC IAR/EOIR acknowledgement, ISR/handler ownership,
DMA/cache behavior, storage, generated-root, networking, SSH, broader PCIe
enumeration, Milestone 11.3, or a phase transition.

A future task must supply a new source contract and acceptance criteria before
any GPIO ownership retry, interrupt-delivery work, non-idempotent clock/reset
operation, reset-controller path, or broader RP1 clock/reset driver ownership.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-write-restore-closeout/evidence-map.json.
- Source contract evidence:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-write-restore-source-contract/evidence-map.json.
- Local/static core evidence:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-write-restore-core/evidence-map.json.
- Control proof evidence:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-write-restore-control-pi5/evidence-map.json.
- Real proof evidence:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-write-restore-pi5/evidence-map.json.

## Validation

- Static inspection: source contract, core, no-MMIO/no-RP1/no-GIC control
  proof, real proof, restore evidence, and evidence maps inspected.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

Accepted as rp1-clock-adc-ctrl-write-restore-frontier-closed.

No explicit worker-owned task remains in this queue. Supervisor planning is
required for the next Milestone 11.2 feature slice. GPIO ownership retries,
interrupt-delivery work, non-idempotent clock/reset operations, and broader
RP1 clock/reset driver ownership are not progress unless a future supervisor
task supplies a new source contract and acceptance criteria.
