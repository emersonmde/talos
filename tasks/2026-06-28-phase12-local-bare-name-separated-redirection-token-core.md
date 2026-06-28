# Phase 12 Local Bare-Name Separated Redirection Token Core

Task id:
phase12-local-bare-name-separated-redirection-token-core-20260628

## Summary

Implemented the fixed-/bin bare-name separated redirection-token core for the
already accepted local descriptor-backed stdin/stdout/stderr VFS userspace
surfaces.

Accepted witnesses:

- 'stdin < /etc/banner.txt'
- 'stdout > /tmp/talos-output-alpha.txt'
- 'stdout >> /tmp/talos-output-alpha.txt'
- 'stderr 2> /tmp/talos-error-beta.log'
- 'stderr 2>> /tmp/talos-error-beta.log'

The implementation resolves bare names only through the bounded /bin lookup,
then executes through descriptor-backed VFS open/read, userspace launch/status,
child-only descriptor rebinding, volatile-vfs /tmp leaf readback, and descriptor
restoration controls.

## Findings and Disposition

- fixed: Bare-name command parsing now consumes a separated redirection
  operator token followed by a non-empty operand token for the exact accepted
  stdin/stdout/stderr surfaces.
- fixed: Added QEMU/substitute local_command_loop coverage for bare-name
  separated stdin/stdout/stderr truncate/append witnesses, descriptor-backed
  readbacks, direct separated-token controls, and fail-closed unsupported forms.
- fixed: Updated prior bare-name redirection negative controls that treated the
  now-accepted separated stdout/stderr append syntax as invalid.
- fixed: Added a local command capability marker for bounded bare-name
  separated redirection tokens.
- fixed: Updated roadmap and early POSIX shape docs with the accepted/deferred
  grammar frontier.
- deferred: Pipeline separated-token support, separated explicit fd syntax such
  as '1 > path' or '2 > path', mixed direct/bare broadening,
  PATH/current-directory lookup, command lookup beyond bounded /bin, arbitrary
  shell grammar, persistence, generated-root retry, boot publication, live
  networking/SSH, Pi 5 hardware proof, and phase transition.
- not-an-issue: The accepted command-visible behavior remains backed by
  descriptor/VFS/userspace layers; no fake/kernel-backed command expansion was
  added.

## Evidence

- static inspection: src/local_command_loop.rs parser changes are bounded to
  fixed-/bin bare-name command execution and reuse the accepted volatile /tmp
  leaf path policy.
- fmt/lint: cargo fmt --all -- --check: passed.
- QEMU/substitute unit tests: cargo -Zjson-target-spec test --quiet
  local_command_loop: passed; 882 Talos no_std tests retained in
  tasks/evidence/2026-06-28-phase12-local-bare-name-separated-redirection-token-core/qemu-substitute-local-command-loop.log.
- evidence validation: jq empty
  tasks/evidence/2026-06-28-phase12-local-bare-name-separated-redirection-token-core/classification.json
  tasks/evidence/2026-06-28-phase12-local-bare-name-separated-redirection-token-core/evidence-map.json:
  passed.
- diff validation: git diff --check: passed.
- docs validation: /home/node/.cargo/bin/mdbook build: passed.
- staged diff validation: git diff --cached --check: passed before commit.

## Accepted Frontier

Accepted bare-name separated redirection tokens are limited to:

    stdin < /etc/banner.txt
    stdout > /tmp/talos-output-alpha.txt
    stdout >> /tmp/talos-output-alpha.txt
    stderr 2> /tmp/talos-error-beta.log
    stderr 2>> /tmp/talos-error-beta.log

The output forms reuse the accepted safe volatile /tmp leaf policy. The stdin
form reads only initramfs:/etc/banner.txt. No separated explicit fd syntax is
accepted. Unsupported paths, missing operands, unsupported operators, reserved
basename aliases, unsupported commands, and pipeline separated-token forms fail
closed before file creation/write or new successful process records.

selected_next_task=phase12-local-direct-pipeline-separated-redirection-token-core-20260628.

## Scope Statement

No Pi 5 hardware/lab action, boot publication, generated-root retry, live
networking/SSH, persistence, arbitrary paths, pipeline separated-token support,
separated explicit fd syntax, or phase transition was performed or claimed.
