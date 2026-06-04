# Phase 10 Stderr Regular-File Redirection Closeout

Task: phase10-stderr-regular-file-redirection-closeout-20260604
Status: accepted

## Scope

Checkpoint the accepted stderr regular-file redirection slice after the
task-owned volatile VFS write/read implementation.

The accepted shell-visible form is exactly 'exec stderr 2>/tmp/stderr.txt' for
the VFS-backed '/bin/stderr' fixture. The launched child has fd2 rebound to a
minimal volatile regular-file descriptor for '/tmp/stderr.txt', writes the
userspace stderr fixture bytes through that descriptor, exits through the
accepted lifecycle/status path, and leaves shell fd2 restored for later normal
stderr. A normal stdout control remains visible through fd1 and proves the
stderr sink does not capture stdout unless an accepted stdout form is used.

This closeout does not add code and does not expand into append, arbitrary
output paths, persistent storage, broad writable filesystem behavior,
descriptor moves, broader descriptor syntax, here-docs, broader pipes, jobs,
signals, networking, SSH, Pi 5 proof, or a phase transition.

## Findings

- fixed: Consolidated exactly 'exec stderr 2>/tmp/stderr.txt' as an accepted
  child-only fd2 sink backed by the volatile VFS scratch file
  '/tmp/stderr.txt'.
- fixed: Confirmed the descriptor route is fd2=regular-file with
  'op=sink', 'target-path=/tmp/stderr.txt',
  'target-stream=regular-file', and
  'target-route=volatile-vfs:/tmp/stderr.txt'.
- fixed: Confirmed the userspace stderr fixture writes 0x1f bytes through the
  descriptor-backed TalosWrite path with 'source=userspace-talos-write' and
  'route=volatile-vfs:/tmp/stderr.txt'.
- fixed: Confirmed descriptor-backed readback through 'cat /tmp/stderr.txt'
  observes 'Talos userspace stderr fixture' with 'bytes=0x1f' and
  'source=volatile-vfs-descriptor-read'.
- fixed: Confirmed shell fd2 restoration by retaining the following normal
  'exec stderr' control, which reports fd2=stdio-output and writes to
  'runtime-console0/stderr'.
- fixed: Confirmed normal stdout remains distinct through a later
  'exec stdout' control routed to 'runtime-console0/stdout'.
- fixed: Retained stdout regular-file redirection, /dev/null sinks, read-only
  input redirection, normal stdio, descriptor dup/close redirection,
  descriptor-mixing pipeline, VFS exec/open/read/write, lifecycle/status,
  waitpid, laststatus, negative-form, and descriptor-backed cat controls.
- fixed: Updated the roadmap with a closeout entry that keeps the accepted
  boundary to volatile create/truncate/write/read of the one scratch stderr
  file and explicitly defers append, arbitrary paths, persistence, wider
  filesystem mutation, hardware proof, networking, SSH, and any phase
  transition.
- deferred: append, partial overwrite, unlink, rename, mkdir, directory
  mutation, permissions, metadata timestamps, fsync, arbitrary output paths,
  persistent storage, broad writable filesystem behavior, arbitrary descriptor
  syntax, descriptor moves, here-docs, multi-stage or concurrent pipelines,
  pipefail, jobs, async execution, fork, signals, Pi 5 proof, networking, SSH,
  and any phase transition.

## Evidence Map

- stderr regular-file redirection evidence:
  'tasks/evidence/2026-06-04-phase10-stderr-regular-file-redirection-core/qemu-local-shell-stderr-regular-file-redirection-smoke.log'
  records command 'exec stderr 2>/tmp/stderr.txt', 'fd2=regular-file',
  'exec-redirection op=sink ... target-path=/tmp/stderr.txt ...
  target-stream=regular-file target-route=volatile-vfs:/tmp/stderr.txt
  child-only=true shell-restored=true', 'exec-stderr ... bytes=0x1f
  return=0x1f stream=regular-file route=volatile-vfs:/tmp/stderr.txt
  source=userspace-talos-write', lifecycle/status, waitpid, laststatus,
  descriptor-backed readback with 'cat /tmp/stderr.txt',
  'cat path=/tmp/stderr.txt bytes=0x1f
  source=volatile-vfs-descriptor-read', final classification
  'qemu-local-shell-stderr-regular-file-redirection-complete', errors=0, and
  PASS.
- shell fd2 restoration control: the stderr regular-file redirection log
  records a following normal 'exec stderr' with fd2 restored to the stdio
  output route, visible 'Talos userspace stderr fixture', and
  'route=runtime-console0/stderr'.
- distinct stdout control: the stderr regular-file redirection log records a
  following normal 'exec stdout' with visible 'Talos userspace stdout fixture'
  and 'route=runtime-console0/stdout'.
- retained deterministic negatives in the task-owned log:
  'exec stderr 2>>/tmp/stderr.txt', 'exec stderr 2>/tmp/other.txt', and
  'exec stdout >/tmp/stderr.txt' all fail without accepting append,
  arbitrary-path output, or stdout output to the stderr scratch file.
- retained stdout regular-file redirection evidence:
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log'.
- retained '/dev/null' output sink evidence:
  'tasks/evidence/2026-06-04-phase10-dev-null-stdout-redirection-contract-core/qemu-local-shell-dev-null-stdout-redirection-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-dev-null-stderr-redirection-smoke.log'.
- retained read-only input redirection evidence:
  'tasks/evidence/2026-06-04-phase10-dev-null-stdin-redirection-core/qemu-local-shell-dev-null-stdin-redirection-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-readonly-regular-file-stdin-redirection-core/qemu-local-shell-readonly-regular-file-stdin-redirection-smoke.log'.
- retained normal stdio and distinct stream controls:
  'tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-distinct-stderr-routing-core/qemu-local-shell-distinct-stderr-routing-smoke.log'.
- retained descriptor redirection and close controls:
  'tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-stderr-close-redirection-core/qemu-local-shell-stderr-close-redirection-smoke.log'.
- retained descriptor-mixing pipeline controls:
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
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log', and
  'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log'.

## Accepted Frontier

Accepted:

- exactly 'exec stderr 2>/tmp/stderr.txt';
- the scratch output path '/tmp/stderr.txt' only;
- child-only fd2 rebinding to a volatile VFS regular-file descriptor;
- minimal volatile create/truncate behavior for that one file when opening the
  accepted stderr sink;
- descriptor-backed write from the userspace stderr fixture, close, reopen,
  and descriptor-backed readback through 'cat /tmp/stderr.txt';
- the captured payload 'Talos userspace stderr fixture' with 0x1f bytes and
  readback provenance 'source=volatile-vfs-descriptor-read';
- shell fd2 restoration after child exit, proven by a later normal
  'exec stderr' routed to runtime-console0/stderr;
- distinct stdout visibility after the redirected stderr command, proven by a
  later normal 'exec stdout' routed to runtime-console0/stdout;
- VFS-backed launch, lifecycle/status, waitpid, laststatus, normal stdio,
  deterministic negatives, descriptor redirection/pipeline controls, read-only
  input controls, /dev/null sink controls, stdout regular-file output, and
  descriptor-backed cat are covered by retained QEMU/substitute evidence.

Deferred:

- append, partial overwrite, unlink, rename, mkdir, directory mutation,
  permissions, timestamps, fsync, persistence, and broader writable
  filesystem semantics;
- arbitrary output paths and path creation beyond '/tmp/stderr.txt';
- arbitrary descriptor syntax such as 'N>target', 'N>>target', 'N>&M',
  descriptor moves, and close/restore expansion beyond accepted exact forms;
- here-docs, globbing, quoting, variables, environment-backed path behavior,
  and wider shell grammar;
- multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, and job control;
- Pi 5 proof, networking, SSH, and any phase transition.

## Next Step Requirement

The next queued task,
'phase10-regular-file-output-redirection-frontier-closeout-20260604', is
mechanically unblocked by this accepted closeout if the repo remains clean or
any unrelated changes do not conflict with its explicit docs/evidence-only
scope. It must stay limited to reconciling accepted stdout and stderr
regular-file output redirection and must not implement append, arbitrary paths,
persistence, broader filesystem mutation, hardware proof, networking, SSH, or
a phase transition.

## Validation

- static inspection: accepted stderr regular-file redirection evidence,
  stdout regular-file redirection, '/dev/null' output sinks, read-only input
  redirection, normal stdio, descriptor redirection, pipeline, VFS/open/read,
  lifecycle/wait/status/cat controls, deterministic negatives, and roadmap
  entries were inspected.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final stderr regular-file redirection closeout commit recorded in
supervisor state.
