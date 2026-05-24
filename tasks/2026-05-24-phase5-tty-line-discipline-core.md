# Phase 5 TTY Line Discipline Core

Task: `phase5-tty-line-discipline-core-20260524`

Status: accepted as the Milestone 5.2 target-independent line-discipline core.

## Scope

This task promotes the canonical-lite behavior from the QEMU polling RX
diagnostic into `src/tty.rs` as `TtyLineDiscipline`. The core is byte-oriented
and target-independent. It owns raw-mode pass-through, canonical-lite line
assembly, deterministic echo bytes, named deferred control events, buffer-limit
reporting, and parser outcomes. Timeout remains outside the core in the polling
diagnostic wrapper.

The QEMU RX diagnostic still enters through the runtime-console input boundary
and now uses the shared line-discipline state through `PollingTtyRxResult`.
This task does not add Pi 5 input, UART interrupts, descriptor tables, syscalls,
userspace, shell commands, filesystem, networking, SSH, or scheduler blocking
I/O.

## Parser Invariants

- `TtyMode::Raw` records input bytes without newline, backspace/delete, control,
  or echo translation.
- `TtyMode::CanonicalLite` accepts printable ASCII and tab into the bounded
  line buffer and echoes accepted bytes.
- CR and LF terminate the current canonical line and emit CRLF echo bytes.
- Backspace and delete remove one buffered byte when non-empty and emit the erase
  echo sequence.
- Ctrl-C, Ctrl-D, Ctrl-Z, and Ctrl-U are recorded as future-compatibility control
  labels only; they do not deliver POSIX signals or EOF.
- Escape is retained as input data without terminal parsing; unsupported C0 bytes
  are recorded as `unsupported-control`.
- Buffer exhaustion sets `truncated` and returns `TtyInputOutcome::BufferLimit`;
  timeout classification belongs to polling callers, not the line discipline.

## Evidence

QEMU/substitute regression evidence from `scripts/qemu-tty-rx-diagnostic.sh`:

```text
qemu-tty-rx-diagnostic: raw-len=15 line-len=8 terminated=true timeout=false truncated=true backspaces=1 deletes=1 controls=1
qemu-tty-rx-diagnostic: line-hex=61 62 63 64 65 66 67 68
qemu-tty-rx-diagnostic: echo-hex=61 62 58 08 20 08 63 59 08 20 08 64 65 66 67 68 0d 0a
qemu-tty-rx-diagnostic: control-events=ctrl-c
qemu-tty-rx-diagnostic: PASS
```

Unit coverage now includes canonical newline handling, printable/tab/Escape
editing, backspace/delete echo, named Ctrl-C/Ctrl-D/Ctrl-Z/Ctrl-U labels,
unsupported C0 controls, buffer limits without implicit timeout, raw-mode
pass-through, and the existing polling timeout/diagnostic path.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed 82 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- QEMU/substitute: `scripts/qemu-tty-rx-diagnostic.sh` passed with the exact
  line, echo, control-event, truncation, and PASS output above.
- image/archive inspection: `scripts/rpi5-image.sh` passed.
- fmt/lint/typecheck: `scripts/rpi5-format-guard-check.sh` passed.
- static inspection: `git diff --check` passed.
- static inspection: `mdbook build` was not run because `mdbook` is
  unavailable in this container.

## Next Work

The next queued Milestone 5.2 task may define the console input result contract
that a later Pi 5 UART10 input proof and descriptor-facing stdin work can depend
on. Pi 5 hardware input remains gated by the explicit hardware-test task and
`hardwareTestLock`.
