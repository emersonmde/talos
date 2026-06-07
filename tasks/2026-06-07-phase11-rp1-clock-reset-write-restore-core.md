# Phase 11 RP1 Clock/Reset Write/Restore Core

Task id: phase11-rp1-clock-reset-write-restore-core-20260607

Status: accepted

Classification: accepted-local-static-clock-adc-ctrl-write-restore-core

## Goal

Implement the accepted CLK_ADC_CTRL idempotent write/readback/restore core and
paired no-MMIO/no-RP1/no-GIC control locally, producing candidate artifacts but
no hardware run.

## Scope

- Used only the target, operation sequence, report fields, forbidden
  operations, classifications, and control requirements accepted by
  phase11-rp1-clock-reset-write-restore-source-contract-20260607.
- Implemented the real candidate as the accepted sequence: pre-read
  CLK_ADC_CTRL, write the pre-read value back, post-read, restore-write the
  pre-read value, and restore-read.
- Implemented a paired control candidate with the same serial/output shape,
  simulated zero raw values, not-constructed address fields, and no forbidden
  RP1/GIC/MSI-X/PCIe/MIP/GPIO/pads/RIO/clock-reset MMIO address construction.
- Retained static/archive evidence for the real and control candidate
  boundary.

## Non-Goals

No Pi 5 hardware run, hardwareTestLock acquisition, published boot archive,
non-idempotent clock enable/disable, divider/source/PLL programming,
reset-controller writes, GPIO writes, event generation, interrupt enablement or
delivery, GIC IAR/EOIR acknowledgement, ISR installation, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3, or
phase transition.

## Findings

- fixed: added two explicit Pi 5 boot scenarios:
  rpi5_rp1_clock_adc_ctrl_write_restore and
  rpi5_rp1_clock_adc_ctrl_write_restore_no_mmio_control.
- fixed: added RP1_CLK_ADC_CTRL as 0x1f00018144, with a unit assertion
  matching the accepted source contract.
- fixed: the real candidate emits
  TALOS: rp1-clock-adc-ctrl-write-restore-result with the accepted contract
  id, target, register, base, offset, address, width, pre/post/restore raw and
  decoded fields, equality booleans, retained GPIO14/GPIO16 fsel 13 blocker
  context, and terminal classification.
- fixed: the real candidate performs only the accepted operation sequence for
  CLK_ADC_CTRL.
- fixed: the control candidate emits
  TALOS: rp1-clock-adc-ctrl-write-restore-control with the same field shape,
  not-constructed MMIO address fields, simulated zero raw values, and
  classification=simulated/control.
- fixed: real and control archive review scripts retain marker, report-shape,
  artifact identity, and forbidden-string checks.
- deferred: serialized Pi 5 control proof, real Pi 5 write/readback/restore
  proof, broader clock/reset ownership, GPIO ownership retries, event
  generation, interrupt delivery, handler ownership, DMA/cache, storage,
  generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3,
  and phase transition.
- not-an-issue: UART10 FR polling loads remain in both candidates because
  they are the existing firmware-preserved serial flush path, not RP1
  clock/reset/GPIO/RIO/pads/MSI-X/PCIe/MIP/GIC MMIO.

No findings were removed in this task.

## Candidate Artifacts

Real candidate:

- Archive: target/talos-rpi5-rp1-clock-adc-ctrl-write-restore-core.tar.gz
- Archive SHA256:
  64175bff01b53efdb3ab71219e24e7c9740e34a0f82bcbe31c4fc65e51e74add
- kernel_2712.img SHA256:
  19f2485cd3078e68cf29596b46493d11ba9911016ef6bafc9747d3cb0515a252
- kernel_2712.img size: 47232 bytes
- Marker: TALOS: rp1-clock-adc-ctrl-write-restore-result

Control candidate:

- Archive:
  target/talos-rpi5-rp1-clock-adc-ctrl-write-restore-no-mmio-control-core.tar.gz
- Archive SHA256:
  dd57daffc4196e8314336492fd86ee098974dd5fb30085ea2e689d24bf0de659
- kernel_2712.img SHA256:
  9dec210a6885dc38b7982ac69b83e45478b80ec965952a499cd064f29559f2f2
- kernel_2712.img size: 46888 bytes
- Marker: TALOS: rp1-clock-adc-ctrl-write-restore-control

These are local/static artifacts only. No hardware behavior is accepted.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet: pass.
- Archive review:
  scripts/rpi5-rp1-clock-adc-ctrl-write-restore-review.sh passed.
- Archive review:
  scripts/rpi5-rp1-clock-adc-ctrl-write-restore-no-mmio-control-review.sh
  passed.
- Static source/string/disassembly inspection: passed; real candidate
  constructs the accepted 0x1f00018144 register address and performs the
  contracted operation sequence; control candidate constructs no forbidden
  RP1/GIC/MSI-X/PCIe/MIP/GPIO/pads/RIO/clock-reset MMIO address.
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

Promote phase11-rp1-clock-reset-write-restore-control-pi5-20260607 only after
this task is accepted and committed and hardwareTestLock remains
unlocked/restored. The next task owns the serialized Pi 5 no-MMIO/no-RP1/no-GIC
control proof before any real CLK_ADC_CTRL write/readback/restore run.
