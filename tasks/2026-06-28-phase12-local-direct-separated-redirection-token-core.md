# Phase 12 Local Direct Separated Redirection Token Core

Task:
phase12-local-direct-separated-redirection-token-core-20260628

Status: accepted and committed; durable supervisor state records the final
commit SHA.

## Summary

Implemented the direct absolute-path separated redirection-token core for the
local POSIX/VFS/userspace lane. The accepted witness forms are:

- '/bin/stdin < /etc/banner.txt'
- '/bin/stdout > /tmp/talos-output-alpha.txt'
- '/bin/stdout >> /tmp/talos-output-alpha.txt'
- '/bin/stderr 2> /tmp/talos-error-beta.log'
- '/bin/stderr 2>> /tmp/talos-error-beta.log'

The implementation keeps all successful command-visible behavior on the
existing descriptor-backed VFS/userspace path: executable open/read, userspace
launch/status, child-only descriptor rebinding, volatile-vfs /tmp regular file
write/readback, waitpid/laststatus observations, and descriptor restoration.
Only direct absolute-path command parsing accepts separated operator and path
tokens in this task. Fixed-/bin bare-name commands and pipelines remain
deferred to queued follow-up tasks.

## Findings

- fixed: Direct absolute-path command parsing now consumes a separated
  redirection operator token followed by a non-empty operand token for the
  accepted direct surfaces.
- fixed: Separated stdin redirection is intentionally standalone for this
  slice; it does not combine with an output redirection token. The existing
  fused combined stdin/stdout witness remains unchanged.
- fixed: Added a focused QEMU/substitute local_command_loop test covering the
  accepted separated stdin/stdout/stderr truncate/append witnesses, descriptor
  readbacks, restoration controls, missing operands, unsupported paths,
  unsupported operators, separated explicit fd forms, bare-name leakage, and
  pipeline leakage.
- fixed: Updated the stale direct stderr negative control so it no longer
  treats the accepted separated stderr form as invalid.
- fixed: Added a local command capability marker for direct separated
  redirection tokens.
- not-an-issue: The accepted output path policy remains the previously accepted
  safe volatile /tmp leaf policy: non-empty basename, no nested slash, no
  dot/dotdot basename, no writes outside volatile /tmp, and no cross-stream
  reserved basename alias.
- deferred: Fixed-/bin bare-name separated-token support, pipeline
  separated-token support, separated explicit fd syntax such as '1 > path' or
  '2 > path', PATH/current-directory lookup, command lookup beyond bounded
  /bin, arbitrary shell grammar, persistence, generated-root retry, boot
  publication, live networking/SSH, Pi 5 hardware action, and phase transition
  remain outside this task.

## Evidence

- static inspection: src/local_command_loop.rs parser changes are bounded to
  direct absolute-path command parsing plus a capability marker and tests.
- QEMU/substitute unit tests:
  'cargo -Zjson-target-spec test --quiet local_command_loop' passed with 881
  Talos no_std tests using QEMU 9.2.0 on PATH.
- retained transcript:
  tasks/evidence/2026-06-28-phase12-local-direct-separated-redirection-token-core/qemu-substitute-local-command-loop.log.
- task-owned JSON:
  tasks/evidence/2026-06-28-phase12-local-direct-separated-redirection-token-core/classification.json
  and evidence-map.json.

## Accepted Boundary

Accepted direct separated redirection tokens are limited to:

~~~text
/bin/stdin < /etc/banner.txt
/bin/stdout > /tmp/talos-output-alpha.txt
/bin/stdout >> /tmp/talos-output-alpha.txt
/bin/stderr 2> /tmp/talos-error-beta.log
/bin/stderr 2>> /tmp/talos-error-beta.log
~~~

The output forms reuse the accepted safe volatile /tmp leaf policy. The stdin
form reads only initramfs:/etc/banner.txt. No separated explicit fd syntax is
accepted: '1 > path', '1 >> path', '2 > path', and similar multi-token fd
aliases fail closed. Bare-name separated-token commands and pipeline
separated-token commands fail closed in this slice.

selected_next_task=phase12-local-bare-name-separated-redirection-token-core-20260628.

Live network/SSH remains paused. No Pi 5 hardware/lab action, boot
publication, generated-root retry, persistence, arbitrary paths, or phase
transition was performed or claimed.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet local_command_loop: passed with 881
  Talos no_std tests using QEMU 9.2.0 on PATH.
- jq empty on task-owned classification/evidence JSON: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.
