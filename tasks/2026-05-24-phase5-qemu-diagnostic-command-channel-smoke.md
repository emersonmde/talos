# Phase 5 QEMU Diagnostic Command Channel Smoke

Task: `phase5-qemu-diagnostic-command-channel-smoke-20260524`
Status: accepted

## Scope

This task proves the accepted Milestone 5.3 diagnostic command-channel contract
over the QEMU polling TTY path. `scripts/qemu-diagnostic-command-channel-smoke.sh`
starts QEMU with a socket-backed PL011 serial port, injects one command after
each kernel `ready command=N` line, and captures the serial transcript at
`target/qemu-diagnostic-command-channel-smoke.log`.

The kernel diagnostic path calls `tty::run_polling_rx_diagnostic` for each
command, then dispatches the completed TTY line through
`diagnostic_command::dispatch_default_diagnostic_command`. It does not read
UART bytes from the command dispatcher, print from IRQ context, allocate, add
descriptor tables, define syscalls, add userspace shell behavior, execute
filesystem-backed commands, add networking, SSH, SMP, UART interrupts, or add
scheduler-blocking I/O.

## Transcript Summary

Captured QEMU/substitute evidence:

```text
qemu-diagnostic-command-channel-smoke: start command-count=4 backend=runtime-console0/qemu-virt-pl011 input=tty-canonical-lite
qemu-diagnostic-command-channel-smoke: line command=0 hex=68 65 6c 70
diag: ok help
diag: commands help list status
qemu-diagnostic-command-channel-smoke: dispatch command=0 status=handled responses=2
qemu-diagnostic-command-channel-smoke: line command=1 hex=6c 69 73 74
diag: ok list
diag: commands help list status
qemu-diagnostic-command-channel-smoke: dispatch command=1 status=handled responses=2
qemu-diagnostic-command-channel-smoke: line command=2 hex=62 6f 67 75 73
diag: error unknown-command
qemu-diagnostic-command-channel-smoke: dispatch command=2 status=unknown-command responses=1
qemu-diagnostic-command-channel-smoke: line command=3 hex=73 74 61 74 75 73
diag: ok status
diag: version phase5.3-contract-v1
diag: runtime-console runtime-console0
diag: tty canonical-lite line-capacity 8
diag: command-count 3
diag: commands help list status
qemu-diagnostic-command-channel-smoke: dispatch command=3 status=handled responses=6
qemu-diagnostic-command-channel-smoke: PASS
```

## Command Classification

- `help`: retained discovery command with two deterministic response lines.
- `list`: retained command-list command with two deterministic response lines.
- `bogus`: retained negative smoke input for deterministic unknown-command
  behavior.
- `status`: retained status command tied to accepted command-channel version,
  runtime-console0 identity, canonical-lite TTY line capacity, command count,
  and command list.

Deferred command classes remain destructive fault triggers, allocator stress
commands, panic commands, filesystem-backed commands, process controls,
networking, SSH, SMP controls, and shell grammar.

## IRQ and Context Audit

The smoke runs from the QEMU diagnostic branch in `kernel_main`, after target
initialization and before generic QEMU smoke exit. It uses polling input and a
runtime-console response sink in normal kernel context. No command dispatch path
is called from the IRQ handler, and the smoke does not enable UART interrupts or
timer-driven command execution.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test` passed 90 no_std tests.
- QEMU/substitute: `scripts/qemu-smoke.sh` passed and preserved the accepted
  generic QEMU smoke output.
- QEMU/substitute: `scripts/qemu-diagnostic-command-channel-smoke.sh` passed
  with the transcript summary above.
- image/archive inspection: `scripts/rpi5-image.sh` passed.
- fmt/lint/typecheck: `scripts/rpi5-format-guard-check.sh`, `git diff --check`,
  and `git diff --cached --check` passed.
- static inspection: `mdbook` was unavailable in the container, so mdBook
  build was not run.

## Next Task

The next queued Milestone 5.3 task may perform the serialized Pi 5 diagnostic
command-channel proof. Descriptor tables, syscalls, userspace shell behavior,
filesystem commands, networking, SSH, SMP, UART interrupts, and scheduler
blocking I/O remain out of scope until later supervisor-planned tasks.
