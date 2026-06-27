# Phase 12 Local Pipeline Output Regular-File Redirection Frontier Checkpoint

Task: phase12-local-pipeline-output-regular-file-redirection-frontier-checkpoint-20260627

Status: accepted; commit recorded in supervisor state.

## Scope

- Reviewed the accepted direct and fixed-/bin bare-name pipeline-output
  regular-file redirection tasks, closeouts, classifications, evidence maps,
  QEMU/substitute transcript, retained regression controls, and project docs.
- Reconciled the exact accepted direct witness
  '/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt' and fixed-/bin
  bare-name witness 'stdout | stdin >/tmp/pipeline-report.txt' against
  retained pipeline, redirection, process-status, ps, pipestatus, and
  cat-banner controls.
- Kept the accepted surface limited to the two exact pipeline-output
  regular-file witnesses. There is no later queued same-lane local POSIX/VFS
  task after this checkpoint, so supervisor planning is required before
  another worker promotion.
- Did not implement runtime feature changes, append pipeline-output
  redirection, stderr forms, arbitrary paths, persistent writable filesystem
  behavior, live networking/SSH, Pi 5 hardware action, generated-root retry, or
  a phase transition.

## Findings

- not-an-issue: the accepted direct witness is exactly
  '/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt'. Retained evidence
  records descriptor-backed VFS launch of both stages, fd1 from the producer to
  the pipe endpoint, fd0 from that pipe endpoint on the consumer, child-only
  fd1 redirection to volatile-vfs:/tmp/pipeline-report.txt, loader temporary
  descriptor closure, userspace report write through redirected fd1,
  descriptor-backed 'cat /tmp/pipeline-report.txt' readback, shell fd1
  restoration, and coherent lifecycle/status observations.
- not-an-issue: the accepted fixed-/bin bare-name witness is exactly
  'stdout | stdin >/tmp/pipeline-report.txt'. Retained evidence records
  bounded resolution only to '/bin/stdout' and '/bin/stdin' before using the
  same VFS/userspace launch, pipe handoff, final-stage fd1 sink, readback, and
  shell restoration path as the direct witness.
- not-an-issue: retained fail-closed evidence keeps unsupported direct and
  bare-name forms outside the accepted surface: alternate output targets,
  append syntax, wrong final-stage programs, explicit '1>', spaced output
  grammar, consumer names with path separators, and neighboring malformed
  pipeline redirections.
- fixed: roadmap, Phase 12, and early POSIX docs now record the
  pipeline-output regular-file redirection frontier checkpoint and that no
  later queued same-lane local POSIX/VFS task is mechanically objective.
- deferred: append pipeline-output redirection, stderr forms, input/combined
  pipeline redirections, arbitrary paths, persistent writable filesystem
  behavior, environment-backed PATH, current-directory search, command lookup
  beyond bounded /bin, arbitrary shell grammar, unbounded or concurrent
  pipelines, scheduler concurrency, fork/signals, process groups/sessions,
  live networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
  transition remain outside this checkpoint.

## Evidence

- static inspection:
  tasks/2026-06-27-phase12-local-direct-pipeline-output-regular-file-redirection-core.md,
  tasks/2026-06-27-phase12-local-direct-pipeline-output-regular-file-redirection-closeout.md,
  tasks/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-core.md,
  and
  tasks/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-closeout.md.
- task-owned JSON inspection:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-regular-file-redirection-core/classification.json,
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-regular-file-redirection-core/evidence-map.json,
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-regular-file-redirection-closeout/classification.json,
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-regular-file-redirection-closeout/evidence-map.json,
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-core/classification.json,
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-core/evidence-map.json,
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-closeout/classification.json,
  and
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-closeout/evidence-map.json.
- QEMU/substitute transcript inspection:
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-core/qemu-substitute-focused-test.log.
- project doc inspection: docs/src/roadmap.md,
  docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.
- checkpoint evidence map:
  tasks/evidence/2026-06-27-phase12-local-pipeline-output-regular-file-redirection-frontier-checkpoint/evidence-map.json.

## Validation

- 'jq empty
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-regular-file-redirection-core/classification.json
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-regular-file-redirection-core/evidence-map.json
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-regular-file-redirection-closeout/classification.json
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-output-regular-file-redirection-closeout/evidence-map.json
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-core/classification.json
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-core/evidence-map.json
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-closeout/classification.json
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-output-regular-file-redirection-closeout/evidence-map.json
  tasks/evidence/2026-06-27-phase12-local-pipeline-output-regular-file-redirection-frontier-checkpoint/evidence-map.json'
  passed.
- 'git diff --check' passed.
- '/home/node/.cargo/bin/mdbook build' passed.
- 'git diff --cached --check' passed.

## Result

The accepted pipeline-output regular-file redirection frontier is limited to:

~~~text
/bin/stdout | /bin/stdin >/tmp/pipeline-report.txt
stdout | stdin >/tmp/pipeline-report.txt
~~~

The direct form and fixed-/bin bare-name form both use descriptor-backed VFS
program loading, accepted userspace launch/status, producer fd1 to the pipe
endpoint, consumer fd0 from the pipe endpoint, child-only consumer fd1 to
volatile-vfs:/tmp/pipeline-report.txt, userspace report writing through
redirected fd1, descriptor-backed readback, and shell fd1 restoration.
Unsupported direct and bare-name variants remain fail-closed.

selected_next_task: null.
planningNeeded: true because no later queued same-lane local POSIX/VFS task
has complete objective dependencies, acceptance criteria, validation gates,
docs requirements, and evidence requirements.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.
