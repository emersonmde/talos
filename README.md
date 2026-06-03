# Talos

Talos is a small Rust bare-metal operating system project for AArch64. The
first real board target is Raspberry Pi 5. QEMU `virt` is used for fast local
validation of architecture-independent work.

The project has two goals:

- Build an understandable kernel that can grow toward Unix-like abstractions:
  processes, file descriptors, pipes, filesystems, sockets, a shell, and basic
  command-line programs.
- Test whether a real hardware lab plus asynchronous agent work can move a
  kernel project forward without losing engineering discipline.

This is not a production OS, a Linux replacement, or a stable platform. It is an
early kernel and lab workflow with a strong bias toward small steps, evidence,
and documentation that stays close to the code.

## Status

Talos currently has:

- A `no_std` Rust AArch64 kernel skeleton.
- Custom target definitions for QEMU `virt` and Raspberry Pi 5.
- QEMU boot and smoke-test support.
- Raspberry Pi 5 image and boot-tree staging scripts.
- Early PL011/RP1 UART output paths.
- AArch64 exception-vector setup and early panic reporting.
- Bounded firmware device-tree parsing for boot arguments, memory banks, and
  reserved ranges.
- Early low-memory bootstrap allocation and initial EL2 translation-table work.

Talos does not yet have userspace, scheduling, filesystems, networking, storage
drivers, or a stable syscall interface.

## Development Model

Talos is developed with a mix of human review and OpenClaw agents. The agents
are useful for long-running implementation and hardware loops, but their output
is treated as evidence, not authority.

The working pattern is:

1. Define a narrow task with acceptance criteria.
2. Implement a small change.
3. Run local gates such as format, tests, QEMU smoke, and docs.
4. For Pi 5 work, stage a boot archive and review it before publishing.
5. Use Talos Lab to publish, power-cycle, capture serial/TFTP evidence, and
   roll back when needed.
6. Record the result in task notes, architecture docs, or ADRs before calling
   the work done.

The supervisor role keeps roadmap, task records, and acceptance criteria
aligned. Worker agents take bounded implementation or diagnostic tasks. The
project lead still owns integration and final direction.

## Talos Lab

Talos Lab is a private lab-control service used for physical Raspberry Pi 5
testing. It is intentionally separate from this kernel repository. Its job is to
provide a narrow API for:

- Publishing a reviewed TFTP boot archive.
- Power-cycling the board.
- Reading serial output.
- Inspecting TFTP request logs.
- Rolling back to a known-good boot tree.

The important idea is the boundary: agents do not need broad host access,
controller credentials, or direct hardware ownership to run a hardware test.
Deployment-specific network and switch details belong in private lab
configuration, not in the README.

See `docs/src/project/lab-controller.md` for the internal contract.

## Build And Test

Talos uses the pinned nightly toolchain in `rust-toolchain.toml`. The Cargo
configuration builds `core`, `alloc`, and `compiler_builtins` for custom
targets.

Run the normal local checks:

```bash
cargo fmt --check
cargo -Zjson-target-spec test
./scripts/qemu-smoke.sh
mdbook build
git diff --check
```

QEMU scripts and the Cargo test runner use `qemu-system-aarch64` from `PATH` by
default. Set `QEMU_SYSTEM_AARCH64=/path/to/qemu-system-aarch64` when the
workspace QEMU build is not on `PATH`. Nographic QEMU smoke scripts share
`scripts/qemu-nographic-smoke-lib.sh` for build/image/run setup; keep
script-specific assertions in the individual smoke script.

The QEMU smoke test expects this line:

```text
talos: qemu smoke PASS
```

Build the default QEMU target:

```bash
cargo -Zjson-target-spec build
```

Build an explicit target:

```bash
cargo +nightly-2026-05-20 \
  -Zjson-target-spec \
  -Zbuild-std=core,compiler_builtins,alloc \
  -Zbuild-std-features=compiler-builtins-mem \
  build --target targets/aarch64-talos-virt.json
```

## Raspberry Pi 5 Artifacts

Build and review a Pi 5 boot archive:

```bash
./scripts/rpi5-image.sh
./scripts/rpi5-boot-tree.sh /path/to/pi-firmware-boot-source target/rpi5-boot-tree
tar -C target/rpi5-boot-tree -czf target/talos-rpi5-boot.tar.gz .
./scripts/rpi5-archive-review.sh target/talos-rpi5-boot.tar.gz
```

These scripts prepare artifacts. They do not prove hardware success. Hardware
claims need serial output, TFTP evidence, and a task note that describes what
was observed.

## Documentation

The project book is built with mdBook from `docs/src/`:

```bash
mdbook build
```

Useful starting points:

- `docs/src/vision.md`
- `docs/src/roadmap.md`
- `docs/src/project/operating-model.md`
- `docs/src/project/testing-strategy.md`
- `docs/src/project/lab-controller.md`
- `docs/src/architecture/README.md`

## License

Talos is licensed under the MIT License. See `LICENSE`.
