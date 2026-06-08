# Phase 11 RP1 Bridge/Setup Closeout

Task id: phase11-rp1-bridge-setup-closeout-20260608

Status: accepted

Classification: pcie2-bridge-setup-state-incomplete-frontier-closed

## Goal

Close out the bridge/setup-state discriminator chain and record the accepted
frontier without implying endpoint ownership, broad RP1 mapping, BAR work,
Milestone 11.3, or a phase transition.

## Scope

- Reconciled the accepted source contract, local/static core, paired
  no-MMIO/no-PCIe/no-RP1/no-GIC control proof, real Pi 5 proof, restore
  evidence, and retained risks.
- Recorded findings with disposition.
- Named accepted and unaccepted claims for bridge/setup state, endpoint
  visibility, endpoint ownership, broad RP1 mapping, BAR discovery/programming,
  interrupt delivery, GPIO/clock work, DMA/cache, storage, generated-root,
  networking, SSH, Milestone 11.3, and phase transition.
- Updated roadmap and RP1/PCIe map contract docs for the closeout frontier.
- Set nextAction to supervisor planning rather than creating a worker-owned
  follow-up task.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime source changes, additional PCIe/RP1/GIC/GPIO/clock/reset operations,
endpoint config retry, endpoint configuration mutation, BAR discovery or
programming, bridge setup writes, CPU-to-PCIe window programming,
PERST/link-control changes, interrupt enablement or delivery, DMA/cache,
storage, generated-root, networking, SSH, Milestone 11.3, or phase transition.

## Findings And Disposition

- fixed: reconciled the accepted source contract boundary. The only real
  selected operations were read-only 32-bit snapshots of PCIE_MISC_PCIE_STATUS,
  PCIE_MISC_MISC_CTRL, PCIE_RC_CFG_PRIV1_ID_VAL3, and outbound window 0
  registers at the accepted BCM2712 PCIe2 offsets.
- fixed: reconciled the local/static core evidence. The real archive retained
  the source-contract report shape and classification vocabulary, while the
  paired control retained the same output shape without constructing forbidden
  BCM2712 PCIe, RP1, MIP, GIC, GPIO, clock/reset, DMA, or other MMIO
  addresses.
- fixed: reconciled the control proof as
  no-mmio-pcie2-bridge-setup-state-control-visible. The accepted rerun passed
  capture-transaction-v2, retained two 49,496-byte candidate fetches, 30
  control markers, and restore to the pre-run tree.
- fixed: reconciled the real Pi 5 proof as
  pcie2-bridge-setup-state-incomplete. The accepted unchanged rerun passed
  capture-transaction-v2, retained two 50,736-byte candidate fetches, 90
  result markers, link/preflight predicates true, root-complex class code
  0x060400, visible outbound window 0 registers, and restore to the pre-run
  tree.
- fixed: recorded why the real result is incomplete. The accepted outbound
  window values were win0_lo=0x80000000, win0_hi=0x0,
  win0_base_limit=0x3ff00000, win0_base_hi=0x1c, and win0_limit_hi=0x1c,
  which do not match the source-expected PCIe 0 -> CPU 0x1f_0000_0000 shape.
- deferred: supervisor planning must choose any later endpoint visibility
  retry, bridge/BAR/setup discriminator, interrupt-delivery, GPIO/clock retry,
  or blocker-driven alternate Milestone 11.2 slice. This closeout creates no
  worker-owned follow-up task.
- deferred: pcie2-bridge-setup-state-visible, expected RP1 vendor/device
  visibility, endpoint ownership, broad RP1 mapping, BAR discovery/programming,
  bridge setup writes, PERST/link-control, interrupt delivery, GPIO/clock
  ownership, DMA/cache, storage, generated-root, networking, SSH, Milestone
  11.3, and phase transition remain outside this accepted frontier.
- not-an-issue: visible link/preflight state and root-complex class code are
  accepted as setup-state evidence only; the outbound-window mismatch correctly
  prevents accepting visible source-expected bridge setup or RP1 endpoint
  ownership.

No findings were removed.

## Closeout Classification

Accepted as pcie2-bridge-setup-state-incomplete-frontier-closed.

The accepted frontier is limited to the source-backed read-only bridge/setup
state discriminator, the paired no-MMIO/no-PCIe/no-RP1/no-GIC control proof,
and the real Pi 5 incomplete result under identity-joined hardware evidence.
The real proof reached the accepted PCIe2 link/preflight predicates with
PCIE_MISC_PCIE_STATUS=0x3e0b0, PCIE_MISC_MISC_CTRL=0xa8003000 with
SCB_ACCESS_EN=true and CFG_READ_UR_MODE=true, and
PCIE_RC_CFG_PRIV1_ID_VAL3=0x30060400 with class code 0x060400.

The accepted hardware result also proves the selected outbound window 0
registers are visible, but not in the source-expected PCIe 0 -> CPU
0x1f_0000_0000 shape. The retained values are win0_lo=0x80000000,
win0_hi=0x0, win0_base_limit=0x3ff00000, win0_base_hi=0x1c, and
win0_limit_hi=0x1c.

This does not accept pcie2-bridge-setup-state-visible, expected RP1
vendor/device visibility, endpoint ownership, broad RP1 mapping, BAR discovery
or programming, bridge setup writes, PERST/link-control changes, interrupt
delivery, GPIO/clock ownership, DMA/cache, storage, generated-root, networking,
SSH, Milestone 11.3, or a phase transition.

Same-shaped endpoint config identity and same-shaped bridge/setup-state
hardware reruns remain closed unless a future supervisor task supplies a
different discriminator or new acceptance criteria. Supervisor planning is
required for the next Milestone 11.2 frontier.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-closeout/evidence-map.json.
- Source contract:
  tasks/2026-06-08-phase11-rp1-bridge-setup-source-contract.md.
- Local/static core:
  tasks/2026-06-08-phase11-rp1-bridge-setup-core.md.
- Control Pi 5 proof:
  tasks/2026-06-08-phase11-rp1-bridge-setup-control-pi5.md.
- Real Pi 5 proof:
  tasks/2026-06-08-phase11-rp1-bridge-setup-pi5.md.
- Classification records:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-control-pi5/classification.json,
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-pi5/classification.json.

## Validation

- static inspection: source contract, local/static core, control proof, real
  proof, restore evidence, and evidence maps inspected.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as pcie2-bridge-setup-state-incomplete-frontier-closed.

Next action: no worker-owned follow-up task is created by this closeout.
Set planningNeeded=true for supervisor planning of the next Milestone 11.2
frontier if work should continue.
