# Console Device Model

This note defines the Phase 5 starting boundary for Talos console work. It covers the accepted source inventory, the first runtime console write core, the named default console identity, the internal write-result and input-result contracts, and the local input source inventory. Talos still does not implement descriptor tables, userspace, filesystem, networking, SSH, or a shell.

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

The first accepted receive path is QEMU-only and polling. `src/pl011.rs` now exposes a small RX-empty check plus data-register byte read for PL011, and `runtime_console::ConsoleInputBackend` gives the diagnostic a console-facing polling boundary. `ConsoleInputPollOutcome` names the internal polling result as byte available, no data, backend unavailable, or backend error. This does not enable UART interrupts, buffering, scheduler readiness, descriptor reads, or Pi 5 input.

## Runtime Console Ownership Boundary

Early logging is allowed to stay polling-only, synchronous, output-only, target-owned, and best effort. It has no input path, no interrupt-driven UART behavior, no descriptor identity, no blocking semantics, no scheduler sleep or wakeup dependency, and no shell-specific command channel.

`src/runtime_console.rs` is the first runtime console write core. It owns normal kernel console write routing through `RuntimeConsole` and the named default-console facade, while `print!` / `println!` remain the public kernel formatting surface. This first responsibility is output only: formatted kernel text is routed to the current target backend through the `runtime-console0` runtime console identity.

`DEFAULT_RUNTIME_CONSOLE` names this identity and exposes the stable internal name `runtime-console0`. `target::console::_print` now calls `write_default_console_output`, which constructs a `RuntimeConsole` for `DEFAULT_RUNTIME_CONSOLE` around the target-selected backend. The identity belongs to runtime console code; QEMU virt and Raspberry Pi 5 target modules still own which PL011 backend backs that identity on a given boot.

The write core returns an internal `ConsoleWriteOutcome`. Successful writes report `ConsoleWriteResult { device, bytes_written }` for the named console and complete formatted kernel message accepted by the runtime facade. Failed writes return `ConsoleWriteError::BackendWriteFailed { device, bytes_accepted }`, where `bytes_accepted` counts only complete string fragments the backend accepted before the failure. The current PL011 backends are still polling and normally infallible, so `target::console::_print` continues to panic on a runtime write failure rather than exposing a recoverable kernel diagnostic API.

This result contract is internal to the kernel console boundary. It is not a POSIX errno value, syscall ABI, descriptor status, blocking contract, or partial-write promise for userspace.

The input side has a matching internal polling contract. `poll_default_console_input` returns `ConsoleInputPollOutcome` for `runtime-console0`:

- `ByteAvailable { device, byte }` means the selected backend produced one input byte during this poll.
- `NoData { device }` means the backend was present but RX was empty at poll time.
- `BackendUnavailable { device }` means no accepted input backend is attached to that console identity.
- `BackendError { device }` names a future backend failure that is distinct from ordinary RX-empty polling.

QEMU PL011 currently uses only `ByteAvailable` and `NoData`, because the accepted backend can distinguish data from RX-empty but has no recoverable error channel. Polling diagnostics own their own timeout policy around repeated `NoData`; timeout is not a console-backend result. These names are deliberately internal. Later descriptor and syscall work may map them to readiness, blocking, EOF, or errno-style behavior, but this contract does not implement POSIX `read`, `poll`, nonblocking I/O, or descriptor lifetime.

The runtime console must not own POSIX process resources. Later descriptor work should attach `stdin`, `stdout`, and `stderr` handles to console objects through the descriptor layer, not by teaching the scheduler, boot code, or shell a private printing shortcut. The first `stdout` and `stderr` descriptors should point at `runtime-console0` through descriptor-owned handles; they should not call QEMU or Pi 5 target backends directly.

## Input Source Inventory

The first local input work should start as a polling diagnostic, not a TTY, blocking read, shell command channel, or UART-interrupt path.

QEMU virt has the accepted first input surface:

- source: `src/target/qemu_virt.rs`;
- UART: `qemu-virt-pl011-uart0` at `0x0900_0000`;
- current ownership: QEMU target code initializes the PL011 and backs `runtime-console0` output through `qemu_virt::console()`;
- input shape: `Pl011::poll_read_byte` checks RX-empty before reading, and `phase5-qemu-polling-tty-rx-diagnostic-20260524` passes the target-owned backend through the runtime-console/TTY boundary for a QEMU-only smoke.

This remains polling and bounded. The accepted diagnostic proves that a short injected QEMU serial line reaches kernel code through `ConsoleInputPollOutcome::ByteAvailable`, treats repeated `NoData` as a diagnostic-level timeout only after its bounded wait limit, applies the canonical-lite newline, backspace/delete, echo, control-event, and truncation policy, and reports exact line and echo bytes. It does not add descriptor allocation, task blocking, scheduler wakeups, userspace, shell commands, Pi 5 input, or UART interrupts.

Raspberry Pi 5 has two plausible local UART surfaces, with different risks:

- `bcm2712-uart10-pl011` at `0x10_7d00_1000` is the accepted normal output backend for `runtime-console0`. It preserves firmware/BL31 UART programming, uses 32-bit PL011 data-register writes, polls TX-ready, and flushes posted writes. A future UART10 input diagnostic is attractive because it matches the current output console, but it must prove that receiving bytes through the lab serial path works without disturbing the firmware-preserved baud and output contract.
- `rp1-uart0-pl011-pcie2` at `0x1f_0003_0000` and `rp1-uart0-pl011-firmware-preserved` at `0x1c_0003_0000` remain target metadata plus pin-control setup in `rpi5::init_stub()`. Historical first-light work used RP1 UART0, but the accepted runtime console output path does not depend on it. Choosing RP1 UART0 first would mix input bring-up with RP1/PCIe address and pinmux ownership, so it should wait until a task explicitly owns that hardware risk.

The first Pi 5 input claim must be serialized with `hardwareTestLock` and include:

- candidate boot archive name, archive SHA256, kernel image SHA256, and staged TFTP tree or upload identifier;
- pre-run TFTP cursor and post-run TFTP delta proving the candidate image was fetched;
- lab-controller serial capture showing the kernel reached the input diagnostic;
- lab-controller serial write or equivalent recorded injected bytes;
- serial output proving the kernel observed the exact injected byte sequence or an explicit bounded timeout classification;
- restore or post-run state note when the task stages a diagnostic archive over a known-good boot tree.

Failed input boots or timeouts are evidence, not incidents. They should be classified by whether the candidate image was fetched, whether ordinary output survived, whether injected bytes were sent, and whether the diagnostic reported RX data, RX-empty, or timeout.

The first Pi 5 UART10 polling RX proof is accepted as hardware evidence for the current local input path. The diagnostic is gated by `TALOS_RPI5_UART10_POLLING_RX_DIAGNOSTIC`, uses the firmware-preserved UART10 backend for `runtime-console0`, and feeds that backend through the same `runtime_console::poll_default_console_input` and `tty::run_polling_rx_diagnostic_with_limit` boundary as the QEMU proof. The accepted run published `target/talos-rpi5-uart10-rx-boot.tar.gz` with archive SHA256 `bab86eacea7868b4fd92423370c4991b11cc6f270c60b7b38ae5960336f54209`; TFTP served the 90,344-byte candidate `kernel_2712.img`; the lab wrote the 15-byte sequence `61 62 58 08 63 59 7f 64 03 65 66 67 68 69 0d`; and serial output reached `rpi5-uart10-rx-diagnostic: PASS` with echo bytes `61 62 58 08 20 08 63 59 08 20 08 64 65 66 67 68 0d 0a` and `control-events=ctrl-c`.

This proves polling UART10 receive for the bounded local diagnostic path. It does not add UART interrupts, scheduler blocking, descriptor readiness, POSIX errno mapping, shell command input, filesystem behavior, networking, SSH, RP1 UART0 ownership, or a stable userspace ABI.

## Descriptor And TTY Compatibility Constraints

Descriptor writes should eventually call the same console write operation used by kernel diagnostics, translating `ConsoleWriteOutcome` through a descriptor layer once descriptor ownership, blocking behavior, and errno mapping exist. Descriptor reads should similarly translate `ConsoleInputPollOutcome` through descriptor-owned readiness and blocking policy only after those layers exist. `stdin` requires a real input source and should not be faked by output-only console work. Line editing, canonical mode, echo, signals, PTYs, and terminal window state belong to the TTY layer. Blocking writes or reads can only sleep tasks after scheduler sleep/wakeup queues exist. Internal errors should remain structured so a later syscall boundary can map them to errno-style values without exposing current kernel-console names as ABI. QEMU and Pi 5 target differences should remain behind target/runtime console backend boundaries.

The first `stdin` descriptor should attach to the input side of the selected console object only after an input source exists. Until scheduler sleep/wakeup and descriptor lifetime exist, input diagnostics should report readiness or bounded polling results directly to kernel diagnostics instead of pretending to offer POSIX `read`.

## Diagnostic Surface Policy

Phase 4 QEMU and Pi 5 timer/scheduler boot images remain validation surfaces, not console interfaces. A future local diagnostic command channel may replace some special boot-image diagnostics, but that channel must be planned as a console/TTY feature rather than hidden in timer, scheduler, or target code.

## Phase 5.1 Checkpoint

The accepted Phase 5.1 model is output-capable and input-planned. runtime-console0 is the default runtime console identity for normal kernel diagnostics. The runtime console owns output routing and its internal write-result contract, while target modules own the physical PL011 backend for QEMU and Pi 5.

Milestone 5.2 may start with a TTY/stdio shape document only. That design task may define raw/canonical behavior, newline/backspace/echo/control-character policy, and descriptor-facing stdin/stdout/stderr shape, but it must not implement UART RX, line discipline, descriptor tables, syscalls, userspace, shell behavior, hardware tests, or blocking I/O.

The accepted Milestone 5.2 shape is documented in [TTY and Stdio Shape](tty-stdio.md). It keeps TTY policy above the runtime console backend, treats stdin, stdout, and stderr as future descriptor-owned streams, and is now backed by the first QEMU-only polling PL011 RX diagnostic with bounded echo and line capture.

## Next Implementation Boundary

The bounded implementation task `phase5-runtime-console-write-core-20260524` added the output-only runtime console write core. The follow-up `phase5-console-write-result-contract-20260524` made its success/error boundary explicit with complete-write byte accounting for future descriptor compatibility. The `phase5-console-device-identity-boundary-20260524` slice named the default output-side console identity as `runtime-console0` and routed normal kernel diagnostics through `write_default_console_output`. The `phase5-console-input-source-inventory-20260524` slice inventoried QEMU and Pi 5 local input options, and `phase5-qemu-polling-tty-rx-diagnostic-20260524` accepted the QEMU-only polling PL011 RX diagnostic as the first local input proof.

The core is backed by the existing polling PL011 paths and preserves the accepted early serial output contract. The QEMU input diagnostic adds only bounded polling RX through the structured internal input result contract and the shared TTY line-discipline core. It does not add UART interrupts, descriptor tables, userspace, filesystems, networking, SSH, a shell, or sleep/blocking behavior.
