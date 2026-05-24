# Phase 5 Runtime Console Write Core

Status: accepted as the first Phase 5.1 runtime console implementation slice.

## Scope

This task adds `src/runtime_console.rs`, an output-only runtime console write core. Normal kernel formatted output still enters through `print!` and `println!`, but `target::console::_print` now hands the active target backend to `runtime_console::write_default_console_output`.

The target modules still own hardware differences:

- QEMU virt continues to use `qemu_virt::console()` and the PL011 at `0x0900_0000`.
- Raspberry Pi 5 continues to use `rpi5::firmware_console()` and the firmware-preserved UART10 path at `0x10_7d00_1000`.
- Pi 5 early helper output remains on `target::console::write_static`, `write_hex_usize`, `write_hex_u64`, and `write_dec_usize` so panic, OOM, exception, and boot reports can keep their narrow early-output behavior.

## Boundary

The runtime console write core owns only normal kernel output routing. It does not introduce UART interrupts, input, TTY line discipline, descriptor tables, userspace, syscalls, filesystems, networking, SSH, shell behavior, scheduler policy changes, or sleeping/blocking I/O.

Normal Pi 5 serial output is intended to be preserved, not changed. The implementation keeps Pi 5 backend selection behind `target::rpi5::firmware_console()` and continues to use the existing polling PL011 behavior with posted-write flushing.

## Evidence

- Static inspection: `target::console::_print` routes through `runtime_console::write_default_console_output`, while target-owned backend selection remains in `target::console::runtime_backend`.
- Unit tests: `runtime_console` tests cover static and formatted kernel output routing to a captured backend.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed after the write-core refactor.
- Image/archive inspection: `scripts/rpi5-image.sh` passed for the normal Pi 5 image after the write-core refactor.
- Formatting/build gates: `cargo fmt --all -- --check`, `cargo -Zjson-target-spec test`, `scripts/rpi5-format-guard-check.sh`, and `git diff --check` passed.
- Documentation gate: `mdbook` was unavailable in the worker container, so `mdbook build` was not run.
