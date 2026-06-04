# Phase 10 Bounded Runtime Stdin Wait Closeout

Task: phase10-bounded-runtime-stdin-wait-closeout-20260604

Status: accepted

## Summary

Closed the bounded runtime-console0 stdin wait checkpoint. The accepted
frontier is now: shell-visible VFS-backed `exec stdin` launches `/bin/stdin`,
inherits `fd0=stdio-input`, observes ordinary no-data as readiness/no-data,
and may retry within a task-local finite budget. Delayed `talos-console0`
bytes arriving within that budget are consumed through inherited fd0 and
reported through inherited fd1. No bytes within the budget remains
`-EAGAIN`/readiness-no-data and is not true EOF.

This is deliberately below full POSIX blocking read semantics. It proves a
bounded local read-wait feature without accepting scheduler wait queues,
process sleep/wakeup policy, `select`/`poll`, nonblocking descriptor flags,
or terminal EOF/Ctrl-D behavior.

## Findings And Disposition

- fixed: reconciled the accepted delayed-byte bounded wait behavior with the
  no-bytes-within-budget readiness/no-data regression.
- fixed: recorded the retained evidence map for immediate runtime-console0
  stdin, stdout/stderr, descriptor-backed VFS exec, lifecycle/status, waitpid,
  laststatus, negative exec controls, and descriptor-backed cat.
- fixed: updated the roadmap current-status sequence with this closeout and
  the bounded feature frontier.
- not-an-issue: the bounded retry loop remains task-local fixture behavior for
  `/bin/stdin`; it does not need scheduler wait queues to satisfy this task.
- deferred: scheduler-backed blocking reads/readiness, wait queues,
  `select`/`poll`, nonblocking descriptor flags, Ctrl-D EOF policy,
  canonical terminal expansion, async execution, fork, signals, pipes,
  redirection, distinct stderr stream routing, writable filesystem behavior,
  libc stdio, Pi 5 proof, networking, and SSH remain out of scope.

## Evidence Map

- delayed runtime stdin bounded wait:
  `tasks/evidence/2026-06-04-phase10-bounded-runtime-stdin-wait-core/qemu-local-shell-bounded-runtime-stdin-wait-smoke.log`.
  Static inspection found `Talos userspace stdin fixture no-data: readiness`
  before `Talos userspace stdin fixture read: talos-console0`, inherited
  `fd0=stdio-input`, `fd1=stdio-output`, `fd2=stdio-output`,
  `loader-temp-open=false`, `return=0x000000000000000e`,
  `read-source=runtime-console0/local-input`,
  `source=userspace-talos-read+userspace-talos-write`,
  `read-result=bounded-wait/delayed-input`,
  `readiness-observations=0x0000000000000234`, matching waitpid/laststatus
  coverage, final `participants=18 expected=18 errors=0`, classification
  `qemu-local-shell-userspace-stdin-complete`, and PASS.
- no-bytes-within-budget readiness/no-data regression:
  `tasks/evidence/2026-06-04-phase10-bounded-runtime-stdin-wait-core/qemu-local-shell-bounded-runtime-stdin-no-data-smoke.log`.
  Static inspection found `Talos userspace stdin fixture no-data: readiness`,
  `return=0xfffffffffffffff5`, `read-result=readiness/no-data`,
  `readiness-observations=0x00000000000f4240`, no delayed input read, no
  hang, final `participants=18 expected=18 errors=0`, classification
  `qemu-local-shell-userspace-stdin-complete`, and PASS.
- immediate runtime-console0 stdin regression:
  `tasks/evidence/2026-06-03-phase10-runtime-console0-stdin-core/qemu-local-shell-runtime-console0-stdin-smoke.log`.
  Static inspection found `Talos userspace stdin fixture read: talos-console0`
  and `exec-stdin ... bytes=0x000000000000000e
  return=0x000000000000000e read-source=runtime-console0/local-input`
  without the bounded-wait result field, with the same inherited descriptor
  lineage and PASS.
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
  `tasks/evidence/2026-06-03-phase10-minimal-path-lookup-exec-core/qemu-local-shell-path-lookup-smoke.log`,
  `tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log`,
  and
  `tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log`.
  Static inspection found descriptor-backed `source=vfs-open-read` exec
  records, lifecycle/status records, consuming `waitpid`, non-consuming
  `laststatus`, deterministic negative exec controls, descriptor-backed
  `cat /etc/banner.txt`, final classifications, and PASS.

## Accepted Frontier

Accepted:

- `/bin/stdin` may perform a bounded task-local retry after an initial
  runtime-console0/local-input readiness/no-data observation.
- delayed runtime-console0/local-input bytes arriving within that bounded
  budget are consumed through inherited fd0 and reported through inherited fd1.
- no bytes within the bounded budget remains readiness/no-data (`-EAGAIN`),
  not true terminal EOF.
- descriptor-backed VFS/open/read exec lineage, standard descriptor
  inheritance, loader temporary descriptor non-leak, lifecycle/status,
  consuming waitpid, non-consuming laststatus, stdout/stderr, fixed `/bin`
  lookup, negative exec controls, and descriptor-backed cat remain retained
  regressions.

Deferred:

- scheduler-backed blocking reads/readiness, wait queues, `select`/`poll`,
  nonblocking descriptor flags, terminal EOF/Ctrl-D policy, canonical terminal
  expansion, async execution, fork, signals, pipes, redirection, distinct
  stderr routing, writable filesystem behavior, libc stdio, Pi 5 proof,
  networking, and SSH.

## Next Action

No explicit queued follow-up task remains. Supervisor planning is required
before the next feature task. The most direct feature-led candidate is
scheduler-backed blocking/readiness for stdin, because it would replace the
accepted bounded retry with process wait/wakeup semantics. Ctrl-D EOF and
distinct stderr stream routing remain plausible later local I/O tasks, but they
should not be promoted by the worker without an explicit supervisor task.

## Validation Summary

- static inspection: reviewed the accepted task record and retained evidence
  logs listed above.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

hardwareTestLock remained unlocked/restored and unused.
