# Phase 12 Local Bare-Name Bounded Tmp Output Path Append Redirection Core

Task:
phase12-local-bare-name-bounded-tmp-output-path-append-redirection-core-20260628

Status: accepted.

## Summary

Accepted the fixed-/bin bare-name bounded volatile /tmp output-path append
redirection slice. The shell now accepts the bare-name append witnesses:

- 'stdout >>/tmp/talos-output-alpha.txt'
- 'stderr 2>>/tmp/talos-error-beta.log'

Each append witness follows an initial accepted truncate/create write to the
same safe volatile /tmp leaf. Both forms resolve only through bounded /bin
lookup to '/bin/stdout' or '/bin/stderr', launch descriptor-backed
VFS/userspace programs, rebind only child fd1 or fd2 to the selected
volatile-vfs target, and preserve shell descriptors for later normal bare-name
stdout/stderr controls.

## Findings

- fixed: Bare-name stdout append redirection now reuses the accepted safe
  stdout volatile /tmp leaf parser instead of accepting only '/tmp/stdout.txt'.
- fixed: Bare-name stderr append redirection now reuses the accepted safe
  stderr volatile /tmp leaf parser instead of accepting only '/tmp/stderr.txt'.
- fixed: Removed the now-dead exact stderr path helper after the bare-name
  stderr append parser moved to the supported safe leaf policy.
- fixed: Added focused local_command_loop coverage for bare-name stdout/stderr
  truncate-then-append witnesses at caller-chosen safe /tmp leaves,
  descriptor-backed cat readback, later bare-name descriptor restoration
  controls, direct append regression controls, and deterministic negative
  path/grammar/command controls.
- fixed: Updated a stale negative-control expectation that treated
  'stderr 2>>/tmp/other.txt' as invalid; safe bare-name stderr append to a
  caller-chosen /tmp leaf is now accepted by this task.
- fixed: Cleaned task-owned src/local_command_loop.rs clippy diagnostics so the
  retained all-targets clippy baseline no longer references task-owned source.
- not-an-issue: The path policy is unchanged from the prior bounded volatile
  /tmp frontier: absolute /tmp leaf only, non-empty ASCII basename, no nested
  slash, no dot or dotdot basename, no writes outside volatile /tmp, and no
  cross-stream reserved basename alias.
- not-an-issue: Direct path-form append remains accepted and retained as a
  regression/control surface.
- deferred: Persistent writable filesystem behavior, nested or traversal
  paths, paths outside volatile /tmp, device aliasing beyond accepted
  /dev/null controls, environment-backed PATH, current-directory search,
  command lookup beyond bounded /bin, arbitrary descriptor syntax, descriptor
  moves, separated redirection tokens, arbitrary shell grammar,
  pipeline/combined-pipeline path generalization, live networking/SSH,
  generated-root retry, Pi 5 hardware proof, boot publication, and phase
  transition remain outside this slice.
- deferred: The repo-wide all-targets clippy baseline still fails under the
  current toolchain on out-of-scope files/classes: grouped literal digits,
  unwrap_or_default, manual is_multiple_of, too_many_arguments, enum variant
  names, comparison extremes, get_first, collapsible if, large enum variant,
  slow vector initialization, eq_op, and items_after_test_module. The refreshed
  retained log has task_owned_refs=absent for src/local_command_loop.rs.

## Evidence

- static inspection: src/local_command_loop.rs changes are limited to the
  bare-name append parser gates, removal of the newly dead exact stderr helper,
  and task-owned tests/regression updates.
- QEMU/substitute local command scenario:
  'cargo -Zjson-target-spec test local_command_loop_appends_bare_name_stdout_stderr_to_bounded_tmp_leaf_paths'
  passed after the task-owned clippy cleanup; the saved focused log reports
  'test result: ok. 878 passed' and the task-owned witness test passed.
- touched-module tests:
  'cargo -Zjson-target-spec test --quiet local_command_loop' passed under the
  workspace QEMU runner; 878 no_std tests passed.
- fmt: 'cargo fmt --all -- --check' passed after rustfmt was applied.
- lint/typecheck: 'cargo clippy -Zjson-target-spec --all-targets -- -D warnings'
  failed on pre-existing/out-of-scope repo-wide lints across unrelated modules,
  with no diagnostics referencing src/local_command_loop.rs after the
  task-owned cleanup; retained log:
  tasks/evidence/2026-06-28-phase12-local-bare-name-bounded-tmp-output-path-append-redirection-core/clippy-all-targets-failure.log
- evidence JSON:
  tasks/evidence/2026-06-28-phase12-local-bare-name-bounded-tmp-output-path-append-redirection-core/classification.json
  and evidence-map.json validate with jq.
- diff validation: 'git diff --check' passed.
- docs validation: '/home/node/.cargo/bin/mdbook build' passed.
- staged diff validation: 'git diff --cached --check' passed before commit.

## Accepted Boundary

The accepted bare-name sequence is:

~~~text
stdout >/tmp/talos-output-alpha.txt
stdout >>/tmp/talos-output-alpha.txt
stderr 2>/tmp/talos-error-beta.log
stderr 2>>/tmp/talos-error-beta.log
~~~

Descriptor-backed cat readback reports 0x3e bytes for each selected volatile
VFS file, proving the truncate-then-append ordering for two userspace fixture
writes. Later normal 'stdout' and 'stderr' controls prove shell fd1 and fd2
restoration. Direct path-form append to the same safe leaves remains retained
as regression evidence. Unsupported append paths, malformed append grammar,
unsupported descriptor syntax, and unsupported bare command names fail before
file creation/write or new successful process records.

selected_next_task:
phase12-local-bounded-tmp-output-path-append-redirection-frontier-checkpoint-20260628

Live network/SSH remains paused. No Pi 5 hardware claim is made.
