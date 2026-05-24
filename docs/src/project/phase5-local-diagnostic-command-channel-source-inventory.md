# Phase 5 Local Diagnostic Command Channel Source Inventory

Status: accepted source inventory for Milestone 5.3 planning.

This note inventories the accepted console, TTY, and diagnostic surfaces before
Talos adds a local kernel diagnostic command channel. It is not an
implementation task. It does not add a parser, command registry, hardware run,
descriptor table, syscall ABI, userspace shell, filesystem, networking, SSH,
SMP, UART interrupts, or scheduler blocking I/O.

## Accepted Console And TTY Boundary

The command channel must consume complete TTY lines and emit response text
through the accepted local console path:

- runtime-console0 is the named default runtime console identity.
- src/runtime_console.rs owns the internal output write boundary and the
  ConsoleWriteOutcome result contract.
- ConsoleInputPollOutcome names byte-available, no-data, unavailable-input,
  and backend-error polling results without exposing POSIX readiness or errno.
- src/tty.rs owns the target-independent TtyLineDiscipline core and the
  PollingTtyRxResult diagnostic completion labels.
- QEMU PL011 and Pi 5 UART10 have both proved the same bounded polling TTY RX
  path with the 15-byte injected sequence used by the Milestone 5.2 evidence.

The diagnostic command channel must not read UART MMIO directly. It should
receive an already completed TTY line from the TTY-facing boundary and write
bounded response text through runtime-console0. This keeps target hardware
selection in target modules and keeps descriptor, syscall, and userspace shell
ownership separate.

## Candidate Command Providers

The first command provider set should expose only current kernel-owned state
that already has evidence-backed ownership:

- help or commands: list the built-in diagnostic commands and response
  framing. This proves deterministic command discovery without shell grammar.
- status: report a small static kernel status line such as target family,
  runtime console identity, and accepted command-channel version.
- ticks or timer: report the accepted EL2 timer/monotonic tick diagnostic
  counters when that data is available on the selected diagnostic path.
- sched: report bounded single-core scheduler counters for the current
  diagnostic path, such as task progress and switch/preemption counters, without
  mutating runnable queues from the command layer.
- memory: report a small accepted memory/runtime summary only from existing
  boot-time facts, such as managed memory range or bootstrap allocator status,
  and only if the provider can avoid allocation-heavy formatting.

These are candidates, not committed command names. The next contract task owns
the exact command names, token limits, response framing, and unknown-command
behavior.

## Boot-Only Regression Surfaces

Several existing diagnostics should remain boot-image or QEMU script regression
gates rather than become interactive commands in the first command channel:

- QEMU qemu-smoke, timer IRQ, context-switch, scheduler-yield,
  timer-preemption, and TTY RX diagnostic scripts.
- Pi 5 timer IRQ, timer-preemption, and UART10 RX diagnostic images, because
  they prove target-specific hardware paths with archive/TFTP/serial evidence.
- Pi 5 exception, panic, translation-fault, allocator OOM, Vec/String/realloc
  growth, page-frame reuse, and heap-expansion policy diagnostics.

These surfaces are useful validation artifacts, but promoting them directly to
commands would blur destructive/faulting tests, boot-time setup, and interactive
diagnostics. They should be retained as regression gates until a closeout task
explicitly retires or renames them.

## Deferred Or Retired Surfaces

The first command channel must defer:

- shell grammar, command execution, environment variables, pipes, redirection,
  globbing, path lookup, process spawning, and scripts;
- descriptor allocation, fd lifetime, read/write syscalls, errno mapping,
  readiness polling, user/kernel copy, and blocking I/O;
- filesystem-backed commands, userspace process state, networking, SSH, SMP,
  RP1 UART0, UART interrupts, DMA/cache ownership, and scheduler sleep/wakeup;
- interactive fault injection, panic triggers, translation-fault triggers, and
  allocator OOM commands unless a later task defines explicit safety criteria.

Historical stale Pi 5 probe/proof surfaces deleted during maintainability
remediation remain retired and are not command-channel candidates.

## Recommended Next Task

The next bounded task should be
phase5-diagnostic-command-channel-contract-20260524.

Recommended acceptance criteria:

- The command-channel contract consumes complete TTY lines rather than UART
  bytes directly.
- Unknown, help/list, and status behavior are deterministic and bounded.
- Command names, argument token limits, response framing, and error reporting
  are kernel diagnostics only, not shell or POSIX ABI.
- TTY, runtime-console, descriptor, syscall, and shell responsibilities remain
  separated in docs and code.
- If Rust code is added, focused no_std tests cover parser and dispatcher edge
  cases.

That task may introduce a target-independent parser/dispatcher shape, but it
must not run hardware, implement descriptor/syscall/userspace behavior, add
filesystem-backed commands, or introduce networking, SSH, SMP, UART interrupts,
or scheduler blocking reads.

## Validation

- static inspection: git status --short was clean before documentation edits.
- static inspection: accepted source references included src/runtime_console.rs,
  src/tty.rs, src/target/qemu_virt.rs, src/target/rpi5.rs,
  src/scheduler.rs, src/arch/aarch64/generic_timer.rs,
  src/diagnostics/rpi5.rs, the QEMU and Pi 5 diagnostic scripts, and the
  accepted Phase 5.2 checkpoint.
- static inspection: git diff --check passed after documentation edits.
- static inspection: mdbook build was not run because mdbook is unavailable in
  this container.
- Rust fmt/tests were not required because this task changed only Markdown
  documentation and durable task state.
