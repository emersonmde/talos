# Phase 10 Scheduler-Backed Stdin Wait Closeout

Task: phase10-scheduler-backed-stdin-wait-closeout-20260604

Status: accepted

## Summary

Closed out the scheduler-backed runtime-console0 stdin wait frontier accepted
by `phase10-scheduler-backed-stdin-wait-core-20260604`.

The accepted local stdin semantics are narrow:

- a VFS-backed `/bin/stdin` launched through the shell may observe ordinary
  runtime-console0/local-input no-data as `-EAGAIN`;
- after that no-data observation, the process may enter a scheduler-owned
  stdin wait/readiness state tied to the same task identity and inherited fd0;
- delayed `talos-console0` bytes may wake/resume the waiting path, then get
  consumed through inherited fd0 and reported through inherited fd1;
- no delayed input inside the finite QEMU/substitute harness remains
  readiness/no-data with `timeout/no-false-eof`, not true terminal EOF.

The older bounded retry evidence is no longer the accepted delayed-stdin
mechanism. Its smoke wrapper remains only as compatibility scaffolding for the
scheduler-backed markers.

## Findings And Disposition

- fixed: scheduler-backed delayed stdin evidence is reconciled with explicit
  sleep and wake/resume markers tied to task `0x100001`, fd0, wait-cycle
  count, blocked/runnable state, and
  `source=scheduler-runtime-console-readiness`.
- fixed: no-data/no-delayed-input evidence remains `-EAGAIN` with
  `read-result=readiness/no-data` and `result=timeout/no-false-eof`, avoiding
  a false terminal EOF claim.
- fixed: immediate runtime-console0 stdin remains a separate regression where
  available `talos-console0` bytes are consumed without the scheduler wait
  path.
- fixed: retained evidence still covers stdout/stderr, descriptor-backed VFS
  exec, loader temporary descriptor non-leak, lifecycle/status, consuming
  `waitpid`, non-consuming `laststatus`, fixed `/bin` lookup, negative exec
  controls, and descriptor-backed `cat /etc/banner.txt`.
- removed: acceptance of `read-result=bounded-wait/delayed-input` as the
  delayed stdin mechanism; it is superseded by
  `read-result=scheduler-wait/delayed-input`.
- deferred: true terminal EOF/Ctrl-D, select/poll, nonblocking flags, pipes,
  redirection, async jobs, fork, signals, termios, writable filesystem
  behavior, libc stdio, distinct physical stderr routing, Pi 5 proof,
  networking, and SSH remain out of scope.
- not-an-issue: this closeout made no code changes and did not acquire
  `hardwareTestLock`.

## Evidence Map

- scheduler-backed delayed input:
  `tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log`.
  Static inspection shows `Talos userspace stdin fixture no-data: readiness`,
  `talos: stdin-wait ... result=sleep ... source=scheduler-runtime-console-readiness`,
  `talos: stdin-wait ... result=wakeup/resume ... source=scheduler-runtime-console-readiness`,
  delayed `Talos userspace stdin fixture read: talos-console0`, and
  `read-result=scheduler-wait/delayed-input`.
- no-false-EOF/no-data control:
  `tasks/evidence/2026-06-04-phase10-runtime-stdin-readiness-distinction-core/qemu-local-shell-runtime-stdin-readiness-smoke.log`.
  Static inspection shows no available input returns `-EAGAIN`, records
  `read-result=readiness/no-data`, and records
  `scheduler-wait-result=timeout/no-false-eof` without terminal EOF.
- immediate runtime-console0 stdin control:
  `tasks/evidence/2026-06-03-phase10-runtime-console0-stdin-core/qemu-local-shell-runtime-console0-stdin-smoke.log`.
  Static inspection shows immediate `talos-console0` bytes read through
  inherited fd0 and reported through fd1.
- stdout/stderr and process controls:
  `tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log`
  plus the scheduler-backed smoke log retain userspace stdout/stderr,
  descriptor-backed VFS exec, lifecycle/status, consuming `waitpid`,
  non-consuming `laststatus`, fixed `/bin` lookup, negative exec controls,
  and descriptor-backed `cat /etc/banner.txt`.

## Accepted Frontier

Accepted:

- scheduler-owned runtime-console0/local-input wait/readiness after inherited
  fd0 no-data for the VFS-backed `/bin/stdin` process;
- wake/resume on delayed `talos-console0` bytes through that same stdin path;
- delayed bytes consumed through inherited fd0 and reported through inherited
  fd1;
- no-delayed-input classified as readiness/no-data with no false terminal EOF;
- retained descriptor-backed VFS/open/read exec lineage, standard descriptor
  inheritance, loader temporary descriptor non-leak, lifecycle/status,
  `waitpid`, `laststatus`, stdout/stderr, negative exec controls, and
  descriptor-backed cat regressions.

Deferred:

- true terminal EOF/Ctrl-D;
- select/poll and nonblocking descriptor flags;
- pipes, redirection, async execution, fork, signals, and termios;
- writable filesystem behavior and libc stdio;
- distinct physical stderr routing;
- Pi 5 proof, networking, and SSH.

## Recommendation

Ctrl-D EOF remains the next smallest feature-led local I/O task. It directly
completes the semantic split created by the accepted readiness/no-data and
scheduler wait work: ordinary absence of input is already `-EAGAIN`, while a
terminal EOF signal still has no runtime-console0 behavior.

## Validation Summary

- static inspection: accepted task docs and retained evidence logs inspected.
- diff hygiene: `git diff --check` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff hygiene: `git diff --cached --check` passed before commit.

hardwareTestLock remained unlocked/restored and unused.
