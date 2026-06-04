# Phase 10 Regular-File Append-Create Redirection Core

Task: phase10-regular-file-append-create-redirection-core-20260604
Status: accepted

## Scope

Accept exact missing-file append-create forms for the VFS-backed stdout and
stderr fixtures:

- 'exec stdout >>/tmp/stdout.txt'
- 'exec stderr 2>>/tmp/stderr.txt'

Both forms start with no prior truncate/create setup in the task-owned
transcript. The child descriptor is rebound only for the launched process, the
volatile scratch file is created if missing without truncating an existing
file, the userspace fixture writes through TalosWrite, descriptor-backed cat
reads back exactly one fixture payload, and the shell descriptor is restored.

This is not arbitrary output paths, persistent storage, partial overwrite,
rename/unlink, directory creation, broad writable filesystem semantics,
arbitrary descriptor syntax, descriptor moves, here-docs, multi-stage or
concurrent pipelines, pipefail, jobs, fork, signals, async execution,
networking, SSH, Pi 5 proof, boot archive publication, or hardwareTestLock
acquisition.

## Findings

- fixed: Removed the old pre-exec requirement that
  'StdoutAppendTmpStdout' and 'StderrAppendTmpStderr' fail when the matching
  volatile scratch file does not already exist.
- fixed: Added volatile scratch-file append-open create-if-missing behavior.
  Truncate/create still resets length, while append-create only marks a
  missing scratch file as existing and preserves length for existing files.
- fixed: Added no_std unit coverage proving missing-file stdout append-create
  writes one '/bin/stdout' payload to '/tmp/stdout.txt', reads back
  'bytes=0x1f source=volatile-vfs-descriptor-read', reports waitpid and
  laststatus lifecycle evidence, and restores fd1 for normal stdout.
- fixed: Added no_std unit coverage proving missing-file stderr append-create
  writes one '/bin/stderr' payload to '/tmp/stderr.txt', reads back
  'bytes=0x1f source=volatile-vfs-descriptor-read', reports waitpid and
  laststatus lifecycle evidence, restores fd2 for normal stderr, and keeps
  normal stdout distinct.
- fixed: Added dedicated QEMU/substitute boot scenarios and smoke wrappers for
  stdout and stderr append-create so the retained transcripts prove no prior
  truncate/create command occurred.
- fixed: Preserved deterministic negatives for arbitrary append paths and
  cross-stream scratch path mixups:
  'exec stdout >>/tmp/other.txt',
  'exec stdout >>/tmp/stderr.txt',
  'exec stderr 2>>/tmp/other.txt', and
  'exec stderr 2>>/tmp/stdout.txt'.
- fixed: Retained controls for setup-then-append, truncate/create output
  redirection, input redirection, /dev/null redirection, descriptor
  redirection/pipeline controls, normal stdio, lifecycle/status, waitpid,
  laststatus, VFS exec/open/read/write lineage, and descriptor-backed cat.
- deferred: arbitrary output paths, persistent storage, broad writable
  filesystem mutation, arbitrary descriptor syntax beyond the exact fd1/fd2
  forms, descriptor moves, here-docs, multi-stage/concurrent pipelines,
  pipefail, jobs, fork, signals, async execution, process
  accounting/concurrency, Pi 5 proof, networking, SSH, and phase transition.

## Evidence

- QEMU/substitute stdout append-create smoke:
  'tasks/evidence/2026-06-04-phase10-regular-file-append-create-redirection-core/qemu-local-shell-stdout-regular-file-append-create-redirection-smoke.log'
  records 'exec stdout >>/tmp/stdout.txt' as the first scratch-file mutation,
  'exec-redirection op=append', 'source-fd=0x1',
  'target-path=/tmp/stdout.txt',
  'target-route=volatile-vfs:/tmp/stdout.txt',
  'source=shell-redirection-stdout-tmp-stdout-append',
  userspace TalosWrite provenance, waitpid, laststatus, descriptor-backed
  'cat /tmp/stdout.txt' readback with 'bytes=0x1f
  source=volatile-vfs-descriptor-read', normal 'exec stdout' restoration
  through 'runtime-console0/stdout', arbitrary-path and cross-stream
  negatives, descriptor-backed 'cat /etc/banner.txt', final
  'qemu-local-shell-stdout-regular-file-append-create-redirection-complete',
  errors=0, and PASS.
- QEMU/substitute stderr append-create smoke:
  'tasks/evidence/2026-06-04-phase10-regular-file-append-create-redirection-core/qemu-local-shell-stderr-regular-file-append-create-redirection-smoke.log'
  records 'exec stderr 2>>/tmp/stderr.txt' as the first scratch-file mutation,
  'exec-redirection op=append', 'source-fd=0x2',
  'target-path=/tmp/stderr.txt',
  'target-route=volatile-vfs:/tmp/stderr.txt',
  'source=shell-redirection-stderr-tmp-stderr-append',
  userspace TalosWrite provenance, waitpid, laststatus, descriptor-backed
  'cat /tmp/stderr.txt' readback with 'bytes=0x1f
  source=volatile-vfs-descriptor-read', normal 'exec stderr' restoration
  through 'runtime-console0/stderr', normal 'exec stdout' distinct-stream
  control, arbitrary-path and cross-stream negatives, descriptor-backed
  'cat /etc/banner.txt', final
  'qemu-local-shell-stderr-regular-file-append-create-redirection-complete',
  errors=0, and PASS.
- Retained setup-then-append controls:
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-append-redirection-core/qemu-local-shell-stdout-regular-file-append-redirection-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-stderr-regular-file-append-redirection-core/qemu-local-shell-stderr-regular-file-append-redirection-smoke.log'.
- Retained truncate/create output redirection controls:
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-stderr-regular-file-redirection-core/qemu-local-shell-stderr-regular-file-redirection-smoke.log'.
- Retained input and /dev/null controls:
  'tasks/evidence/2026-06-04-phase10-readonly-regular-file-stdin-redirection-core/qemu-local-shell-readonly-regular-file-stdin-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-dev-null-stdin-redirection-core/qemu-local-shell-dev-null-stdin-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-dev-null-stdout-redirection-contract-core/qemu-local-shell-dev-null-stdout-redirection-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-dev-null-stderr-redirection-smoke.log'.
- Retained descriptor redirection and pipeline controls:
  'tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-close-redirection-core/qemu-local-shell-stderr-close-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log',
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log',
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log'.
- Retained VFS exec/open/read/write, lifecycle/status, waitpid, laststatus,
  negative-control, and descriptor-backed cat controls:
  'tasks/evidence/2026-06-03-phase10-absolute-vfs-exec-dispatch-core/qemu-local-shell-absolute-vfs-exec-dispatch-smoke.log',
  'tasks/evidence/2026-06-03-phase10-vfs-exec-nonzero-status-core/qemu-local-shell-nonzero-vfs-exec-status-smoke.log',
  'tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log',
  'tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log',
  'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log', and
  'tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log'.

## Accepted Frontier

Accepted:

- exactly 'exec stdout >>/tmp/stdout.txt' with no prior setup in the
  task-owned transcript;
- exactly 'exec stderr 2>>/tmp/stderr.txt' with no prior setup in the
  task-owned transcript;
- scratch paths '/tmp/stdout.txt' and '/tmp/stderr.txt' only;
- VFS-backed '/bin/stdout' and '/bin/stderr' fixtures only for these forms;
- child-only fd1/fd2 rebinding to volatile VFS regular-file descriptors;
- append-create for missing volatile scratch files and append-without-truncate
  for existing scratch files;
- descriptor-backed userspace writes and descriptor-backed readback through
  'cat /tmp/stdout.txt' and 'cat /tmp/stderr.txt';
- readback of one fixture payload per missing-file append-create transcript
  with 'bytes=0x1f source=volatile-vfs-descriptor-read';
- shell fd1/fd2 restoration after child exit and stderr evidence that normal
  stdout remains distinct.

Deferred:

- arbitrary output paths and arbitrary append paths;
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

## Validation

- fmt/lint/typecheck: 'cargo fmt --all -- --check' passed.
- unit tests: 'cargo -Zjson-target-spec test --quiet' passed.
- QEMU/substitute:
  'scripts/qemu-local-shell-stdout-regular-file-append-create-redirection-smoke.sh'
  passed with retained PASS log.
- QEMU/substitute:
  'scripts/qemu-local-shell-stderr-regular-file-append-create-redirection-smoke.sh'
  passed with retained PASS log.
- retained-control static inspection: append-after-setup, truncate/create,
  input redirection, /dev/null, descriptor redirection/pipeline,
  lifecycle/status, waitpid, laststatus, and descriptor-backed cat evidence
  paths were checked for presence and retained PASS/completion markers where
  applicable.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: accepted implementation and evidence pending final validation/commit;
final SHA will be recorded in durable supervisor state.
