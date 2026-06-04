# Phase 10 Stdout Close Redirection Closeout

Task: phase10-stdout-close-redirection-closeout-20260604

Status: accepted

## Summary

This closeout checkpoints the accepted stdout descriptor-close redirection
frontier before any inverse stderr close syntax or broader shell I/O work. The
accepted feature is limited to one exact shell-visible form for VFS-backed exec
children:

- 'exec stdout 1>&-': child fd1 is closed before '/bin/stdout' attempts its
  userspace 'TalosWrite'.

The form is child-only. The shell descriptor table is restored after the
launched child exits, and the retained normal 'exec stdout' control proves fd1
returns to 'stream=stdout route=runtime-console0/stdout'.

The accepted physical sink remains shared runtime-console0. This frontier
accepts descriptor-table close behavior and closed-descriptor route metadata,
not separate physical sinks, file redirection, pipes, or writable filesystem
behavior.

## Findings And Disposition

- fixed: Reconciled the accepted '1>&-' stdout descriptor-close record into a
  durable descriptor-policy boundary.
- fixed: Preserved child-only semantics and shell fd1 restoration as the
  defining acceptance rule for stdout descriptor close.
- fixed: Preserved the normal stdout route restoration control proving later
  non-redirected children still report 'stream=stdout
  route=runtime-console0/stdout'.
- fixed: Preserved both accepted descriptor-dup directions, normal stdout and
  stderr routes, stdin scheduler/readiness/EOF controls, descriptor-backed VFS
  exec, lifecycle/status, consuming 'waitpid', non-consuming 'laststatus',
  deterministic negative exec/redirection controls, loader temporary descriptor
  non-leak, and descriptor-backed 'cat /etc/banner.txt' evidence.
- not-an-issue: The '-EBADF' result is reported by the userspace stdout
  fixture's fd1 'TalosWrite' after descriptor-backed VFS loading succeeds; it
  is not a loader, launch, lifecycle, or status failure.
- not-an-issue: fd1 and fd2 still share the runtime-console0 physical output
  device. That is consistent with the accepted route-metadata boundary.
- deferred: stderr '2>&-', arbitrary 'N>&-', descriptor moves, regular-file
  redirection, append/truncate, pipes, here-docs, shell variables, quoting
  expansion, writable filesystem behavior, separate physical stdout/stderr
  sinks, terminal policy, async jobs, fork, signals, libc stdio, Pi 5 proof,
  networking, and SSH.

## Evidence Map

- stdout descriptor-close redirection:
  'tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log'.
  Static inspection of QEMU/substitute evidence shows 'exec stdout 1>&-',
  descriptor-backed '/bin/stdout' VFS/open/read lineage,
  'exec-descriptors ... inherited-count=2 fd1=closed',
  'exec-redirection op=close source-fd=1 result=closed-descriptor
  child-only=true shell-restored=true', userspace fd1 write return '-EBADF'
  as 'stream=closed route=closed-descriptor', lifecycle/status, 'waitpid',
  'laststatus', deterministic unsupported redirection negatives,
  descriptor-backed 'cat /etc/banner.txt', final classification
  'qemu-local-shell-stdout-close-redirection-complete', and PASS.
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

- 'exec stdout 1>&-' is the only accepted exact descriptor-close redirection
  form.
- Redirection affects only the launched VFS-backed executable's child
  descriptor table.
- '/bin/stdout' continues to load through descriptor-backed VFS/open/read
  before the userspace fixture attempts its fd1 write.
- The accepted observable close result is fd1 write '-EBADF' with
  'stream=closed route=closed-descriptor'.
- The shell descriptor table is restored after the redirected child launch.
- Normal stdout/stderr, both descriptor-dup directions, stdin
  wait/readiness/EOF, VFS exec, lifecycle/status, 'waitpid', 'laststatus',
  negative controls, and descriptor-backed cat behavior remain covered by
  retained evidence.

Deferred:

- stderr '2>&-' and arbitrary 'N>&-';
- descriptor moves, regular-file redirection, append/truncate, and pipes;
- writable filesystem behavior, separate physical sinks, full terminal policy,
  async jobs, fork, signals, libc stdio, Pi 5 proof, networking, and SSH.

## Next Step Requirement

The queued 'phase10-stderr-close-redirection-core-20260604' task is
mechanically unblocked as the inverse standard-stream descriptor-close slice.
It has explicit scope, non-goals, validation gates, docs, evidence
requirements, accepted stdout close evidence as a regression, and an unlocked
hardwareTestLock condition. It must stay bounded to child-only 'exec stderr
2>&-'; arbitrary descriptor close syntax, file redirection, pipes, writable
filesystem behavior, networking, and SSH remain outside this frontier.

## Validation Summary

- static inspection: accepted task records and retained evidence logs were
  inspected.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.
