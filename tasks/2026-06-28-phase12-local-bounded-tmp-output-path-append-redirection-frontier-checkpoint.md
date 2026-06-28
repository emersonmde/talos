# Phase 12 Local Bounded Tmp Output Path Append Redirection Frontier Checkpoint

Task:
phase12-local-bounded-tmp-output-path-append-redirection-frontier-checkpoint-20260628

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Summary

Checkpointed the bounded volatile /tmp output-path append redirection frontier
without runtime feature changes. The accepted frontier is limited to safe
volatile /tmp leaf paths for stdout/stderr append redirection through direct
path-form and fixed-/bin bare-name VFS-backed execution.

## Findings

- fixed: Added this checkpoint record and task-owned JSON evidence to freeze the
  direct path-form and fixed-/bin bare-name append boundary before any
  persistence, path broadening, pipeline path generalization, networking,
  hardware, or phase transition work.
- fixed: Updated roadmap, Phase 12, and early POSIX docs with the accepted
  append checkpoint boundary.
- not-an-issue: The accepted append frontier remains exactly the safe volatile
  /tmp leaf policy from the direct and bare-name core tasks: absolute
  /tmp/<basename>, non-empty ASCII basename, no nested slash, no dot/dotdot
  basename, no writes outside volatile /tmp, and no cross-stream reserved
  basename alias.
- not-an-issue: Direct path-form append remains retained as a regression/control
  surface for the fixed-/bin bare-name append witnesses.
- deferred: Persistent writable filesystem behavior, nested or traversal paths,
  paths outside volatile /tmp, device aliasing beyond accepted /dev/null
  controls, environment-backed PATH, current-directory search, command lookup
  beyond bounded /bin, arbitrary shell grammar, pipeline/combined-pipeline path
  generalization, live networking/SSH, Pi 5 hardware proof, generated-root
  retry, boot publication, and phase transition remain outside this frontier.
- deferred: No later queued same-lane local POSIX/VFS task exists after this
  checkpoint, so selected_next_task is null and planningNeeded=true for
  supervisor planning.

## Evidence

- static inspection: retained accepted direct append task
  tasks/2026-06-28-phase12-local-direct-bounded-tmp-output-path-append-redirection-core.md
  and bare-name append task
  tasks/2026-06-28-phase12-local-bare-name-bounded-tmp-output-path-append-redirection-core.md
  both prove the same safe volatile /tmp leaf append policy and descriptor-backed
  cat readback boundary.
- retained QEMU/substitute local command scenarios:
  'cargo -Zjson-target-spec test local_command_loop_appends_direct_path_stdout_stderr_to_bounded_tmp_leaf_paths'
  passed for the direct path-form task, and
  'cargo -Zjson-target-spec test local_command_loop_appends_bare_name_stdout_stderr_to_bounded_tmp_leaf_paths'
  passed for the fixed-/bin bare-name task.
- retained touched-module tests: 'cargo -Zjson-target-spec test --quiet
  local_command_loop' passed for the accepted bare-name task; 878 no_std tests
  passed.
- evidence JSON:
  tasks/evidence/2026-06-28-phase12-local-bounded-tmp-output-path-append-redirection-frontier-checkpoint/classification.json
  and evidence-map.json validate with jq.
- diff validation: 'git diff --check' passed.
- docs validation: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff validation: 'git diff --cached --check' passed before commit.

## Accepted Boundary

The accepted direct path-form sequence is:

~~~text
/bin/stdout >/tmp/talos-output-alpha.txt
/bin/stdout >>/tmp/talos-output-alpha.txt
/bin/stderr 2>/tmp/talos-error-beta.log
/bin/stderr 2>>/tmp/talos-error-beta.log
~~~

The accepted fixed-/bin bare-name sequence is:

~~~text
stdout >/tmp/talos-output-alpha.txt
stdout >>/tmp/talos-output-alpha.txt
stderr 2>/tmp/talos-error-beta.log
stderr 2>>/tmp/talos-error-beta.log
~~~

Both forms launch descriptor-backed VFS/userspace programs, rebind only child
fd1 or fd2 to the selected volatile-vfs target, use append-at-EOF semantics for
the second write, and rely on descriptor-backed cat readback of 0x3e bytes per
file plus later normal stdout/stderr controls for shell descriptor restoration.
Unsupported append paths, malformed append grammar, unsupported descriptor
syntax, separated append tokens, unsupported bare command names, PATH/current
directory lookup, command lookup beyond bounded /bin, and path-containing stage
names remain deterministic negatives that fail before file creation/write or new
successful process records.

selected_next_task: null

planningNeeded: true

planningReason: No later queued same-lane local POSIX/VFS task exists after the
bounded volatile /tmp output-path append redirection checkpoint. Supervisor
planning is required before persistence, path broadening, pipeline path
generalization, networking/SSH, hardware proof, generated-root retry, boot
publication, or phase transition work.

Live network/SSH remains paused. No Pi 5 hardware claim is made.
