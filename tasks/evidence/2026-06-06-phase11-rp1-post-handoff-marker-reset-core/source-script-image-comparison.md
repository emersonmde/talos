# Source/Script/Image Comparison

Task: phase11-rp1-post-handoff-marker-reset-core-20260606

## Compared Inputs

- Accepted handoff reset closeout:
  tasks/2026-06-06-phase11-rp1-entry-control-handoff-closeout.md.
- New source routing: build.rs, src/main.rs, src/target/rpi5.rs, and
  src/arch/aarch64/boot.S.
- Helper scripts:
  scripts/rpi5-rp1-post-handoff-marker-reset-image.sh and
  scripts/rpi5-rp1-post-handoff-marker-reset-boot-tree.sh.
- Candidate archive:
  target/talos-rpi5-post-handoff-marker-reset-core.tar.gz.

## Dispositions

- fixed: rpi5_rp1_post_handoff_marker_reset is a recognized build scenario and
  has task-owned image/boot-tree scripts.
- fixed: rust_entry writes the accepted TALOS: rust_entry early-phase line and
  then calls run_rp1_post_handoff_marker_reset_diagnostic before the normal
  BootInfo and target initialization path.
- fixed: run_rp1_post_handoff_marker_reset_diagnostic emits the unique marker
  rpi5-rp1-post-handoff-marker-reset: post-handoff-marker and classification
  marker-before-reset before PSCI SYSTEM_RESET.
- fixed: static disassembly proves marker writes and wait_uart10_empty_early_phase
  precede the mov w0, #0x0009; movk w0, #0x8400; smc #0 reset sequence.
- not-an-issue: the prior rpi5_rp1_handoff_reset candidate remains the accepted
  reset-side-effect control, but it is not sufficient to prove serial marker
  observability.
- not-an-issue: the new candidate performs no RP1 UART0 FR read and retains no
  rpi5-rp1-uart0-fr-read strings.
- deferred: Pi 5 publication, hardware lock acquisition, serial/TFTP capture,
  reset-loop classification, and restore are explicitly deferred to the queued
  Pi 5 discriminator.

## Candidate Identity

- archive SHA-256:
  73a74db1d08d89a3aa371d5329bc6158553bef172a82f0b479598bc29f15acaa
- root/prefixed kernel SHA-256:
  42367beda5de1d0564417e6267a59bd5ae5b770798fa4a3cbb3c0ce101554350
- kernel size: 51,736
- arm64 Image header: text_offset=0, header_image_size=51736, flags=12,
  magic=ARMd

Evidence level: source/static image/archive inspection only. No hardware lock,
archive publication, TFTP observation, serial hardware run, power cycle, or
restore was performed.
