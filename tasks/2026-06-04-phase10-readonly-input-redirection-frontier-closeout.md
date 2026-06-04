# Phase 10 Read-Only Input Redirection Frontier Closeout

Task: phase10-readonly-input-redirection-frontier-closeout-20260604
Status: accepted

## Scope

Checkpoint the accepted read-only input-redirection frontier after the
task-owned '/dev/null' stdin and regular-file stdin slices.

The accepted shell-visible input redirection forms are exactly
'exec stdin </dev/null' and 'exec stdin </etc/banner.txt' for the VFS-backed
'/bin/stdin' fixture. The launched child has fd0 rebound to either the
'/dev/null' source device or a read-only initramfs regular-file descriptor for
'/etc/banner.txt'; the shell restores fd0 after the child exits.

This closeout does not add code and does not expand into output regular-file
redirection, append/truncate, writable filesystem behavior, arbitrary
descriptor syntax, arbitrary path expansion, here-docs, broader pipe syntax,
Pi 5 proof, networking, SSH, or a phase transition.

## Findings

- fixed: Consolidated exact 'exec stdin </dev/null' as an accepted child-only
  fd0 source redirection backed by the '/dev/null' device-source descriptor.
- fixed: Consolidated exact 'exec stdin </etc/banner.txt' as an accepted
  child-only fd0 source redirection backed by the descriptor-facing
  initramfs 'TalosOpen'/'TalosRead' regular-file path.
- fixed: Confirmed both input forms report child-only shell restoration, and
  following normal 'exec stdin' controls read from runtime-console0/local-input
  through restored shell fd0.
- fixed: Retained stdout/stderr '/dev/null' sink evidence so the frontier is
  distinguishable from accepted output-to-device redirection while still
  deferring output regular-file redirection and filesystem mutation.
- fixed: Retained descriptor dup/close redirection, descriptor-mixing
  pipeline, VFS exec/open/read, lifecycle/status, waitpid, laststatus,
  deterministic negative, normal stdio, stdin readiness/EOF, and
  descriptor-backed 'cat /etc/banner.txt' controls.
- fixed: Updated the roadmap with a single read-only input redirection
  frontier entry that explicitly prevents drift into output regular-file
  redirection, append/truncate, writable filesystem behavior, broader
  descriptor syntax, networking, SSH, Pi 5 proof, or a phase transition.
- deferred: output regular-file redirection, append/truncate, writable
  filesystem mutation, arbitrary descriptor syntax, descriptor moves,
  arbitrary path expansion, here-docs, broader pipe syntax, multi-stage or
  concurrent pipelines, pipefail, jobs, async execution, fork, signals,
  Pi 5 proof, networking, SSH, and any phase transition.

## Evidence Map

- '/dev/null' stdin source evidence:
  'tasks/evidence/2026-06-04-phase10-dev-null-stdin-redirection-core/qemu-local-shell-dev-null-stdin-redirection-smoke.log'
  records command 'exec stdin </dev/null', visible
  'Talos userspace stdin fixture read-result: null-source-eof/no-data',
  'fd0=device', 'exec-redirection op=source ...
  source-path=/dev/null ... source-stream=null-source
  source-route=device:/dev/null child-only=true shell-restored=true',
  'exec-stdin ... bytes=0x0 return=0x0 read-source=device:/dev/null ...
  read-result=null-source-eof/no-data', lifecycle/status, waitpid,
  laststatus, unsupported-form negatives, final classification
  'qemu-local-shell-dev-null-stdin-redirection-complete', errors=0, and PASS.
- read-only regular-file stdin source evidence:
  'tasks/evidence/2026-06-04-phase10-readonly-regular-file-stdin-redirection-core/qemu-local-shell-readonly-regular-file-stdin-redirection-smoke.log'
  records command 'exec stdin </etc/banner.txt', visible
  'Talos userspace stdin fixture read: Talos initramfs fixture',
  'fd0=regular-file', 'exec-redirection op=source ...
  source-path=/etc/banner.txt ... source-stream=regular-file
  source-route=initramfs:/etc/banner.txt child-only=true shell-restored=true',
  'exec-stdin ... bytes=0x18 return=0x18
  read-source=initramfs:/etc/banner.txt ...
  read-result=regular-file-eof-after-read', lifecycle/status, waitpid,
  laststatus, deterministic negatives, final classification
  'qemu-local-shell-readonly-regular-file-stdin-redirection-complete',
  errors=0, and PASS.
- shell fd0 restoration controls: both input-redirection logs record following
  normal 'exec stdin' runs with 'fd0=stdio-input' and
  'read-source=runtime-console0/local-input'.
- retained '/dev/null' output sink evidence:
  'tasks/evidence/2026-06-04-phase10-dev-null-stdout-redirection-contract-core/qemu-local-shell-dev-null-stdout-redirection-smoke.log' and
  'tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-dev-null-stderr-redirection-smoke.log'.
- retained runtime-console0 stdin readiness, scheduler wait, and terminal EOF
  controls:
  'tasks/evidence/2026-06-04-phase10-runtime-stdin-readiness-distinction-core/qemu-local-shell-runtime-stdin-readiness-smoke.log',
  'tasks/evidence/2026-06-04-phase10-scheduler-backed-stdin-wait-core/qemu-local-shell-scheduler-backed-stdin-wait-smoke.log',
  'tasks/evidence/2026-06-03-phase10-stdin-eof-no-data-core/qemu-local-shell-stdin-eof-no-data-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-terminal-ctrl-d-eof-core/qemu-local-shell-terminal-ctrl-d-eof-smoke.log'.
- retained normal stdio and distinct stream controls:
  'tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stdin-inherited-fd-core/qemu-local-shell-userspace-stdin-smoke.log',
  'tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log', and
  'tasks/evidence/2026-06-04-phase10-distinct-stderr-routing-core/qemu-local-shell-distinct-stderr-routing-smoke.log'.
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
- retained VFS exec/open/read, lifecycle/status, waitpid, laststatus,
  negative controls, and descriptor-backed file I/O:
  'tasks/evidence/2026-06-03-phase10-absolute-vfs-exec-dispatch-core/qemu-local-shell-absolute-vfs-exec-dispatch-smoke.log',
  'tasks/evidence/2026-06-03-phase10-vfs-exec-nonzero-status-core/qemu-local-shell-nonzero-vfs-exec-status-smoke.log',
  'tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log',
  'tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log', and
  'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log'.

## Accepted Frontier

Accepted:

- exactly 'exec stdin </dev/null';
- exactly 'exec stdin </etc/banner.txt';
- child-only fd0 rebinding to either the '/dev/null' source device or a
  read-only regular-file descriptor for '/etc/banner.txt';
- source identity as 'device:/dev/null' or
  'initramfs:/etc/banner.txt', with regular-file reads performed through the
  accepted descriptor-backed VFS/open/read path;
- zero-byte true EOF/no-data behavior for '/dev/null' and EOF-after-read
  behavior for the banner fixture;
- shell fd0 restoration after child exit;
- accepted stdout/stderr '/dev/null' sink redirection remains separate from
  this input frontier and is covered by retained evidence;
- VFS-backed launch, lifecycle/status, waitpid, laststatus, normal stdio
  restoration, deterministic negatives, descriptor redirection/pipeline
  controls, stdin readiness/EOF controls, and descriptor-backed cat are
  covered by retained QEMU/substitute evidence.

Deferred:

- output regular-file redirection, append/truncate, writable filesystem
  mutation, and broader file/device semantics;
- shorthand or arbitrary descriptor syntax such as 'N<target', 'N>target',
  'N>&M', descriptor moves, and close/restore expansion beyond the accepted
  exact forms;
- arbitrary path expansion, here-docs, globbing, quoting, variables, and
  environment-backed path behavior;
- multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, and job control;
- Pi 5 proof, networking, SSH, and any phase transition.

## Next Step Requirement

No further input-redirection implementation is mechanically required by this
frontier. The next feature-led task should be planned explicitly by the
supervisor because the obvious follow-ups split across distinct risks:
output redirection to a regular file requires writable filesystem semantics;
broader descriptor grammar affects parser and descriptor ownership; and
process-accounting work belongs to the lifecycle surface. The worker must not
promote one of those directions without an explicit queued task.

## Validation

- static inspection: accepted '/dev/null' stdin, read-only regular-file stdin,
  '/dev/null' output sink, normal stdio, descriptor redirection, pipeline,
  stdin readiness/EOF, VFS/open/read, lifecycle/wait/status/cat controls, and
  roadmap entries were inspected.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final frontier closeout commit recorded in supervisor state.
