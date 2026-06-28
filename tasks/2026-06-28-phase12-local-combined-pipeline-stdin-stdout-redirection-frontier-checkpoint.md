# Phase 12 Local Combined Pipeline Stdin Stdout Redirection Frontier Checkpoint

Task: phase12-local-combined-pipeline-stdin-stdout-redirection-frontier-checkpoint-20260628

Status: accepted; commit recorded in supervisor state.

## Scope

- Reviewed the accepted direct and fixed-/bin bare-name combined pipeline
  stdin/stdout redirection cores, closeouts, classifications, evidence maps,
  QEMU/substitute transcripts, retained regression controls, and project docs.
- Reconciled the exact accepted direct witness
  '/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt' with
  the exact fixed-/bin bare-name witness
  'stdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt'.
- Kept the accepted surface limited to those exact combined pipeline
  stdin/stdout redirection witnesses. There is no later queued same-lane local
  POSIX/VFS task after this checkpoint, so supervisor planning is required
  before another worker promotion.
- Did not implement runtime feature changes, append/combined stderr pipeline
  redirections, arbitrary paths, persistent writable filesystem behavior, live
  networking/SSH, Pi 5 hardware action, generated-root retry, or a phase
  transition.

## Findings

- not-an-issue: the accepted direct path-form witness is exactly
  '/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt'.
  Retained evidence records descriptor-backed VFS loading for both stages,
  accepted userspace launch/status, producer fd0 from
  initramfs:/etc/banner.txt, producer fd1 to the pipe endpoint, consumer fd0
  from that pipe endpoint, child-only consumer fd1 redirection to
  volatile-vfs:/tmp/pipeline-combined.txt, descriptor-backed readback through
  'cat /tmp/pipeline-combined.txt', shell fd0/fd1 restoration, and coherent
  process-status observations.
- not-an-issue: the accepted fixed-/bin bare-name witness is exactly
  'stdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt'. Retained
  evidence records bounded resolution only to '/bin/stdin' for both stages
  before using the same VFS/userspace launch, pipe handoff, final-stage fd1
  regular-file redirection, readback, shell restoration, and status paths as
  the direct witness.
- not-an-issue: retained fail-closed evidence keeps unsupported neighboring
  forms outside the accepted surface: append combined pipeline forms, stderr
  combined pipeline forms, wrong output path, stdout producer with the combined
  sink, explicit fd1 syntax, separated redirection tokens, persistent /var
  target, path-containing bare-name stage names, unsupported stage names,
  multistage combined redirection, arbitrary input/output paths, persistent
  writable filesystem behavior, PATH/current-directory lookup, and command
  lookup beyond bounded /bin.
- fixed: roadmap, Phase 12, and early POSIX docs now record the combined
  pipeline stdin/stdout frontier checkpoint and that no later queued same-lane
  local POSIX/VFS task is mechanically objective.
- deferred: append/combined stderr pipeline redirections, arbitrary paths,
  persistent writable filesystem behavior, environment-backed PATH,
  current-directory search, command lookup beyond bounded /bin, arbitrary shell
  grammar, unbounded or concurrent pipelines, scheduler concurrency,
  fork/signals, process groups/sessions, live networking/SSH, Pi 5 hardware
  proof, generated-root retry, and phase transition remain outside this
  checkpoint.

## Evidence

- static inspection:
  tasks/2026-06-28-phase12-local-direct-combined-pipeline-stdin-stdout-redirection-core.md,
  tasks/2026-06-28-phase12-local-direct-combined-pipeline-stdin-stdout-redirection-closeout.md,
  tasks/2026-06-28-phase12-local-bare-name-combined-pipeline-stdin-stdout-redirection-core.md,
  and
  tasks/2026-06-28-phase12-local-bare-name-combined-pipeline-stdin-stdout-redirection-closeout.md.
- task-owned JSON inspection:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stdin-stdout-redirection-core/classification.json,
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stdin-stdout-redirection-core/evidence-map.json,
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stdin-stdout-redirection-closeout/classification.json,
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stdin-stdout-redirection-closeout/evidence-map.json,
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stdin-stdout-redirection-core/classification.json,
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stdin-stdout-redirection-core/evidence-map.json,
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stdin-stdout-redirection-closeout/classification.json,
  and
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stdin-stdout-redirection-closeout/evidence-map.json.
- QEMU/substitute transcript inspection:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-stdin-stdout-redirection-core/qemu-local-shell-direct-combined-pipeline-stdin-stdout-redirection-smoke.log
  and
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-stdin-stdout-redirection-core/qemu-local-shell-bare-name-combined-pipeline-stdin-stdout-redirection-smoke.log.
- project doc inspection: docs/src/roadmap.md,
  docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.
- checkpoint classification:
  tasks/evidence/2026-06-28-phase12-local-combined-pipeline-stdin-stdout-redirection-frontier-checkpoint/classification.json.
- checkpoint evidence map:
  tasks/evidence/2026-06-28-phase12-local-combined-pipeline-stdin-stdout-redirection-frontier-checkpoint/evidence-map.json.

## Validation

- 'jq empty' over task-owned direct combined pipeline, bare-name combined
  pipeline, closeout, and checkpoint JSON evidence passed.
- 'git diff --check' passed.
- '/home/node/.cargo/bin/mdbook build' passed.
- 'git diff --cached --check' passed before commit.

## Result

The accepted combined pipeline stdin/stdout redirection frontier is limited to:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined.txt
stdin </etc/banner.txt | stdin >/tmp/pipeline-combined.txt
~~~

The direct form and fixed-/bin bare-name form both use descriptor-backed VFS
program loading, accepted userspace launch/status, producer fd0 from
initramfs:/etc/banner.txt, producer fd1 to the pipe endpoint, final-stage
consumer fd0 from that pipe endpoint, child-only consumer fd1 to
volatile-vfs:/tmp/pipeline-combined.txt, descriptor-backed readback of the
nested userspace stdin report, and shell fd0/fd1 restoration. Unsupported
direct and bare-name variants remain fail-closed.

selected_next_task: null.
planningNeeded: true because no later queued same-lane local POSIX/VFS task has
complete objective dependencies, acceptance criteria, validation gates, docs
requirements, and evidence requirements.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.
