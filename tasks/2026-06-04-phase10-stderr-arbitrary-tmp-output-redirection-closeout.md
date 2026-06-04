# Phase 10 Stderr Arbitrary /tmp Output Redirection Closeout

Task: phase10-stderr-arbitrary-tmp-output-redirection-closeout-20260604
Status: accepted

## Scope

Close out the accepted stderr arbitrary volatile '/tmp' basename output
redirection frontier without adding code.

The accepted shell-visible forms are:

- stderr truncate/create: 'exec stderr 2>/tmp/<basename>';
- stderr append/create: 'exec stderr 2>>/tmp/<basename>';
- descriptor-backed readback: 'cat /tmp/<basename>'.

The basename grammar is the conservative Phase 10 output-path policy accepted
by the stdout arbitrary-/tmp core and reused by the stderr core: a non-empty
basename under '/tmp/' made only of ASCII letters, digits, '.', '_', and '-'.
The route remains volatile VFS only and does not imply persistent storage,
directories, broad writable filesystem mutation, arbitrary input paths,
descriptor moves, or broader descriptor grammar.

This closeout does not add code, run Pi 5 hardware, acquire hardwareTestLock,
add input arbitrary paths, expand descriptor moves or arbitrary descriptor
syntax, add process accounting/concurrency, networking, SSH, or a phase
transition.

## Findings

- fixed: Consolidated stderr truncate/create for a non-canonical scratch path
  with 'exec stderr 2>/tmp/omega.err', target route
  'volatile-vfs:/tmp/omega.err', userspace TalosWrite provenance,
  lifecycle/status, waitpid, laststatus, shell fd2 restoration, distinct
  stdout behavior, and descriptor-backed readback.
- fixed: Consolidated stderr append/create for a non-canonical scratch path
  with 'exec stderr 2>>/tmp/theta.log', target route
  'volatile-vfs:/tmp/theta.log', userspace TalosWrite provenance,
  lifecycle/status, waitpid, laststatus, and descriptor-backed readback.
- fixed: Confirmed each accepted stderr redirection records child-only fd2
  rebinding, 'source-fd=0x2', matching 'target-path=/tmp/<basename>',
  matching 'target-route=volatile-vfs:/tmp/<basename>', and regular-file
  descriptor-backed cat/readback bytes.
- fixed: Confirmed deterministic negatives reject outside-/tmp paths, nested
  paths, empty basenames, unsupported explicit fd numbers, traversal, the
  reserved stdout scratch name '/tmp/stdout.txt', and fd2 shorthand output
  redirection without explicit '2>'.
- fixed: Retained stdout arbitrary-/tmp output controls, exact stdout/stderr
  truncate and append/create redirection, explicit fd1 exact-path aliases,
  /dev/null redirection, read-only stdin redirection, normal stdio,
  descriptor redirection/pipeline controls, VFS exec/open/read/write lineage,
  lifecycle/status, waitpid, laststatus, deterministic negatives, and
  descriptor-backed cat/readback evidence.
- not-an-issue: The stderr slice reuses the already accepted conservative
  volatile '/tmp' output basename policy; this closeout records no new ADR or
  policy change.
- not-an-issue: Pi 5 hardware proof remains outside this QEMU/substitute
  feature slice; hardwareTestLock stayed unlocked/restored and unused.
- deferred: input arbitrary paths, persistence, recursive directories, path
  traversal, broad writable filesystem mutation, arbitrary descriptor syntax
  beyond accepted forms, descriptor moves, here-docs, wider shell grammar,
  multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, process accounting/concurrency, Pi 5 proof, networking, SSH, and
  phase transition.

## Evidence Map

- stderr arbitrary-/tmp primary evidence:
  'tasks/evidence/2026-06-04-phase10-stderr-arbitrary-tmp-output-redirection-core/qemu-local-shell-stderr-arbitrary-tmp-output-redirection-smoke.log'
  records 'exec stderr 2>/tmp/omega.err' and
  'exec stderr 2>>/tmp/theta.log'.
- truncate/create readback evidence:
  the primary log records 'target-path=/tmp/omega.err',
  'target-route=volatile-vfs:/tmp/omega.err',
  'source=shell-redirection-stderr-tmp-stderr',
  'source=userspace-talos-write', waitpid, laststatus, and
  'cat /tmp/omega.err' with 'source=volatile-vfs-descriptor-read'.
- append/create readback evidence:
  the primary log records 'target-path=/tmp/theta.log',
  'target-route=volatile-vfs:/tmp/theta.log',
  'source=shell-redirection-stderr-tmp-stderr-append',
  'source=userspace-talos-write', waitpid, laststatus, and
  'cat /tmp/theta.log' with 'source=volatile-vfs-descriptor-read'.
- restoration and stdout-separation evidence:
  the primary log records shell fd2 restoration through a later normal
  'exec stderr' routed to 'runtime-console0/stderr' and a distinct normal
  'exec stdout' routed to 'runtime-console0/stdout'.
- negative evidence:
  the primary log records deterministic rejection for
  'exec stderr 2>/var/err.txt', 'exec stderr 2>/tmp/n/e',
  'exec stderr 2>/tmp/', 'exec stderr 3>/tmp/omega.err',
  'exec stderr 2>/tmp/../bad.txt', 'exec stderr 2>/tmp/stdout.txt', and
  'exec stderr >/tmp/misbound.err', then final classification
  'qemu-local-shell-stderr-arbitrary-tmp-output-redirection-complete',
  errors=0, and PASS.
- retained stdout arbitrary-/tmp evidence:
  'tasks/evidence/2026-06-04-phase10-stdout-arbitrary-tmp-output-redirection-core/qemu-local-shell-stdout-arbitrary-tmp-output-redirection-smoke.log'.
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

- stderr output redirection to conservative volatile '/tmp/<basename>' targets
  for '2>' and '2>>';
- basenames made only of ASCII letters, digits, '.', '_', and '-';
- volatile VFS routes reported as 'target-route=volatile-vfs:/tmp/<basename>';
- VFS-backed '/bin/stderr' fixture output through inherited fd2 only;
- child-only fd2 rebinding and shell fd2 restoration after child exit;
- truncate/create and append/create behavior for accepted stderr targets;
- descriptor-backed userspace TalosWrite provenance and descriptor-backed
  readback through 'cat /tmp/<basename>';
- waitpid, laststatus, lifecycle/status, retained stdout arbitrary-/tmp
  redirection, exact stdout/stderr redirection, /dev/null redirection,
  read-only stdin redirection, descriptor redirection/pipeline controls, and
  VFS exec/open/read/write controls.

Deferred:

- arbitrary input paths;
- persistence, recursive directories, traversal, nested paths, partial
  overwrite, unlink, rename, mkdir, directory mutation, permissions,
  timestamps, fsync, and broad writable filesystem mutation;
- arbitrary descriptor syntax beyond accepted fd1/fd2 forms, descriptor moves,
  and here-docs;
- globbing, quoting, variables, environment-backed path behavior, and wider
  shell grammar;
- multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, process accounting/concurrency, and job control;
- Pi 5 proof, networking, SSH, and any phase transition.

## Next Step

The arbitrary '/tmp' output redirection frontier closeout is mechanically
unblocked by this closeout because accepted stdout and stderr volatile
'/tmp/<basename>' output evidence is now recorded. That next task must remain
docs/evidence reconciliation only and must require supervisor planning before
any broader capability after the frontier closeout.

## Validation

- static inspection: accepted stderr arbitrary-/tmp QEMU/substitute evidence
  was inspected, including truncate/create, append/create, matching target
  paths/routes, userspace TalosWrite provenance, descriptor-backed readbacks,
  fd2 restoration, distinct stdout behavior, deterministic negatives, waitpid,
  laststatus, completion markers, errors=0, and PASS lines.
- static inspection: retained stdout arbitrary-/tmp, exact stdout/stderr
  truncate and append/create, explicit fd1, /dev/null redirection, read-only
  stdin redirection, normal stdio, descriptor redirection/pipeline controls,
  VFS exec/open/read/write, lifecycle/status, waitpid, laststatus,
  negative-control, and descriptor-backed cat evidence paths were checked for
  presence and PASS/completion markers.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final stderr arbitrary-/tmp output redirection closeout commit recorded
in supervisor state.
