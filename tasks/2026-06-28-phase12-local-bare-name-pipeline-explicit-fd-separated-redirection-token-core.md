# Phase 12 Local Bare-Name Pipeline Explicit-Fd Separated Redirection Token Core

Task:
phase12-local-bare-name-pipeline-explicit-fd-separated-redirection-token-core-20260628

## Summary

Implemented explicit fd, operator, and path as separate tokens for the already
accepted fixed-/bin bare-name two-stage combined pipeline surfaces. The change
does not add arbitrary shell grammar, mixed direct/bare stages, persistence, or
hardware claims.

## Findings

- fixed: Bare-name pipeline consumer parsing now accepts 'stdin 1 > path',
  'stdin 1 >> path', 'stderr 2 > path', and 'stderr 2 >> path' only on the
  accepted final-stage output-redirection positions.
- fixed: Bare-name pipeline argument guards now classify explicit fd,
  separated operator, and path as a bounded redirection token sequence instead
  of a literal stage argv.
- fixed: The old separated-token regression that expected 'stderr 2 >> path'
  to fail closed was updated because that exact form is now the accepted task
  boundary.
- fixed: Added a local command capability marker for fixed-/bin bare-name VFS
  pipeline explicit-fd separated redirection tokens.
- not-an-issue: The direct absolute-path explicit-fd parser and executor path
  already provided the descriptor/VFS/userspace execution, pipe handoff,
  child-only final-stage descriptor rebinding, append-at-EOF readback, and
  restoration controls needed by this slice.
- deferred: Mixed direct/bare stages, unsupported bare stage names, unsupported
  fd tokens, producer/output misuse, explicit fd input redirection, fd
  duplication/close syntax, nested/traversal paths, paths outside volatile
  /tmp, PATH/current-directory lookup, command lookup beyond bounded /bin,
  arbitrary shell grammar, persistent writable filesystem behavior, live
  networking/SSH, Pi 5 hardware/lab proof, generated-root retry, boot
  publication, and phase transition remain outside this task.

## Accepted Boundary

Accepted fixed-/bin bare-name pipeline explicit-fd separated-token witnesses:

~~~text
stdin < /etc/banner.txt | stdin 1 > /tmp/talos-pipeline-output-alpha.txt
stdin < /etc/banner.txt | stdin 1 >> /tmp/talos-pipeline-output-alpha.txt
stdin < /etc/banner.txt | stderr 2 > /tmp/talos-pipeline-error-beta.log
stdin < /etc/banner.txt | stderr 2 >> /tmp/talos-pipeline-error-beta.log
~~~

These witnesses remain backed by bounded /bin VFS lookup, descriptor-backed VFS
execution for both stages, initramfs-backed producer stdin, pipe handoff,
child-only final-stage fd1/fd2 volatile-vfs rebinding, append-at-EOF semantics,
descriptor-backed cat readback, waitpid/laststatus/pipestatus observations, and
descriptor restoration controls.

selected_next_task=phase12-local-explicit-fd-separated-redirection-token-frontier-checkpoint-20260628.

Live network/SSH remains paused. No Pi 5 hardware/lab action, boot publication,
generated-root retry, persistence, arbitrary paths, PATH/current-directory
lookup, or phase transition was performed or claimed.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet local_command_loop: passed with 887
  Talos no_std tests using QEMU 9.2.0 on PATH.
- jq empty on task-owned classification/evidence JSON: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.
