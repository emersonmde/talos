# Phase 5 Console Device Identity Boundary

Status: accepted as the Phase 5.1 default runtime console identity slice.

## Scope

This task names the output-side runtime console identity without adding descriptors, input, TTY state, or userspace ABI. Normal kernel formatted output still enters through `print!` and `println!`, but `target::console::_print` now routes through `runtime_console::write_default_console_output`.

The default output console is `DEFAULT_RUNTIME_CONSOLE` with the internal name `runtime-console0`. `RuntimeConsole` stores that identity beside the backend and continues to own complete-write byte accounting and backend failure mapping.

## Boundary

The identity belongs to the runtime console layer. Target code still owns hardware backend selection:

- QEMU virt backs `runtime-console0` with its existing PL011 UART at `0x0900_0000`.
- Raspberry Pi 5 backs `runtime-console0` with the firmware-preserved UART10 PL011 path at `0x10_7d00_1000`.
- Pi 5 early helper output remains separate for panic, OOM, exception, fault, DTB, and memory diagnostics that must not depend on broad formatting or future console objects.

This task does not add descriptor tables, descriptor lifetime, process ownership, stdin, UART receive, TTY line discipline, syscalls, userspace, filesystems, networking, SSH, shell behavior, UART interrupts, or scheduler sleep/wakeup behavior. Normal Pi 5 serial output is intended to be preserved; no hardware run was required.

Later `stdout` and `stderr` descriptors should attach to `runtime-console0` through descriptor-owned handles and translate `ConsoleWriteOutcome` at the descriptor/syscall boundary. They should not call QEMU or Pi 5 target backends directly. `stdin` still requires a real input source and remains deferred.

## Evidence

- Static inspection: `DEFAULT_RUNTIME_CONSOLE` exposes the internal default console name `runtime-console0`, and `RuntimeConsole::new` binds backends to that identity.
- Static inspection: `target::console::_print` routes normal kernel diagnostics through `runtime_console::write_default_console_output`; target modules still own backend construction.
- Unit tests: runtime console tests cover default console identity naming and the named default-console output path.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed after the identity boundary change.
- Image/archive inspection: `scripts/rpi5-image.sh` and `scripts/rpi5-format-guard-check.sh` passed after the identity boundary change.
- Documentation gate: `mdbook` was unavailable in the worker container, so `mdbook build` was not run.
