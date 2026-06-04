# Phase 10 Regular-File Append-Create Redirection Closeout

Task: phase10-regular-file-append-create-redirection-closeout-20260604
Status: accepted

## Scope

Close out the accepted missing-file append-create redirection frontier without
adding code.

The accepted shell-visible append-create forms are exactly:

- stdout append-create: 'exec stdout >>/tmp/stdout.txt';
- stdout readback: 'cat /tmp/stdout.txt';
- stderr append-create: 'exec stderr 2>>/tmp/stderr.txt';
- stderr readback: 'cat /tmp/stderr.txt'.

Both transcripts start with no prior truncate/create setup for the matching
scratch file. The child descriptor is rebound only for the launched process,
the volatile scratch file is created if missing, existing append semantics
remain non-truncating, the VFS-backed userspace fixture writes through
TalosWrite, descriptor-backed cat reads back exactly one fixture payload, and
the shell descriptor is restored.

This closeout does not add code and does not expand into arbitrary output
paths, persistent storage, broad writable filesystem mutation, arbitrary
descriptor syntax, descriptor moves, multi-stage or concurrent pipelines,
process accounting/concurrency, networking, SSH, Pi 5 proof, hardwareTestLock
acquisition, or a phase transition.

## Findings

- fixed: Consolidated the accepted missing-file append-create forms as exact
  scratch paths '/tmp/stdout.txt' for stdout and '/tmp/stderr.txt' for stderr.
- fixed: Confirmed append-create no longer requires a prior setup/truncate
  command in the task-owned transcripts, while existing setup-then-append
  behavior still appends without truncating existing volatile scratch-file
  contents.
- fixed: Confirmed stdout append-create uses child-only fd1 regular-file
  rebinding with 'op=append', 'target-path=/tmp/stdout.txt',
  'target-route=volatile-vfs:/tmp/stdout.txt',
  'source=shell-redirection-stdout-tmp-stdout-append',
  userspace TalosWrite provenance, lifecycle/status, waitpid, laststatus,
  shell fd1 restoration, and descriptor-backed readback of one stdout fixture
  payload with 'bytes=0x1f source=volatile-vfs-descriptor-read'.
- fixed: Confirmed stderr append-create uses child-only fd2 regular-file
  rebinding with 'op=append', 'target-path=/tmp/stderr.txt',
  'target-route=volatile-vfs:/tmp/stderr.txt',
  'source=shell-redirection-stderr-tmp-stderr-append',
  userspace TalosWrite provenance, lifecycle/status, waitpid, laststatus,
  shell fd2 restoration, distinct normal stdout behavior, and
  descriptor-backed readback of one stderr fixture payload with
  'bytes=0x1f source=volatile-vfs-descriptor-read'.
- fixed: Retained setup-then-append, truncate/create output redirection,
  read-only input redirection, /dev/null redirection, normal stdio,
  descriptor redirection/pipeline controls, VFS exec/open/read/write lineage,
  lifecycle/status, waitpid, laststatus, deterministic negatives, and
  descriptor-backed cat/readback controls.
- not-an-issue: Pi 5 hardware proof remains outside this QEMU/substitute
  feature slice; hardwareTestLock stayed unlocked/restored and unused.
- deferred: arbitrary output paths, stdout/stderr scratch path mixups beyond
  accepted negatives, persistence, partial overwrite, unlink, rename, mkdir,
  directory mutation, permissions, timestamps, fsync, broad writable
  filesystem behavior, arbitrary descriptor syntax, descriptor moves,
  here-docs, wider shell grammar, multi-stage or concurrent pipelines,
  pipefail, jobs, async execution, fork, signals, process
  accounting/concurrency, Pi 5 proof, networking, SSH, and phase transition.

## Evidence Map

- stdout append-create evidence:
  'tasks/evidence/2026-06-04-phase10-regular-file-append-create-redirection-core/qemu-local-shell-stdout-regular-file-append-create-redirection-smoke.log'
  records 'exec stdout >>/tmp/stdout.txt' as the first scratch-file mutation,
  'exec-redirection op=append', 'source-fd=0x1',
  'target-path=/tmp/stdout.txt',
  'target-route=volatile-vfs:/tmp/stdout.txt',
  'source=shell-redirection-stdout-tmp-stdout-append',
  userspace TalosWrite provenance, waitpid, laststatus, descriptor-backed
  readback with 'cat /tmp/stdout.txt', one visible 'Talos userspace stdout
  fixture' payload, 'cat path=/tmp/stdout.txt bytes=0x1f
  source=volatile-vfs-descriptor-read', later normal 'exec stdout' restoration
  through 'runtime-console0/stdout', arbitrary-path and cross-stream
  negatives, descriptor-backed 'cat /etc/banner.txt', final classification
  'qemu-local-shell-stdout-regular-file-append-create-redirection-complete',
  errors=0, and PASS.
- stderr append-create evidence:
  'tasks/evidence/2026-06-04-phase10-regular-file-append-create-redirection-core/qemu-local-shell-stderr-regular-file-append-create-redirection-smoke.log'
  records 'exec stderr 2>>/tmp/stderr.txt' as the first scratch-file
  mutation, 'exec-redirection op=append', 'source-fd=0x2',
  'target-path=/tmp/stderr.txt',
  'target-route=volatile-vfs:/tmp/stderr.txt',
  'source=shell-redirection-stderr-tmp-stderr-append',
  userspace TalosWrite provenance, waitpid, laststatus, descriptor-backed
  readback with 'cat /tmp/stderr.txt', one visible 'Talos userspace stderr
  fixture' payload, 'cat path=/tmp/stderr.txt bytes=0x1f
  source=volatile-vfs-descriptor-read', later normal 'exec stderr' restoration
  through 'runtime-console0/stderr', later normal 'exec stdout' distinct-stream
  control, arbitrary-path and cross-stream negatives, descriptor-backed
  'cat /etc/banner.txt', final classification
  'qemu-local-shell-stderr-regular-file-append-create-redirection-complete',
  errors=0, and PASS.
- retained setup-then-append controls:
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-append-redirection-core/qemu-local-shell-stdout-regular-file-append-redirection-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-stderr-regular-file-append-redirection-core/qemu-local-shell-stderr-regular-file-append-redirection-smoke.log'.
- retained truncate/create output redirection controls:
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-stderr-regular-file-redirection-core/qemu-local-shell-stderr-regular-file-redirection-smoke.log'.
- retained read-only input redirection evidence:
  'tasks/evidence/2026-06-04-phase10-readonly-regular-file-stdin-redirection-core/qemu-local-shell-readonly-regular-file-stdin-redirection-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-dev-null-stdin-redirection-core/qemu-local-shell-dev-null-stdin-redirection-smoke.log'.
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
  'tasks/evidence/2026-06-04-phase10-regular-file-append-create-redirection-core/qemu-local-shell-stdout-regular-file-append-create-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-regular-file-append-create-redirection-core/qemu-local-shell-stderr-regular-file-append-create-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-append-redirection-core/qemu-local-shell-stdout-regular-file-append-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-regular-file-append-redirection-core/qemu-local-shell-stderr-regular-file-append-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-regular-file-redirection-core/qemu-local-shell-stderr-regular-file-redirection-smoke.log', and
  'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log'.

## Accepted Frontier

Accepted:

- exactly 'exec stdout >>/tmp/stdout.txt' with no prior setup/truncate command
  in the task-owned transcript;
- exactly 'exec stderr 2>>/tmp/stderr.txt' with no prior setup/truncate command
  in the task-owned transcript;
- scratch append-create paths '/tmp/stdout.txt' and '/tmp/stderr.txt' only;
- VFS-backed '/bin/stdout' and '/bin/stderr' fixtures only for these
  append-create forms;
- child-only fd1/fd2 rebinding to volatile VFS regular-file descriptors;
- create-if-missing behavior for the accepted volatile scratch files and
  append-without-truncate behavior for existing scratch files;
- descriptor-backed userspace writes and descriptor-backed readback through
  'cat /tmp/stdout.txt' and 'cat /tmp/stderr.txt';
- readback of one fixture payload per append-create transcript with
  'bytes=0x1f source=volatile-vfs-descriptor-read';
- shell fd1/fd2 restoration after child exit and stderr evidence that normal
  stdout remains distinct;
- setup-then-append, truncate/create, read-only input redirection, /dev/null
  redirection, normal stdio, descriptor redirection/pipeline controls,
  VFS-backed launch, lifecycle/status, waitpid, laststatus, deterministic
  negatives, and descriptor-backed cat/readback are covered by retained
  QEMU/substitute evidence.

Deferred:

- arbitrary output paths and arbitrary append paths;
- stdout-to-stderr and stderr-to-stdout append path mixups beyond accepted
  negative controls;
- partial overwrite, unlink, rename, mkdir, directory mutation, permissions,
  timestamps, fsync, persistence, and broader writable filesystem semantics;
- arbitrary descriptor syntax such as 'N>>target' and 'N>target' beyond
  already accepted exact fd1/fd2 forms, descriptor moves, and close/restore
  expansion beyond accepted exact forms;
- here-docs, globbing, quoting, variables, environment-backed path behavior,
  and wider shell grammar;
- multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, process accounting/concurrency, and job control;
- Pi 5 proof, networking, SSH, and any phase transition.

## Next Step

The explicit fd1 regular-file redirection core is mechanically unblocked by
this closeout because the accepted append-create evidence and retained
stdout/stderr controls satisfy its dependencies. That next task must stay
limited to exact '1>/tmp/stdout.txt' and '1>>/tmp/stdout.txt' aliases for the
VFS-backed '/bin/stdout' fixture.

## Validation

- static inspection: accepted stdout and stderr append-create QEMU/substitute
  evidence was inspected, including no-prior-setup command transcripts,
  descriptor routes, userspace TalosWrite provenance, descriptor-backed
  readbacks, fd1/fd2 restoration, distinct stdout behavior, deterministic
  negatives, waitpid, laststatus, completion markers, errors=0, and PASS
  lines.
- static inspection: retained setup-then-append, truncate/create, read-only
  input redirection, /dev/null redirection, normal stdio, descriptor
  redirection/pipeline, VFS/open/read/write, lifecycle/status, waitpid,
  laststatus, negative-control, and descriptor-backed cat evidence paths were
  checked for presence and PASS/completion markers.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final regular-file append-create redirection closeout commit recorded
in supervisor state.
