# Phase 10 Minimal Stdout-To-Stdin Pipe Core

Task: phase10-minimal-stdout-to-stdin-pipe-core-20260604

Status: accepted

## Summary

Implemented the first exact shell-visible pipeline slice:
`exec stdout | exec stdin`. The shell parses one bounded two-stage form,
launches `/bin/stdout` and `/bin/stdin` through the accepted fixed `/bin`
VFS exec path, temporarily connects producer fd1 to a pipe writer and
consumer fd0 to the matching pipe reader, then restores the shell standard
descriptor table.

The task-owned QEMU/substitute transcript records the producer write through
fd1, the consumer read through fd0, deterministic EOF after the producer
writer closes, both VFS exec summaries, the consumer lifecycle as the current
`waitpid`/`laststatus` observation, and a post-pipeline descriptor-backed
`cat /etc/banner.txt` control.

## Findings And Disposition

- fixed: Added a bounded pipeline request parser for exactly one pipe between
  two exec forms without admitting leading, trailing, or multi-stage pipes.
- fixed: Added a task-local pipe endpoint descriptor kind and in-memory pipe
  state for this exact stdout-to-stdin producer/consumer path.
- fixed: Routed `/bin/stdout` userspace fd1 writes into the pipe writer and
  `/bin/stdin` userspace fd0 reads from the matching pipe reader.
- fixed: Restored shell fd1 after the producer and shell fd0 after the
  consumer, then recorded `shell-restored=true` in the pipeline summary.
- fixed: Proved deterministic pipe EOF by having the consumer read again after
  the producer writer closed and recording
  `read-result=pipe-eof-after-writer-close`.
- fixed: Added unit and QEMU/substitute coverage for the positive pipeline,
  unsupported leading/trailing/multi-stage pipe forms, mixed redirection
  controls, bad command controls, wait/laststatus, and descriptor-backed cat
  regression.
- not-an-issue: This slice executes the bounded producer before the consumer
  instead of accepting POSIX-complete concurrent pipeline scheduling; the
  accepted boundary is descriptor ownership, byte transfer, EOF, and status
  reporting for one exact two-stage form.
- deferred: multi-stage pipelines, stdout-only stderr-not-piped proof, pipefail,
  background jobs, async execution, fork, signals, job control, file
  redirection, arbitrary descriptor syntax, writable filesystem behavior, Pi 5
  proof, networking, and SSH remain out of scope.

## Evidence Map

- minimal stdout-to-stdin pipeline:
  `tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log`.
  QEMU/substitute evidence shows `exec stdout | exec stdin`,
  `fd1=pipe-endpoint` for `/bin/stdout`, `stream=pipe-writer
  route=pipe:stdout-to-stdin`, `fd0=pipe-endpoint` for `/bin/stdin`,
  `read-source=pipe:stdout-to-stdin`, matching 31-byte write/read counts,
  `writer-closed=true reader-eof=true shell-restored=true`, consumer
  `waitpid`/`laststatus`, final
  `qemu-local-shell-minimal-stdout-to-stdin-pipeline-complete`, and PASS.
- descriptor redirection controls:
  `tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log`,
  `tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log`,
  `tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log`, and
  `tasks/evidence/2026-06-04-phase10-stderr-close-redirection-core/qemu-local-shell-stderr-close-redirection-smoke.log`.
- normal userspace stdio controls:
  `tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log` and
  `tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log`.
- scheduler-backed stdin and descriptor-backed cat controls:
  `tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log` and
  `tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log`.

## Accepted Frontier

Accepted:

- `exec stdout | exec stdin` parses as the first exact two-stage pipeline
  form.
- Both participants launch through the accepted fixed `/bin` VFS exec path.
- Producer child fd1 is a pipe writer; consumer child fd0 is the matching pipe
  reader.
- Producer bytes written through fd1 are read by the consumer through fd0.
- The pipe reports deterministic EOF after the producer writer closes.
- Shell fd0/fd1/fd2 are restored after the pipeline command completes.
- `waitpid` and `laststatus` report the consumer lifecycle for this bounded
  pipeline form.
- Unsupported leading, trailing, multi-stage, redirection-mixed, and wrong-path
  pipeline forms fail deterministically.

Deferred:

- multi-stage pipelines and concurrent scheduling;
- stdout-only stderr-not-piped proof and pipe status policy beyond the consumer;
- pipefail, background jobs, async execution, fork, signals, job control;
- file redirection, arbitrary descriptor syntax, writable filesystem behavior,
  Pi 5 proof, networking, and SSH.

## Validation Summary

- formatting: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 398 no_std
  tests.
- QEMU/substitute:
  `scripts/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.sh`
  passed with retained task evidence.
- QEMU/substitute controls:
  `scripts/qemu-local-shell-stdout-to-stderr-redirection-smoke.sh`,
  `scripts/qemu-local-shell-stderr-to-stdout-redirection-smoke.sh`,
  `scripts/qemu-local-shell-stdout-close-redirection-smoke.sh`,
  `scripts/qemu-local-shell-stderr-close-redirection-smoke.sh`,
  `scripts/qemu-local-shell-userspace-stdout-smoke.sh`,
  `scripts/qemu-local-shell-userspace-stdin-smoke.sh`,
  `scripts/qemu-local-shell-scheduler-backed-stdin-wait-smoke.sh`, and
  `scripts/qemu-local-cat-banner-smoke.sh` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- diff checks: `git diff --check` and `git diff --cached --check` passed.

hardwareTestLock remained unlocked/restored and unused.
