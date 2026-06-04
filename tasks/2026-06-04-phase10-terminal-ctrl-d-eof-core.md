# Phase 10 Terminal Ctrl-D EOF Core

Task: phase10-terminal-ctrl-d-eof-core-20260604

Status: accepted

## Summary

Implemented the smallest true terminal EOF path for shell-visible VFS-backed
`exec stdin`. A first runtime-console0/local-input byte of Ctrl-D 0x04 on
inherited `fd0=stdio-input` now returns `0` from the console-backed
`TalosRead` path. The launched `/bin/stdin` fixture reports the result
through inherited fd1 as
`Talos userspace stdin fixture read-result: terminal-eof` and records
`read-result=terminal-eof`.

Ordinary runtime-console0 no-data remains `-EAGAIN` with
`read-result=readiness/no-data`, and delayed input remains covered by the
accepted scheduler wait/wakeup path.

## Findings And Disposition

- fixed: first-byte Ctrl-D 0x04 on console-backed stdin now maps to bounded
  true EOF, returning `0` without copying data.
- fixed: `/bin/stdin` now reports true EOF through the accepted userspace
  fd1 stdout path with an explicit `terminal-eof` marker.
- fixed: added a task-owned QEMU/substitute smoke wrapper that sends
  `exec stdin` followed by raw Ctrl-D and retains the evidence log.
- fixed: added a focused local command-loop unit regression for Ctrl-D EOF
  plus waitpid/laststatus observation.
- not-an-issue: ordinary absence of input is still readiness/no-data; the
  retained no-data control remains `-EAGAIN` and does not claim EOF.
- not-an-issue: delayed bytes still use scheduler wait/wakeup evidence and are
  not reclassified as EOF.
- deferred: full termios canonical mode, POSIX terminal sessions, signals,
  job control, pipes, redirection, select/poll, nonblocking flags, async
  execution, fork, writable filesystem behavior, distinct physical stderr
  routing, Pi 5 proof, networking, and SSH remain out of scope.

## Evidence Map

- terminal Ctrl-D EOF:
  `tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log`.
  QEMU/substitute evidence shows `exec stdin`, visible
  `Talos userspace stdin fixture read-result: terminal-eof`,
  `exec-stdin ... return=0x0000000000000000 ... read-result=terminal-eof`,
  waitpid/laststatus, stdout, VFS exec/status, negative exec controls,
  descriptor-backed cat, final classification, and PASS.
- scheduler-backed delayed input control:
  `tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log`.
  Rerun QEMU/substitute control passed; retained evidence shows
  `read-result=scheduler-wait/delayed-input` and wake/resume markers.
- no-data/readiness control:
  `tasks/evidence/2026-06-04-phase10-runtime-stdin-readiness-distinction-core/qemu-local-shell-runtime-stdin-readiness-smoke.log`.
  Rerun QEMU/substitute control passed; retained evidence shows `-EAGAIN`,
  `read-result=readiness/no-data`, and `timeout/no-false-eof`.
- stderr/stdout and process controls:
  `tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log`.
  Rerun QEMU/substitute control passed and retains userspace stderr/stdout,
  VFS exec/status/wait, negative exec, and descriptor-backed cat behavior.

## Accepted Frontier

Accepted:

- Ctrl-D 0x04 as the first available runtime-console0/local-input byte for a
  VFS-backed `/bin/stdin` inherited fd0 read is true terminal EOF.
- Terminal EOF is reported to the shell through inherited fd1 with
  `read-result=terminal-eof` and `return=0`.
- Ordinary no-data/readiness remains `-EAGAIN`, not EOF.
- Delayed runtime-console0 bytes remain scheduler-wait/delayed-input, not EOF.
- Descriptor-backed VFS/open/read exec lineage, standard descriptor
  inheritance, loader temporary descriptor non-leak, lifecycle/status,
  `waitpid`, `laststatus`, stdout/stderr, negative exec controls, and
  descriptor-backed cat regressions remain retained.

Deferred:

- full termios/canonical mode beyond this bounded EOF byte;
- POSIX signal/session/job-control semantics;
- select/poll and nonblocking descriptor flags;
- pipes, redirection, async execution, fork, writable filesystem behavior,
  distinct stderr routing, Pi 5 proof, networking, and SSH.

## Validation Summary

- formatting: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute: `scripts/qemu-local-shell-terminal-ctrl-d-eof-smoke.sh --quiet` passed with retained task evidence.
- QEMU/substitute: `scripts/qemu-local-shell-scheduler-backed-stdin-wait-smoke.sh --quiet` passed as delayed-input control.
- QEMU/substitute: `scripts/qemu-local-shell-runtime-stdin-readiness-smoke.sh --quiet` passed as no-data/readiness control.
- QEMU/substitute: `scripts/qemu-local-shell-userspace-stderr-smoke.sh --quiet` passed as stderr/stdout and VFS exec/status/wait/cat control.

hardwareTestLock remained unlocked/restored and unused.
