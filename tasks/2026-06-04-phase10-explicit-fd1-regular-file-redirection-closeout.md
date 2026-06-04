# Phase 10 Explicit Fd1 Regular-File Redirection Closeout

Task: phase10-explicit-fd1-regular-file-redirection-closeout-20260604
Status: accepted

## Scope

Close out the accepted explicit fd1 regular-file output grammar aliases without
adding code.

The accepted shell-visible forms are exactly:

- fd1 truncate/create alias: 'exec stdout 1>/tmp/stdout.txt';
- fd1 append/create alias: 'exec stdout 1>>/tmp/stdout.txt';
- fd1 readback: 'cat /tmp/stdout.txt'.

Both aliases are grammar spellings for the already accepted stdout fd1 volatile
regular-file descriptor path. The truncate form creates/truncates the bounded
scratch file, the append form appends through the accepted append/create path,
the child descriptor is rebound only for the launched process, userspace writes
carry TalosWrite provenance, descriptor-backed cat reads the scratch file back,
and shell fd1 is restored afterward.

This closeout does not add code and does not expand into arbitrary descriptor
syntax, arbitrary output paths, fd2 alias expansion beyond the accepted exact
stderr forms, descriptor moves, persistent storage, broad writable filesystem
mutation, here-docs, multi-stage or concurrent pipelines, process
accounting/concurrency, networking, SSH, Pi 5 proof, hardwareTestLock
acquisition, or a phase transition.

## Findings

- fixed: Consolidated 'exec stdout 1>/tmp/stdout.txt' as an accepted explicit
  fd1 truncate/create alias for the volatile VFS scratch file
  '/tmp/stdout.txt'.
- fixed: Consolidated 'exec stdout 1>>/tmp/stdout.txt' as an accepted explicit
  fd1 append/create alias for the same volatile VFS scratch file.
- fixed: Confirmed the truncate alias records child-only fd1 rebinding with
  'op=sink', 'source-fd=0x1', 'target-path=/tmp/stdout.txt',
  'target-stream=regular-file',
  'target-route=volatile-vfs:/tmp/stdout.txt',
  'source=shell-redirection-stdout-tmp-stdout', userspace TalosWrite
  provenance, lifecycle/status, waitpid, laststatus, shell fd1 restoration,
  and descriptor-backed readback of one stdout fixture payload with
  'bytes=0x1f source=volatile-vfs-descriptor-read'.
- fixed: Confirmed the append alias records child-only fd1 rebinding with
  'op=append', 'source-fd=0x1', 'target-path=/tmp/stdout.txt',
  'target-stream=regular-file',
  'target-route=volatile-vfs:/tmp/stdout.txt',
  'source=shell-redirection-stdout-tmp-stdout-append', userspace TalosWrite
  provenance, lifecycle/status, waitpid, laststatus, shell fd1 restoration,
  and descriptor-backed readback of two stdout fixture payloads with
  'bytes=0x3e source=volatile-vfs-descriptor-read'.
- fixed: Confirmed unsupported explicit fd numbers and arbitrary output paths
  remain deterministic negatives, including 'exec stdout 3>/tmp/stdout.txt'
  and 'exec stdout 1>/tmp/other.txt' in the task-owned QEMU/substitute
  transcript.
- fixed: Retained implicit stdout/stderr truncate and append/create
  redirection, read-only input redirection, /dev/null stdio redirection,
  normal stdio, descriptor redirection/pipeline controls, VFS
  exec/open/read/write lineage, lifecycle/status, waitpid, laststatus,
  deterministic negatives, and descriptor-backed cat/readback controls.
- not-an-issue: The explicit fd1 aliases keep the existing stdout regular-file
  redirection source labels because they are aliases for the accepted fd1
  descriptor operations, not a new descriptor class.
- not-an-issue: Pi 5 hardware proof remains outside this QEMU/substitute
  feature slice; hardwareTestLock stayed unlocked/restored and unused.
- deferred: arbitrary descriptor syntax beyond these exact fd1 aliases and
  already accepted exact fd2 forms, descriptor moves, arbitrary output paths,
  fd2 shorthand aliases, persistent storage, partial overwrite, unlink,
  rename, mkdir, directory mutation, permissions, timestamps, fsync, broad
  writable filesystem behavior, here-docs, wider shell grammar,
  multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, process accounting/concurrency, Pi 5 proof, networking, SSH, and
  phase transition.

## Evidence Map

- explicit fd1 truncate evidence:
  'tasks/evidence/2026-06-04-phase10-explicit-fd1-regular-file-redirection-core/qemu-local-shell-explicit-fd1-regular-file-redirection-smoke.log'
  records 'exec stdout 1>/tmp/stdout.txt', 'exec-redirection op=sink',
  'source-fd=0x1', 'target-path=/tmp/stdout.txt',
  'target-route=volatile-vfs:/tmp/stdout.txt',
  'source=shell-redirection-stdout-tmp-stdout', userspace TalosWrite
  provenance, waitpid, laststatus, descriptor-backed readback with
  'cat /tmp/stdout.txt', one visible 'Talos userspace stdout fixture' payload,
  'cat path=/tmp/stdout.txt bytes=0x1f
  source=volatile-vfs-descriptor-read', later normal 'exec stdout'
  restoration through 'runtime-console0/stdout', final classification
  'qemu-local-shell-explicit-fd1-regular-file-redirection-complete',
  errors=0, and PASS.
- explicit fd1 append evidence:
  the same task-owned log records 'exec stdout 1>>/tmp/stdout.txt',
  'exec-redirection op=append', 'source-fd=0x1',
  'target-path=/tmp/stdout.txt',
  'target-route=volatile-vfs:/tmp/stdout.txt',
  'source=shell-redirection-stdout-tmp-stdout-append', userspace TalosWrite
  provenance, waitpid, laststatus, descriptor-backed readback with
  'cat /tmp/stdout.txt', two visible 'Talos userspace stdout fixture'
  payloads, 'cat path=/tmp/stdout.txt bytes=0x3e
  source=volatile-vfs-descriptor-read', deterministic negatives for
  unsupported explicit fd numbers and arbitrary output paths, final
  classification, errors=0, and PASS.
- retained implicit stdout/stderr truncate-create evidence:
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-stderr-regular-file-redirection-core/qemu-local-shell-stderr-regular-file-redirection-smoke.log'.
- retained implicit stdout/stderr append and append-create evidence:
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-append-redirection-core/qemu-local-shell-stdout-regular-file-append-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-regular-file-append-redirection-core/qemu-local-shell-stderr-regular-file-append-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-regular-file-append-create-redirection-core/qemu-local-shell-stdout-regular-file-append-create-redirection-smoke.log',
  and
  'tasks/evidence/2026-06-04-phase10-regular-file-append-create-redirection-core/qemu-local-shell-stderr-regular-file-append-create-redirection-smoke.log'.
- retained read-only input redirection evidence:
  'tasks/evidence/2026-06-04-phase10-readonly-regular-file-stdin-redirection-core/qemu-local-shell-readonly-regular-file-stdin-redirection-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-dev-null-stdin-redirection-core/qemu-local-shell-dev-null-stdin-redirection-smoke.log'.
- retained /dev/null stdio redirection evidence:
  'tasks/evidence/2026-06-04-phase10-dev-null-stdout-redirection-contract-core/qemu-local-shell-dev-null-stdout-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-dev-null-stderr-redirection-smoke.log',
  and
  'tasks/evidence/2026-06-04-phase10-dev-null-stdin-redirection-core/qemu-local-shell-dev-null-stdin-redirection-smoke.log'.
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
  'tasks/evidence/2026-06-04-phase10-explicit-fd1-regular-file-redirection-core/qemu-local-shell-explicit-fd1-regular-file-redirection-smoke.log',
  and 'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log'.

## Accepted Frontier

Accepted:

- exactly 'exec stdout 1>/tmp/stdout.txt';
- exactly 'exec stdout 1>>/tmp/stdout.txt';
- the scratch output path '/tmp/stdout.txt' only;
- VFS-backed '/bin/stdout' fixture only for these explicit fd1 aliases;
- child-only fd1 rebinding to the volatile VFS regular-file descriptor path;
- truncate/create behavior for the accepted fd1 sink alias and append/create
  behavior for the accepted fd1 append alias;
- descriptor-backed userspace writes and descriptor-backed readback through
  'cat /tmp/stdout.txt';
- readback of one fixture payload for the truncate alias with
  'bytes=0x1f source=volatile-vfs-descriptor-read';
- readback of two fixture payloads for the append alias with
  'bytes=0x3e source=volatile-vfs-descriptor-read';
- shell fd1 restoration after child exit, proven by later normal stdout routed
  to runtime-console0/stdout;
- implicit stdout/stderr truncate and append/create, read-only input
  redirection, /dev/null stdio redirection, normal stdio, descriptor
  redirection/pipeline controls, VFS-backed launch, lifecycle/status,
  waitpid, laststatus, deterministic negatives, and descriptor-backed cat are
  covered by retained QEMU/substitute evidence.

Deferred:

- arbitrary descriptor syntax such as general 'N>target' and 'N>>target'
  beyond the accepted exact fd1 aliases and already accepted exact fd2 forms;
- descriptor moves, descriptor duplication expansion, and close/restore
  expansion beyond accepted exact forms;
- arbitrary output paths and arbitrary append paths;
- fd2 shorthand aliases beyond already accepted exact stderr forms;
- partial overwrite, unlink, rename, mkdir, directory mutation, permissions,
  timestamps, fsync, persistence, and broader writable filesystem semantics;
- here-docs, globbing, quoting, variables, environment-backed path behavior,
  and wider shell grammar;
- multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, process accounting/concurrency, and job control;
- Pi 5 proof, networking, SSH, and any phase transition.

## Next Step Requirement

No explicit queued follow-up remains mechanically unblocked by this closeout.
Supervisor planning is required before choosing the next bounded feature-led
task, including any broader descriptor grammar, arbitrary output paths,
process accounting/concurrency, persistent filesystem behavior, networking,
SSH, hardware proof, or phase-transition work.

## Validation

- static inspection: accepted explicit fd1 regular-file redirection
  QEMU/substitute evidence was inspected, including truncate and append
  command transcripts, descriptor routes, userspace TalosWrite provenance,
  descriptor-backed readbacks, fd1 restoration, deterministic negatives,
  waitpid, laststatus, completion markers, errors=0, and PASS lines.
- static inspection: retained implicit stdout/stderr truncate and
  append/create, read-only input redirection, /dev/null stdio redirection,
  normal stdio, descriptor redirection/pipeline, VFS/open/read/write,
  lifecycle/status, waitpid, laststatus, negative-control, and
  descriptor-backed cat evidence paths were checked for presence and
  PASS/completion markers.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final explicit fd1 regular-file redirection closeout commit recorded
in supervisor state.
