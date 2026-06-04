# Phase 10 /dev/null Redirection Frontier Closeout

Task: phase10-dev-null-redirection-frontier-closeout-20260604

Status: accepted

## Summary

This closeout checkpoints the accepted explicit '/dev/null' file/device
redirection frontier after stdout and stderr sink redirection are both covered.

Accepted behavior is limited to these exact child-only output sink forms:

- 'exec stdout >/dev/null': the VFS-backed '/bin/stdout' child has fd1
  rebound to the explicit '/dev/null' sink device.
- 'exec stderr 2>/dev/null': the VFS-backed '/bin/stderr' child has fd2
  rebound to the same explicit '/dev/null' sink device.

Both forms record 'op=sink', 'target-path=/dev/null',
'target-stream=null-sink', and 'target-route=device:/dev/null'; route the
userspace write to 'stream=null-sink route=device:/dev/null'; discard the
accepted 31-byte fixture payload after validating/copying the userspace
buffer; and return the accepted byte count. The target descriptor mutation is
child-only, and the shell restores the relevant standard descriptor after the
child exits.

This frontier accepts '/dev/null' only as an explicit sink device. It does not
accept regular-file redirection, append/truncate, input redirection, writable
filesystem behavior, arbitrary descriptor syntax, broader file/device
semantics, multi-stage/concurrent pipelines, Pi 5 proof, networking, SSH, or a
phase transition.

## Findings And Disposition

- fixed: Reconciled stdout-to-/dev/null and stderr-to-/dev/null as sibling
  output sink forms using the same explicit '/dev/null' device contract.
- fixed: Confirmed both forms are child-only descriptor-table mutations with
  shell descriptor restoration proven by following normal stdio controls.
- fixed: Confirmed both accepted sink writes report the null-sink route and
  accepted byte-count discard accounting rather than writable filesystem
  behavior.
- fixed: Preserved descriptor redirection controls, descriptor-mixing pipeline
  controls, normal stdio/stderr routing, stdin wait/readiness/EOF, VFS exec,
  lifecycle/status, waitpid, laststatus, negative controls, and
  descriptor-backed cat as retained regression coverage.
- fixed: Updated the roadmap to prevent acceptance drift from the two exact
  '/dev/null' output sink forms into regular-file, input, append/truncate, or
  broader file/device redirection.
- not-an-issue: The stdout and stderr smoke logs include later visible
  fixture payloads because those later normal 'exec stdout' and 'exec stderr'
  controls intentionally prove shell descriptor restoration after the
  redirected child exits.
- deferred: '1>/dev/null', 'exec stdout 1>file',
  'exec stderr 2>file', 'exec stderr 2>>/dev/null',
  'exec stderr </dev/null', regular-file redirection, append/truncate, input
  redirection, arbitrary descriptor syntax, writable filesystem behavior,
  broader file/device semantics, multi-stage/concurrent pipelines, Pi 5 proof,
  networking, SSH, and any phase transition.

## Evidence Map

- stderr-to-/dev/null evidence:
  'tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-dev-null-stderr-redirection-smoke.log'.
  Static inspection shows command 3 'exec stderr 2>/dev/null', child
  'fd2=device', 'exec-redirection op=sink source-fd=2
  target-path=/dev/null target-stream=null-sink
  target-route=device:/dev/null child-only=true shell-restored=true',
  'exec-stderr ... stream=null-sink route=device:/dev/null', 'bytes=0x1f'
  and 'return=0x1f', later normal 'exec stderr' output routed to
  'runtime-console0/stderr', 'waitpid', 'laststatus', deterministic negative
  redirection forms, 'cat /etc/banner.txt', final classification
  'qemu-local-shell-dev-null-stderr-redirection-complete', errors=0, and PASS.
- stdout-to-/dev/null evidence:
  'tasks/evidence/2026-06-04-phase10-dev-null-stdout-redirection-contract-core/qemu-local-shell-dev-null-stdout-redirection-smoke.log'.
  Static inspection shows command 3 'exec stdout >/dev/null', child
  'fd1=device', 'exec-redirection op=sink source-fd=1
  target-path=/dev/null target-stream=null-sink
  target-route=device:/dev/null child-only=true shell-restored=true',
  'exec-stdout ... stream=null-sink route=device:/dev/null', 'bytes=0x1f'
  and 'return=0x1f', later normal 'exec stdout' output routed to
  'runtime-console0/stdout', 'waitpid', 'laststatus', deterministic negative
  redirection forms, 'cat /etc/banner.txt', final classification
  'qemu-local-shell-dev-null-stdout-redirection-complete', errors=0, and PASS.
- retained stdout sibling control from the stderr iteration:
  'tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-dev-null-stdout-redirection-control.log'.
- retained normal stdio and stderr routing controls:
  'tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-userspace-stderr-control.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-distinct-stderr-routing-core/qemu-local-shell-distinct-stderr-routing-smoke.log'.
- retained descriptor redirection controls:
  'tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-control.log',
  'tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-stderr-close-redirection-core/qemu-local-shell-stderr-close-redirection-smoke.log'.
- retained descriptor-mixing pipeline controls:
  'tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-control.log',
  'tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log',
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log',
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log'.
- retained stdin readiness and EOF controls:
  'tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-runtime-stdin-readiness-control.log',
  'tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-stdin-eof-no-data-control.log',
  'tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log'.
- retained VFS exec, lifecycle/status, waitpid, laststatus, negative controls,
  and descriptor-backed file I/O:
  'tasks/evidence/2026-06-03-phase10-absolute-vfs-exec-dispatch-core/qemu-local-shell-absolute-vfs-exec-dispatch-smoke.log',
  'tasks/evidence/2026-06-03-phase10-vfs-exec-nonzero-status-core/qemu-local-shell-nonzero-vfs-exec-status-smoke.log',
  'tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log',
  'tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log',
  'tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-cat-banner-control.log', and
  'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log'.

## Accepted Frontier

Accepted:

- exactly 'exec stdout >/dev/null';
- exactly 'exec stderr 2>/dev/null';
- '/dev/null' as an explicit output sink device only;
- child-only rebinding of the target child descriptor to the null sink;
- shell restoration of fd1/fd2 after the redirected child exits;
- userspace writes to the sink return the accepted byte count while
  discarding the fixture payload;
- VFS-backed launch, lifecycle/status, 'waitpid', 'laststatus', normal stdio
  restoration, deterministic negatives, and descriptor-backed cat are covered
  by retained QEMU/substitute evidence.

Deferred:

- shorthand or arbitrary descriptor syntax such as '1>/dev/null' and
  arbitrary 'N>target' or 'N>&M';
- regular-file redirection, append/truncate, input redirection, writable
  filesystem behavior, and broader file/device semantics;
- multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, and job control;
- Pi 5 proof, networking, SSH, and any phase transition.

## Next Step Requirement

Supervisor planning is required before the next feature-led shell I/O task.
The queue has no further explicit task after this closeout, and the worker
must not infer a broader feature or phase transition.

The bounded recommendation is to plan the next file/device redirection slice
only after choosing an explicit contract, with '/dev/null' input redirection
or a read-only VFS-backed regular-file redirection target as likely smaller
next candidates than writable regular-file output. Append/truncate and
writable filesystem behavior require a separate filesystem mutation plan.

## Validation Summary

- static inspection: accepted stdout-to-/dev/null and stderr-to-/dev/null
  task records, closeout records, roadmap entries, task-owned evidence logs,
  and retained control evidence were inspected.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

Acceptance commit: recorded in durable supervisor state after commit creation.
