# Phase 10 Runtime-Console0 Userspace Stdin Core

Task: phase10-runtime-console0-stdin-core-20260603

Status: accepted

## Summary

Accepted the first runtime-console0-backed userspace stdin slice. Shell-visible
`exec stdin` still resolves through the accepted fixed `/bin` lookup to
`/bin/stdin`, reads the executable through descriptor-backed VFS/open/read,
and runs it through the accepted loader, startup ABI, descriptor inheritance,
lifecycle/status, `waitpid`, and `laststatus` chain. The launched fixture
now performs one bounded `TalosRead` through inherited `fd0=stdio-input`
from the same runtime-console0/local input backend used by the prompt instead
of constructing a deterministic `FixedStdin` proof buffer.

The retained QEMU/substitute smoke feeds `talos-console0` immediately after
the `exec stdin` command terminator. The shell parser consumes only the
command line, then the launched fixture consumes the remaining runtime-console0
input bytes through fd0 and reports
`Talos userspace stdin fixture read: talos-console0` through inherited fd1
using the accepted userspace `TalosWrite` path.

## Findings And Disposition

- fixed: replaced the `/bin/stdin` fixture's `FixedStdin` proof-buffer source
  with a runtime-console0/local-input source passed through the same
  descriptor-backed `TalosRead` syscall dispatcher used for initramfs-backed
  shell exec.
- fixed: added `read_descriptor_from_console_input` beside the existing fixed
  stdin reader, preserving the accepted fixed-stdin dispatcher for older Phase
  7 proof-buffer evidence and using a sibling initramfs dispatcher only where
  console-backed stdin is explicit.
- fixed: updated the shell transcript to record
  `read-source=runtime-console0/local-input`, preventing acceptance drift back
  to proof-buffer stdin.
- fixed: added `scripts/qemu-local-shell-runtime-console0-stdin-smoke.sh` so
  runtime-console0 stdin evidence is retained under this task instead of
  overwriting older proof-buffer evidence.
- not-an-issue: the older proof-buffer stdin transcript remains retained
  evidence for the previous frontier; it is no longer counted as acceptance for
  runtime-console0-backed stdin.
- deferred: EOF/no-data/error stdin variants, blocking scheduler I/O, terminal
  canonical policy expansion, async execution, fork, signals, pipes,
  redirection, distinct stderr stream routing, writable filesystem behavior,
  libc stdio, Pi 5 proof, networking, and SSH remain out of scope.

## Evidence

- QEMU/substitute runtime-console0 stdin smoke:
  `tasks/evidence/2026-06-03-phase10-runtime-console0-stdin-core/qemu-local-shell-runtime-console0-stdin-smoke.log`.
  The log contains `exec stdin`, the visible report
  `Talos userspace stdin fixture read: talos-console0`, descriptor-backed
  `source=vfs-open-read`, inherited `fd0`/`fd1`/`fd2`,
  `loader-temp-open=false`, argv0 `/bin/stdin`, deterministic empty envp,
  `exec-stdin fd=0x0000000000000000 bytes=0x000000000000000e
  return=0x000000000000000e read-source=runtime-console0/local-input
  stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000033
  stdout-return=0x0000000000000033
  source=userspace-talos-read+userspace-talos-write`, zero lifecycle status
  for `/bin/stdin`, matching `waitpid` and non-consuming `laststatus`,
  retained stdout fixture, fixed `/bin` lookup, negative exec controls,
  descriptor-backed `cat /etc/banner.txt`,
  `classification=qemu-local-shell-userspace-stdin-complete`, and PASS.
- retained QEMU/substitute stderr regression:
  `tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log`.
- retained QEMU/substitute proof-buffer stdin regression:
  `tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log`.
- target no_std/unit tests: `cargo -Zjson-target-spec test --quiet
  local_command_loop_execs_userspace_stdin_fixture_through_fd0` passed under
  QEMU with the Talos toolchain PATH; the runner executed 388 no_std tests.

## Validation Summary

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed under QEMU
  with 388 no_std tests.
- QEMU/substitute: `scripts/qemu-local-shell-runtime-console0-stdin-smoke.sh
  --quiet` passed and retained the runtime-console0 stdin evidence log above.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed with the
  existing large search-index warning.
- staged whitespace inspection: recorded before commit.

## Next Action

Accepted and committed. The queued runtime-console0 stdin closeout task is
mechanically unblocked for the next worker wake.
