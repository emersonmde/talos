# Console Device Model

This note defines the Phase 5 starting boundary for Talos console work. It covers the accepted source inventory, the first runtime console write core, the named default console identity, and the internal write-result contract. Talos still does not implement descriptor tables, TTY line discipline, input, userspace, filesystem, networking, SSH, or a shell.

## Current Early Logging Surface

The current kernel printing path is target-routed and synchronous:

- `print!` and `println!` build `core::fmt::Arguments`.
- `target::console::_print` passes the active target backend to `runtime_console::write_default_console_output`.
- `runtime_console::RuntimeConsole` owns normal kernel write routing for the named `runtime-console0` default console.
- The target backend implements `core::fmt::Write::write_str`.
- `Pl011` writes bytes through polling MMIO and translates line feeds to CRLF.

QEMU virt owns the simple PL011 path:

- source: `src/target/qemu_virt.rs`;
- UART: `qemu-virt-pl011-uart0` at `0x0900_0000`;
- initialization: `qemu_virt::init()` calls `console().init_early()`;
- backend: `Pl011::new(PL011_BASE)`.

Raspberry Pi 5 owns a firmware-preserved early UART path:

- sources: `src/target/rpi5.rs`, `src/boot/rpi5.rs`, and `src/boot/rpi5_reports.rs`;
- primary lab-visible UART: BCM2712 UART10 at `0x10_7d00_1000`;
- backend: `rpi5::firmware_console()` returns `Pl011::new_with_posted_write_flush(UART10_BASE)`;
- policy: preserve firmware/BL31 UART10 baud programming, poll TX-ready, use 32-bit PL011 data-register writes, and flush posted writes;
- RP1 UART0 pin setup exists in `rpi5::init_stub()`, but the accepted normal console path does not depend on RP1 UART0 for visible output.

The early helper path remains intentionally separate: `target::console::write_static`, `write_hex_usize`, `write_hex_u64`, and `write_dec_usize`. On Pi 5 those helpers route to the proven UART10 word-write path and are used for panic/OOM, exception/fault reports, DTB and memory reports, and other bring-up diagnostics that must not rely on broad formatting or allocation.

## Runtime Console Ownership Boundary

Early logging is allowed to stay polling-only, synchronous, output-only, target-owned, and best effort. It has no input path, no interrupt-driven UART behavior, no descriptor identity, no blocking semantics, no scheduler sleep or wakeup dependency, and no shell-specific command channel.

`src/runtime_console.rs` is the first runtime console write core. It owns normal kernel console write routing through `RuntimeConsole` and the named default-console facade, while `print!` / `println!` remain the public kernel formatting surface. This first responsibility is output only: formatted kernel text is routed to the current target backend through the `runtime-console0` runtime console identity.

`DEFAULT_RUNTIME_CONSOLE` names this identity and exposes the stable internal name `runtime-console0`. `target::console::_print` now calls `write_default_console_output`, which constructs a `RuntimeConsole` for `DEFAULT_RUNTIME_CONSOLE` around the target-selected backend. The identity belongs to runtime console code; QEMU virt and Raspberry Pi 5 target modules still own which PL011 backend backs that identity on a given boot.

The write core returns an internal `ConsoleWriteOutcome`. Successful writes report `ConsoleWriteResult { device, bytes_written }` for the named console and complete formatted kernel message accepted by the runtime facade. Failed writes return `ConsoleWriteError::BackendWriteFailed { device, bytes_accepted }`, where `bytes_accepted` counts only complete string fragments the backend accepted before the failure. The current PL011 backends are still polling and normally infallible, so `target::console::_print` continues to panic on a runtime write failure rather than exposing a recoverable kernel diagnostic API.

This result contract is internal to the kernel console boundary. It is not a POSIX errno value, syscall ABI, descriptor status, blocking contract, or partial-write promise for userspace.

The runtime console must not own POSIX process resources. Later descriptor work should attach `stdin`, `stdout`, and `stderr` handles to console objects through the descriptor layer, not by teaching the scheduler, boot code, or shell a private printing shortcut. The first `stdout` and `stderr` descriptors should point at `runtime-console0` through descriptor-owned handles; they should not call QEMU or Pi 5 target backends directly.

## Descriptor And TTY Compatibility Constraints

Descriptor writes should eventually call the same console write operation used by kernel diagnostics, translating `ConsoleWriteOutcome` through a descriptor layer once descriptor ownership, blocking behavior, and errno mapping exist. `stdin` requires a real input source and should not be faked by output-only console work. Line editing, canonical mode, echo, signals, PTYs, and terminal window state belong to a later TTY layer. Blocking writes or reads can only sleep tasks after scheduler sleep/wakeup queues exist. Internal errors should remain structured so a later syscall boundary can map them to errno-style values without exposing current kernel-console names as ABI. QEMU and Pi 5 target differences should remain behind target/runtime console backend boundaries.

## Diagnostic Surface Policy

Phase 4 QEMU and Pi 5 timer/scheduler boot images remain validation surfaces, not console interfaces. A future local diagnostic command channel may replace some special boot-image diagnostics, but that channel must be planned as a console/TTY feature rather than hidden in timer, scheduler, or target code.

## Next Implementation Boundary

The bounded implementation task `phase5-runtime-console-write-core-20260524` added the output-only runtime console write core. The follow-up `phase5-console-write-result-contract-20260524` made its success/error boundary explicit with complete-write byte accounting for future descriptor compatibility. The `phase5-console-device-identity-boundary-20260524` slice named the default output-side console identity as `runtime-console0` and routed normal kernel diagnostics through `write_default_console_output`.

The core is backed by the existing polling PL011 paths and preserves the accepted early serial output contract. It does not add UART interrupts, input, TTY line discipline, descriptor tables, userspace, filesystems, networking, SSH, a shell, or sleep/blocking behavior.
