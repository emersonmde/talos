# Phase 5 Console Write Result Contract

Status: accepted as the Phase 5.1 runtime console write-result boundary.

## Scope

This task gives the output-only runtime console path an explicit internal result contract while preserving the public `print!` / `println!` formatting surface.

`src/runtime_console.rs` now returns `ConsoleWriteOutcome` from `write_default_console_output` and `RuntimeConsole::write_kernel_args`:

- `Ok(ConsoleWriteResult)` means the runtime console facade accepted the complete formatted kernel message.
- `ConsoleWriteResult { device, bytes_written }` reports the console identity and number of message bytes accepted by the facade.
- `Err(ConsoleWriteError::BackendWriteFailed { device, bytes_accepted })` means a backend write failed for that console after `bytes_accepted` complete string fragments had been accepted.

The contract is internal to kernel console code. It is not a POSIX errno value, syscall ABI, descriptor status, blocking contract, or userspace partial-write guarantee.

## Boundary

The current QEMU and Raspberry Pi 5 PL011 backends remain target-owned polling output paths. They normally return success, so `target::console::_print` continues to panic on failure through `expect("serial console write failed")`. That keeps kernel diagnostics fail-fast while still leaving a structured status for later descriptor writes to translate.

The task does not add descriptor tables, file descriptors, syscalls, userspace, TTY line discipline, input, UART interrupts, blocking or sleeping writes, filesystems, networking, SSH, shell behavior, scheduler policy changes, or hardware boot behavior changes. Normal Pi 5 serial output is intended to be preserved; no hardware run was required.

## Evidence

- Static inspection: `RuntimeConsole` owns byte accounting and maps backend `fmt::Error` into `ConsoleWriteError::BackendWriteFailed`; `target::console::_print` now routes through `runtime_console::write_default_console_output`.
- Unit tests: runtime console tests cover complete static writes, formatted writes, byte counts, and backend failure propagation.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed after the contract change.
- Image/archive inspection: `scripts/rpi5-image.sh` and `scripts/rpi5-format-guard-check.sh` passed after the contract change.
- Documentation gate: `mdbook` was unavailable in the worker container, so `mdbook build` was not run.
