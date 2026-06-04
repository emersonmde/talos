# Phase 10 Stdout Arbitrary /tmp Output Redirection Closeout

Task: phase10-stdout-arbitrary-tmp-output-redirection-closeout-20260604
Status: accepted

## Scope

Close out the accepted stdout arbitrary volatile '/tmp' basename output
redirection frontier without adding code.

The accepted shell-visible forms are:

- stdout truncate/create: 'exec stdout >/tmp/<basename>';
- stdout append/create: 'exec stdout >>/tmp/<basename>';
- explicit fd1 truncate/create: 'exec stdout 1>/tmp/<basename>';
- explicit fd1 append/create: 'exec stdout 1>>/tmp/<basename>';
- descriptor-backed readback: 'cat /tmp/<basename>'.

The basename grammar is the conservative Phase 10 output-path policy accepted
by the core task: a non-empty basename under '/tmp/' made only of ASCII
letters, digits, '.', '_', and '-'. The route remains volatile VFS only and
does not imply persistent storage, directories, broad writable filesystem
mutation, arbitrary input paths, or broader descriptor grammar.

This closeout does not add code, run Pi 5 hardware, acquire hardwareTestLock,
add stderr arbitrary paths, add input arbitrary paths, expand descriptor moves
or arbitrary descriptor syntax, add process accounting/concurrency, networking,
SSH, or a phase transition.

## Findings

- fixed: Consolidated stdout truncate/create for a non-canonical scratch path
  with 'exec stdout >/tmp/alpha.log', target route
  'volatile-vfs:/tmp/alpha.log', userspace TalosWrite provenance,
  lifecycle/status, waitpid, laststatus, shell fd1 restoration, and
  descriptor-backed readback.
- fixed: Consolidated stdout append/create for a non-canonical scratch path
  with 'exec stdout >>/tmp/beta.out', target route
  'volatile-vfs:/tmp/beta.out', userspace TalosWrite provenance,
  lifecycle/status, waitpid, laststatus, and descriptor-backed readback.
- fixed: Consolidated explicit fd1 truncate and append aliases for arbitrary
  volatile '/tmp' basenames with 'exec stdout 1>/tmp/gamma.log' and
  'exec stdout 1>>/tmp/delta.out'.
- fixed: Confirmed each accepted redirection records child-only fd1 rebinding,
  'source-fd=0x1', matching 'target-path=/tmp/<basename>',
  matching 'target-route=volatile-vfs:/tmp/<basename>', and regular-file
  descriptor-backed cat/readback bytes.
- fixed: Confirmed deterministic negatives reject outside-/tmp paths,
  nested paths, empty basenames, unsupported explicit fd numbers, traversal,
  and the reserved stderr scratch name '/tmp/stderr.txt'.
- fixed: Retained exact stdout/stderr truncate and append/create redirection,
  explicit fd1 exact-path aliases, /dev/null redirection, read-only stdin
  redirection, normal stdio, descriptor redirection/pipeline controls, VFS
  exec/open/read/write lineage, lifecycle/status, waitpid, laststatus,
  deterministic negatives, and descriptor-backed cat/readback evidence.
- not-an-issue: The core task already added an ADR entry for the conservative
  volatile '/tmp' output basename policy; this closeout records no new policy.
- not-an-issue: Pi 5 hardware proof remains outside this QEMU/substitute
  feature slice; hardwareTestLock stayed unlocked/restored and unused.
- deferred: stderr arbitrary paths, input arbitrary paths, persistence,
  recursive directories, path traversal, broad writable filesystem mutation,
  arbitrary descriptor syntax beyond accepted fd1 aliases and already accepted
  exact fd2 forms, descriptor moves, here-docs, wider shell grammar,
  multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, process accounting/concurrency, Pi 5 proof, networking, SSH, and
  phase transition.

## Evidence Map

- stdout arbitrary-/tmp primary evidence:
  'tasks/evidence/2026-06-04-phase10-stdout-arbitrary-tmp-output-redirection-core/qemu-local-shell-stdout-arbitrary-tmp-output-redirection-smoke.log'
  records 'exec stdout >/tmp/alpha.log', 'exec stdout >>/tmp/beta.out',
  'exec stdout 1>/tmp/gamma.log', and
  'exec stdout 1>>/tmp/delta.out'.
- truncate/create readback evidence:
  the primary log records 'target-path=/tmp/alpha.log',
  'target-route=volatile-vfs:/tmp/alpha.log',
  'source=shell-redirection-stdout-tmp-stdout',
  'source=userspace-talos-write', waitpid, laststatus, and
  'cat /tmp/alpha.log' with 'source=volatile-vfs-descriptor-read'.
- append/create readback evidence:
  the primary log records 'target-path=/tmp/beta.out',
  'target-route=volatile-vfs:/tmp/beta.out',
  'source=shell-redirection-stdout-tmp-stdout-append',
  'source=userspace-talos-write', waitpid, laststatus, and
  'cat /tmp/beta.out' with 'source=volatile-vfs-descriptor-read'.
- explicit fd1 arbitrary-path evidence:
  the primary log records 'source-fd=0x0000000000000001',
  'target-path=/tmp/gamma.log',
  'target-route=volatile-vfs:/tmp/gamma.log',
  'target-path=/tmp/delta.out',
  'target-route=volatile-vfs:/tmp/delta.out', descriptor-backed readbacks,
  waitpid, laststatus, and shell fd1 restoration through a later normal
  'exec stdout' routed to runtime-console0/stdout.
- negative evidence:
  the primary log records deterministic rejection for
  'exec stdout >/var/out.txt', 'exec stdout >/tmp/nested/out.txt',
  'exec stdout >/tmp/', 'exec stdout 3>/tmp/alpha.log',
  'exec stdout >/tmp/../bad.txt', and
  'exec stdout >/tmp/stderr.txt', then final classification
  'qemu-local-shell-stdout-arbitrary-tmp-output-redirection-complete',
  errors=0, and PASS.
- retained exact output redirection evidence:
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-regular-file-redirection-core/qemu-local-shell-stderr-regular-file-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-append-redirection-core/qemu-local-shell-stdout-regular-file-append-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-regular-file-append-redirection-core/qemu-local-shell-stderr-regular-file-append-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-regular-file-append-create-redirection-core/qemu-local-shell-stdout-regular-file-append-create-redirection-smoke.log',
  and
  'tasks/evidence/2026-06-04-phase10-regular-file-append-create-redirection-core/qemu-local-shell-stderr-regular-file-append-create-redirection-smoke.log'.
- retained explicit fd1 evidence:
  'tasks/evidence/2026-06-04-phase10-explicit-fd1-regular-file-redirection-core/qemu-local-shell-explicit-fd1-regular-file-redirection-smoke.log'.
- retained /dev/null and read-only input redirection evidence:
  'tasks/evidence/2026-06-04-phase10-dev-null-stdout-redirection-contract-core/qemu-local-shell-dev-null-stdout-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-dev-null-stderr-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-dev-null-stdin-redirection-core/qemu-local-shell-dev-null-stdin-redirection-smoke.log',
  and
  'tasks/evidence/2026-06-04-phase10-readonly-regular-file-stdin-redirection-core/qemu-local-shell-readonly-regular-file-stdin-redirection-smoke.log'.
- retained normal stdio, descriptor redirection, and pipeline controls:
  'tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-close-redirection-core/qemu-local-shell-stderr-close-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log',
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log',
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log',
  and
  'tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log'.
- retained VFS exec/open/read/write, lifecycle/status, waitpid, laststatus,
  negative controls, and descriptor-backed file I/O:
  'tasks/evidence/2026-06-03-phase10-absolute-vfs-exec-dispatch-core/qemu-local-shell-absolute-vfs-exec-dispatch-smoke.log',
  'tasks/evidence/2026-06-03-phase10-vfs-exec-nonzero-status-core/qemu-local-shell-nonzero-vfs-exec-status-smoke.log',
  'tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log',
  'tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log',
  and 'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log'.

## Accepted Frontier

Accepted:

- stdout output redirection to conservative volatile '/tmp/<basename>' targets
  for '>', '>>', '1>', and '1>>';
- basenames made only of ASCII letters, digits, '.', '_', and '-';
- volatile VFS routes reported as 'target-route=volatile-vfs:/tmp/<basename>';
- VFS-backed '/bin/stdout' fixture output through inherited fd1 only;
- child-only fd1 rebinding and shell fd1 restoration after child exit;
- truncate/create and append/create behavior for accepted stdout targets;
- descriptor-backed userspace TalosWrite provenance and descriptor-backed
  readback through 'cat /tmp/<basename>';
- waitpid, laststatus, lifecycle/status, retained exact stdout/stderr
  redirection, /dev/null redirection, read-only stdin redirection, descriptor
  redirection/pipeline controls, and VFS exec/open/read/write controls.

Deferred:

- stderr arbitrary output paths and input arbitrary paths;
- persistence, recursive directories, traversal, nested paths, partial
  overwrite, unlink, rename, mkdir, directory mutation, permissions,
  timestamps, fsync, and broad writable filesystem mutation;
- arbitrary descriptor syntax beyond accepted fd1 aliases and already
  accepted exact fd2 forms, descriptor moves, and here-docs;
- globbing, quoting, variables, environment-backed path behavior, and wider
  shell grammar;
- multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, process accounting/concurrency, and job control;
- Pi 5 proof, networking, SSH, and any phase transition.

## Next Step

The stderr arbitrary-/tmp output redirection core is mechanically unblocked by
this closeout because the accepted stdout task record, roadmap entry, and ADR
now document the conservative volatile '/tmp' basename output path policy.
That next task must stay limited to stderr '2>' and '2>>' output redirection
using the same policy and QEMU/substitute evidence.

## Validation

- static inspection: accepted stdout arbitrary-/tmp QEMU/substitute evidence
  was inspected, including truncate/create, append/create, explicit fd1
  aliases, matching target paths/routes, userspace TalosWrite provenance,
  descriptor-backed readbacks, fd1 restoration, deterministic negatives,
  waitpid, laststatus, completion markers, errors=0, and PASS lines.
- static inspection: retained exact stdout/stderr truncate and append/create,
  explicit fd1, /dev/null redirection, read-only stdin redirection, normal
  stdio, descriptor redirection/pipeline controls, VFS exec/open/read/write,
  lifecycle/status, waitpid, laststatus, negative-control, and
  descriptor-backed cat evidence paths were checked for presence and
  PASS/completion markers.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final stdout arbitrary-/tmp output redirection closeout commit recorded
in supervisor state.
