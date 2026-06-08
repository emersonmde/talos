# Phase 11 RP1 Observed Aperture Core

Task id: phase11-rp1-observed-aperture-core-20260608

Status: accepted

Classification: accepted-local-static-observed-aperture-core

## Goal

Implement the accepted observed-aperture discriminator as a local/static real
candidate plus paired no-MMIO control.

## Scope

- Added the real rpi5_rp1_observed_aperture_read boot scenario using only the
  accepted one-read contract from
  phase11-rp1-observed-aperture-source-contract-20260608.
- Added the paired rpi5_rp1_observed_aperture_no_mmio_control boot scenario
  with the same report shape and classification vocabulary while constructing
  no forbidden RP1/PCIe/MIP/GIC/GPIO/clock/reset/DMA MMIO path.
- Added image, boot-tree, archive, and static archive-review scripts for the
  real and control candidates.
- Retained local/static archive-review evidence for both candidates.
- Recorded findings with disposition.

## Non-Goals

No Pi 5 hardware run, boot archive publication to the lab, hardwareTestLock
acquisition, endpoint config retry, same-shaped 0x1f RP1 read rerun,
same-shaped bridge/setup rerun, BAR discovery/programming, bridge setup
writes, PERST/link-control changes, GPIO/pad/clock/reset writes, interrupt
enablement/delivery, GIC acknowledgement, DMA/cache, storage, generated-root,
networking, SSH, Milestone 11.3, or phase transition.

## Findings

- fixed: real candidate performs exactly one 32-bit volatile load from the
  accepted observed CPU physical address 0x1c00030018 and emits the
  before-read marker before that load.
- fixed: real candidate report preserves contract id, target, source RP1 bus
  address 0xc040030018, observed CPU physical address 0x1c00030018, register
  offset 0x18, width, raw value, sentinel booleans, PL011-FR-shaped boolean,
  retained bridge/setup mismatch fields, classification vocabulary, and
  terminal classification.
- fixed: real classification vocabulary is limited to the accepted contract:
  observed-aperture-rp1-uart0-fr-visible, sentinel, all-ones, zero,
  no-return-or-trap, inconclusive-capture,
  no-mmio-observed-aperture-control-visible, and staging/build-blocker.
- fixed: paired control preserves output shape and classification vocabulary
  with not-constructed address fields and simulated raw value 0x90 while
  performing no volatile RP1/PCIe/MIP/GIC/GPIO/clock/reset/DMA MMIO load or
  store.
- fixed: review scripts assert candidate/control markers, contract strings,
  report fields, accepted classification vocabulary, real 0x1c00030018 address
  string, and absence of forbidden same-shaped 0x1f/PCIe/control strings.
- deferred: Pi 5 control proof, Pi 5 real proof, live RP1 ownership, endpoint
  ownership, broad RP1 mapping, UART ownership, interrupt delivery, DMA/cache,
  storage, generated-root, networking, SSH, Milestone 11.3, and phase
  transition.
- not-an-issue: UART10 serial output MMIO remains the existing evidence
  transport path for both candidates; it is not part of the RP1 observed
  aperture under test.

No findings were removed in this task.

## Candidate Artifacts

Real candidate:

- Archive: target/talos-rpi5-rp1-observed-aperture-read-core.tar.gz
- Archive SHA-256:
  4b6511582239f8d991b8da5383ae3cc61b03ccfd961c57b4b7123e3e8b55a5ae
- Kernel SHA-256:
  4b6064d5c23d905fc903ff6ac599e7283121c188366cbd635b7fd92d5ac5e2c0
- Kernel size: 47,664 bytes
- Marker: TALOS: rp1-observed-aperture-result

Control candidate:

- Archive:
  target/talos-rpi5-rp1-observed-aperture-no-mmio-control-core.tar.gz
- Archive SHA-256:
  27c94fbcec820c0afdc9ec43d7e155a1aae92e1663297f8f2c27d96c1faf8d2e
- Kernel SHA-256:
  7903a41314b8b4cbb72aac40b4c95b5d0c242cf76fe745b27a51127dc3ff36be
- Kernel size: 47,344 bytes
- Marker: TALOS: rp1-observed-aperture-control

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-core/evidence-map.json.
- Real archive review:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-core/real-archive-review.txt.
- Control archive review:
  tasks/evidence/2026-06-08-phase11-rp1-observed-aperture-core/control-archive-review.txt.
- Real scripts:
  scripts/rpi5-rp1-observed-aperture-read-image.sh,
  scripts/rpi5-rp1-observed-aperture-read-boot-tree.sh,
  scripts/rpi5-rp1-observed-aperture-read-archive.sh,
  scripts/rpi5-rp1-observed-aperture-read-review.sh.
- Control scripts:
  scripts/rpi5-rp1-observed-aperture-no-mmio-control-image.sh,
  scripts/rpi5-rp1-observed-aperture-no-mmio-control-boot-tree.sh,
  scripts/rpi5-rp1-observed-aperture-no-mmio-control-archive.sh,
  scripts/rpi5-rp1-observed-aperture-no-mmio-control-review.sh.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed after exporting the
  local QEMU 9.2.0 tool path; 423 no_std tests passed.
- static/archive inspection:
  scripts/rpi5-rp1-observed-aperture-read-review.sh passed for the real
  archive.
- static/archive inspection:
  scripts/rpi5-rp1-observed-aperture-no-mmio-control-review.sh passed for the
  control archive.
- git diff check: passed.
- docs validation: not required; no docs/src files were touched.
- git diff cached check before commit: passed.

## Result

Accepted as accepted-local-static-observed-aperture-core.

Next mechanically unblocked task:
phase11-rp1-observed-aperture-control-pi5-20260608 after this task is
committed and hardwareTestLock remains unlocked/restored. Publish only the
committed no-MMIO/no-PCIe/no-RP1/no-GIC control archive for that task.
