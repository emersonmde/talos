# Phase 11 RP1 Observed GPIO14 Ownership/Route Core

Task id: phase11-rp1-observed-gpio-ownership-route-core-20260608

Status: accepted

Classification: accepted-local-static-observed-gpio14-ownership-route-core

## Goal

Implement the accepted read-only observed-aperture GPIO14 ownership and parent
route preflight as a local/static real candidate plus paired no-MMIO control,
without running Pi 5 hardware.

## Scope

- Updated the existing GPIO14 ownership/route preflight scenarios to the
  accepted observed-aperture source contract from
  phase11-rp1-observed-gpio-ownership-route-source-contract-20260608.
- Real candidate now performs only the selected read-only observed-aperture
  RP1 GPIO/RIO/pad loads plus the accepted read-only INTID 160 GIC status
  inputs.
- Paired control preserves the same output shape while constructing no RP1
  GPIO/RIO/pads/clock/reset, MSI-X/PCIe/MIP, GIC, DMA, or other forbidden MMIO
  address.
- Retained local/static archive-review evidence for both candidates.
- Recorded findings with disposition.

## Non-Goals

No Pi 5 hardware run, boot archive publication to the lab, hardwareTestLock
acquisition, GPIO/RIO/pad/INTE/CTRL writes, IRQRESET, interrupt unmasking,
IAR/EOIR acknowledgement, ISR/handler install, event generation, interrupt
delivery, endpoint config retry, bridge setup write, DMA/cache, storage,
generated-root, networking, SSH, Milestone 11.3, or phase transition.

## Findings

- fixed: real candidate uses the current observed-aperture contract id and
  target, not the retained 0x1f GPIO ownership/restore contract.
- fixed: real candidate performs 32-bit volatile loads from only
  0x1c000d0070, 0x1c000d0074, 0x1c000d011c, 0x1c000d0124,
  0x1c000e0000, 0x1c000e0004, 0x1c000e0008, and 0x1c000f003c for RP1
  ownership-adjacent state.
- fixed: real candidate retains the accepted read-only parent-route status
  inputs for GIC INTID 160: GICD_ISENABLER5, GICD_ISPENDR5,
  GICD_ISACTIVER5, and GICC_HPPIR. It still performs no IAR/EOIR
  acknowledgement or interrupt-delivery claim.
- fixed: classification vocabulary now matches the accepted observed contract:
  visible, non-GPIO-function blocker, route-or-source-state blocker, sentinel,
  all-ones, zero, no-return/trap, inconclusive-capture, no-MMIO control, and
  staging/build blocker.
- fixed: paired control emits the same field shape with not-constructed address
  fields and no-mmio-observed-gpio14-ownership-route-control-visible while
  constructing no forbidden RP1/GIC address.
- fixed: archive review scripts require the new contract/target strings and
  reject retained old-contract/control strings.
- deferred: Pi 5 no-MMIO control proof, Pi 5 real proof, GPIO ownership, event
  generation, pending generation, interrupt delivery, restore-after-write
  semantics, DMA/cache, networking, SSH, Milestone 11.3, and phase transition.
- not-an-issue: UART10 remains the existing evidence transport path for both
  candidates; it is not part of the RP1 observed-aperture preflight under test.

No findings were removed in this task.

## Candidate Artifacts

Real candidate:

- Archive:
  target/talos-rpi5-rp1-observed-gpio14-ownership-route-preflight-read-core.tar.gz
- Archive SHA-256:
  fd605773116d65caa532307226af2fbc0ca92d747cf09f499031d923abec81e3
- Kernel SHA-256:
  fe08c6380936a9cd97d6bdbb8ca7ed755075fab5b3b04634c26267be4fdfbee6
- Kernel size: 50,496 bytes
- Marker: TALOS: rp1-gpio14-ownership-route-preflight-result

Control candidate:

- Archive:
  target/talos-rpi5-rp1-observed-gpio14-ownership-route-preflight-no-mmio-control-core.tar.gz
- Archive SHA-256:
  cca7afb1108926e5bc8c706e353236151423b4f4e73ce1856060adc7f0c6fae0
- Kernel SHA-256:
  c44ef9845753726644323a0f1e460eae95dd89837d7656fe6ac591ce63427793
- Kernel size: 48,432 bytes
- Marker: TALOS: rp1-gpio14-ownership-route-preflight-control

## Evidence

- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-core/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-core/classification.json.
- Static implementation review:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-core/static-implementation-review.md.
- Real archive review:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-core/real-archive-review.txt.
- Control archive review:
  tasks/evidence/2026-06-08-phase11-rp1-observed-gpio-ownership-route-core/control-archive-review.txt.
- Real scripts:
  scripts/rpi5-rp1-gpio14-ownership-route-preflight-read-image.sh,
  scripts/rpi5-rp1-gpio14-ownership-route-preflight-read-boot-tree.sh,
  scripts/rpi5-rp1-gpio14-ownership-route-preflight-read-archive.sh,
  scripts/rpi5-rp1-gpio14-ownership-route-preflight-read-review.sh.
- Control scripts:
  scripts/rpi5-rp1-gpio14-ownership-route-preflight-no-mmio-control-image.sh,
  scripts/rpi5-rp1-gpio14-ownership-route-preflight-no-mmio-control-boot-tree.sh,
  scripts/rpi5-rp1-gpio14-ownership-route-preflight-no-mmio-control-archive.sh,
  scripts/rpi5-rp1-gpio14-ownership-route-preflight-no-mmio-control-review.sh.

## Validation

- fmt/lint: cargo fmt --all -- --check passed after formatting.
- unit tests: cargo -Zjson-target-spec test --quiet passed after exporting the
  local QEMU 9.2.0 tool path.
- shell syntax: bash -n passed for touched archive review scripts.
- static/archive inspection:
  scripts/rpi5-rp1-gpio14-ownership-route-preflight-read-review.sh passed for
  the real archive.
- static/archive inspection:
  scripts/rpi5-rp1-gpio14-ownership-route-preflight-no-mmio-control-review.sh
  passed for the control archive.
- jq evidence-map/classification checks: passed.
- git diff --check: passed.
- mdbook build: passed because docs/src/project/phase11-rp1-pcie-map-contract.md
  was updated.
- git diff --cached --check: passed before commit.

## Accepted Claims

- The real local/static candidate implements the accepted observed-aperture
  GPIO14 ownership/route preflight report and read set.
- The control local/static candidate preserves output shape without
  constructing forbidden RP1, GIC, PCIe/MIP, GPIO/RIO/pads, clock/reset, DMA,
  or other MMIO addresses.

## Rejected Claims And Retained Risks

This task does not accept Pi 5 hardware behavior, GPIO ownership, event
generation, interrupt pending generation, interrupt delivery, GIC
acknowledgement, handler ownership, GPIO/RIO/pad/INTE writes, parent-route
masking writes, DMA/cache, networking, SSH, Milestone 11.3, or phase
transition.

## Next Action

The next queued worker task is
phase11-rp1-observed-gpio-ownership-route-control-pi5-20260608, mechanically
blocked until this core task is committed and hardwareTestLock remains
unlocked and restored.
