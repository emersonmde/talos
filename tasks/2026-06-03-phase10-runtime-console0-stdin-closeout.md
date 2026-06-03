# Phase 10 Runtime-Console0 Userspace Stdin Closeout

Task: phase10-runtime-console0-stdin-closeout-20260603

Status: accepted

## Scope

Close out the accepted runtime-console0-backed userspace stdin frontier from
`phase10-runtime-console0-stdin-core-20260603` and decide whether the queued
EOF/no-data stdin task remains mechanically unblocked.

This checkpoint reconciles the accepted runtime-console0/local-input stdin
implementation record, retained QEMU/substitute evidence, descriptor-backed
VFS exec lineage, lifecycle/status, `waitpid`, non-consuming `laststatus`,
retained regressions, deferred surfaces, and residual risks. It does not
implement code, add shell features, run Pi 5 hardware, publish boot archives,
acquire `hardwareTestLock`, add pipes or redirection, add blocking I/O, or
expand terminal policy.

## Findings And Dispositions

- fixed: Reconciled `exec stdin` as the first accepted userspace fd0 read
  from runtime-console0/local-input plumbing. The shell resolves the bare name
  through the accepted fixed `/bin` lookup to `/bin/stdin`, reads that
  executable through descriptor-backed VFS/open/read, and launches it through
  the accepted loader, startup ABI, descriptor inheritance, lifecycle/status,
  `waitpid`, and `laststatus` chain.
- fixed: Confirmed the fd0 read source is no longer the deterministic
  proof-buffer stdin path. The retained transcript records
  `read-source=runtime-console0/local-input` after the QEMU/substitute input
  feeds `talos-console0` through the same runtime-console0/local prompt input
  plumbing used by the shell.
- fixed: Confirmed the process-originated read/write record is tied to the
  launched process descriptor table:
  `exec-stdin fd=0x0000000000000000 bytes=0x000000000000000e
  return=0x000000000000000e read-source=runtime-console0/local-input
  stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000033
  stdout-return=0x0000000000000033
  source=userspace-talos-read+userspace-talos-write`.
- fixed: Confirmed the visible report
  `Talos userspace stdin fixture read: talos-console0` is emitted through the
  accepted inherited fd1 userspace TalosWrite path, not through a shell parser
  shortcut or metadata-only launch record.
- fixed: Confirmed inherited descriptor lineage remains explicit:
  `fd0=stdio-input`, `fd1=stdio-output`, `fd2=stdio-output`,
  `inherited-count=0x3`, and `loader-temp-open=false`.
- fixed: Confirmed retained regressions remain covered in the runtime-console0
  smoke: userspace stdout, historical proof-buffer stdin as retained evidence
  only, userspace stderr, fixed `/bin` lookup, literal argv, absolute exec,
  `/bin/status42` nonzero status, `/bin/init` and `/bin/zero` zero-status
  controls, deterministic negative exec cases, `waitpid`, non-consuming
  `laststatus`, loader temporary descriptor non-leak, and descriptor-backed
  `cat /etc/banner.txt`.
- not-an-issue: The older proof-buffer stdin transcript remains useful
  historical/regression evidence for the prior frontier. It is not counted as
  acceptance for runtime-console0-backed stdin.
- not-an-issue: The accepted runtime-console0 path is still bounded and
  non-blocking in the QEMU/substitute harness. This closeout accepts the input
  source and byte/return semantics for the existing local input plumbing, not
  a scheduler-blocking terminal read contract.
- deferred: EOF/no-data/error stdin variants, blocking scheduler I/O,
  readiness polling, terminal canonical policy expansion, async execution,
  fork, signals, pipes, redirection, distinct stderr stream routing, writable
  filesystem behavior, libc stdio, Pi 5 proof, networking, and SSH remain
  outside this frontier.

## Accepted Frontier

The accepted local shell execution and stdin frontier now includes:

- descriptor-backed absolute VFS exec and fixed `/bin` bare-name exec for the
  accepted userspace fixtures;
- literal argv propagation, canonical resolved path argv0 for bare-name exec,
  deterministic empty envp, and inherited standard descriptor records;
- process-originated runtime-console0/local-input bytes consumed by
  `/bin/stdin` through inherited `fd0=stdio-input` using the process
  descriptor TalosRead syscall-substitute path;
- process-originated reporting of those read bytes through inherited
  `fd1=stdio-output` using the accepted TalosWrite syscall-substitute path;
- loader/VFS temporary descriptor non-inheritance, lifecycle/status,
  consuming `waitpid`, non-consuming `laststatus`, deterministic negative
  exec controls, and descriptor-backed `cat /etc/banner.txt` regressions.

This closeout does not accept proof-buffer-only stdin as the current frontier,
EOF/no-data/error stdin semantics, blocking terminal reads, POSIX libc stdio,
terminal session policy, pipes, redirection, distinct stderr routing, writable
filesystem behavior, Pi 5 hardware behavior, networking, or SSH.

## Evidence Map

- accepted implementation record:
  `tasks/2026-06-03-phase10-runtime-console0-stdin-core.md`.
- accepted implementation commit:
  `58c88530d406b9a7ec7397895dc7da9f614a3922`.
- QEMU/substitute runtime-console0 stdin transcript:
  `tasks/evidence/2026-06-03-phase10-runtime-console0-stdin-core/qemu-local-shell-runtime-console0-stdin-smoke.log`.
  Static inspection found `exec stdin`, visible emitted stdout report
  `Talos userspace stdin fixture read: talos-console0`, runtime-console0
  backend markers, resolved `/bin/stdin`, descriptor-backed
  `source=vfs-open-read`, inherited `fd0`/`fd1`/`fd2`,
  `loader-temp-open=false`, deterministic empty envp, argv0 `/bin/stdin`,
  `exec-stdin fd=0x0000000000000000 bytes=0x000000000000000e
  return=0x000000000000000e read-source=runtime-console0/local-input
  stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000033
  stdout-return=0x0000000000000033
  source=userspace-talos-read+userspace-talos-write`, zero lifecycle status,
  matching `waitpid` and `laststatus`, retained stdout fixture, fixed
  `/bin` lookup, nonzero `/bin/status42` control, deterministic negative
  exec cases, descriptor-backed `cat /etc/banner.txt`, final
  participants=18 expected=18 errors=0, classification
  `qemu-local-shell-userspace-stdin-complete`, and PASS.
- retained userspace stdout regression transcript:
  `tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log`.
- retained proof-buffer stdin historical/regression transcript:
  `tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log`.
- retained userspace stderr regression transcript:
  `tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log`.

## Next Feature Recommendation

The queued EOF/no-data stdin variant task remains mechanically unblocked from
the accepted evidence. The runtime-console0/local-input fd0 path now has a
successful bounded-read transcript with explicit byte count and return value;
the next narrow local I/O primitive is to prove the deterministic no-data or
EOF result for the same inherited fd0 path without weakening the successful
read regression.

That task should stay narrow: one VFS-backed fixture, one deterministic
runtime-console0/local-input no-data or EOF condition, a visible fd1 report of
the result, retained successful runtime-console0 stdin evidence, and retained
stdout/stderr, lifecycle/status, `waitpid`, `laststatus`, negative exec, and
descriptor-backed cat regressions. It should not add blocking scheduler I/O,
readiness polling APIs, terminal canonical policy, async execution, fork,
signals, pipes, redirection, writable filesystem behavior, libc stdio, Pi 5
proof, networking, or SSH.

## Validation

- static inspection: accepted implementation task record and retained
  QEMU/substitute runtime-console0 stdin evidence log were inspected.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

`hardwareTestLock` remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
