# Phase 11 RP1 Clock ADC Enable Toggle Closeout

Task id: phase11-rp1-clock-adc-enable-toggle-closeout-20260607

Status: accepted

Classification: rp1-clock-adc-ctrl-enable-toggle-mismatch-restored-frontier-closed

## Goal

Close out the reversible CLK_ADC_CTRL enable-bit transition chain without
implying successful non-idempotent clock ownership, broad clock/reset
ownership, GPIO ownership, interrupt delivery, handler ownership, or a phase
transition.

## Scope

- Reconciled the accepted source contract, local/static core,
  no-MMIO/no-RP1/no-GIC control proof, real Pi 5 proof, restore evidence, and
  evidence maps.
- Recorded findings with disposition.
- Named accepted and unaccepted claims for the selected CLK_ADC_CTRL
  enable-bit transition attempt, restore discipline, broad clock/reset
  ownership, GPIO ownership, event generation, interrupt delivery, handler
  ownership, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
  and the next Milestone 11.2 step.
- Updated roadmap/project contract docs only for the accepted closeout
  frontier.
- Set planningNeeded=true for supervisor planning; no worker-owned follow-up
  task is created here.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime source changes, additional clock/reset writes, reset-controller writes,
GPIO/RIO/pad writes, event generation, interrupt enablement or delivery, GIC
IAR/EOIR acknowledgement, ISR installation, broad clock/reset driver
ownership, DMA/cache, storage, generated-root, networking, SSH, broader PCIe
enumeration, Milestone 11.3, or phase transition.

## Reconciliation

The chain closes as
rp1-clock-adc-ctrl-enable-toggle-mismatch-restored-frontier-closed.

- Source contract:
  phase11-rp1-clock-adc-enable-toggle-source-contract-20260607 accepted
  phase11-rp1-clock-adc-enable-toggle-source-contract-v1, selecting one
  bounded pre-state-derived CLK_ADC_CTRL enable-bit transition at CPU physical
  0x1f00018144. The only real-candidate operations are pre-read, compute
  transition_raw = pre_raw ^ 0x00000800, transition-write, post-read,
  restore-write pre_raw, and restore-read. Acceptance for a successful
  transition required the post-read to differ from pre-read only by bit 11 and
  restore-read to equal pre-read.
- Local/static core:
  phase11-rp1-clock-adc-enable-toggle-core-20260607 accepted a real candidate
  implementing only the contracted CLK_ADC_CTRL enable-bit
  transition/readback/restore sequence plus a paired no-MMIO/no-RP1/no-GIC
  control candidate with the same output shape and no forbidden RP1
  clock/reset, GPIO/RIO/pads, MSI-X/PCIe/MIP, or GIC MMIO construction.
- Control proof:
  phase11-rp1-clock-adc-enable-toggle-control-pi5-20260607 accepted the
  no-MMIO/no-RP1/no-GIC control output shape as visible on Pi 5 after v2
  identity join, two 47,240-byte candidate TFTP fetches, 84 control markers,
  final selected-tree identity, and restore proof.
- Real proof:
  phase11-rp1-clock-adc-enable-toggle-pi5-20260607 accepted
  rp1-clock-adc-ctrl-enable-toggle-mismatch-restored. The decisive rerun
  passed v2 identity join for tree
  7024bb54a9446c681d4a8b9c80372fe52a4d4f93b7939f299a8eb2d7199a697a,
  retained two 47,512-byte da591740/kernel_2712.img fetches, final
  selected-tree identity, restore proof, and 78
  TALOS: rp1-clock-adc-ctrl-enable-toggle-result records. The visible result
  reported pre-raw=0xdeaddead, transition-raw=0xdeadd6ad,
  post-raw=0xdeaddead, restore-raw=0xdeaddead, one-bit-transition=true,
  post-enable-flipped=false, post-delta-is-transition-mask=false, and
  restore-eq-pre=true. The lab was finally restored to the original pre-run
  tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: closed the source/core/control/real proof chain as an accepted
  restored mismatch frontier with retained identity-joined Pi 5 evidence.
- fixed: retained the paired no-MMIO/no-RP1/no-GIC control proof as satisfied
  before accepting the real diagnostic proof.
- fixed: retained the rejected first real run, known-good control, decisive
  candidate rerun, identity join, stable TFTP evidence, and restore proof as
  capture hygiene evidence; acceptance uses only the decisive identity-joined
  rerun.
- fixed: updated roadmap and project contract docs for the final
  mismatch-restored closeout boundary.
- deferred: a successful non-idempotent CLK_ADC_CTRL transition, GPIO
  ownership retry, interrupt-delivery work, broader clock/reset ownership,
  reset-controller paths, event generation, handler ownership, DMA/cache,
  storage, generated-root, networking, SSH, broader PCIe enumeration,
  Milestone 11.3, and phase transition require supervisor planning with new
  acceptance criteria.
- not-an-issue: the observed 0xdeaddead raw value is accepted only as the raw
  value restored by this selected proof; it is not interpreted as broad RP1
  clock/reset state.

No findings were removed in this closeout task.

## Accepted Claims

Accepted frontier:
rp1-clock-adc-ctrl-enable-toggle-mismatch-restored-frontier-closed.

The accepted boundary is limited to the source-backed CLK_ADC_CTRL enable-bit
transition/readback/restore contract, the local real/control candidate split,
the paired no-MMIO/no-RP1/no-GIC control output proof, and the real Pi 5
visibility proof that the selected transition attempt ran under identity-joined
serial/TFTP/final-tree evidence and restored the observed pre-read raw value.

This is a precise blocker, not a successful non-idempotent clock ownership
proof. The post-read still matched the pre-read value instead of the requested
0x00000800 enable-bit transition, even though restore-read matched pre-read.

## Retained Risks And Unaccepted Claims

The closeout does not accept successful CLK_ADC_CTRL enable-bit transition
ownership, broad RP1 clock/reset ownership, additional non-idempotent clock
enable/disable/divider/source/PLL programming, reset-controller writes, GPIO
ownership, GPIO event generation, interrupt pending generation, interrupt
delivery, GIC IAR/EOIR acknowledgement, ISR/handler ownership, DMA/cache
behavior, storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, or a phase transition.

Same-shaped CLK_ADC_CTRL enable-bit transition hardware reruns are blocked
unless a future supervisor task supplies a different discriminator or new
acceptance criteria. Future GPIO ownership retry, interrupt-delivery work,
non-idempotent clock/reset operation, reset-controller path, or broader RP1
clock/reset driver ownership requires a new source contract and acceptance
criteria.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-clock-adc-enable-toggle-closeout/evidence-map.json.
- Source contract evidence:
  tasks/evidence/2026-06-07-phase11-rp1-clock-adc-enable-toggle-source-contract/evidence-map.json.
- Local/static core evidence:
  tasks/evidence/2026-06-07-phase11-rp1-clock-adc-enable-toggle-core/evidence-map.json.
- Control proof evidence:
  tasks/evidence/2026-06-07-phase11-rp1-clock-adc-enable-toggle-control-pi5/evidence-map.json.
- Real proof evidence:
  tasks/evidence/2026-06-07-phase11-rp1-clock-adc-enable-toggle-pi5/evidence-map.json.

## Validation

- Static inspection: source contract, core, no-MMIO/no-RP1/no-GIC control
  proof, real proof, restore evidence, and evidence maps inspected.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

Accepted as
rp1-clock-adc-ctrl-enable-toggle-mismatch-restored-frontier-closed.

No explicit worker-owned task remains in this queue. Supervisor planning is
required for the next Milestone 11.2 feature slice. Same-shaped enable-toggle
hardware reruns, GPIO ownership retries, interrupt-delivery work,
non-idempotent clock/reset operations, and broader RP1 clock/reset driver
ownership are not progress unless a future supervisor task supplies a new
source contract and acceptance criteria.
