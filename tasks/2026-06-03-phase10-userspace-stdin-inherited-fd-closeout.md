# Phase 10 Userspace Stdin Through Inherited FD Closeout

Task: phase10-userspace-stdin-inherited-fd-closeout-20260603

Status: accepted

## Scope

Close out the accepted userspace stdin-through-inherited-fd frontier from
`phase10-userspace-stdin-inherited-fd-core-20260603` and decide whether the
queued stderr-through-inherited-fd task remains mechanically unblocked.

This checkpoint reconciles the accepted stdin implementation record, retained
QEMU/substitute stdin and stdout evidence, descriptor/read/write lineage, VFS
exec lineage, lifecycle/status, `waitpid`, non-consuming `laststatus`,
retained regressions, deferred surfaces, and residual risks. It does not
implement code, add shell features, run Pi 5 hardware, publish boot archives,
acquire `hardwareTestLock`, broaden stdin semantics, or add stderr behavior.

## Findings And Dispositions

- fixed: Reconciled `exec stdin` as the first accepted process-originated
  stdin read path. The shell resolves the bare name through the accepted fixed
  `/bin` lookup to `/bin/stdin`, reads that executable through
  descriptor-backed VFS/open/read, and launches it through the accepted loader,
  startup ABI, descriptor inheritance, lifecycle/status, `waitpid`, and
  `laststatus` chain.
- fixed: Confirmed the fd0 read is tied to the launched process descriptor
  table: `exec-stdin fd=0x0000000000000000 bytes=0x000000000000000a
  return=0x000000000000000a stdout-fd=0x0000000000000001
  stdout-bytes=0x000000000000002f stdout-return=0x000000000000002f
  source=userspace-talos-read+userspace-talos-write`.
- fixed: Confirmed the visible report
  `Talos userspace stdin fixture read: talos-fd0` is emitted through the
  already accepted userspace stdout path on inherited `fd1=stdio-output`,
  not through a kernel shell parser shortcut or metadata-only launch record.
- fixed: Confirmed inherited descriptor lineage remains explicit:
  `fd0=stdio-input`, `fd1=stdio-output`, `fd2=stdio-output`,
  `inherited-count=0x3`, and `loader-temp-open=false` for the stdin exec.
- fixed: Confirmed process-observation behavior remains covered. The
  `/bin/stdin` lifecycle record reports zero status, consuming `waitpid`
  observes the record, and non-consuming `laststatus` reports the same latest
  lifecycle identity/status after `waitpid`.
- fixed: Confirmed retained regressions remain covered: userspace stdout,
  fixed `/bin` lookup, absolute exec, literal argv, `/bin/init` and
  `/bin/zero` zero-status controls, deterministic negative exec cases,
  loader temporary descriptor non-leak, inherited standard descriptors, and
  descriptor-backed `cat /etc/banner.txt`.
- not-an-issue: The accepted stdin fixture proves one deterministic bounded
  TalosRead-style userspace input path from the inherited fd0 descriptor. It is
  intentionally not a runtime-console0/TTY-backed stdin implementation, libc
  stdio implementation, blocking I/O policy, or hardware claim.
- deferred: Runtime-console0/TTY-backed process stdin, EOF/no-data/error
  variants, stderr-specific output, blocking I/O, pipes, redirection,
  fork/async execution, libc stdio, writable filesystem behavior, broader
  shell grammar, hardware proof, networking, and SSH remain outside this
  frontier.

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
- zero status controls, consuming `waitpid`, non-consuming `laststatus`,
  deterministic negative exec controls, unsupported grammar rejection, and
  descriptor-backed `cat /etc/banner.txt` regressions.

This closeout does not accept runtime-console0/TTY-backed process stdin,
EOF/no-data/error stdin variants, stderr-specific userspace output, terminal
blocking behavior, pipes, redirection, general POSIX shell grammar, libc
stdio, writable filesystem behavior, Pi 5 hardware behavior, networking, or
SSH.

## Evidence Map

- accepted implementation record:
  `tasks/2026-06-03-phase10-userspace-stdin-inherited-fd-core.md`.
- accepted implementation commit:
  `3c9e1adae27a37b8df753af72edf429743d40847`.
- QEMU/substitute userspace stdin transcript:
  `tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log`.
  Static inspection found `exec stdin`, visible emitted stdout report
  `Talos userspace stdin fixture read: talos-fd0`, resolved `/bin/stdin`,
  descriptor-backed `source=vfs-open-read`, inherited `fd0`/`fd1`/`fd2`,
  `loader-temp-open=false`, deterministic empty envp, argv0 `/bin/stdin`,
  `exec-stdin fd=0x0000000000000000 bytes=0x000000000000000a
  return=0x000000000000000a stdout-fd=0x0000000000000001
  stdout-bytes=0x000000000000002f stdout-return=0x000000000000002f
  source=userspace-talos-read+userspace-talos-write`, zero lifecycle status,
  matching `waitpid` and `laststatus`, retained stdout fixture, retained
  `/bin/init` and `/bin/zero` controls, literal argv control, deterministic
  negative exec cases, descriptor-backed `cat /etc/banner.txt`, final
  participants=18 expected=18 errors=0, classification
  `qemu-local-shell-userspace-stdin-complete`, and PASS.
- QEMU/substitute userspace stdout regression transcript:
  `tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log`.
  Static inspection found `exec stdout`, visible emitted stdout
  `Talos userspace stdout fixture`, `exec-stdout fd=0x0000000000000001
  bytes=0x000000000000001f return=0x000000000000001f
  source=userspace-talos-write`, descriptor inheritance, matching `waitpid`
  and `laststatus`, descriptor-backed `cat /etc/banner.txt`, classification
  `qemu-local-shell-userspace-stdout-complete`, and PASS.

## Next Feature Recommendation

The queued stderr-through-inherited-fd task remains mechanically unblocked from
the accepted evidence. The descriptor inheritance frontier already records
`fd2=stdio-output` for successful VFS-backed exec paths, and the accepted
stdout/stdin slices prove process descriptor TalosWrite and TalosRead
operations from launched userspace fixtures with retained lifecycle/status,
`waitpid`, `laststatus`, fixed `/bin` lookup, and VFS exec regressions.

The next feature-led local I/O primitive should stay narrow: one read-only
executable fixture, one bounded userspace write to inherited `fd2`, a
stderr-specific visible line and byte-count record, retained stdout and stdin
regressions, and retained `/bin/status42`, `/bin/init`, `/bin/zero`,
`waitpid`, `laststatus`, loader temporary descriptor non-leak, negative
exec, and descriptor-backed cat regressions. Pipes, redirection, terminal
coloring/policy, blocking I/O, writable filesystem behavior, libc stdio,
hardware proof, networking, and SSH should remain deferred.

## Validation

- static inspection: accepted task records and retained QEMU/substitute
  evidence logs were inspected.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

`hardwareTestLock` remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
