# Phase 10 Stderr Close Redirection Closeout

Task: phase10-stderr-close-redirection-closeout-20260604

Status: accepted

## Summary

This closeout checkpoints the accepted stderr descriptor-close redirection
frontier before any broader descriptor-close syntax or shell I/O work. The
accepted feature is limited to the two exact shell-visible standard-stream
close forms for VFS-backed exec children:

- 'exec stdout 1>&-': child fd1 is closed before '/bin/stdout' attempts its
  userspace 'TalosWrite'.
- 'exec stderr 2>&-': child fd2 is closed before '/bin/stderr' attempts its
  userspace 'TalosWrite'.

Both forms are child-only. The shell descriptor table is restored after each
launched child exits; retained normal 'exec stdout' and 'exec stderr' controls
prove fd1 and fd2 return to their runtime-console0 routes.

The accepted physical sink remains shared runtime-console0. This frontier
accepts descriptor-table close behavior and closed-descriptor route metadata,
not separate physical sinks, arbitrary descriptor syntax, file redirection,
pipes, or writable filesystem behavior.

## Findings And Disposition

- fixed: Reconciled the accepted '2>&-' stderr descriptor-close record into
  the same durable descriptor-policy boundary as the prior '1>&-' stdout
  close record.
- fixed: Preserved child-only semantics and shell fd2 restoration as the
  defining acceptance rule for stderr descriptor close.
- fixed: Preserved normal stdout and stderr route restoration controls,
  proving later non-redirected children still report 'stream=stdout
  route=runtime-console0/stdout' and 'stream=stderr
  route=runtime-console0/stderr'.
- fixed: Preserved both accepted descriptor-close directions, both accepted
  descriptor-dup directions, normal stdout and stderr routes, stdin
  scheduler/readiness/EOF controls, descriptor-backed VFS exec,
  lifecycle/status, consuming 'waitpid', non-consuming 'laststatus',
  deterministic negative exec/redirection controls, loader temporary
  descriptor non-leak, and descriptor-backed 'cat /etc/banner.txt' evidence.
- not-an-issue: The '-EBADF' result is reported by the userspace stderr
  fixture's fd2 'TalosWrite' after descriptor-backed VFS loading succeeds; it
  is not a loader, launch, lifecycle, or status failure.
- not-an-issue: fd1 and fd2 still share the runtime-console0 physical output
  device. That is consistent with the accepted route-metadata boundary.
- deferred: arbitrary 'N>&-' descriptor close syntax, descriptor moves,
  regular-file redirection, append/truncate, pipes, here-docs, shell
  variables, quoting expansion, writable filesystem behavior, separate
  physical stdout/stderr sinks, full terminal policy, async jobs, fork,
  signals, libc stdio, Pi 5 proof, networking, and SSH.

## Evidence Map

- stderr descriptor-close redirection:
  'tasks/evidence/2026-06-04-phase10-stderr-close-redirection-core/qemu-local-shell-stderr-close-redirection-smoke.log'.
  Static inspection of QEMU/substitute evidence shows 'exec stderr 2>&-',
  descriptor-backed '/bin/stderr' VFS/open/read lineage,
  'exec-descriptors ... inherited-count=2 fd2=closed',
  'exec-redirection op=close source-fd=2 result=closed-descriptor
  child-only=true shell-restored=true', userspace fd2 write return '-EBADF'
  as 'stream=closed route=closed-descriptor', lifecycle/status, 'waitpid',
  'laststatus', normal stderr restoration, retained stdout close,
  deterministic unsupported redirection negatives, descriptor-backed
  'cat /etc/banner.txt', final classification
  'qemu-local-shell-stderr-close-redirection-complete', and PASS.
- stdout descriptor-close regression:
  'tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log'.
  QEMU/substitute evidence retains 'exec stdout 1>&-', closed child fd1,
  shell fd1 restoration, fd1 write return '-EBADF' as
  'stream=closed route=closed-descriptor', descriptor-backed VFS loading,
  lifecycle/status, descriptor-backed 'cat /etc/banner.txt', and PASS.
- normal stderr restoration control:
  'tasks/evidence/2026-06-04-phase10-stderr-close-redirection-core/qemu-local-shell-stderr-close-redirection-smoke.log'
  and
  'tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log'.
  QEMU/substitute evidence retains fd2 writes as 'stream=stderr
  route=runtime-console0/stderr'.
- normal stdout restoration control:
  'tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log'
  and
  'tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log'.
  QEMU/substitute evidence retains fd1 writes as 'stream=stdout
  route=runtime-console0/stdout'.
- stdout-to-stderr descriptor-dup control:
  'tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log'.
  QEMU/substitute evidence retains 'exec stdout 1>&2', child-only fd1 rebinding
  to fd2, shell restoration, lifecycle/status, 'waitpid', 'laststatus',
  negative redirection controls, descriptor-backed 'cat /etc/banner.txt', and
  PASS.
- stderr-to-stdout descriptor-dup control:
  'tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log'.
  QEMU/substitute evidence retains 'exec stderr 2>&1', child-only fd2 rebinding
  to fd1, shell restoration, the forward '1>&2' regression, lifecycle/status,
  'waitpid', 'laststatus', descriptor-backed 'cat /etc/banner.txt', and PASS.
- distinct stderr route control:
  'tasks/evidence/2026-06-04-phase10-distinct-stderr-routing-core/qemu-local-shell-distinct-stderr-routing-smoke.log'.
  QEMU/substitute evidence retains fd2 as 'stream=stderr
  route=runtime-console0/stderr' while preserving stdout route control.
- scheduler-backed stdin wait/readiness control:
  'tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log'.
  QEMU/substitute evidence retains scheduler wait/sleep and wake/resume markers
  before delayed runtime-console0 bytes are consumed through inherited fd0.
- terminal EOF control:
  'tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log'.
  QEMU/substitute evidence retains first-byte Ctrl-D true EOF with 'return=0'
  and 'read-result=terminal-eof'.
- descriptor-backed cat control:
  'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log'.
  QEMU/substitute evidence retains 'cat /etc/banner.txt' through the
  descriptor-backed initramfs open/read surface with final classification
  'qemu-local-cat-banner-complete' and PASS.

## Accepted Frontier

Accepted:

- 'exec stdout 1>&-' and 'exec stderr 2>&-' are the only accepted exact
  descriptor-close redirection forms.
- Redirection affects only the launched VFS-backed executable's child
  descriptor table.
- '/bin/stdout' and '/bin/stderr' continue to load through descriptor-backed
  VFS/open/read before their userspace fixtures attempt fd1/fd2 writes.
- The accepted observable close result is the fixture write '-EBADF' with
  'stream=closed route=closed-descriptor'.
- The shell descriptor table is restored after the redirected child launch.
- Normal stdout/stderr, both descriptor-dup directions, stdin
  wait/readiness/EOF, VFS exec, lifecycle/status, 'waitpid', 'laststatus',
  negative controls, and descriptor-backed cat behavior remain covered by
  retained evidence.

Deferred:

- arbitrary 'N>&-' descriptor close syntax and descriptor moves;
- regular-file redirection, append/truncate, here-docs, and pipes;
- writable filesystem behavior, separate physical sinks, full terminal policy,
  async jobs, fork, signals, libc stdio, Pi 5 proof, networking, and SSH.

## Next Step Requirement

The queued 'phase10-fd-close-redirection-frontier-closeout-20260604' task is
mechanically unblocked after this task is accepted and committed. It must
remain a docs/evidence reconciliation only. It may recommend the next
feature-led shell I/O task or require supervisor planning, but must not
implement arbitrary descriptor close/move syntax, file redirection, pipes,
writable filesystem behavior, networking, SSH, or a phase transition.

## Validation Summary

- static inspection: accepted task records and retained evidence logs were
  inspected.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.
