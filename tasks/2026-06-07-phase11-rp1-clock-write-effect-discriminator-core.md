# Phase 11 RP1 Clock Write-Effect Discriminator Core

Task id: phase11-rp1-clock-write-effect-discriminator-core-20260607

Status: accepted

Classification: accepted-local-static-clock-adc-window-coherence-core

## Goal

Implement the accepted RP1 clock write-effect discriminator core and paired
no-MMIO/no-RP1/no-GIC control locally, without running hardware.

## Scope

- Used only the diagnostic target, operation ordering, report fields,
  classifications, and forbidden operations accepted by
  phase11-rp1-clock-write-effect-discriminator-source-contract-20260607.
- Implemented the real candidate as a read-only ADC clock-register window:
  CLK_SYS_CTRL, CLK_UART_CTRL, CLK_ADC_CTRL, an ordering barrier, CLK_ADC_CTRL
  again, CLK_ADC_DIV_INT, and CLK_ADC_SEL.
- Implemented the paired control candidate with the same serial/output shape,
  simulated raw values, not-constructed address fields, and no forbidden
  RP1/GIC/MSI-X/PCIe/MIP/GPIO/pads/RIO/clock-reset MMIO address construction.
- Retained archive review and strings evidence for the real/control boundary.

## Non-Goals

No Pi 5 hardware run, boot archive publication beyond local/static artifacts,
hardwareTestLock acquisition, uncontracted clock/reset writes,
reset-controller writes, GPIO/RIO/pad writes, event generation, interrupt
enablement or delivery, GIC IAR/EOIR acknowledgement, ISR installation,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, or phase transition.

## Findings

- fixed: added two explicit Pi 5 boot scenarios:
  rpi5_rp1_clock_adc_window_coherence_read and
  rpi5_rp1_clock_adc_window_coherence_no_mmio_control.
- fixed: added CLK_ADC_DIV_INT and CLK_ADC_SEL constants with unit assertions
  matching the accepted source contract.
- fixed: the real candidate emits TALOS:
  rp1-clock-adc-window-coherence-result with the accepted contract id, target,
  clock-manager base, register offsets/addresses/width/raw values, sys/uart
  guard fields, ADC CTRL decoded fields, selector shape booleans, repeated
  sentinel booleans, retained enable-toggle mismatch context, and terminal
  classification.
- fixed: the real candidate performs only the accepted read-only sequence and
  uses a local ordering barrier between the two CLK_ADC_CTRL reads.
- fixed: the control candidate emits TALOS:
  rp1-clock-adc-window-coherence-control with the same field shape,
  not-constructed address fields, simulated raw values, and
  classification=no-mmio-clock-adc-window-coherence-control-visible.
- fixed: archive review scripts retain marker, report-shape, artifact
  identity, and forbidden-string checks; the control review checks absence of
  forbidden RP1/GIC/MSI-X/PCIe/MIP/GPIO/pads/RIO/clock-reset address strings.
- deferred: serialized Pi 5 control proof, real Pi 5 discriminator proof,
  broad clock/reset ownership, GPIO ownership, event generation, interrupt
  delivery, handler ownership, DMA/cache, storage, generated-root, networking,
  SSH, broader PCIe enumeration, Milestone 11.3, and phase transition.
- not-an-issue: UART10 FR polling loads remain in both candidates because they
  are the existing firmware-preserved serial flush path, not RP1 clock/reset,
  GPIO/RIO/pads, MSI-X/PCIe/MIP, or GIC MMIO.

No findings were removed in this task.

## Candidate Artifacts

Real candidate:

- Archive: target/talos-rpi5-rp1-clock-adc-window-coherence-read-core.tar.gz
- Archive SHA256:
  5b49611443a8548f044d93ab066343a3e20b9d7a83f627d27e18c09843183112
- kernel_2712.img SHA256:
  b88cbb89c3b2cfb35ae6e1a74dc871b5721ce77a5ef790a78f575415daa7bc3f
- kernel_2712.img size: 48056 bytes
- Marker: TALOS: rp1-clock-adc-window-coherence-result

Control candidate:

- Archive:
  target/talos-rpi5-rp1-clock-adc-window-coherence-no-mmio-control-core.tar.gz
- Archive SHA256:
  d9a62977cb33b9c6b27525096951773575f18c53b1efbf3e3b143479058b3b88
- kernel_2712.img SHA256:
  f7038e4debc3f73d32937989a68ec5ade6e8e2c613c7e5fe0e351f4722f3acd0
- kernel_2712.img size: 47360 bytes
- Marker: TALOS: rp1-clock-adc-window-coherence-control

These are local/static artifacts only. No hardware behavior is accepted.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet with QEMU on PATH: pass.
- Archive review:
  scripts/rpi5-rp1-clock-adc-window-coherence-read-review.sh passed.
- Archive review:
  scripts/rpi5-rp1-clock-adc-window-coherence-no-mmio-control-review.sh
  passed.
- Static source/string inspection: passed; real candidate constructs the
  accepted RP1 clock-register addresses and performs only the contracted
  read-only sequence; control candidate constructs no forbidden
  RP1/GIC/MSI-X/PCIe/MIP/GPIO/pads/RIO/clock-reset MMIO address.
- git diff --check: pass.
- mdbook build: pass.
- git diff --cached --check: pass.

## Result

Accepted. This accepts only the local/static real candidate, the local/static
no-MMIO/no-RP1/no-GIC control candidate, and their archive/static boundary. It
does not accept Pi 5 hardware behavior, broad RP1 clock/reset ownership, any
RP1 clock/reset write, GPIO ownership, GPIO event generation, interrupt
delivery, handler ownership, DMA/cache, storage, generated-root, networking,
SSH, broader PCIe enumeration, Milestone 11.3, or a phase transition.

## Follow-Up

Promote phase11-rp1-clock-write-effect-discriminator-control-pi5-20260607 only
after this task is accepted and committed and hardwareTestLock remains
unlocked/restored. The next task owns the serialized Pi 5
no-MMIO/no-RP1/no-GIC control proof before any real ADC clock-window
coherence run.
