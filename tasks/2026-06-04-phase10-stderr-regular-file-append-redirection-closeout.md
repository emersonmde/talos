# Phase 10 Stderr Regular-File Append Redirection Closeout

Task: phase10-stderr-regular-file-append-redirection-closeout-20260604
Status: accepted

## Scope

Close out the accepted stderr append redirection slice without adding code.

The accepted shell-visible append sequence is exactly:

- setup/create-truncate: 'exec stderr 2>/tmp/stderr.txt';
- append: 'exec stderr 2>>/tmp/stderr.txt';
- readback: 'cat /tmp/stderr.txt'.

Both exec commands launch the VFS-backed '/bin/stderr' fixture. The setup
command preserves the prior volatile create/truncate behavior for
'/tmp/stderr.txt'. The append command rebinds only child fd2 to the same
volatile regular-file descriptor without truncating existing contents, writes
the second stderr fixture through userspace TalosWrite, restores shell fd2, and
leaves descriptor-backed cat able to read two fixture payloads in order.
Normal stdout remains distinct after the stderr append sequence.

This closeout does not add code and does not expand into missing-file append
create, stdout-to-stderr path mixups, arbitrary append paths, persistent
storage, broad writable filesystem mutation, broader descriptor grammar,
descriptor moves, here-docs, multi-stage or concurrent pipelines, jobs, fork,
signals, async execution, process accounting/concurrency, networking, SSH,
Pi 5 proof, or a phase transition.

## Findings

- fixed: Consolidated the accepted stderr append sequence as requiring the
  prior accepted truncate/create setup before
  'exec stderr 2>>/tmp/stderr.txt'.
- fixed: Confirmed the append command is limited to '/tmp/stderr.txt' and the
  VFS-backed '/bin/stderr' fixture; append to '/tmp/other.txt' remains a
  deterministic 'exec-invalid-path' negative.
- fixed: Confirmed stdout-to-stderr path mixups remain rejected:
  'exec stdout >/tmp/stderr.txt' and 'exec stdout >>/tmp/stdout.txt' are
  deterministic negatives in the task-owned smoke.
- fixed: Confirmed stderr append uses child-only fd2 regular-file rebinding
  with 'op=append', 'target-path=/tmp/stderr.txt',
  'target-stream=regular-file',
  'target-route=volatile-vfs:/tmp/stderr.txt',
  'source=shell-redirection-stderr-tmp-stderr-append', and
  'shell-restored=true'.
- fixed: Confirmed the second payload is written through
  'source=userspace-talos-write' and read back through
  'source=volatile-vfs-descriptor-read' with 'bytes=0x3e', proving two stderr
  fixture payloads in order rather than overwritten contents.
- fixed: Confirmed later normal 'exec stderr' routes fd2 back to
  'runtime-console0/stderr' and later normal 'exec stdout' routes fd1 through
  'runtime-console0/stdout', preserving stream separation and shell
  restoration claims.
- fixed: Retained stdout append/truncate, stderr truncate, read-only input
  redirection, /dev/null redirection, normal stdio, descriptor
  redirection/pipeline, VFS exec/open/read/write, lifecycle/status, waitpid,
  laststatus, negative controls, and descriptor-backed cat controls.
- fixed: Updated the roadmap with the stderr append closeout boundary and kept
  the next append frontier closeout mechanically gated on this accepted
  closeout.
- deferred: append-create for missing files, arbitrary append paths,
  stdout-to-stderr and stderr-to-stdout append mixups beyond accepted controls,
  persistence, partial overwrite, directory mutation, metadata, permissions,
  fsync, broad writable filesystem behavior, arbitrary descriptor syntax,
  descriptor moves, here-docs, wider shell grammar, multi-stage or concurrent
  pipelines, pipefail, jobs, async execution, fork, signals, process
  accounting/concurrency, Pi 5 proof, networking, SSH, and phase transition.

## Evidence Map

- stderr append redirection evidence:
  'tasks/evidence/2026-06-04-phase10-stderr-regular-file-append-redirection-core/qemu-local-shell-stderr-regular-file-append-redirection-smoke.log'
  records setup command 'exec stderr 2>/tmp/stderr.txt' with
  'exec-redirection op=sink ... target-path=/tmp/stderr.txt ...
  target-stream=regular-file target-route=volatile-vfs:/tmp/stderr.txt
  child-only=true shell-restored=true', then append command
  'exec stderr 2>>/tmp/stderr.txt' with 'fd2=regular-file',
  'exec-redirection op=append ... target-path=/tmp/stderr.txt ...
  target-stream=regular-file target-route=volatile-vfs:/tmp/stderr.txt
  child-only=true shell-restored=true
  source=shell-redirection-stderr-tmp-stderr-append',
  'exec-stderr ... bytes=0x1f return=0x1f stream=regular-file
  route=volatile-vfs:/tmp/stderr.txt source=userspace-talos-write',
  lifecycle/status, waitpid, laststatus, descriptor-backed readback with
  'cat /tmp/stderr.txt', two visible 'Talos userspace stderr fixture'
  payloads, 'cat path=/tmp/stderr.txt bytes=0x3e
  source=volatile-vfs-descriptor-read', final classification
  'qemu-local-shell-stderr-regular-file-append-redirection-complete',
  errors=0, and PASS.
- shell fd2 restoration and distinct stdout controls:
  the stderr append evidence records a later normal 'exec stderr' with
  'fd2=stdio-output' and 'route=runtime-console0/stderr', followed by normal
  'exec stdout' with 'fd1=stdio-output' and
  'route=runtime-console0/stdout'.
- task-owned deterministic negatives:
  the stderr append evidence rejects 'exec stdout >>/tmp/stdout.txt',
  'exec stderr 2>>/tmp/other.txt', and 'exec stdout >/tmp/stderr.txt' with
  'exec-invalid-path', preserving the arbitrary-path and stream-mix
  boundaries for this slice.
- retained stdout append and truncate/create evidence:
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-append-redirection-core/qemu-local-shell-stdout-regular-file-append-redirection-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log'.
- retained stderr truncate/create evidence:
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
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-regular-file-redirection-core/qemu-local-shell-stderr-regular-file-redirection-smoke.log', and
  'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log'.

## Accepted Frontier

Accepted:

- exactly 'exec stderr 2>/tmp/stderr.txt' followed later by exactly
  'exec stderr 2>>/tmp/stderr.txt';
- setup/truncate-create is required before append in the accepted evidence;
- scratch append path '/tmp/stderr.txt' only;
- VFS-backed '/bin/stderr' fixture only for this append form;
- child-only fd2 rebinding to the volatile VFS regular-file descriptor;
- append without truncating existing '/tmp/stderr.txt' contents;
- descriptor-backed write from the userspace stderr fixture and readback
  through 'cat /tmp/stderr.txt';
- readback of two 'Talos userspace stderr fixture' payloads with
  'bytes=0x3e source=volatile-vfs-descriptor-read';
- shell fd2 restoration after child exit and distinct stdout behavior after
  the append sequence;
- stdout append/truncate, stderr truncate/create, read-only input redirection,
  /dev/null redirection, normal stdio, descriptor redirection/pipeline
  controls, VFS-backed launch, lifecycle/status, waitpid, laststatus,
  deterministic negatives, and descriptor-backed cat/readback are covered by
  retained QEMU/substitute evidence.

Deferred:

- append-create for missing files and append to arbitrary paths;
- stdout-to-stderr and stderr-to-stdout append path mixups beyond the accepted
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

The queued regular-file append frontier closeout remains mechanically
unblocked by this closeout: it is in the same Phase 10 milestone, has explicit
acceptance criteria and validation gates, depends only on accepted stdout and
stderr append evidence, and must stay a docs/evidence checkpoint. After that
frontier closeout, explicit supervisor planning is required before any broader
descriptor grammar, arbitrary path, process accounting/concurrency, or phase
transition work.

## Validation

- static inspection: accepted stderr append task record and QEMU/substitute
  evidence were inspected, including setup/truncate, append, descriptor route,
  userspace TalosWrite provenance, descriptor-backed readback, fd2
  restoration, distinct stdout behavior, deterministic negatives, waitpid,
  laststatus, and PASS lines.
- static inspection: retained stdout append/truncate, stderr truncate,
  read-only input redirection, /dev/null redirection, normal stdio, descriptor
  redirection/pipeline, VFS/open/read/write, lifecycle/status, waitpid,
  laststatus, negative-control, and descriptor-backed cat evidence paths were
  checked for presence and PASS/completion markers.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final stderr regular-file append redirection closeout commit recorded
in supervisor state.
