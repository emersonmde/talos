# Phase 10 Pipeline File-Redirection Frontier Closeout

Task: phase10-pipeline-file-redirection-frontier-closeout-20260605
Status: accepted

## Scope

Close out the accepted pipeline plus volatile-file redirection frontier without
adding code.

Accepted shell-visible forms:

- 'exec stdout | exec stdin >/tmp/pipe-consumer.txt'
- 'exec stdout >/tmp/pipe-source.txt | exec stdin'

These forms compose the accepted exact two-stage VFS-backed stdout-to-stdin
pipeline with the accepted volatile VFS '/tmp/<basename>' stdout sink policy.
The first form redirects only the consumer fd1 to a child-only volatile regular
file while the producer still feeds the pipe. The second form redirects only
the producer fd1 to a child-only volatile regular file, so the downstream
consumer receives deterministic pipe EOF/no-data.

This closeout does not implement code, run Pi 5 hardware, acquire
hardwareTestLock, broaden pipeline grammar, add multi-stage or concurrent
pipelines, add process accounting/concurrency, add arbitrary descriptor syntax,
add persistent storage, add recursive directories, add arbitrary input paths,
add networking/SSH, or make a phase transition.

## Findings

- fixed: Consolidated the consumer-output composition evidence showing
  producer fd1 as 'pipe-endpoint', consumer fd0 as 'pipe-endpoint', consumer
  fd1 as 'regular-file', the child-only sink route
  'volatile-vfs:/tmp/pipe-consumer.txt', pipe bytes written/read at 0x1f, and
  'source=shell-pipe-consumer-stdout-redirection'.
- fixed: Consolidated descriptor-backed readback of
  '/tmp/pipe-consumer.txt', including the 0x44-byte userspace stdin report,
  consuming 'waitpid', non-consuming 'laststatus', errors=0, final
  classification, and PASS.
- fixed: Consolidated the producer file-redirection-away evidence showing the
  producer pipe endpoint installation being superseded by child-only fd1
  redirection to 'volatile-vfs:/tmp/pipe-source.txt'. The retained transcript
  records bytes written/read at zero, reader EOF, shell restoration,
  'source=shell-pipe-producer-file-redirection-away', and the consumer
  'pipe-eof/no-data' result.
- fixed: Consolidated descriptor-backed readback of
  '/tmp/pipe-source.txt', proving the producer's 0x1f-byte stdout fixture
  reached the volatile VFS file instead of the pipe.
- fixed: Confirmed deterministic negatives reject consumer append redirection,
  stderr-producer consumer-output redirection, producer append redirection,
  stderr producer file redirection, and producer plus consumer file redirection
  outside the accepted exact forms.
- fixed: Retained plain stdout-to-stdin pipeline, stderr-not-piped pipeline,
  descriptor-mixing pipeline, arbitrary '/tmp' output redirection,
  descriptor-backed cat, waitpid, and laststatus controls with
  PASS/classification markers.
- not-an-issue: The consumer-output core's initial overlong negative command
  and the producer-redirection core's initial generic-classification harness
  wiring were corrected before acceptance; the retained evidence for both
  tasks contains the intended feature proof and PASS marker.
- not-an-issue: No pipeline/redirection ordering ADR is required. The accepted
  behavior is the existing child-only redirection policy composed with the
  exact two-stage pipe setup: consumer redirection changes the consumer sink,
  and producer redirection overrides the producer pipe writer for fd1.
- not-an-issue: Pi 5 hardware proof remains outside this QEMU/substitute
  feature slice; hardwareTestLock stayed unlocked/restored and unused.
- deferred: multi-stage/concurrent pipelines, pipefail, jobs, fork/signals,
  arbitrary descriptor syntax, descriptor moves, consumer append/stderr
  redirection, producer append/stderr redirection, producer and consumer file
  redirection in the same pipeline, persistent filesystem behavior, recursive
  directories, process accounting/concurrency, Pi 5 proof, networking, SSH,
  and phase transition.

## Evidence Map

- consumer-output primary evidence:
  'tasks/evidence/2026-06-05-phase10-pipeline-consumer-output-redirection-core/qemu-local-shell-pipeline-consumer-output-redirection-smoke.log'
  records 'exec stdout | exec stdin >/tmp/pipe-consumer.txt'.
- consumer-output pipe and sink evidence:
  the primary consumer-output log records producer
  'stream=pipe-writer route=pipe:stdout-to-stdin', consumer
  'fd0=pipe-endpoint fd1=regular-file', the child-only sink route
  'volatile-vfs:/tmp/pipe-consumer.txt', bytes written/read at 0x1f, writer
  closure, reader EOF, shell restoration, and
  'source=shell-pipe-consumer-stdout-redirection'.
- consumer-output readback and lifecycle evidence:
  the primary consumer-output log records 'cat /tmp/pipe-consumer.txt'
  reading the 0x44-byte consumer report, consuming 'waitpid', non-consuming
  'laststatus', deterministic negatives, errors=0,
  'classification=qemu-local-shell-pipeline-consumer-output-redirection-complete',
  and PASS.
- producer file-redirection-away primary evidence:
  'tasks/evidence/2026-06-05-phase10-pipeline-producer-file-redirection-away-core/qemu-local-shell-pipeline-producer-file-redirection-away-smoke.log'
  records 'exec stdout >/tmp/pipe-source.txt | exec stdin'.
- producer redirection and EOF evidence:
  the primary producer-redirection log records producer fd1 as
  'regular-file', the child-only sink route
  'volatile-vfs:/tmp/pipe-source.txt', stdout writing to
  'stream=regular-file route=volatile-vfs:/tmp/pipe-source.txt', bytes
  written/read at zero, reader EOF, shell restoration,
  'source=shell-pipe-producer-file-redirection-away', and consumer
  'pipe-eof/no-data'.
- producer file readback and lifecycle evidence:
  the primary producer-redirection log records 'cat /tmp/pipe-source.txt'
  reading the redirected 0x1f-byte stdout fixture, consuming 'waitpid',
  non-consuming 'laststatus', deterministic negatives, errors=0,
  'classification=qemu-local-shell-pipeline-producer-file-redirection-away-complete',
  and PASS.
- retained minimal pipeline evidence:
  'tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log'.
- retained pipeline stderr and descriptor-mixing evidence:
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

- exact two-stage consumer-output form
  'exec stdout | exec stdin >/tmp/pipe-consumer.txt';
- exact two-stage producer-output-away form
  'exec stdout >/tmp/pipe-source.txt | exec stdin';
- child-only consumer fd1 volatile VFS redirection while producer fd1 remains
  the pipe writer;
- child-only producer fd1 volatile VFS redirection overriding the producer pipe
  writer so the consumer observes pipe EOF/no-data;
- descriptor-backed readback of both volatile output files;
- waitpid, laststatus, lifecycle/status, VFS exec/open/read/write,
  descriptor restoration, pipe endpoint closure, minimal pipeline,
  descriptor-mixing pipeline, arbitrary '/tmp' output redirection, and
  descriptor-backed cat controls.

Deferred:

- multi-stage or concurrent pipelines, pipefail, jobs, async execution, fork,
  signals, process accounting/concurrency, and job control;
- arbitrary descriptor syntax, descriptor moves, here-docs, broader shell
  grammar, quoting, variables, globbing, and command substitution;
- consumer append/stderr redirection, producer append/stderr redirection, and
  producer plus consumer file redirection in the same pipeline;
- persistent filesystem behavior, recursive directories, traversal, broad
  writable filesystem mutation, and path persistence claims;
- Pi 5 proof, networking, SSH, and any phase transition.

## Next Step

No explicit queued follow-up task should be promoted from this closeout. The
next boundary is supervisor planning for either minimal process
accounting/concurrency or another explicitly justified shell feature.

## Validation

- static inspection: accepted pipeline consumer-output QEMU/substitute
  evidence was inspected, including producer pipe writer records, consumer
  pipe reader and volatile VFS fd1 sink records, descriptor-backed file
  readback, waitpid, laststatus, deterministic negatives, completion marker,
  errors=0, and PASS.
- static inspection: accepted pipeline producer file-redirection-away
  QEMU/substitute evidence was inspected, including producer volatile VFS fd1
  sink records, zero pipe bytes, consumer pipe EOF/no-data, descriptor-backed
  file readback, waitpid, laststatus, deterministic negatives, completion
  marker, errors=0, and PASS.
- static inspection: retained minimal pipeline, stderr-not-piped pipeline,
  descriptor-mixing pipeline, arbitrary '/tmp' output redirection,
  descriptor-backed cat, waitpid, and laststatus controls were checked for
  PASS/classification markers.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final pipeline file-redirection frontier closeout commit recorded in
supervisor state.
