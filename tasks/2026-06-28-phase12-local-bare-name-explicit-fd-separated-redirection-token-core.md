# Phase 12 Local Bare-Name Explicit-Fd Separated Redirection Token Core

Task:
phase12-local-bare-name-explicit-fd-separated-redirection-token-core-20260628

Status: accepted and committed; durable supervisor state records the final
commit SHA.

## Summary

Implemented the fixed-/bin bare-name explicit-fd separated redirection-token
core for the local POSIX/VFS/userspace lane. The accepted witness forms are:

- 'stdout 1 > /tmp/talos-output-alpha.txt'
- 'stdout 1 >> /tmp/talos-output-alpha.txt'
- 'stderr 2 > /tmp/talos-error-beta.log'
- 'stderr 2 >> /tmp/talos-error-beta.log'

The implementation keeps all successful command-visible behavior on the
existing descriptor-backed VFS/userspace path: bounded /bin executable lookup,
executable open/read, userspace launch/status, child-only fd1/fd2 descriptor
rebinding, volatile-vfs /tmp regular file write/readback, append-at-EOF
behavior, waitpid/laststatus observations, process-table accounting, and
descriptor restoration. Only fixed-/bin bare-name command parsing accepts the
new fd-token form in this task. Pipeline explicit-fd separated-token support
remains deferred to the queued follow-up task.

## Findings

- fixed: Fixed-/bin bare-name command parsing now consumes an explicit fd token
  followed by a separated output redirection operator token and a non-empty path
  operand token for the accepted stdout/stderr surfaces.
- fixed: The parser keeps the fd token out of child argv, so the accepted forms
  still launch /bin/stdout and /bin/stderr with argc1/argv0-only startup state
  after bounded /bin lookup.
- fixed: Added a focused QEMU/substitute local_command_loop test covering the
  accepted bare-name fd-token stdout/stderr truncate/append witnesses,
  descriptor readbacks, restoration controls, retained direct explicit-fd
  controls, wrong fd/command pairings, missing operands, unsupported paths,
  unsupported operators, unsupported command names, and pipeline leakage.
- fixed: Updated older direct and bare-name separated-token negative controls so
  they reject unsupported command/fd forms instead of treating the newly
  accepted bare-name fd1 form as invalid.
- fixed: Added a local command capability marker for bounded bare-name
  explicit-fd separated redirection tokens.
- not-an-issue: The accepted output path policy remains the previously accepted
  safe volatile /tmp leaf policy: non-empty basename, no nested slash, no
  dot/dotdot basename, no writes outside volatile /tmp, and no cross-stream
  reserved basename alias.
- deferred: Direct pipeline explicit-fd separated-token support, bare-name
  pipeline explicit-fd separated-token support, explicit fd input redirection,
  fd duplication/close syntax, PATH/current-directory lookup, command lookup
  beyond bounded /bin, arbitrary shell grammar, persistence, generated-root
  retry, boot publication, live networking/SSH, Pi 5 hardware action, and phase
  transition remain outside this task.

## Evidence

- static inspection: src/local_command_loop.rs parser changes are bounded to
  fixed-/bin bare-name command parsing plus a capability marker and tests.
- QEMU/substitute unit tests:
  'cargo -Zjson-target-spec test --quiet local_command_loop' passed with 885
  Talos no_std tests using QEMU 9.2.0 on PATH.
- retained transcript:
  tasks/evidence/2026-06-28-phase12-local-bare-name-explicit-fd-separated-redirection-token-core/qemu-substitute-local-command-loop.log.
- task-owned JSON:
  tasks/evidence/2026-06-28-phase12-local-bare-name-explicit-fd-separated-redirection-token-core/classification.json
  and evidence-map.json.

## Accepted Boundary

Accepted fixed-/bin bare-name explicit-fd separated redirection tokens are
limited to:

~~~text
stdout 1 > /tmp/talos-output-alpha.txt
stdout 1 >> /tmp/talos-output-alpha.txt
stderr 2 > /tmp/talos-error-beta.log
stderr 2 >> /tmp/talos-error-beta.log
~~~

The output forms reuse the accepted safe volatile /tmp leaf policy. Wrong
fd/command pairings, unsupported fd tokens, missing operands, unsupported paths,
unsupported bare command names, mixed direct/bare broadening, and pipelines fail
closed in this slice.

selected_next_task=phase12-local-direct-pipeline-explicit-fd-separated-redirection-token-core-20260628.

Live network/SSH remains paused. No Pi 5 hardware/lab action, boot publication,
generated-root retry, persistence, arbitrary paths, PATH/current-directory
lookup, or phase transition was performed or claimed.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet local_command_loop: passed with 885
  Talos no_std tests using QEMU 9.2.0 on PATH.
- jq empty on task-owned classification/evidence JSON: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.
