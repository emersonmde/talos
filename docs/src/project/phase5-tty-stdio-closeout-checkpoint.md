# Phase 5 TTY and Stdio Closeout Checkpoint

Status: accepted for Milestone 5.2 closeout.

This checkpoint reconciles the accepted TTY and stdio work before Talos starts a
local diagnostic command channel, descriptor implementation, userspace, shell,
filesystem, networking, SSH, SMP, or any later phase work.

## Accepted Capabilities

Milestone 5.2 now has an evidence-backed local serial input shape:

- runtime-console0 remains the named default runtime console identity.
- The TTY/stdio design keeps line discipline above the runtime-console backend
  and below future descriptor/syscall policy.
- Raw mode and canonical-lite mode are documented for the first serial TTY.
- Canonical-lite parsing is implemented in a target-independent
  TtyLineDiscipline core.
- The runtime-console input contract distinguishes byte available, RX empty,
  backend unavailable, and backend error without exposing those names as POSIX
  ABI.
- QEMU virt proves PL011 polling RX through the runtime-console/TTY boundary
  with deterministic echo, backspace/delete handling, line completion, bounded
  timeout coverage, truncation reporting, and named control events.
- Raspberry Pi 5 proves the same diagnostic path on the firmware-preserved
  UART10 console with serialized hardware evidence.

## Evidence Reconciliation

Accepted work:

- 43be668 accepted the TTY/stdio behavior shape and descriptor-facing stdin,
  stdout, and stderr direction.
- bccd274 accepted the QEMU-only polling TTY RX diagnostic. The injected
  sequence 61 62 58 08 63 59 7f 64 03 65 66 67 68 69 0d produced
  line-hex=61 62 63 64 65 66 67 68, control-events=ctrl-c, and
  qemu-tty-rx-diagnostic: PASS.
- 87b4a55 accepted the target-independent line-discipline core and tests for
  raw mode, canonical-lite editing, newline, backspace/delete, control labels,
  buffer limits, unsupported controls, and timeout classification at the
  polling wrapper.
- 81ace36 accepted the internal console input result contract and preserved the
  distinction between backend polling outcomes and diagnostic completion
  outcomes.
- b2b3e74 accepted the Pi 5 UART10 polling RX proof. The accepted archive
  target/talos-rpi5-uart10-rx-boot.tar.gz had SHA256
  bab86eacea7868b4fd92423370c4991b11cc6f270c60b7b38ae5960336f54209, the
  kernel image had SHA256
  2a497150163f6e53ec6b5d4b33c4e44f0f3d29f6f34f4b319a9e93515ba83a6d, TFTP
  served the 90,344-byte candidate kernel_2712.img, and serial output reached
  rpi5-uart10-rx-diagnostic: PASS after the lab wrote the 15-byte test
  sequence.

Evidence levels covered static documentation inspection, fmt/lint/typecheck,
no-std unit tests, QEMU/substitute smokes, image/archive inspection, serialized
Pi 5 hardware boot/output, lab-controller serial injection, TFTP/archive proof,
restore proof, and whitespace inspection.

## Deferred Work

The accepted Milestone 5.2 boundary remains deliberately narrow. Deferred work
includes:

- UART interrupts, receive buffering, readiness notification, and scheduler
  sleep/wakeup integration.
- Descriptor tables, stdin/stdout/stderr file-descriptor lifetime, dup/close,
  errno mapping, nonblocking behavior, and user/kernel copy.
- POSIX termios, isatty, ioctl, signals, sessions, process groups, job control,
  PTYs, terminal size, escape-sequence parsing, Unicode, locale, and full
  terminal emulation.
- Local shell behavior, command execution, filesystem-backed programs, process
  creation, wait/exit, and EL0 userspace.
- RP1 UART0 ownership, PCIe/RP1 driver policy, DMA/cache ownership, filesystem,
  networking, SSH, and SMP.

The Pi 5 UART10 proof is a bounded polling diagnostic. It is not evidence for
interrupt-driven UART receive, descriptor reads, blocking I/O, shell command
input, POSIX signals, termios, PTYs, filesystem behavior, networking, or SSH.

## Next Bounded Slice

The next supervisor-planned worker task should be
phase5-local-diagnostic-command-channel-source-inventory-20260524: a
documentation/source-inventory slice for Milestone 5.3 that defines the local
kernel diagnostic command channel boundary before any implementation.

That task should inventory the accepted runtime-console and TTY surfaces,
identify which existing boot-image diagnostics are candidates for later
command-channel exposure, and state how the command channel remains separate
from the future user shell. It should not implement command parsing,
descriptors, syscalls, userspace, filesystem behavior, networking, SSH, SMP, or
new hardware tests.

## Validation

- static inspection: git status --short was clean before checkpoint edits.
- static inspection: git diff --check passed after checkpoint edits.
- static inspection: mdbook build was not run because mdbook is unavailable in
  this container.
- Rust fmt/tests were not required because this checkpoint changes only
  Markdown documentation and durable task state.
