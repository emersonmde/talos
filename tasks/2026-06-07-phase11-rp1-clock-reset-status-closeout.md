# Phase 11 RP1 Clock/Reset Status Closeout

Task id: phase11-rp1-clock-reset-status-closeout-20260607

Status: accepted

## Goal

Close out the read-only RP1 clock manager status source/core/control/real
diagnostic chain without implying clock/reset ownership, GPIO ownership,
interrupt delivery, or a phase transition.

## Scope

- Reconciled the accepted source contract, local/static core,
  no-MMIO/no-RP1/no-GIC control proof, and real Pi 5 diagnostic proof.
- Recorded findings with disposition.
- Named accepted and unaccepted claims for RP1 clock manager visibility,
  capture and restore hygiene, clock/reset ownership, GPIO ownership, interrupt
  delivery, and the next Milestone 11.2 step.
- Updated only roadmap/project contract docs for the accepted read-only
  frontier.
- Set the next action to supervisor planning; no worker-owned follow-up task is
  created here.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime source changes, RP1 clock/reset writes, reset-controller writes, GPIO
ownership retry, GPIO event generation, interrupt delivery, GIC IAR/EOIR
acknowledgement, ISR installation, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe enumeration, Milestone 11.3, or phase transition.

## Reconciliation

The chain closes as rp1-clock-manager-status-frontier-closed.

- Source contract:
  phase11-rp1-clock-reset-status-source-contract-20260607 accepted
  phase11-rp1-clock-reset-status-source-contract-v1, selecting a read-only
  RP1 clock manager status snapshot after GPIO14 and GPIO16 both blocked on
  fsel 13 / unknown function. The allowed reads are PLL_SYS_CS at
  0x1f00020000, CLK_SYS_CTRL at 0x1f00018014, CLK_SYS_DIV_INT at
  0x1f00018018, CLK_SYS_SEL at 0x1f00018020, CLK_SLOW_SYS_CTRL at
  0x1f00018024, CLK_UART_CTRL at 0x1f00018054, CLK_UART_DIV_INT at
  0x1f00018058, and CLK_UART_SEL at 0x1f00018060. Linux reset behavior is
  retained only as forbidden source context.
- Local/static core:
  phase11-rp1-clock-reset-status-core-20260607 accepted a real candidate with
  only the contracted read-only clock manager loads and a paired control
  candidate that constructs no forbidden RP1 clock/reset, GPIO/RIO/pads,
  MSI-X/PCIe/MIP, or GIC MMIO path.
- Control proof:
  phase11-rp1-clock-reset-status-control-pi5-20260607 accepted the
  no-MMIO/no-RP1/no-GIC output shape as visible on Pi 5 after v2 identity join,
  two 47,120-byte candidate TFTP fetches, stable pre-restore TFTP evidence,
  final selected-tree identity, and restore proof.
- Real proof:
  phase11-rp1-clock-reset-status-pi5-20260607 accepted
  rp1-clock-manager-status-visible. The decisive rerun passed v2 identity join
  for tree 3e64059ed440eaf48f096d8e2e4113609dbfe9f78444955003547515439c3704,
  retained two 47,280-byte da591740/kernel_2712.img fetches, final
  selected-tree identity, restore proof, and 320
  TALOS: rp1-clock-manager-status-result records. The visible result reported
  pll-sys-lock=true, clk-sys-enabled=true, clk-uart-enabled=true, and
  classification=rp1-clock-manager-status-visible. The lab was finally
  restored to the original pre-run tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Findings And Disposition

- fixed: closed the source/core/control/real proof chain as an accepted
  read-only RP1 clock manager status frontier with retained identity-joined
  Pi 5 evidence.
- fixed: retained the paired no-MMIO/no-RP1/no-GIC control proof as satisfied
  before accepting the real diagnostic proof.
- fixed: retained the rejected first real run, known-good/control run, and
  decisive candidate rerun as capture hygiene evidence; acceptance uses only
  the decisive identity-joined rerun.
- fixed: updated docs to record the accepted read-only boundary and retained
  risks.
- deferred: clock/reset writes, reset ownership, GPIO ownership retries, GPIO
  event generation, interrupt delivery, handler ownership, DMA/cache, storage,
  generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3,
  and phase transition remain future work requiring supervisor planning.
- not-an-issue: visible PLL_SYS/CLK_SYS/CLK_UART status values are valid
  read-only clock manager evidence, but not evidence that Talos owns clocks or
  resets.

No findings were removed in this closeout task.

## Accepted Claims

Accepted frontier: rp1-clock-manager-status-frontier-closed.

The accepted boundary is limited to the source-backed RP1 clock manager status
contract, the local real/control candidate split, the no-MMIO/no-RP1/no-GIC
control output proof, and the real Pi 5 visibility proof for the selected
read-only clock manager status snapshot. The accepted result reports PLL_SYS
locked, CLK_SYS enabled, and CLK_UART enabled through the contracted status
reads.

## Retained Risks And Unaccepted Claims

The closeout does not accept RP1 clock/reset writes, reset-controller writes,
clock/reset ownership, GPIO ownership, GPIO event generation, interrupt pending
generation, interrupt delivery, GIC acknowledgement, ISR/handler ownership,
DMA/cache behavior, storage, generated-root, networking, SSH, broader PCIe
enumeration, Milestone 11.3, or a phase transition.

The clock-manager status result is retained as the accepted read-only
observation for this boundary. A future task must supply a new source contract
and acceptance criteria before any clock/reset write, GPIO ownership retry, or
interrupt-delivery work.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-closeout/evidence-map.json.
- Source contract evidence:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-source-contract/evidence-map.json.
- Local/static core evidence:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-core/evidence-map.json.
- Control proof evidence:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-control-pi5/evidence-map.json.
- Real proof evidence:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-pi5/evidence-map.json.

## Validation

- Static inspection: source contract, core, no-MMIO/no-RP1/no-GIC control
  proof, real proof, and evidence maps inspected.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

Accepted as rp1-clock-manager-status-frontier-closed.

No explicit worker-owned task remains in this queue. Supervisor planning is
required for the next Milestone 11.2 feature slice. Clock/reset writes, GPIO
ownership retries, and interrupt-delivery work are not progress unless a future
supervisor task supplies a new source contract and acceptance criteria.
