# Phase 10 Combined Stdin/Stdout Redirection Closeout

Task: phase10-combined-stdin-stdout-redirection-closeout-20260605
Status: accepted

## Scope

Close out the accepted single-command combined stdin/stdout redirection
frontier without adding code.

The accepted shell-visible form is:

- 'exec stdin </etc/banner.txt >/tmp/stdin-report.txt'

This form composes the already accepted read-only initramfs fd0 source policy
for '/etc/banner.txt' with the already accepted volatile VFS stdout sink policy
for conservative '/tmp/<basename>' output targets. No redirection ordering,
path policy, descriptor policy, or persistence policy changed in this closeout.

This closeout does not implement code, run Pi 5 hardware, acquire
hardwareTestLock, add arbitrary input paths, '/dev/null' combined input, append
or stderr combined forms, persistent storage, recursive directories, arbitrary
descriptor syntax, descriptor moves, multi-command redirection, process
accounting/concurrency, networking, SSH, or a phase transition.

## Findings

- fixed: Consolidated the primary combined-redirection evidence showing fd0
  rebound to 'initramfs:/etc/banner.txt' and fd1 rebound to
  'volatile-vfs:/tmp/stdin-report.txt' for one VFS-backed userspace
  '/bin/stdin' launch.
- fixed: Confirmed the child descriptor table records 'fd0=regular-file',
  'fd1=regular-file', and 'fd2=stdio-output', with separate child-only
  'exec-redirection' records for fd0 and fd1 and shell descriptor restoration.
- fixed: Consolidated descriptor-backed readback of
  '/tmp/stdin-report.txt', including the userspace stdin fixture's report,
  waitpid consumption, non-consuming laststatus, lifecycle/status provenance,
  errors=0, final classification, and PASS.
- fixed: Confirmed deterministic negatives reject output-first ordering,
  combined '/dev/null' input, explicit fd1 aliasing in this combined form, and
  spaced input grammar outside the accepted surface.
- fixed: Retained read-only stdin, '/dev/null' stdin, arbitrary '/tmp' stdout
  output, pipeline, descriptor routing, waitpid, laststatus, and
  descriptor-backed cat control evidence with PASS/classification markers.
- not-an-issue: The core task's canonical line-capacity increase from 32 to 64
  bytes was an implementation fix needed to carry the accepted exact serial
  command; it did not broaden the conservative parser grammar or require an
  ADR.
- not-an-issue: Pi 5 hardware proof remains outside this QEMU/substitute
  feature slice; hardwareTestLock stayed unlocked/restored and unused.
- deferred: arbitrary input paths, '/dev/null' combined input, append combined
  forms, stderr combined forms, broader descriptor grammar, descriptor moves,
  here-docs, quoting, variables, globbing, multi-command redirection,
  multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, persistent filesystem behavior, process accounting/concurrency,
  Pi 5 proof, networking, SSH, and phase transition.

## Evidence Map

- primary combined-redirection evidence:
  'tasks/evidence/2026-06-05-phase10-combined-stdin-stdout-redirection-core/qemu-local-shell-combined-stdin-stdout-redirection-smoke.log'
  records the accepted
  'exec stdin </etc/banner.txt >/tmp/stdin-report.txt' command.
- fd0 source evidence:
  the primary log records
  'exec-redirection op=source source-fd=0x0000000000000000
  source-path=/etc/banner.txt source-route=initramfs:/etc/banner.txt
  child-only=true shell-restored=true'.
- fd1 sink evidence:
  the primary log records
  'exec-redirection op=sink source-fd=0x0000000000000001
  target-path=/tmp/stdin-report.txt
  target-route=volatile-vfs:/tmp/stdin-report.txt child-only=true
  shell-restored=true'.
- child descriptor evidence:
  the primary log records 'exec-descriptors' with 'fd0=regular-file',
  'fd1=regular-file', 'fd2=stdio-output',
  'loader-temp-open=false', and
  'source=shell-process-descriptor-table'.
- userspace stdin/stdout composition evidence:
  the primary log records '/bin/stdin' reading from
  'initramfs:/etc/banner.txt', writing 'stdout-bytes=0x3d' through redirected
  fd1, and later descriptor-backed 'cat /tmp/stdin-report.txt' reading
  'Talos userspace stdin fixture read: Talos initramfs fixture' from
  'source=volatile-vfs-descriptor-read'.
- lifecycle evidence:
  the primary log records the accepted loader, startup ABI, lifecycle/status,
  consuming 'waitpid', non-consuming 'laststatus', errors=0, final
  'classification=qemu-local-shell-combined-stdin-stdout-redirection-complete',
  and PASS.
- deterministic negative evidence:
  the primary log rejects
  'exec stdin >/tmp/stdin-report.txt </etc/banner.txt',
  'exec stdin </dev/null >/tmp/stdin-report.txt',
  'exec stdin </etc/banner.txt 1>/tmp/stdin-report.txt', and
  'exec stdin < /etc/banner.txt >/tmp/stdin-report.txt'.
- retained read-only and '/dev/null' stdin evidence:
  'tasks/evidence/2026-06-04-phase10-readonly-regular-file-stdin-redirection-core/qemu-local-shell-readonly-regular-file-stdin-redirection-smoke.log'
  and
  'tasks/evidence/2026-06-04-phase10-dev-null-stdin-redirection-core/qemu-local-shell-dev-null-stdin-redirection-smoke.log'.
- retained arbitrary '/tmp' stdout output evidence:
  'tasks/evidence/2026-06-04-phase10-stdout-arbitrary-tmp-output-redirection-core/qemu-local-shell-stdout-arbitrary-tmp-output-redirection-smoke.log'.
- retained pipeline and descriptor-routing evidence:
  'tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log'
  and
  'tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log'.
- retained lifecycle/status and descriptor-backed cat evidence:
  'tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log',
  'tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log',
  and 'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log'.

## Accepted Frontier

Accepted:

- exact combined form
  'exec stdin </etc/banner.txt >/tmp/stdin-report.txt';
- fd0 source rebinding to the accepted read-only initramfs
  '/etc/banner.txt' route for the child process only;
- fd1 sink rebinding to the accepted volatile VFS '/tmp/<basename>' route for
  the child process only;
- one VFS-backed userspace '/bin/stdin' process observing both descriptor
  mutations in its inherited descriptor table;
- descriptor-backed readback of the volatile output file through
  'cat /tmp/stdin-report.txt';
- waitpid, laststatus, lifecycle/status, VFS exec/open/read/write, read-only
  input redirection, arbitrary '/tmp' stdout output redirection, pipeline,
  descriptor-routing, and descriptor-backed cat controls.

Deferred:

- arbitrary input paths and '/dev/null' combined input;
- append, stderr, explicit fd alias, and reordered combined-redirection forms;
- arbitrary descriptor syntax, descriptor moves, here-docs, broader shell
  grammar, quoting, variables, globbing, and multi-command redirection;
- persistent filesystem behavior, recursive directories, traversal, broad
  writable filesystem mutation, and path persistence claims;
- multi-stage/concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, process accounting/concurrency, and job control;
- Pi 5 proof, networking, SSH, and any phase transition.

## Next Step

The next explicit queued task,
'phase10-pipeline-consumer-output-redirection-core-20260605', is mechanically
unblocked after this closeout. It must remain bounded to exact consumer stdout
file redirection on the accepted two-stage pipeline and must not infer broader
pipeline scheduling, process accounting/concurrency, persistent filesystem
behavior, hardware proof, networking, SSH, or a phase transition.

## Validation

- static inspection: accepted combined-redirection QEMU/substitute evidence
  was inspected, including fd0 source rebinding, fd1 volatile VFS sink
  rebinding, child descriptor inheritance, descriptor-backed file readback,
  waitpid, laststatus, deterministic negatives, completion marker, errors=0,
  and PASS.
- static inspection: retained read-only stdin, '/dev/null' stdin,
  arbitrary '/tmp' stdout output, pipeline, descriptor-routing, waitpid,
  laststatus, and descriptor-backed cat controls were checked for
  PASS/classification markers.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final combined stdin/stdout redirection closeout commit recorded in
supervisor state.
