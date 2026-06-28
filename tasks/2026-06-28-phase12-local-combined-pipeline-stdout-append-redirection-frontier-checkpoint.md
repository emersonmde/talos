# Phase 12 Local Combined Pipeline Stdout Append Redirection Frontier Checkpoint

Task id: phase12-local-combined-pipeline-stdout-append-redirection-frontier-checkpoint-20260628

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted local-only combined pipeline stdout append redirection
frontier after the direct path-form and fixed-/bin bare-name cores accepted:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined-append.txt
/bin/stdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt
stdin </etc/banner.txt | stdin >/tmp/pipeline-combined-append.txt
stdin </etc/banner.txt | stdin >>/tmp/pipeline-combined-append.txt
~~~

This checkpoint is static reconciliation work. It does not change source
behavior, run Pi 5 hardware, resume live networking/SSH, retry generated-root
command input, accept combined stderr append, accept arbitrary paths, accept
persistent writable filesystem behavior, or accept a phase transition.

## Findings

- fixed: The accepted combined pipeline stdout append frontier is reconciled
  against the retained direct path-form core record, fixed-/bin bare-name core
  record, task-owned classification/evidence maps, QEMU/substitute transcripts,
  docs, and regression controls.
- fixed: The accepted witnesses remain exactly the direct path-form sequence
  '/bin/stdin </etc/banner.txt | /bin/stdin
  >/tmp/pipeline-combined-append.txt' then '/bin/stdin </etc/banner.txt |
  /bin/stdin >>/tmp/pipeline-combined-append.txt' and the fixed-/bin
  bare-name sequence 'stdin </etc/banner.txt | stdin
  >/tmp/pipeline-combined-append.txt' then 'stdin </etc/banner.txt | stdin
  >>/tmp/pipeline-combined-append.txt'. Bare-name command lookup is only the
  accepted fixed bounded /bin lookup to '/bin/stdin'.
- fixed: The retained evidence records producer fd0 from
  initramfs:/etc/banner.txt, producer fd1 as the pipe endpoint, consumer fd0
  as the pipe endpoint, consumer fd1 rebinding to
  volatile-vfs:/tmp/pipeline-combined-append.txt, inherited fd2, closed loader
  temporary descriptors, first-command truncate/sink behavior, second-command
  append-at-EOF behavior, descriptor-backed 'cat
  /tmp/pipeline-combined-append.txt' readback of two nested userspace stdin
  reports in order, coherent process status observations, and later normal
  direct/bare-name fd0/fd1 restoration controls.
- fixed: No next local POSIX/VFS task is selected because no later queued
  same-lane task exists after this checkpoint with complete objective
  dependencies, acceptance criteria, validation gates, docs requirements, and
  evidence requirements. Durable state must set planningNeeded=true for
  supervisor planning.
- not-an-issue: No implementation change is required for this checkpoint; the
  core tasks already recorded static inspection, fmt/lint/typecheck, unit
  test, QEMU/substitute, JSON, diff, docs, and staged-diff evidence.
- deferred: Combined stderr pipeline redirections, arbitrary input/output
  paths, persistent writable filesystem behavior, separated redirection-token
  grammar, explicit fd1 syntax, environment-backed PATH, current-directory
  search, command lookup beyond bounded /bin, quoting, escaping, globbing,
  variables, shell functions, arbitrary shell grammar, unbounded/concurrent
  pipelines, scheduler concurrency, fork/signals, process groups/sessions,
  live networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
  transition.

## Evidence Map

- Direct combined pipeline stdout append redirection core:
  tasks/2026-06-28-phase12-local-direct-combined-pipeline-stdout-append-redirection-core.md.
- Bare-name combined pipeline stdout append redirection core:
  tasks/2026-06-28-phase12-local-bare-name-combined-pipeline-stdout-append-redirection-core.md.
- Task-owned checkpoint classification and evidence:
  tasks/evidence/2026-06-28-phase12-local-combined-pipeline-stdout-append-redirection-frontier-checkpoint/classification.json
  and
  tasks/evidence/2026-06-28-phase12-local-combined-pipeline-stdout-append-redirection-frontier-checkpoint/evidence-map.json.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted combined pipeline stdout append redirection frontier is local-only
and static/unit/QEMU-substitute backed. The accepted witnesses are exactly:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/pipeline-combined-append.txt
/bin/stdin </etc/banner.txt | /bin/stdin >>/tmp/pipeline-combined-append.txt
stdin </etc/banner.txt | stdin >/tmp/pipeline-combined-append.txt
stdin </etc/banner.txt | stdin >>/tmp/pipeline-combined-append.txt
~~~

The direct witnesses resolve through explicit descriptor-backed VFS path
loading. The bare-name witnesses resolve only through the bounded fixed /bin
lookup to '/bin/stdin'. Each accepted producer records fd0 from
initramfs:/etc/banner.txt, fd1 to the serialized pipe endpoint, inherited fd2,
closed loader temporary descriptors, and accepted userspace launch/status
records. Each accepted consumer records fd0 from the pipe endpoint, fd1 to
volatile-vfs:/tmp/pipeline-combined-append.txt, inherited fd2, first-command
truncate/sink behavior, second-command append-at-EOF behavior, and later
descriptor-backed 'cat /tmp/pipeline-combined-append.txt' readback of the two
nested userspace stdin reports in order. Subsequent normal direct and bare-name
controls prove shell fd0/fd1 restoration.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain combined stderr pipeline redirections, arbitrary
input/output paths, persistent writable filesystem behavior, separated
redirection-token grammar, explicit fd1 syntax, environment-backed PATH,
current-directory search, command lookup beyond bounded /bin, quoting,
escaping, globbing, variables, shell functions, arbitrary shell grammar,
unbounded/concurrent pipelines, scheduler concurrency, fork/signals, process
groups/sessions, broad procfs/Linux ps, PID policy expansion, waitpid options,
live networking/SSH, Pi 5 hardware proof, generated-root command-input retry,
and phase transition.

## Validation

- Static inspection of task records, retained evidence paths, docs, and git
  diff: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

## Result

selected_next_task: null.

planningNeeded: true.

No later queued same-lane local POSIX/VFS task is mechanically objective after
this checkpoint. Supervisor planning is required before further worker
promotion.
