# Phase 10 Standard Descriptor Inheritance Closeout

Task: phase10-standard-descriptor-inheritance-closeout-20260603

Status: accepted

## Scope

Close out the accepted standard descriptor inheritance frontier from
`phase10-standard-descriptor-inheritance-exec-core-20260603` and recommend
the next mechanically plannable local shell execution primitive.

This checkpoint reconciles the accepted implementation record, retained
QEMU/substitute descriptor inheritance evidence, temporary descriptor non-leak
evidence, wait/status regressions, deferred surfaces, and residual risks. It
does not implement code, add shell features, run Pi 5 hardware, publish boot
archives, acquire `hardwareTestLock`, or jump to networking/SSH.

## Findings And Dispositions

- fixed: Reconciled the accepted descriptor inheritance evidence as a narrow
  exec-time record for standard slots inherited from the shell process
  descriptor table.
- fixed: Confirmed successful `exec /bin/status42`, `exec /bin/init`, and
  `exec /bin/zero` each report inherited `fd0=stdio-input`,
  `fd1=stdio-output`, and `fd2=stdio-output`.
- fixed: Confirmed the same successful exec transcripts report
  `loader-temp-fd=0x3` and `loader-temp-open=false`, keeping the
  loader/VFS executable-read temporary descriptor out of the inherited
  descriptor set.
- fixed: Confirmed nonzero status for `/bin/status42`, zero-status controls
  for `/bin/init` and `/bin/zero`, `laststatus`, `waitpid`, negative exec
  controls, and descriptor-backed `cat /etc/banner.txt` remain covered by
  retained QEMU/substitute evidence.
- not-an-issue: The descriptor record is observational at this frontier; it
  proves inherited process descriptor lineage but does not yet claim
  userspace stdio reads or writes through those descriptors.
- deferred: userspace stdio I/O through inherited descriptors, broad
  descriptor inheritance policy, close-on-exec behavior, dup/close policy
  across exec, fork, asynchronous execution, multiple children, PATH lookup,
  broad argv/envp, pipes, redirection, writable filesystem, Pi 5 hardware
  proof, networking, and SSH remain outside this frontier.

## Accepted Frontier

The accepted local execution frontier now includes:

- absolute VFS exec for `/bin/status42`, `/bin/init`, and `/bin/zero`
  through descriptor-backed VFS/open/read, the accepted loader, startup ABI,
  launch, lifecycle/status, and shell observation records;
- standard descriptor inheritance records for the successful exec paths with
  `fd0`, `fd1`, and `fd2` tied to the shell process descriptor table;
- loader/VFS temporary descriptor non-leak evidence through
  `loader-temp-open=false`;
- nonzero status variation, zero-status controls, `laststatus`, consuming
  `waitpid` observation, deterministic negative exec controls, and
  descriptor-backed `cat /etc/banner.txt` regressions.

This closeout does not accept userspace stdio I/O through inherited
descriptors, broad descriptor inheritance rules, close-on-exec policy,
process replacement, fork, asynchronous execution, multiple children, PATH
lookup, broad argv/envp, pipes, redirection, writable filesystem, Pi 5
hardware behavior, networking, or SSH.

## Evidence Map

- accepted implementation record:
  `tasks/2026-06-03-phase10-standard-descriptor-inheritance-exec-core.md`.
- accepted implementation commit:
  `ab8ac4a59721d5f29b5896ed5621a30ec25ccd6e`.
- QEMU/substitute descriptor inheritance transcript:
  `tasks/evidence/2026-06-03-phase10-standard-descriptor-inheritance-exec-core/qemu-local-shell-standard-descriptor-inheritance-smoke.log`.
  Static inspection found `exec-descriptors` records for `/bin/status42`,
  `/bin/init`, and `/bin/zero`, inherited standard descriptors
  `fd0=stdio-input`, `fd1=stdio-output`, `fd2=stdio-output`,
  `loader-temp-open=false`, matching `laststatus`, negative exec controls,
  descriptor-backed `cat /etc/banner.txt`, final participants=15
  expected=15 errors=0, classification
  `qemu-local-shell-vfs-exec-complete`, and PASS.
- QEMU/substitute waitpid regression transcript:
  `tasks/evidence/2026-06-03-phase10-standard-descriptor-inheritance-exec-core/qemu-local-shell-standard-descriptor-inheritance-waitpid-regression.log`.
  Static inspection found no-child before exec, `/bin/status42` status
  `0x2a`, waitpid consumption, already-consumed no-child, matching
  non-consuming `laststatus`, zero-status `/bin/init` and `/bin/zero`
  wait controls, descriptor inheritance records for each successful exec,
  deterministic negative exec controls, descriptor-backed
  `cat /etc/banner.txt`, final participants=18 expected=18 errors=0,
  classification `qemu-local-shell-waitpid-complete`, and PASS.

## Next Feature Recommendation

The next feature-led local execution primitive should be minimal literal argv
expansion for absolute VFS exec. The accepted chain already carries
`argc=1` and `argv[0]` for absolute paths; the next useful OS surface is to
parse literal shell words after the executable path and pass them through the
startup ABI record for a VFS-backed userspace program.

That task should stay narrow: absolute executable paths only, deterministic
literal token parsing without quoting/globbing, explicit `argc` and
`argv[n]` records, retained descriptor inheritance and loader-temp non-leak
records, and retained `/bin/status42`, `/bin/init`, `/bin/zero`,
`waitpid`, `laststatus`, negative exec, and descriptor-backed cat
regressions. PATH lookup, pipes, redirection, userspace stdio I/O, broad
descriptor policy, environment variables, auxv/TLS, writable filesystem,
hardware proof, networking, and SSH should remain deferred.

## Validation

- static inspection: accepted task record and retained QEMU/substitute
  evidence logs were inspected.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

hardwareTestLock remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
