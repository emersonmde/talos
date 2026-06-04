# Phase 10 Distinct Stderr Routing Core

Task: phase10-distinct-stderr-routing-core-20260604

Status: accepted

## Summary

Implemented the smallest distinct stderr routing evidence path for
shell-visible VFS-backed `exec stderr`. Talos still writes fd1 and fd2 to
the shared runtime-console0 physical sink, but inherited stdio descriptor
objects now expose descriptor-derived stream and route metadata. The
`/bin/stderr` fixture records:

`exec-stderr ... stream=stderr route=runtime-console0/stderr source=userspace-talos-write`

The stdout control records:

`exec-stdout ... stream=stdout route=runtime-console0/stdout source=userspace-talos-write`

This accepts fd2 stream identity/routing metadata without claiming pipes,
redirection, file-backed stderr, or separate physical console sinks.

## Findings And Disposition

- fixed: Added descriptor-object stream and runtime-console route names derived
  from reserved inherited stdio descriptor references.
- fixed: Added `stream=stdout route=runtime-console0/stdout` to
  `exec-stdout` evidence and `stream=stderr route=runtime-console0/stderr`
  to `exec-stderr` evidence.
- fixed: Added a task-owned
  `qemu_local_shell_distinct_stderr_routing` QEMU/substitute scenario and
  smoke wrapper with a distinct evidence label/classification.
- fixed: Updated the QEMU smoke expectations and local command-loop unit
  checks so stdout and stderr controls prove the accepted stream identity.
- fixed: Added the new boot scenario to the build-time check-cfg allowlist and
  QEMU dispatch path, removing unexpected-cfg warnings.
- not-an-issue: fd1 and fd2 still share the runtime-console0 physical output
  backend; the accepted split is descriptor/stream-origin metadata, not a
  separate sink.
- deferred: pipes, redirection, file-backed stderr, separate physical sinks,
  terminal colors/policy, libc stdio, async jobs, fork, signals, writable
  filesystem behavior, Pi 5 proof, networking, and SSH remain out of scope.

## Evidence Map

- distinct stderr routing:
  `tasks/evidence/2026-06-04-phase10-distinct-stderr-routing-core/qemu-local-shell-distinct-stderr-routing-smoke.log`.
  QEMU/substitute evidence shows `exec stderr`, descriptor-backed
  `/bin/stderr` VFS/open/read lineage, inherited fd2 write metadata
  `stream=stderr route=runtime-console0/stderr`, lifecycle/status,
  `waitpid`, `laststatus`, nonzero status control, zero-status controls,
  negative exec controls, descriptor-backed `cat /etc/banner.txt`, final
  classification `qemu-local-shell-distinct-stderr-routing-complete`, and
  PASS.
- stdout control:
  `tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log`.
  Rerun QEMU/substitute evidence shows fd1 still records
  `stream=stdout route=runtime-console0/stdout` and is not mislabeled as
  stderr.
- scheduler-backed stdin wait control:
  `tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log`.
  Rerun QEMU/substitute evidence retains delayed input wake/resume behavior.
- no-data/readiness control:
  `tasks/evidence/2026-06-04-phase10-runtime-stdin-readiness-distinction-core/qemu-local-shell-runtime-stdin-readiness-smoke.log`.
  Rerun QEMU/substitute evidence retains `-EAGAIN`,
  `read-result=readiness/no-data`, and no false EOF.
- terminal EOF control:
  `tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log`.
  Rerun QEMU/substitute evidence retains `return=0` true terminal EOF.

## Accepted Frontier

Accepted:

- fd2 stderr writes through inherited stdio now carry explicit
  `stream=stderr route=runtime-console0/stderr` metadata.
- fd1 stdout writes carry explicit
  `stream=stdout route=runtime-console0/stdout` metadata.
- The accepted physical sink remains runtime-console0 for both streams.
- Descriptor-backed VFS/open/read exec lineage, standard descriptor
  inheritance, loader temporary descriptor non-leak, lifecycle/status,
  `waitpid`, `laststatus`, negative exec controls, descriptor-backed cat,
  scheduler-backed stdin wait/readiness, and Ctrl-D EOF regressions remain
  retained.

Deferred:

- separate stdout/stderr physical sinks;
- pipes, redirection, file-backed stderr, and writable filesystem behavior;
- libc stdio, async jobs, fork, signals, termios expansion, Pi 5 proof,
  networking, and SSH.

## Validation Summary

- formatting: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 391 tests
  and no unexpected-cfg warnings in
  `target/phase10-distinct-stderr-routing-cargo-test.log`.
- QEMU/substitute:
  `scripts/qemu-local-shell-distinct-stderr-routing-smoke.sh --quiet` passed
  with retained task evidence.
- QEMU/substitute:
  `scripts/qemu-local-shell-userspace-stdout-smoke.sh --quiet` passed as the
  stdout control.
- QEMU/substitute:
  `scripts/qemu-local-shell-scheduler-backed-stdin-wait-smoke.sh --quiet`
  passed as the delayed-input control.
- QEMU/substitute:
  `scripts/qemu-local-shell-runtime-stdin-readiness-smoke.sh --quiet` passed
  as the no-data/readiness control.
- QEMU/substitute:
  `scripts/qemu-local-shell-terminal-ctrl-d-eof-smoke.sh --quiet` passed as
  the true EOF control.
- docs: `/home/node/.cargo/bin/mdbook build` passed with the existing large
  search-index warning.
- diff checks: `git diff --check` passed.

hardwareTestLock remained unlocked/restored and unused.
