# Phase 12 Local Pipeline Output Append Regular-File Redirection Frontier Checkpoint

Task: phase12-local-pipeline-output-append-regular-file-redirection-frontier-checkpoint-20260627

Status: accepted; commit recorded in supervisor state.

## Scope

- Reviewed the accepted direct and fixed-/bin bare-name pipeline-output append
  regular-file redirection cores, closeouts, classifications, evidence maps,
  QEMU/substitute transcripts, retained regression controls, and project docs.
- Reconciled the exact accepted direct truncate-then-append witnesses
  '/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt' and
  '/bin/stdout | /bin/stdin >>/tmp/pipeline-report.txt' with the exact
  fixed-/bin bare-name witnesses 'stdout | stdin >/tmp/pipeline-report.txt'
  and 'stdout | stdin >>/tmp/pipeline-report.txt'.
- Kept the accepted surface limited to those four exact pipeline-output append
  regular-file witnesses. There is no later queued same-lane local POSIX/VFS
  task after this checkpoint, so supervisor planning is required before another
  worker promotion.
- Did not implement runtime feature changes, stderr pipeline redirection or
  append, input/combined pipeline redirections, arbitrary paths, persistent
  writable filesystem behavior, live networking/SSH, Pi 5 hardware action,
  generated-root retry, or a phase transition.

## Findings

- not-an-issue: the accepted direct witnesses are exactly
  '/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt' followed by
  '/bin/stdout | /bin/stdin >>/tmp/pipeline-report.txt'. Retained evidence
  records descriptor-backed VFS loading of both stages in both commands,
  accepted userspace launch/status, fd1 from the producer to the pipe endpoint,
  fd0 from that pipe endpoint on the consumer, child-only consumer fd1
  redirection to volatile-vfs:/tmp/pipeline-report.txt, first-command
  truncate/sink semantics, second-command append-at-EOF semantics,
  descriptor-backed readback of two userspace stdin reports in order, shell fd1
  restoration, and coherent process-status observations.
- not-an-issue: the accepted fixed-/bin bare-name witnesses are exactly
  'stdout | stdin >/tmp/pipeline-report.txt' followed by
  'stdout | stdin >>/tmp/pipeline-report.txt'. Retained evidence records
  bounded resolution only to '/bin/stdout' and '/bin/stdin' before using the
  same VFS/userspace launch, pipe handoff, final-stage fd1 sink/append,
  readback, shell restoration, and status paths as the direct witnesses.
- not-an-issue: retained fail-closed evidence keeps unsupported direct and
  bare-name variants outside the accepted surface: alternate output targets,
  unsupported command names, path-containing consumer names, wrong final-stage
  programs, explicit '1>', spaced output grammar, malformed append grammar,
  stderr forms, input redirection on pipelines, arbitrary output paths, and
  persistent-storage claims.
- fixed: roadmap, Phase 12, and early POSIX docs now record the
  pipeline-output append regular-file redirection frontier checkpoint and that
  no later queued same-lane local POSIX/VFS task is mechanically objective.
- deferred: stderr pipeline redirection and append, input/combined pipeline
  redirections, arbitrary paths, persistent writable filesystem behavior,
  environment-backed PATH, current-directory search, command lookup beyond
  bounded /bin, arbitrary shell grammar, unbounded or concurrent pipelines,
  scheduler concurrency, fork/signals, process groups/sessions, live
  networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
  transition remain outside this checkpoint.

## Evidence

- static inspection:
  tasks/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-core.md,
  tasks/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-closeout.md,
  tasks/2026-06-27-phase12-local-bare-name-pipeline-output-append-regular-file-redirection-core.md,
  and
  tasks/2026-06-27-phase12-local-bare-name-pipeline-output-append-regular-file-redirection-closeout.md.
- task-owned JSON inspection:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-core/classification.json,
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-core/evidence-map.json,
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-closeout/classification.json,
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-closeout/evidence-map.json,
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-append-regular-file-redirection-core/classification.json,
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-append-regular-file-redirection-core/evidence-map.json,
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-append-regular-file-redirection-closeout/classification.json,
  and
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-append-regular-file-redirection-closeout/evidence-map.json.
- QEMU/substitute transcript inspection:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-append-regular-file-redirection-core/qemu-local-shell-direct-pipeline-output-append-regular-file-redirection-smoke.log,
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-append-regular-file-redirection-core/qemu-local-shell-bare-name-pipeline-output-append-regular-file-redirection-smoke.log,
  and retained regression smoke summaries for the direct and bare-name append
  cores.
- project doc inspection: docs/src/roadmap.md,
  docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.
- checkpoint classification:
  tasks/evidence/2026-06-27-phase12-local-pipeline-output-append-regular-file-redirection-frontier-checkpoint/classification.json.
- checkpoint evidence map:
  tasks/evidence/2026-06-27-phase12-local-pipeline-output-append-regular-file-redirection-frontier-checkpoint/evidence-map.json.

## Validation

- 'jq empty' over task-owned direct append, bare-name append, and checkpoint
  JSON evidence passed.
- 'git diff --check' passed.
- '/home/node/.cargo/bin/mdbook build' passed.
- 'git diff --cached --check' passed before commit.

## Result

The accepted pipeline-output append regular-file redirection frontier is
limited to:

~~~text
/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt
/bin/stdout | /bin/stdin >>/tmp/pipeline-report.txt
stdout | stdin >/tmp/pipeline-report.txt
stdout | stdin >>/tmp/pipeline-report.txt
~~~

The direct form and fixed-/bin bare-name form both use descriptor-backed VFS
program loading, accepted userspace launch/status, producer fd1 to the pipe
endpoint, consumer fd0 from the pipe endpoint, child-only consumer fd1 to
volatile-vfs:/tmp/pipeline-report.txt, first-command truncate/sink semantics,
second-command append-at-EOF semantics, descriptor-backed readback of two
userspace stdin reports in order, and shell fd1 restoration. Unsupported direct
and bare-name variants remain fail-closed.

selected_next_task: null.
planningNeeded: true because no later queued same-lane local POSIX/VFS task has
complete objective dependencies, acceptance criteria, validation gates, docs
requirements, and evidence requirements.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.
