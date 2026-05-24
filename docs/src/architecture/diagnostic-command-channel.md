# Diagnostic Command Channel

This document records the first Milestone 5.3 command-channel contract. The
channel is kernel-owned, local, serial, and diagnostic-only. It is not a shell,
syscall ABI, descriptor table, filesystem command runner, networking endpoint,
SSH path, SMP control plane, UART interrupt path, or scheduler-blocking read
policy.

## Boundary

The command channel sits above the accepted TTY line discipline:

- input: a completed TTY line, already assembled by `src/tty.rs`;
- parser/dispatcher: target-independent code in `src/diagnostic_command.rs`;
- output: newline-framed diagnostic response text through a
  `DiagnosticResponseSink`;
- runtime output attachment: `runtime_console::RuntimeConsole` implements the
  response sink so the channel can write through runtime-console0.

The channel must not read UART bytes or PL011 MMIO directly. Target modules own
QEMU and Pi 5 UART backends, runtime-console0 owns the console identity, the TTY
owns line assembly and echo policy, and future descriptor/syscall work owns
POSIX-facing stream lifetime and errno mapping.

## Parser Contract

The accepted parser consumes byte slices representing complete TTY lines. It
interprets ASCII space and tab as token separators, supports lowercase
alphanumeric command tokens plus `-` and `_`, and rejects shell
metacharacters instead of treating them as syntax.

Limits are deliberately small:

- command or argument token length: 16 bytes;
- argument token count: two parser-level arguments;
- current built-in commands: zero arguments accepted.

Parse failures are stable diagnostic labels:

- `empty-command`;
- `invalid-utf8`;
- `unsupported-token-byte`;
- `token-too-long`;
- `too-many-arguments`.

These labels are not POSIX errno, shell status, or syscall ABI.

## Built-In Commands

The contract accepts three built-ins for the first QEMU smoke:

- `help`: emit a bounded command discovery response;
- `list`: emit the same bounded built-in command list;
- `status`: emit command-channel version, runtime console identity, TTY mode
  label, canonical line capacity, command count, and command list.

Unknown commands emit `diag: error unknown-command`. Built-in commands with
arguments emit `diag: error unexpected-argument`. Responses are deterministic
newline-framed text and avoid filesystem, process, environment, path lookup,
redirection, globbing, pipelines, or script semantics.

## QEMU Smoke Evidence

The accepted QEMU diagnostic command-channel smoke injects four serial commands
through the QEMU virt PL011 polling input backend. Each command is first
assembled by the TTY canonical-lite line discipline and only then dispatched by
`src/diagnostic_command.rs`. The captured transcript lives at
`target/qemu-diagnostic-command-channel-smoke.log` for the run and is
summarized in `tasks/2026-05-24-phase5-qemu-diagnostic-command-channel-smoke.md`.

Retained command classifications:

- `help`: retained discovery command; handled with two bounded response lines.
- `list`: retained command-list command; handled with two bounded response
  lines.
- `status`: retained status command tied to accepted command-channel,
  runtime-console0, and TTY state; handled with six bounded response lines.
- `bogus`: retained negative smoke input; classified as deterministic
  `unknown-command` with one bounded error line.

The smoke emits the response transcript outside IRQ context from the QEMU
diagnostic path in `kernel_main`. It does not allocate, does not print from
the IRQ handler, and does not introduce descriptors, syscalls, userspace shell
behavior, filesystem-backed commands, networking, SSH, SMP, UART interrupts, or
scheduler blocking I/O.

## Pi 5 Hardware Evidence

The accepted Pi 5 diagnostic command-channel proof reuses the same command
sequence over the firmware-preserved UART10 path. The hardware transcript in
`tasks/evidence/2026-05-24-pi5-diagnostic-command-channel-proof/serial-observe-settle-full.json`
shows `help`, `list`, `bogus`, and `status` entering through canonical-lite
TTY line assembly and dispatching through the same target-independent command
boundary.

The accepted archive was
`target/talos-rpi5-diagnostic-command-channel-prefixed-boot.tar.gz` with
SHA256 `babf8d0161fa37891319461e136f53d616d453966f63059ba479eb44afc10f66`.
Its kernel image was 96,304 bytes with SHA256
`83aa4425449e79989e15a91df35902de047b1db2d9e303027f766caf91a8305b`. TFTP
evidence shows the Pi fetched `da591740/kernel_2712.img` at the same size.

The Pi 5 diagnostic path inserts a bounded hardware-proof-only settle window
after receiving a complete command line and before emitting the response
transcript. This is not scheduler blocking I/O, a descriptor wait, or a shell
prompt; it exists only to let the lab serial collector capture complete
bounded proof lines.

## Current Deferrals

The command-channel contract intentionally defers:

- descriptor tables, syscall ABI, user/kernel copy, POSIX `read` or `write`,
  readiness polling, errno mapping, and blocking I/O;
- userspace shell grammar, command execution, environment variables, pipelines,
  redirection, globbing, path lookup, process spawning, and scripts;
- filesystem-backed commands, networking, SSH, SMP, RP1 UART0, UART interrupts,
  DMA/cache ownership, and scheduler sleep/wakeup;
- destructive fault triggers, allocator stress commands, panic commands, and
  translation-fault commands unless a later task accepts explicit safety
  criteria.

## Validation Role

This contract is source-backed by focused no_std parser/dispatcher tests. The
QEMU and Pi 5 command-channel smokes now cover help/list, unknown-command
handling, and status over the accepted polling TTY path.
