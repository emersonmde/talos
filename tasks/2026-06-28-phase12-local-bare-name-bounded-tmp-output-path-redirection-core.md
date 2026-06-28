# Phase 12 Local Bare Name Bounded Tmp Output Path Redirection Core

Task: phase12-local-bare-name-bounded-tmp-output-path-redirection-core-20260628

## Summary

Accepted the fixed-/bin bare-name bounded volatile /tmp output-path redirection
slice. The shell now accepts:

- 'stdout >/tmp/talos-output-alpha.txt'
- 'stderr 2>/tmp/talos-error-beta.log'

Both forms resolve only through the accepted bounded /bin lookup to
'/bin/stdout' or '/bin/stderr', then launch descriptor-backed VFS/userspace
programs. The accepted path policy is identical to the direct path-form task:
a normalized absolute volatile /tmp leaf path with a non-empty ASCII basename,
no slash after '/tmp/', no '.' or '..' basename, no writes outside volatile
/tmp, and no cross-stream reserved basename alias ('/tmp/stderr.txt' for
stdout, '/tmp/stdout.txt' for stderr).

## Findings

- fixed: Bare-name stdout truncate/create redirection now uses the existing
  supported stdout volatile /tmp leaf parser instead of accepting only
  '/tmp/stdout.txt'.
- fixed: Bare-name stderr truncate/create redirection now uses the existing
  supported stderr volatile /tmp leaf parser instead of accepting only
  '/tmp/stderr.txt'.
- fixed: Added focused local_command_loop coverage for bare-name stdout/stderr
  safe-path witnesses, descriptor-backed cat readback, later normal descriptor
  restoration controls, negative path-policy cases, and unsupported bare
  command names.
- not-an-issue: Unsupported path forms continue to fail before file writes and
  before successful process-table changes; unsupported bare command names keep
  their existing UnknownCommand classification while still avoiding file/write
  effects.
- not-an-issue: Direct path-form bounded /tmp witnesses remain retained
  controls.
- not-an-issue: Append-form generalization remains deferred; bare-name append
  parsing still uses the prior exact-path boundary.
- not-an-issue: Environment-backed PATH and current-directory search remain
  deferred; this slice uses only the existing fixed bounded /bin lookup.
- deferred: Persistence, nested/traversal paths, paths outside volatile /tmp,
  broader device aliasing, command lookup beyond bounded /bin, append-path
  generalization, pipeline path generalization, live networking/SSH,
  generated-root retry, and Pi 5 hardware proof remain outside this slice.

## Evidence

- static inspection: src/local_command_loop.rs changes are limited to the
  bare-name stdout/stderr truncate parser gates and task-owned tests.
- unit tests / QEMU-substitute:
  'cargo -Zjson-target-spec test --quiet local_command_loop_redirects_bare_name_stdout_stderr_to_bounded_tmp_leaf_paths'
  passed; the saved focused log reports 'test result: ok. 876 passed'.
- regression tests / QEMU-substitute:
  'cargo -Zjson-target-spec test --quiet local_command_loop' passed; the saved
  regression log reports 'test result: ok. 876 passed'.
- fmt/lint/typecheck: 'cargo fmt --all -- --check' passed.
- diff validation: 'git diff --check' passed.
- docs validation: '/home/node/.cargo/bin/mdbook build' passed.
- evidence JSON:
  tasks/evidence/2026-06-28-phase12-local-bare-name-bounded-tmp-output-path-redirection-core/classification.json
  and evidence-map.json validate with jq.

## Accepted Boundary

The accepted bare-name commands write through child-only fd1/fd2 rebinding to
volatile VFS regular files, descriptor-backed cat reads the bytes back from the
chosen safe /tmp leaf paths, and later normal 'stdout' and 'stderr' controls
prove shell fd1/fd2 restoration. Unsupported paths fail before new successful
process-table records are created, and unsupported command names retain their
UnknownCommand status without file/write effects.

selected_next_task=phase12-local-bounded-tmp-output-path-redirection-frontier-checkpoint-20260628.

No persistent writable filesystem behavior, append-path generalization,
pipeline path generalization, live networking/SSH, generated-root retry, Pi 5
hardware proof, boot publication, fake/kernel-backed command expansion, or
phase transition is accepted.
