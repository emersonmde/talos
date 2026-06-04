# Phase 10 Stderr Close Redirection Core

Task: phase10-stderr-close-redirection-core-20260604

Status: accepted

## Summary

Implemented the inverse exact descriptor-close redirection slice:
`exec stderr 2>&-`. The command resolves `/bin/stderr` through the accepted
fixed `/bin` lookup and descriptor-backed VFS/open/read path, then launches
the child with fd2 closed. The userspace stderr fixture attempts its
`TalosWrite` through fd2 and records the deterministic closed-descriptor
result:

`exec-stderr ... return=0xfffffffffffffff7 stream=closed route=closed-descriptor source=userspace-talos-write`

The shell descriptor table is restored after the child launch; the following
normal `exec stderr` control records fd2 as
`stream=stderr route=runtime-console0/stderr`.

## Findings And Disposition

- fixed: Added the bounded `2>&-` redirection token to shell exec parsing
  without admitting arbitrary `N>&-` close syntax or including the token in
  argv.
- fixed: Reused the child-only descriptor redirection path so fd2 is closed
  only while launching the child and the original shell fd2 entry is restored
  before returning to the prompt.
- fixed: Generalized descriptor inheritance reporting so either accepted exact
  close form can record one closed standard output descriptor and
  inherited-count `2`.
- fixed: Preserved userspace stderr fixture execution on closed fd2 by
  recording the `-EBADF` write result instead of treating it as a launch
  failure.
- fixed: Added task-owned unit and QEMU/substitute coverage for `exec stderr
  2>&-`, shell fd2 restoration, retained `1>&-`, deterministic unsupported
  file redirection, and descriptor-backed `cat /etc/banner.txt`.
- not-an-issue: The physical stdout/stderr sink remains shared
  runtime-console0; this task accepts descriptor-table close semantics and
  route metadata, not separate physical streams.
- deferred: arbitrary descriptor close syntax, descriptor moves, regular-file
  redirection, append/truncate, pipes, here-docs, writable filesystem
  behavior, async execution, fork, signals, libc stdio, Pi 5 proof,
  networking, and SSH remain out of scope.

## Evidence Map

- stderr descriptor-close redirection:
  `tasks/evidence/2026-06-04-phase10-stderr-close-redirection-core/qemu-local-shell-stderr-close-redirection-smoke.log`.
  QEMU/substitute evidence shows `exec stderr 2>&-`, descriptor-backed
  `/bin/stderr` VFS/open/read lineage, fd2 closed in the child descriptor
  evidence, `exec-redirection op=close source-fd=2 ... child-only=true
  shell-restored=true`, fd2 userspace write return `-EBADF`, lifecycle/status,
  `waitpid`, `laststatus`, normal stderr restoration, retained stdout close,
  unsupported redirection negative, descriptor-backed `cat /etc/banner.txt`,
  final `qemu-local-shell-stderr-close-redirection-complete`, and PASS.
- stdout descriptor-close regression:
  `tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log`.
- descriptor-dup direction controls:
  `tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log`
  and
  `tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log`.
- normal stdout/stderr route controls:
  `tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log`
  and
  `tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log`.
- stdin EOF/readiness control:
  `tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log`.

## Accepted Frontier

Accepted:

- `exec stderr 2>&-` parses as the inverse exact descriptor-close redirection
  form.
- Redirection is child-only for the launched VFS-backed executable.
- `/bin/stderr` remains loaded through descriptor-backed VFS/open/read and
  attempts its fd2 `TalosWrite`; closed fd2 returns `-EBADF`.
- The shell fd2 descriptor is restored after the child exec; a following
  normal `exec stderr` records the stderr route again.
- The accepted `exec stdout 1>&-` behavior remains covered as a regression.

Deferred:

- arbitrary `N>&-` and descriptor moves;
- regular-file redirection, append/truncate, here-docs, and pipes;
- writable filesystem behavior, separate physical sinks, async jobs, fork,
  signals, libc stdio, Pi 5 proof, networking, and SSH.

## Validation Summary

- formatting: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute:
  `scripts/qemu-local-shell-stderr-close-redirection-smoke.sh --quiet`
  passed with retained task evidence.
- QEMU/substitute controls:
  `scripts/qemu-local-shell-stdout-close-redirection-smoke.sh --quiet`,
  `scripts/qemu-local-shell-stdout-to-stderr-redirection-smoke.sh --quiet`,
  `scripts/qemu-local-shell-stderr-to-stdout-redirection-smoke.sh --quiet`,
  `scripts/qemu-local-shell-userspace-stdout-smoke.sh --quiet`,
  `scripts/qemu-local-shell-userspace-stderr-smoke.sh --quiet`, and
  `scripts/qemu-local-shell-terminal-ctrl-d-eof-smoke.sh --quiet` passed.
- diff checks: `git diff --check` passed before staging.

hardwareTestLock remained unlocked/restored and unused.
