# Phase 10 Userspace Stdout Through Inherited FD Core

Task: phase10-userspace-stdout-inherited-fd-core-20260603

## Summary

Accepted the minimal userspace stdout slice for VFS-backed shell exec. The
read-only initramfs now includes `/bin/stdout`, a bounded executable fixture
that reaches the accepted descriptor-backed VFS/open/read, loader, startup ABI,
launch, descriptor inheritance, lifecycle/status, waitpid, and laststatus path.
When launched by shell-visible `exec stdout`, it emits
`Talos userspace stdout fixture` through inherited `fd1=stdio-output` by using
the process descriptor `TalosWrite` syscall-substitute path.

The stdout fixture keeps execution intentionally narrow. It proves one
deterministic userspace output payload and records the write byte count and
return value. It does not add pipes, redirection, stderr expansion, stdin
reads, blocking I/O, fork, async execution, libc stdio, environment-backed PATH,
current-directory search, broader grammar, writable filesystem behavior, Pi 5
hardware proof, networking, or SSH.

## Findings And Disposition

- fixed: accepted exec fixtures previously reported launch/status records but
  did not emit process-originated stdout bytes. Added `/bin/stdout` and a
  bounded `TalosWrite` dispatch through the launched process descriptor table.
- fixed: `exec stdout` now resolves through the accepted fixed `/bin` lookup,
  reads `/bin/stdout` through descriptor-backed VFS/open/read, and records
  `exec-stdout fd=0x1 bytes=0x1f return=0x1f source=userspace-talos-write`.
- fixed: added `qemu_local_shell_stdout` plus
  `scripts/qemu-local-shell-userspace-stdout-smoke.sh` to retain stdout,
  lifecycle/status, waitpid, laststatus, nonzero `/bin/status42`, zero-status
  `/bin/init` and `/bin/zero`, absolute exec, fixed lookup, negative exec, and
  descriptor-backed cat evidence.
- fixed: `/bin` listings now include `stdout`; updated affected local command
  tests and QEMU smoke expectations.
- not-an-issue: `/bin/status42`, `/bin/init`, and `/bin/zero` remain status-only
  controls and do not emit stdout bytes.
- deferred: stdin reads through inherited `fd0`, stderr-specific output,
  pipes, redirection, fork/async execution, libc stdio, writable filesystem
  behavior, hardware proof, networking, and SSH remain out of scope.

## Evidence

- unit tests: `cargo -Zjson-target-spec test --quiet` passed with 386 no_std
  tests, including
  `local_command_loop_execs_userspace_stdout_fixture_through_fd1`.
- QEMU/substitute stdout smoke:
  `tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log`.
  The log contains `exec stdout`, the emitted line
  `Talos userspace stdout fixture`, `talos: exec path=/bin/stdout
  source=vfs-open-read`, inherited `fd0`/`fd1`/`fd2`, loader temporary
  descriptor non-leak, `argv0=/bin/stdout`, deterministic empty envp,
  `exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f
  return=0x000000000000001f source=userspace-talos-write`, zero lifecycle
  status for `/bin/stdout`, matching `waitpid` and `laststatus`, retained
  `/bin/status42` nonzero control, retained `/bin/init` and `/bin/zero`
  zero-status controls, deterministic negative exec cases, descriptor-backed
  `cat /etc/banner.txt`,
  `classification=qemu-local-shell-userspace-stdout-complete`, and PASS.
- QEMU/substitute regression:
  `tasks/evidence/2026-06-03-phase10-minimal-path-lookup-exec-core/qemu-local-shell-path-lookup-smoke.log`
  was rerun and passed with the updated builtins frontier.
- diff hygiene: `git diff --check` passed.
- docs validation: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff hygiene: `git diff --cached --check` passed before commit.

## Validation Summary

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute: `scripts/qemu-local-shell-userspace-stdout-smoke.sh --quiet`
  passed and retained the stdout evidence log above.
- QEMU/substitute regression: `scripts/qemu-local-shell-path-lookup-smoke.sh
  --quiet` passed and refreshed the retained path lookup evidence log.
- docs: `/home/node/.cargo/bin/mdbook build` passed.

## Next Action

Promote `phase10-userspace-stdout-inherited-fd-closeout-20260603` after this
task is accepted and committed. The closeout should reconcile the stdout
frontier before stdin, pipes, redirection, writable filesystem behavior,
broader shell grammar, hardware proof, networking, or SSH planning.
