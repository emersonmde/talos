# Phase 10 stderr arbitrary /tmp output redirection core

Task: `phase10-stderr-arbitrary-tmp-output-redirection-core-20260604`

## Goal

Broaden stderr regular-file redirection from the exact `/tmp/stderr.txt`
target to conservative volatile `/tmp` basename output paths while preserving
the accepted descriptor-backed VFS/userspace path.

## Scope

Accepted stderr forms:

- `exec stderr 2>/tmp/<basename>`
- `exec stderr 2>>/tmp/<basename>`

The basename grammar is intentionally conservative: non-empty names under
`/tmp/` made of ASCII letters, digits, `.`, `_`, and `-`. Nested paths,
path traversal, outside-`/tmp` paths, empty basenames, unsupported fd numbers,
fd2 shorthand without `2>`, and the reserved stdout scratch name
`/tmp/stdout.txt` remain rejected.

## Findings and Disposition

- Fixed: stderr redirection records now carry a bounded volatile path/route
  field so logs can report the actual `target-path=/tmp/<basename>` and
  `target-route=volatile-vfs:/tmp/<basename>`.
- Fixed: stderr volatile scratch storage now records the active accepted path
  and descriptor-backed `cat` reads require the matching path.
- Fixed: the parser accepts stderr `2>` and `2>>` for conservative
  `/tmp` basenames.
- Fixed: `cat /tmp/<basename>` now checks volatile stdout storage and then
  volatile stderr storage by path, preserving stdout arbitrary controls while
  enabling stderr readback.
- Fixed: QEMU/substitute scenario and wrapper cover accepted truncate and
  append forms plus deterministic negative path/fd/mixup cases.
- Removed: stale fixed-path-only stderr target/readback helpers and stale
  unsupported-test assumptions made obsolete by arbitrary stderr paths.
- Deferred: input arbitrary paths, persistent storage, broad writable
  filesystem mutation, descriptor moves, recursive directories, process
  accounting/concurrency, Pi 5 proof, networking, SSH, and phase transition.

## Evidence

Primary evidence: `tasks/evidence/2026-06-04-phase10-stderr-arbitrary-tmp-output-redirection-core/qemu-local-shell-stderr-arbitrary-tmp-output-redirection-smoke.log`.

Evidence level: QEMU/substitute.

The retained transcript records:

- truncate/create: `exec stderr 2>/tmp/omega.err` with
  `target-route=volatile-vfs:/tmp/omega.err`, userspace TalosWrite
  provenance, descriptor-backed readback, waitpid, and laststatus;
- append/create: `exec stderr 2>>/tmp/theta.log` with
  `target-route=volatile-vfs:/tmp/theta.log` and descriptor-backed readback;
- shell fd2 restoration through a following normal `exec stderr` routed to
  `runtime-console0/stderr`;
- stdout separation through a following normal `exec stdout` routed to
  `runtime-console0/stdout`;
- negatives for `/var/err.txt`, nested `/tmp/n/e`, `/tmp/`,
  `3>/tmp/omega.err`, `/tmp/../bad.txt`, `/tmp/stdout.txt`, and fd2
  shorthand `>/tmp/misbound.err`;
- final `errors=0`, classification
  `qemu-local-shell-stderr-arbitrary-tmp-output-redirection-complete`, and
  PASS.

Retained-control static inspection: stdout arbitrary `/tmp`, exact
stdout/stderr file redirection, append/create, explicit fd1, `/dev/null`,
read-only stdin redirection, descriptor/pipeline controls, VFS
exec/open/read/write, lifecycle/status, waitpid, laststatus, and
descriptor-backed cat evidence remain available from the prior accepted task
records and evidence logs.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute: `scripts/qemu-local-shell-stderr-arbitrary-tmp-output-redirection-smoke.sh --quiet` passed and retained the primary evidence log.
- retained-control static inspection: prior accepted PASS logs listed above
  were reviewed for unchanged control coverage.
- diff check: `git diff --check` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff check: `git diff --cached --check` passed before commit.

No new ADR is required because this reuses the accepted conservative volatile
`/tmp` basename output path policy and does not widen the grammar.

hardwareTestLock remained unlocked/restored and unused.
