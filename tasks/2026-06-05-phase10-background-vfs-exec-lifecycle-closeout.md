# Phase 10 Background VFS Exec Lifecycle Closeout

Task: phase10-background-vfs-exec-lifecycle-closeout-20260605
Status: accepted
Accepted core commit: 2a2b99437804c4cc75e495871353e24fefc0a04f

## Scope

Close out the accepted background VFS exec lifecycle core without adding code.

Accepted shell-visible form:

- 'exec /bin/status42 &'

This closeout reconciles the accepted evidence and documentation for the
single exact background VFS exec/accounting boundary. It does not implement
new runtime behavior, run Pi 5 hardware, acquire hardwareTestLock, add a jobs
command, add foreground/background job control, add process groups or signals,
broaden async syntax, add background pipelines/redirections, add true
scheduler-concurrent userspace execution, add networking/SSH, or make a phase
transition.

## Findings

- fixed: Consolidated the primary background launch evidence. The accepted
  transcript records 'exec /bin/status42 &',
  'source=vfs-open-read mode=background', the normal VFS exec
  source/loader/launch/descriptors/startup ABI records, and a stable
  background job id/pid/command record with 'state=running',
  'status=pending', and 'shell-responsive=true'.
- fixed: Consolidated the shell-responsiveness evidence. The command loop
  accepts and completes 'cat /etc/banner.txt' immediately after the background
  launch, while first reporting the background job as completed with
  'status=0x2a', 'observed-status=0x2a', and 'reaped=true'.
- fixed: Consolidated foreground lifecycle isolation evidence. After the
  background completion, foreground 'waitpid' reports 'no-child' and
  'laststatus' reports 'last-process none'; a later foreground
  'exec /bin/zero' still produces normal consuming 'waitpid' and
  non-consuming 'laststatus' records.
- fixed: Confirmed deterministic negatives for unsupported async syntax:
  'exec /bin/status42&' and 'exec stdout &' remain rejected without shrinking
  the accepted foreground VFS exec, pipeline, redirection, stdio, waitpid,
  laststatus, or descriptor-backed cat behavior.
- fixed: Confirmed roadmap language names the accepted boundary precisely and
  keeps true scheduler-concurrent user processes, multiple background jobs,
  jobs/fg/bg, process groups, signals, background pipelines/redirections,
  arbitrary async syntax, broad process table policy, Pi 5 proof, networking,
  SSH, and phase transition deferred.
- not-an-issue: No ADR correction is required for this closeout. The accepted
  behavior is intentionally a narrow shell-owned accounting surface layered on
  the existing VFS exec and lifecycle records, and the broader POSIX job
  control policy remains explicitly deferred.
- not-an-issue: Pi 5 hardware proof remains outside this QEMU/substitute
  feature slice; hardwareTestLock stayed unlocked/restored and unused.
- deferred: jobs/accounting list inspection, multiple background jobs,
  kill/fg/bg/disown, process groups, sessions, terminal ownership, signals,
  fork, true async scheduler concurrency, background pipelines/redirections,
  pipefail, Pi 5 proof, networking, SSH, and phase transition.

## Evidence Map

- primary background lifecycle evidence:
  'tasks/evidence/2026-06-05-phase10-background-vfs-exec-lifecycle-core/qemu-local-shell-background-vfs-exec-lifecycle-smoke.log'
  records the accepted 'exec /bin/status42 &' form, VFS exec provenance,
  background job accounting, shell responsiveness, foreground waitpid and
  laststatus isolation, foreground '/bin/zero' controls, deterministic
  negatives, errors=0,
  'classification=qemu-local-shell-background-vfs-exec-lifecycle-complete',
  and PASS. This evidence was committed with
  2a2b99437804c4cc75e495871353e24fefc0a04f.
- retained control inspection:
  'tasks/evidence/2026-06-05-phase10-background-vfs-exec-lifecycle-core/retained-control-inspection.txt'
  records PASS/classification markers for accepted pipeline/file redirection,
  waitpid, laststatus, stdio, VFS exec/open/read/write, descriptor-backed cat,
  and descriptor restoration controls.
- retained combined stdin/stdout redirection evidence:
  'tasks/evidence/2026-06-05-phase10-combined-stdin-stdout-redirection-core/qemu-local-shell-combined-stdin-stdout-redirection-smoke.log'.
- retained pipeline consumer-output evidence:
  'tasks/evidence/2026-06-05-phase10-pipeline-consumer-output-redirection-core/qemu-local-shell-pipeline-consumer-output-redirection-smoke.log'.
- retained pipeline producer-redirection-away evidence:
  'tasks/evidence/2026-06-05-phase10-pipeline-producer-file-redirection-away-core/qemu-local-shell-pipeline-producer-file-redirection-away-smoke.log'.
- retained minimal pipeline evidence:
  'tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log'.
- retained stdout regular-file redirection evidence:
  'tasks/evidence/2026-06-04-phase10-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log'.

## Accepted Boundary

Accepted:

- exact trailing ampersand form 'exec /bin/status42 &';
- VFS-backed '/bin/status42' launch using the accepted open/read, loader,
  launch, descriptor inheritance, startup ABI, and lifecycle/status lineage;
- one shell-owned background job accounting record with stable job id, pid,
  command path, running/pending launch state, completed status 0x2a, observed
  status 0x2a, and reaped=true on observation;
- command-loop responsiveness after the background launch, proven by a
  descriptor-backed 'cat /etc/banner.txt' command;
- foreground lifecycle isolation for 'waitpid' and 'laststatus' after the
  background completion;
- foreground VFS exec lifecycle control through 'exec /bin/zero';
- retained pipeline, file redirection, descriptor inheritance, stdio,
  waitpid, laststatus, negative, and descriptor-backed cat controls.

Deferred:

- jobs/accounting list surface and multi-entry job table behavior;
- multiple background jobs, job control commands, kill/fg/bg/disown, process
  groups, sessions, terminal ownership, and signals;
- fork and true scheduler-concurrent userspace processes;
- background pipelines/redirections, broader async grammar, pipefail, and
  multi-stage/concurrent pipeline scheduling;
- Pi 5 hardware proof, networking, SSH, and any phase transition.

## Next Step

The next mechanically unblocked task is
'phase10-jobs-accounting-list-core-20260605'. Keep it limited to a minimal
shell-visible accounting inspection surface for the accepted background job
record; do not broaden into POSIX job control, signals, process groups,
sessions, or hardware proof.

## Validation

- static inspection: accepted background QEMU/substitute evidence was
  inspected, including VFS exec provenance, job id/pid/accounting fields,
  shell responsiveness, completed/reaped status 0x2a, foreground waitpid and
  laststatus isolation, foreground '/bin/zero' controls, deterministic
  negatives, completion marker, errors=0, and PASS.
- static inspection: retained control evidence was inspected for
  pipeline/file redirection, waitpid, laststatus, stdio, VFS
  exec/open/read/write, descriptor restoration, descriptor-backed cat,
  PASS/classification markers, and accepted commit references.
- diff check: 'git diff --check' passed.
- docs: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff check: 'git diff --cached --check' passed before commit.

hardwareTestLock remained unlocked/restored and unused.

## Commit

Commit: final background VFS exec lifecycle closeout commit recorded in
supervisor state.
