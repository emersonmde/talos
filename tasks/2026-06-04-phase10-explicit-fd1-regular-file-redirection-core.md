# Phase 10 Explicit Fd1 Regular-File Redirection Core

Task: phase10-explicit-fd1-regular-file-redirection-core-20260604
Status: accepted

## Scope

Accept exact shell-visible fd1 aliases for the VFS-backed stdout regular-file
redirection path:

- 'exec stdout 1>/tmp/stdout.txt'
- 'exec stdout 1>>/tmp/stdout.txt'

Both aliases reuse the accepted volatile VFS scratch target
'/tmp/stdout.txt'. The truncate form creates/truncates through the existing
regular-file sink path, the append form appends through the existing
append/create path, the child descriptor is rebound only for the launched
process, userspace writes carry TalosWrite provenance, descriptor-backed cat
reads back the scratch file, and shell fd1 is restored afterward.

This is not arbitrary fd redirection, arbitrary output paths, fd2 grammar
expansion, descriptor moves, persistent storage, broader writable filesystem
semantics, stdin write redirection, here-docs, multi-stage/concurrent
pipelines, pipefail, jobs, fork, signals, async execution, process
accounting/concurrency, networking, SSH, Pi 5 proof, boot archive publication,
or hardwareTestLock acquisition.

## Findings

- fixed: Added exact parser aliases '1>/tmp/stdout.txt' and
  '1>>/tmp/stdout.txt', mapping them to the already accepted fd1
  truncate/create and append/create redirection implementations.
- fixed: Added no_std unit coverage proving the explicit fd1 truncate alias
  writes one '/bin/stdout' payload to '/tmp/stdout.txt', reports waitpid and
  laststatus, reads back 'bytes=0x1f source=volatile-vfs-descriptor-read', and
  restores normal stdout.
- fixed: Added no_std unit coverage proving the explicit fd1 append alias
  appends a second payload to the same volatile scratch file, reads back
  'bytes=0x3e source=volatile-vfs-descriptor-read', and restores normal
  stdout.
- fixed: Added deterministic unit and QEMU/substitute negatives for
  unsupported explicit fd number '3>/tmp/stdout.txt' and arbitrary path
  '1>/tmp/other.txt'. Unit coverage also checks '1>>/tmp/other.txt'.
- fixed: Added a dedicated QEMU/substitute smoke wrapper and retained
  task-owned transcript for the explicit fd1 forms.
- not-an-issue: Redirection records keep the existing
  'shell-redirection-stdout-tmp-stdout' and
  'shell-redirection-stdout-tmp-stdout-append' source labels because the
  explicit fd1 forms are grammar aliases for the same fd1-backed descriptor
  operations, not a new descriptor class.
- deferred: arbitrary descriptors, arbitrary output paths, fd2 alias
  expansion beyond accepted forms, descriptor moves, persistent storage, broad
  writable filesystem semantics, here-docs, multi-stage/concurrent pipelines,
  pipefail, jobs, fork, signals, async execution, process
  accounting/concurrency, Pi 5 proof, networking, SSH, and phase transition.

## Evidence

- QEMU/substitute explicit fd1 regular-file redirection smoke:
  'tasks/evidence/2026-06-04-phase10-explicit-fd1-regular-file-redirection-core/qemu-local-shell-explicit-fd1-regular-file-redirection-smoke.log'
  records 'exec stdout 1>/tmp/stdout.txt' with
  'exec-redirection op=sink', 'source-fd=0x1',
  'target-path=/tmp/stdout.txt',
  'target-route=volatile-vfs:/tmp/stdout.txt', userspace TalosWrite
  provenance, waitpid, laststatus, descriptor-backed 'cat /tmp/stdout.txt'
  readback with 'bytes=0x1f source=volatile-vfs-descriptor-read', and normal
  'exec stdout' restoration through 'runtime-console0/stdout'.
- The same transcript records 'exec stdout 1>>/tmp/stdout.txt' with
  'exec-redirection op=append', fd1 regular-file descriptor rebinding,
  userspace TalosWrite provenance, waitpid, laststatus, descriptor-backed
  readback with 'bytes=0x3e source=volatile-vfs-descriptor-read',
  deterministic negatives for 'exec stdout 3>/tmp/stdout.txt' and
  'exec stdout 1>/tmp/other.txt', final
  'qemu-local-shell-explicit-fd1-regular-file-redirection-complete',
  errors=0, and PASS.
- Retained-control static inspection confirmed PASS/completion markers for
  implicit stdout and stderr regular-file output redirection, stdout/stderr
  append-create, read-only input redirection, /dev/null stdio redirection,
  descriptor dup/close and pipeline controls, waitpid, laststatus, and
  descriptor-backed cat evidence.

## Accepted Frontier

Accepted:

- exactly 'exec stdout 1>/tmp/stdout.txt' for the VFS-backed '/bin/stdout'
  fixture;
- exactly 'exec stdout 1>>/tmp/stdout.txt' for the VFS-backed '/bin/stdout'
  fixture;
- the existing '/tmp/stdout.txt' volatile scratch target only;
- child-only fd1 rebinding to the accepted volatile VFS regular-file
  descriptor path;
- userspace TalosWrite provenance, descriptor-backed readback, waitpid,
  laststatus, and shell fd1 restoration.

Deferred:

- arbitrary fd numbers and arbitrary output paths;
- fd2 alias expansion beyond already accepted exact stderr forms;
- descriptor moves and broader descriptor grammar;
- persistent storage and broad writable filesystem mutation;
- process accounting/concurrency, Pi 5 proof, networking, SSH, and phase
  transition.

## Validation

- fmt/lint/typecheck: 'cargo fmt --all -- --check' passed.
- unit tests: 'cargo -Zjson-target-spec test --quiet' passed.
- QEMU/substitute:
  'scripts/qemu-local-shell-explicit-fd1-regular-file-redirection-smoke.sh'
  passed with retained PASS log.
- retained-control static inspection: implicit stdout/stderr redirection,
  append-create, input redirection, /dev/null, descriptor/pipeline controls,
  lifecycle/status, waitpid, laststatus, and descriptor-backed cat evidence
  paths were checked for retained PASS/completion markers.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.
