# Phase 5 TTY and Stdio Shape

Task: phase5-tty-stdio-shape-doc-20260524

## Goal

Define the first TTY and stdio behavior contract before implementation work begins.

## Contract Summary

The accepted shape keeps TTY policy above runtime-console0 and below future descriptor/syscall policy. The first local TTY is a serial line-discipline object backed by an accepted console input/output pair. It is not a target UART driver, shell, descriptor table, or POSIX ABI.

Raw mode delivers bytes without translation or line assembly and defaults to echo off. Canonical mode collects a bounded byte-oriented line, accepts CR/LF/CRLF as line termination, supports backspace/delete erase, echoes deterministic printable bytes and erase sequences, and records control bytes as named diagnostic events rather than signals.

Future stdin, stdout, and stderr should be descriptor-capable streams. fd 0 attaches only after a real input source exists; fd 1 and fd 2 attach to writable TTY streams that may initially share runtime-console0.

## Known POSIX Gaps

termios, ioctl, signals, sessions, job control, PTYs, terminal size, escape parsing, blocking/nonblocking I/O, readiness wakeups, descriptor lifetime, errno mapping, userspace, filesystems, local shell, networking, and SSH remain deferred.

## Next Task Recommendation

Queue phase5-qemu-polling-tty-rx-diagnostic-20260524 as the first implementation slice. It should add a QEMU-only PL011 polling RX byte operation and a bounded canonical-lite diagnostic with QEMU/substitute evidence for injected serial input. It must not implement Pi 5 input, UART interrupts, descriptors, syscalls, userspace, scheduler blocking I/O, shell commands, filesystem, networking, or SSH.

## Evidence

- static inspection: docs/src/architecture/tty-stdio.md defines the raw/canonical, newline, backspace, echo, control-character, and stdio stream contract.
- static inspection: docs/src/project/phase5-tty-stdio-shape.md records accepted scope, POSIX gaps, and the next bounded implementation recommendation.
- fmt/lint/typecheck: git status --short was clean before edits; git diff --check and git diff --cached --check passed.
- static inspection: mdbook was unavailable in the container, so mdbook build was not run.
- Rust fmt/tests were not required because this task changed only Markdown documentation and durable task state.
