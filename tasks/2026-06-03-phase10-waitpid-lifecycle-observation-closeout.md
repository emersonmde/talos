# Phase 10 Waitpid Lifecycle Observation Closeout

Task: phase10-waitpid-lifecycle-observation-closeout-20260603

Status: accepted

## Scope

Close out the minimal wait/waitpid-style lifecycle observation frontier from
`phase10-minimal-waitpid-lifecycle-observation-core-20260603` and decide the
next mechanically plannable local execution primitive.

This closeout does not implement code, add shell features, run Pi 5 hardware,
publish boot archives, acquire `hardwareTestLock`, or jump to networking/SSH.

## Findings And Dispositions

- fixed: Reconciled the accepted waitpid evidence as a narrow, consuming
  observation of one completed child lifecycle/status record produced by the
  VFS-backed exec path.
- fixed: Confirmed the accepted frontier includes deterministic no-child
  behavior before exec and after the single waitable record has been consumed.
- fixed: Confirmed `/bin/status42` reports status `0x2a` through the accepted
  descriptor-backed VFS/open/read, loader, startup ABI, launch,
  lifecycle/status, and waitpid observation chain.
- fixed: Confirmed `/bin/init` and `/bin/zero` remain zero-status controls
  through the same accepted absolute VFS exec path and waitpid observation
  surface.
- fixed: Confirmed `laststatus` remains a non-consuming regression/control
  surface over the latest lifecycle/status record after waitpid consumption.
- fixed: Confirmed deterministic negative exec controls and descriptor-backed
  `cat /etc/banner.txt` remain covered by retained QEMU/substitute evidence.
- not-an-issue: The exec transcript still prints `reaped=true` on the
  lifecycle line before `waitpid`; in this frontier that field is a record
  completion marker in the existing lifecycle/status transcript, while the
  accepted consuming wait observation is proven by the following `waitpid`
  output and second no-child result.
- deferred: asynchronous execution, multiple children, broad zombie-table
  policy, process replacement, fork, signals, descriptor inheritance expansion,
  PATH lookup, broad argv/envp, pipes, redirection, writable filesystem, Pi 5
  hardware proof, networking, and SSH remain outside this frontier.

## Accepted Frontier

The accepted process-lifecycle frontier is now:

- the shell exposes `waitpid` as a minimal wait/waitpid-shaped observation
  command;
- `waitpid` before any successful exec reports no-child from the lifecycle
  record source;
- successful absolute VFS exec records one waitable completed child
  lifecycle/status record;
- `waitpid` after `exec /bin/status42` reports the child identity, shell
  parent ownership, executable path, exited state, status `0x2a`,
  observed-status `0x2a`, and lifecycle-record source;
- a second `waitpid` after the same exec reports deterministic no-child
  behavior instead of fabricating another lifecycle record;
- `laststatus` remains non-consuming and still agrees with the latest
  lifecycle/status semantics;
- `/bin/init` and `/bin/zero` remain zero-status wait controls, negative exec
  controls remain deterministic, and `cat /etc/banner.txt` remains
  descriptor-backed.

This closeout does not accept asynchronous execution, multiple children, broad
zombie management, process replacement, fork, signals, descriptor inheritance
expansion, PATH lookup, pipes, redirection, writable filesystem, Pi 5 hardware
behavior, networking, or SSH.

## Next Feature Recommendation

The next feature-led local execution primitive should be standard descriptor
inheritance across VFS-backed exec. That task should prove that the launched
userspace process receives shell-owned standard descriptors `0`, `1`, and `2`
while loader/VFS temporary descriptors do not leak into the launched process
descriptor set.

Descriptor inheritance should happen before PATH lookup, literal argv
expansion, pipes, redirection, networking, or SSH because those later features
need a clear process-local standard descriptor baseline.

## Evidence Map

- accepted implementation record:
  `tasks/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core.md`.
- QEMU/substitute wait observation evidence:
  `tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log`.
- the retained QEMU/substitute transcript includes no-child before exec,
  `/bin/status42` status `0x2a`, waitpid consumption, second no-child,
  matching non-consuming `laststatus`, `/bin/init` and `/bin/zero`
  zero-status wait controls, deterministic negative exec controls,
  descriptor-backed `cat /etc/banner.txt`, final participants=18 expected=18
  errors=0, classification=`qemu-local-shell-waitpid-complete`, and PASS.

## Validation

- static inspection: accepted task record and retained QEMU/substitute evidence
  inspected.
- static inspection: `git diff --check` passed.
- documentation: `/home/node/.cargo/bin/mdbook build` passed.
- staged whitespace inspection: `git diff --cached --check` passed before
  commit.

No Pi 5 hardware, power-cycle, boot archive publication, or
`hardwareTestLock` acquisition was performed.

Acceptance commit: recorded in durable supervisor state after commit creation.
