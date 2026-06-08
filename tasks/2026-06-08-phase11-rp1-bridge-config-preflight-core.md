# Phase 11 RP1 Bridge/Config Preflight Core

Task id: phase11-rp1-bridge-config-preflight-core-20260608

Status: accepted

Classification: accepted-local-static-bridge-config-preflight-core

## Goal

Implement the accepted bridge/config-preflight discriminator locally with a
paired no-MMIO/no-PCIe/no-RP1/no-GIC control candidate.

## Scope

- Added the real rpi5_rp1_bridge_config_preflight_read boot scenario using
  only the accepted source contract sequence from
  phase11-rp1-bridge-config-preflight-source-contract-20260608.
- Added the paired rpi5_rp1_bridge_config_preflight_no_mmio_control boot
  scenario with the same report shape and classification vocabulary while
  constructing no forbidden MMIO path.
- Added image, boot-tree, archive, and static archive-review scripts for the
  real and control candidates.
- Retained local/static evidence for both boot archives.
- Recorded findings with disposition.

## Non-Goals

No Pi 5 hardware run, boot archive publication, hardwareTestLock acquisition,
operations outside the accepted source contract, broad bridge setup, BAR
programming, endpoint ownership claim, endpoint configuration mutation,
interrupt delivery, DMA/cache, storage, generated-root, networking, SSH,
Milestone 11.3, or phase transition.

## Findings

- fixed: real candidate reads PCIE_MISC_PCIE_STATUS at 0x1000124068, decodes
  PCIE_PORT, DL_ACTIVE, PHYLINKUP, LINK_IN_L23, and status sentinel state.
- fixed: real candidate performs exactly one selected 32-bit read from
  PCIE_MISC_MISC_CTRL at 0x1000124008 and decodes SCB_ACCESS_EN,
  CFG_READ_UR_MODE, RCB_MPS_MODE, RCB_64B_MODE, MAX_BURST_SIZE, and sentinel
  state.
- fixed: real candidate reports the accepted source contract id, target,
  source offsets, CPU physical addresses, retained endpoint config identity
  classification, classification vocabulary, and terminal classification.
- fixed: control candidate reports the same output shape and classification
  vocabulary with not-constructed addresses and simulated values while
  performing no volatile MMIO load or store.
- fixed: review scripts assert the real/control markers, contract strings,
  report fields, accepted classification vocabulary, and forbidden string
  absence in the candidate archives.
- deferred: Pi 5 control proof, Pi 5 real proof, bridge setup, BAR discovery
  or programming, endpoint config retries, interrupt delivery, DMA/cache,
  storage, generated-root, networking, SSH, Milestone 11.3, and phase
  transition.
- not-an-issue: UART10 serial flush reads remain present in both candidates as
  the existing firmware-preserved serial output path, not as PCIe/RP1/GIC
  diagnostic MMIO.

No findings were removed in this task.

## Candidate Artifacts

Real candidate:

- Archive: target/talos-rpi5-rp1-bridge-config-preflight-read-core.tar.gz
- Archive SHA-256:
  f2736728a40db884b2e6ac8984a86c801bb8376ee4064d2e67380a3969140a2b
- Kernel SHA-256:
  1595ebd3d2d13005c3e48e0dd019c54058c5ff0762e4f997e141a7ef57a733b6
- Kernel size: 48,000 bytes
- Marker: TALOS: rp1-bridge-config-preflight-result

Control candidate:

- Archive:
  target/talos-rpi5-rp1-bridge-config-preflight-no-mmio-control-core.tar.gz
- Archive SHA-256:
  392aa7cc0a2ce4e102dfdee92b07c2c351766aa29f4d8446085b110ac8b15bad
- Kernel SHA-256:
  8250aa5124d06190ba3d95d3cfd9e9fbedce9ab2c2af7040c4b0d37e11c4d262
- Kernel size: 47,504 bytes
- Marker: TALOS: rp1-bridge-config-preflight-control

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-config-preflight-core/evidence-map.json.
- Real archive review:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-config-preflight-core/real-archive-review.txt.
- Control archive review:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-config-preflight-core/control-archive-review.txt.
- Real scripts:
  scripts/rpi5-rp1-bridge-config-preflight-read-image.sh,
  scripts/rpi5-rp1-bridge-config-preflight-read-boot-tree.sh,
  scripts/rpi5-rp1-bridge-config-preflight-read-archive.sh,
  scripts/rpi5-rp1-bridge-config-preflight-read-review.sh.
- Control scripts:
  scripts/rpi5-rp1-bridge-config-preflight-no-mmio-control-image.sh,
  scripts/rpi5-rp1-bridge-config-preflight-no-mmio-control-boot-tree.sh,
  scripts/rpi5-rp1-bridge-config-preflight-no-mmio-control-archive.sh,
  scripts/rpi5-rp1-bridge-config-preflight-no-mmio-control-review.sh.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed after exporting the
  local QEMU 9.2.0 tool path.
- static/archive inspection:
  scripts/rpi5-rp1-bridge-config-preflight-read-review.sh passed for the real
  archive.
- static/archive inspection:
  scripts/rpi5-rp1-bridge-config-preflight-no-mmio-control-review.sh passed
  for the control archive.
- git diff check: passed.
- docs validation: not required; no docs/src files were touched.
- git diff cached check before commit: passed.

## Result

Accepted as accepted-local-static-bridge-config-preflight-core.

Next mechanically unblocked task:
phase11-rp1-bridge-config-preflight-control-pi5-20260608 after this task is
committed and hardwareTestLock remains unlocked/restored. Publish only the
committed no-MMIO/no-PCIe/no-RP1/no-GIC control archive for that task.
