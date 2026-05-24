# Phase 5 TTY and Stdio Shape

Status: accepted as the Milestone 5.2 documentation boundary before any TTY, UART RX, descriptor, syscall, userspace, shell, filesystem, networking, SSH, hardware, or scheduler blocking implementation.

## Scope

This task defines the first local TTY and stdio behavior contract. It reconciles:

- [Console Device Model](../architecture/console.md), which owns runtime-console0 and the output/input-source boundary.
- [Early POSIX Shape](early-posix-shape.md), which defines future process-local file descriptors and descriptor-backed streams.
- [TTY and Stdio Shape](../architecture/tty-stdio.md), which records the accepted Milestone 5.2 contract.

No kernel code, boot image, hardware publish, power-cycle, hardware test, UART RX implementation, line-discipline implementation, descriptor table, syscall ABI, userspace, shell command channel, filesystem, networking, SSH, or scheduler behavior changed in this task.

## Accepted Contract

The first local TTY is a line-discipline object layered over the accepted runtime console model, not a target UART driver and not a descriptor table. Target modules still own the physical UART backend, runtime console owns runtime-console0, and future descriptor code should attach stdin, stdout, and stderr to TTY-backed stream objects.

Raw mode delivers bytes without line assembly or translation, with echo off by default. Canonical mode collects a bounded byte-oriented line, treats \r, \n, and collapsed \r\n as line terminators, handles backspace/delete as erase, appends printable ASCII and tab, and records unsupported control bytes as named events rather than POSIX signals.

Output newline policy keeps the existing console convention: logical \n reaches serial as CRLF through the backend path. Canonical echo mirrors accepted printable bytes, CRLF line termination, and \x08 \x08 erase. Raw-mode echo, if used by a diagnostic, must be labeled as diagnostic behavior.

## Stdio Shape

stdin, stdout, and stderr are future descriptor-capable streams:

- fd 0 should attach to the readable side of the controlling TTY only after an input source exists.
- fd 1 should attach to the normal writable side of the controlling TTY.
- fd 2 should attach to a separate writable stream identity that may initially share the same console device as fd 1.

The descriptor layer owns fd allocation, lifetime, inheritance, blocking/nonblocking behavior, errno mapping, close/dup, and syscall ABI. TTY code must not call QEMU or Pi 5 target UART backends directly, and kernel diagnostics must not present direct TTY calls as POSIX read or write.

## POSIX Gaps And Deferred Work

Deferred work remains explicit:

- QEMU PL011 RX implementation and a polling TTY RX diagnostic.
- Pi 5 UART10 input proof under hardwareTestLock.
- UART interrupts, buffering, scheduler sleep/wakeup, and readiness polling.
- Descriptor tables, syscalls, errno mapping, userspace, and process lifetime integration.
- termios, ioctl, signals, sessions, controlling terminals, process groups, job control, and PTYs.
- Terminal size, escape parsing, cursor state, Unicode, locale, and full terminal emulation.
- Local shell, filesystems, program loading, networking, and SSH.

## Next Implementation Recommendation

The next bounded task should be phase5-qemu-polling-tty-rx-diagnostic-20260524.

Recommended scope:

- add a QEMU-only PL011 polling RX byte operation that checks RX-empty state before reading;
- expose that input source to a TTY diagnostic without descriptor tables or syscalls;
- implement a bounded canonical-lite diagnostic that accepts injected serial bytes, echoes according to the accepted policy, reports the observed line, control events, truncation, or timeout, and exits through normal kernel diagnostics;
- validate with QEMU/substitute evidence that exact injected bytes reach the diagnostic.

Non-goals for that task should include Pi 5 hardware, UART interrupts, descriptor implementation, syscalls, userspace, scheduler blocking I/O, shell commands, filesystem, networking, and SSH.

## Validation

- fmt/lint/typecheck: git status --short was clean before documentation edits.
- fmt/lint/typecheck: git diff --check passed for documentation and task-record changes.
- static inspection: mdbook was unavailable in the container, so mdbook build was not run.
- Rust fmt/tests were not required because this task changed only Markdown documentation and durable task state.
