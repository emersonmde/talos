# Phase 10 Terminal Ctrl-D EOF Closeout

Task: phase10-terminal-ctrl-d-eof-closeout-20260604

Status: accepted

## Summary

Closed out the terminal Ctrl-D EOF frontier accepted by
`phase10-terminal-ctrl-d-eof-core-20260604`.

The accepted stdin semantics are now split three ways for shell-visible
VFS-backed `exec stdin`:

- first-byte runtime-console0/local-input Ctrl-D 0x04 on inherited fd0 is true
  terminal EOF and returns 0;
- ordinary runtime-console0/local-input no-data remains `-EAGAIN` with
  `read-result=readiness/no-data`;
- delayed runtime-console0 bytes still use the scheduler-owned stdin
  wait/wakeup path and report `read-result=scheduler-wait/delayed-input`.

This is a terminal-stdin policy checkpoint only. It does not accept full
termios, signals, sessions, job control, pipes, redirection, select/poll,
nonblocking flags, async execution, fork, writable filesystem behavior, or
distinct physical stderr routing.

## Findings And Disposition

- fixed: terminal Ctrl-D EOF is reconciled as a true EOF condition distinct
  from readiness/no-data and delayed-input wakeup.
- fixed: the accepted EOF path reports through inherited fd1 with explicit
  `Talos userspace stdin fixture read-result: terminal-eof` and
  `return=0x0000000000000000`.
- fixed: retained evidence maps stdout/stderr, descriptor-backed VFS exec,
  loader temporary descriptor non-leak, lifecycle/status, consuming
  `waitpid`, non-consuming `laststatus`, fixed `/bin` lookup, negative
  exec controls, and descriptor-backed `cat /etc/banner.txt`.
- not-an-issue: ordinary no available runtime-console0 input remains
  readiness/no-data with `-EAGAIN` and `timeout/no-false-eof`, not terminal
  EOF.
- not-an-issue: scheduler-backed delayed input remains the accepted waiting
  path for later bytes and is not reclassified as EOF.
- deferred: full termios/canonical mode, POSIX signals, sessions, job control,
  select/poll, nonblocking flags, pipes, redirection, async execution, fork,
  writable filesystem behavior, libc stdio, distinct physical stderr routing,
  Pi 5 proof, networking, and SSH remain out of scope.
- not-an-issue: this closeout made no code changes and did not acquire
  `hardwareTestLock`.

## Evidence Map

- terminal Ctrl-D EOF:
  `tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log`.
  Static inspection shows `exec stdin` followed by Ctrl-D, visible
  `Talos userspace stdin fixture read-result: terminal-eof`,
  `exec-stdin ... return=0x0000000000000000 ...
  read-result=terminal-eof`, consuming `waitpid`, non-consuming
  `laststatus`, stdout, VFS exec/status, negative exec controls,
  descriptor-backed `cat /etc/banner.txt`, final classification, and PASS.
- scheduler-backed delayed input control:
  `tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log`.
  Static inspection shows `talos: stdin-wait ... result=sleep`,
  `talos: stdin-wait ... result=wakeup/resume`, delayed
  `Talos userspace stdin fixture read: talos-console0`, and
  `read-result=scheduler-wait/delayed-input`.
- no-data/readiness control:
  `tasks/evidence/2026-06-04-phase10-runtime-stdin-readiness-distinction-core/qemu-local-shell-runtime-stdin-readiness-smoke.log`.
  Static inspection shows no available input returns `-EAGAIN`, records
  `read-result=readiness/no-data`, and records
  `scheduler-wait-result=timeout/no-false-eof`.
- immediate runtime-console0 stdin control:
  `tasks/evidence/2026-06-03-phase10-runtime-console0-stdin-core/qemu-local-shell-runtime-console0-stdin-smoke.log`.
  Static inspection from the accepted prior closeout keeps immediate
  `talos-console0` byte consumption through inherited fd0 separate from EOF
  and delayed-input behavior.
- stdout/stderr and process controls:
  `tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log`
  plus the Ctrl-D EOF smoke retain userspace stdout/stderr,
  descriptor-backed VFS exec, loader temporary descriptor non-leak,
  lifecycle/status, consuming `waitpid`, non-consuming `laststatus`, fixed
  `/bin` lookup, negative exec controls, and descriptor-backed
  `cat /etc/banner.txt`.

## Accepted Frontier

Accepted:

- Ctrl-D 0x04 as the first available runtime-console0/local-input byte on
  inherited fd0 is true terminal EOF for VFS-backed `/bin/stdin`;
- terminal EOF returns 0 from the fd0 read path and is reported through
  inherited fd1 with `read-result=terminal-eof`;
- ordinary no-data/readiness remains `-EAGAIN`, not EOF;
- delayed runtime-console0 bytes remain
  `read-result=scheduler-wait/delayed-input` with scheduler sleep and
  wake/resume markers;
- retained descriptor-backed VFS/open/read exec lineage, standard descriptor
  inheritance, loader temporary descriptor non-leak, lifecycle/status,
  `waitpid`, `laststatus`, stdout/stderr, negative exec controls, and
  descriptor-backed cat regressions.

Deferred:

- full termios/canonical mode beyond the accepted first-byte Ctrl-D EOF;
- POSIX signals, sessions, and job control;
- select/poll and nonblocking descriptor flags;
- pipes, redirection, async execution, fork, writable filesystem behavior, and
  libc stdio;
- distinct physical stderr routing;
- Pi 5 proof, networking, and SSH.

## Recommendation

Distinct stderr routing is the next smallest feature-led local I/O task. It is
already queued, stays inside the same Phase 10 shell I/O milestone, and follows
directly from the accepted stdout/stderr inherited descriptor controls: fd2
currently shares the accepted `stdio-output` backend with fd1, so the next
bounded step is to prove process-originated fd2 output can be routed and
identified distinctly without adding pipes, redirection, or broader terminal
policy.

## Validation Summary

- static inspection: accepted task docs and retained evidence logs inspected.
- diff hygiene: `git diff --check` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff hygiene: `git diff --cached --check` passed before commit.

hardwareTestLock remained unlocked/restored and unused.
