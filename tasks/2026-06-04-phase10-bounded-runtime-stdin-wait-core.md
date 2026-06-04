# Phase 10 Bounded Runtime Stdin Wait Core

Task: phase10-bounded-runtime-stdin-wait-core-20260604

Status: accepted

## Summary

Implemented the thinnest bounded runtime-console0/local-input wait path for
shell-visible VFS-backed `exec stdin`. The launched `/bin/stdin` fixture now
records an initial `-EAGAIN` readiness/no-data observation when no bytes are
immediately available, emits the visible readiness report through inherited
fd1, then retries `TalosRead` within a task-local finite budget. If
`talos-console0` bytes arrive during that budget, the fixture consumes them
through inherited `fd0=stdio-input` and reports the delayed bytes through
inherited fd1.

The no-bytes-within-budget path remains deterministic: it reports
`read-result=readiness/no-data`, records the bounded readiness observation
count, and exits without claiming true EOF or full POSIX blocking read
semantics.

## Findings And Disposition

- fixed: added a task-local bounded retry loop for `/bin/stdin` runtime-console0
  input after the first no-data observation.
- fixed: made delayed input evidence visibly distinct with
  `read-result=bounded-wait/delayed-input` and `readiness-observations=...`.
- fixed: preserved the no-bytes-within-budget readiness/no-data path with
  `-EAGAIN`; ordinary no-data still is not true EOF.
- fixed: added a QEMU/substitute delayed-byte smoke wrapper and retained
  task-local delayed-byte and no-bytes-within-budget evidence logs.
- not-an-issue: immediate `talos-console0` stdin still succeeds without a
  readiness observation and keeps the accepted `read-result` field absent.
- deferred: scheduler-backed blocking reads, wait queues, select/poll,
  nonblocking descriptor flags, Ctrl-D EOF policy, canonical terminal
  expansion, async execution, fork, signals, pipes, redirection, distinct
  stderr stream routing, writable filesystem behavior, libc stdio, Pi 5
  hardware proof, networking, and SSH remain out of scope.

## Evidence Map

- delayed runtime stdin bounded wait:
  `tasks/evidence/2026-06-04-phase10-bounded-runtime-stdin-wait-core/qemu-local-shell-bounded-runtime-stdin-wait-smoke.log`.
  QEMU/substitute evidence shows `Talos userspace stdin fixture no-data:
  readiness` before `Talos userspace stdin fixture read: talos-console0`, then
  `exec-stdin ... bytes=0x000000000000000e return=0x000000000000000e
  read-source=runtime-console0/local-input ... stdout-bytes=0x64
  read-result=bounded-wait/delayed-input readiness-observations=0x234`, plus
  inherited fd0/fd1/fd2, loader temporary descriptor non-leak,
  lifecycle/status, consuming `waitpid`, non-consuming `laststatus`, negative
  exec controls, descriptor-backed `cat /etc/banner.txt`, final
  `participants=18 expected=18 errors=0`, classification
  `qemu-local-shell-userspace-stdin-complete`, and PASS.
- no-bytes-within-budget readiness/no-data regression:
  `tasks/evidence/2026-06-04-phase10-bounded-runtime-stdin-wait-core/qemu-local-shell-bounded-runtime-stdin-no-data-smoke.log`.
  QEMU/substitute evidence shows `return=0xfffffffffffffff5`,
  `read-result=readiness/no-data`, and
  `readiness-observations=0x00000000000f4240` without delayed bytes or hang,
  with the same VFS exec, descriptor inheritance, lifecycle/status, waitpid,
  laststatus, negative controls, descriptor-backed cat, final classification,
  and PASS.
- immediate runtime-console0 stdin regression:
  `tasks/evidence/2026-06-03-phase10-runtime-console0-stdin-core/qemu-local-shell-runtime-console0-stdin-smoke.log`.
  Re-run evidence confirmed `Talos userspace stdin fixture read:
  talos-console0` and `exec-stdin ... bytes=0x000000000000000e
  return=0x000000000000000e read-source=runtime-console0/local-input` without
  a readiness observation.
- unit tests:
  `cargo -Zjson-target-spec test --quiet` passed with 390 no_std tests,
  including delayed-input and no-bytes-within-budget stdin tests.

## Accepted Frontier

Accepted:

- `/bin/stdin` may perform a bounded task-local retry after an initial
  runtime-console0/local-input no-data observation.
- delayed runtime-console0/local-input bytes arriving within the bounded budget
  are consumed through inherited fd0 and reported through inherited fd1.
- no bytes within the bounded budget remains readiness/no-data (`-EAGAIN`), not
  true EOF.
- descriptor-backed VFS/open/read exec lineage, standard descriptor
  inheritance, loader temporary descriptor non-leak, lifecycle/status,
  consuming waitpid, non-consuming laststatus, stdout/stderr, fixed `/bin`
  lookup, negative exec controls, and descriptor-backed cat remain retained
  regressions.

Deferred:

- full scheduler-backed blocking read semantics, readiness polling APIs,
  nonblocking descriptor flags, terminal EOF/Ctrl-D policy, pipes,
  redirection, async execution, signals, fork, writable filesystem behavior,
  distinct stderr stream routing, Pi 5 proof, networking, and SSH.

## Validation Summary

- formatting: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 390 no_std
  tests.
- QEMU/substitute: delayed runtime stdin bounded-wait smoke passed with retained
  evidence.
- QEMU/substitute: no-bytes-within-budget readiness/no-data regression passed
  with retained evidence.
- QEMU/substitute: immediate runtime-console0 stdin regression passed and
  retained the accepted successful read behavior.

hardwareTestLock remained unlocked/restored and unused.
