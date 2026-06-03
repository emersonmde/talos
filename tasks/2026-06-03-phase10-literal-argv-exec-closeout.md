# Phase 10 Literal Argv Exec Closeout

Task: phase10-literal-argv-exec-closeout-20260603

Status: accepted

## Scope

Close out the accepted literal argv frontier from
`phase10-literal-argv-exec-core-20260603` and recommend the next
mechanically plannable local shell execution primitive.

This checkpoint reconciles the accepted implementation record, retained
QEMU/substitute literal argv evidence, parser limits, startup ABI records,
descriptor inheritance and loader temporary descriptor non-leak evidence,
wait/status regressions, deferred surfaces, and residual risks. It does not
implement code, add shell features, run Pi 5 hardware, publish boot archives,
acquire `hardwareTestLock`, or jump to networking/SSH.

## Findings And Dispositions

- fixed: Reconciled the accepted literal argv evidence as a narrow absolute
  VFS exec startup ABI record with deterministic whitespace token splitting.
- fixed: Confirmed `exec /bin/status42 alpha beta` reports
  `state=literal-argv-absolute-empty-envp`, `argc=0x3`,
  `argv0=/bin/status42`, `argv1=alpha`, `argv2=beta`, deterministic
  empty envp, adjusted startup pointers, and copied startup bytes from the
  initial user stack record.
- fixed: Confirmed the argv-expanded successful exec remains backed by
  descriptor-backed VFS/open/read, the accepted loader, process launch,
  lifecycle/status, inherited `fd0`/`fd1`/`fd2`, and
  `loader-temp-open=false`.
- fixed: Confirmed `waitpid`, non-consuming `laststatus`, zero-status
  `/bin/init` and `/bin/zero` controls, missing and relative exec
  negatives, unsupported glob/escape-style grammar rejection, and
  descriptor-backed `cat /etc/banner.txt` remain covered by retained
  QEMU/substitute evidence.
- not-an-issue: `argv[0]` remains the absolute executable path at this
  frontier; no POSIX shell command-name rewriting or PATH-derived argv policy
  is accepted here.
- deferred: quoting, escaping, globbing, variables, command substitution,
  environment variables, broad envp/auxv/TLS, PATH lookup, userspace stdio I/O
  through inherited descriptors, pipes, redirection, writable filesystem
  behavior, Pi 5 hardware proof, networking, and SSH remain outside this
  frontier.

## Accepted Frontier

The accepted local execution frontier now includes:

- absolute VFS exec for `/bin/status42`, `/bin/init`, and `/bin/zero`
  through descriptor-backed VFS/open/read, the accepted loader, startup ABI,
  launch, lifecycle/status, and shell observation records;
- literal argv propagation for absolute-path exec with `argv[0]` as the
  executable path and following whitespace-separated shell words copied into
  the startup ABI record;
- deterministic empty envp retained for the argv-expanded exec record;
- standard descriptor inheritance records for successful exec paths and
  loader/VFS temporary descriptor non-leak evidence through
  `loader-temp-open=false`;
- nonzero status variation, zero-status controls, `laststatus`, consuming
  `waitpid` observation, deterministic negative exec controls, unsupported
  grammar rejection, and descriptor-backed `cat /etc/banner.txt`
  regressions.

This closeout does not accept quote/escape/glob parsing, variable expansion,
environment-backed execution state, PATH lookup, userspace stdio I/O through
inherited descriptors, pipes, redirection, writable filesystem behavior, Pi 5
hardware behavior, networking, or SSH.

## Evidence Map

- accepted implementation record:
  `tasks/2026-06-03-phase10-literal-argv-exec-core.md`.
- accepted implementation commit:
  `bbe73c3c9943a794c7cf3aad2b6d6cb0b89b7020`.
- QEMU/substitute literal argv transcript:
  `tasks/evidence/2026-06-03-phase10-literal-argv-exec-core/qemu-local-shell-literal-argv-smoke.log`.
  Static inspection found `exec /bin/status42 alpha beta`, literal argv
  startup ABI state, `argc=0x3`, matching argv values, inherited standard
  descriptors, `loader-temp-open=false`, status `0x2a`, consuming
  `waitpid`, matching non-consuming `laststatus`, `/bin/init` and
  `/bin/zero` zero-status controls, unsupported glob rejection, missing and
  relative exec negatives, descriptor-backed `cat /etc/banner.txt`, final
  participants=12 expected=12 errors=0, classification
  `qemu-local-shell-literal-argv-complete`, and PASS.

## Next Feature Recommendation

The next feature-led local execution primitive should be minimal fixed
`/bin` PATH-style lookup for bare executable names. The accepted chain now
has descriptor-backed absolute exec, status/lifecycle observation, standard
descriptor inheritance, and literal argv propagation. Resolving a single bare
command token to `/bin/<name>` is the thinnest real shell execution feature
that exercises VFS lookup and loader/lifecycle reuse without inventing a full
environment-backed PATH model.

That task should stay narrow: bare executable names only, fixed `/bin`
resolution only, no current-directory search, no environment-backed `PATH`,
no hashing/caching, no quoting/globbing/variables, explicit argv policy for
the resolved command, retained absolute exec controls, and retained
`/bin/status42`, `/bin/init`, `/bin/zero`, `waitpid`, `laststatus`,
descriptor inheritance, loader-temp non-leak, negative exec, and
descriptor-backed cat regressions. Pipes, redirection, userspace stdio I/O,
writable filesystem, hardware proof, networking, and SSH should remain
deferred.

## Validation

- static inspection: accepted task record and retained QEMU/substitute
  evidence log were inspected.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

hardwareTestLock remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
