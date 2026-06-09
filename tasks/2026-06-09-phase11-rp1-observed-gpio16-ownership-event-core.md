# Phase 11 RP1 Observed GPIO16 Ownership/Event Core

Task id: phase11-rp1-observed-gpio16-ownership-event-core-20260609

Status: accepted

Classification: accepted-local-static-observed-gpio16-ownership-event-core

## Goal

Implement the accepted read-only observed-aperture GPIO16 ownership/event
preflight as a local/static real candidate plus paired no-MMIO control, without
running Pi 5 hardware.

## Scope

- Updated the retained GPIO16 event-discriminator scenarios to the accepted
  observed-aperture source contract from
  phase11-rp1-observed-gpio16-ownership-event-source-contract-20260609.
- Real candidate now performs only the selected read-only observed-aperture
  RP1 GPIO/RIO/pad loads plus the accepted read-only INTID 160 GIC status
  inputs.
- Paired control preserves the same output shape while constructing no RP1
  GPIO/RIO/pads/clock/reset, MSI-X/PCIe/MIP, GIC, DMA, or other forbidden MMIO
  address.
- Retained local/static archive-review and disassembly evidence for both
  candidates.
- Recorded findings with disposition.

## Non-Goals

No Pi 5 hardware run, boot archive publication to the lab, hardwareTestLock
acquisition, GPIO/RIO/pad/INTE/CTRL writes, IRQRESET, interrupt unmasking,
IAR/EOIR acknowledgement, ISR/handler install, event generation, interrupt
delivery, GPIO14 function change, endpoint config retry, bridge setup write,
clock/reset write, DMA/cache, storage, generated-root, networking, SSH,
Milestone 11.3, or phase transition.

## Findings

- fixed: real candidate uses the current observed-aperture GPIO16 contract id
  and target, not the retained write-backed source-expected 0x1f GPIO16 event
  discriminator contract.
- fixed: real candidate performs 32-bit volatile loads from only
  0x1c000d0080, 0x1c000d0084, 0x1c000d011c, 0x1c000d0124,
  0x1c000e0000, 0x1c000e0004, 0x1c000e0008, and 0x1c000f0044 for RP1
  ownership/event preflight state.
- fixed: real candidate retains the accepted read-only parent-route status
  inputs for GIC INTID 160: GICD_ISENABLER5, GICD_ISPENDR5,
  GICD_ISACTIVER5, and GICC_HPPIR. It still performs no IAR/EOIR
  acknowledgement or interrupt-delivery claim.
- fixed: classification vocabulary now matches the accepted observed contract:
  visible, non-GPIO-function blocker, route-or-source-state blocker, sentinel,
  all-ones, zero, no-return/trap, inconclusive-capture, no-MMIO control, and
  staging/build blocker.
- fixed: paired control emits the same field shape with not-constructed address
  fields and no-mmio-observed-gpio16-ownership-event-control-visible while
  constructing no forbidden RP1/GIC address.
- fixed: archive review scripts require the new contract/target strings and
  reject retained old-contract/write-backed action/restore/control strings.
- removed: the retained source-expected write-backed GPIO16 action/restore path
  from the selected real diagnostic.
- deferred: Pi 5 no-MMIO control proof, Pi 5 real proof, GPIO ownership, event
  generation, pending generation, interrupt delivery, restore-after-write
  semantics, DMA/cache, networking, SSH, Milestone 11.3, and phase transition.
- not-an-issue: the scenario/script names still contain the historical
  discriminator wording; the emitted contract/target and archive-review gates
  now define this task as a read-only observed-aperture preflight.

No findings were removed without replacement in this task.

## Candidate Artifacts

Real candidate:

- Archive:
  target/task-evidence/2026-06-09-gpio16-core/gpio16-real.tar.gz
- Archive SHA-256:
  87f43945d5ac6c704cf194053c187ed07751c3e82e0d6e15088a760262bbe60b
- Kernel SHA-256:
  77db209655acd8f14c4cb63c3689f97d4198d6ba5adb96faf236c03423ce2760
- Kernel size: 50,640 bytes
- Marker: TALOS: rp1-gpio16-owned-event-discriminator-result

Control candidate:

- Archive:
  target/task-evidence/2026-06-09-gpio16-core/gpio16-control.tar.gz
- Archive SHA-256:
  e2c65e3a5c97ebdd00793cb009e37f3c262744e5c6482f3756d3a813bbf5f559
- Kernel SHA-256:
  be5a41097b310aeca0c60f3fb43381d1362b6fc07cfa298230ff5dc909f151f9
- Kernel size: 48,640 bytes
- Marker: TALOS: rp1-gpio16-owned-event-discriminator-control

## Evidence

- Evidence map:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-core/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-core/classification.json.
- Static implementation review:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-core/static-implementation-review.md.
- Real archive review:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-core/real-archive-review.txt.
- Control archive review:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-core/control-archive-review.txt.
- Real disassembly excerpt:
  tasks/evidence/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-core/real-disassembly.txt.
- Real scripts:
  scripts/rpi5-rp1-gpio16-owned-event-discriminator-image.sh,
  scripts/rpi5-rp1-gpio16-owned-event-discriminator-boot-tree.sh,
  scripts/rpi5-rp1-gpio16-owned-event-discriminator-archive.sh,
  scripts/rpi5-rp1-gpio16-owned-event-discriminator-review.sh.
- Control scripts:
  scripts/rpi5-rp1-gpio16-owned-event-discriminator-no-mmio-control-image.sh,
  scripts/rpi5-rp1-gpio16-owned-event-discriminator-no-mmio-control-boot-tree.sh,
  scripts/rpi5-rp1-gpio16-owned-event-discriminator-no-mmio-control-archive.sh,
  scripts/rpi5-rp1-gpio16-owned-event-discriminator-no-mmio-control-review.sh.

## Validation

- fmt/lint: cargo fmt --all -- --check passed after formatting.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- shell syntax: bash -n passed for touched GPIO16 image, boot-tree, archive,
  and review scripts.
- static/archive inspection:
  scripts/rpi5-rp1-gpio16-owned-event-discriminator-review.sh passed for the
  real archive.
- static/archive inspection:
  scripts/rpi5-rp1-gpio16-owned-event-discriminator-no-mmio-control-review.sh
  passed for the control archive.
- jq evidence-map/classification checks: passed.
- git diff --check: passed.
- mdbook build: passed because docs/src files were touched.
- git diff --cached --check: passed before commit.

## Accepted Claims

- The real local/static candidate implements the accepted observed-aperture
  GPIO16 ownership/event preflight report and read set.
- The control local/static candidate preserves output shape without
  constructing forbidden RP1, GIC, PCIe/MIP, GPIO/RIO/pads, clock/reset, DMA,
  or other MMIO addresses.

## Rejected Claims And Retained Risks

This task does not accept Pi 5 hardware behavior, GPIO ownership, event
generation, interrupt pending generation, interrupt delivery, GIC
acknowledgement, handler ownership, GPIO/RIO/pad/INTE writes, parent-route
masking writes, GPIO14 function changes, DMA/cache, networking, SSH, Milestone
11.3, or phase transition.

## Next Action

The next queued worker task is
phase11-rp1-observed-gpio16-ownership-event-control-pi5-20260609,
mechanically blocked until this core task is committed and hardwareTestLock
remains unlocked and restored.
