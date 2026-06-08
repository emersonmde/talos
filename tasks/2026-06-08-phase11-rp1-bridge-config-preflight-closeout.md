# Phase 11 RP1 Bridge/Config Preflight Closeout

Task id: phase11-rp1-bridge-config-preflight-closeout-20260608

Status: accepted

Classification: pcie2-bridge-preflight-ready-frontier-closed

## Goal

Close out the bridge/config-preflight discriminator chain and record the
accepted frontier without implying a phase transition or selecting the next
Milestone 11.2 direction.

## Scope

- Reconciled the accepted source contract, local/static core, paired
  no-MMIO/no-PCIe/no-RP1/no-GIC control proof, real Pi 5 proof, restore
  evidence, and retained risks.
- Recorded findings with disposition.
- Named accepted and unaccepted claims for bridge/config preflight, endpoint
  config identity, broad RP1 mapping, endpoint ownership, PCIe writes, bridge
  setup, interrupt delivery, GPIO ownership, clocks/resets, DMA/cache,
  storage, generated-root, networking, SSH, Milestone 11.3, and the next
  Milestone 11.2 planning boundary.
- Updated the roadmap and RP1/PCIe map contract docs for the closeout
  frontier.
- Set nextAction to supervisor planning rather than creating a worker-owned
  follow-up task.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime source changes, additional PCIe/RP1/GIC/GPIO/clock/reset operations,
event generation, endpoint configuration mutation, BAR discovery or
programming, bridge setup, PERST/link-control change, interrupt enablement or
delivery, GIC acknowledgement, ISR installation, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, or phase transition.

## Findings And Disposition

- fixed: reconciled the accepted source contract boundary:
  pcie2-bridge-misc-ctrl-preflight-read may read PCIe2 host-link status at
  0x1000124068, then read exactly one 32-bit dword from PCIE_MISC_MISC_CTRL at
  0x1000124008 and decode the source-defined preflight bits.
- fixed: reconciled the local/static core evidence: the real archive retained
  the accepted report shape and the paired control retained the same
  classification vocabulary while constructing no forbidden BCM2712 PCIe, RP1,
  MIP, GIC, DMA, or other MMIO address.
- fixed: reconciled the control proof as
  no-mmio-pcie2-bridge-preflight-control-visible; the accepted rerun passed
  pi5-capture-transaction-v2, retained two 47,504-byte candidate fetches, 60
  control markers, and restore to the pre-run tree.
- fixed: reconciled the real Pi 5 proof as pcie2-bridge-preflight-ready; the
  accepted rerun passed pi5-capture-transaction-v2, retained two 48,000-byte
  candidate fetches, 123 result markers, host-link-up status 0x3e0b0,
  PCIE_MISC_MISC_CTRL=0xa8003000, SCB_ACCESS_EN=true,
  CFG_READ_UR_MODE=true, and restore to the pre-run tree.
- deferred: supervisor planning must choose any later bridge/setup,
  interrupt-delivery, GPIO/clock retry, or blocker-driven alternate
  Milestone 11.2 slice. This closeout creates no worker-owned follow-up task.
- deferred: endpoint ownership, expected RP1 vendor/device visibility, broad
  RP1 mapping, endpoint configuration mutation, BAR discovery/programming,
  bridge setup, PERST/link-control, interrupt delivery, DMA/cache, storage,
  generated-root, networking, SSH, Milestone 11.3, and phase transition remain
  outside this accepted frontier.
- not-an-issue: the ready-shaped PCIE_MISC_MISC_CTRL result is treated as a
  preflight state discriminator, not proof of endpoint ownership, bridge
  setup, BAR visibility, or interrupt delivery.

No findings were removed.

## Closeout Classification

Accepted as pcie2-bridge-preflight-ready-frontier-closed.

The accepted frontier is limited to the source-backed read-only bridge/config
preflight discriminator, the paired no-MMIO/no-PCIe/no-RP1/no-GIC control
proof, and the real Pi 5 ready result under identity-joined hardware evidence.
The real proof reached the accepted PCIe2 host-link-up precondition with
PCIE_MISC_PCIE_STATUS=0x3e0b0, then read PCIE_MISC_MISC_CTRL as 0xa8003000
with SCB_ACCESS_EN=true and CFG_READ_UR_MODE=true.

This does not accept expected RP1 vendor/device visibility, endpoint
ownership, broad RP1 mapping, endpoint configuration mutation, BAR discovery
or programming, bridge setup, PERST/link-control changes, interrupt delivery,
DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3, or a
phase transition.

Same-shaped endpoint config identity hardware reruns remain closed unless a
future supervisor task supplies a different discriminator or new acceptance
criteria. Supervisor planning is required for the next Milestone 11.2 frontier.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-config-preflight-closeout/evidence-map.json.
- Source contract:
  tasks/2026-06-08-phase11-rp1-bridge-config-preflight-source-contract.md.
- Local/static core:
  tasks/2026-06-08-phase11-rp1-bridge-config-preflight-core.md.
- Control Pi 5 proof:
  tasks/2026-06-08-phase11-rp1-bridge-config-preflight-control-pi5.md.
- Real Pi 5 proof:
  tasks/2026-06-08-phase11-rp1-bridge-config-preflight-pi5.md.
- Classification records:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-config-preflight-control-pi5/classification.json,
  tasks/evidence/2026-06-08-phase11-rp1-bridge-config-preflight-pi5/classification.json.

## Validation

- static inspection: source contract, local/static core, control proof, real
  proof, restore evidence, and evidence maps inspected.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as pcie2-bridge-preflight-ready-frontier-closed.

Next action: no worker-owned follow-up task is created by this closeout.
Set planningNeeded=true for supervisor planning of the next Milestone 11.2
frontier if work should continue.
