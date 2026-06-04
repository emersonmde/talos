# Phase 10 Runtime Stdin Readiness Distinction Closeout

Task: phase10-runtime-stdin-readiness-distinction-closeout-20260604

Status: accepted

## Summary

Closed the runtime-console0 stdin readiness distinction checkpoint. The
accepted frontier is now: a shell-visible VFS-backed `/bin/stdin` process
inherits `fd0=stdio-input`, reads runtime-console0/local-input through the
userspace `TalosRead` path, reports available bytes through inherited fd1, and
reports ordinary no-data as readiness/no-data with `-EAGAIN`. Ordinary
runtime-console0/local-input no-data is not accepted as true EOF.

The evidence keeps the successful `talos-console0` read and the no-data
readiness branch distinct. The next queued bounded runtime stdin wait task is
mechanically unblocked by this closeout, but it remains a local bounded feature
step below scheduler-backed blocking reads and POSIX readiness APIs.

## Findings And Disposition

- fixed: reconciled the accepted readiness/no-data implementation with the
  previous EOF/no-data wording so closeout language no longer treats ordinary
  no-data as terminal EOF.
- fixed: recorded the accepted evidence map for readiness/no-data stdin,
  successful runtime-console0 stdin, stdout/stderr, VFS exec, lifecycle/status,
  waitpid, laststatus, negative exec controls, and descriptor-backed cat.
- fixed: updated the roadmap current-status sequence to include this closeout
  and to name the bounded runtime stdin wait core as mechanically unblocked.
- not-an-issue: fixed proof-buffer stdin retains bounded EOF behavior as older
  Phase 7 proof evidence; this closeout is scoped only to runtime-console0/local
  input used by shell-visible VFS-backed exec stdin.
- deferred: true terminal EOF policy, Ctrl-D EOF, scheduler-backed blocking
  reads, wait queues, select/poll, nonblocking flags, async execution, fork,
  signals, pipes, redirection, distinct stderr stream routing, writable
  filesystem behavior, libc stdio, Pi 5 proof, networking, and SSH remain out
  of scope.

## Evidence Map

- readiness/no-data stdin:
  `tasks/evidence/2026-06-04-phase10-runtime-stdin-readiness-distinction-core/qemu-local-shell-runtime-stdin-readiness-smoke.log`.
  Static inspection found the process-originated report
  `Talos userspace stdin fixture no-data: readiness`,
  inherited `fd0=stdio-input`, `fd1=stdio-output`, `fd2=stdio-output`,
  `loader-temp-open=false`, `return=0xfffffffffffffff5`,
  `read-source=runtime-console0/local-input`,
  `source=userspace-talos-read+userspace-talos-write`,
  `read-result=readiness/no-data`, matching waitpid/laststatus coverage,
  final `participants=18 expected=18 errors=0`, classification
  `qemu-local-shell-userspace-stdin-complete`, and PASS.
- successful runtime-console0 stdin regression:
  `tasks/evidence/2026-06-03-phase10-runtime-console0-stdin-core/qemu-local-shell-runtime-console0-stdin-smoke.log`.
  Static inspection found `Talos userspace stdin fixture read: talos-console0`
  and `exec-stdin ... bytes=0x000000000000000e return=0x000000000000000e
  read-source=runtime-console0/local-input`, with the same inherited standard
  descriptor lineage and PASS.
- stdout/stderr regressions:
  `tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log`
  and
  `tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log`.
  The retained logs show process-originated stdout/stderr fixture output,
  inherited standard descriptors, lifecycle/status, waitpid, laststatus,
  negative exec controls, descriptor-backed cat, final classifications, and
  PASS.
- VFS exec, fixed `/bin` lookup, lifecycle/status, waitpid, laststatus,
  negative exec controls, and descriptor-backed cat:
  `tasks/evidence/2026-06-03-phase10-minimal-path-lookup-exec-core/qemu-local-shell-path-lookup-smoke.log`
  and
  `tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log`.
  Static inspection found descriptor-backed `source=vfs-open-read` exec
  records, `waitpid` consuming lifecycle records, `laststatus` retaining the
  latest lifecycle status, deterministic negative exec controls, descriptor-
  backed `cat /etc/banner.txt`, final classifications, and PASS.

## Accepted Frontier

Accepted:

- runtime-console0/local-input no-data is a readiness/no-data result for
  shell-visible VFS-backed exec stdin, surfaced as `-EAGAIN` and reported
  through inherited fd1.
- available runtime-console0/local-input bytes are consumed by the launched
  userspace fixture through inherited fd0 and reported through inherited fd1.
- descriptor-backed VFS/open/read exec lineage, standard descriptor inheritance,
  loader temporary descriptor non-leak, lifecycle/status, consuming waitpid,
  non-consuming laststatus, stdout/stderr, fixed `/bin` lookup, negative exec
  controls, and descriptor-backed cat remain retained regressions.

Deferred:

- true terminal EOF for runtime-console0/local-input, Ctrl-D EOF policy,
  scheduler-backed blocking reads, wait queues, select/poll, nonblocking flag
  APIs, async execution, fork, signals, pipes, redirection, distinct stderr
  routing, writable filesystem behavior, libc stdio, Pi 5 proof, networking,
  and SSH.

## Next Action

The queued `phase10-bounded-runtime-stdin-wait-core-20260604` task is
mechanically unblocked for the next worker wake. It should remain bounded to
delayed runtime-console0/local-input bytes and a no-bytes-within-budget
readiness/no-data regression; it must not be documented as full POSIX blocking
read semantics.

## Validation Summary

- static inspection: reviewed the accepted task record and retained evidence
  logs listed above.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

hardwareTestLock remained unlocked/restored and unused.
