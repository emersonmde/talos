# Phase 10 /dev/null Stdio Redirection Frontier Closeout

Task: phase10-dev-null-stdio-redirection-frontier-closeout-20260604
Status: accepted

## Scope

Checkpoint the accepted /dev/null standard-I/O redirection frontier after all
three standard descriptor directions have task-owned evidence.

The accepted shell-visible forms are exactly:

- 'exec stdout >/dev/null': launched VFS-backed '/bin/stdout' has child fd1
  rebound to the explicit /dev/null sink device.
- 'exec stderr 2>/dev/null': launched VFS-backed '/bin/stderr' has child fd2
  rebound to the explicit /dev/null sink device.
- 'exec stdin </dev/null': launched VFS-backed '/bin/stdin' has child fd0
  rebound to the explicit /dev/null source device.

All three mutations are child-only descriptor-table changes. The shell restores
the relevant standard descriptor after the child exits, proven by following
normal 'exec stdout', 'exec stderr', and 'exec stdin' controls.

This closeout does not add code and does not expand into regular-file input or
output redirection, append/truncate, writable filesystem behavior, arbitrary
descriptor syntax, broader file/device semantics, multi-stage/concurrent
pipelines, Pi 5 proof, networking, SSH, or a phase transition.

## Findings

- fixed: Reconciled /dev/null as an explicit device with two sink routes
  for fd1/fd2 and one source route for fd0, instead of filesystem mutation.
- fixed: Confirmed stdout and stderr sink redirection both report
  'op=sink', 'target-path=/dev/null', 'target-stream=null-sink',
  'target-route=device:/dev/null', return the accepted 31-byte TalosWrite
  count, and discard the userspace fixture payload.
- fixed: Confirmed stdin source redirection reports 'op=source',
  'source-path=/dev/null', 'source-stream=null-source',
  'source-route=device:/dev/null', returns a zero-byte TalosRead result, and
  reports 'read-result=null-source-eof/no-data' without polling
  runtime-console0 input.
- fixed: Confirmed descriptor restoration remains child-only for fd0/fd1/fd2
  by mapping following normal stdio controls back to runtime-console0 routes.
- fixed: Preserved retained evidence for descriptor redirection, descriptor
  close, descriptor-mixing pipelines, normal stdout/stderr routes, stdin
  readiness and terminal EOF, VFS exec, lifecycle/status, waitpid, laststatus,
  deterministic negatives, and descriptor-backed cat.
- fixed: Updated the roadmap to prevent acceptance drift from the exact
  /dev/null stdio forms into regular-file redirection, append/truncate,
  writable filesystems, broader descriptor syntax, or broader file/device
  semantics.
- not-an-issue: The task-owned smoke logs include later visible stdout,
  stderr, and stdin fixture payloads because those later commands intentionally
  prove shell descriptor restoration after the redirected children exit.
- deferred: '1>/dev/null', arbitrary 'N>target', 'N<target', 'N>&M',
  descriptor moves, output regular-file redirection, append/truncate,
  read-only regular-file stdin redirection, writable filesystem behavior,
  here-docs, arbitrary path expansion, multi-stage/concurrent pipelines,
  pipefail, jobs, async execution, fork, signals, job control, Pi 5 proof,
  networking, SSH, and any phase transition.

## Evidence Map

- stdout-to-/dev/null evidence:
  'tasks/evidence/2026-06-04-phase10-dev-null-stdout-redirection-contract-core/qemu-local-shell-dev-null-stdout-redirection-smoke.log'
  records command 3 'exec stdout >/dev/null', child 'fd1=device',
  'exec-redirection op=sink ... target-path=/dev/null ...
  target-stream=null-sink target-route=device:/dev/null child-only=true
  shell-restored=true', 'exec-stdout ... stream=null-sink
  route=device:/dev/null', 'bytes=0x1f', 'return=0x1f', lifecycle/status,
  waitpid, laststatus, deterministic negatives, final classification
  'qemu-local-shell-dev-null-stdout-redirection-complete', errors=0, and PASS.
- stdout restoration control: the same log records the following normal
  'exec stdout', visible 'Talos userspace stdout fixture', 'fd1=stdio-output',
  and 'stream=stdout route=runtime-console0/stdout'.
- stderr-to-/dev/null evidence:
  'tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-dev-null-stderr-redirection-smoke.log'
  records command 3 'exec stderr 2>/dev/null', child 'fd2=device',
  'exec-redirection op=sink ... target-path=/dev/null ...
  target-stream=null-sink target-route=device:/dev/null child-only=true
  shell-restored=true', 'exec-stderr ... stream=null-sink
  route=device:/dev/null', 'bytes=0x1f', 'return=0x1f', lifecycle/status,
  waitpid, laststatus, deterministic negatives, final classification
  'qemu-local-shell-dev-null-stderr-redirection-complete', errors=0, and PASS.
- stderr restoration control: the same log records the following normal
  'exec stderr', visible 'Talos userspace stderr fixture', 'fd2=stdio-output',
  and 'stream=stderr route=runtime-console0/stderr'.
- stdin-from-/dev/null evidence:
  'tasks/evidence/2026-06-04-phase10-dev-null-stdin-redirection-core/qemu-local-shell-dev-null-stdin-redirection-smoke.log'
  records command 3 'exec stdin </dev/null', child 'fd0=device',
  'exec-redirection op=source ... source-path=/dev/null ...
  source-stream=null-source source-route=device:/dev/null child-only=true
  shell-restored=true', visible
  'Talos userspace stdin fixture read-result: null-source-eof/no-data',
  'exec-stdin ... bytes=0x0 return=0x0 read-source=device:/dev/null ...
  read-result=null-source-eof/no-data', lifecycle/status, waitpid,
  laststatus, deterministic negatives, final classification
  'qemu-local-shell-dev-null-stdin-redirection-complete', errors=0, and PASS.
- stdin restoration control: the same log records the following normal
  'exec stdin', visible 'Talos userspace stdin fixture read: talos-console0',
  'fd0=stdio-input', and 'read-source=runtime-console0/local-input'.
- retained descriptor redirection and close controls:
  'tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log',
  'tasks/evidence/2026-06-04-phase10-stdout-close-redirection-core/qemu-local-shell-stdout-close-redirection-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-stderr-close-redirection-core/qemu-local-shell-stderr-close-redirection-smoke.log'.
- retained descriptor-mixing pipeline controls:
  'tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log',
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log',
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log'.
- retained stdin readiness and EOF controls:
  'tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log',
  'tasks/evidence/2026-06-04-phase10-runtime-stdin-readiness-distinction-core/qemu-local-shell-runtime-stdin-readiness-smoke.log',
  'tasks/evidence/2026-06-03-phase10-stdin-eof-no-data-core/qemu-local-shell-stdin-eof-no-data-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log'.
- retained normal stdio and stream routing controls:
  'tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-distinct-stderr-routing-core/qemu-local-shell-distinct-stderr-routing-smoke.log'.
- retained VFS exec, lifecycle/status, waitpid, laststatus, negative controls,
  and descriptor-backed file I/O:
  'tasks/evidence/2026-06-03-phase10-absolute-vfs-exec-dispatch-core/qemu-local-shell-absolute-vfs-exec-dispatch-smoke.log',
  'tasks/evidence/2026-06-03-phase10-vfs-exec-nonzero-status-core/qemu-local-shell-nonzero-vfs-exec-status-smoke.log',
  'tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log',
  'tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log', and
  'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log'.

## Accepted Frontier

Accepted:

- exactly 'exec stdout >/dev/null';
- exactly 'exec stderr 2>/dev/null';
- exactly 'exec stdin </dev/null';
- /dev/null as an explicit fd1/fd2 output sink device and explicit fd0 input
  source device;
- child-only rebinding of the relevant standard descriptor with shell
  restoration after the child exits;
- sink writes return the accepted byte count while discarding userspace bytes;
- source reads return zero bytes as true /dev/null EOF/no-data without polling
  runtime-console0;
- VFS-backed launch, lifecycle/status, waitpid, laststatus, normal stdio
  restoration, deterministic negatives, descriptor redirection/pipeline
  controls, stdin readiness/EOF controls, and descriptor-backed cat are
  covered by retained QEMU/substitute evidence.

Deferred:

- shorthand or arbitrary descriptor syntax such as '1>/dev/null', 'N>target',
  'N<target', 'N>&M', and descriptor moves;
- regular-file input/output redirection, append/truncate, writable filesystem
  behavior, broader file/device semantics, here-docs, and arbitrary path
  expansion;
- multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, and job control;
- Pi 5 proof, networking, SSH, and any phase transition.

## Next Step Requirement

The queued read-only regular-file stdin redirection core is mechanically
unblocked after this closeout is accepted and committed. It must stay bounded
to a read-only input source opened through the accepted descriptor-backed
initramfs/VFS path. It must not add output regular-file redirection,
append/truncate, writable filesystem behavior, arbitrary descriptor syntax,
networking, SSH, Pi 5 proof, or a phase transition.

## Validation

- static inspection: accepted stdout, stderr, and stdin /dev/null task records,
  task-owned evidence logs, retained stdio/redirection/pipeline/stdin/VFS/
  lifecycle/wait/status/cat controls, and roadmap entries were inspected.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final closeout commit recorded in supervisor state.
