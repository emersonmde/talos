# Phase 10 Regular-File Append Redirection Frontier Closeout

Task: phase10-regular-file-append-redirection-frontier-closeout-20260604
Status: accepted

## Scope

Close out the accepted regular-file append redirection frontier without adding
code.

The accepted shell-visible append sequences are exactly:

- stdout setup/create-truncate: 'exec stdout >/tmp/stdout.txt';
- stdout append: 'exec stdout >>/tmp/stdout.txt';
- stdout readback: 'cat /tmp/stdout.txt';
- stderr setup/create-truncate: 'exec stderr 2>/tmp/stderr.txt';
- stderr append: 'exec stderr 2>>/tmp/stderr.txt';
- stderr readback: 'cat /tmp/stderr.txt'.

Both stdout commands launch the VFS-backed '/bin/stdout' fixture, and both
stderr commands launch the VFS-backed '/bin/stderr' fixture. Append is accepted
only after the prior setup/truncate-create command in the retained evidence.
The append command rebinds only the child target descriptor to the same
volatile regular-file descriptor without truncating existing contents, writes
the second fixture payload through userspace TalosWrite, restores the shell
descriptor, and leaves descriptor-backed cat able to read two fixture payloads
in order.

This closeout does not add code and does not expand into missing-file append
create, arbitrary append paths, persistent storage, broad writable filesystem
mutation, arbitrary descriptor syntax, descriptor moves, here-docs,
multi-stage or concurrent pipelines, jobs, fork, signals, async execution,
process accounting/concurrency, networking, SSH, Pi 5 proof, or a phase
transition.

## Findings

- fixed: Consolidated the accepted append frontier as two exact scratch paths:
  '/tmp/stdout.txt' for stdout append and '/tmp/stderr.txt' for stderr append.
- fixed: Confirmed setup/truncate-create remains required before each accepted
  append command; missing-file append-create remains deferred.
- fixed: Confirmed stdout append uses child-only fd1 regular-file rebinding
  with 'op=append', 'target-path=/tmp/stdout.txt',
  'target-route=volatile-vfs:/tmp/stdout.txt',
  'source=shell-redirection-stdout-tmp-stdout-append',
  'source=userspace-talos-write', shell fd1 restoration, and descriptor-backed
  readback of two stdout fixture payloads with 'bytes=0x3e
  source=volatile-vfs-descriptor-read'.
- fixed: Confirmed stderr append uses child-only fd2 regular-file rebinding
  with 'op=append', 'target-path=/tmp/stderr.txt',
  'target-route=volatile-vfs:/tmp/stderr.txt',
  'source=shell-redirection-stderr-tmp-stderr-append',
  'source=userspace-talos-write', shell fd2 restoration, distinct stdout
  behavior, and descriptor-backed readback of two stderr fixture payloads with
  'bytes=0x3e source=volatile-vfs-descriptor-read'.
- fixed: Retained stdout/stderr truncate redirection, read-only input
  redirection, /dev/null redirection, normal stdio, descriptor
  redirection/pipeline controls, VFS exec/open/read/write lineage,
  lifecycle/status, waitpid, laststatus, deterministic negatives, and
  descriptor-backed cat/readback controls.
- fixed: Updated the roadmap with the append frontier boundary and recorded
  that explicit supervisor planning is required before any broader descriptor
  grammar, arbitrary path, process accounting/concurrency, or phase-transition
  work.
- deferred: append-create for missing files, arbitrary append paths,
  stdout-to-stderr and stderr-to-stdout append path mixups beyond accepted
  negative controls, partial overwrite, unlink, rename, mkdir, directory
  mutation, permissions, timestamps, fsync, persistence, broad writable
  filesystem behavior, arbitrary descriptor syntax, descriptor moves,
  here-docs, wider shell grammar, multi-stage or concurrent pipelines,
  pipefail, jobs, async execution, fork, signals, process
  accounting/concurrency, Pi 5 proof, networking, SSH, and phase transition.

## Evidence Map

- stdout append redirection evidence:
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-append-redirection-core/qemu-local-shell-stdout-regular-file-append-redirection-smoke.log'
  records setup command 'exec stdout >/tmp/stdout.txt', append command
  'exec stdout >>/tmp/stdout.txt', 'exec-redirection op=append',
  'target-path=/tmp/stdout.txt',
  'target-route=volatile-vfs:/tmp/stdout.txt',
  'source=shell-redirection-stdout-tmp-stdout-append',
  userspace TalosWrite provenance, shell fd1 restoration through a later
  normal 'exec stdout', deterministic negatives, descriptor-backed readback
  with 'cat /tmp/stdout.txt', two visible 'Talos userspace stdout fixture'
  payloads, 'cat path=/tmp/stdout.txt bytes=0x3e
  source=volatile-vfs-descriptor-read', final classification
  'qemu-local-shell-stdout-regular-file-append-redirection-complete',
  errors=0, and PASS.
- stderr append redirection evidence:
  'tasks/evidence/2026-06-04-phase10-stderr-regular-file-append-redirection-core/qemu-local-shell-stderr-regular-file-append-redirection-smoke.log'
  records setup command 'exec stderr 2>/tmp/stderr.txt', append command
  'exec stderr 2>>/tmp/stderr.txt', 'exec-redirection op=append',
  'target-path=/tmp/stderr.txt',
  'target-route=volatile-vfs:/tmp/stderr.txt',
  'source=shell-redirection-stderr-tmp-stderr-append',
  userspace TalosWrite provenance, shell fd2 restoration through a later
  normal 'exec stderr', distinct normal stdout behavior, deterministic
  negatives, descriptor-backed readback with 'cat /tmp/stderr.txt', two
  visible 'Talos userspace stderr fixture' payloads,
  'cat path=/tmp/stderr.txt bytes=0x3e
  source=volatile-vfs-descriptor-read', final classification
  'qemu-local-shell-stderr-regular-file-append-redirection-complete',
  errors=0, and PASS.
- retained stdout/stderr truncate-create evidence:
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-stderr-regular-file-redirection-core/qemu-local-shell-stderr-regular-file-redirection-smoke.log'.
- retained read-only input redirection evidence:
  'tasks/evidence/2026-06-04-phase10-dev-null-stdin-redirection-core/qemu-local-shell-dev-null-stdin-redirection-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-readonly-regular-file-stdin-redirection-core/qemu-local-shell-readonly-regular-file-stdin-redirection-smoke.log'.
- retained /dev/null output sink evidence:
  'tasks/evidence/2026-06-04-phase10-dev-null-stdout-redirection-contract-core/qemu-local-shell-dev-null-stdout-redirection-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-dev-null-stderr-redirection-smoke.log'.
- retained normal stdio controls:
  'tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log', and
  'tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log'.
- retained descriptor redirection and pipeline controls:
  'tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-close-redirection-core/qemu-local-shell-stderr-close-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log',
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log',
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log'.
- retained VFS exec/open/read/write, lifecycle/status, waitpid, laststatus,
  negative controls, and descriptor-backed file I/O:
  'tasks/evidence/2026-06-03-phase10-absolute-vfs-exec-dispatch-core/qemu-local-shell-absolute-vfs-exec-dispatch-smoke.log',
  'tasks/evidence/2026-06-03-phase10-vfs-exec-nonzero-status-core/qemu-local-shell-nonzero-vfs-exec-status-smoke.log',
  'tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log',
  'tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-append-redirection-core/qemu-local-shell-stdout-regular-file-append-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-regular-file-append-redirection-core/qemu-local-shell-stderr-regular-file-append-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-regular-file-redirection-core/qemu-local-shell-stderr-regular-file-redirection-smoke.log', and
  'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log'.

## Accepted Frontier

Accepted:

- exactly 'exec stdout >/tmp/stdout.txt' followed later by exactly
  'exec stdout >>/tmp/stdout.txt';
- exactly 'exec stderr 2>/tmp/stderr.txt' followed later by exactly
  'exec stderr 2>>/tmp/stderr.txt';
- setup/truncate-create is required before append in the accepted evidence;
- scratch append paths '/tmp/stdout.txt' and '/tmp/stderr.txt' only;
- VFS-backed '/bin/stdout' and '/bin/stderr' fixtures only for these append
  forms;
- child-only fd1/fd2 rebinding to volatile VFS regular-file descriptors;
- append without truncating existing scratch-file contents;
- descriptor-backed userspace writes and descriptor-backed readback through
  'cat /tmp/stdout.txt' and 'cat /tmp/stderr.txt';
- readback of two fixture payloads per stream with 'bytes=0x3e
  source=volatile-vfs-descriptor-read';
- shell fd1/fd2 restoration after child exit and stderr-append evidence that
  normal stdout remains distinct;
- stdout/stderr truncate-create, read-only input redirection, /dev/null
  redirection, normal stdio, descriptor redirection/pipeline controls,
  VFS-backed launch, lifecycle/status, waitpid, laststatus, deterministic
  negatives, and descriptor-backed cat/readback are covered by retained
  QEMU/substitute evidence.

Deferred:

- append-create for missing files and append to arbitrary paths;
- stdout-to-stderr and stderr-to-stdout append path mixups beyond accepted
  negative controls;
- partial overwrite, unlink, rename, mkdir, directory mutation, permissions,
  timestamps, fsync, persistence, and broader writable filesystem semantics;
- arbitrary descriptor syntax such as 'N>>target', descriptor moves, and
  close/restore expansion beyond accepted exact forms;
- here-docs, globbing, quoting, variables, environment-backed path behavior,
  and wider shell grammar;
- multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, process accounting/concurrency, and job control;
- Pi 5 proof, networking, SSH, and any phase transition.

## Next Step Requirement

No explicit queued follow-up remains mechanically unblocked by this frontier
closeout. Supervisor planning is required before choosing the next bounded
feature-led task, including any broader descriptor grammar, append-create,
arbitrary path, process accounting/concurrency, or phase-transition work.

## Validation

- static inspection: accepted stdout and stderr append task records and
  QEMU/substitute evidence were inspected, including setup/truncate, append,
  descriptor routes, userspace TalosWrite provenance, descriptor-backed
  readbacks, fd1/fd2 restoration, distinct stdout behavior, deterministic
  negatives, waitpid, laststatus, completion markers, errors=0, and PASS
  lines.
- static inspection: retained stdout/stderr truncate, read-only input
  redirection, /dev/null redirection, normal stdio, descriptor
  redirection/pipeline, VFS/open/read/write, lifecycle/status, waitpid,
  laststatus, negative-control, and descriptor-backed cat evidence paths were
  checked for presence and PASS/completion markers.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final regular-file append redirection frontier closeout commit
recorded in supervisor state.
