# Phase 10 Userspace Stdio Triad Closeout

Task: phase10-userspace-stdio-triad-closeout-20260603

Status: accepted

## Scope

Close out the accepted userspace standard stdio frontier after the accepted
fd1 stdout write, fd0 stdin read, and fd2 stderr write slices.

This checkpoint reconciles accepted userspace stdin, stdout, and stderr task
records; descriptor inheritance; descriptor-backed VFS exec; loader, startup,
launch, lifecycle/status, `waitpid`, and `laststatus` lineage; and retained
regression evidence. It does not implement code, add shell features, run Pi 5
hardware, publish boot archives, acquire `hardwareTestLock`, add pipes or
redirection, or plan the remaining Phase 10 backlog.

## Findings And Dispositions

- fixed: Reconciled the accepted process-originated fd1 write frontier.
  `exec stdout` resolves through the accepted fixed `/bin` lookup to
  `/bin/stdout`, reads the executable through descriptor-backed VFS/open/read,
  launches it through the accepted loader/startup/lifecycle chain, and emits
  `Talos userspace stdout fixture` through inherited `fd1=stdio-output`.
- fixed: Reconciled the accepted process-originated fd0 read frontier.
  `exec stdin` resolves to `/bin/stdin`, reads the deterministic proof input
  `talos-fd0\n` through inherited `fd0=stdio-input`, and reports
  `Talos userspace stdin fixture read: talos-fd0` through the accepted fd1
  stdout write path.
- fixed: Reconciled the accepted process-originated fd2 write frontier.
  `exec stderr` resolves to `/bin/stderr` and emits
  `Talos userspace stderr fixture` through inherited `fd2=stdio-output` with
  a process descriptor TalosWrite-style record.
- fixed: Confirmed the accepted stdio operations are tied to launched process
  descriptor tables, not shell built-ins or metadata-only launch reports:
  `exec-stdout fd=1 bytes=0x1f return=0x1f source=userspace-talos-write`,
  `exec-stdin fd=0 bytes=0xa return=0xa stdout-fd=1 stdout-bytes=0x2f
  stdout-return=0x2f source=userspace-talos-read+userspace-talos-write`, and
  `exec-stderr fd=2 bytes=0x1f return=0x1f source=userspace-talos-write`.
- fixed: Confirmed descriptor inheritance remains explicit for successful
  VFS-backed exec paths: `fd0=stdio-input`, `fd1=stdio-output`,
  `fd2=stdio-output`, `inherited-count=0x3`, and
  `loader-temp-open=false`.
- fixed: Confirmed retained process and file regressions remain covered across
  the accepted stdio evidence: lifecycle/status records, consuming `waitpid`,
  non-consuming `laststatus`, fixed `/bin` lookup, absolute exec, literal argv,
  `/bin/status42` nonzero status, `/bin/init` and `/bin/zero` zero-status
  controls, deterministic negative exec controls, loader temporary descriptor
  non-leak, and descriptor-backed `cat /etc/banner.txt`.
- not-an-issue: The accepted `stdio` shell built-in and older kernel-backed
  command-loop output remain regression/control surfaces. They are not counted
  as userspace stdio capability; the accepted capability is the launched
  VFS-backed fixture using inherited process descriptors.
- not-an-issue: `fd2` currently shares the accepted `stdio-output` backend with
  `fd1`. This checkpoint accepts fd2 descriptor identity and
  process-originated fd2 writes, not separate stdout/stderr stream routing.
- deferred: Runtime-console0/TTY-backed process stdin, EOF/no-data/error
  stdin variants beyond the accepted proof input, blocking scheduler I/O,
  close/dup/fork inheritance policy, pipes, redirection, distinct stderr stream
  separation, writable filesystem behavior, broad shell grammar, libc stdio,
  Pi 5 proof, networking, and SSH remain outside this frontier.

## Accepted Frontier

The accepted local shell execution and standard stdio frontier now includes:

- descriptor-backed absolute VFS exec and fixed `/bin` bare-name exec for the
  accepted userspace fixtures;
- literal argv propagation, canonical resolved path argv0 for bare-name exec,
  deterministic empty envp, and inherited standard descriptor records;
- process-originated stdout bytes from `/bin/stdout` through inherited fd1
  using the TalosWrite syscall-substitute path;
- process-originated stdin bytes consumed by `/bin/stdin` through inherited
  fd0 using the TalosRead syscall-substitute path, with reporting through the
  accepted fd1 write path;
- process-originated stderr fixture bytes from `/bin/stderr` through inherited
  fd2 using the TalosWrite syscall-substitute path;
- loader/VFS temporary descriptor non-inheritance, lifecycle/status,
  `waitpid`, non-consuming `laststatus`, deterministic negative exec controls,
  and descriptor-backed `cat /etc/banner.txt` regressions.

This closeout does not accept a userspace shell, POSIX libc stdio, terminal
session semantics, runtime-console0/TTY-backed process stdin, blocking I/O,
pipes, redirection, distinct stderr routing, broad descriptor policy, writable
filesystem behavior, Pi 5 hardware behavior, networking, or SSH.

## Evidence Map

- accepted stdout implementation record:
  `tasks/2026-06-03-phase10-userspace-stdout-inherited-fd-core.md`.
- accepted stdout closeout:
  `tasks/2026-06-03-phase10-userspace-stdout-inherited-fd-closeout.md`.
- stdout implementation commit:
  `7a49e4ae61aac8611a95a26018d7d52c0cb3183f`.
- QEMU/substitute userspace stdout transcript:
  `tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log`.
  Static inspection found visible emitted stdout
  `Talos userspace stdout fixture`, resolved `/bin/stdout`,
  descriptor-backed `source=vfs-open-read`, inherited `fd0`/`fd1`/`fd2`,
  `loader-temp-open=false`, argv0 `/bin/stdout`,
  `exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f
  return=0x000000000000001f source=userspace-talos-write`, lifecycle/status,
  `waitpid`, `laststatus`, descriptor-backed `cat /etc/banner.txt`,
  classification `qemu-local-shell-userspace-stdout-complete`, and PASS.
- accepted stdin implementation record:
  `tasks/2026-06-03-phase10-userspace-stdin-inherited-fd-core.md`.
- accepted stdin closeout:
  `tasks/2026-06-03-phase10-userspace-stdin-inherited-fd-closeout.md`.
- stdin implementation commit:
  `3c9e1adae27a37b8df753af72edf429743d40847`.
- QEMU/substitute userspace stdin transcript:
  `tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log`.
  Static inspection found visible emitted stdout report
  `Talos userspace stdin fixture read: talos-fd0`, resolved `/bin/stdin`,
  descriptor-backed `source=vfs-open-read`, inherited `fd0`/`fd1`/`fd2`,
  `loader-temp-open=false`, argv0 `/bin/stdin`,
  `exec-stdin fd=0x0000000000000000 bytes=0x000000000000000a
  return=0x000000000000000a stdout-fd=0x0000000000000001
  stdout-bytes=0x000000000000002f stdout-return=0x000000000000002f
  source=userspace-talos-read+userspace-talos-write`, lifecycle/status,
  `waitpid`, `laststatus`, descriptor-backed `cat /etc/banner.txt`,
  classification `qemu-local-shell-userspace-stdin-complete`, and PASS.
- accepted stderr implementation record:
  `tasks/2026-06-03-phase10-userspace-stderr-inherited-fd-core.md`.
- accepted stderr closeout:
  `tasks/2026-06-03-phase10-userspace-stderr-inherited-fd-closeout.md`.
- stderr implementation commit:
  `6377a331e8a4190a82ebf8983f799f10136bc85e`.
- QEMU/substitute userspace stderr transcript:
  `tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log`.
  Static inspection found visible emitted line
  `Talos userspace stderr fixture`, resolved `/bin/stderr`,
  descriptor-backed `source=vfs-open-read`, inherited `fd0`/`fd1`/`fd2`,
  `loader-temp-open=false`, argv0 `/bin/stderr`,
  `exec-stderr fd=0x0000000000000002 bytes=0x000000000000001f
  return=0x000000000000001f source=userspace-talos-write`, lifecycle/status,
  `waitpid`, `laststatus`, `/bin/status42` nonzero regression,
  `/bin/init` and `/bin/zero` zero-status controls, deterministic negative
  exec cases, descriptor-backed `cat /etc/banner.txt`, classification
  `qemu-local-shell-userspace-stderr-complete`, and PASS.

## Next Feature Recommendation

The next feature-led local I/O task should be a minimal
runtime-console0/TTY-backed userspace stdin slice: launch a VFS-backed fixture
whose inherited fd0 read consumes a bounded byte sequence from the same
descriptor-backed serial input path used by the local prompt, then report the
read through the accepted fd1 stdout path. That would convert stdin from
deterministic proof-buffer input into real local-interactivity plumbing while
preserving the accepted VFS exec, descriptor inheritance, lifecycle/status,
`waitpid`, `laststatus`, stdout, stderr, and cat regressions.

That task should stay narrow. It should not add blocking scheduler I/O,
canonical terminal policy expansion, async execution, fork, signals, pipes,
redirection, distinct stderr routing, writable filesystem behavior, libc
stdio, broad descriptor policy, Pi 5 proof, networking, or SSH.

Because no explicit queued follow-up exists after this closeout, supervisor
planning is required before the worker promotes any broader shell I/O task.

## Validation

- static inspection: accepted stdin/stdout/stderr task records and retained
  QEMU/substitute evidence logs were inspected.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

`hardwareTestLock` remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
