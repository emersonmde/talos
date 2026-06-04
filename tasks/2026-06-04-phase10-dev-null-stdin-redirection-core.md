# Phase 10 /dev/null Stdin Redirection Core

Task: phase10-dev-null-stdin-redirection-core-20260604
Status: accepted

## Scope

Accept exactly one /dev/null source form: `exec stdin </dev/null`. The
launched VFS-backed `/bin/stdin` child gets fd0 rebound to an explicit
`/dev/null` device source for that child only. The shell restores fd0 after
the child exits.

This is not regular-file input redirection and not broader file/device
semantics. `/dev/null` is accepted here only as a named read source device:
`TalosRead` on the device returns zero bytes as true source EOF without
polling runtime-console0. Regular-file input redirection, output regular-file
redirection, append/truncate, shorthand forms, arbitrary descriptor syntax,
writable filesystem behavior, multi-stage/concurrent pipelines, Pi 5 proof,
networking, SSH, and a phase transition remain deferred.

## Findings

- fixed: Added `StdinFromDevNull` as the fd0 source sibling of the accepted
  fd1/fd2 /dev/null sink redirections. The parser accepts only the exact
  `</dev/null` token, and execution restricts that redirection to the
  VFS-backed `/bin/stdin` fixture.
- fixed: The child descriptor table reports `fd0=device`; the redirection
  record reports `op=source`, `source-path=/dev/null`,
  `source-stream=null-source`, and
  `source-route=device:/dev/null`.
- fixed: Reading the `/dev/null` descriptor returns zero bytes without
  consuming runtime-console0 input. The stdin fixture reports
  `read-source=device:/dev/null` and
  `read-result=null-source-eof/no-data` through accepted stdout/status paths.
- fixed: A following normal `exec stdin` control consumes `talos-console0`
  through restored shell fd0, proving the mutation is child-only.
- fixed: Added no_std unit coverage, a dedicated QEMU/substitute wrapper,
  kernel boot scenario labels/classification, expected dispatch rows, and
  task-owned evidence.
- fixed: Deterministic negatives keep `exec stdout </dev/null`,
  `exec stdin </etc/banner.txt`, and `exec stdin < /dev/null` outside the
  accepted surface.
- not-an-issue: The task smoke includes a later visible stdin payload because
  the normal `exec stdin` control intentionally proves shell fd0 restoration.
- deferred: regular-file input redirection, output regular-file redirection,
  append/truncate, shorthand/broader descriptor syntax, writable filesystem
  behavior, broader file/device semantics, multi-stage/concurrent pipelines,
  Pi 5 proof, networking, SSH, and a phase transition.

## Evidence

- QEMU/substitute task smoke:
  `tasks/evidence/2026-06-04-phase10-dev-null-stdin-redirection-core/qemu-local-shell-dev-null-stdin-redirection-smoke.log`
  records command 3 `exec stdin </dev/null`, `fd0=device`,
  `exec-redirection op=source ... source-path=/dev/null ...
  source-stream=null-source source-route=device:/dev/null`,
  `exec-stdin ... read-source=device:/dev/null ...
  read-result=null-source-eof/no-data`, `waitpid`, `laststatus`, normal
  `exec stdin` restoration through runtime-console0/local-input,
  deterministic negative redirection forms, descriptor-backed
  `cat /etc/banner.txt`, final
  `qemu-local-shell-dev-null-stdin-redirection-complete`, and PASS.
- Retained stdout/stderr /dev/null sink controls:
  `tasks/evidence/2026-06-04-phase10-dev-null-stdout-redirection-contract-core/qemu-local-shell-dev-null-stdout-redirection-smoke.log`,
  `tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-dev-null-stderr-redirection-smoke.log`.
- Retained stdin controls:
  `tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log`,
  `tasks/evidence/2026-06-04-phase10-runtime-stdin-readiness-distinction-core/qemu-local-shell-runtime-stdin-readiness-smoke.log`,
  `tasks/evidence/2026-06-03-phase10-stdin-eof-no-data-core/qemu-local-shell-stdin-eof-no-data-smoke.log`,
  `tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log`.
- Retained descriptor redirection and pipeline controls:
  `tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log`,
  `tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log`,
  `tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log`,
  `tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log`.
- Retained VFS exec, waitpid/laststatus, and descriptor-backed cat controls:
  `tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log`,
  `tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log`,
  `tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log`.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute: task-owned /dev/null stdin redirection smoke passed.
- QEMU/substitute: retained stdout/stderr /dev/null sinks, stdin controls,
  descriptor redirection/pipeline controls, waitpid/laststatus, and
  descriptor-backed cat evidence paths were inspected and present.
- diff check: `git diff --check` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff check: `git diff --cached --check` passed before commit.

## Commit

Commit: accepted implementation and task evidence are committed; final SHA is
recorded in supervisor state.
