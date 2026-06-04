# Phase 10 /dev/null Stdout Redirection Contract Core

Task: phase10-dev-null-stdout-redirection-contract-core-20260604
Status: accepted

## Scope

Accept exactly one file/device redirection sink form:
`exec stdout >/dev/null`. The launched VFS-backed `/bin/stdout` child gets
fd1 rebound to an explicit `/dev/null` device sink for that child only. The
shell restores fd1 after the child exits.

This is not writable filesystem support. `/dev/null` is a named sink device
that validates the user write buffer and discards the copied bytes.
Regular-file output redirection, append/truncate, input redirection, stderr
redirection, arbitrary descriptor syntax, broader file/device semantics,
multi-stage pipelines, writable filesystem behavior, Pi 5 proof, networking,
SSH, and a phase transition remain deferred.

## Findings

- fixed: Added a POSIX descriptor object contract for `/dev/null` as
  `DescriptorObjectKind::Device` with stable `device:/dev/null` route
  metadata. `TalosWrite` copies/validates the user buffer and reports the
  requested byte count while discarding the bytes.
- fixed: Added exact parser support for `>/dev/null` as a child-only fd1 sink
  redirection. The redirection record reports `op=sink`,
  `target-path=/dev/null`, `target-stream=null-sink`, and
  `target-route=device:/dev/null`.
- fixed: The descriptor inheritance record for the redirected child reports
  `fd1=device`; normal `exec stdout` after the redirected child proves shell
  fd1 restoration.
- fixed: Added task-owned no_std coverage and QEMU/substitute smoke coverage.
- not-an-issue: The stdout fixture payload appears once in the task smoke log
  because the follow-up normal `exec stdout` control intentionally proves
  restoration; the redirected command itself reports `stream=null-sink` and
  does not emit the payload on runtime-console0/stdout.
- deferred: `1>/dev/null`, `1>file`, `exec stdout | stderr`,
  regular-file redirection, stderr-to-/dev/null, append/truncate, input
  redirection, and broader descriptor syntax remain deterministic negatives or
  later explicit tasks.

## Evidence

- QEMU/substitute task smoke:
  `tasks/evidence/2026-06-04-phase10-dev-null-stdout-redirection-contract-core/qemu-local-shell-dev-null-stdout-redirection-smoke.log`
  records command 3 `exec stdout >/dev/null`, `fd1=device`,
  `exec-redirection op=sink ... target-path=/dev/null ... target-route=device:/dev/null`,
  `exec-stdout ... stream=null-sink route=device:/dev/null`, normal
  `exec stdout` restoration, deterministic negative redirection forms,
  descriptor-backed `cat /etc/banner.txt`, final
  `qemu-local-shell-dev-null-stdout-redirection-complete`, and PASS.
- Retained QEMU/substitute controls passed in this iteration:
  `scripts/qemu-local-shell-userspace-stdout-smoke.sh`,
  `scripts/qemu-local-shell-stdout-to-stderr-redirection-smoke.sh`,
  `scripts/qemu-local-shell-stdout-close-redirection-smoke.sh`,
  `scripts/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.sh`,
  `scripts/qemu-local-shell-runtime-stdin-readiness-smoke.sh`,
  `scripts/qemu-local-shell-terminal-ctrl-d-eof-smoke.sh`, and
  `scripts/qemu-local-cat-banner-smoke.sh`.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 402 no_std
  tests.
- QEMU/substitute: task-owned stdout-to-/dev/null smoke passed.
- QEMU/substitute: retained normal stdout, descriptor dup/close redirection,
  descriptor-mixing pipeline, stdin readiness/EOF, and descriptor-backed cat
  controls passed.
- diff check: `git diff --check` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff check: `git diff --cached --check` passed before commit.

## Commit

Commit: accepted implementation and task evidence are committed; final SHA is
recorded in supervisor state.
