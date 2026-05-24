# Phase 5 QEMU Polling TTY RX Diagnostic

Task: `phase5-qemu-polling-tty-rx-diagnostic-20260524`

Status: accepted as the first Milestone 5.2 QEMU-only serial input proof.

## Scope

This task adds a QEMU virt PL011 polling receive diagnostic while preserving the
runtime-console and TTY boundary. `Pl011::poll_read_byte` checks RX-empty before
reading the data register. `runtime_console::ConsoleInputBackend` exposes that
operation as an internal console-facing polling input backend. The diagnostic
client calls `tty::run_polling_rx_diagnostic` rather than reading target UART
MMIO directly.

The diagnostic implements only a bounded canonical-lite parser for proof
purposes. It records line bytes, echo bytes, backspace/delete handling, named
control events, truncation, and timeout classification. It does not add Pi 5
hardware input, UART interrupts, descriptor tables, syscalls, userspace, shell
commands, filesystem, networking, SSH, or scheduler blocking I/O.

## Evidence

QEMU/substitute evidence from `scripts/qemu-tty-rx-diagnostic.sh`:

```text
qemu-tty-rx-diagnostic: raw-len=15 line-len=8 terminated=true timeout=false truncated=true backspaces=1 deletes=1 controls=1
qemu-tty-rx-diagnostic: line-hex=61 62 63 64 65 66 67 68
qemu-tty-rx-diagnostic: echo-hex=61 62 58 08 20 08 63 59 08 20 08 64 65 66 67 68 0d 0a
qemu-tty-rx-diagnostic: control-events=ctrl-c
qemu-tty-rx-diagnostic: PASS
```

The injected input sequence is:

```text
61 62 58 08 63 59 7f 64 03 65 66 67 68 69 0d
```

The sequence exercises printable bytes, backspace, delete, Ctrl-C, truncation at
the eight-byte diagnostic capacity, and CR line termination. The timeout path is
covered by `tty::tests::polling_rx_diagnostic_reports_bounded_timeout_without_input`.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed 77 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed and preserved the accepted
  generic QEMU smoke behavior.
- QEMU/substitute: `scripts/qemu-tty-rx-diagnostic.sh` passed with the exact
  injected byte evidence above.
- image/archive inspection: `scripts/rpi5-image.sh` passed.
- fmt/lint/typecheck: `scripts/rpi5-format-guard-check.sh` passed.
- static inspection: `git diff --check` passed.
- static inspection: `mdbook build` was not run because `mdbook` is
  unavailable in this container.

## Next Work

The next queued Milestone 5.2 task may promote the canonical-lite behavior into
a target-independent line discipline with focused tests. Pi 5 UART10 polling RX
proof remains gated on the accepted input result contract and
`hardwareTestLock`.
