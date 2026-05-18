# QEMU Skeleton

Talos starts with a fast AArch64 virt target before physical Raspberry Pi 5 work.
The target exists to validate the Rust toolchain, linker layout, generic AArch64 boot
flow, panic output, and pure no_std tests. It is not a Raspberry Pi 5 emulator.

## Target Split

- talos-aarch64-virt uses QEMU virt, Cortex-A76, and the PL011 UART at
  0x0900_0000.
- talos-rpi5-bcm2712 has a target JSON and Rust target stub, but no board MMIO
  assumptions yet.

The AArch64 entry path preserves the firmware/QEMU x0 value in BootInfo::dtb_pa.
For QEMU this may be zero or a generated DTB pointer. For the Pi 5 target it will
preserve the firmware-provided physical DTB address from the arm64 boot ABI.

## Layout

The QEMU image is linked at 0x4020_0000, above QEMU virt's generated
low-memory DTB reservation, and starts with the arm64 Image header QEMU expects
for -kernel. The linker script defines:

- __kernel_start / __kernel_end
- .text.boot first in the image
- FP/SIMD enabled before entering Rust, because debug core formatting and
  precondition paths can emit SIMD instructions
- page-aligned text, rodata, and data sections
- __bss_start / __bss_end, cleared by the assembly entry stub
- __heap_start / __heap_end, reserving 1 MiB for a later allocator
- __stack_bottom / __stack_top, reserving 256 KiB for the boot stack

A linker map is emitted to target/talos-aarch64-virt.map for early boot
inspection.

## Commands

~~~bash
cargo +nightly -Zjson-target-spec -Zbuild-std=core,compiler_builtins -Zbuild-std-features=compiler-builtins-mem build
./scripts/qemu-smoke.sh
cargo -Zjson-target-spec test
mdbook build
~~~

The repository .cargo/config.toml already supplies the custom target and
build-std settings. Current nightly Cargo still requires -Zjson-target-spec
when invoking the checked-in JSON target.
