# Phase 10 Userspace Stderr Through Inherited FD Closeout

Task: phase10-userspace-stderr-inherited-fd-closeout-20260603

Status: accepted

## Scope

Close out the accepted userspace stderr-through-inherited-fd frontier from
`phase10-userspace-stderr-inherited-fd-core-20260603` and decide whether the
queued stdio triad closeout remains mechanically unblocked.

This checkpoint reconciles the accepted stderr implementation record, retained
QEMU/substitute stderr, stdout, and stdin evidence, descriptor/write lineage,
VFS exec lineage, lifecycle/status, `waitpid`, non-consuming `laststatus`,
retained regressions, deferred surfaces, and residual risks. It does not
implement code, add shell features, run Pi 5 hardware, publish boot archives,
acquire `hardwareTestLock`, split stdout/stderr streams, or broaden stdio
semantics beyond the accepted fd2 fixture.

## Findings And Dispositions

- fixed: Reconciled `exec stderr` as the first accepted
  process-originated inherited fd2 write path. The shell resolves the bare name
  through the accepted fixed `/bin` lookup to `/bin/stderr`, reads that
  executable through descriptor-backed VFS/open/read, and launches it through
  the accepted loader, startup ABI, descriptor inheritance, lifecycle/status,
  `waitpid`, and `laststatus` chain.
- fixed: Confirmed the fd2 write is tied to the launched process descriptor
  table: `exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f
  return=0x000000000000001f source=userspace-talos-write`.
- fixed: Confirmed the visible line `Talos userspace stderr fixture` is
  emitted by the accepted TalosWrite-style process descriptor path, not by a
  kernel shell parser shortcut or metadata-only launch record.
- fixed: Confirmed inherited descriptor lineage remains explicit:
  `fd0=stdio-input`, `fd1=stdio-output`, `fd2=stdio-output`,
  `inherited-count=0x3`, and `loader-temp-open=false` for the stderr exec.
- fixed: Confirmed process-observation behavior remains covered. The
  `/bin/stderr` lifecycle record reports zero status, consuming `waitpid`
  observes the record, and non-consuming `laststatus` reports the same latest
  lifecycle identity/status after `waitpid`.
- fixed: Confirmed retained regressions remain covered: userspace stdout,
  userspace stdin, fixed `/bin` lookup, literal argv, `/bin/status42`
  nonzero status, `/bin/init` and `/bin/zero` zero-status controls,
  deterministic negative exec cases, loader temporary descriptor non-leak,
  inherited standard descriptors, and descriptor-backed `cat /etc/banner.txt`.
- not-an-issue: `fd2` currently shares the accepted `stdio-output` backend
  with `fd1`. This closeout accepts descriptor identity and
  process-originated fd2 writes, not separate stdout/stderr stream routing or
  terminal policy.
- deferred: Distinct stderr stream separation, pipes, redirection, terminal
  policy, blocking I/O, fork/async execution, libc stdio, writable filesystem
  behavior, broader shell grammar, hardware proof, networking, and SSH remain
  outside this frontier.

## Accepted Frontier

The accepted local shell execution and I/O frontier now includes:

- absolute VFS exec and fixed `/bin` bare-name exec for the accepted fixtures;
- literal argv propagation with canonical resolved path argv0 for bare-name
  exec and deterministic empty envp;
- inherited standard descriptor records for successful exec paths, with
  loader/VFS temporary descriptor non-leak evidence through
  `loader-temp-open=false`;
- process-originated stdout bytes from `/bin/stdout` through inherited
  `fd1=stdio-output` using the process descriptor `TalosWrite`
  syscall-substitute path;
- process-originated stdin bytes consumed by `/bin/stdin` through inherited
  `fd0=stdio-input` using the process descriptor `TalosRead`
  syscall-substitute path, reported through the accepted fd1 stdout path;
- process-originated stderr fixture bytes from `/bin/stderr` through
  inherited `fd2=stdio-output` using the process descriptor `TalosWrite`
  syscall-substitute path;
- zero and nonzero status controls, consuming `waitpid`, non-consuming
  `laststatus`, deterministic negative exec controls, unsupported grammar
  rejection, and descriptor-backed `cat /etc/banner.txt` regressions.

This closeout does not accept distinct stdout/stderr stream routing,
runtime-console0/TTY-backed process stdin, EOF/no-data/error stdin variants,
terminal blocking behavior, pipes, redirection, general POSIX shell grammar,
libc stdio, writable filesystem behavior, Pi 5 hardware behavior, networking,
or SSH.

## Evidence Map

- accepted implementation record:
  `tasks/2026-06-03-phase10-userspace-stderr-inherited-fd-core.md`.
- accepted implementation commit:
  `6377a331e8a4190a82ebf8983f799f10136bc85e`.
- QEMU/substitute userspace stderr transcript:
  `tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log`.
  Static inspection found `exec stderr`, visible emitted line
  `Talos userspace stderr fixture`, resolved `/bin/stderr`,
  descriptor-backed `source=vfs-open-read`, inherited `fd0`/`fd1`/`fd2`,
  `loader-temp-open=false`, deterministic empty envp, argv0 `/bin/stderr`,
  `exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f
  return=0x000000000000001f source=userspace-talos-write`, zero lifecycle
  status, matching `waitpid` and `laststatus`, `/bin/status42` nonzero
  regression, `/bin/init` and `/bin/zero` zero-status controls,
  deterministic negative exec cases, descriptor-backed `cat /etc/banner.txt`,
  final participants=18 expected=18 errors=0, classification
  `qemu-local-shell-userspace-stderr-complete`, and PASS.
- QEMU/substitute userspace stdout regression transcript:
  `tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log`.
  Static inspection found `exec stdout`, `exec-stdout fd=0x0000000000000001
  bytes=0x000000000000001f return=0x000000000000001f
  source=userspace-talos-write`, classification
  `qemu-local-shell-userspace-stdout-complete`, and PASS.
- QEMU/substitute userspace stdin regression transcript:
  `tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log`.
  Static inspection found `exec stdin`, visible emitted stdout report
  `Talos userspace stdin fixture read: talos-fd0`,
  `exec-stdin fd=0x0000000000000000 bytes=0x000000000000000a
  return=0x000000000000000a stdout-fd=0x0000000000000001
  stdout-bytes=0x000000000000002f stdout-return=0x000000000000002f
  source=userspace-talos-read+userspace-talos-write`, classification
  `qemu-local-shell-userspace-stdin-complete`, and PASS.

## Next Feature Recommendation

The queued stdio triad closeout remains mechanically unblocked from the
accepted evidence. The local shell execution frontier now has one
process-originated operation for each inherited standard descriptor: fd0 read,
fd1 write, and fd2 write, all tied to VFS-backed userspace fixtures,
descriptor inheritance records, lifecycle/status, `waitpid`, and
`laststatus`.

The next closeout should reconcile that triad as the accepted stdio frontier
and guard against acceptance drift: kernel built-ins and regression/control
surfaces are not userspace stdio capability; the accepted capability is the
process descriptor TalosRead/TalosWrite operation from launched VFS-backed
fixtures. Pipes, redirection, distinct stderr routing, terminal policy,
blocking I/O, writable filesystem behavior, hardware proof, networking, and
SSH should remain deferred until explicitly planned.

## Validation

- static inspection: accepted task records and retained QEMU/substitute
  evidence logs were inspected.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

`hardwareTestLock` remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
