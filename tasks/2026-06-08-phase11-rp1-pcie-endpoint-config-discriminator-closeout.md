# Phase 11 RP1 PCIe Endpoint/Config Discriminator Closeout

Task id: phase11-rp1-pcie-endpoint-config-discriminator-closeout-20260608

Status: accepted

Classification: pcie2-host-link-up-rp1-window-sentinel-frontier-closed

## Goal

Close out the RP1 PCIe endpoint/config/decode discriminator chain by
reconciling the accepted source contract, local/static core, no-MMIO/no-RP1/
no-GIC control proof, real Pi 5 proof, restore evidence, retained risks, and
next action without implying a phase transition.

## Scope

- Reconciled the accepted read-only PCIe2 host-link status source contract,
  local/static implementation, paired control proof, and real Pi 5 proof.
- Confirmed the accepted frontier is limited to visible/link-up BCM2712 PCIe2
  host status while the retained RP1 SYSINFO/clock-window path remains
  sentinel-shaped.
- Updated roadmap and Phase 11 project contract docs for the accepted closeout
  frontier.
- Recorded findings with disposition.

## Non-Goals

No runtime implementation, hardware run, boot archive publication,
hardwareTestLock acquisition, endpoint config-space access, PCIe write,
bridge setup, PERST/link-control change, MSI/MIP/GIC operation, RP1
peripheral/SYSINFO/clock/GPIO retry, clock/reset write, reset-controller
write, GPIO/RIO/pad write, event generation, interrupt enablement or delivery,
GIC IAR/EOIR acknowledgement, ISR installation, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, or phase transition.

## Reconciliation

Source contract:

- phase11-rp1-pcie-endpoint-config-discriminator-source-contract-20260608
  accepted phase11-rp1-pcie-endpoint-config-discriminator-source-contract-v1.
- The selected discriminator is read-only pcie2-host-link-status-read.
- The only allowed operation is a single 32-bit load from BCM2712 PCIe2
  PCIE_MISC_PCIE_STATUS at CPU physical 0x1000124068.
- Source inspection rejected endpoint config-space probing in this slice
  because retained Broadcom STB PCIe code gates config access on link-up and
  performs an EXT_CFG_INDEX write before reading EXT_CFG_DATA.

Core:

- phase11-rp1-pcie-endpoint-config-discriminator-core-20260608 accepted the
  local/static real and control candidates.
- The real candidate reports the contracted PCIe2 base, register, offset,
  physical address, width, raw status, decoded link bits, retained RP1
  SYSINFO/clock sentinel context, and terminal classification.
- The control candidate preserves output shape while constructing no
  forbidden BCM2712 PCIe, RP1 peripheral/SYSINFO/clock/GPIO/MSI-X, MIP, GIC,
  or DMA MMIO address.

Control proof:

- phase11-rp1-pcie-endpoint-config-discriminator-control-pi5-20260608
  accepted the paired control as no-mmio-pcie2-host-link-status-control-visible.
- After an inconclusive first candidate run, a production-timer known-good
  control passed, and the unchanged control candidate rerun passed the v2
  identity join with two 46,672-byte candidate TFTP fetches, 118 control
  markers, final selected-tree identity, and restore proof.
- The control accepts only the no-MMIO/no-RP1/no-GIC output/capture path.

Real proof:

- phase11-rp1-pcie-endpoint-config-discriminator-pi5-20260608 accepted the
  real Pi 5 proof as pcie2-host-link-up-rp1-window-sentinel.
- After an inconclusive first capture and known-good control triage, the
  unchanged real candidate rerun passed the v2 identity join with two
  46,880-byte candidate TFTP fetches, 120 result markers, final selected-tree
  identity, and restore proof.
- The accepted output reported PCIE_MISC_PCIE_STATUS raw=0x3e0b0 with
  pcie-port=true, dl-active=true, phylinkup=true, link-in-l23=false,
  status-is-deaddead=false, retained-rp1-window-sentinel=true, and terminal
  classification pcie2-host-link-up-rp1-window-sentinel.

## Accepted Claims

- The source-backed read-only PCIe2 host-link status discriminator is
  implemented locally and visible on Pi 5.
- PCIE_MISC_PCIE_STATUS at 0x1000124068 returns a non-sentinel value with
  DL_ACTIVE and PHYLINKUP set on the accepted Pi 5 run.
- The result separates visible BCM2712 PCIe2 host/link state from the retained
  RP1 SYSINFO/clock-window sentinel context.
- The accepted control and real proofs are joined to candidate identity,
  same-run TFTP fetches, serial markers, final selected-tree identity, and
  restore proof under pi5-capture-transaction-v2.
- The lab was restored to tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 after the
  accepted hardware runs.

## Retained Risks And Rejected Claims

- Endpoint config-space access remains unaccepted.
- Broad RP1 mapping, endpoint ownership, and any PCIe write remain unaccepted.
- Bridge setup, PERST/link-control changes, MSI/MIP/GIC operations, interrupt
  delivery, and ISR/handler ownership remain unaccepted.
- Clock/reset ownership, GPIO ownership, event generation, DMA/cache, storage,
  generated-root, networking, SSH, Milestone 11.3, and phase transition remain
  unaccepted.
- Same-shaped PCIe2 host-link status hardware reruns are not progress unless a
  future supervisor task supplies a different discriminator or new acceptance
  criteria.

## Findings And Disposition

- fixed: reconciled the source contract, core, control proof, real proof,
  evidence maps, and restore evidence into one accepted closeout frontier.
- fixed: classified the chain as
  pcie2-host-link-up-rp1-window-sentinel-frontier-closed rather than accepting
  endpoint config-space access, broad RP1 mapping, or endpoint ownership by
  implication.
- fixed: recorded the real output as visible/link-up BCM2712 PCIe2 host status
  with the retained RP1 SYSINFO/clock-window sentinel still active as
  comparator context.
- fixed: retained the standard inconclusive-run triage record for the control
  and real Pi 5 captures.
- deferred: endpoint config-space access, bridge setup, PERST/link-control
  changes, MSI/MIP/GIC operations, interrupt-delivery work, broader RP1
  mapping, GPIO ownership retry, DMA/cache, storage, generated-root,
  networking, SSH, Milestone 11.3, and phase transition require supervisor
  planning and explicit acceptance criteria.
- not-an-issue: no additional hardware run is required for closeout because
  the accepted control and real proof tasks already captured identity-joined
  hardware evidence and restore proof.

No findings were removed.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-closeout/evidence-map.json.
- Static reconciliation notes:
  tasks/evidence/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-closeout/static-reconciliation-notes.md.
- Source contract task:
  tasks/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-source-contract.md.
- Core task:
  tasks/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-core.md.
- Control proof task:
  tasks/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-control-pi5.md.
- Real proof task:
  tasks/2026-06-08-phase11-rp1-pcie-endpoint-config-discriminator-pi5.md.

## Validation

- Static inspection: source contract, core, control proof, real proof, restore
  evidence, and evidence maps inspected.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

Accepted as pcie2-host-link-up-rp1-window-sentinel-frontier-closed. This
closeout accepts only the read-only PCIe2 host-link status frontier: the host
status register is visible and link-up while the retained RP1 SYSINFO/
clock-window path remains sentinel-shaped. It does not accept endpoint
config-space access, broad RP1 mapping, endpoint ownership, PCIe writes,
interrupt delivery, DMA/cache, storage, generated-root, networking, SSH,
Milestone 11.3, or phase transition.

## Next Action

Supervisor planning is required for the next Milestone 11.2 feature slice. A
future task may use this boundary to plan endpoint config-space access,
broader RP1 mapping discrimination, interrupt-delivery work, GPIO ownership
retry, or a later Milestone 11.3 slice, but this closeout does not create a
worker-owned follow-up task.
