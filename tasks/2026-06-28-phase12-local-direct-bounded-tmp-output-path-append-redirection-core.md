# Phase 12 Local Direct Bounded Tmp Output Path Append Redirection Core

Task:
phase12-local-direct-bounded-tmp-output-path-append-redirection-core-20260628

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Summary

Accepted the direct path-form bounded volatile /tmp output-path append
redirection slice. The shell now accepts the direct append witnesses:

- '/bin/stdout >>/tmp/talos-output-alpha.txt'
- '/bin/stderr 2>>/tmp/talos-error-beta.log'

Each append witness follows an initial accepted truncate/create write to the
same safe volatile /tmp leaf. Both forms still launch descriptor-backed
VFS/userspace programs, rebind only child fd1 or fd2 to the selected
volatile-vfs target, and preserve shell descriptors for later normal direct
stdout/stderr controls.

## Findings

- fixed: Direct absolute path-form stdout append redirection now uses the
  existing supported stdout volatile /tmp leaf parser instead of accepting
  only '/tmp/stdout.txt'.
- fixed: Direct absolute path-form stderr append redirection now uses the
  existing supported stderr volatile /tmp leaf parser instead of accepting
  only '/tmp/stderr.txt'.
- fixed: Added focused local_command_loop coverage for direct stdout/stderr
  truncate-then-append witnesses at caller-chosen safe /tmp leaves,
  descriptor-backed cat readback, later direct descriptor restoration controls,
  and deterministic negative path/grammar controls.
- fixed: The required project-wide clippy gate surfaced build-script warnings
  in generated-root ELF helper code; those were mechanically remediated by
  accepting a Path slice and grouping program-header fields without changing
  generated ELF semantics.
- not-an-issue: The accepted path policy is unchanged from the prior bounded
  volatile /tmp frontier: absolute /tmp leaf only, non-empty ASCII basename,
  no nested slash, no dot or dotdot basename, no writes outside volatile /tmp,
  and no cross-stream reserved basename alias.
- not-an-issue: Direct truncate/create redirection, bare-name truncate/create
  redirection, fixed-path append controls, and pipeline/combined-pipeline
  controls remain retained regression surfaces.
- deferred: Fixed-/bin bare-name append-path generalization is the selected
  next task.
- deferred: Persistent writable filesystem behavior, nested or traversal
  paths, paths outside volatile /tmp, device aliasing beyond accepted
  /dev/null controls, environment-backed PATH, current-directory search,
  command lookup beyond bounded /bin, arbitrary descriptor syntax, descriptor
  moves, separated redirection tokens, arbitrary shell grammar, pipeline path
  generalization, live networking/SSH, generated-root retry, Pi 5 hardware
  proof, boot publication, and phase transition remain outside this slice.

## Evidence

- static inspection: src/local_command_loop.rs changes are limited to the
  direct absolute append parser gates and task-owned tests; build.rs changes
  are mechanical clippy remediation for the required project-wide gate.
- QEMU/substitute local command scenario:
  'cargo -Zjson-target-spec test local_command_loop_appends_direct_path_stdout_stderr_to_bounded_tmp_leaf_paths'
  passed; the saved focused log reports 'test result: ok. 877 passed' and the
  task-owned witness test passed.
- fmt/lint/typecheck: 'cargo fmt --all -- --check' passed and
  'cargo clippy -Zjson-target-spec --all-targets -- -D warnings' passed.
- evidence JSON:
  tasks/evidence/2026-06-28-phase12-local-direct-bounded-tmp-output-path-append-redirection-core/classification.json
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

Descriptor-backed cat readback reports 0x3e bytes for each selected volatile
VFS file, proving the truncate-then-append ordering for two userspace fixture
writes. Later normal '/bin/stdout' and '/bin/stderr' controls prove shell fd1
and fd2 restoration. Unsupported append paths and malformed append forms fail
before file creation/write or new successful process records.

selected_next_task=phase12-local-bare-name-bounded-tmp-output-path-append-redirection-core-20260628.

Live network/SSH remains paused. No Pi 5 hardware claim is made.
