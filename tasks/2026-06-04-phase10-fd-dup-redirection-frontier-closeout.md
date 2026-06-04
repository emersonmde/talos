# Phase 10 FD Dup Redirection Frontier Closeout

Task: phase10-fd-dup-redirection-frontier-closeout-20260604

Status: accepted

## Summary

This closeout checkpoints the accepted descriptor-duplication redirection
frontier before any broader shell I/O planning. The accepted feature is limited
to two exact shell-visible forms for VFS-backed exec children:

- 'exec stdout 1>&2': child fd1 is rebound to the inherited fd2 stderr route.
- 'exec stderr 2>&1': child fd2 is rebound to the inherited fd1 stdout route.

Both forms are child-only. The shell descriptor table is restored after the
launched child exits, and retained normal 'exec stdout' / 'exec stderr'
controls prove fd1 and fd2 route metadata return to their non-redirected
values.

The accepted physical sink remains shared runtime-console0. This frontier
accepts descriptor-table behavior and stream/route metadata, not separate
physical sinks, file redirection, pipes, or writable filesystem behavior.

## Findings And Disposition

- fixed: Reconciled the accepted forward '1>&2' and inverse '2>&1'
  descriptor-duplication direction records into one frontier boundary.
- fixed: Preserved child-only semantics and shell descriptor restoration as the
  defining acceptance rule for both descriptor-dup forms.
- fixed: Preserved normal stdout and stderr route controls proving
  non-redirected children still report 'stream=stdout
  route=runtime-console0/stdout' for fd1 and 'stream=stderr
  route=runtime-console0/stderr' for fd2.
- fixed: Preserved scheduler-backed stdin wait/readiness, Ctrl-D EOF,
  descriptor-backed VFS exec, lifecycle/status, consuming 'waitpid',
  non-consuming 'laststatus', deterministic negative exec/redirection
  controls, loader temporary descriptor non-leak, and descriptor-backed
  'cat /etc/banner.txt' evidence.
- not-an-issue: fd1 and fd2 still share the runtime-console0 physical output
  device. That is consistent with the accepted route-metadata boundary.
- deferred: arbitrary 'N>&M', descriptor close/move syntax, regular-file
  redirection, append/truncate, pipes, here-docs, shell variables, quoting
  expansion, writable filesystem behavior, separate physical stdout/stderr
  sinks, terminal policy, async jobs, fork, signals, libc stdio, Pi 5 proof,
  networking, and SSH.
- deferred: selecting the next feature-led shell I/O slice requires supervisor
  planning because descriptor close/restore syntax, minimal pipe
  producer-consumer lifecycle, and file/device redirection each carry different
  scope and dependency risks.

## Evidence Map

- stdout-to-stderr descriptor-dup redirection:
  'tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log'.
  Static inspection of QEMU/substitute evidence shows 'exec stdout 1>&2',
  descriptor-backed '/bin/stdout' VFS/open/read lineage,
  'exec-redirection op=dup source-fd=1 target-fd=2
  target-stream=stderr target-route=runtime-console0/stderr
  child-only=true shell-restored=true', userspace fd1 write metadata
  'stream=stderr route=runtime-console0/stderr', lifecycle/status,
  'waitpid', 'laststatus', deterministic unsupported inverse/file/pipe
  redirection negatives, descriptor-backed 'cat /etc/banner.txt', final
  classification 'qemu-local-shell-stdout-to-stderr-redirection-complete', and
  PASS.
- stderr-to-stdout descriptor-dup redirection:
  'tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log'.
  Static inspection of QEMU/substitute evidence shows 'exec stderr 2>&1',
  descriptor-backed '/bin/stderr' VFS/open/read lineage,
  'exec-redirection op=dup source-fd=2 target-fd=1
  target-stream=stdout target-route=runtime-console0/stdout
  child-only=true shell-restored=true', userspace fd2 write metadata
  'stream=stdout route=runtime-console0/stdout', lifecycle/status,
  'waitpid', 'laststatus', retained forward '1>&2' regression,
  deterministic unsupported file-redirection negative, descriptor-backed
  'cat /etc/banner.txt', final classification
  'qemu-local-shell-stderr-to-stdout-redirection-complete', and PASS.
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
  QEMU/substitute evidence retains the fd2 stream/route split while preserving
  stdout route control, lifecycle/status, wait controls, negative exec
  controls, descriptor-backed 'cat /etc/banner.txt', final classification
  'qemu-local-shell-distinct-stderr-routing-complete', and PASS.
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

- 'exec stdout 1>&2' and 'exec stderr 2>&1' are the only accepted exact
  descriptor-duplication redirection forms.
- Redirection affects only the launched VFS-backed executable's child
  descriptor table.
- The shell descriptor table is restored after each redirected child launch.
- '/bin/stdout' and '/bin/stderr' continue to load through descriptor-backed
  VFS/open/read before userspace writes through fd1/fd2.
- The accepted observable distinction is stream/route metadata:
  'stdout -> stderr' reports 'stream=stderr route=runtime-console0/stderr';
  'stderr -> stdout' reports 'stream=stdout route=runtime-console0/stdout'.
- Normal stdout/stderr, stdin wait/readiness/EOF, VFS exec,
  lifecycle/status, 'waitpid', 'laststatus', negative controls, and
  descriptor-backed cat behavior remain covered by retained evidence.

Deferred:

- arbitrary descriptor duplication beyond the two exact forms;
- descriptor close/move syntax;
- regular-file redirection, append/truncate, and pipes;
- writable filesystem behavior, separate physical sinks, full terminal
  policy, async jobs, fork, signals, libc stdio, Pi 5 proof, networking, and
  SSH.

## Next Step Requirement

Supervisor planning is required before the next feature-led shell I/O task.
The smallest plausible continuation inside descriptor policy is descriptor
close/restore syntax, because it builds directly on child-only descriptor-table
mutation. Minimal pipe producer-consumer lifecycle would be more user-visible
but requires new producer/consumer lifecycle semantics. File/device redirection
requires an explicit writable target or device-sink plan and must not be
accepted by implication from descriptor duplication.

## Validation Summary

- static inspection: accepted direction task records and retained evidence logs
  were inspected.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed with the existing large
  search-index warning.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.
