# Phase 10 Read-Only Regular-File Stdin Redirection Closeout

Task: phase10-readonly-regular-file-stdin-redirection-closeout-20260604
Status: accepted

## Scope

Checkpoint the accepted read-only regular-file input-redirection frontier after
the task-owned core evidence for 'exec stdin </etc/banner.txt'.

The accepted shell-visible form is exactly 'exec stdin </etc/banner.txt' for
the VFS-backed '/bin/stdin' fixture. The launched child has fd0 rebound to a
read-only initramfs regular-file descriptor for '/etc/banner.txt'; the shell
restores fd0 after the child exits.

This closeout does not add code and does not expand into output regular-file
redirection, append/truncate, writable filesystem behavior, arbitrary
descriptor syntax, arbitrary path expansion, here-docs, broader pipe syntax,
Pi 5 proof, networking, SSH, or a phase transition.

## Findings

- fixed: Reconciled 'exec stdin </etc/banner.txt' as the first accepted
  regular-file input source, backed by the descriptor-facing initramfs
  'TalosOpen'/'TalosRead' path rather than a kernel-backed command shim.
- fixed: Confirmed the child fd0 source reports 'fd0=regular-file',
  'op=source', 'source-path=/etc/banner.txt',
  'source-stream=regular-file', and
  'source-route=initramfs:/etc/banner.txt'.
- fixed: Confirmed the userspace stdin fixture reads exactly the banner
  payload from fd0, reports 'read-source=initramfs:/etc/banner.txt',
  'bytes=0x18', and 'read-result=regular-file-eof-after-read', then exits
  through the accepted lifecycle/status path.
- fixed: Confirmed shell fd0 restoration with a following normal 'exec stdin'
  control that reads 'talos-console0' from runtime-console0/local-input.
- fixed: Preserved retained controls for '/dev/null' stdin source behavior,
  runtime-console0 stdin readiness/EOF behavior, userspace stdout/stderr,
  descriptor dup/close redirection, descriptor-mixing pipelines,
  waitpid/laststatus, deterministic negatives, and descriptor-backed
  'cat /etc/banner.txt'.
- fixed: Updated the roadmap so the accepted frontier remains exactly one
  read-only regular-file stdin redirection form and does not drift into output
  regular-file redirection, append/truncate, writable filesystem behavior, or
  broader descriptor grammar.
- not-an-issue: The accepted file-description table capacity of two remains
  sufficient for this frontier because only redirected fd0 and the loader
  temporary descriptor are live together.
- deferred: output regular-file redirection, append/truncate, writable
  filesystem mutation, arbitrary descriptor syntax, descriptor moves,
  arbitrary path expansion, here-docs, broader pipe syntax, multi-stage or
  concurrent pipelines, pipefail, jobs, async execution, fork, signals,
  Pi 5 proof, networking, SSH, and any phase transition.

## Evidence Map

- read-only regular-file stdin evidence:
  'tasks/evidence/2026-06-04-phase10-readonly-regular-file-stdin-redirection-core/qemu-local-shell-readonly-regular-file-stdin-redirection-smoke.log'
  records command 3 'exec stdin </etc/banner.txt', visible
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
- shell fd0 restoration control: the same log records the following normal
  'exec stdin', visible 'Talos userspace stdin fixture read: talos-console0',
  'fd0=stdio-input', and 'read-source=runtime-console0/local-input'.
- retained '/dev/null' stdin source control:
  'tasks/evidence/2026-06-04-phase10-dev-null-stdin-redirection-core/qemu-local-shell-dev-null-stdin-redirection-smoke.log'.
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
- retained VFS exec, lifecycle/status, waitpid, laststatus, negative controls,
  and descriptor-backed file I/O:
  'tasks/evidence/2026-06-03-phase10-absolute-vfs-exec-dispatch-core/qemu-local-shell-absolute-vfs-exec-dispatch-smoke.log',
  'tasks/evidence/2026-06-03-phase10-vfs-exec-nonzero-status-core/qemu-local-shell-nonzero-vfs-exec-status-smoke.log',
  'tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log',
  'tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log', and
  'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log'.

## Accepted Frontier

Accepted:

- exactly 'exec stdin </etc/banner.txt';
- child-only fd0 rebinding to a read-only regular-file source descriptor for
  '/etc/banner.txt';
- source identity as 'initramfs:/etc/banner.txt', with reads performed through
  the accepted descriptor-backed VFS/open/read path;
- 'TalosRead' content provenance, byte count, and EOF-after-read behavior for
  the banner fixture;
- shell fd0 restoration after child exit;
- VFS-backed launch, lifecycle/status, waitpid, laststatus, normal stdio
  restoration, deterministic negatives, '/dev/null' stdin control,
  descriptor redirection/pipeline controls, stdin readiness/EOF controls, and
  descriptor-backed cat are covered by retained QEMU/substitute evidence.

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

The queued read-only input redirection frontier closeout is mechanically
unblocked after this closeout is accepted and committed. It must remain
docs/evidence reconciliation only across accepted '/dev/null' stdin and
read-only regular-file stdin behavior, and must not add output regular-file
redirection, append/truncate, writable filesystem behavior, arbitrary
descriptor syntax, networking, SSH, Pi 5 proof, or a phase transition.

## Validation

- static inspection: accepted read-only regular-file stdin task record,
  task-owned evidence log, retained stdio/redirection/pipeline/stdin/VFS/
  lifecycle/wait/status/cat controls, and roadmap entries were inspected.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final closeout commit recorded in supervisor state.
