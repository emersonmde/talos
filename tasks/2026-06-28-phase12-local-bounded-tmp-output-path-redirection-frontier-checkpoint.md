# Phase 12 Local Bounded Tmp Output Path Redirection Frontier Checkpoint

Task id:
phase12-local-bounded-tmp-output-path-redirection-frontier-checkpoint-20260628

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted local-only bounded volatile /tmp output-path
redirection frontier after the direct path-form and fixed-/bin bare-name cores
accepted:

~~~text
/bin/stdout >/tmp/talos-output-alpha.txt
/bin/stderr 2>/tmp/talos-error-beta.log
stdout >/tmp/talos-output-alpha.txt
stderr 2>/tmp/talos-error-beta.log
~~~

This checkpoint is static reconciliation work. It does not change source
behavior, run Pi 5 hardware, resume live networking/SSH, retry generated-root
command input, accept append-path generalization, accept persistent writable
filesystem behavior, accept pipeline path generalization, or accept a phase
transition.

## Findings

- fixed: The accepted bounded volatile /tmp output-path frontier is reconciled
  against the retained direct path-form core record, fixed-/bin bare-name core
  record, task-owned classification/evidence maps, QEMU/substitute regression
  evidence, docs, and retained fixed-path controls.
- fixed: The accepted witnesses remain exactly the direct path-form and
  fixed-/bin bare-name stdout/stderr truncate/create redirection forms
  targeting safe caller-chosen volatile /tmp leaf paths.
- fixed: The retained evidence records descriptor-backed VFS/userspace launch,
  child-only fd1/fd2 rebinding to volatile-vfs safe /tmp leaves,
  descriptor-backed cat readback of stdout/stderr fixture bytes, lifecycle
  status records, later direct and bare-name descriptor restoration controls,
  and negative path-policy controls that fail before file creation/write or
  new successful process records.
- fixed: No next local POSIX/VFS task is selected because no later queued
  same-lane task exists after this checkpoint with complete objective
  dependencies, acceptance criteria, validation gates, docs requirements, and
  evidence requirements. Durable state must set planningNeeded=true for
  supervisor planning.
- not-an-issue: No implementation change is required for this checkpoint; the
  core tasks already recorded static inspection, fmt/lint/typecheck,
  QEMU/substitute unit/regression evidence, JSON, diff, docs, and staged-diff
  evidence.
- deferred: append-path generalization, persistent writable filesystem
  behavior, nested or traversal paths, paths outside volatile /tmp, device
  aliasing beyond accepted /dev/null controls, environment-backed PATH,
  current-directory search, command lookup beyond bounded /bin, arbitrary
  shell grammar, pipeline/combined-pipeline path generalization, live
  networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
  transition.

## Evidence Map

- Direct bounded /tmp output-path redirection core:
  tasks/2026-06-28-phase12-local-direct-bounded-tmp-output-path-redirection-core.md.
- Bare-name bounded /tmp output-path redirection core:
  tasks/2026-06-28-phase12-local-bare-name-bounded-tmp-output-path-redirection-core.md.
- Task-owned checkpoint classification and evidence:
  tasks/evidence/2026-06-28-phase12-local-bounded-tmp-output-path-redirection-frontier-checkpoint/classification.json
  and
  tasks/evidence/2026-06-28-phase12-local-bounded-tmp-output-path-redirection-frontier-checkpoint/evidence-map.json.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted bounded volatile /tmp output-path redirection frontier is
local-only and static/unit/QEMU-substitute backed. The accepted witnesses are
exactly:

~~~text
/bin/stdout >/tmp/talos-output-alpha.txt
/bin/stderr 2>/tmp/talos-error-beta.log
stdout >/tmp/talos-output-alpha.txt
stderr 2>/tmp/talos-error-beta.log
~~~

The direct witnesses load through explicit descriptor-backed VFS paths. The
bare-name witnesses resolve only through the bounded fixed /bin lookup to
/bin/stdout and /bin/stderr. Both forms use the same conservative path policy:
an absolute volatile /tmp leaf path with a non-empty ASCII basename, no nested
slash, no dot or dotdot basename, no writes outside volatile /tmp, and no
cross-stream reserved basename alias. Each accepted command records child-only
fd1 or fd2 rebinding to the selected volatile-vfs target, descriptor-backed cat
readback of the expected userspace fixture bytes, coherent lifecycle/status
observations, and later direct or bare-name descriptor restoration controls.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain append-path generalization, persistent writable
filesystem behavior, nested or traversal paths, paths outside volatile /tmp,
device aliasing beyond accepted /dev/null controls, environment-backed PATH,
current-directory search, command lookup beyond bounded /bin, arbitrary shell
grammar, pipeline/combined-pipeline path generalization, scheduler
concurrency, fork/signals, process groups/sessions, broad procfs/Linux ps, PID
policy expansion, waitpid options, live networking/SSH, Pi 5 hardware proof,
generated-root command-input retry, and phase transition.

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
