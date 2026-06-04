# Phase 10 Pipeline Stderr Not Piped Core

Task: phase10-pipeline-stderr-not-piped-core-20260604

Status: accepted

## Summary

Implemented `exec stderr | exec stdin` as the bounded stdout-only pipeline semantic control. The shell accepts `/bin/stderr` as the producer side of the exact two-stage pipeline, but only producer fd1 is connected to the consumer pipe. The stderr fixture still writes through fd2 to `stream=stderr route=runtime-console0/stderr`, the consumer sees an empty stdout pipe as deterministic `pipe-eof/no-data`, and `exec stdout | exec stdin` remains the positive control.

## Findings And Disposition

- fixed: Allowed `/bin/stderr` as the producer for the existing exact two-stage pipeline without changing the consumer path or admitting redirections.
- fixed: Preserved the pipe endpoint on producer fd1 while leaving producer fd2 as the inherited stderr route, proving stderr bytes do not enter consumer fd0.
- fixed: Added a distinct pipe EOF/no-data userspace stdin report for an empty pipe, separate from terminal Ctrl-D EOF and runtime-console0 readiness no-data.
- fixed: Added unit coverage for `exec stderr | exec stdin`, retained positive `exec stdout | exec stdin`, and unsupported descriptor mixing such as `exec stderr 2>&1 | exec stdin`.
- fixed: Added a task-owned QEMU/substitute smoke and boot scenario for the stderr-not-piped control.
- deferred: `2>&1` inside pipelines, stderr piping by default, pipefail, multi-stage pipelines, concurrent scheduling, async execution, fork, signals, file redirection, writable filesystem behavior, Pi 5 proof, networking, and SSH remain out of scope.

## Evidence Map

- stderr-not-piped pipeline: `tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log`. QEMU/substitute evidence shows `exec stderr | exec stdin`, producer fd1 as the pipe endpoint, producer fd2 as `stream=stderr route=runtime-console0/stderr`, zero pipe bytes written/read, `read-result=pipe-eof/no-data`, `writer-closed=true reader-eof=true shell-restored=true`, consumer `waitpid`/`laststatus`, final `qemu-local-shell-pipeline-stderr-not-piped-complete`, and PASS.
- positive stdout-to-stdin pipeline control: the same task-owned QEMU/substitute log records `exec stdout | exec stdin`, 31 bytes written/read through the pipe, `stream=pipe-writer route=pipe:stdout-to-stdin`, and `read-result=pipe-eof-after-writer-close`.
- distinct stderr routing control: `tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log` and refreshed `tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-distinct-stderr-routing-smoke.log`.
- descriptor redirection controls: refreshed `tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log` and `tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log`.
- descriptor-backed cat control: the task-owned QEMU/substitute log records `cat /etc/banner.txt` after both pipeline forms.

## Accepted Frontier

Accepted:

- `exec stderr | exec stdin` parses as the stderr-not-piped semantic control for the existing exact two-stage pipeline grammar.
- Producer fd1 is the only pipe writer endpoint; producer fd2 remains the inherited stderr route and writes stderr bytes to runtime-console0/stderr.
- The consumer fd0 pipe read observes zero bytes and deterministic `pipe-eof/no-data`.
- The accepted `exec stdout | exec stdin` pipeline positive control still transfers bytes through fd1 to fd0.
- Shell descriptors are restored after both pipeline forms.
- `waitpid` and `laststatus` report the consumer lifecycle for this bounded pipeline form.
- Unsupported descriptor-mixing pipeline forms fail deterministically.

Deferred:

- `2>&1` inside pipelines and explicit stderr piping;
- multi-stage pipelines, pipefail, background jobs, async execution, fork, signals, job control;
- file redirection, arbitrary descriptor syntax, writable filesystem behavior, Pi 5 proof, networking, and SSH.

## Validation Summary

- formatting: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 399 no_std tests.
- QEMU/substitute: `scripts/qemu-local-shell-pipeline-stderr-not-piped-smoke.sh` passed with retained task evidence.
- QEMU/substitute controls: `scripts/qemu-local-shell-distinct-stderr-routing-smoke.sh`, `scripts/qemu-local-shell-stdout-to-stderr-redirection-smoke.sh`, and `scripts/qemu-local-shell-stderr-to-stdout-redirection-smoke.sh` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- diff checks: `git diff --check` and `git diff --cached --check` passed.

hardwareTestLock remained unlocked/restored and unused.
