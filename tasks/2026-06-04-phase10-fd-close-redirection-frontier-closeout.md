# Phase 10 FD Close Redirection Frontier Closeout

Task: phase10-fd-close-redirection-frontier-closeout-20260604

Status: accepted

## Summary

This closeout checkpoints the accepted descriptor-close redirection frontier
before any broader shell I/O planning. The accepted feature is limited to two
exact shell-visible forms for VFS-backed exec children:

- 'exec stdout 1>&-': child fd1 is closed before '/bin/stdout' attempts its
  userspace 'TalosWrite'.
- 'exec stderr 2>&-': child fd2 is closed before '/bin/stderr' attempts its
  userspace 'TalosWrite'.

Both forms are child-only. The shell descriptor table is restored after the
launched child exits, and retained normal 'exec stdout' / 'exec stderr'
controls prove fd1 and fd2 route metadata return to their non-closed values.

The accepted physical sink remains shared runtime-console0. This frontier
accepts descriptor-table close behavior and closed-descriptor metadata, not
arbitrary descriptor syntax, descriptor moves, file redirection, pipes, or
writable filesystem behavior.

## Findings And Disposition

- fixed: Reconciled the accepted stdout '1>&-' and stderr '2>&-'
  descriptor-close direction records into one frontier boundary.
- fixed: Preserved child-only semantics and shell descriptor restoration as
  the defining acceptance rule for both descriptor-close forms.
- fixed: Preserved normal stdout and stderr route controls proving
  non-redirected children still report 'stream=stdout
  route=runtime-console0/stdout' for fd1 and 'stream=stderr
  route=runtime-console0/stderr' for fd2.
- fixed: Preserved both accepted descriptor-dup directions, scheduler-backed
  stdin wait/readiness, Ctrl-D EOF, descriptor-backed VFS exec,
  lifecycle/status, consuming 'waitpid', non-consuming 'laststatus',
  deterministic negative exec/redirection controls, loader temporary
  descriptor non-leak, and descriptor-backed 'cat /etc/banner.txt' evidence.
- not-an-issue: The '-EBADF' close result is reported by the relevant
  userspace fixture's fd write after descriptor-backed VFS loading succeeds;
  it is not a loader, launch, lifecycle, or status failure.
- not-an-issue: fd1 and fd2 still share the runtime-console0 physical output
  device. That is consistent with the accepted route-metadata boundary.
- deferred: arbitrary 'N>&-' descriptor close syntax, descriptor moves,
  descriptor close-and-restore syntax beyond the two exact forms,
  regular-file redirection, append/truncate, pipes, here-docs, shell
  variables, quoting expansion, writable filesystem behavior, separate
  physical stdout/stderr sinks, full terminal policy, async jobs, fork,
  signals, libc stdio, Pi 5 proof, networking, and SSH.
- deferred: selecting the next feature-led shell I/O slice requires supervisor
  planning because minimal pipe producer-consumer lifecycle, file/device
  redirection, broader descriptor manipulation, and shell syntax expansion
  each carry different scope and dependency risks.

## Evidence Map

- stdout descriptor-close redirection:
  'tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log'.
  Static inspection of QEMU/substitute evidence shows 'exec stdout 1>&-',
  descriptor-backed '/bin/stdout' VFS/open/read lineage,
  'exec-descriptors ... inherited-count=2 fd1=closed',
  'exec-redirection op=close source-fd=1 result=closed-descriptor
  child-only=true shell-restored=true', userspace fd1 write return '-EBADF'
  as 'stream=closed route=closed-descriptor', lifecycle/status, 'waitpid',
  'laststatus', normal stdout restoration, deterministic unsupported
  redirection negatives, descriptor-backed 'cat /etc/banner.txt', final
  classification 'qemu-local-shell-stdout-close-redirection-complete', and
  PASS.
- stderr descriptor-close redirection:
  'tasks/evidence/2026-06-04-phase10-stderr-close-redirection-core/qemu-local-shell-stderr-close-redirection-smoke.log'.
  Static inspection of QEMU/substitute evidence shows 'exec stderr 2>&-',
  descriptor-backed '/bin/stderr' VFS/open/read lineage,
  'exec-descriptors ... inherited-count=2 fd2=closed',
  'exec-redirection op=close source-fd=2 result=closed-descriptor
  child-only=true shell-restored=true', userspace fd2 write return '-EBADF'
  as 'stream=closed route=closed-descriptor', lifecycle/status, 'waitpid',
  'laststatus', normal stderr restoration, retained stdout close regression,
  deterministic unsupported redirection negatives, descriptor-backed
  'cat /etc/banner.txt', final classification
  'qemu-local-shell-stderr-close-redirection-complete', and PASS.
- stdout-to-stderr descriptor-dup control:
  'tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log'.
  QEMU/substitute evidence retains 'exec stdout 1>&2', child-only fd1
  rebinding to inherited fd2, shell restoration, stream/route metadata,
  lifecycle/status, 'waitpid', 'laststatus', deterministic negative
  redirection controls, descriptor-backed 'cat /etc/banner.txt', and PASS.
- stderr-to-stdout descriptor-dup control:
  'tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log'.
  QEMU/substitute evidence retains 'exec stderr 2>&1', child-only fd2
  rebinding to inherited fd1, shell restoration, the forward '1>&2'
  regression, lifecycle/status, 'waitpid', 'laststatus', descriptor-backed
  'cat /etc/banner.txt', and PASS.
- normal stdout route control:
  'tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log'.
  QEMU/substitute evidence retains fd1 writes as 'stream=stdout
  route=runtime-console0/stdout', plus lifecycle/status, 'waitpid',
  'laststatus', deterministic negative exec controls, descriptor-backed
  'cat /etc/banner.txt', final classification
  'qemu-local-shell-userspace-stdout-complete', and PASS.
- normal stderr route control:
  'tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log'.
  QEMU/substitute evidence retains fd2 writes as 'stream=stderr
  route=runtime-console0/stderr', plus lifecycle/status, 'waitpid',
  'laststatus', deterministic negative exec controls, descriptor-backed
  'cat /etc/banner.txt', final classification
  'qemu-local-shell-userspace-stderr-complete', and PASS.
- distinct stderr route control:
  'tasks/evidence/2026-06-04-phase10-distinct-stderr-routing-core/qemu-local-shell-distinct-stderr-routing-smoke.log'.
  QEMU/substitute evidence retains fd2 as 'stream=stderr
  route=runtime-console0/stderr' while preserving stdout route control.
- scheduler-backed stdin wait/readiness control:
  'tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log'.
  QEMU/substitute evidence retains scheduler sleep and wake/resume markers,
  delayed runtime-console0 bytes, and
  'read-result=scheduler-wait/delayed-input'.
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
- The shell descriptor table is restored after each redirected child launch.
- '/bin/stdout' and '/bin/stderr' continue to load through descriptor-backed
  VFS/open/read before userspace attempts fd1/fd2 writes.
- The accepted observable close result is the fixture write '-EBADF' with
  'stream=closed route=closed-descriptor'.
- Normal stdout/stderr, both descriptor-dup directions, stdin
  wait/readiness/EOF, VFS exec, lifecycle/status, 'waitpid', 'laststatus',
  negative controls, and descriptor-backed cat behavior remain covered by
  retained evidence.

Deferred:

- arbitrary descriptor close beyond the two exact forms;
- descriptor moves and broad descriptor close/restore syntax;
- regular-file redirection, append/truncate, here-docs, and pipes;
- writable filesystem behavior, separate physical sinks, full terminal
  policy, async jobs, fork, signals, libc stdio, Pi 5 proof, networking, and
  SSH.

## Next Step Requirement

Supervisor planning is required before the next feature-led shell I/O task.
The descriptor-close frontier is now closed for the two exact standard-stream
forms only. Do not infer arbitrary descriptor syntax, pipes, file/device
redirection, writable filesystem behavior, networking, SSH, or a phase
transition from this closeout.

## Validation Summary

- static inspection: accepted direction task records and retained evidence logs
  were inspected.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.
