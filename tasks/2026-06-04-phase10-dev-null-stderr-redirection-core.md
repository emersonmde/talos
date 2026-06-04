# Phase 10 /dev/null Stderr Redirection Core

Task: phase10-dev-null-stderr-redirection-core-20260604
Status: accepted

## Scope

Accept exactly one inverse /dev/null sink form: `exec stderr 2>/dev/null`.
The launched VFS-backed `/bin/stderr` child gets fd2 rebound to the explicit
`/dev/null` sink device for that child only. The shell restores fd2 after the
child exits.

This is not writable filesystem support. `/dev/null` remains the accepted
named sink device: `TalosWrite` validates/copies the userspace write buffer,
discards the bytes, and returns the accepted byte count. Regular-file output
redirection, append/truncate, input redirection, arbitrary descriptor syntax,
explicit stderr pipe syntax, multi-stage/concurrent pipelines, writable
filesystem behavior, Pi 5 proof, networking, SSH, and a phase transition
remain deferred.

## Findings

- fixed: Added `StderrToDevNull` as the fd2 sibling of the accepted fd1
  sink redirection. The parser accepts only `2>/dev/null` for stderr.
- fixed: The child descriptor table reports `fd2=device`; the redirection
  record reports `op=sink`, `target-path=/dev/null`,
  `target-stream=null-sink`, and `target-route=device:/dev/null`.
- fixed: The stderr fixture write routes to `stream=null-sink` and
  `route=device:/dev/null`, so the redirected fixture payload is absent from
  runtime-console0/stderr.
- fixed: A following normal `exec stderr` control proves shell fd2
  restoration through `stream=stderr route=runtime-console0/stderr`.
- fixed: Added task-owned no_std coverage, a dedicated QEMU/substitute
  wrapper, kernel boot scenario labels/classification, expected dispatch
  rows, and task-owned evidence.
- not-an-issue: The task smoke log includes one stderr fixture payload line
  because the later normal `exec stderr` control intentionally proves
  restoration after the redirected child exits.
- deferred: `exec stderr 2>file`, `exec stderr 2>>/dev/null`,
  `exec stderr </dev/null`, regular-file redirection, append/truncate, input
  redirection, arbitrary descriptor syntax, writable filesystem behavior,
  multi-stage/concurrent pipelines, Pi 5 proof, networking, SSH, and a phase
  transition remain deterministic negatives or later explicit tasks.

## Evidence

- QEMU/substitute task smoke:
  `tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-dev-null-stderr-redirection-smoke.log`
  records command 3 `exec stderr 2>/dev/null`, `fd2=device`,
  `exec-redirection op=sink ... target-path=/dev/null ...
  target-route=device:/dev/null`, `exec-stderr ... stream=null-sink
  route=device:/dev/null`, no redirected stderr fixture payload, `waitpid`,
  `laststatus`, normal `exec stderr` restoration, stdout-to-/dev/null
  regression control, deterministic negative redirection forms,
  descriptor-backed `cat /etc/banner.txt`, final
  `qemu-local-shell-dev-null-stderr-redirection-complete`, and PASS.
- Retained QEMU/substitute controls passed in this iteration under the same
  evidence directory: stdout-to-/dev/null control, normal userspace stderr,
  stdout-to-stderr descriptor dup, descriptor-mixing pipeline, runtime stdin
  readiness, stdin EOF/no-data, and descriptor-backed cat.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute: task-owned stderr-to-/dev/null smoke passed.
- QEMU/substitute: retained normal stderr, stdout-to-/dev/null,
  descriptor redirection, descriptor-mixing pipeline, stdin readiness/EOF,
  and descriptor-backed cat controls passed.
- diff check: `git diff --check` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff check: `git diff --cached --check` passed before commit.

## Commit

Commit: accepted implementation and task evidence are committed; final SHA is
recorded in supervisor state.
