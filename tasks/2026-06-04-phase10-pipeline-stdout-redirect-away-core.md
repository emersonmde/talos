# Phase 10 Pipeline Stdout Redirect Away Core

Task: phase10-pipeline-stdout-redirect-away-core-20260604

Status: accepted

## Summary

Implemented the exact inverse descriptor-mixing pipeline form
`exec stdout 1>&2 | exec stdin`. The shell still installs the producer pipe on
fd1 first, then applies child-only `1>&2` so producer fd1 is rebound to the
inherited stderr route instead of the pipe writer endpoint. The VFS-backed
`/bin/stdout` fixture writes 31 bytes to `stream=stderr`, the pipe transfers
zero bytes, and the VFS-backed `/bin/stdin` consumer reports deterministic
`pipe-eof/no-data` through inherited fd1. Shell fd0/fd1/fd2 are restored after
the command, and consumer `waitpid`/`laststatus` remain tied to the lifecycle
record.

The accepted boundary is only this exact two-stage form plus the already
accepted `exec stderr 2>&1 | exec stdin` and plain stdout pipeline controls.
Arbitrary descriptor syntax, file/device redirection, and broader pipeline
scheduling remain deferred.

## Findings And Disposition

- fixed: Allowed pipeline producer redirection for exactly the
  `StdoutToStderr` producer case while keeping the fixed producer/consumer
  `/bin` paths and consumer-redirection rejection intact.
- fixed: Allowed child descriptor redirection to restore a pipe endpoint after
  a child-only dup, so `1>&2` can temporarily replace the producer pipe writer
  with inherited stderr and then restore fd1 for pipeline cleanup.
- fixed: Added no_std unit coverage for `exec stdout 1>&2 | exec stdin`,
  shell descriptor restoration, zero pipe bytes, `pipe-eof/no-data`,
  waitpid/laststatus, retained `2>&1` stderr-to-pipe positive control, and
  retained plain stdout pipeline transfer control.
- fixed: Added a task-owned QEMU/substitute smoke wrapper and boot scenario
  for `exec stdout 1>&2 | exec stdin`.
- fixed: Corrected the new smoke script command sequence after the first QEMU
  attempt sent `bogus` for the waitpid/laststatus control slots; the rerun
  passed and retained evidence.
- deferred: arbitrary `N>&M`, explicit stderr pipe syntax, file/device
  redirection, multi-stage/concurrent pipelines, pipefail, jobs, async
  execution, fork/signals, writable filesystem behavior, Pi 5 proof,
  networking, and SSH remain out of scope.

## Evidence Map

- inverse stdout-redirection-away positive evidence:
  `tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log`.
  QEMU/substitute evidence records `exec stdout 1>&2 | exec stdin`, producer
  fd1 rebound to `target-stream=stderr target-route=runtime-console0/stderr`,
  `/bin/stdout` writing 31 bytes through stderr, zero pipe bytes written/read,
  consumer `/bin/stdin` reporting `pipe-eof/no-data`, shell descriptor
  restoration, consumer waitpid/laststatus, final classification
  `qemu-local-shell-pipeline-stdout-redirect-away-complete`, and PASS.
- retained stderr-to-pipe positive control:
  `tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log`
  was refreshed and passed. It preserves fd1 pipe setup followed by child-only
  `2>&1`, fd2 as `stream=pipe-writer route=pipe:stdout-to-stdin`, 31 bytes
  transferred to `/bin/stdin`, waitpid/laststatus, final classification, and
  PASS.
- retained plain stdout pipeline control:
  `tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log`
  was refreshed and passed. It preserves 31-byte stdout transfer,
  writer-close EOF, consumer waitpid/laststatus, cat control, final
  classification, and PASS.
- retained plain stderr-not-piped control:
  `tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log`
  was refreshed and passed. It preserves stdout-only pipe semantics for
  `exec stderr | exec stdin`: zero pipe bytes, fd2 stderr routing,
  `pipe-eof/no-data`, cat control, final classification, and PASS.
- descriptor-dup controls:
  `tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log`
  and
  `tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log`
  were refreshed and passed.
- descriptor-close controls:
  `tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log`
  and
  `tasks/evidence/2026-06-04-phase10-stderr-close-redirection-core/qemu-local-shell-stderr-close-redirection-smoke.log`
  were refreshed and passed.
- retained stdin/readiness/EOF, VFS exec, lifecycle/status, waitpid,
  laststatus, negative controls, and descriptor-backed cat coverage remains
  referenced from the accepted Phase 10 stdio and pipeline records.

## Accepted Frontier

Accepted:

- exactly `exec stdout 1>&2 | exec stdin` for inverse descriptor-mixing
  pipelines;
- producer fd1 is installed as the pipe writer before child-only redirection;
- producer fd1 is then rebound to inherited fd2/stderr for the `/bin/stdout`
  write, so no stdout fixture bytes enter the pipe;
- consumer fd0 reads deterministic pipe EOF/no-data and reports it through
  inherited fd1;
- writer-close EOF, consumer waitpid, laststatus, no leaked pipe endpoints,
  and shell descriptor restoration are covered by task-owned QEMU/substitute
  evidence;
- existing `exec stderr 2>&1 | exec stdin`, `exec stderr | exec stdin`, and
  `exec stdout | exec stdin` controls remain passing.

Deferred:

- arbitrary descriptor syntax and arbitrary `N>&M`;
- explicit stderr pipe syntax, file/device redirection, and writable
  filesystem behavior;
- multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, Pi 5 proof, networking, and SSH.

## Validation Summary

- formatting: `cargo fmt --all` applied formatting; final
  `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 401 no_std
  tests.
- QEMU/substitute: `scripts/qemu-local-shell-pipeline-stdout-redirect-away-smoke.sh`
  passed with retained task evidence.
- QEMU/substitute controls:
  `scripts/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.sh`,
  `scripts/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.sh`,
  `scripts/qemu-local-shell-pipeline-stderr-not-piped-smoke.sh`,
  `scripts/qemu-local-shell-stdout-to-stderr-redirection-smoke.sh`,
  `scripts/qemu-local-shell-stderr-to-stdout-redirection-smoke.sh`,
  `scripts/qemu-local-shell-stdout-close-redirection-smoke.sh`, and
  `scripts/qemu-local-shell-stderr-close-redirection-smoke.sh` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- diff checks: `git diff --check` and `git diff --cached --check` passed
  before commit.

hardwareTestLock remained unlocked/restored and unused.
