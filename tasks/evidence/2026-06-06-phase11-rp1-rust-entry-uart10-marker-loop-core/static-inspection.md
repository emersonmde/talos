# Phase 11 RP1 Rust-Entry UART10 Marker Loop Static Inspection

Task id: phase11-rp1-rust-entry-uart10-marker-loop-core-20260606

## Inputs Inspected

- build.rs
- src/main.rs
- src/target/rpi5.rs
- scripts/rpi5-rust-entry-uart10-marker-loop-image.sh
- scripts/rpi5-rust-entry-uart10-marker-loop-boot-tree.sh
- scripts/rpi5-rust-entry-uart10-marker-loop-archive.sh
- scripts/rpi5-rust-entry-uart10-marker-loop-review.sh
- target/talos-rpi5-rust-entry-uart10-marker-loop-core.tar.gz
- target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-rust-entry-uart10-marker-loop.img
- target/aarch64-talos-rpi5-bcm2712/debug/talos

## Archive And Image Identity

The candidate archive is
target/talos-rpi5-rust-entry-uart10-marker-loop-core.tar.gz with SHA-256
ab6de452670427cee2d411cbcd2a92602331e9d03a9d68dae20b75d649d1565b.

The selected kernel image is
target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712-rust-entry-uart10-marker-loop.img
with SHA-256
6335cc2f229c38258d88000fe968248ca2e47d61e47f874bf246862e0d2b248a.
The image is 45,328 bytes. The arm64 Image header reports text_offset=0,
header_image_size=45328, and flags=12. Generic archive review accepted the
config, mirrored da591740 boot files, kernel_2712/kernel8 identity, and ARMd
image contract.

## Path Provenance

Disassembly retained in disassembly-start.txt shows _start preserving x0,
clearing BSS, setting sp, restoring x0, and branching to rust_entry.

Disassembly retained in disassembly-rust-entry.txt shows rust_entry branches
directly to run_rust_entry_uart10_marker_loop for this scenario. It does not
call BootInfo::from_aarch64_x0, target::init, exception setup, kernel_main, or
any RP1 diagnostic path.

Disassembly retained in disassembly-marker-loop.txt shows
run_rust_entry_uart10_marker_loop loading the marker string address and length,
calling write_early_static, waiting for UART10 empty through the existing
0x107d001000 UART10 flag-register path, and branching back to repeat. There is
no PSCI SYSTEM_RESET call in the marker-loop path.

## String And Symbol Checks

The marker string TALOS: reu10-loop is present in the kernel image.

The archive review checked that these RP1 UART0 FR-read strings are absent from
the candidate kernel image:

- rpi5-rp1-uart0-fr-read
- rp1-uart0-fr-read
- phase11-rp1-pcie-map-contract-v1

The retained symbol scan includes _start, rust_entry, and
run_rust_entry_uart10_marker_loop. It does not include run_rp1_uart0_fr_read or
read_rp1 symbols for this selected image.

## Findings And Disposition

- fixed: selected scenario is routed at rust_entry before BootInfo parsing or
  target initialization.
- fixed: repeated marker output uses the existing UART10 early-phase helper
  path only.
- fixed: the marker-loop path does not perform RP1 UART0 FR reads or carry
  RP1 FR-read report strings.
- fixed: the candidate archive identity, kernel identity, size, and Image
  header fields are retained.
- deferred: visible marker observability requires the serialized Pi 5
  discriminator.
- not-an-issue: the UART10 helper necessarily reads UART10 FR at
  0x107d001018 to wait for transmit empty; this is BCM2712 UART10, not RP1
  UART0 FR at 0x1f00030018.

## Classification

Classification: ready-for-rust-entry-uart10-marker-loop-pi5-discriminator.

This static core accepts no RP1 mapped/unmapped, GPIO, interrupt, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe, Milestone 11.2, or
phase-transition behavior.
