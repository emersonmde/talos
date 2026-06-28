# Phase 12 Local Direct Pipeline Explicit-Fd Separated Redirection Token Core

Task:
phase12-local-direct-pipeline-explicit-fd-separated-redirection-token-core-20260628

## Goal

Accept explicit fd, operator, and path as separate tokens for the already
accepted direct absolute-path two-stage combined pipeline surfaces, without
adding bare-name pipeline support or broad shell grammar.

## Scope

- Accept only direct absolute-path two-stage pipeline witnesses where
  /bin/stdin reads initramfs:/etc/banner.txt and the final stage redirects fd1
  or fd2 to a safe volatile-vfs /tmp leaf path.
- Preserve descriptor-backed VFS executable open/read, userspace launch/status,
  pipe handoff, child-only final-stage descriptor rebinding, append-at-EOF
  semantics, descriptor-backed readback, process/status observations, and
  descriptor restoration controls.
- Keep bare-name pipeline explicit-fd separated tokens, mixed direct/bare
  stages, producer/output misuse, unsupported fd tokens, unsupported paths, and
  unsupported grammar fail-closed.

## Findings

- fixed: Added direct absolute-path pipeline consumer parsing for fd token,
  separated operator token, and path token on the accepted stdout/stderr final
  stages.
- fixed: Kept the new parser out of the fixed-/bin bare-name pipeline path, so
  bare-name pipeline explicit-fd separated tokens remain fail-closed for the
  queued successor task.
- fixed: Added a focused QEMU/substitute local_command_loop regression covering
  stdout and stderr truncate/append witnesses, descriptor readbacks, direct
  no-fd separated-token controls, wrong fd/output pairings, missing operands,
  unsupported paths, mixed stages, bare-name pipeline attempts, and producer
  misuse.
- fixed: Added a local command capability marker for direct absolute-path VFS
  pipeline explicit-fd separated redirection tokens.
- not-an-issue: The accepted output path policy remains the previously accepted
  safe volatile /tmp leaf policy; exact task witnesses are additionally bounded
  by the direct combined-pipeline recognizers.
- deferred: Fixed-/bin bare-name pipeline explicit-fd separated tokens, explicit
  fd input redirection, fd duplication/close syntax, nested/traversal paths,
  paths outside volatile /tmp, PATH/current-directory lookup, command lookup
  beyond bounded /bin, arbitrary shell grammar, persistent writable filesystem
  behavior, generated-root retry, boot publication, live networking/SSH, Pi 5
  hardware action, and phase transition remain outside this task.

## Accepted Boundary

Accepted direct absolute-path pipeline explicit-fd separated-token witnesses:

~~~text
/bin/stdin < /etc/banner.txt | /bin/stdin 1 > /tmp/talos-pipeline-output-alpha.txt
/bin/stdin < /etc/banner.txt | /bin/stdin 1 >> /tmp/talos-pipeline-output-alpha.txt
/bin/stdin < /etc/banner.txt | /bin/stderr 2 > /tmp/talos-pipeline-error-beta.log
/bin/stdin < /etc/banner.txt | /bin/stderr 2 >> /tmp/talos-pipeline-error-beta.log
~~~

These witnesses remain backed by descriptor/VFS/userspace execution for both
stages, initramfs-backed producer stdin, pipe handoff, child-only final-stage
fd1/fd2 volatile-vfs rebinding, append-at-EOF semantics, and descriptor-backed
cat readback.

selected_next_task=phase12-local-bare-name-pipeline-explicit-fd-separated-redirection-token-core-20260628.

Live network/SSH remains paused. No Pi 5 hardware/lab action, boot publication,
generated-root retry, persistence, arbitrary paths, PATH/current-directory
lookup, or phase transition was performed or claimed.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet local_command_loop: passed with 886
  Talos no_std tests using QEMU 9.2.0 on PATH.
- jq empty on task-owned classification/evidence JSON: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.
