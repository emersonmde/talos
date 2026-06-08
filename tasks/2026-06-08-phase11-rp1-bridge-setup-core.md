# Phase 11 RP1 Bridge/Setup Core

Task id: phase11-rp1-bridge-setup-core-20260608

Status: accepted

Classification: accepted-local-static-bridge-setup-core

## Goal

Implement the accepted bridge/setup-state source contract locally with a
paired no-MMIO/no-PCIe/no-RP1/no-GIC control candidate.

## Scope

- Added the real rpi5_rp1_bridge_setup_state_read boot scenario using only
  the accepted read-only sequence from
  phase11-rp1-bridge-setup-source-contract-20260608.
- Added the paired rpi5_rp1_bridge_setup_state_no_mmio_control boot scenario
  with the same report shape and classification vocabulary while constructing
  no forbidden MMIO path.
- Added image, boot-tree, archive, and static archive-review scripts for the
  real and control candidates.
- Retained local/static archive-review evidence for both candidates.
- Recorded findings with disposition.

## Non-Goals

No Pi 5 hardware run, boot archive publication, hardwareTestLock acquisition,
source-contract expansion, endpoint configuration retry, endpoint ownership
claim, BAR discovery or programming, bridge setup writes, CPU-to-PCIe window
programming, interrupt delivery, DMA/cache, storage, generated-root,
networking, SSH, Milestone 11.3, or phase transition.

## Findings

- fixed: real candidate reads only PCIE_MISC_PCIE_STATUS at 0x1000124068,
  PCIE_MISC_MISC_CTRL at 0x1000124008, PCIE_RC_CFG_PRIV1_ID_VAL3 at
  0x100012043c, and outbound window 0 registers at 0x100012400c,
  0x1000124010, 0x1000124070, 0x1000124080, and 0x1000124084.
- fixed: real candidate decodes the accepted link/preflight predicates, root
  complex class code 0x060400, and the retained pcie2 PCIe 0 to CPU
  0x1f_0000_0000 outbound window shape.
- fixed: terminal classifications are limited to the accepted vocabulary:
  pcie2-bridge-setup-state-visible, incomplete, sentinel, link-down-skip,
  inconclusive-capture, no-mmio-pcie2-bridge-setup-state-control-visible, and
  staging/build-blocker.
- fixed: paired control preserves the report fields and classification
  vocabulary with not-constructed addresses and simulated source-expected
  values while performing no volatile MMIO load or store.
- fixed: review scripts assert candidate/control markers, contract strings,
  output fields, classification vocabulary, and absence of forbidden endpoint
  config/BAR/DMA/control strings.
- deferred: Pi 5 control proof, Pi 5 real proof, endpoint visibility retry,
  endpoint ownership, BAR discovery/programming, bridge setup writes,
  interrupt delivery, DMA/cache, storage, generated-root, networking, SSH,
  Milestone 11.3, and phase transition.
- not-an-issue: UART10 serial flush reads remain present in both candidates as
  the existing firmware-preserved serial output path, not as PCIe/RP1/GIC
  diagnostic MMIO.

No findings were removed in this task.

## Candidate Artifacts

Real candidate:

- Archive: target/talos-rpi5-rp1-bridge-setup-state-read-core.tar.gz
- Archive SHA-256:
  2ed822cd0b2e6491da6a6d9447456d83228a694358326b7a15a5ab663f251d17
- Kernel SHA-256:
  8a1c6a6cd64ecbc1228a4eda56a0cecbea041590c12bb0b60ba2674e0ac5a71b
- Kernel size: 50,736 bytes
- Marker: TALOS: rp1-bridge-setup-state-result

Control candidate:

- Archive:
  target/talos-rpi5-rp1-bridge-setup-state-no-mmio-control-core.tar.gz
- Archive SHA-256:
  2173cad5a63e41b0ceb704eed480cd3f5278cce7adce600d3a2c624bd81390b4
- Kernel SHA-256:
  a741904c5c7a75b1f9db0ea811c1f9adc79ff4bfb7a4d05e655ce2e5eaaf2d10
- Kernel size: 49,496 bytes
- Marker: TALOS: rp1-bridge-setup-state-control

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-core/evidence-map.json.
- Real archive review:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-core/real-archive-review.txt.
- Control archive review:
  tasks/evidence/2026-06-08-phase11-rp1-bridge-setup-core/control-archive-review.txt.
- Real scripts:
  scripts/rpi5-rp1-bridge-setup-state-read-image.sh,
  scripts/rpi5-rp1-bridge-setup-state-read-boot-tree.sh,
  scripts/rpi5-rp1-bridge-setup-state-read-archive.sh,
  scripts/rpi5-rp1-bridge-setup-state-read-review.sh.
- Control scripts:
  scripts/rpi5-rp1-bridge-setup-state-no-mmio-control-image.sh,
  scripts/rpi5-rp1-bridge-setup-state-no-mmio-control-boot-tree.sh,
  scripts/rpi5-rp1-bridge-setup-state-no-mmio-control-archive.sh,
  scripts/rpi5-rp1-bridge-setup-state-no-mmio-control-review.sh.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed after exporting the
  local QEMU 9.2.0 tool path.
- static/archive inspection:
  scripts/rpi5-rp1-bridge-setup-state-read-review.sh passed for the real
  archive.
- static/archive inspection:
  scripts/rpi5-rp1-bridge-setup-state-no-mmio-control-review.sh passed for
  the control archive.
- git diff check: passed.
- docs validation: not required; no docs/src files were touched.
- git diff cached check before commit: passed.

## Result

Accepted as accepted-local-static-bridge-setup-core.

Next mechanically unblocked task:
phase11-rp1-bridge-setup-control-pi5-20260608 after this task is committed and
hardwareTestLock remains unlocked/restored. Publish only the committed
no-MMIO/no-PCIe/no-RP1/no-GIC control archive for that task.
