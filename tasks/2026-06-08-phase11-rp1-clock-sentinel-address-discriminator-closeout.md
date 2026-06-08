# Phase 11 RP1 Clock Sentinel Address Discriminator Closeout

Task id: phase11-rp1-clock-sentinel-address-discriminator-closeout-20260608

Status: accepted

Classification: rp1-sysinfo-and-clock-window-sentinel-frontier-closed

## Goal

Close out the RP1 SYSINFO versus retained clock-window sentinel discriminator
chain by reconciling the accepted source contract, local/static core,
no-MMIO/no-RP1/no-GIC control proof, real Pi 5 proof, restore evidence,
retained risks, and next action.

## Scope

- Reconciled the accepted read-only SYSINFO identity versus retained
  clock-window sentinel source contract, local/static implementation, paired
  control proof, and real Pi 5 proof.
- Confirmed the accepted frontier is limited to the broader
  SYSINFO/address-decode sentinel boundary reported by the real candidate.
- Updated roadmap and Phase 11 project contract docs for the accepted closeout
  frontier.
- Recorded findings with disposition.

## Non-Goals

No runtime implementation, hardware run, boot archive publication,
hardwareTestLock acquisition, RP1 clock/reset write, reset-controller write,
GPIO/RIO/pad write, event generation, interrupt enablement or delivery, GIC
IAR/EOIR acknowledgement, ISR installation, broad clock/reset driver
ownership, DMA/cache, storage, generated-root, networking, SSH, broader PCIe
enumeration, Milestone 11.3, or phase transition.

## Reconciliation

Source contract:

- phase11-rp1-clock-sentinel-address-discriminator-source-contract-20260608
  accepted phase11-rp1-clock-sentinel-address-discriminator-source-contract-v1.
- The selected discriminator is read-only
  rp1-sysinfo-vs-clock-sentinel-read.
- Allowed 32-bit reads are SYSINFO_CHIP_ID at 0x1f00000000,
  SYSINFO_PLATFORM at 0x1f00000004, and CLK_ADC_CTRL at 0x1f00018144.
- No writes, restore operations, GPIO/RIO/pad operations, MSI-X/PCIe/MIP
  operations, GIC operations, interrupt operations, DMA/cache operations, or
  clock/reset programming operations are selected.

Core:

- phase11-rp1-clock-sentinel-address-discriminator-core-20260608 accepted the
  local/static real and control candidates.
- The real candidate reports the contracted SYSINFO and CLK_ADC_CTRL reads,
  expected chip id, sentinel/equality booleans, retained ADC clock-window
  context, and terminal classification.
- The control candidate preserves output shape while constructing no
  forbidden RP1 SYSINFO, clock/reset, GPIO/RIO/pads, MSI-X/PCIe/MIP, or GIC
  MMIO address.

Control proof:

- phase11-rp1-clock-sentinel-address-discriminator-control-pi5-20260608
  accepted the paired control as no-mmio-sysinfo-clock-sentinel-control-visible.
- After an inconclusive first candidate run, a production-timer known-good
  control was retained, and the unchanged control candidate rerun passed the
  v2 identity join with two 47,288-byte candidate TFTP fetches, 60 control
  markers, final selected-tree identity, and restore proof.
- The control accepts only the no-MMIO/no-RP1/no-GIC output/capture path.

Real proof:

- phase11-rp1-clock-sentinel-address-discriminator-pi5-20260608 accepted the
  real Pi 5 proof as rp1-sysinfo-and-clock-window-sentinel.
- After an inconclusive first capture, a production-timer known-good control
  was retained, and the unchanged real candidate rerun passed the v2 identity
  join with two 47,776-byte candidate TFTP fetches, 62 result markers, final
  selected-tree identity, and restore proof.
- The accepted output reported SYSINFO_CHIP_ID, SYSINFO_PLATFORM, and
  CLK_ADC_CTRL all returning 0xdeaddead, with chip-id-matches-expected=false,
  chip-id-is-deaddead=true, platform-is-deaddead=true,
  adc-ctrl-is-deaddead=true, sysinfo-pair-equal=true,
  sysinfo-vs-adc-same=true, and retained ADC-window classification
  rp1-clock-adc-window-readback-sentinel.

## Accepted Claims

- The source-backed read-only SYSINFO identity versus retained clock-window
  sentinel discriminator is implemented locally and visible on Pi 5.
- The Pi 5 result is classified as rp1-sysinfo-and-clock-window-sentinel
  because the selected SYSINFO identity/address-decode reads returned the same
  0xdeaddead sentinel value as the retained CLK_ADC_CTRL comparator.
- The accepted real proof is joined to candidate identity, same-run TFTP
  fetches, serial markers, final selected-tree identity, and restore proof
  under pi5-capture-transaction-v2.
- The lab was restored to tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 after the
  accepted hardware runs.

## Retained Risks And Rejected Claims

- Live RP1 SYSINFO chip identity remains unaccepted.
- Broad RP1 clock/reset ownership and any new clock/reset write remain
  unaccepted.
- Successful non-idempotent clock ownership, divider/source/PLL/
  frequency-counter/reset-controller writes, and broader clock-manager
  operation remain unaccepted.
- GPIO ownership, GPIO/RIO/pad writes, event generation, interrupt delivery,
  GIC acknowledgement, ISR/handler ownership, DMA/cache, storage,
  generated-root, networking, SSH, broader PCIe behavior, Milestone 11.3, and
  phase transition remain unaccepted.
- Same-shaped SYSINFO-vs-clock-window sentinel hardware reruns remain blocked
  unless a future supervisor task supplies a different discriminator or new
  acceptance criteria.

## Findings And Disposition

- fixed: reconciled the source contract, core, control proof, real proof,
  evidence maps, and restore evidence into one accepted closeout frontier.
- fixed: classified the chain as
  rp1-sysinfo-and-clock-window-sentinel-frontier-closed instead of accepting
  live RP1 SYSINFO identity or broad clock/reset ownership by implication.
- fixed: recorded the real output as a broader SYSINFO/address-decode
  sentinel boundary that can inform future supervisor planning without
  authorizing immediate writes or GPIO/interrupt work.
- fixed: retained the standard inconclusive-run triage record for the control
  and real Pi 5 captures.
- deferred: any GPIO ownership retry, interrupt-delivery slice, broader
  clock/reset step, different address/decode discriminator, or Milestone 11.3
  work requires supervisor planning and explicit acceptance criteria.
- not-an-issue: no additional hardware run is required for closeout because
  the accepted control and real proof tasks already captured identity-joined
  hardware evidence and restore proof.

No findings were removed.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-closeout/evidence-map.json.
- Static reconciliation notes:
  tasks/evidence/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-closeout/static-reconciliation-notes.md.
- Source contract task:
  tasks/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-source-contract.md.
- Core task:
  tasks/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-core.md.
- Control proof task:
  tasks/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-control-pi5.md.
- Real proof task:
  tasks/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-pi5.md.

## Validation

- Static inspection: source contract, core, control proof, real proof, restore
  evidence, and evidence maps inspected.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

Accepted as rp1-sysinfo-and-clock-window-sentinel-frontier-closed. This
closeout accepts only the read-only SYSINFO/address-decode sentinel frontier.
It does not accept live RP1 SYSINFO identity, broad RP1 clock/reset ownership,
any new clock/reset write, GPIO ownership, event generation, interrupt
delivery, handler ownership, DMA/cache, storage, generated-root, networking,
SSH, broader PCIe enumeration, Milestone 11.3, or phase transition.

## Next Action

Supervisor planning is required for the next Milestone 11.2 feature slice. A
future task may use this boundary to plan a different address/decode
discriminator, GPIO ownership retry, interrupt-delivery slice, or broader
clock/reset step, but this closeout does not create a worker-owned follow-up
task.
