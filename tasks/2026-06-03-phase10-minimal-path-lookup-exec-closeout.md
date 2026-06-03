# Phase 10 Minimal Path Lookup Exec Closeout

Task: phase10-minimal-path-lookup-exec-closeout-20260603

Status: accepted

## Scope

Close out the accepted minimal fixed `/bin` PATH-style lookup frontier from
`phase10-minimal-path-lookup-exec-core-20260603` and recommend the next
mechanically plannable local shell primitive.

This checkpoint reconciles the accepted bare-name lookup implementation
record, retained QEMU/substitute evidence, argv policy, deterministic negative
controls, absolute exec behavior, descriptor inheritance and loader temporary
descriptor non-leak evidence, wait/status regressions, deferred surfaces, and
residual risks. It does not implement code, add shell features, run Pi 5
hardware, publish boot archives, acquire `hardwareTestLock`, or expand PATH
semantics beyond the accepted fixed `/bin` lookup.

## Findings And Dispositions

- fixed: Reconciled `exec status42 alpha beta` as a narrow bare-name lookup
  feature: the shell resolves only the first bare exec token to
  `/bin/status42`, then executes that VFS object through descriptor-backed
  open/read, the accepted loader, startup ABI, launch, lifecycle/status,
  inherited standard descriptor records, `waitpid`, and `laststatus`.
- fixed: Confirmed the accepted argv policy is canonical resolved path argv0.
  Bare `exec status42 alpha beta` records `argv0=/bin/status42`,
  `argv1=alpha`, and `argv2=beta`, with deterministic empty envp and
  copied startup bytes tied to the initial user stack record.
- fixed: Confirmed absolute exec remains unchanged for the accepted control:
  `exec /bin/status42 gamma` still reaches the same descriptor-backed
  VFS/open/read, loader, startup ABI, launch, lifecycle/status, inherited
  descriptor, and loader temporary descriptor non-leak chain.
- fixed: Confirmed status and process-observation regressions remain covered:
  `/bin/status42` reports status `0x2a`, `exec init` and `exec zero`
  report zero status through fixed `/bin` lookup, `waitpid` consumes the
  latest lifecycle record, and non-consuming `laststatus` reports the same
  latest lifecycle identity/status.
- fixed: Confirmed deterministic negative controls remain bounded:
  missing bare names fail as `exec-not-found`, path-like relative names fail
  as `exec-invalid-path`, directories and non-executable/non-ELF files fail
  as `exec-not-executable`, and unsupported glob grammar remains rejected.
- fixed: Confirmed descriptor-backed `cat /etc/banner.txt` remains covered
  as the file-operation regression surface.
- not-an-issue: The accepted lookup is intentionally not an environment-backed
  PATH implementation. It is a fixed `/bin/<name>` resolution rule for a bare
  first exec token only.
- deferred: Environment-backed PATH, current-directory search, command
  hashing/caching, shell builtin conversion, quoting, escaping, globbing,
  variables, command substitution, userspace stdio I/O through inherited
  descriptors, pipes, redirection, writable filesystem behavior, Pi 5 hardware
  proof, networking, and SSH remain outside this frontier.

## Accepted Frontier

The accepted local shell execution frontier now includes:

- absolute VFS exec for `/bin/status42`, `/bin/init`, and `/bin/zero`;
- fixed `/bin` bare-name lookup for the first shell-visible exec token, with
  bare names resolving only to `/bin/<name>`;
- canonical resolved path argv0 for bare-name exec, with later literal argv
  words copied unchanged into the startup ABI record;
- deterministic empty envp for the accepted exec startup ABI records;
- inherited standard descriptor records for successful exec paths and
  loader/VFS temporary descriptor non-leak evidence through
  `loader-temp-open=false`;
- nonzero status variation, zero-status controls, consuming `waitpid`,
  non-consuming `laststatus`, deterministic negative exec controls,
  unsupported grammar rejection, and descriptor-backed `cat /etc/banner.txt`
  regressions.

This closeout does not accept a general POSIX shell PATH model, command search
through the current directory, quoting/escaping/globbing/variables,
environment propagation, userspace stdio I/O through inherited descriptors,
pipes, redirection, writable filesystem behavior, Pi 5 hardware behavior,
networking, or SSH.

## Evidence Map

- accepted implementation record:
  `tasks/2026-06-03-phase10-minimal-path-lookup-exec-core.md`.
- accepted implementation commit:
  `c734f19446a9e559c1ef57ce4458a0c230fe9858`.
- QEMU/substitute minimal fixed `/bin` lookup transcript:
  `tasks/evidence/2026-06-03-phase10-minimal-path-lookup-exec-core/qemu-local-shell-path-lookup-smoke.log`.
  Static inspection found bare `exec status42 alpha beta` resolving to
  `/bin/status42`, descriptor-backed VFS/open/read, loader/startup/launch/
  lifecycle/status records, inherited `fd0`/`fd1`/`fd2`,
  `loader-temp-open=false`, `state=literal-argv-absolute-empty-envp`,
  `argc=0x3`, canonical `argv0=/bin/status42`, `argv1=alpha`,
  `argv2=beta`, status `0x2a`, matching `waitpid` and `laststatus`,
  bare `exec init` and `exec zero` zero-status controls, absolute
  `exec /bin/status42 gamma` regression, deterministic missing/path-like/
  directory/non-executable/glob negatives, descriptor-backed
  `cat /etc/banner.txt`, final participants=17 expected=17 errors=0,
  classification `qemu-local-shell-path-lookup-complete`, and PASS.

## Next Feature Recommendation

The next feature-led local shell primitive should be minimal userspace stdout
I/O through the already inherited standard descriptor path. The accepted chain
now proves descriptor-backed VFS lookup/read, executable loading, startup
argv/envp, lifecycle/status, `waitpid`, `laststatus`, and inherited
`fd1=stdio-output`; the thinnest next real feature is for a VFS-backed
userspace fixture to emit bytes through inherited stdout rather than only
reporting kernel-observed launch/status records.

That future task should stay narrow: one read-only executable fixture, one
bounded userspace write/status surface, inherited stdout only, retained
fixed-`/bin` lookup and absolute exec controls, retained `/bin/status42`,
`/bin/init`, `/bin/zero`, `waitpid`, `laststatus`, descriptor inheritance,
loader-temp non-leak, negative exec, and descriptor-backed cat regressions. It
should not add pipes, redirection, writable filesystem behavior, a full libc
stdio stack, environment-backed PATH, broader shell grammar, hardware proof,
networking, or SSH.

## Validation

- static inspection: accepted task record and retained QEMU/substitute
  evidence log were inspected.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

`hardwareTestLock` remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
