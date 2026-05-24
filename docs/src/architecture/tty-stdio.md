# TTY and Stdio Shape

This note defines the first Talos TTY and stdio contract. It is a design boundary, not an implementation. Talos still does not implement UART RX, TTY line discipline, descriptor tables, syscalls, userspace, scheduler blocking I/O, filesystems, networking, SSH, or a shell.

The TTY layer must sit above the runtime console backend. Target modules own physical UART selection, runtime console owns the named runtime-console0 console object, and later descriptor code should attach stdin, stdout, and stderr to console-backed stream objects without calling target UART code directly.

## First Local TTY

The first local TTY should be a single serial diagnostic TTY backed by runtime-console0 output and by an explicitly accepted input source. It should start on QEMU virt because the accepted input-source inventory names QEMU PL011 polling RX as the lowest-risk first input proof.

The initial implementation should keep these identities distinct:

- console device: runtime-console0, the target-backed console object;
- TTY: the line-discipline object layered over the console input/output pair;
- stdio streams: stdin, stdout, and stderr, future descriptor-facing handles attached by descriptor code;
- diagnostic command channel: a later kernel-owned client of the TTY, not the TTY itself.

The TTY must not own process resources, descriptor allocation, syscall ABI, scheduler sleep queues, or shell command dispatch.

## Raw And Canonical Modes

Raw mode means bytes are delivered to the TTY client without line assembly. Raw input does not translate carriage return, line feed, backspace, delete, escape, tab, or control bytes. Echo is off by default in raw mode. A diagnostic may print byte values for visibility, but that is diagnostic output rather than raw-mode semantics.

Canonical mode means the TTY collects an editable line until a line terminator or a bounded buffer limit. The first canonical implementation should be deliberately small:

- \r, \n, and a collapsed \r\n pair terminate the current line.
- Backspace 0x08 and delete 0x7f remove the previous buffered byte when the line is non-empty.
- Printable ASCII bytes and tab are appended while capacity remains.
- Other C0 control bytes except Escape are recorded as named control events, not delivered as signals.
- On buffer exhaustion, the diagnostic reports truncation and terminates or drains the line according to the task-specific test contract.

Canonical editing should remain byte-oriented at first. UTF-8 validation, locale, grapheme deletion, history, cursor motion, and terminal escape parsing are deferred.

`src/tty.rs` now expresses this first behavior as the target-independent `TtyLineDiscipline` core. `TtyMode::Raw` records pass-through bytes without canonical translation or echo, while `TtyMode::CanonicalLite` performs bounded line assembly, deterministic echo decisions, named deferred control events, Escape as input data, and explicit buffer-limit reporting. Polling diagnostics own timeout policy around the core; the line discipline itself reports parser outcomes and does not know about QEMU, PL011 MMIO, Pi 5 hardware, scheduler blocking, descriptors, or shell commands.

## Newline And Echo Policy

Output written through stdout or stderr should preserve the existing console output convention: logical \n text reaches the serial console as CRLF through the backend path. The descriptor layer may later expose partial-write and errno semantics, but the current console write-result contract stays internal until that boundary exists.

Canonical-mode echo should be explicit and deterministic:

- Echo printable ASCII and tab exactly as accepted into the line buffer.
- Echo line termination as CRLF.
- Echo accepted backspace/delete as the usual erase sequence \x08 \x08.
- Do not echo unsupported control bytes as terminal actions; the diagnostic may report them by name after the line completes.

Raw mode should default to no echo. If a raw diagnostic echoes bytes, the task must label that behavior as a diagnostic choice.

## Control Characters

The first TTY contract reserves familiar control characters without implementing POSIX side effects:

- Ctrl-C, 0x03, is recorded as an interrupt-request control event. It does not deliver a signal or kill a task.
- Ctrl-D, 0x04, is recorded as an end-of-input control event. It does not implement EOF for a process.
- Ctrl-Z, 0x1a, is recorded as a suspend-request control event. It does not stop a process group.
- Ctrl-U, 0x15, may clear the current canonical buffer once canonical editing exists.
- Escape, 0x1b, is input data unless a later terminal parser task accepts escape-sequence handling.

These names are for kernel diagnostics and future compatibility only. They are not syscall ABI.

## Stdio Descriptor Shape

stdin, stdout, and stderr should become process-local file descriptors through the descriptor model described in [Early POSIX Shape](../project/early-posix-shape.md). The TTY design should assume:

- fd 0, stdin, references the readable side of the controlling TTY only after an input source exists;
- fd 1, stdout, references the normal writable side of the controlling TTY;
- fd 2, stderr, references a separate writable stream identity that may initially share the same console device as stdout;
- descriptor entries reference open file descriptions or kernel objects rather than embedding UART addresses;
- blocking, nonblocking, close, dup, fork or spawn inheritance, and errno mapping belong to descriptor and syscall tasks.

Until descriptor tables exist, kernel diagnostics may call TTY functions directly. They must not present those direct calls as POSIX read or write.

## POSIX Gaps

The first TTY is intentionally not POSIX-complete. Deferred work includes:

- termios configuration, isatty, ioctl, and stable ABI constants;
- signals for interrupt, quit, suspend, and window changes;
- sessions, controlling terminal ownership, process groups, and job control;
- PTYs for SSH and local pseudo-terminal use;
- terminal size, escape-sequence parsing, alternate screens, and cursor state;
- scheduler-backed blocking reads and writes, readiness polling, and wakeups;
- descriptor lifetime, errno mapping, permissions, and namespace integration;
- Unicode, locale, and full terminal emulation.

The design should still leave room for those features by keeping TTY state above the console backend and below descriptor/syscall policy.

## First QEMU Polling RX Diagnostic

The first bounded implementation after this design is `phase5-qemu-polling-tty-rx-diagnostic-20260524`, a QEMU-only polling TTY RX diagnostic:

- `Pl011::poll_read_byte` checks RX-empty state before reading the data register;
- `runtime_console::ConsoleInputBackend` exposes the input-capable backend without making diagnostic clients call target UART MMIO directly;
- `tty::run_polling_rx_diagnostic` reads injected serial bytes, applies the canonical-lite newline, backspace/delete, echo, control-event, truncation, and timeout policy, and prints the observed line or timeout classification;
- keep descriptor tables, syscalls, userspace, scheduler blocking, UART interrupts, Pi 5 hardware, shell commands, filesystem, networking, and SSH out of scope.

Acceptance evidence is captured by `scripts/qemu-tty-rx-diagnostic.sh`, which injects `61 62 58 08 63 59 7f 64 03 65 66 67 68 69 0d` and records `line-hex=61 62 63 64 65 66 67 68`, the exact echo bytes, `control-events=ctrl-c`, truncation, and `qemu-tty-rx-diagnostic: PASS`. Pi 5 input proof remains a later hardware-locked task.
