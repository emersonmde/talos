# Phase 11 RP1 Clock Write-Effect Discriminator Closeout

Task id: phase11-rp1-clock-write-effect-discriminator-closeout-20260607

Status: accepted

Classification: rp1-clock-adc-window-readback-sentinel-frontier-closed

## Goal

Close out the RP1 clock write-effect discriminator chain by reconciling the
accepted source contract, local/static core, no-MMIO/no-RP1/no-GIC control
proof, real Pi 5 proof, restore evidence, retained risks, and next action.

## Scope

- Reconciled the accepted read-only ADC clock-window coherence source contract,
  local/static implementation, paired control proof, and real Pi 5 proof.
- Confirmed the accepted frontier is limited to the read-only sentinel/result
  boundary reported by the real candidate.
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

- phase11-rp1-clock-write-effect-discriminator-source-contract-20260607
  accepted phase11-rp1-clock-write-effect-discriminator-source-contract-v1.
- The selected discriminator is read-only
  rp1-clk-adc-window-coherence-read.
- Allowed 32-bit reads are CLK_SYS_CTRL at 0x1f00018014, CLK_UART_CTRL at
  0x1f00018054, two ordered CLK_ADC_CTRL reads at 0x1f00018144,
  CLK_ADC_DIV_INT at 0x1f00018148, and CLK_ADC_SEL at 0x1f00018150.
- No writes or restore operations are selected. Same-shaped CLK_ADC_CTRL
  enable-bit transition reruns remain blocked.

Core:

- phase11-rp1-clock-write-effect-discriminator-core-20260607 accepted the
  local/static real and control candidates.
- The real candidate reports the contracted read-only register window,
  decoded guard/ADC fields, retained enable-toggle mismatch context, and
  terminal classification.
- The control candidate preserves output shape while constructing no forbidden
  RP1 clock/reset, GPIO/RIO/pads, MSI-X/PCIe/MIP, or GIC MMIO address.

Control proof:

- phase11-rp1-clock-write-effect-discriminator-control-pi5-20260607 accepted
  the paired control as no-mmio-clock-adc-window-coherence-control-visible.
- After an inconclusive first candidate run, a production-timer known-good
  control passed the v2 identity join, and the unchanged control candidate
  rerun retained two matching 47,360-byte TFTP fetches, 52 control markers,
  final selected-tree identity, and restore proof.
- The control accepts only the no-MMIO/no-RP1/no-GIC output/capture path.

Real proof:

- phase11-rp1-clock-write-effect-discriminator-pi5-20260607 accepted the real
  Pi 5 proof as rp1-clock-adc-window-readback-sentinel.
- After an inconclusive first capture, a production-timer known-good control
  passed the v2 identity join, and the unchanged real candidate rerun retained
  two matching 48,056-byte TFTP fetches, 52 result markers, final selected-tree
  identity, and restore proof.
- The accepted output reported CLK_SYS_CTRL, CLK_UART_CTRL, two ordered
  CLK_ADC_CTRL reads, CLK_ADC_DIV_INT, and CLK_ADC_SEL all returning
  0xdeaddead, with adc-ctrl-stable=true, adc-window-all-equal=true,
  adc-window-all-deaddead=true, adc-sel-zero=false, adc-sel-one-hot=false,
  adc-sel-multi-bit=true, and retained enable-toggle restore equality.

## Accepted Claims

- The source-backed read-only ADC clock-window coherence discriminator is
  implemented locally and visible on Pi 5.
- The Pi 5 result is classified as
  rp1-clock-adc-window-readback-sentinel because the selected clock-manager
  window returned the repeated 0xdeaddead sentinel value across CLK_SYS_CTRL,
  CLK_UART_CTRL, CLK_ADC_CTRL, CLK_ADC_DIV_INT, and CLK_ADC_SEL.
- The accepted real proof is joined to candidate identity, same-run TFTP
  fetches, serial markers, final selected-tree identity, and restore proof
  under pi5-capture-transaction-v2.
- The lab was restored to tree
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 after the
  accepted hardware runs.

## Retained Risks And Rejected Claims

- Successful non-idempotent RP1 clock ownership remains unaccepted.
- Broad RP1 clock/reset ownership and any new clock/reset write remain
  unaccepted.
- Divider/source/PLL/frequency-counter/reset-controller writes remain
  unaccepted.
- GPIO ownership, GPIO/RIO/pad writes, event generation, interrupt delivery,
  GIC acknowledgement, ISR/handler ownership, DMA/cache, storage,
  generated-root, networking, SSH, broader PCIe behavior, Milestone 11.3, and
  phase transition remain unaccepted.
- Same-shaped CLK_ADC_CTRL enable-bit transition hardware reruns remain
  blocked unless a future supervisor task supplies different accepted
  criteria.

## Findings And Disposition

- fixed: reconciled the source contract, core, control proof, real proof,
  evidence maps, and restore evidence into one accepted closeout frontier.
- fixed: classified the chain as
  rp1-clock-adc-window-readback-sentinel-frontier-closed instead of accepting
  broad RP1 clock/reset ownership by implication.
- fixed: recorded the real output as a read-only repeated-sentinel boundary
  that can inform future supervisor planning without authorizing immediate
  writes.
- fixed: retained the standard inconclusive-run triage record for the control
  and real Pi 5 captures.
- deferred: future GPIO ownership retry, interrupt-delivery slice, or broader
  clock/reset step requires supervisor planning and explicit acceptance
  criteria using this boundary.
- not-an-issue: no additional hardware run is required for closeout because
  the accepted control and real proof tasks already captured identity-joined
  hardware evidence and restore proof.

No findings were removed.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-07-phase11-rp1-clock-write-effect-discriminator-closeout/evidence-map.json.
- Static reconciliation notes:
  tasks/evidence/2026-06-07-phase11-rp1-clock-write-effect-discriminator-closeout/static-reconciliation-notes.md.
- Source contract task:
  tasks/2026-06-07-phase11-rp1-clock-write-effect-discriminator-source-contract.md.
- Core task:
  tasks/2026-06-07-phase11-rp1-clock-write-effect-discriminator-core.md.
- Control proof task:
  tasks/2026-06-07-phase11-rp1-clock-write-effect-discriminator-control-pi5.md.
- Real proof task:
  tasks/2026-06-07-phase11-rp1-clock-write-effect-discriminator-pi5.md.

## Validation

- Static inspection: source contract, core, control proof, real proof, restore
  evidence, and evidence maps inspected.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

Accepted as rp1-clock-adc-window-readback-sentinel-frontier-closed. This
closeout accepts only the read-only ADC clock-window coherence sentinel
frontier. It does not accept successful non-idempotent clock ownership, broad
RP1 clock/reset ownership, any new clock/reset write, GPIO ownership, event
generation, interrupt delivery, handler ownership, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3, or
phase transition.

## Next Action

Supervisor planning is required for the next Milestone 11.2 feature slice. A
future task may use this boundary to plan a GPIO ownership retry,
interrupt-delivery slice, or broader clock/reset step, but this closeout does
not create a worker-owned follow-up task.
