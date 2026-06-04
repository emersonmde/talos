# Phase 10 Pipeline Stderr Dup To Stdout Core

Task: phase10-pipeline-stderr-dup-to-stdout-core-20260604

Status: accepted

## Summary

Implemented the exact descriptor-mixing pipeline form
`exec stderr 2>&1 | exec stdin`. The shell now installs the producer pipe on
fd1 first, then applies child-only `2>&1` so producer fd2 duplicates the
pipe-backed fd1 endpoint. The `/bin/stderr` fixture writes 31 bytes into the
pipe, `/bin/stdin` reads those bytes from inherited fd0, reports the stderr
fixture line through inherited fd1, observes writer-close EOF, and leaves shell
fd0/fd1/fd2 restored afterward.

The accepted boundary is only this exact two-stage form. Plain
`exec stderr | exec stdin` remains the stdout-only negative/control behavior,
plain `exec stdout | exec stdin` remains the positive stdout pipe transfer
control, and inverse `exec stdout 1>&2 | exec stdin` remains deferred.

## Findings And Disposition

- fixed: Allowed pipeline producer redirection only for the exact
  `StderrToStdout` producer case while keeping consumer redirection and
  stdout `1>&2` pipeline forms rejected.
- fixed: Allowed child descriptor inheritance and descriptor-dup target
  validation to treat a pipe endpoint as a writable output endpoint for the
  exact mixed producer.
- fixed: Aligned redirection metadata for pipe targets with write-path route
  metadata: `target-stream=pipe-writer target-route=pipe:stdout-to-stdin`.
- fixed: Updated the stdin fixture pipe read check so pipe payloads can be one
  of the explicitly proven fixture byte streams, stdout or stderr, instead of
  assuming every pipe read must be the stdout fixture.
- fixed: Added no_std unit coverage for the mixed pipeline, shell descriptor
  restoration, waitpid/laststatus, retained stderr-not-piped control, retained
  stdout pipeline control, and deterministic rejection of
  `exec stdout 1>&2 | exec stdin`.
- fixed: Added a task-owned QEMU/substitute smoke wrapper and boot scenario for
  `exec stderr 2>&1 | exec stdin`.
- deferred: Inverse `exec stdout 1>&2 | exec stdin`, arbitrary `N>&M`,
  explicit stderr pipe syntax, file/device redirection, multi-stage pipelines,
  concurrent POSIX pipeline scheduling, pipefail, jobs, fork/signals, writable
  filesystem behavior, Pi 5 proof, networking, and SSH remain out of scope.

## Evidence Map

- mixed stderr-to-pipe positive evidence:
  `tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log`.
  QEMU/substitute evidence records `exec stderr 2>&1 | exec stdin`,
  producer fd1 as the pipe writer, producer fd2 as a child-only duplicate of
  that pipe endpoint, `exec-redirection op=dup source-fd=2 target-fd=1`,
  `target-stream=pipe-writer target-route=pipe:stdout-to-stdin`,
  `exec-stderr ... stream=pipe-writer route=pipe:stdout-to-stdin`,
  31 bytes written/read, consumer `/bin/stdin` reporting the stderr fixture,
  writer-close EOF, consumer waitpid/laststatus, final classification
  `qemu-local-shell-pipeline-stderr-dup-to-stdout-complete`, and PASS.
- plain stderr-not-piped control:
  `tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log`
  was refreshed and passed. It preserves `exec stderr | exec stdin` as
  stdout-only: zero pipe bytes, fd2 stderr routing, `pipe-eof/no-data`, cat
  control, final classification, and PASS.
- plain stdout pipeline control:
  `tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log`
  was refreshed and passed. It preserves 31-byte stdout transfer, writer-close
  EOF, consumer waitpid/laststatus, cat control, final classification, and
  PASS.
- descriptor-dup controls:
  `tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log`
  and
  `tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log`
  were refreshed and passed.
- descriptor-close controls:
  `tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log`
  and
  `tasks/evidence/2026-06-04-phase10-stderr-close-redirection-core/qemu-local-shell-stderr-close-redirection-smoke.log`
  were refreshed and passed.
- retained stdin/readiness/EOF, VFS exec, lifecycle/status, waitpid,
  laststatus, negative controls, and descriptor-backed cat coverage remains
  referenced from the accepted Phase 10 stdio and minimal pipeline records.

## Accepted Frontier

Accepted:

- exactly `exec stderr 2>&1 | exec stdin` for descriptor-mixing pipelines;
- producer fd1 is installed as the pipe writer before child-only redirection;
- producer fd2 duplicates that pipe-backed fd1 endpoint for the `/bin/stderr`
  write;
- consumer fd0 reads the stderr fixture bytes from the pipe and reports them
  through inherited fd1;
- writer-close EOF, consumer waitpid, laststatus, no leaked pipe endpoints, and
  shell descriptor restoration are covered by task-owned QEMU/substitute
  evidence;
- existing `exec stderr | exec stdin` and `exec stdout | exec stdin`
  controls remain passing.

Deferred:

- inverse `exec stdout 1>&2 | exec stdin`;
- arbitrary descriptor syntax, explicit stderr pipe syntax, file/device
  redirection, and writable filesystem behavior;
- multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, Pi 5 proof, networking, and SSH.

## Validation Summary

- formatting: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with the QEMU
  runner path exported; 400 no_std tests passed.
- QEMU/substitute: `scripts/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.sh`
  passed with retained task evidence.
- QEMU/substitute controls:
  `scripts/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.sh`,
  `scripts/qemu-local-shell-pipeline-stderr-not-piped-smoke.sh`,
  `scripts/qemu-local-shell-stderr-to-stdout-redirection-smoke.sh`,
  `scripts/qemu-local-shell-stdout-to-stderr-redirection-smoke.sh`,
  `scripts/qemu-local-shell-stdout-close-redirection-smoke.sh`, and
  `scripts/qemu-local-shell-stderr-close-redirection-smoke.sh` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- diff checks: `git diff --check` and `git diff --cached --check` passed
  before commit.

hardwareTestLock remained unlocked/restored and unused.
