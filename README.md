# Talos

Talos is the working codename for a Rust bare-metal operating system project.

The goal is to build a small, understandable Rust-based kernel with generic
kernel architecture and Raspberry Pi 5 as the first supported physical board.
Talos should boot on a Pi 5, use the available CPU cores with preemptive
scheduling, expose a usable shell over SSH, and grow toward Unix/POSIX-compatible
abstractions.

This is separate from DaedalusOS, which remains the Raspberry Pi 4 kernel.

## Documentation

Project docs live in docs/src/ and are built with mdBook:

```bash
mdbook build
```

The first planning documents are:

- docs/src/vision.md
- docs/src/project/operating-model.md
- docs/src/project/roadmap-process.md
- docs/src/project/agent-task-template.md

## Current Status

Phase 1 has started with a minimal Rust no_std AArch64 kernel skeleton.
The first runnable target is QEMU virt, which is used for fast toolchain and
generic AArch64 validation. It is not a Raspberry Pi 5 emulator.

The repository should stay publishable as a standalone Git project. Generated
build output and mdBook output are ignored; source, target definitions,
scripts, task records, and docs are kept in git.

## Build and Run

Use the pinned nightly toolchain from rust-toolchain.toml. The Cargo config
builds core and compiler_builtins for the custom target.

~~~bash
cargo -Zjson-target-spec build
./scripts/qemu-smoke.sh
cargo -Zjson-target-spec test
~~~

The smoke script boots QEMU virt with a Cortex-A76 CPU and expects this serial
line:

~~~text
talos: qemu smoke PASS
~~~

Useful explicit forms:

~~~bash
cargo +nightly -Zjson-target-spec -Zbuild-std=core,compiler_builtins -Zbuild-std-features=compiler-builtins-mem build --target targets/aarch64-talos-virt.json
cargo -Zjson-target-spec run
~~~

The linker map is emitted at target/talos-aarch64-virt.map.

## Targets

- talos-aarch64-virt: QEMU virt target with PL011 serial at 0x0900_0000.
- talos-rpi5-bcm2712: reserved target definition for the physical Pi 5 path.

The boot entry preserves the AArch64 x0 value as an opaque physical DTB
pointer in BootInfo. QEMU may provide a generated DTB pointer; Pi 5 firmware
will later provide the real device-tree address through the arm64 boot ABI.
