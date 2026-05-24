# Phase 5 Diagnostic Command Channel Contract

Task: phase5-diagnostic-command-channel-contract-20260524
Status: accepted

## Scope

This task defined the first durable command-channel contract and added the
minimal target-independent parser/dispatcher shape. It consumes completed TTY
lines and emits bounded diagnostic responses through a sink that can attach to
runtime-console0.

No hardware run, hardware publish, Pi 5-specific behavior, descriptor table,
syscall ABI, userspace shell, filesystem command execution, networking, SSH,
SMP, UART interrupt, scheduler blocking read, process management, pipeline,
redirection, globbing, path lookup, or script behavior was added.

## Contract Summary

- Source boundary: `src/diagnostic_command.rs`.
- Input boundary: complete TTY line bytes, not direct UART bytes.
- Parser limits: 16-byte command/argument tokens and at most two argument
  tokens.
- Built-ins: `help`, `list`, and `status`.
- Response framing: deterministic `diag:` newline-framed kernel diagnostic
  text.
- Runtime output attachment: `runtime_console::RuntimeConsole` implements
  `DiagnosticResponseSink`, preserving runtime-console0 as the output
  identity.
- Error labels: `empty-command`, `invalid-utf8`,
  `unsupported-token-byte`, `token-too-long`, `too-many-arguments`,
  `unexpected-argument`, and `unknown-command`.

These labels and responses are internal kernel diagnostics only. They are not
POSIX errno, shell status, descriptor readiness, syscall ABI, filesystem
behavior, networking behavior, or SSH behavior.

## Test Coverage

The no_std test suite covers:

- completed TTY line token parsing with spaces and tabs;
- rejection of empty input, shell-like unsupported syntax, overlong tokens, and
  excessive arguments;
- deterministic `help` command response;
- deterministic `status` response without POSIX/process/filesystem state;
- unknown command, parse-error, and unexpected-argument reporting;
- response sink failure propagation.

## Validation

- static inspection: `git status --short` was clean before edits.
- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed 90 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed.
- image/archive inspection: `scripts/rpi5-image.sh` passed.
- fmt/lint/typecheck: `scripts/rpi5-format-guard-check.sh` and
  `git diff --check` passed.
- static inspection: `mdbook` was unavailable in the container, so mdBook
  build was not run.

## Next Task

The next queued task is
`phase5-qemu-diagnostic-command-channel-smoke-20260524`. It should prove this
contract over the accepted QEMU polling TTY path with a captured serial
transcript before any Pi 5 command-channel proof or descriptor/syscall/shell
work starts.
