# Phase 11 RP1 Endpoint Config Identity Core

Task id: phase11-rp1-endpoint-config-identity-core-20260608

Status: accepted

Classification: accepted-local-static-endpoint-config-identity-core

## Goal

Implement the accepted RP1 endpoint config identity-read discriminator core and
paired no-MMIO/no-RP1/no-GIC control locally, without running hardware.

## Scope

- Added the real rpi5_rp1_endpoint_config_identity_read boot scenario using
  the accepted source contract sequence from
  phase11-rp1-endpoint-config-identity-source-contract-20260608.
- Added the paired rpi5_rp1_endpoint_config_identity_no_mmio_control boot
  scenario with the same report shape and no constructed forbidden MMIO
  addresses.
- Added image, boot-tree, archive, and static archive-review scripts for the
  real and control candidates.
- Retained local/static evidence for both boot archives.
- Recorded findings with disposition.

## Non-Goals

No hardware run, hardwareTestLock acquisition, boot archive publication,
endpoint configuration mutation, EXT_CFG_DATA write, BAR programming, bridge
setup, PERST/link-control changes, MSI/MIP/GIC operation, RP1 peripheral
retry, RP1 SYSINFO/clock/GPIO operation, DMA/cache, storage, generated-root,
networking, SSH, Milestone 11.3, or phase transition.

## Findings

- fixed: real candidate reads PCIE_MISC_PCIE_STATUS at 0x1000124068,
  decodes DL_ACTIVE and PHYLINKUP, and skips config access if either link bit
  is not set or the status read is the 0xdeaddead capture sentinel.
- fixed: real candidate performs exactly one accepted controller index write
  when link is ready: EXT_CFG_INDEX at 0x1000129000 gets 0x00100000 for
  0002:01:00.0 offset 0x0.
- fixed: real candidate performs exactly one 32-bit EXT_CFG_DATA + 0 read at
  0x1000128000, decodes vendor/device, reports sentinel booleans, and uses
  only the accepted classification vocabulary.
- fixed: control candidate reports the same contract, target, BDF, offsets,
  index value, expected IDs, booleans, and classification field while using
  not-constructed for controller/status/index/data addresses and performing no
  volatile MMIO load/store.
- fixed: review scripts assert the real/control markers, contract strings,
  report fields, and forbidden string absence in the candidate archives.
- deferred: Pi 5 publication and serial/TFTP/restore proof remain queued for
  the serialized control and real hardware tasks.

No findings were removed or classified as not-an-issue.

## Candidate Artifacts

Real candidate:

- Archive: target/talos-rpi5-rp1-endpoint-config-identity-read-core.tar.gz
- Archive SHA-256:
  f3f158bb9e319b36d39188619c8c8372611ad9451945bcf7be4f741411092d1a
- Kernel SHA-256:
  d8d679cc74a10b954c7723bc665842d12a07120612eac6d8e14bb8d171a68cea
- Kernel size: 48,456 bytes
- Marker: TALOS: rp1-endpoint-config-identity-result

Control candidate:

- Archive:
  target/talos-rpi5-rp1-endpoint-config-identity-no-mmio-control-core.tar.gz
- Archive SHA-256:
  53617447da04e045f09a9eaa3e10a4205046b2e73fe3a54b58263b097cba975e
- Kernel SHA-256:
  853f553f44457c848f28a8c977c5d821541c9509d7c83ab74e9911c1717849ba
- Kernel size: 47,608 bytes
- Marker: TALOS: rp1-endpoint-config-identity-control

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-endpoint-config-identity-core/evidence-map.json.
- Real archive review:
  tasks/evidence/2026-06-08-phase11-rp1-endpoint-config-identity-core/real-archive-review.txt.
- Control archive review:
  tasks/evidence/2026-06-08-phase11-rp1-endpoint-config-identity-core/control-archive-review.txt.
- Real scripts:
  scripts/rpi5-rp1-endpoint-config-identity-read-image.sh,
  scripts/rpi5-rp1-endpoint-config-identity-read-boot-tree.sh,
  scripts/rpi5-rp1-endpoint-config-identity-read-archive.sh,
  scripts/rpi5-rp1-endpoint-config-identity-read-review.sh.
- Control scripts:
  scripts/rpi5-rp1-endpoint-config-identity-no-mmio-control-image.sh,
  scripts/rpi5-rp1-endpoint-config-identity-no-mmio-control-boot-tree.sh,
  scripts/rpi5-rp1-endpoint-config-identity-no-mmio-control-archive.sh,
  scripts/rpi5-rp1-endpoint-config-identity-no-mmio-control-review.sh.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- static/archive inspection:
  scripts/rpi5-rp1-endpoint-config-identity-read-review.sh passed for the real
  archive.
- static/archive inspection:
  scripts/rpi5-rp1-endpoint-config-identity-no-mmio-control-review.sh passed
  for the control archive.
- git diff check: passed.
- docs validation: not required; no docs/src files were touched.
- git diff cached check before commit: passed.

## Result

Accepted as accepted-local-static-endpoint-config-identity-core.

Next mechanically unblocked task:
phase11-rp1-endpoint-config-identity-control-pi5-20260608 after this task is
committed and hardwareTestLock remains unlocked/restored. Publish only the
committed no-MMIO/no-RP1/no-GIC control archive for that task.
