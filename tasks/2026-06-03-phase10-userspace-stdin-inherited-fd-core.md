# Phase 10 Userspace Stdin Through Inherited FD Core

Task: phase10-userspace-stdin-inherited-fd-core-20260603

## Summary

Accepted the minimal userspace stdin slice for VFS-backed shell exec. The
read-only initramfs now includes `/bin/stdin`, a bounded executable fixture
that reaches the accepted descriptor-backed VFS/open/read, loader, startup ABI,
launch, descriptor inheritance, lifecycle/status, waitpid, and laststatus path.
When launched by shell-visible `exec stdin`, it performs a bounded
`TalosRead` through inherited `fd0=stdio-input`, then reports the consumed
bytes through the accepted userspace `TalosWrite` stdout path on
`fd1=stdio-output`.

The stdin fixture remains intentionally narrow. It proves one deterministic
fixed fd0 input buffer (`talos-fd0\n`), records the read byte count and return
value, and writes `Talos userspace stdin fixture read: talos-fd0` through
stdout. It does not add terminal canonical-mode expansion, blocking scheduler
I/O, async execution, signals, pipes, redirection, writable filesystem
behavior, libc stdio, broad descriptor policy, hardware proof, networking, or
SSH.

## Findings And Disposition

- fixed: accepted stdout proved process-originated `fd1` writes but stdin was
  still deferred. Added `/bin/stdin` and a bounded `TalosRead` dispatch through
  the launched process descriptor table.
- fixed: `exec stdin` now resolves through the accepted fixed `/bin` lookup,
  reads `/bin/stdin` through descriptor-backed VFS/open/read, then records
  `exec-stdin fd=0x0 bytes=0xa return=0xa stdout-fd=0x1 stdout-bytes=0x2f
  stdout-return=0x2f source=userspace-talos-read+userspace-talos-write`.
- fixed: the first implementation overlapped the stdout prefix buffer with the
  fd0 read destination, corrupting the reported bytes. The stdin read buffer
  now lives at a non-overlapping user-memory offset, and the smoke asserts the
  emitted `talos-fd0` bytes.
- fixed: added `qemu_local_shell_stdin` plus
  `scripts/qemu-local-shell-userspace-stdin-smoke.sh` to retain stdin,
  stdout, lifecycle/status, waitpid, laststatus, `/bin/init`, `/bin/zero`,
  literal argv, negative exec, and descriptor-backed cat evidence.
- fixed: `/bin` listings now include `stdin`; updated local command tests and
  QEMU smoke expectations.
- not-an-issue: the fixed fd0 byte source is a QEMU/substitute proof input
  passed through the inherited descriptor/read boundary, not shell parser
  command text.
- deferred: runtime-console0/TTY-backed stdin, EOF/no-data/error variants,
  stderr-specific output, pipes, redirection, fork/async execution, libc stdio,
  writable filesystem behavior, hardware proof, networking, and SSH remain out
  of scope.

## Evidence

- unit tests: `cargo -Zjson-target-spec test --quiet` passed with no output
  after `cargo fmt --all -- --check`, including
  `local_command_loop_execs_userspace_stdin_fixture_through_fd0` and the
  retained stdout fixture test.
- QEMU/substitute stdin smoke:
  `tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log`.
  The log contains `exec stdin`, the emitted stdout report
  `Talos userspace stdin fixture read: talos-fd0`, `talos: exec path=/bin/stdin
  source=vfs-open-read`, inherited `fd0`/`fd1`/`fd2`, loader temporary
  descriptor non-leak, `argv0=/bin/stdin`, deterministic empty envp,
  `exec-stdin fd=0x0000000000000000 bytes=0x000000000000000a
  return=0x000000000000000a stdout-fd=0x0000000000000001
  stdout-bytes=0x000000000000002f stdout-return=0x000000000000002f
  source=userspace-talos-read+userspace-talos-write`, zero lifecycle status for
  `/bin/stdin`, matching `waitpid` and `laststatus`, retained stdout fixture,
  retained `/bin/init` and `/bin/zero` controls, literal argv control,
  deterministic negative exec cases, descriptor-backed `cat /etc/banner.txt`,
  `classification=qemu-local-shell-userspace-stdin-complete`, and PASS.
- QEMU/substitute stdout regression:
  `tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log`
  was rerun and passed with the updated `/bin` fixture inventory.
- diff hygiene: `git diff --check` passed.
- docs validation: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff hygiene: `git diff --cached --check` passed before commit.

## Validation Summary

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute: `scripts/qemu-local-shell-userspace-stdin-smoke.sh --quiet`
  passed and retained the stdin evidence log above.
- QEMU/substitute regression:
  `scripts/qemu-local-shell-userspace-stdout-smoke.sh --quiet` passed and
  refreshed the retained stdout evidence log.
- docs: `/home/node/.cargo/bin/mdbook build` passed.

## Next Action

Accepted and committed. Supervisor should plan the next explicit Phase 10
feature-led local I/O task; likely candidates are stdin closeout or a bounded
stderr/userspace stdio follow-up, but no worker phase transition is inferred
from this task.
