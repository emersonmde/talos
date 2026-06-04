# Phase 10 Scheduler-Backed Stdin Wait Core

Task: phase10-scheduler-backed-stdin-wait-core-20260604

Status: accepted

## Summary

Implemented the smallest scheduler-owned runtime-console0/local-input
readiness wait for shell-visible VFS-backed `exec stdin`. After the first
inherited `fd0=stdio-input` `TalosRead` returns `-EAGAIN`, the stdin path
now records a scheduler wait with the accepted process/task identity, descriptor
0, blocked sleep state, runnable wake state, wait-cycle count, and
`source=scheduler-runtime-console-readiness`. Delayed `talos-console0`
bytes wake the same task path, get consumed through fd0, and are reported
through inherited fd1.

No-data/no-delayed-input remains readiness/no-data with
`timeout/no-false-eof`; it is not reclassified as terminal EOF.

## Findings And Disposition

- fixed: replaced the accepted task-local bounded retry loop as the delayed
  stdin mechanism with an explicit scheduler wait/sleep and wake/resume model
  tied to task `0x100001` and fd0.
- fixed: added a one-byte readiness handoff so the scheduler wake consumes the
  delayed byte without losing it before the userspace `TalosRead` retry.
- fixed: added live wait markers and summary fields:
  `result=sleep`, `result=wakeup/resume`,
  `scheduler-wait-result=wakeup/resume`, and
  `scheduler-wait-source=scheduler-runtime-console-readiness`.
- fixed: retained the no-data control as `-EAGAIN` with
  `result=timeout/no-false-eof`; ordinary absence of input is still not EOF.
- fixed: added a task-owned QEMU/substitute scheduler-wait smoke wrapper and
  evidence log.
- removed: the prior accepted delayed-stdin evidence no longer depends on
  `read-result=bounded-wait/delayed-input`; the bounded script path is kept as
  compatibility scaffolding but now checks the scheduler-backed markers.
- deferred: select/poll, nonblocking flags, Ctrl-D EOF, pipes, redirection,
  async jobs, fork, signals, termios, writable filesystem behavior, libc stdio,
  distinct stderr routing, Pi 5 proof, networking, and SSH remain out of scope.

## Evidence Map

- scheduler-backed delayed stdin wait:
  `tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log`.
  QEMU/substitute evidence shows `Talos userspace stdin fixture no-data:
  readiness`, `talos: stdin-wait ... wake-state=blocked ... result=sleep
  source=scheduler-runtime-console-readiness`, `talos: stdin-wait ...
  wake-state=runnable ... result=wakeup/resume`, delayed
  `Talos userspace stdin fixture read: talos-console0`, and
  `exec-stdin ... read-result=scheduler-wait/delayed-input ...
  scheduler-wait-result=wakeup/resume`. The same log retains waitpid,
  laststatus, stdout, VFS exec/status, negative exec controls, descriptor-backed
  cat, final classification, and PASS.
- no-false-EOF/no-data control:
  `tasks/evidence/2026-06-04-phase10-runtime-stdin-readiness-distinction-core/qemu-local-shell-runtime-stdin-readiness-smoke.log`.
  QEMU/substitute evidence shows no input bytes produce `-EAGAIN`,
  `read-result=readiness/no-data`, scheduler sleep, and
  `result=timeout/no-false-eof` without terminal EOF or hang.
- immediate runtime-console0 stdin control:
  `tasks/evidence/2026-06-03-phase10-runtime-console0-stdin-core/qemu-local-shell-runtime-console0-stdin-smoke.log`.
  QEMU/substitute evidence still reads immediate `talos-console0` bytes
  through inherited fd0 without scheduler-wait fields.
- stderr/stdout and broader exec controls:
  `tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log`
  and the scheduler-wait smoke retain stdout/stderr, VFS exec/status/wait,
  negative exec, and descriptor-backed cat controls.

## Accepted Frontier

Accepted:

- `/bin/stdin` may enter a scheduler-owned wait/readiness state after an
  initial runtime-console0/local-input `-EAGAIN`.
- Delayed runtime-console0/local-input bytes wake the waiting task path, are
  consumed through inherited fd0, and are reported through inherited fd1.
- No delayed input within the finite QEMU/substitute harness records
  `timeout/no-false-eof` and remains readiness/no-data, not true EOF.
- Descriptor-backed VFS/open/read exec lineage, standard descriptor
  inheritance, loader temporary descriptor non-leak, lifecycle/status,
  consuming waitpid, non-consuming laststatus, stdout/stderr, fixed `/bin`
  lookup, negative exec controls, and descriptor-backed cat remain retained
  regressions.

Deferred:

- Ctrl-D EOF, select/poll, nonblocking flags, pipes, redirection, async
  execution, signals, fork, termios, writable filesystem behavior, distinct
  stderr routing, Pi 5 proof, networking, and SSH.

## Validation Summary

- formatting: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 390 no_std
  tests.
- QEMU/substitute: scheduler-backed delayed stdin wait smoke passed with
  retained evidence.
- QEMU/substitute: no-false-EOF/no-data readiness control passed with retained
  evidence.
- QEMU/substitute: immediate runtime-console0 stdin control passed with retained
  evidence.
- QEMU/substitute: userspace stderr/stdout and VFS exec/status/wait/cat control
  passed with retained evidence.

hardwareTestLock remained unlocked/restored and unused.
