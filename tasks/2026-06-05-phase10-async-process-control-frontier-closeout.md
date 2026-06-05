# Phase 10 Async Process-Control Frontier Closeout

Task: phase10-async-process-control-frontier-closeout-20260605
Status: accepted

## Scope

Close out the accepted async/background process-control frontier without adding
runtime behavior.

Accepted shell-visible forms:

- 'exec /bin/status42 &'
- 'jobs'

This closeout reconciles the accepted background VFS exec lifecycle and
jobs/accounting list slices with retained VFS exec, descriptor inheritance,
waitpid, laststatus, pipeline, redirection, and descriptor-backed file controls.
It does not add POSIX job control, multiple jobs, signals, process groups,
sessions, terminal ownership, true scheduler-concurrent userspace execution,
background pipelines/redirections, Pi 5 hardware proof, networking, SSH, or a
phase transition.

## Findings

- fixed: Consolidated the accepted background launch evidence. The retained
  transcript records 'exec /bin/status42 &' through VFS/open/read, loader,
  launch, descriptor inheritance, startup ABI, and background accounting with
  'source=vfs-open-read mode=background'.
- fixed: Consolidated the shell-owned background lifecycle record. The accepted
  evidence reports a stable job id, pid, command '/bin/status42',
  'state=running', 'status=pending', 'shell-responsive=true', then completion
  with status '0x2a', matching observed status, and 'reaped=true'.
- fixed: Consolidated the accepted 'jobs' inspection surface. It reports
  'jobs none' before launch, the running record after launch, and the completed
  record on the next inspection without claiming a broad process table or
  procfs contract.
- fixed: Confirmed background accounting does not create or consume foreground
  lifecycle records: foreground 'waitpid' reports 'no-child' and 'laststatus'
  reports 'last-process none' after background observation, while a later
  foreground 'exec /bin/zero' retains the accepted waitpid/laststatus controls.
- fixed: Confirmed retained controls still cover VFS exec/open/read/write,
  descriptor inheritance and restoration, descriptor-backed cat, plain
  stdout-to-stdin pipeline transfer, pipeline/file redirection composition,
  deterministic async and job-control negatives, errors=0, final
  classifications, and PASS.
- fixed: Confirmed roadmap language does not imply POSIX job control,
  scheduling fairness, process groups, signal delivery, terminal ownership,
  multiple jobs, Pi 5 proof, networking, SSH, or a phase transition.
- not-an-issue: No ADR correction is required for this closeout. The accepted
  contract remains a narrow shell-owned process-accounting boundary layered on
  existing VFS exec and lifecycle evidence.
- not-an-issue: Pi 5 hardware proof remains outside this QEMU/substitute
  frontier closeout; hardwareTestLock stayed unlocked/restored and unused.
- deferred: multiple background jobs, stale-entry policy beyond the accepted
  single record, kill/fg/bg/disown, process groups, sessions, terminal
  ownership, signals, fork, true scheduler-concurrent userspace execution,
  background pipelines/redirections, pipefail, process-tree/procfs inspection,
  scheduling fairness proof, Pi 5 proof, networking, SSH, and phase transition.

## Evidence Map

- background VFS exec lifecycle core:
  'tasks/evidence/2026-06-05-phase10-background-vfs-exec-lifecycle-core/qemu-local-shell-background-vfs-exec-lifecycle-smoke.log'
  records the accepted 'exec /bin/status42 &' launch, VFS exec provenance,
  background job running/completed records, shell responsiveness through
  descriptor-backed 'cat /etc/banner.txt', foreground waitpid/laststatus
  isolation, foreground '/bin/zero' controls, deterministic negatives,
  errors=0,
  'classification=qemu-local-shell-background-vfs-exec-lifecycle-complete',
  and PASS. This evidence was committed with
  2a2b99437804c4cc75e495871353e24fefc0a04f.
- background lifecycle closeout:
  'tasks/2026-06-05-phase10-background-vfs-exec-lifecycle-closeout.md'
  reconciles the same accepted background lifecycle boundary and was committed
  with 5001e19ba083a9802ea2981b3ac91da72323b5fe.
- jobs/accounting list core:
  'tasks/evidence/2026-06-05-phase10-jobs-accounting-list-core/qemu-local-shell-jobs-accounting-list-smoke.log'
  records 'jobs none', 'exec /bin/status42 &' through the accepted VFS exec
  path, running and completed jobs records, foreground waitpid/laststatus
  isolation, foreground '/bin/zero' controls, retained pipeline/cat controls,
  deterministic 'fg', 'bg', and 'kill %1' negatives, errors=0,
  'classification=qemu-local-shell-jobs-accounting-list-complete', and PASS.
  This evidence was committed with
  c06926cf8bc9533ff7ec76a5579fbb6e64c480cd.
- jobs/accounting list closeout:
  'tasks/2026-06-05-phase10-jobs-accounting-list-closeout.md' reconciles the
  accepted minimal accounting inspection surface and was committed with
  59afb7c4fcceaed428310a86ece7e02770df6b25.
- retained control inspections:
  'tasks/evidence/2026-06-05-phase10-background-vfs-exec-lifecycle-core/retained-control-inspection.txt'
  and
  'tasks/evidence/2026-06-05-phase10-jobs-accounting-list-core/retained-control-inspection.txt'
  retain PASS/classification markers for the accepted async/jobs evidence and
  historical pipeline, file redirection, descriptor-backed cat, waitpid, and
  laststatus controls.
- pipeline/file-redirection frontier control:
  'tasks/2026-06-05-phase10-pipeline-file-redirection-frontier-closeout.md'
  records the accepted file-redirection and pipeline composition frontier
  committed with 6740d1bb1ff247fcccb7c4b2bdde6943820b7d25.

## Accepted Frontier

Accepted:

- exact trailing ampersand form 'exec /bin/status42 &' through the accepted
  fixed '/bin' VFS exec path;
- one shell-owned background accounting record with stable job id, pid,
  command label, running/completed state, pending/completed status, observed
  status, and reaped flag;
- shell responsiveness after the background launch at the command-loop
  boundary;
- 'jobs' as a minimal inspection command for the single accepted background
  accounting record;
- foreground waitpid/laststatus isolation after background observation;
- normal foreground VFS exec waitpid/laststatus control after the async/jobs
  slice;
- deterministic unsupported async/job-control negatives;
- retained VFS exec/open/read/write, descriptor inheritance/restoration,
  pipeline, file redirection, descriptor-backed cat, waitpid, and laststatus
  controls.

Deferred:

- multiple jobs and stale-entry policy beyond the single accepted record;
- kill/fg/bg/disown, process groups, sessions, terminal ownership, signals,
  fork, process-tree/procfs inspection, broad POSIX job control, and scheduling
  fairness proof;
- true scheduler-concurrent userspace execution, background
  pipelines/redirections, pipefail, and multi-stage/concurrent pipeline
  scheduling;
- Pi 5 hardware proof, networking, SSH, and any phase transition.

## Next Step

Supervisor planning is required before any further process-control, local
storage, Pi 5 proof, networking, SSH, or phase-transition work. The worker must
not infer the next feature from this frontier closeout.

## Validation

- static inspection: accepted background VFS exec lifecycle QEMU/substitute
  evidence was inspected for VFS exec provenance, running/completed background
  records, shell responsiveness, completed/reaped status 0x2a, foreground
  waitpid and laststatus isolation, foreground '/bin/zero' controls,
  deterministic negatives, completion marker, errors=0, and PASS.
- static inspection: accepted jobs/accounting list QEMU/substitute evidence
  was inspected for 'jobs none', running and completed jobs records, stable
  id/pid/command fields, foreground waitpid/laststatus isolation, foreground
  '/bin/zero' controls, deterministic job-control negatives, completion
  marker, errors=0, and PASS.
- static inspection: retained control evidence was inspected for pipeline/file
  redirection, descriptor inheritance, descriptor-backed cat, VFS
  exec/open/read/write, waitpid, laststatus, PASS/classification markers, and
  accepted commit references.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final async process-control frontier closeout commit recorded in
supervisor state.
