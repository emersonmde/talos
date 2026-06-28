# Phase 12 Local Pipeline Stderr Regular-File Redirection Frontier Checkpoint

Task: phase12-local-pipeline-stderr-regular-file-redirection-frontier-checkpoint-20260627

Status: accepted; commit recorded in supervisor state.

## Scope

- Reviewed the accepted direct and fixed-/bin bare-name pipeline stderr
  regular-file redirection cores, closeouts, classifications, evidence maps,
  QEMU/substitute transcripts, retained regression controls, and project docs.
- Reconciled the exact accepted direct witness
  '/bin/stdout | /bin/stderr 2>/tmp/pipeline-stderr.txt' with the exact
  fixed-/bin bare-name witness 'stdout | stderr 2>/tmp/pipeline-stderr.txt'.
- Kept the accepted surface limited to those two exact pipeline stderr
  regular-file witnesses. There is no later queued same-lane local POSIX/VFS
  task after this checkpoint, so supervisor planning is required before another
  worker promotion.
- Did not implement runtime feature changes, append pipeline stderr
  redirection, input/combined pipeline redirections, arbitrary paths,
  persistent writable filesystem behavior, live networking/SSH, Pi 5 hardware
  action, generated-root retry, or a phase transition.

## Findings

- not-an-issue: the accepted direct witness is exactly
  '/bin/stdout | /bin/stderr 2>/tmp/pipeline-stderr.txt'. Retained evidence
  records descriptor-backed VFS loading of both stages, accepted userspace
  launch/status, producer fd1 to the pipe endpoint, final-stage consumer fd0
  from that pipe endpoint, child-only consumer fd2 redirection to
  volatile-vfs:/tmp/pipeline-stderr.txt, sink/truncate semantics,
  descriptor-backed readback of the 0x1f-byte stderr fixture, shell fd2
  restoration, and coherent process-status observations.
- not-an-issue: the accepted fixed-/bin bare-name witness is exactly
  'stdout | stderr 2>/tmp/pipeline-stderr.txt'. Retained evidence records
  bounded resolution only to '/bin/stdout' and '/bin/stderr' before using the
  same VFS/userspace launch, pipe handoff, final-stage fd2 sink/truncate,
  readback, shell restoration, and status paths as the direct witness.
- not-an-issue: the stderr fixture intentionally writes to stderr and does not
  consume stdin. Retained evidence therefore keeps producer bytes-written=0x1f,
  consumer pipe bytes-read=0, and reader-eof=false while still proving the
  final stage inherited fd0 from the pipe endpoint.
- not-an-issue: retained fail-closed evidence keeps unsupported direct and
  bare-name variants outside the accepted surface: append 2>>, unsupported
  command names, path-containing stage names in the bare-name witness, stdout
  final-stage redirection, input redirection on pipelines, combined pipeline
  redirections, malformed spacing/grammar, alternate or arbitrary paths,
  persistent-storage claims, PATH/current-directory lookup, and command lookup
  beyond bounded /bin.
- fixed: roadmap, Phase 12, and early POSIX docs now record the pipeline
  stderr regular-file redirection frontier checkpoint and that no later queued
  same-lane local POSIX/VFS task is mechanically objective.
- deferred: append pipeline stderr redirection, input/combined pipeline
  redirections, arbitrary paths, persistent writable filesystem behavior,
  environment-backed PATH, current-directory search, command lookup beyond
  bounded /bin, arbitrary shell grammar, unbounded or concurrent pipelines,
  scheduler concurrency, fork/signals, process groups/sessions, live
  networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
  transition remain outside this checkpoint.

## Evidence

- static inspection:
  tasks/2026-06-27-phase12-local-direct-pipeline-stderr-regular-file-redirection-core.md,
  tasks/2026-06-27-phase12-local-direct-pipeline-stderr-regular-file-redirection-closeout.md,
  tasks/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-core.md,
  and
  tasks/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-closeout.md.
- task-owned JSON inspection:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stderr-regular-file-redirection-core/classification.json,
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stderr-regular-file-redirection-core/evidence-map.json,
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stderr-regular-file-redirection-closeout/classification.json,
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stderr-regular-file-redirection-closeout/evidence-map.json,
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-core/classification.json,
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-core/evidence-map.json,
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-closeout/classification.json,
  and
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-closeout/evidence-map.json.
- QEMU/substitute transcript inspection:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stderr-regular-file-redirection-core/qemu-local-shell-direct-pipeline-stderr-regular-file-redirection-smoke.log,
  tasks/evidence/2026-06-27-phase12-local-bare-name-pipeline-stderr-regular-file-redirection-core/qemu-local-shell-bare-name-pipeline-stderr-regular-file-redirection-smoke.log,
  and retained regression smoke summaries for the direct and bare-name stderr
  cores.
- project doc inspection: docs/src/roadmap.md,
  docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.
- checkpoint classification:
  tasks/evidence/2026-06-27-phase12-local-pipeline-stderr-regular-file-redirection-frontier-checkpoint/classification.json.
- checkpoint evidence map:
  tasks/evidence/2026-06-27-phase12-local-pipeline-stderr-regular-file-redirection-frontier-checkpoint/evidence-map.json.

## Validation

- 'jq empty' over task-owned direct pipeline stderr, bare-name pipeline stderr,
  and checkpoint JSON evidence passed.
- 'git diff --check' passed.
- '/home/node/.cargo/bin/mdbook build' passed.
- 'git diff --cached --check' passed before commit.

## Result

The accepted pipeline stderr regular-file redirection frontier is limited to:

~~~text
/bin/stdout | /bin/stderr 2>/tmp/pipeline-stderr.txt
stdout | stderr 2>/tmp/pipeline-stderr.txt
~~~

The direct form and fixed-/bin bare-name form both use descriptor-backed VFS
program loading, accepted userspace launch/status, producer fd1 to the pipe
endpoint, final-stage consumer fd0 from that pipe endpoint, child-only consumer
fd2 to volatile-vfs:/tmp/pipeline-stderr.txt, sink/truncate semantics,
descriptor-backed readback of the 0x1f-byte userspace stderr fixture, and shell
fd2 restoration. Unsupported direct and bare-name variants remain fail-closed.

selected_next_task: null.
planningNeeded: true because no later queued same-lane local POSIX/VFS task has
complete objective dependencies, acceptance criteria, validation gates, docs
requirements, and evidence requirements.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.
