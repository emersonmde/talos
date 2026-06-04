# Phase 10 Stdout-To-Stderr FD Dup Redirection Closeout

Task: phase10-stdout-to-stderr-fd-dup-redirection-closeout-20260604

Status: accepted

## Summary

This closeout checkpoints the accepted 'exec stdout 1>&2' boundary before any
inverse descriptor-duplication, broader redirection, pipes, or writable
filesystem behavior. The accepted feature is intentionally narrow: a
VFS-backed '/bin/stdout' launch may duplicate the inherited fd2 descriptor
target into child fd1, while the shell descriptor table is restored after the
child launch.

The accepted physical sink remains shared runtime-console0. The accepted
distinction is descriptor target and route metadata:

- redirected child fd1: 'stream=stderr route=runtime-console0/stderr'
- normal child fd1 control: 'stream=stdout route=runtime-console0/stdout'
- normal child fd2 control: 'stream=stderr route=runtime-console0/stderr'

## Findings And Disposition

- fixed: Closed out the accepted '1>&2' descriptor-duplication boundary with
  retained evidence for child-only fd1 rebinding and shell descriptor
  restoration.
- fixed: Preserved the normal stdout route control proving a later
  non-redirected 'exec stdout' still writes through fd1 to
  'runtime-console0/stdout'.
- fixed: Preserved the distinct stderr route control proving fd2 stderr writes
  remain 'runtime-console0/stderr'.
- fixed: Preserved stdin scheduler wait/readiness and Ctrl-D EOF controls so
  redirection did not narrow the accepted local-input behavior.
- fixed: Preserved VFS exec, lifecycle/status, consuming 'waitpid',
  non-consuming 'laststatus', deterministic negative exec/redirection
  controls, loader temporary descriptor non-leak, and descriptor-backed
  'cat /etc/banner.txt' evidence.
- not-an-issue: fd1 and fd2 still share the runtime-console0 physical output
  device. This closeout accepts route metadata and descriptor-table behavior,
  not separate hardware or file sinks.
- deferred: inverse '2>&1', descriptor close/move syntax, regular-file
  redirection, append/truncate, pipes, here-docs, shell variables, quoting
  expansion, writable filesystem behavior, full POSIX fork/signals/job
  control, libc stdio, Pi 5 proof, networking, and SSH.

## Evidence Map

- stdout-to-stderr descriptor-dup redirection:
  'tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log'.
  QEMU/substitute evidence shows 'exec stdout 1>&2',
  descriptor-backed '/bin/stdout' VFS/open/read lineage,
  'exec-redirection op=dup source-fd=1 target-fd=2
  target-stream=stderr target-route=runtime-console0/stderr
  child-only=true shell-restored=true', userspace fd1 write metadata
  'stream=stderr route=runtime-console0/stderr', lifecycle/status,
  'waitpid', 'laststatus', unsupported inverse/file/pipe redirection
  negatives, descriptor-backed 'cat /etc/banner.txt', final classification
  'qemu-local-shell-stdout-to-stderr-redirection-complete', and PASS.
- normal stdout route control:
  'tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log'.
  QEMU/substitute evidence retains inherited fd1 writes as
  'stream=stdout route=runtime-console0/stdout', plus lifecycle/status,
  'waitpid', 'laststatus', negative exec controls, descriptor-backed
  'cat /etc/banner.txt', final classification
  'qemu-local-shell-userspace-stdout-complete', and PASS.
- distinct stderr route control:
  'tasks/evidence/2026-06-04-phase10-distinct-stderr-routing-core/qemu-local-shell-distinct-stderr-routing-smoke.log'.
  QEMU/substitute evidence retains inherited fd2 writes as
  'stream=stderr route=runtime-console0/stderr', with stdout route control,
  scheduler-backed stdin wait/readiness and Ctrl-D EOF regressions,
  lifecycle/status, 'waitpid', 'laststatus', negative exec controls,
  descriptor-backed 'cat /etc/banner.txt', final classification
  'qemu-local-shell-distinct-stderr-routing-complete', and PASS.
- scheduler-backed stdin wait/readiness control:
  'tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log'.
  QEMU/substitute evidence retains scheduler sleep and wake/resume markers,
  delayed runtime-console0 bytes, and
  'read-result=scheduler-wait/delayed-input'.
- terminal EOF control:
  'tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log'.
  QEMU/substitute evidence retains first-byte Ctrl-D true EOF with
  'return=0' and 'read-result=terminal-eof'.
- VFS/process controls:
  The redirection and stderr-routing smokes retain descriptor-backed VFS exec
  lineage, loader temporary descriptor non-leak, zero/nonzero
  lifecycle/status observations, consuming 'waitpid', non-consuming
  'laststatus', deterministic negative exec/redirection controls, and
  descriptor-backed 'cat /etc/banner.txt'.

## Accepted Frontier

Accepted:

- 'exec stdout 1>&2' is the first exact descriptor-duplication redirection
  form.
- Redirection is child-only and affects the launched VFS-backed executable's
  descriptor table, not the shell descriptor table after the command returns.
- '/bin/stdout' remains loaded through descriptor-backed VFS/open/read and
  writes via fd1, but fd1 resolves to the inherited fd2 stderr route for the
  redirected child.
- Normal 'exec stdout' and 'exec stderr' controls retain stdout/stderr route
  metadata.
- Unsupported inverse, file, and pipe redirection forms fail deterministically
  without shrinking accepted VFS exec behavior.

Deferred:

- inverse '2>&1';
- descriptor close/move syntax, regular-file redirection, append/truncate,
  and pipes;
- writable filesystem behavior, separate physical sinks, full terminal
  policy, async jobs, fork, signals, libc stdio, Pi 5 proof, networking, and
  SSH.

## Next Step Recommendation

The inverse descriptor-duplication slice, 'exec stderr 2>&1', is mechanically
unblocked by this closeout because the forward direction, normal stdout route,
normal stderr route, and child-only descriptor restoration evidence are
accepted and retained. It should remain bounded to descriptor duplication and
must not claim file redirection, pipes, descriptor close/move syntax, or
writable filesystem behavior.

## Validation Summary

- static inspection: accepted core task record and retained evidence logs were
  inspected.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed with the existing large
  search-index warning.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.
