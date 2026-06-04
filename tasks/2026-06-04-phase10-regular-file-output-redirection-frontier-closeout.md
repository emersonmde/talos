# Phase 10 Regular-File Output Redirection Frontier Closeout

Task: phase10-regular-file-output-redirection-frontier-closeout-20260604
Status: accepted

## Scope

Checkpoint the accepted regular-file output redirection frontier across the
task-owned stdout and stderr slices.

The accepted shell-visible forms are exactly:

- 'exec stdout >/tmp/stdout.txt' for the VFS-backed '/bin/stdout' fixture.
- 'exec stderr 2>/tmp/stderr.txt' for the VFS-backed '/bin/stderr' fixture.

Each form rebinds only the launched child descriptor to a minimal volatile VFS
regular-file descriptor, writes fixture bytes through the descriptor-backed
TalosWrite path, closes, reopens, and reads back through descriptor-backed VFS
read/cat. The shell descriptor is restored after child exit. The accepted
scratch paths are limited to '/tmp/stdout.txt' and '/tmp/stderr.txt'.

This closeout does not add code and does not expand into append, arbitrary
output paths, persistent storage, broad writable filesystem mutation,
descriptor moves, broader descriptor syntax, here-docs, multi-stage or
concurrent pipelines, jobs, signals, networking, SSH, Pi 5 proof, or a phase
transition.

## Findings

- fixed: Consolidated the accepted stdout form
  'exec stdout >/tmp/stdout.txt' with child fd1 routed to
  'volatile-vfs:/tmp/stdout.txt'.
- fixed: Consolidated the accepted stderr form
  'exec stderr 2>/tmp/stderr.txt' with child fd2 routed to
  'volatile-vfs:/tmp/stderr.txt'.
- fixed: Confirmed both stdout and stderr regular-file sinks use task-owned
  volatile create/truncate/write/read semantics for only the accepted scratch
  file paths.
- fixed: Confirmed both fixtures write 0x1f bytes through
  'source=userspace-talos-write' and read back the same payloads through
  'source=volatile-vfs-descriptor-read'.
- fixed: Confirmed shell fd1 and fd2 restoration through later normal
  'exec stdout' and 'exec stderr' controls routed to runtime-console0.
- fixed: Confirmed stderr redirection does not capture stdout; the
  task-owned stderr log retains a following normal stdout control through
  'runtime-console0/stdout'.
- fixed: Retained read-only input redirection, /dev/null output sinks,
  normal stdio, descriptor dup/close redirection, descriptor-mixing pipeline,
  VFS exec/open/read/write, lifecycle/status, waitpid, laststatus,
  deterministic negative-form, and descriptor-backed cat controls.
- fixed: Updated the roadmap with the consolidated output regular-file
  frontier and the required supervisor-planning boundary after this closeout.
- deferred: append, partial overwrite, unlink, rename, mkdir, directory
  mutation, permissions, metadata timestamps, fsync, arbitrary output paths,
  persistent storage, broad writable filesystem behavior, arbitrary descriptor
  syntax, descriptor moves, here-docs, multi-stage or concurrent pipelines,
  pipefail, jobs, async execution, fork, signals, Pi 5 proof, networking, SSH,
  and any phase transition.

## Evidence Map

- stdout regular-file redirection evidence:
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log'
  records command 'exec stdout >/tmp/stdout.txt', 'fd1=regular-file',
  'exec-redirection op=sink ... target-path=/tmp/stdout.txt ...
  target-stream=regular-file target-route=volatile-vfs:/tmp/stdout.txt
  child-only=true shell-restored=true', 'exec-stdout ... bytes=0x1f
  return=0x1f stream=regular-file route=volatile-vfs:/tmp/stdout.txt
  source=userspace-talos-write', lifecycle/status, waitpid, laststatus,
  descriptor-backed readback with 'cat /tmp/stdout.txt',
  'cat path=/tmp/stdout.txt bytes=0x1f
  source=volatile-vfs-descriptor-read', final classification
  'qemu-local-shell-stdout-regular-file-redirection-complete', errors=0, and
  PASS.
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
- shell descriptor restoration controls:
  the stdout log records a later normal 'exec stdout' routed to
  'runtime-console0/stdout'; the stderr log records a later normal
  'exec stderr' routed to 'runtime-console0/stderr'.
- distinct stream control:
  the stderr log records a following normal 'exec stdout' with visible
  'Talos userspace stdout fixture' through 'runtime-console0/stdout'.
- task-owned deterministic negatives:
  stdout log rejects 'exec stdout >>/tmp/stdout.txt',
  'exec stdout >/tmp/other.txt', and 'exec stderr 2>/tmp/stdout.txt';
  stderr log rejects 'exec stderr 2>>/tmp/stderr.txt',
  'exec stderr 2>/tmp/other.txt', and 'exec stdout >/tmp/stderr.txt'.
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
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-regular-file-redirection-core/qemu-local-shell-stderr-regular-file-redirection-smoke.log', and
  'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log'.

## Accepted Frontier

Accepted:

- exactly 'exec stdout >/tmp/stdout.txt';
- exactly 'exec stderr 2>/tmp/stderr.txt';
- scratch output paths '/tmp/stdout.txt' and '/tmp/stderr.txt' only;
- child-only fd1 and fd2 rebinding to volatile VFS regular-file descriptors
  for those exact paths;
- minimal volatile create/truncate behavior when opening either accepted sink;
- descriptor-backed write from the userspace stdout/stderr fixtures, close,
  reopen, and descriptor-backed readback through 'cat';
- captured payloads 'Talos userspace stdout fixture' and
  'Talos userspace stderr fixture', each with 0x1f bytes and readback
  provenance 'source=volatile-vfs-descriptor-read';
- shell fd1/fd2 restoration after child exit;
- distinct stdout/stderr behavior, /dev/null output sinks, read-only input
  redirection, normal stdio, descriptor redirection/pipeline controls,
  VFS-backed launch, lifecycle/status, waitpid, laststatus, deterministic
  negatives, and descriptor-backed cat/readback are covered by retained
  QEMU/substitute evidence.

Deferred:

- append, partial overwrite, unlink, rename, mkdir, directory mutation,
  permissions, timestamps, fsync, persistence, and broader writable
  filesystem semantics;
- arbitrary output paths and path creation beyond '/tmp/stdout.txt' and
  '/tmp/stderr.txt';
- arbitrary descriptor syntax such as 'N>target', 'N>>target', 'N>&M',
  descriptor moves, and close/restore expansion beyond accepted exact forms;
- here-docs, globbing, quoting, variables, environment-backed path behavior,
  and wider shell grammar;
- multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, process accounting/concurrency, and job control;
- Pi 5 proof, networking, SSH, and any phase transition.

## Next Step Requirement

No further queued task is mechanically unblocked by this closeout. The next
feature-led direction crosses a real planning choice between append behavior,
broader descriptor grammar, and process accounting/concurrency. Supervisor
planning is required before the worker promotes or executes further work.

## Validation

- static inspection: accepted stdout and stderr regular-file redirection
  evidence, read-only input redirection, '/dev/null' output sinks, normal
  stdio, descriptor redirection, pipeline, VFS/open/read/write,
  lifecycle/wait/status/cat controls, deterministic negatives, and roadmap
  entries were inspected.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final regular-file output redirection frontier closeout commit
recorded in supervisor state.
