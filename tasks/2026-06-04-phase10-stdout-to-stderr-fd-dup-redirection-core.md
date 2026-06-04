# Phase 10 Stdout-To-Stderr FD Dup Redirection Core

Task: phase10-stdout-to-stderr-fd-dup-redirection-core-20260604

Status: accepted

## Summary

Implemented the first shell-visible descriptor-duplication redirection slice:
`exec stdout 1>&2`. The command still resolves `/bin/stdout` through the
accepted fixed `/bin` lookup and descriptor-backed VFS/open/read path, but the
launched child temporarily binds fd1 to the inherited fd2 target. The
userspace stdout fixture writes through fd1 and records:

`exec-stdout ... stream=stderr route=runtime-console0/stderr source=userspace-talos-write`

The shell descriptor table is restored after the child launch; the retained
normal `exec stdout` control records fd1 as
`stream=stdout route=runtime-console0/stdout`.

## Findings And Disposition

- fixed: Added a bounded `1>&2` redirection token to shell exec parsing without
  admitting broader shell syntax or including the redirection token in argv.
- fixed: Applied redirection only around the child exec descriptor table by
  replacing fd1 with a copy of the inherited fd2 descriptor entry, then
  restoring the original fd1 entry before returning to the shell.
- fixed: Added explicit `exec-redirection op=dup source-fd=... target-fd=...`
  evidence with child-only and shell-restored markers.
- fixed: Added a task-owned QEMU/substitute smoke scenario and wrapper for
  stdout-to-stderr descriptor-dup redirection.
- fixed: Added unit coverage for redirected stdout, shell descriptor
  restoration through a following normal stdout exec, and deterministic
  rejection of unsupported `2>&1`, file redirection, and pipe syntax.
- not-an-issue: The accepted physical sink remains shared runtime-console0;
  this task accepts descriptor target/route metadata, not separate physical
  stdout/stderr devices.
- deferred: inverse `2>&1`, descriptor close/move syntax, regular-file
  redirection, append/truncate, pipes, here-docs, writable filesystem behavior,
  async execution, fork, signals, libc stdio, Pi 5 proof, networking, and SSH
  remain out of scope.

## Evidence Map

- stdout-to-stderr descriptor-dup redirection:
  `tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log`.
  QEMU/substitute evidence shows `exec stdout 1>&2`, descriptor-backed
  `/bin/stdout` VFS/open/read lineage, `exec-redirection ... source-fd=1
  target-fd=2 ... target-stream=stderr target-route=runtime-console0/stderr
  child-only=true shell-restored=true`, fd1 userspace write metadata
  `stream=stderr route=runtime-console0/stderr`, lifecycle/status,
  `waitpid`, `laststatus`, unsupported redirection negatives, descriptor-backed
  `cat /etc/banner.txt`, final
  `qemu-local-shell-stdout-to-stderr-redirection-complete`, and PASS.
- normal stdout route control:
  `tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log`.
  Rerun QEMU/substitute evidence retains fd1 as
  `stream=stdout route=runtime-console0/stdout`.
- distinct stderr route control:
  `tasks/evidence/2026-06-04-phase10-distinct-stderr-routing-core/qemu-local-shell-distinct-stderr-routing-smoke.log`.
  Rerun QEMU/substitute evidence retains fd2 as
  `stream=stderr route=runtime-console0/stderr`.
- stdin EOF/readiness control:
  `tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log`.
  Rerun QEMU/substitute evidence retains true terminal EOF behavior alongside
  stdout/stderr and VFS exec regressions.

## Accepted Frontier

Accepted:

- `exec stdout 1>&2` parses as the first exact descriptor-dup redirection form.
- Redirection is child-only for the launched VFS-backed executable.
- `/bin/stdout` remains loaded through descriptor-backed VFS/open/read and
  writes via fd1, but fd1 resolves to the inherited fd2 stderr target for that
  child.
- The shell fd1 descriptor is restored after the child exec; a following
  normal `exec stdout` records the stdout route again.
- Unsupported inverse, file, and pipe redirection forms fail deterministically.

Deferred:

- inverse `2>&1`;
- regular-file redirection, append/truncate, descriptor close/move syntax, and
  pipes;
- writable filesystem behavior, separate physical sinks, async jobs, fork,
  signals, libc stdio, Pi 5 proof, networking, and SSH.

## Validation Summary

- formatting: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 393 no_std
  tests.
- QEMU/substitute:
  `scripts/qemu-local-shell-stdout-to-stderr-redirection-smoke.sh --quiet`
  passed with retained task evidence.
- QEMU/substitute:
  `scripts/qemu-local-shell-userspace-stdout-smoke.sh --quiet` passed as the
  normal stdout route control.
- QEMU/substitute:
  `scripts/qemu-local-shell-distinct-stderr-routing-smoke.sh --quiet` passed
  as the normal stderr route control.
- QEMU/substitute:
  `scripts/qemu-local-shell-terminal-ctrl-d-eof-smoke.sh --quiet` passed as
  the stdin EOF/readiness control relevant to touched shell I/O code.
- docs: `/home/node/.cargo/bin/mdbook build` passed with the existing large
  search-index warning.
- diff checks: `git diff --check` passed before commit.

hardwareTestLock remained unlocked/restored and unused.
