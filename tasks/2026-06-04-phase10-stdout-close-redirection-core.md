# Phase 10 Stdout Close Redirection Core

Task: phase10-stdout-close-redirection-core-20260604

Status: accepted

## Summary

Implemented the first exact descriptor-close redirection slice:
`exec stdout 1>&-`. The command still resolves `/bin/stdout` through the
accepted fixed `/bin` lookup and descriptor-backed VFS/open/read path, then
launches the child with fd1 closed. The userspace stdout fixture attempts its
`TalosWrite` through fd1 and records the deterministic closed-descriptor
result:

`exec-stdout ... return=0xfffffffffffffff7 stream=closed route=closed-descriptor source=userspace-talos-write`

The shell descriptor table is restored after the child launch; the following
normal `exec stdout` control records fd1 as
`stream=stdout route=runtime-console0/stdout`.

## Findings And Disposition

- fixed: Added the bounded `1>&-` redirection token to shell exec parsing
  without admitting arbitrary `N>&-` close syntax or including the token in
  argv.
- fixed: Applied descriptor close only around the launched child table and
  restored the original shell fd1 entry before returning to the prompt.
- fixed: Relaxed descriptor inheritance reporting for this exact child-only
  case so fd1 is recorded as `closed` with inherited-count `2` while fd0 and
  fd2 remain standard descriptors.
- fixed: Preserved userspace stdout fixture execution on closed fd1 by
  recording the `-EBADF` write result instead of treating it as a launch
  failure.
- fixed: Added task-owned unit and QEMU/substitute coverage for `exec stdout
  1>&-`, shell fd1 restoration, unsupported `2>&-`, file redirection, and pipe
  redirection negatives.
- not-an-issue: The physical stdout/stderr sink remains shared
  runtime-console0; this task accepts descriptor-table close semantics and
  route metadata, not separate physical streams.
- deferred: stderr `2>&-`, arbitrary descriptor close syntax, descriptor
  moves, regular-file redirection, append/truncate, pipes, here-docs, writable
  filesystem behavior, async execution, fork, signals, libc stdio, Pi 5 proof,
  networking, and SSH remain out of scope.

## Evidence Map

- stdout descriptor-close redirection:
  `tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log`.
  QEMU/substitute evidence shows `exec stdout 1>&-`, descriptor-backed
  `/bin/stdout` VFS/open/read lineage, fd1 closed in the child descriptor
  evidence, `exec-redirection op=close source-fd=1 ... child-only=true
  shell-restored=true`, fd1 userspace write return `-EBADF`, lifecycle/status,
  `waitpid`, `laststatus`, unsupported redirection negatives,
  descriptor-backed `cat /etc/banner.txt`, final
  `qemu-local-shell-stdout-close-redirection-complete`, and PASS.
- normal stdout restoration control:
  `tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log`
  and retained
  `tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log`.
  Both record normal fd1 as `stream=stdout route=runtime-console0/stdout`.
- descriptor-dup direction controls:
  `tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log`
  and
  `tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log`.
- distinct stderr route control:
  `tasks/evidence/2026-06-04-phase10-distinct-stderr-routing-core/qemu-local-shell-distinct-stderr-routing-smoke.log`.
- stdin EOF/readiness control:
  `tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log`.

## Accepted Frontier

Accepted:

- `exec stdout 1>&-` parses as the first exact descriptor-close redirection
  form.
- Redirection is child-only for the launched VFS-backed executable.
- `/bin/stdout` remains loaded through descriptor-backed VFS/open/read and
  attempts its fd1 `TalosWrite`; closed fd1 returns `-EBADF`.
- The shell fd1 descriptor is restored after the child exec; a following
  normal `exec stdout` records the stdout route again.
- Unsupported `2>&-`, file, and pipe redirection forms fail deterministically.

Deferred:

- stderr `2>&-` and arbitrary `N>&-`;
- descriptor moves, regular-file redirection, append/truncate, and pipes;
- writable filesystem behavior, separate physical sinks, async jobs, fork,
  signals, libc stdio, Pi 5 proof, networking, and SSH.

## Validation Summary

- formatting: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute:
  `scripts/qemu-local-shell-stdout-close-redirection-smoke.sh` passed with
  retained task evidence.
- QEMU/substitute controls:
  `scripts/qemu-local-shell-stdout-to-stderr-redirection-smoke.sh`,
  `scripts/qemu-local-shell-stderr-to-stdout-redirection-smoke.sh`,
  `scripts/qemu-local-shell-userspace-stdout-smoke.sh`,
  `scripts/qemu-local-shell-distinct-stderr-routing-smoke.sh`, and
  `scripts/qemu-local-shell-terminal-ctrl-d-eof-smoke.sh` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- diff checks: `git diff --check` and `git diff --cached --check` passed.

hardwareTestLock remained unlocked/restored and unused.
