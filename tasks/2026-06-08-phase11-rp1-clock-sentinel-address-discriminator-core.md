# Phase 11 RP1 Clock Sentinel Address Discriminator Core

Task id: phase11-rp1-clock-sentinel-address-discriminator-core-20260608

Status: accepted

Classification: accepted-local-static-sysinfo-clock-sentinel-core

## Goal

Implement the accepted clock-sentinel address/decode discriminator core and
paired no-MMIO/no-RP1/no-GIC control locally, without running hardware.

## Scope

- Used only the target, read ordering, report fields, classifications, control
  requirements, and forbidden operations accepted by
  phase11-rp1-clock-sentinel-address-discriminator-source-contract-20260608.
- Implemented the real candidate as a read-only SYSINFO identity versus
  retained clock-window sentinel comparison:
  SYSINFO_CHIP_ID, SYSINFO_PLATFORM, and CLK_ADC_CTRL.
- Implemented the paired control candidate with the same output shape,
  simulated raw values, not-constructed address fields, and no forbidden RP1,
  GIC, MSI-X, PCIe, MIP, GPIO, pads, RIO, or clock/reset MMIO address
  construction.
- Retained archive review and strings evidence for the real/control boundary.

## Non-Goals

No Pi 5 hardware run, boot archive publication beyond local/static artifacts,
hardwareTestLock acquisition, RP1 clock/reset writes, reset-controller writes,
GPIO/RIO/pad writes, event generation, interrupt enablement or delivery, GIC
IAR/EOIR acknowledgement, ISR installation, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3, or
phase transition.

## Findings

- fixed: added two explicit Pi 5 boot scenarios:
  rpi5_rp1_sysinfo_clock_sentinel_read and
  rpi5_rp1_sysinfo_clock_sentinel_no_mmio_control.
- fixed: added SYSINFO base, SYSINFO_CHIP_ID, SYSINFO_PLATFORM, and expected
  RP1 C0 chip-id constants with unit assertions matching the accepted source
  contract.
- fixed: the real candidate emits TALOS:
  rp1-sysinfo-clock-sentinel-result with the accepted contract id, target,
  SYSINFO base, clock-manager base, register offsets, physical addresses,
  width, raw values, expected chip id, accepted booleans, retained ADC
  clock-window sentinel context, and terminal classification.
- fixed: the real candidate performs only the accepted read-only volatile load
  sequence: SYSINFO_CHIP_ID, SYSINFO_PLATFORM, and CLK_ADC_CTRL.
- fixed: the control candidate emits TALOS:
  rp1-sysinfo-clock-sentinel-control with the same field shape,
  not-constructed address fields, simulated raw values, and
  classification=no-mmio-sysinfo-clock-sentinel-control-visible.
- fixed: archive review scripts retain marker, report-shape, artifact
  identity, and forbidden-string checks; the control review checks absence of
  forbidden RP1/GIC/MSI-X/PCIe/MIP/GPIO/pads/RIO/clock-reset address strings.
- deferred: serialized Pi 5 control proof, real Pi 5 discriminator proof,
  broad clock/reset ownership, GPIO ownership, event generation, interrupt
  delivery, handler ownership, DMA/cache, storage, generated-root, networking,
  SSH, broader PCIe enumeration, Milestone 11.3, and phase transition.
- not-an-issue: UART10 FR polling loads remain in both candidates because they
  are the existing firmware-preserved serial flush path, not RP1 SYSINFO,
  clock/reset, GPIO/RIO/pads, MSI-X/PCIe/MIP, or GIC MMIO.

No findings were removed in this task.

## Candidate Artifacts

Real candidate:

- Archive: target/talos-rpi5-rp1-sysinfo-clock-sentinel-read-core.tar.gz
- Archive SHA256:
  f60e5899e994c4be98ccd3ac826b5c88f271db968056aff6afb9c1cf705fe42a
- kernel_2712.img SHA256:
  b61eb83442ee5bd332da0de8e53b42c63d4b9950a5a9b81db5f1abfc26bf1794
- kernel_2712.img size: 47776 bytes
- Marker: TALOS: rp1-sysinfo-clock-sentinel-result

Control candidate:

- Archive:
  target/talos-rpi5-rp1-sysinfo-clock-sentinel-no-mmio-control-core.tar.gz
- Archive SHA256:
  e5d19fde6321e19684ee8d8e5970baf88e2eaeed0bf9e3252e8ed64958d51041
- kernel_2712.img SHA256:
  1aa668c7b0665a0f1148f7bd48435a324f80aad0347e6971a1efc8538ddd150c
- kernel_2712.img size: 47288 bytes
- Marker: TALOS: rp1-sysinfo-clock-sentinel-control

These are local/static artifacts only. No hardware behavior is accepted.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet with QEMU on PATH: pass, 423 tests.
- Archive review:
  scripts/rpi5-rp1-sysinfo-clock-sentinel-read-review.sh passed.
- Archive review:
  scripts/rpi5-rp1-sysinfo-clock-sentinel-no-mmio-control-review.sh passed.
- Static source/string inspection: passed; real candidate constructs only the
  accepted RP1 SYSINFO and CLK_ADC_CTRL read addresses, while control
  candidate constructs no forbidden RP1/GIC/MSI-X/PCIe/MIP/GPIO/pads/RIO or
  clock/reset MMIO address.
- git diff --check: pass.
- mdbook build: not run because no docs/src files were touched.
- git diff --cached --check: pass.

## Result

Accepted. This accepts only the local/static real candidate, the local/static
no-MMIO/no-RP1/no-GIC control candidate, and their archive/static boundary. It
does not accept Pi 5 hardware behavior, broad RP1 clock/reset ownership, any
RP1 clock/reset write, GPIO ownership, GPIO event generation, interrupt
delivery, handler ownership, DMA/cache, storage, generated-root, networking,
SSH, broader PCIe enumeration, Milestone 11.3, or a phase transition.

## Follow-Up

Promote
phase11-rp1-clock-sentinel-address-discriminator-control-pi5-20260608 only
after this task is accepted and committed, hardwareTestLock remains
unlocked/restored, and supervisorIntervention remains inactive. The next task
owns the serialized Pi 5 no-MMIO/no-RP1/no-GIC control proof before any real
SYSINFO-vs-clock-sentinel run.
