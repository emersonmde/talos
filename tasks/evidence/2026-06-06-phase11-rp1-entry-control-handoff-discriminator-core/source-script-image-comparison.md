# Source/Script/Image Comparison

Task: phase11-rp1-entry-control-handoff-discriminator-core-20260606

## Compared Inputs

- Previous entry-control source candidate:
  target/talos-rpi5-rp1-entry-control-source-core.tar.gz.
- Candidate-rerun blocker evidence:
  tasks/evidence/2026-06-05-phase11-rp1-entry-control-candidate-rerun/.
- Accepted known-good runtime-readiness marker-boundary closeout:
  tasks/2026-06-06-phase11-known-good-runtime-marker-boundary-closeout.md.
- New handoff-reset candidate:
  target/talos-rpi5-rp1-handoff-reset-discriminator-core.tar.gz.
- Source routing: build.rs, src/main.rs, src/target/rpi5.rs, and
  src/arch/aarch64/boot.S.
- Helper scripts: scripts/rpi5-rp1-handoff-reset-image.sh,
  scripts/rpi5-rp1-handoff-reset-boot-tree.sh,
  scripts/rpi5-rp1-entry-control-image.sh, and
  scripts/rpi5-rp1-entry-control-boot-tree.sh.

## Dispositions

- fixed: rpi5_rp1_handoff_reset branches from rust_entry before
  BootInfo::from_aarch64_x0, target::init, boot reports, memory planning,
  allocator setup, or the RP1 UART0 FR read path.
- fixed: the new candidate's acceptance signal is PSCI SYSTEM_RESET and the
  resulting repeated TFTP boot/fetch sequence, not only serial marker
  visibility.
- fixed: task-owned helper scripts produce
  kernel_2712-rp1-handoff-reset.img, root kernel_2712.img/kernel8.img, and a
  byte-identical da591740/ mirror from the accepted Pi 5 boot-tree shape.
- fixed: disassembly proves _start calls rust_entry, rust_entry calls the
  handoff-reset diagnostic immediately, and that diagnostic issues smc #0 with
  PSCI function id 0x84000009.
- not-an-issue: the previous entry-control serial-marker candidate remains as
  retained historical evidence, but the next hardware discriminator should use
  the reset-side-effect candidate because the blocker is serial-marker absence
  after confirmed candidate fetch.
- not-an-issue: the new image retains the accepted arm64 Image contract:
  _start and __kernel_start at 0x200000, text_offset=0,
  header_image_size=45248, flags=12, and ARMd magic.
- deferred: hardware publication, power cycle, fresh TFTP/serial cursors,
  repeated TFTP reset-side-effect classification, restore, and hardware lock
  handling are explicitly deferred to the queued Pi 5 discriminator.

## Candidate Identity

- archive: target/talos-rpi5-rp1-handoff-reset-discriminator-core.tar.gz
- archive SHA-256:
  ee251a145b88df55fd162b0150a82d62a9671906f401948524d27d45929516c6
- root/prefixed kernel SHA-256:
  38170a7fe229b37bfb358479f09d45a14a342af86b16c51d36b3c33023255594
- kernel size: 45,248
- arm64 Image header: text_offset=0, header_image_size=45248, flags=12,
  magic=ARMd

Evidence level: source/static image/archive inspection only. No hardware lock,
archive publication, TFTP observation, serial hardware run, power cycle, or
restore was performed.
