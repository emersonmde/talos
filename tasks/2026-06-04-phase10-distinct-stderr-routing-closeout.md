# Phase 10 Distinct Stderr Routing Closeout

Task: phase10-distinct-stderr-routing-closeout-20260604

Status: accepted

## Summary

The accepted distinct-stderr-routing core proves fd2 is no longer just an
unlabeled alias of fd1 in shell-visible VFS-backed userspace execution
evidence. Both streams still share the runtime-console0 physical output sink,
but inherited descriptor metadata now distinguishes:

- fd1: `stream=stdout route=runtime-console0/stdout`
- fd2: `stream=stderr route=runtime-console0/stderr`

This closeout accepts descriptor/stream-origin distinction only. It does not
accept pipes, redirection, file-backed stderr, separate physical sinks, terminal
colors or policy, writable filesystem behavior, networking, or SSH.

## Findings And Disposition

- fixed: Closed out the fd2 routing boundary with an evidence map that ties
  `/bin/stderr` output to inherited fd2 and
  `stream=stderr route=runtime-console0/stderr`.
- fixed: Preserved the stdout control showing inherited fd1 as
  `stream=stdout route=runtime-console0/stdout`, not mislabeled stderr.
- fixed: Preserved stdin scheduler wait/readiness and Ctrl-D EOF controls so
  stderr routing did not collapse the accepted local-input distinctions.
- fixed: Preserved VFS exec, lifecycle/status, `waitpid`, `laststatus`,
  negative exec controls, and descriptor-backed `cat /etc/banner.txt`.
- not-an-issue: fd1 and fd2 still write to the same runtime-console0 physical
  sink. The accepted split is descriptor metadata and stream-origin routing,
  not separate hardware or file sinks.
- deferred: pipes, redirection, file-backed stderr, separate physical sinks,
  writable filesystem behavior, terminal colors/policy, full termios, libc
  stdio, async jobs, fork, signals, Pi 5 proof, networking, and SSH.

## Evidence Map

- distinct stderr routing:
  `tasks/evidence/2026-06-04-phase10-distinct-stderr-routing-core/qemu-local-shell-distinct-stderr-routing-smoke.log`.
  QEMU/substitute evidence shows `exec stderr`, descriptor-backed
  `/bin/stderr` VFS/open/read lineage, inherited fd2 write metadata
  `stream=stderr route=runtime-console0/stderr`, lifecycle/status,
  `waitpid`, `laststatus`, nonzero status control, zero-status controls,
  negative exec controls, descriptor-backed `cat /etc/banner.txt`, final
  classification `qemu-local-shell-distinct-stderr-routing-complete`, and
  PASS.
- stdout control:
  `tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log`.
  QEMU/substitute evidence shows `exec stdout` writing through inherited fd1
  with `stream=stdout route=runtime-console0/stdout`, final classification
  `qemu-local-shell-userspace-stdout-complete`, and PASS.
- scheduler-backed stdin wait control:
  `tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log`.
  QEMU/substitute evidence retains scheduler sleep and wake/resume markers,
  delayed `talos-console0` bytes, `read-result=scheduler-wait/delayed-input`,
  final classification `qemu-local-shell-userspace-stdin-complete`, and PASS.
- no-data/readiness control:
  `tasks/evidence/2026-06-04-phase10-runtime-stdin-readiness-distinction-core/qemu-local-shell-runtime-stdin-readiness-smoke.log`.
  QEMU/substitute evidence retains `-EAGAIN`,
  `read-result=readiness/no-data`, `timeout/no-false-eof`, final
  classification `qemu-local-shell-userspace-stdin-complete`, and PASS.
- terminal EOF control:
  `tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log`.
  QEMU/substitute evidence retains Ctrl-D true EOF with `return=0`,
  `read-result=terminal-eof`, final classification
  `qemu-local-shell-userspace-stdin-complete`, and PASS.
- VFS/process controls:
  The distinct stderr routing smoke retains descriptor-backed VFS exec lineage,
  zero and nonzero lifecycle/status observations, consuming `waitpid`,
  non-consuming `laststatus`, negative exec controls, loader temporary
  descriptor non-leak, and descriptor-backed `cat /etc/banner.txt`.

## Accepted Frontier

Accepted:

- fd2 stderr writes from VFS-backed userspace execution carry explicit
  `stream=stderr route=runtime-console0/stderr` metadata.
- fd1 stdout writes carry explicit
  `stream=stdout route=runtime-console0/stdout` metadata.
- The physical output sink remains shared runtime-console0.
- Descriptor-backed VFS/open/read exec lineage, standard descriptor
  inheritance, loader temporary descriptor non-leak, lifecycle/status,
  `waitpid`, `laststatus`, negative exec controls, descriptor-backed cat,
  scheduler-backed stdin wait/readiness, and Ctrl-D EOF remain retained.

Deferred:

- pipes and pipe-backed descriptor wiring;
- shell redirection and descriptor duplication/close policy;
- file-backed stderr/stdout and writable filesystem behavior;
- separate physical stdout/stderr sinks;
- terminal colors/policy, full termios, libc stdio, async jobs, fork, signals,
  Pi 5 proof, networking, and SSH.

## Next Step Recommendation

Supervisor planning is required before the next feature. Pipes and redirection
both need explicit scope, dependencies, acceptance gates, and deferred-boundary
language because redirection may imply descriptor duplication and writable file
targets, while pipes imply producer/consumer process lifecycle behavior.

If the supervisor chooses between them, the narrower next feature is likely a
descriptor-routing redirection slice that does not claim writable filesystem
support unless a writable target is explicitly planned and evidenced.

## Validation Summary

- static inspection: accepted core task record and retained evidence logs were
  inspected.
- diff check: `git diff --check` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed with the existing large
  search-index warning.
- staged diff check: `git diff --cached --check` passed before commit.

hardwareTestLock remained unlocked/restored and unused.
