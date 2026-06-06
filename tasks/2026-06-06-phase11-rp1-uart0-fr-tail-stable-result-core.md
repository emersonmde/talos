# Phase 11 RP1 UART0 FR Tail-Stable Result Core

Task id: phase11-rp1-uart0-fr-tail-stable-result-core-20260606

Status: accepted

## Goal

Add the local/static tail-stable RP1 UART0 FR-read result candidate and matching
no-RP1-MMIO control candidate required before another hardware proof.

## Scope

- Added boot scenarios rpi5_rp1_uart0_fr_tail_stable_result and
  rpi5_rp1_uart0_fr_tail_stable_no_mmio_control.
- Routed both scenarios directly from rust_entry, before BootInfo parsing,
  target initialization, boot reports, memory planning, allocator setup,
  scheduler work, command-loop work, or broader Phase 11 behavior.
- Added a tail-stable RP1 candidate that performs exactly one 32-bit volatile
  load from RP1_UART0_FR at 0x1f00030018 and, if the load returns, loops
  forever emitting a compact read-result/classification marker containing the
  raw value.
- Added a matching no-RP1-MMIO control candidate that emits the same repeated
  result-output shape with classification=simulated/control, without
  constructing the RP1 FR address or performing an RP1 load.
- Added task-owned image, boot-tree, archive, and archive-review scripts for
  both candidates.
- Produced non-published candidate archives:
  target/talos-rpi5-rp1-uart0-fr-tail-stable-result-core.tar.gz and
  target/talos-rpi5-rp1-uart0-fr-tail-stable-no-mmio-control-core.tar.gz.

## Non-Goals

No Pi 5 hardware run, boot archive publication, hardwareTestLock acquisition,
RP1 address change, repeated RP1 loads, GPIO, pin-control, clocks, resets,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, phase transition, or hardware mapped/read-value acceptance.

## Candidates

RP1 tail-stable result archive:
target/talos-rpi5-rp1-uart0-fr-tail-stable-result-core.tar.gz

- Archive SHA-256:
  521ea4e092321f19e414020303f4127aaf5f076095bd636bf2578cc452959541
- Boot-tree identity:
  b912e810a162700602469b9039c22143cddcd587a53312bdecdc1ca4d0f04a27
- Kernel SHA-256:
  63c0eddbbd90da106a5eb20095e3108a05d53001edd017d1d6a7c4b9271cc2b8
- Kernel size: 45,800 bytes
- Arm64 Image fields: text_offset=0, header_image_size=45800, flags=12,
  magic=ARMd
- Repeated marker: TALOS: fr-tail-stable-result

No-MMIO control archive:
target/talos-rpi5-rp1-uart0-fr-tail-stable-no-mmio-control-core.tar.gz

- Archive SHA-256:
  050542fe57dc5d53ce5ee87a16c139701c8beedc25f8a1c26a8b5b1b21bdae9b
- Boot-tree identity:
  74e0c0a9c294927d8e7f377fd8272d587050bd3dd5fb471e120f126e65fd4764
- Kernel SHA-256:
  c9c473242310875f232c19c97caa82dfc2ca843c3bf159a3380d01f6b2105581
- Kernel size: 45,728 bytes
- Arm64 Image fields: text_offset=0, header_image_size=45728, flags=12,
  magic=ARMd
- Repeated marker: TALOS: fr-tail-stable-control

## Static Path

The RP1 candidate reports
rpi5-rp1-uart0-fr-tail-stable-result: before-rp1-load, flushes UART10, then
performs one volatile 32-bit load from 0x1f00030018. Assembly review shows the
selected function constructs the pointer with x19 = 0x1f00030018 and has
exactly one ldr w23, [x19]. The terminal loop repeatedly reports contract id
phase11-rp1-pcie-map-contract-v1, target rp1-uart0-fr-read, address, width 32,
the retained raw value, and classification=mapped/read-value.

The no-MMIO control candidate reports
rpi5-rp1-uart0-fr-tail-stable-control: no-rp1-mmio, then loops forever
emitting TALOS: fr-tail-stable-control with the same compact result-output
shape and classification=simulated/control. Assembly review of the selected
function shows no 0x1f00030018 construction, no read_rp1_reg_u32 reference, and
zero RP1 loads; the retained ldr w10, [x9, #24] instructions are UART10 FR
polling in the early serial flush path.

## Findings And Disposition

- fixed: added a tail-stable RP1 result candidate so a returned read result is
  repeated in the terminal serial tail instead of emitted once.
- fixed: added a matching no-MMIO control candidate with the same output shape
  and explicit simulated/control classification.
- fixed: retained task-owned archive review, candidate identity, image/header
  metadata, strings, and assembly evidence for both candidates.
- fixed: documented that the no-MMIO Pi 5 control must pass before any RP1
  mapped/read-value proof is attempted.
- deferred: Pi 5 visibility of the tail-stable no-MMIO control and RP1 result
  candidate remains queued hardware work.
- not-an-issue: source/static evidence alone does not accept RP1 mapped/read
  behavior, bus-fault/trap behavior, firmware-state behavior, GPIO, interrupts,
  DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
  11.2, or a phase transition.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core/evidence-map.json.
- Static inspection:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core/static-inspection.md.
- Assembly review:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core/assembly-review.txt.
- Archive reviews:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core/rp1-archive-review.txt and
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core/control-archive-review.txt.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed.
- image/archive inspection: both task-owned archive review scripts passed.
- static assembly inspection: passed.
- git diff --check: passed.
- mdbook build: passed because docs/src files changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as local-static-tail-stable-result-core-accepted.

The next queued task is the serialized no-MMIO Pi 5 control. RP1 UART0 FR
mapped/read-value, bus-fault/trap, firmware-state, GPIO, interrupts, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe, Milestone 11.2, and
phase transition remain unaccepted.
