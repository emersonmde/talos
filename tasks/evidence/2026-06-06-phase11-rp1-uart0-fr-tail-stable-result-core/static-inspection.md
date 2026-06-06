# Static Inspection

Task: phase11-rp1-uart0-fr-tail-stable-result-core-20260606

## Source

- Added boot scenarios rpi5_rp1_uart0_fr_tail_stable_result and
  rpi5_rp1_uart0_fr_tail_stable_no_mmio_control.
- rust_entry routes both scenarios directly into their Pi 5 diagnostic
  functions and excludes them from normal BootInfo parsing and target
  initialization.
- The RP1 result path emits a before-load marker, flushes UART10, performs
  exactly one read_rp1_reg_u32(RP1_UART0_FR), and repeatedly emits the returned
  raw value with classification=mapped/read-value.
- The control path emits a no-RP1-MMIO marker and repeatedly emits the same
  compact result-output shape with classification=simulated/control.

## Image And Archive

RP1 tail-stable result:

- archive:
  target/talos-rpi5-rp1-uart0-fr-tail-stable-result-core.tar.gz
- archive SHA-256:
  521ea4e092321f19e414020303f4127aaf5f076095bd636bf2578cc452959541
- boot-tree identity:
  b912e810a162700602469b9039c22143cddcd587a53312bdecdc1ca4d0f04a27
- kernel SHA-256:
  63c0eddbbd90da106a5eb20095e3108a05d53001edd017d1d6a7c4b9271cc2b8
- kernel size: 45,800 bytes
- arm64 Image header: text_offset=0, header_image_size=45800, flags=12,
  magic=ARMd

No-MMIO control:

- archive:
  target/talos-rpi5-rp1-uart0-fr-tail-stable-no-mmio-control-core.tar.gz
- archive SHA-256:
  050542fe57dc5d53ce5ee87a16c139701c8beedc25f8a1c26a8b5b1b21bdae9b
- boot-tree identity:
  74e0c0a9c294927d8e7f377fd8272d587050bd3dd5fb471e120f126e65fd4764
- kernel SHA-256:
  c9c473242310875f232c19c97caa82dfc2ca843c3bf159a3380d01f6b2105581
- kernel size: 45,728 bytes
- arm64 Image header: text_offset=0, header_image_size=45728, flags=12,
  magic=ARMd

## Assembly Review

Retained assembly:

- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core/rp1-tail-stable-result-asm.txt
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core/control-tail-stable-asm.txt
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core/assembly-review.txt

Key observations:

- The RP1 selected function constructs x19 = 0x1f00030018 and has exactly one
  ldr w23, [x19], which is the contracted 32-bit volatile load from RP1 UART0
  FR.
- The RP1 terminal loop repeats TALOS: fr-tail-stable-result and prints the
  retained raw value each iteration.
- The no-MMIO selected function has no 0x1f00030018 construction, no
  read_rp1_reg_u32 reference, and zero ldr w23, [x19] RP1 loads.
- The only retained control-path ldr w10, [x9, #24] instructions are UART10
  PL011 FR polling in wait_uart10_empty_early_phase.

## Findings

- fixed: the RP1 result marker is now tail-stable after a returned load.
- fixed: the control marker uses the same repeated result-output shape without
  RP1 MMIO.
- fixed: both archives passed task-owned review and retained image/header
  metadata.
- fixed: assembly evidence proves the one-load/no-load boundary.
- deferred: hardware visibility and RP1 behavior remain queued follow-up work.
- not-an-issue: this source/static task accepts no RP1 mapped/read-value,
  bus-fault/trap, firmware-state, GPIO, interrupts, DMA/cache, storage,
  generated-root, networking, SSH, broader PCIe, Milestone 11.2, or phase
  transition claim.

## Non-Acceptance

This inspection accepts only local source/static/archive evidence. The
serialized no-MMIO Pi 5 control must pass before any RP1 tail-stable
mapped/read-value proof is attempted.
