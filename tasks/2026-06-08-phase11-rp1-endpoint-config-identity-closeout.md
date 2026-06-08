# Phase 11 RP1 Endpoint Config Identity Closeout

Task id: phase11-rp1-endpoint-config-identity-closeout-20260608

Status: accepted

Classification: rp1-endpoint-config-id-all-ones-frontier-closed

## Goal

Close out the RP1 endpoint config identity-read chain and record the accepted
frontier without implying a phase transition.

## Scope

- Reconciled the accepted source contract, local/static core,
  no-MMIO/no-RP1/no-GIC control proof, real Pi 5 proof, restore evidence, and
  retained risks.
- Recorded findings with disposition.
- Named accepted and unaccepted claims for endpoint config identity visibility,
  broad RP1 mapping, endpoint ownership, PCIe writes, bridge setup, interrupt
  delivery, GPIO ownership, clocks/resets, DMA/cache, storage, generated-root,
  networking, SSH, Milestone 11.3, and next Milestone 11.2 planning.
- Updated the roadmap and RP1/PCIe map contract docs for the closeout frontier.
- Set nextAction to supervisor planning rather than creating a worker-owned
  follow-up task.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition,
runtime source changes, endpoint configuration mutation, BAR programming,
bridge setup, PERST/link-control changes, MSI/MIP/GIC operations, interrupt
enablement or delivery, GIC acknowledgement, ISR installation, RP1
clock/reset writes, GPIO/RIO/pad writes, event generation, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, or phase transition.

## Findings And Disposition

- fixed: reconciled the accepted source contract boundary:
  `rp1-endpoint-config-vendor-device-read` may read PCIe2 host-link status,
  write the bounded controller selector `0x00100000` to `EXT_CFG_INDEX`,
  and read exactly one 32-bit config dword from `EXT_CFG_DATA + 0` for
  BDF 0002:01:00.0 offset 0.
- fixed: reconciled the local/static core evidence: the real archive retained
  the accepted report shape and the paired no-MMIO/no-RP1/no-GIC control
  retained the same classification vocabulary without constructing forbidden
  BCM2712 PCIe, RP1, MIP, GIC, or DMA MMIO addresses.
- fixed: reconciled the control proof as
  `no-mmio-rp1-endpoint-config-id-control-visible`; the accepted rerun passed
  `pi5-capture-transaction-v2`, retained two 47,608-byte candidate fetches,
  66 control markers, and restore to the pre-run tree.
- fixed: reconciled the real Pi 5 proof as `rp1-endpoint-config-id-all-ones`;
  the accepted rerun passed `pi5-capture-transaction-v2`, retained two
  48,456-byte candidate fetches, 135 result markers, host-link-up status
  `0x3e0b0`, selector write `0x00100000`, `EXT_CFG_DATA + 0` raw
  `0xffffffff`, and restore to the pre-run tree.
- deferred: expected RP1 vendor/device visibility, endpoint ownership, broad
  RP1 mapping, endpoint configuration mutation, BAR discovery/programming,
  bridge setup, PERST/link-control, interrupt delivery, DMA/cache, storage,
  generated-root, networking, SSH, Milestone 11.3, and phase transition remain
  outside this accepted frontier.
- not-an-issue: the all-ones config dword is an accepted source-contract
  classification and closes this discriminator; it is not a reason to repeat
  the same hardware run as progress.

No findings were removed.

## Closeout Classification

Accepted as rp1-endpoint-config-id-all-ones-frontier-closed.

The accepted frontier is limited to the bounded source-backed endpoint config
identity attempt, the paired no-MMIO/no-RP1/no-GIC control proof, and the real
Pi 5 all-ones config result under identity-joined hardware evidence. The real
proof reached the accepted PCIe2 host-link-up precondition with
`PCIE_MISC_PCIE_STATUS=0x3e0b0`, wrote only the accepted controller selector
`0x00100000` to `EXT_CFG_INDEX`, and read `EXT_CFG_DATA + 0` as
`0xffffffff` (`vendor-id=0xffff`, `device-id=0xffff`) for
BDF 0002:01:00.0 offset 0.

This does not accept expected RP1 vendor/device visibility, endpoint
ownership, broad RP1 mapping, endpoint configuration mutation, BAR
programming or discovery, bridge setup, PERST/link-control changes,
interrupt delivery, DMA/cache, storage, generated-root, networking, SSH,
Milestone 11.3, or a phase transition.

Same-shaped endpoint config identity hardware reruns are not progress unless a
future supervisor task supplies a different discriminator or new acceptance
criteria. Supervisor planning is required for the next Milestone 11.2 frontier.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-endpoint-config-identity-closeout/evidence-map.json.
- Source contract:
  tasks/2026-06-08-phase11-rp1-endpoint-config-identity-source-contract.md.
- Local/static core:
  tasks/2026-06-08-phase11-rp1-endpoint-config-identity-core.md.
- Control Pi 5 proof:
  tasks/2026-06-08-phase11-rp1-endpoint-config-identity-control-pi5.md.
- Real Pi 5 proof:
  tasks/2026-06-08-phase11-rp1-endpoint-config-identity-pi5.md.
- Classification records:
  tasks/evidence/2026-06-08-phase11-rp1-endpoint-config-identity-control-pi5/classification.json,
  tasks/evidence/2026-06-08-phase11-rp1-endpoint-config-identity-pi5/classification.json.

## Validation

- static inspection: source contract, local/static core, control proof, real
  proof, restore evidence, and evidence maps inspected.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as rp1-endpoint-config-id-all-ones-frontier-closed.

Next action: no worker-owned follow-up task is created by this closeout.
Set planningNeeded=true for supervisor planning of a different Milestone 11.2
frontier if work should continue.
