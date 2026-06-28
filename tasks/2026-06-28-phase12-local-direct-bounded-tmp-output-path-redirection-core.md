# Phase 12 Local Direct Bounded Tmp Output Path Redirection Core

Task: phase12-local-direct-bounded-tmp-output-path-redirection-core-20260628

## Summary

Accepted the direct path-form bounded volatile /tmp output-path redirection
slice. The shell now accepts:

- '/bin/stdout >/tmp/talos-output-alpha.txt'
- '/bin/stderr 2>/tmp/talos-error-beta.log'

Both forms still launch descriptor-backed VFS/userspace programs. The accepted
path policy is a normalized absolute volatile /tmp leaf path with a non-empty
ASCII basename, no slash after '/tmp/', no '.' or '..' basename, no writes
outside volatile /tmp, and no cross-stream reserved basename alias
('/tmp/stderr.txt' for stdout, '/tmp/stdout.txt' for stderr). The first
implementation uses the existing LocalCommandVolatilePath policy instead of
adding a pair of fixed witness string cases.

## Findings

- fixed: Direct absolute path-form stdout truncate/create redirection now uses
  the existing supported stdout volatile /tmp leaf parser instead of accepting
  only '/tmp/stdout.txt'.
- fixed: Direct absolute path-form stderr truncate/create redirection now uses
  the existing supported stderr volatile /tmp leaf parser instead of accepting
  only '/tmp/stderr.txt'.
- fixed: Added focused local_command_loop coverage for direct stdout/stderr
  safe-path witnesses, descriptor-backed cat readback, later normal descriptor
  restoration controls, and negative path-policy cases.
- not-an-issue: Append-form generalization remains deferred; the direct append
  parser still uses the prior exact-path boundary.
- not-an-issue: Combined stdin/stdout and pipeline redirection forms keep their
  prior exact-path gates; this task accepts only direct path-form stdout/stderr
  truncate/create output redirection.
- deferred: Bare-name command lookup for the same safe /tmp leaf policy remains
  the selected next task.
- deferred: Persistence, nested/traversal paths, paths outside volatile /tmp,
  broader device aliasing, environment-backed PATH, current-directory lookup,
  command lookup beyond bounded /bin, append-path generalization, pipeline path
  generalization, live networking/SSH, generated-root retry, and Pi 5 hardware
  proof remain outside this slice.

## Evidence

- static inspection: src/local_command_loop.rs changes are limited to the
  direct absolute path parser gates and task-owned tests.
- unit tests / QEMU-substitute:
  'cargo -Zjson-target-spec test --quiet local_command_loop_redirects_direct_path_stdout_stderr_to_bounded_tmp_leaf_paths'
  passed; the saved focused log reports 'test result: ok. 875 passed'.
- regression tests / QEMU-substitute:
  'cargo -Zjson-target-spec test --quiet local_command_loop' passed; the saved
  regression log reports 'test result: ok. 875 passed'.
- fmt/lint/typecheck: 'cargo fmt --all -- --check' passed.
- diff validation: 'git diff --check' passed.
- docs validation: '/home/node/.cargo/bin/mdbook build' passed.
- evidence JSON:
  tasks/evidence/2026-06-28-phase12-local-direct-bounded-tmp-output-path-redirection-core/classification.json
  and evidence-map.json validate with jq.

## Accepted Boundary

The accepted direct path-form commands write through child-only fd1/fd2
rebinding to volatile VFS regular files, descriptor-backed cat reads the bytes
back from the chosen safe /tmp leaf paths, and later normal '/bin/stdout' and
'/bin/stderr' controls prove shell fd1/fd2 restoration. Unsupported paths fail
before new process-table records are created.

selected_next_task=phase12-local-bare-name-bounded-tmp-output-path-redirection-core-20260628.

No persistent writable filesystem behavior, append-path generalization,
pipeline path generalization, live networking/SSH, generated-root retry, Pi 5
hardware proof, boot publication, fake/kernel-backed command expansion, or
phase transition is accepted.
