# Phase 10 Stderr-To-Stdout FD Dup Redirection Closeout

Task: phase10-stderr-to-stdout-fd-dup-redirection-closeout-20260604

Status: accepted

## Summary

This closeout checkpoints the accepted inverse descriptor-duplication
redirection boundary before any broader descriptor redirection, regular-file
redirection, pipes, or writable filesystem behavior. The accepted feature is
intentionally narrow: a VFS-backed `/bin/stderr` launch may duplicate the
inherited fd1 descriptor target into child fd2 for the exact shell-visible
`exec stderr 2>&1` form, while the shell descriptor table is restored after
the child launch.

The accepted physical sink remains shared runtime-console0. The accepted
distinction is descriptor target and route metadata:

- redirected child fd2: `stream=stdout route=runtime-console0/stdout`
- retained child fd1-to-fd2 redirection: `stream=stderr route=runtime-console0/stderr`
- normal child fd1 control: `stream=stdout route=runtime-console0/stdout`
- normal child fd2 control: `stream=stderr route=runtime-console0/stderr`

## Findings And Disposition

- fixed: Closed out the accepted `2>&1` descriptor-duplication boundary with
  retained evidence for child-only fd2 rebinding and shell descriptor
  restoration.
- fixed: Preserved the accepted forward-direction `1>&2` regression proving
  fd1 can still be rebound to the inherited fd2 stderr route for a launched
  VFS-backed child.
- fixed: Preserved normal stdout and stderr route controls proving later
  non-redirected execs restore fd1 as `runtime-console0/stdout` and fd2 as
  `runtime-console0/stderr`.
- fixed: Preserved stdin scheduler wait/readiness and Ctrl-D EOF controls so
  descriptor redirection did not narrow the accepted local-input behavior.
- fixed: Preserved descriptor-backed VFS exec, lifecycle/status, consuming
  `waitpid`, non-consuming `laststatus`, deterministic negative
  exec/redirection controls, loader temporary descriptor non-leak, and
  descriptor-backed `cat /etc/banner.txt` evidence.
- not-an-issue: fd1 and fd2 still share the runtime-console0 physical output
  device. This closeout accepts child descriptor-table behavior and stream/route
  metadata, not separate hardware, terminal, or file sinks.
- deferred: arbitrary `N>&M`, descriptor close/move syntax, regular-file
  redirection, append/truncate, pipes, here-docs, shell variables, quoting
  expansion, writable filesystem behavior, full POSIX fork/signals/job control,
  libc stdio, Pi 5 proof, networking, and SSH.

## Evidence Map

- stderr-to-stdout descriptor-dup redirection:
  `tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log`.
  QEMU/substitute evidence shows `exec stderr 2>&1`,
  descriptor-backed `/bin/stderr` VFS/open/read lineage,
  `exec-redirection op=dup source-fd=2 target-fd=1
  target-stream=stdout target-route=runtime-console0/stdout
  child-only=true shell-restored=true`, userspace fd2 write metadata
  `stream=stdout route=runtime-console0/stdout`, lifecycle/status,
  `waitpid`, `laststatus`, the retained forward-direction `1>&2`
  regression, unsupported file-redirection negative, descriptor-backed
  `cat /etc/banner.txt`, final classification
  `qemu-local-shell-stderr-to-stdout-redirection-complete`, and PASS.
- stdout-to-stderr descriptor-dup regression:
  `tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log`.
  QEMU/substitute evidence retains `exec stdout 1>&2`, child-only fd1
  rebinding to the inherited fd2 target, userspace fd1 write metadata
  `stream=stderr route=runtime-console0/stderr`, deterministic unsupported
  redirection negatives, final classification
  `qemu-local-shell-stdout-to-stderr-redirection-complete`, and PASS.
- distinct stderr route control:
  `tasks/evidence/2026-06-04-phase10-distinct-stderr-routing-core/qemu-local-shell-distinct-stderr-routing-smoke.log`.
  QEMU/substitute evidence retains inherited fd2 writes as
  `stream=stderr route=runtime-console0/stderr`, with stdout route control,
  scheduler-backed stdin wait/readiness and Ctrl-D EOF regressions,
  lifecycle/status, `waitpid`, `laststatus`, negative exec controls,
  descriptor-backed `cat /etc/banner.txt`, final classification
  `qemu-local-shell-distinct-stderr-routing-complete`, and PASS.
- normal stdout route control:
  `tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log`.
  QEMU/substitute evidence retains inherited fd1 writes as
  `stream=stdout route=runtime-console0/stdout`.
- normal stderr route control:
  `tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log`.
  QEMU/substitute evidence retains inherited fd2 writes as
  `stream=stderr route=runtime-console0/stderr`.
- scheduler-backed stdin wait/readiness control:
  `tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log`.
  QEMU/substitute evidence retains scheduler sleep and wake/resume markers,
  delayed runtime-console0 bytes, and
  `read-result=scheduler-wait/delayed-input`.
- terminal EOF control:
  `tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log`.
  QEMU/substitute evidence retains first-byte Ctrl-D true EOF with
  `return=0` and `read-result=terminal-eof`.
- VFS/process controls:
  The redirection, distinct-stderr, and stdio smokes retain descriptor-backed
  VFS exec lineage, loader temporary descriptor non-leak, zero/nonzero
  lifecycle/status observations, consuming `waitpid`, non-consuming
  `laststatus`, deterministic negative exec/redirection controls, and
  descriptor-backed `cat /etc/banner.txt`.

## Accepted Frontier

Accepted:

- `exec stderr 2>&1` is the inverse exact descriptor-duplication redirection
  form.
- Redirection is child-only and affects the launched VFS-backed executable's
  descriptor table, not the shell descriptor table after the command returns.
- `/bin/stderr` remains loaded through descriptor-backed VFS/open/read and
  writes via fd2, but fd2 resolves to the inherited fd1 stdout route for the
  redirected child.
- `exec stdout 1>&2` remains accepted and covered as the forward-direction
  descriptor-dup regression.
- Normal `exec stdout` and `exec stderr` controls retain stdout/stderr
  route metadata.
- Unsupported regular-file redirection, pipes, and bad descriptor forms fail
  deterministically without shrinking accepted VFS exec behavior.

Deferred:

- arbitrary `N>&M`, descriptor close/move syntax, regular-file redirection,
  append/truncate, and pipes;
- writable filesystem behavior, separate physical sinks, full terminal policy,
  async jobs, fork, signals, libc stdio, Pi 5 proof, networking, and SSH.

## Next Step Recommendation

Both exact descriptor-duplication directions are now accepted and retained:
`exec stdout 1>&2` and `exec stderr 2>&1`. The queued descriptor-dup
redirection frontier closeout is mechanically unblocked because both direction
closeouts and their evidence maps are accepted. That next closeout must remain
bounded to the descriptor-dup frontier and must not implement or claim regular
file redirection, pipes, writable filesystem behavior, networking, SSH, or a
phase transition.

## Validation Summary

- static inspection: accepted core task records and retained evidence logs were
  inspected.
- diff check: `git diff --check` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed with the existing large
  search-index warning.
- staged diff check: `git diff --cached --check` passed before commit.

hardwareTestLock remained unlocked/restored and unused.
