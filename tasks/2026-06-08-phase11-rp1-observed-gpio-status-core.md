# Phase 11 RP1 Observed GPIO Status Core

Task id: phase11-rp1-observed-gpio-status-core-20260608

Status: accepted

Classification: accepted-local-static-observed-gpio-status-core

## Goal

Implement the accepted observed-aperture GPIO14 status/control discriminator as
a local/static real candidate plus paired no-MMIO control, without running Pi 5
hardware.

## Scope

- Added the real rpi5_rp1_observed_gpio_status_read boot scenario using only
  the source-contract-selected read pair from
  phase11-rp1-observed-gpio-status-source-contract-20260608.
- Added the paired rpi5_rp1_observed_gpio_status_no_mmio_control boot scenario
  with the same report shape and classification vocabulary while constructing
  no forbidden RP1/PCIe/MIP/GIC/GPIO/RIO/pads/clock/reset/DMA MMIO address.
- Added image, boot-tree, archive, and static archive-review scripts for the
  real and control candidates.
- Retained local/static archive-review evidence for both candidates.
- Recorded findings with disposition.

## Non-Goals

No Pi 5 hardware run, boot archive publication to the lab, hardwareTestLock
acquisition, endpoint ownership claim, broad RP1 mapping claim, endpoint config
retry, BAR discovery/programming, bridge setup writes, PERST/link-control
changes, GPIO/pad/clock/reset writes, interrupt enablement/delivery, GIC
acknowledgement, DMA/cache, storage, generated-root, networking, SSH,
Milestone 11.3, phase transition, or source-contract expansion.

## Findings

- fixed: real candidate emits the accepted start and before-read markers, then
  performs exactly two 32-bit volatile loads from observed CPU physical
  addresses 0x1c000d0070 and 0x1c000d0074.
- fixed: real candidate report preserves contract id, target, source RP1 bus
  addresses 0xc0400d0070 and 0xc0400d0074, observed CPU physical addresses,
  register offsets 0x70 and 0x74, width, raw STATUS/CTRL values, STATUS raw
  and filtered event booleans, CTRL funcsel/outover/oeover/inover/irqover,
  CTRL raw and filtered IRQ-enable booleans, sentinel booleans for each
  register, retained 0x1c00030018 UART0 FR context, classification vocabulary,
  and terminal classification.
- fixed: real classification logic accepts visibility only when the returned
  pair is not the all-sentinel, all-ones, or all-zero pair; it does not imply
  GPIO ownership, interrupt delivery, or broad RP1 mapping.
- fixed: paired control preserves output shape and classification vocabulary
  with not-constructed address fields and simulated STATUS/CTRL raw values,
  while performing no volatile RP1/PCIe/MIP/GIC/GPIO/RIO/pads/clock/reset/DMA
  MMIO load or store.
- fixed: review scripts assert candidate/control markers, contract strings,
  report fields, accepted classification vocabulary, real 0x1c observed
  address strings, and absence of forbidden same-shaped 0x1f GPIO, IO_BANK0,
  PCIe, GIC, and control strings.
- deferred: Pi 5 control proof, Pi 5 real proof, GPIO ownership, event
  generation, interrupt pending generation, interrupt delivery, endpoint
  ownership, broad RP1 mapping, pad/RIO/clock/reset ownership, DMA/cache,
  networking, SSH, Milestone 11.3, and phase transition.
- not-an-issue: UART10 serial output MMIO remains the existing evidence
  transport path for both candidates; it is not part of the RP1 observed
  GPIO status aperture under test.

No findings were removed in this task.

## Candidate Artifacts

Real candidate:

- Archive: target/talos-rpi5-rp1-observed-gpio-status-read-core.tar.gz
- Archive SHA-256:
  48ce03225e1f0e8eba6fadd9e976c110729c8fed1e23338eee9c06f42635dc2d
- Kernel SHA-256:
  c66497ccc7b2001152616254872d3ff187ab8953e0786595eb2f7ab4069dbae1
- Kernel size: 49,656 bytes
- Marker: TALOS: rp1-observed-gpio-status-result

Control candidate:

- Archive:
  target/talos-rpi5-rp1-observed-gpio-status-no-mmio-control-core.tar.gz
- Archive SHA-256:
  39976936b2415be637d4ded0ab7b36c23bd9df2abb0061d4ccb5370a9e22a325
- Kernel SHA-256:
  8b211565dbe7d7f8138e9785eb0b7e8b74acd0010b195a45745eedd7e9c65d93
- Kernel size: 48,952 bytes
- Marker: TALOS: rp1-observed-gpio-status-control

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-core/evidence-map.json.
- Real archive review:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-core/real-archive-review.txt.
- Control archive review:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-status-core/control-archive-review.txt.
- Real scripts:
  scripts/rpi5-rp1-observed-gpio-status-read-image.sh,
  scripts/rpi5-rp1-observed-gpio-status-read-boot-tree.sh,
  scripts/rpi5-rp1-observed-gpio-status-read-archive.sh,
  scripts/rpi5-rp1-observed-gpio-status-read-review.sh.
- Control scripts:
  scripts/rpi5-rp1-observed-gpio-status-no-mmio-control-image.sh,
  scripts/rpi5-rp1-observed-gpio-status-no-mmio-control-boot-tree.sh,
  scripts/rpi5-rp1-observed-gpio-status-no-mmio-control-archive.sh,
  scripts/rpi5-rp1-observed-gpio-status-no-mmio-control-review.sh.

## Validation

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed after exporting the
  local QEMU 9.2.0 tool path; 423 no_std tests passed.
- static/archive inspection:
  scripts/rpi5-rp1-observed-gpio-status-read-review.sh passed for the real
  archive.
- static/archive inspection:
  scripts/rpi5-rp1-observed-gpio-status-no-mmio-control-review.sh passed for
  the control archive.
- jq evidence-map check: passed.
- git diff check: passed.
- mdbook build: not run because no docs/src files were touched.
- git diff --cached --check: passed before commit.

## Accepted Claims

- The real local/static candidate implements exactly the accepted two-read
  observed-aperture GPIO14 STATUS/CTRL contract.
- The control local/static candidate preserves output shape without
  constructing forbidden RP1, PCIe, MIP, GIC, GPIO, RIO, pads, clock/reset,
  DMA, or other MMIO addresses.

## Rejected Claims And Retained Risks

This task does not accept Pi 5 hardware behavior, GPIO ownership, event
generation, interrupt pending generation, interrupt delivery, endpoint
ownership, broad RP1 mapping, pad/RIO/clock/reset ownership, DMA/cache,
networking, SSH, Milestone 11.3, or phase transition.

Same-shaped endpoint config identity, bridge/setup-state, 0x1f RP1 peripheral,
0x1f GPIO/status, 0x1f GPIO bank source-status, and 0x1c UART0 FR hardware
reruns remain closed unless a future supervisor task supplies a different
discriminator or new acceptance criteria.

## Next Action

The next queued worker task is
phase11-rp1-observed-gpio-status-control-pi5-20260608, mechanically blocked
until this core task is committed and hardwareTestLock remains unlocked and
restored.
