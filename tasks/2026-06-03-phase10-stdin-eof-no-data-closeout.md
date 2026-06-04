# Phase 10 Stdin EOF/No-Data Closeout

Task: phase10-stdin-eof-no-data-closeout-20260603

Status: accepted

## Scope

Close out the accepted userspace stdin frontier after the successful
runtime-console0/local-input read and deterministic EOF/no-data inherited fd0
branch.

This checkpoint reconciles the accepted implementation records, retained
QEMU/substitute evidence, descriptor-backed VFS exec lineage, lifecycle/status,
waitpid, non-consuming laststatus, stdout/stderr regressions, deferred
surfaces, and residual risks. It does not implement code, run Pi 5 hardware,
publish boot archives, acquire hardwareTestLock, add blocking I/O, add pipes
or redirection, split output streams, or plan the rest of Phase 10.

## Findings And Dispositions

- fixed: Reconciled the accepted successful stdin path. Shell-visible
  exec stdin resolves through the fixed /bin lookup to /bin/stdin, reads
  the executable through descriptor-backed VFS/open/read, launches through the
  accepted loader/startup/descriptor inheritance/lifecycle chain, consumes
  talos-console0 from runtime-console0/local-input through inherited
  fd0=stdio-input, and reports the bytes through inherited fd1.
- fixed: Reconciled the accepted EOF/no-data branch for the same inherited fd0
  path. With no payload bytes immediately available after the command
  terminator, the fixture treats TalosRead return 0 as deterministic
  EOF/no-data and reports Talos userspace stdin fixture no-data: eof.
- fixed: Confirmed the EOF/no-data transcript is distinguishable from the
  successful byte-read transcript by the visible report, zero byte count,
  zero return value, and read-result=eof/no-data marker.
- fixed: Confirmed both stdin branches are still tied to launched process
  descriptors and userspace read/write paths, not shell parser shortcuts or
  proof-buffer-only metadata.
- fixed: Confirmed retained controls remain represented by accepted evidence:
  userspace stdout and stderr, descriptor-backed VFS exec, fixed /bin lookup,
  literal argv, zero/nonzero status, lifecycle/status records, consuming
  waitpid, non-consuming laststatus, negative exec controls, and
  descriptor-backed cat /etc/banner.txt.
- not-an-issue: The older deterministic proof-buffer stdin record remains
  useful historical/regression evidence, but it is not counted as the current
  accepted runtime-console0/local-input stdin frontier.
- not-an-issue: The EOF/no-data result is a bounded non-blocking local-input
  behavior in the QEMU/substitute harness. This closeout does not accept a
  scheduler-blocking terminal read, readiness API, or broader TTY contract.
- deferred: Blocking scheduler I/O, readiness/polling APIs, canonical terminal
  policy expansion, async execution, fork, signals, pipes, redirection,
  distinct stderr stream routing, writable filesystem behavior, libc stdio,
  Pi 5 proof, networking, and SSH remain outside this frontier.

## Accepted Frontier

The accepted local shell execution and stdin frontier now includes:

- descriptor-backed absolute VFS exec and fixed /bin bare-name exec for the
  accepted userspace fixtures;
- literal argv propagation, canonical resolved path argv0 for bare-name exec,
  deterministic empty envp, inherited standard descriptor records, and loader
  temporary descriptor non-inheritance;
- process-originated successful runtime-console0/local-input bytes consumed by
  /bin/stdin through inherited fd0=stdio-input using the process descriptor
  TalosRead syscall-substitute path;
- process-originated EOF/no-data observation for the same inherited fd0 path
  when runtime-console0/local-input has no bytes immediately available;
- process-originated reporting of both stdin outcomes through inherited
  fd1=stdio-output using the accepted TalosWrite syscall-substitute path;
- lifecycle/status, consuming waitpid, non-consuming laststatus,
  zero/nonzero status controls, deterministic negative exec controls, and
  descriptor-backed cat /etc/banner.txt regressions.

This closeout does not accept proof-buffer-only stdin as the current frontier,
blocking terminal reads, POSIX libc stdio, terminal session policy, pipes,
redirection, distinct stderr stream routing, writable filesystem behavior,
Pi 5 hardware behavior, networking, or SSH.

## Evidence Map

- accepted successful runtime-console0 stdin implementation record:
  tasks/2026-06-03-phase10-runtime-console0-stdin-core.md.
- accepted successful runtime-console0 stdin commit:
  58c88530d406b9a7ec7397895dc7da9f614a3922.
- accepted EOF/no-data implementation record:
  tasks/2026-06-03-phase10-stdin-eof-no-data-core.md.
- accepted EOF/no-data implementation commit:
  67f78c3.
- QEMU/substitute successful runtime-console0 stdin transcript:
  tasks/evidence/2026-06-03-phase10-runtime-console0-stdin-core/qemu-local-shell-runtime-console0-stdin-smoke.log.
  Static inspection found exec stdin, visible report
  Talos userspace stdin fixture read: talos-console0, resolved /bin/stdin,
  descriptor-backed source=vfs-open-read, inherited fd0/fd1/fd2,
  loader-temp-open=false, argv0 /bin/stdin, deterministic empty envp,
  exec-stdin fd=0x0000000000000000 bytes=0x000000000000000e
  return=0x000000000000000e read-source=runtime-console0/local-input
  stdout-fd=0x0000000000000001 stdout-bytes=0x0000000000000033
  stdout-return=0x0000000000000033
  source=userspace-talos-read+userspace-talos-write, zero lifecycle status,
  matching waitpid and laststatus, retained controls, classification
  qemu-local-shell-userspace-stdin-complete, and PASS.
- QEMU/substitute EOF/no-data stdin transcript:
  tasks/evidence/2026-06-03-phase10-stdin-eof-no-data-core/qemu-local-shell-stdin-eof-no-data-smoke.log.
  Static inspection found exec stdin, visible report
  Talos userspace stdin fixture no-data: eof, resolved /bin/stdin,
  descriptor-backed source=vfs-open-read, inherited fd0/fd1/fd2,
  loader-temp-open=false, argv0 /bin/stdin, deterministic empty envp,
  exec-stdin fd=0x0000000000000000 bytes=0x0000000000000000
  return=0x0000000000000000 read-source=runtime-console0/local-input
  stdout-fd=0x0000000000000001 stdout-bytes=0x000000000000002b
  stdout-return=0x000000000000002b
  source=userspace-talos-read+userspace-talos-write read-result=eof/no-data,
  zero lifecycle status for /bin/stdin, matching waitpid and laststatus,
  retained stdout, zero/nonzero status controls, fixed /bin lookup, negative
  exec controls, descriptor-backed cat /etc/banner.txt, classification
  qemu-local-shell-userspace-stdin-complete, and PASS.
- retained userspace stdout regression transcript:
  tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log.
- retained userspace stderr regression transcript:
  tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log.
- retained wait/status and negative-control transcript:
  tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log.
- retained fixed /bin lookup, literal argv, and descriptor-backed cat
  transcript:
  tasks/evidence/2026-06-03-phase10-minimal-path-lookup-exec-core/qemu-local-shell-path-lookup-smoke.log.

## Next Feature Recommendation

No broader shell I/O task is mechanically unblocked by this closeout alone. The
next feature-led local I/O primitive should be supervisor-planned from the
accepted stdin frontier and should stay below the next real capability boundary:
blocking/readiness behavior, pipe or redirection plumbing, or distinct stream
routing all need explicit task decomposition before promotion.

The recommended planning focus is a narrow readiness or blocking-behavior slice
for runtime-console0/local-input if the supervisor chooses to continue stdin
semantics next. It should retain the successful and EOF/no-data stdin logs as
regressions and avoid treating kernel-backed command expansion or proof-buffer
input as feature progress.

## Validation

- static inspection: accepted stdin implementation task records and retained
  QEMU/substitute evidence logs were inspected.
- static inspection: git diff --check passed.
- documentation: /home/node/.cargo/bin/mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

hardwareTestLock remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
