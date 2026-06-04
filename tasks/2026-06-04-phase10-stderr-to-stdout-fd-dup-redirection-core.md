# Phase 10 Stderr-To-Stdout FD Dup Redirection Core

Task: phase10-stderr-to-stdout-fd-dup-redirection-core-20260604

Status: accepted

## Summary

Implemented the inverse shell-visible descriptor-duplication redirection slice:
`exec stderr 2>&1`. The command resolves `/bin/stderr` through the accepted
fixed `/bin` lookup and descriptor-backed VFS/open/read path, then temporarily
binds child fd2 to the inherited fd1 target. The userspace stderr fixture writes
through fd2 and records:

`exec-stderr ... stream=stdout route=runtime-console0/stdout source=userspace-talos-write`

The shell descriptor table is restored after child launch; a following normal
`exec stderr` control records fd2 as
`stream=stderr route=runtime-console0/stderr`.

## Findings And Disposition

- fixed: Added bounded parsing for the exact `2>&1` descriptor-dup token
  without admitting file redirection, append/truncate, pipes, descriptor close,
  descriptor move, or arbitrary descriptor targets.
- fixed: Reused the child-only descriptor-table redirection path for fd2 to fd1
  and retained explicit `exec-redirection op=dup` evidence with
  child-only/shell-restored markers.
- fixed: Added unit coverage for redirected stderr, shell descriptor
  restoration through a following normal stderr exec, and deterministic
  rejection of unsupported bad-descriptor, file-redirection, and pipe forms.
- fixed: Added a task-owned QEMU/substitute smoke scenario and wrapper for
  stderr-to-stdout descriptor-dup redirection.
- fixed: Kept accepted `1>&2`, normal stdout, normal stderr, and terminal EOF
  controls in the validation set.
- not-an-issue: No decision-log update is required; this task applies the
  accepted child-only descriptor-dup semantics from the forward slice to the
  inverse fd2-to-fd1 direction without changing the durable policy.
- deferred: descriptor close/move syntax, arbitrary `N>&M`, regular-file
  redirection, append/truncate, pipes, here-docs, writable filesystem behavior,
  separate physical sinks, async execution, fork, signals, libc stdio, Pi 5
  proof, networking, and SSH remain out of scope.

## Evidence Map

- stderr-to-stdout descriptor-dup redirection:
  `tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log`.
  QEMU/substitute evidence shows `exec stderr 2>&1`, descriptor-backed
  `/bin/stderr` VFS/open/read lineage, `exec-redirection ... source-fd=2
  target-fd=1 ... target-stream=stdout target-route=runtime-console0/stdout
  child-only=true shell-restored=true`, fd2 userspace write metadata
  `stream=stdout route=runtime-console0/stdout`, lifecycle/status,
  `waitpid`, `laststatus`, accepted forward-direction `1>&2` regression,
  deterministic unsupported file-redirection negative, descriptor-backed
  `cat /etc/banner.txt`, final
  `qemu-local-shell-stderr-to-stdout-redirection-complete`, and PASS.
- forward-direction descriptor-dup regression:
  `tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log`.
  Rerun QEMU/substitute evidence retains `exec stdout 1>&2` with fd1 routed
  to `runtime-console0/stderr`.
- normal stdout route control:
  `tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log`.
  Rerun QEMU/substitute evidence retains fd1 as
  `stream=stdout route=runtime-console0/stdout`.
- normal stderr route control:
  `tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log`.
  Rerun QEMU/substitute evidence retains fd2 as
  `stream=stderr route=runtime-console0/stderr`.
- stdin EOF/readiness control:
  `tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log`.
  Rerun QEMU/substitute evidence retains true terminal EOF behavior alongside
  stdout/stderr and VFS exec regressions.

## Accepted Frontier

Accepted:

- `exec stderr 2>&1` parses as the inverse exact descriptor-dup redirection
  form.
- Redirection is child-only for the launched VFS-backed executable.
- `/bin/stderr` remains loaded through descriptor-backed VFS/open/read and
  writes via fd2, but fd2 resolves to the inherited fd1 stdout target for that
  child.
- The shell fd2 descriptor is restored after the child exec; a following normal
  `exec stderr` records the stderr route again.
- Accepted `exec stdout 1>&2` behavior remains covered as a regression.
- Unsupported bad descriptor targets, file redirection, and pipes fail
  deterministically.

Deferred:

- descriptor close/move syntax and arbitrary `N>&M`;
- regular-file redirection, append/truncate, and pipes;
- writable filesystem behavior, separate physical sinks, async jobs, fork,
  signals, libc stdio, Pi 5 proof, networking, and SSH.

## Validation Summary

- formatting: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute:
  `scripts/qemu-local-shell-stderr-to-stdout-redirection-smoke.sh --quiet`
  passed with retained task evidence.
- QEMU/substitute:
  `scripts/qemu-local-shell-stdout-to-stderr-redirection-smoke.sh --quiet`
  passed as the forward-direction descriptor-dup regression.
- QEMU/substitute:
  `scripts/qemu-local-shell-distinct-stderr-routing-smoke.sh --quiet` passed
  as the distinct stderr route control.
- QEMU/substitute:
  `scripts/qemu-local-shell-userspace-stdout-smoke.sh --quiet` passed as the
  normal stdout route control.
- QEMU/substitute:
  `scripts/qemu-local-shell-userspace-stderr-smoke.sh --quiet` passed as the
  normal stderr route control.
- QEMU/substitute:
  `scripts/qemu-local-shell-terminal-ctrl-d-eof-smoke.sh --quiet` passed as
  the stdin EOF/readiness control relevant to touched shell I/O code.
- diff checks: `git diff --check` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed with the existing large
  search-index warning.

hardwareTestLock remained unlocked/restored and unused.
