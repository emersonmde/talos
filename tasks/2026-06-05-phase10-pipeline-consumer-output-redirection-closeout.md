# Phase 10 Pipeline Consumer-Output Redirection Closeout

Task: phase10-pipeline-consumer-output-redirection-closeout-20260605
Status: accepted

## Scope

Close out the accepted pipeline consumer-output redirection frontier without
adding code.

The accepted shell-visible form is:

- 'exec stdout | exec stdin >/tmp/pipe-consumer.txt'

This form composes the accepted exact two-stage VFS-backed stdout-to-stdin
pipeline with the accepted volatile VFS stdout sink policy for conservative
'/tmp/<basename>' output targets. The producer fd1 remains the pipe writer;
only the consumer fd1 is rebound to a child-only volatile regular file. No
pipeline scheduling, descriptor policy, output path policy, persistence
policy, or lifecycle policy changed in this closeout.

This closeout does not implement code, run Pi 5 hardware, acquire
hardwareTestLock, add multi-stage or concurrent pipelines, producer file
redirection, consumer append/stderr redirection, arbitrary descriptor syntax,
persistent storage, recursive directories, arbitrary input paths, process
accounting/concurrency, networking, SSH, or a phase transition.

## Findings

- fixed: Consolidated the primary pipeline consumer-output evidence showing
  producer fd1 as 'pipe-endpoint', consumer fd0 as 'pipe-endpoint', consumer
  fd1 as 'regular-file', and the child-only stdout sink route
  'volatile-vfs:/tmp/pipe-consumer.txt'.
- fixed: Confirmed the producer writes 0x1f accepted fixture bytes to
  'stream=pipe-writer route=pipe:stdout-to-stdin', the consumer reads 0x1f
  bytes from 'pipe:stdout-to-stdin', and the pipeline record reports
  writer closure, reader EOF, shell restoration, and
  'source=shell-pipe-consumer-stdout-redirection'.
- fixed: Consolidated descriptor-backed readback of
  '/tmp/pipe-consumer.txt', including the 0x44-byte userspace stdin report,
  consuming 'waitpid', non-consuming 'laststatus', lifecycle/status
  provenance, errors=0, final classification, and PASS.
- fixed: Confirmed deterministic negatives reject consumer append redirection,
  stderr-producer consumer-output redirection, and producer-file plus
  consumer-file redirection outside the accepted surface.
- fixed: Retained minimal pipeline, pipeline stderr-not-piped,
  descriptor-mixing pipeline, arbitrary '/tmp' output redirection,
  descriptor-backed cat, waitpid, and laststatus control evidence with
  PASS/classification markers.
- not-an-issue: The first core smoke iteration's overlong negative command was
  corrected in the retained evidence; the accepted log proves parser rejection
  for a shorter unsupported producer-file plus consumer-file form rather than
  line truncation.
- not-an-issue: Pi 5 hardware proof remains outside this QEMU/substitute
  feature slice; hardwareTestLock stayed unlocked/restored and unused.
- deferred: producer file redirection away from the pipe, consumer append
  output, stderr-producing pipelines with consumer output redirection,
  producer file redirection combined with consumer output redirection,
  multi-stage/concurrent pipelines, pipefail, jobs, fork/signals, arbitrary
  descriptor syntax, persistence, recursive directories, process
  accounting/concurrency, Pi 5 proof, networking, SSH, and phase transition.

## Evidence Map

- primary pipeline consumer-output evidence:
  'tasks/evidence/2026-06-05-phase10-pipeline-consumer-output-redirection-core/qemu-local-shell-pipeline-consumer-output-redirection-smoke.log'
  records the accepted
  'exec stdout | exec stdin >/tmp/pipe-consumer.txt' command.
- producer pipeline evidence:
  the primary log records '/bin/stdout' inheriting fd1 as 'pipe-endpoint' and
  writing 0x1f bytes to
  'stream=pipe-writer route=pipe:stdout-to-stdin'.
- consumer redirection evidence:
  the primary log records '/bin/stdin' inheriting fd0 as 'pipe-endpoint' and
  fd1 as 'regular-file', plus
  'exec-redirection op=sink source-fd=0x0000000000000001
  target-path=/tmp/pipe-consumer.txt
  target-route=volatile-vfs:/tmp/pipe-consumer.txt child-only=true
  shell-restored=true'.
- pipe and restoration evidence:
  the primary log records
  'bytes-written=0x000000000000001f bytes-read=0x000000000000001f
  writer-closed=true reader-eof=true shell-restored=true
  source=shell-pipe-consumer-stdout-redirection'.
- descriptor-backed readback evidence:
  the primary log records 'cat /tmp/pipe-consumer.txt' reading
  'Talos userspace stdin fixture read: Talos userspace stdout fixture' from
  'source=volatile-vfs-descriptor-read' with bytes=0x44.
- lifecycle evidence:
  the primary log records accepted loader/startup ABI/lifecycle/status
  records for both VFS-backed programs, consuming 'waitpid', non-consuming
  'laststatus', errors=0, final
  'classification=qemu-local-shell-pipeline-consumer-output-redirection-complete',
  and PASS.
- deterministic negative evidence:
  the primary log rejects
  'exec stdout | exec stdin >>/tmp/pipe-consumer.txt',
  'exec stderr | exec stdin >/tmp/pipe-consumer.txt', and
  'exec stdout >/tmp/src.txt | exec stdin >/tmp/out.txt'.
- retained minimal pipeline evidence:
  'tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log'.
- retained pipeline stderr/descriptor-mixing evidence:
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log',
  'tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log',
  and
  'tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log'.
- retained arbitrary '/tmp' output evidence:
  'tasks/evidence/2026-06-04-phase10-stdout-arbitrary-tmp-output-redirection-core/qemu-local-shell-stdout-arbitrary-tmp-output-redirection-smoke.log'
  and
  'tasks/evidence/2026-06-04-phase10-stderr-arbitrary-tmp-output-redirection-core/qemu-local-shell-stderr-arbitrary-tmp-output-redirection-smoke.log'.
- retained lifecycle/status and descriptor-backed cat evidence:
  'tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log',
  'tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log',
  and
  'tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log'.

## Accepted Frontier

Accepted:

- exact two-stage form
  'exec stdout | exec stdin >/tmp/pipe-consumer.txt';
- producer fd1 bound to the accepted pipe writer while producer stdout writes
  fixture bytes to the pipe;
- consumer fd0 bound to the accepted pipe reader and consumer fd1 rebound to
  the accepted volatile VFS '/tmp/<basename>' route for the child process only;
- descriptor-backed readback of the consumer output file through
  'cat /tmp/pipe-consumer.txt';
- waitpid, laststatus, lifecycle/status, VFS exec/open/read/write,
  descriptor restoration, pipe endpoint closure, minimal pipeline,
  descriptor-mixing pipeline, arbitrary '/tmp' output redirection, and
  descriptor-backed cat controls.

Deferred:

- producer output file redirection away from the pipe;
- consumer append output, stderr-producing pipeline consumer output, and
  producer-file plus consumer-file pipeline redirection combinations;
- arbitrary descriptor syntax, descriptor moves, here-docs, broader shell
  grammar, quoting, variables, globbing, pipefail, jobs, async execution,
  fork, signals, process accounting/concurrency, and job control;
- persistent filesystem behavior, recursive directories, traversal, broad
  writable filesystem mutation, and path persistence claims;
- Pi 5 proof, networking, SSH, and any phase transition.

## Next Step

The next explicit queued task,
'phase10-pipeline-producer-file-redirection-away-core-20260605', is
mechanically unblocked after this closeout. It must remain bounded to the
exact inverse pipeline/file-redirection form where producer fd1 redirects to a
volatile '/tmp/<basename>' file and the downstream pipe consumer receives
EOF/no-data.

## Validation

- static inspection: accepted pipeline consumer-output QEMU/substitute
  evidence was inspected, including producer pipe writer records, consumer
  pipe reader and volatile VFS fd1 sink records, descriptor-backed file
  readback, waitpid, laststatus, deterministic negatives, completion marker,
  errors=0, and PASS.
- static inspection: retained minimal pipeline, stderr-not-piped pipeline,
  descriptor-mixing pipeline, arbitrary '/tmp' output redirection,
  descriptor-backed cat, waitpid, and laststatus controls were checked for
  PASS/classification markers.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final pipeline consumer-output redirection closeout commit recorded in
supervisor state.
