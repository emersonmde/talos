# Phase 10 Userspace Stdout Through Inherited FD Closeout

Task: phase10-userspace-stdout-inherited-fd-closeout-20260603

Status: accepted

## Scope

Close out the accepted userspace stdout-through-inherited-fd frontier from
`phase10-userspace-stdout-inherited-fd-core-20260603` and recommend the next
mechanically plannable local I/O primitive.

This checkpoint reconciles the accepted stdout implementation record, retained
QEMU/substitute evidence, descriptor/write lineage, VFS exec lineage,
lifecycle/status, `waitpid`, non-consuming `laststatus`, retained regressions,
deferred surfaces, and residual risks. It does not implement code, add shell
features, run Pi 5 hardware, publish boot archives, acquire `hardwareTestLock`,
or broaden stdio beyond the accepted stdout fixture.

## Findings And Dispositions

- fixed: Reconciled `exec stdout` as the first accepted process-originated
  stdout path. The shell resolves the bare name through the accepted fixed
  `/bin` lookup to `/bin/stdout`, reads that executable through
  descriptor-backed VFS/open/read, and launches it through the accepted loader,
  startup ABI, descriptor inheritance, lifecycle/status, `waitpid`, and
  `laststatus` chain.
- fixed: Confirmed the emitted line `Talos userspace stdout fixture` is tied to
  the launched process descriptor table through
  `exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f
  return=0x000000000000001f source=userspace-talos-write`, not to a kernel shell
  built-in or metadata-only launch report.
- fixed: Confirmed inherited descriptor lineage remains explicit:
  `fd0=stdio-input`, `fd1=stdio-output`, `fd2=stdio-output`,
  `inherited-count=0x3`, and `loader-temp-open=false` for the stdout exec.
- fixed: Confirmed process-observation behavior remains covered. The same
  `/bin/stdout` lifecycle record reports zero status, consuming `waitpid`
  observes the record, and non-consuming `laststatus` reports the same latest
  lifecycle identity/status after `waitpid`.
- fixed: Confirmed retained regressions remain covered: fixed `/bin` lookup,
  absolute exec, `/bin/status42` nonzero status, `/bin/init` and `/bin/zero`
  zero-status controls, deterministic negative exec cases, loader temporary
  descriptor non-leak, inherited standard descriptors, and descriptor-backed
  `cat /etc/banner.txt`.
- not-an-issue: The accepted stdout fixture proves one bounded TalosWrite-style
  userspace output path. It is intentionally not a libc stdio implementation,
  pipe/redirection implementation, stderr policy, writable filesystem behavior,
  or hardware claim.
- deferred: Stdin reads through inherited `fd0`, stderr-specific output,
  blocking I/O, pipes, redirection, fork/async execution, libc stdio, writable
  filesystem behavior, broader shell grammar, hardware proof, networking, and
  SSH remain outside this frontier.

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
- nonzero and zero status controls, consuming `waitpid`, non-consuming
  `laststatus`, deterministic negative exec controls, unsupported grammar
  rejection, and descriptor-backed `cat /etc/banner.txt` regressions.

This closeout does not accept stdin reads through inherited descriptors,
stderr-specific output policy, terminal blocking behavior, pipes, redirection,
general POSIX shell grammar, libc stdio, writable filesystem behavior, Pi 5
hardware behavior, networking, or SSH.

## Evidence Map

- accepted implementation record:
  `tasks/2026-06-03-phase10-userspace-stdout-inherited-fd-core.md`.
- accepted implementation commit:
  `7a49e4ae61aac8611a95a26018d7d52c0cb3183f`.
- QEMU/substitute userspace stdout transcript:
  `tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log`.
  Static inspection found `exec stdout`, visible emitted stdout
  `Talos userspace stdout fixture`, resolved `/bin/stdout`,
  descriptor-backed `source=vfs-open-read`, inherited `fd0`/`fd1`/`fd2`,
  `loader-temp-open=false`, deterministic empty envp, argv0 `/bin/stdout`,
  `exec-stdout fd=0x0000000000000001 bytes=0x000000000000001f
  return=0x000000000000001f source=userspace-talos-write`, zero lifecycle
  status, matching `waitpid` and `laststatus`, `/bin/status42` nonzero
  control, `/bin/init` and `/bin/zero` zero-status controls, absolute exec
  regression, deterministic missing/path-like/directory/non-executable/glob
  negatives, descriptor-backed `cat /etc/banner.txt`, final participants=18
  expected=18 errors=0, classification
  `qemu-local-shell-userspace-stdout-complete`, and PASS.
- QEMU/substitute fixed `/bin` lookup regression transcript:
  `tasks/evidence/2026-06-03-phase10-minimal-path-lookup-exec-core/qemu-local-shell-path-lookup-smoke.log`.
  The stdout core reran this regression after adding `/bin/stdout`.

## Next Feature Recommendation

The next feature-led local I/O primitive should be minimal userspace stdin
through inherited `fd0=stdio-input`, reported through the now-accepted
userspace stdout path. The accepted chain already proves VFS-backed executable
lookup/read, loader/startup/launch, descriptor inheritance, process-originated
stdout, lifecycle/status, `waitpid`, and `laststatus`; the thinnest next real
feature is for a VFS-backed userspace fixture to perform one bounded read on
inherited `fd0` and report the bytes, EOF/no-data result, or deterministic
error through inherited `fd1`.

That future task should stay narrow: one read-only executable fixture, one
bounded userspace read observation, reporting through accepted stdout, retained
VFS exec and descriptor inheritance controls, retained `/bin/stdout`,
`/bin/status42`, `/bin/init`, `/bin/zero`, `waitpid`, `laststatus`, loader
temporary descriptor non-leak, negative exec, and descriptor-backed cat
regressions. It should not add terminal canonical-mode expansion, blocking
scheduler I/O, async execution, signals, pipes, redirection, writable
filesystem behavior, libc stdio, broad descriptor policy, hardware proof,
networking, or SSH.

## Validation

- static inspection: accepted task record and retained QEMU/substitute evidence
  logs were inspected.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

`hardwareTestLock` remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
