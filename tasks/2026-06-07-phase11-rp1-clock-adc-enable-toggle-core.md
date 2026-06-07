# Phase 11 RP1 Clock ADC Enable Toggle Core

Task id: phase11-rp1-clock-adc-enable-toggle-core-20260607

Status: accepted

Classification: accepted-local-static-clock-adc-ctrl-enable-toggle-core

## Goal

Implement the accepted reversible CLK_ADC_CTRL enable-bit transition diagnostic
core and paired no-MMIO/no-RP1/no-GIC control locally, without running
hardware.

## Scope

- Used only the operation sequence, mask, report fields, forbidden operations,
  classifications, and control requirements accepted by
  phase11-rp1-clock-adc-enable-toggle-source-contract-20260607.
- Implemented the real candidate as pre-read CLK_ADC_CTRL, compute
  transition_raw = pre_raw ^ 0x00000800, transition-write, post-read,
  restore-write pre_raw, and restore-read.
- Implemented a paired control candidate with the same serial/output shape,
  simulated raw values, not-constructed address fields, and no forbidden
  RP1/GIC/MSI-X/PCIe/MIP/GPIO/pads/RIO/clock-reset MMIO address construction.
- Retained static/archive evidence for real and control candidate identity,
  Image/archive shape, report shape, restore logic, and the local/static
  real-vs-control boundary.

## Non-Goals

No Pi 5 hardware run, hardwareTestLock acquisition, published boot archive,
uncontracted clock/reset writes, reset-controller writes, GPIO/RIO/pad writes,
event generation, interrupt enablement or delivery, GIC IAR/EOIR
acknowledgement, ISR installation, DMA/cache, storage, generated-root,
networking, SSH, broader PCIe enumeration, Milestone 11.3, or phase
transition.

## Findings

- fixed: added two explicit Pi 5 boot scenarios:
  rpi5_rp1_clock_adc_ctrl_enable_toggle and
  rpi5_rp1_clock_adc_ctrl_enable_toggle_no_mmio_control.
- fixed: added RP1_CLK_CTRL_ENABLE as 0x00000800, with a unit assertion
  matching the accepted source contract.
- fixed: the real candidate emits
  TALOS: rp1-clock-adc-ctrl-enable-toggle-result with the accepted contract id,
  target, register, base, offset, address, width, transition mask,
  pre/transition/post/restore raw fields, decoded pre/post/restore fields,
  transition/restore booleans, retained idempotent proof context, retained
  GPIO14/GPIO16 fsel 13 blocker context, and terminal classification.
- fixed: the real candidate performs only the accepted pre-read,
  enable-bit-toggle transition-write, post-read, restore-write, restore-read
  sequence for CLK_ADC_CTRL.
- fixed: the control candidate emits
  TALOS: rp1-clock-adc-ctrl-enable-toggle-control with the same field shape,
  not-constructed MMIO address fields, simulated values 0 -> 0x800 -> 0x800 ->
  0, and classification=simulated/control.
- fixed: real and control archive review scripts retain marker, report-shape,
  artifact identity, and forbidden-string checks.
- deferred: serialized Pi 5 control proof, real Pi 5 enable-bit transition
  proof, broader clock/reset ownership, GPIO ownership retries, event
  generation, interrupt delivery, handler ownership, DMA/cache, storage,
  generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3,
  and phase transition.
- not-an-issue: UART10 FR polling loads remain in both candidates because they
  are the existing firmware-preserved serial flush path, not RP1 clock/reset,
  GPIO/RIO/pads, MSI-X/PCIe/MIP, or GIC MMIO.

No findings were removed in this task.

## Candidate Artifacts

Real candidate:

- Archive: target/talos-rpi5-rp1-clock-adc-ctrl-enable-toggle-core.tar.gz
- Archive SHA256:
  4d1119458c9d9caaacb6cdadcc1f5c1a1d3e87d8e538d4dcc4d3abbe47574c30
- kernel_2712.img SHA256:
  61a47b90c5ee98e7d3c4ebb934cc57c4a87e68a8944fb1563b83a357b1bae7ca
- kernel_2712.img size: 47512 bytes
- Marker: TALOS: rp1-clock-adc-ctrl-enable-toggle-result

Control candidate:

- Archive:
  target/talos-rpi5-rp1-clock-adc-ctrl-enable-toggle-no-mmio-control-core.tar.gz
- Archive SHA256:
  eb5cebf963ca2b8f9d1ec33c887c07bb053b0a3e6912cc7186211f6a4c253ca5
- kernel_2712.img SHA256:
  94fd0dbdf9a56672bec5620707715de87e1a636c335bd03208a734a54dc6d5db
- kernel_2712.img size: 47240 bytes
- Marker: TALOS: rp1-clock-adc-ctrl-enable-toggle-control

These are local/static artifacts only. No hardware behavior is accepted.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet with QEMU on PATH: pass, 423 tests.
- Archive review:
  scripts/rpi5-rp1-clock-adc-ctrl-enable-toggle-review.sh passed.
- Archive review:
  scripts/rpi5-rp1-clock-adc-ctrl-enable-toggle-no-mmio-control-review.sh
  passed.
- Static source/string inspection: passed; real candidate constructs the
  accepted 0x1f00018144 register address and performs the contracted
  enable-bit transition/restore sequence; control candidate constructs no
  forbidden RP1/GIC/MSI-X/PCIe/MIP/GPIO/pads/RIO/clock-reset MMIO address.
- git diff --check: pass.
- mdbook build: not run; no docs/src files were touched.
- git diff --cached --check: pass.

## Result

Accepted. This accepts only the local/static real candidate, the local/static
no-MMIO/no-RP1/no-GIC control candidate, and their archive/static boundary. It
does not accept Pi 5 hardware behavior, broad RP1 clock/reset ownership, GPIO
ownership, GPIO event generation, interrupt delivery, handler ownership,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, or a phase transition.

## Follow-Up

Promote phase11-rp1-clock-adc-enable-toggle-control-pi5-20260607 only after
this task is accepted and committed and hardwareTestLock remains
unlocked/restored. The next task owns the serialized Pi 5 no-MMIO/no-RP1/no-GIC
control proof before any real CLK_ADC_CTRL enable-bit transition run.
