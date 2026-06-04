# Phase 10 Runtime Stdin Readiness Distinction Core

Task: phase10-runtime-stdin-readiness-distinction-core-20260604

Status: accepted

## Summary

Accepted a narrow semantic correction for shell-visible VFS-backed `exec stdin`:
when runtime-console0/local-input has no bytes immediately available, the
launched `/bin/stdin` fixture now observes `TalosRead` returning `-EAGAIN`,
reports `Talos userspace stdin fixture no-data: readiness` through inherited
fd1, and records `read-result=readiness/no-data`. This is distinct from true
EOF and from the older proof-buffer stdin EOF behavior.

The successful `talos-console0` stdin path remains unchanged: `/bin/stdin`
still consumes available runtime-console0/local-input bytes through inherited
`fd0=stdio-input` using the userspace `TalosRead` path and reports them through
inherited fd1 with `TalosWrite`.

## Findings And Disposition

- fixed: `read_descriptor_from_console_input` now returns `EAGAIN` when no
  runtime-console0/local-input bytes are available instead of returning `0` and
  collapsing ordinary readiness/no-data into EOF.
- fixed: `/bin/stdin` reports the no-data path as
  `Talos userspace stdin fixture no-data: readiness` with
  `read-result=readiness/no-data` and the syscall return value
  `0xfffffffffffffff5` (`-EAGAIN`).
- fixed: added `scripts/qemu-local-shell-runtime-stdin-readiness-smoke.sh` and
  task-owned evidence under
  `tasks/evidence/2026-06-04-phase10-runtime-stdin-readiness-distinction-core/`.
- fixed: updated the shared local shell smoke harness so readiness-mode stdin
  drives the same regression sequence as the successful stdin smoke.
- removed: the legacy `qemu-local-shell-stdin-eof-no-data-smoke.sh` no longer
  writes current semantics into the old 2026-06-03 EOF/no-data evidence path;
  it delegates to the readiness smoke instead.
- not-an-issue: fixed proof-buffer stdin still uses `0` as bounded EOF in the
  older Phase 7 proof surface; this task only changes runtime-console0/local
  input no-data semantics.
- deferred: true terminal EOF policy, Ctrl-D EOF, scheduler-backed blocking
  reads, wait queues, select/poll, nonblocking flag APIs, async execution,
  fork, signals, pipes, redirection, distinct stderr stream routing, writable
  filesystem behavior, libc stdio, Pi 5 proof, networking, and SSH remain out
  of scope.

## Evidence

- QEMU/substitute readiness/no-data stdin smoke:
  `tasks/evidence/2026-06-04-phase10-runtime-stdin-readiness-distinction-core/qemu-local-shell-runtime-stdin-readiness-smoke.log`.
  The log contains `exec stdin`, visible report
  `Talos userspace stdin fixture no-data: readiness`, descriptor-backed
  `source=vfs-open-read`, inherited `fd0=stdio-input`, `fd1=stdio-output`,
  and `fd2=stdio-output`, `loader-temp-open=false`, argv0 `/bin/stdin`,
  deterministic empty envp,
  `exec-stdin fd=0x0000000000000000 bytes=0x0000000000000000
  return=0xfffffffffffffff5 read-source=runtime-console0/local-input
  stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000031
  stdout-return=0x0000000000000031
  source=userspace-talos-read+userspace-talos-write
  read-result=readiness/no-data`, zero lifecycle status, matching `waitpid`
  and `laststatus`, retained stdout fixture, fixed `/bin` lookup, zero/nonzero
  status controls, negative exec controls, descriptor-backed
  `cat /etc/banner.txt`, classification
  `qemu-local-shell-userspace-stdin-complete`, and PASS.
- retained QEMU/substitute successful runtime-console0 stdin regression:
  `tasks/evidence/2026-06-03-phase10-runtime-console0-stdin-core/qemu-local-shell-runtime-console0-stdin-smoke.log`.
  The retained log contains `Talos userspace stdin fixture read:
  talos-console0` and `exec-stdin ... bytes=0x000000000000000e
  return=0x000000000000000e read-source=runtime-console0/local-input`.
- retained stdout/stderr, VFS exec, lifecycle/status, waitpid/laststatus,
  negative exec, and descriptor-backed cat regressions:
  `tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log`,
  `tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log`,
  `tasks/evidence/2026-06-03-phase10-minimal-path-lookup-exec-core/qemu-local-shell-path-lookup-smoke.log`,
  and
  `tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log`.

## Validation Summary

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed under QEMU with
  the Talos toolchain PATH; 389 no_std tests passed.
- QEMU/substitute: `scripts/qemu-local-shell-runtime-stdin-readiness-smoke.sh
  --quiet` passed and retained the task-owned readiness/no-data evidence log.
- QEMU/substitute regression: the successful runtime-console0 stdin regression
  remains retained at the accepted 2026-06-03 evidence path; it was used as the
  talos-console0 input-path control for this task.
- QEMU/substitute regressions scaled to touched code: stdout, stderr, fixed
  `/bin` lookup/cat/negative controls, and waitpid lifecycle smokes were run
  during implementation; their older retained evidence files were left
  unchanged.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

hardwareTestLock remained unlocked/restored and unused.

## Next Action

Accepted and committed. The queued readiness distinction closeout task is
mechanically unblocked for the next worker wake.
