# Phase 11 RP1 PCIe Endpoint/Config Discriminator Core

Task id: phase11-rp1-pcie-endpoint-config-discriminator-core-20260608

Status: accepted

Classification: accepted-local-static-pcie2-host-link-status-core

## Goal

Implement the accepted RP1 PCIe endpoint/config/decode discriminator core and
paired no-MMIO/no-RP1/no-GIC control locally, without running hardware.

## Scope

- Used only the target, read ordering, report fields, classifications, control
  requirements, and forbidden operations accepted by
  phase11-rp1-pcie-endpoint-config-discriminator-source-contract-20260608.
- Implemented the real candidate as one read-only 32-bit
  PCIE_MISC_PCIE_STATUS load at BCM2712 PCIe2 host-controller physical address
  0x1000124068.
- Implemented the paired control candidate with the same output shape,
  simulated status bits, not-constructed address fields, and no BCM2712 PCIe,
  RP1 peripheral/SYSINFO/clock/GPIO/MSI-X, MIP, GIC, or DMA MMIO address
  construction.
- Retained archive review and strings evidence for the real/control boundary.

## Non-Goals

No Pi 5 hardware run, boot archive publication beyond local/static artifacts,
hardwareTestLock acquisition, endpoint config-space access, PCIe writes,
EXT_CFG_INDEX or EXT_CFG_DATA cycles, bridge setup, PERST/link-control changes,
MSI/MIP/GIC operations, RP1 peripheral/SYSINFO/clock/GPIO operations,
DMA/cache, storage, generated-root, networking, SSH, Milestone 11.3, or phase
transition.

## Findings

- fixed: added two explicit Pi 5 boot scenarios:
  rpi5_rp1_pcie2_host_link_status_read and
  rpi5_rp1_pcie2_host_link_status_no_mmio_control.
- fixed: added source-contract constants for PCIe2 controller base
  0x1000120000, PCIE_MISC_PCIE_STATUS offset 0x4068, physical address
  0x1000124068, and the accepted PCIE_PORT, DL_ACTIVE, PHYLINKUP, and
  LINK_IN_L23 bits.
- fixed: the real candidate emits TALOS: rp1-pcie2-host-link-status-result
  with the accepted contract id, target, PCIe2 base, register name, source
  offset, physical address, width, raw status, decoded booleans, retained
  SYSINFO/clock sentinel context, and terminal classification.
- fixed: the real candidate performs only the accepted read-only volatile load
  sequence: one 32-bit PCIE_MISC_PCIE_STATUS load.
- fixed: the control candidate emits TALOS: rp1-pcie2-host-link-status-control
  with the same field shape, not-constructed address fields, simulated raw
  status bits, retained SYSINFO/clock sentinel context, and
  classification=no-mmio-pcie2-host-link-status-control-visible.
- fixed: archive review scripts retain marker, report-shape, artifact
  identity, and forbidden-string checks; the control review checks absence of
  forbidden BCM2712 PCIe, RP1, GIC, MSI-X, MIP, GPIO, clock/reset, and DMA
  address strings.
- deferred: serialized Pi 5 control proof, real Pi 5 discriminator proof,
  endpoint config-space access, bridge setup, PERST/link control, MSI/MIP/GIC
  operations, DMA/cache, storage, generated-root, networking, SSH, Milestone
  11.3, and phase transition.
- not-an-issue: UART10 FR polling loads remain in both candidates because they
  are the existing firmware-preserved serial flush path, not RP1 endpoint,
  config, PCIe host status, GIC, MSI-X/MIP, GPIO, clock/reset, or DMA MMIO.

No findings were removed in this task.

## Candidate Artifacts

Real candidate:

- Archive: target/talos-rpi5-rp1-pcie2-host-link-status-read-core.tar.gz
- Archive SHA256:
  4b4e22a86c6ae77dc71431a174240a872faa6a0ef33ddebd533832e96dbbf293
- kernel_2712.img SHA256:
  8bedf0ad171c2679ac944c58c84f7b84d02903369f5449ce4bc4f0afffca28b9
- kernel_2712.img size: 46880 bytes
- Marker: TALOS: rp1-pcie2-host-link-status-result

Control candidate:

- Archive:
  target/talos-rpi5-rp1-pcie2-host-link-status-no-mmio-control-core.tar.gz
- Archive SHA256:
  90e9d5f038faa5350b983647b781d21bb561f3bfa1b45329cd0e6bd70df99558
- kernel_2712.img SHA256:
  b85ca45e09ccb32f2776ad3ba0faac55b765ca37f9bfbaafb9b30e555f5463ae
- kernel_2712.img size: 46672 bytes
- Marker: TALOS: rp1-pcie2-host-link-status-control

These are local/static artifacts only. No hardware behavior is accepted.

## Validation

- cargo fmt --all -- --check: pass.
- cargo -Zjson-target-spec test --quiet with QEMU on PATH: pass, 423 tests.
- Archive review:
  scripts/rpi5-rp1-pcie2-host-link-status-read-review.sh passed.
- Archive review:
  scripts/rpi5-rp1-pcie2-host-link-status-no-mmio-control-review.sh passed.
- Static source/string inspection: passed; real candidate constructs only the
  accepted PCIe2 host-status read address, while control candidate constructs
  no forbidden BCM2712 PCIe/RP1/GIC/MSI-X/MIP/GPIO/clock-reset/DMA MMIO
  address.
- git diff --check: pass.
- mdbook build: not run because no docs/src files were touched.
- git diff --cached --check: pass.

## Result

Accepted. This accepts only the local/static real candidate, the local/static
no-MMIO/no-RP1/no-GIC control candidate, and their archive/static boundary. It
does not accept Pi 5 hardware behavior, endpoint config-space access, broad
RP1 mapping, endpoint ownership, PCIe writes, interrupt delivery, DMA/cache,
storage, generated-root, networking, SSH, Milestone 11.3, or a phase
transition.

## Follow-Up

Promote
phase11-rp1-pcie-endpoint-config-discriminator-control-pi5-20260608 only after
this task is accepted and committed, hardwareTestLock remains
unlocked/restored, and supervisorIntervention remains inactive. The next task
owns the serialized Pi 5 no-MMIO/no-RP1/no-GIC control proof before any real
PCIE_MISC_PCIE_STATUS hardware run.
