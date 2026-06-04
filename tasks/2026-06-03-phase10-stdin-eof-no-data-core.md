# Phase 10 Stdin EOF/No-Data Core

Task: phase10-stdin-eof-no-data-core-20260603

Status: accepted

## Summary

Accepted a deterministic EOF/no-data branch for userspace inherited fd0 reads.
Shell-visible exec stdin still resolves through the accepted fixed /bin lookup
to /bin/stdin, loads through descriptor-backed VFS/open/read, and runs through
the accepted loader, startup ABI, descriptor inheritance, lifecycle/status,
waitpid, and laststatus chain. When the runtime-console0 local-input backend
has no bytes immediately available after the command terminator, the launched
fixture now treats the fd0 TalosRead return value 0 as an EOF/no-data
observation instead of a successful byte read.

The fixture reports Talos userspace stdin fixture no-data: eof through
inherited fd1 using the accepted userspace TalosWrite path and records
read-result=eof/no-data on the exec-stdin evidence line. The successful
runtime-console0 talos-console0 read remains retained as the regression for the
happy path.

## Findings And Disposition

- fixed: allowed the /bin/stdin fixture to accept a zero-byte
  runtime-console0/local-input fd0 read as a deterministic EOF/no-data result
  instead of failing the exec pipeline.
- fixed: added a distinct stdout report and exec-stdin evidence marker for
  EOF/no-data so the transcript cannot be confused with a successful byte read,
  invalid descriptor, or unrelated syscall error.
- fixed: added scripts/qemu-local-shell-stdin-eof-no-data-smoke.sh to drive
  exec stdin without appending stdin payload bytes, while retaining the
  successful runtime-console0 stdin smoke as a regression.
- not-an-issue: the QEMU boot scenario label/classification remain the
  accepted qemu-local-shell-userspace-stdin values; the EOF/no-data evidence is
  separated by script name, evidence directory, visible no-data report, and
  read-result=eof/no-data.
- deferred: blocking scheduler I/O, readiness/polling APIs, canonical terminal
  policy expansion, signals, async execution, fork, pipes, redirection,
  writable filesystem behavior, libc stdio, distinct stderr stream routing,
  Pi 5 proof, networking, and SSH remain out of scope.

## Evidence

- QEMU/substitute EOF/no-data stdin smoke:
  tasks/evidence/2026-06-03-phase10-stdin-eof-no-data-core/qemu-local-shell-stdin-eof-no-data-smoke.log.
  The log contains exec stdin, the visible report Talos userspace stdin fixture
  no-data: eof, descriptor-backed source=vfs-open-read, inherited fd0/fd1/fd2,
  loader-temp-open=false, argv0 /bin/stdin, deterministic empty envp,
  exec-stdin fd=0x0000000000000000 bytes=0x0000000000000000
  return=0x0000000000000000 read-source=runtime-console0/local-input
  stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000002b
  stdout-return=0x000000000000002b
  source=userspace-talos-read+userspace-talos-write read-result=eof/no-data,
  zero lifecycle status for /bin/stdin, matching waitpid and non-consuming
  laststatus, retained stdout fixture, zero/nonzero status controls, fixed
  /bin lookup, negative exec controls, descriptor-backed cat /etc/banner.txt,
  classification=qemu-local-shell-userspace-stdin-complete, and PASS.
- retained QEMU/substitute successful runtime-console0 stdin regression:
  tasks/evidence/2026-06-03-phase10-runtime-console0-stdin-core/qemu-local-shell-runtime-console0-stdin-smoke.log.
  The log contains Talos userspace stdin fixture read: talos-console0 and
  exec-stdin ... bytes=0x000000000000000e return=0x000000000000000e
  read-source=runtime-console0/local-input.

## Validation Summary

- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet passed under QEMU with the
  Talos toolchain PATH.
- QEMU/substitute: scripts/qemu-local-shell-stdin-eof-no-data-smoke.sh passed
  and retained the EOF/no-data evidence log above.
- QEMU/substitute regression: scripts/qemu-local-shell-runtime-console0-stdin-smoke.sh
  passed and retained the successful runtime-console0 stdin evidence log above.
- static inspection: git diff --check passed.
- documentation: /home/node/.cargo/bin/mdbook build passed with the existing
  large search-index warning.
- staged whitespace inspection: recorded before commit.

## Next Action

Accepted and committed. The queued stdin EOF/no-data closeout task is
mechanically unblocked for the next worker wake.
