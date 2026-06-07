# Phase 11 RP1 Clock/Reset Write/Restore Core Static Inspection

Task: phase11-rp1-clock-reset-write-restore-core-20260607

Evidence level: static source, archive, strings, and disassembly inspection.

## Inputs

- Real archive review:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-write-restore-core/real-archive-review.txt
- Control archive review:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-write-restore-core/control-archive-review.txt
- Real strings:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-write-restore-core/real-candidate-strings.txt
- Control strings:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-write-restore-core/control-candidate-strings.txt
- Real disassembly:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-write-restore-core/real-clock-adc-ctrl-write-restore-asm.txt
- Control disassembly:
  tasks/evidence/2026-06-07-phase11-rp1-clock-reset-write-restore-core/control-clock-adc-ctrl-write-restore-asm.txt

## Real Candidate Boundary

- The real candidate is rpi5_rp1_clock_adc_ctrl_write_restore, implementing
  only the accepted phase11-rp1-clock-reset-write-restore-source-contract-v1
  target rp1-clk-adc-ctrl-idempotent-write-restore.
- The selected register is CLK_ADC_CTRL at CPU physical 0x1f00018144, derived
  from RP1_CLOCK_MANAGER_BASE + 0x144.
- The selected disassembly for run_rp1_clock_adc_ctrl_write_restore shows the
  exact bounded operation sequence after the UART preflush: ldr w20, [x19],
  str w20, [x19], dsb sy, ldr w24, [x19], str w20, [x19], dsb sy, ldr w25,
  [x19]. The retained address setup uses the accepted 0x1f...8144 register
  address.
- The real report loop emits the accepted fields: contract id, target,
  register, clock manager base, source offset, address, width, pre/post/restore
  raw values, pre/post/restore enable/auxsrc/source decode fields,
  post/restore equality booleans, retained GPIO14/GPIO16 fsel 13 blocker
  context, and terminal classification.
- The real classifications are limited to
  rp1-clock-adc-ctrl-idempotent-write-restored,
  rp1-clock-adc-ctrl-idempotent-write-mismatch-restored, and
  rp1-clock-adc-ctrl-idempotent-write-restore-failed for local/static core
  implementation. Missing-manager, inconclusive-capture, and staging/build
  classifications remain hardware/staging classifications for later tasks.

## Control Boundary

- The control candidate is
  rpi5_rp1_clock_adc_ctrl_write_restore_no_mmio_control.
- It emits the paired marker TALOS: rp1-clock-adc-ctrl-write-restore-control,
  the same contract id, target, register, width, raw/decode/equality field
  shape, retained blocker fields, and classification=simulated/control.
- It reports clock-manager-base=not-constructed and address=not-constructed,
  uses simulated zero raw values, and constructs no RP1 clock/reset,
  GPIO/RIO/pads, MSI-X/PCIe/MIP, or GIC MMIO address.
- The control archive review checks the absence of the real marker, real
  classifications, selected 0x1f00018144 clock register string, broader RP1
  clock/GPIO/GIC address strings, and the GIC distributor/CPU-interface
  address strings.
- The selected control disassembly contains the expected UART10 FR polling path
  for serial flushing, but no selected RP1 clock/reset write/readback register
  sequence.

## Disposition

- fixed: added the explicit Pi 5 boot scenarios
  rpi5_rp1_clock_adc_ctrl_write_restore and
  rpi5_rp1_clock_adc_ctrl_write_restore_no_mmio_control.
- fixed: added RP1_CLK_ADC_CTRL = 0x1f00018144 with a unit assertion.
- fixed: the real candidate implements only the accepted pre-read, idempotent
  write, post-read, restore-write, and restore-read sequence for CLK_ADC_CTRL.
- fixed: the control candidate preserves the output shape while avoiding
  forbidden RP1/GIC/MSI-X/PCIe/MIP/GPIO/pads/RIO/clock-reset MMIO address
  construction.
- fixed: archive review scripts retain candidate identity, report-shape, and
  forbidden-string checks for both candidates.
- deferred: serialized Pi 5 control proof, real Pi 5 write/readback/restore
  proof, broader clock/reset ownership, GPIO ownership retries, event
  generation, interrupt delivery, handler ownership, DMA/cache, storage,
  generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3,
  and phase transition.
- not-an-issue: UART10 FR polling remains the existing firmware-preserved
  serial flush path used by both candidates.

No findings were removed in this task.
