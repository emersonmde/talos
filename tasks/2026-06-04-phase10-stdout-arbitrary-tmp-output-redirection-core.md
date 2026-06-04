# Phase 10 stdout arbitrary /tmp output redirection core

Task: `phase10-stdout-arbitrary-tmp-output-redirection-core-20260604`

## Goal

Broaden stdout regular-file redirection from the exact `/tmp/stdout.txt`
target to conservative volatile `/tmp` basename output paths while preserving
the accepted descriptor-backed VFS/userspace path.

## Scope

Accepted stdout forms:

- `exec stdout >/tmp/<basename>`
- `exec stdout >>/tmp/<basename>`
- `exec stdout 1>/tmp/<basename>`
- `exec stdout 1>>/tmp/<basename>`

The basename grammar is intentionally conservative: non-empty names under
`/tmp/` made of ASCII letters, digits, `.`, `_`, and `-`. Nested paths, path
traversal, outside-`/tmp` paths, empty basenames, unsupported fd numbers, and
the reserved stderr scratch name `/tmp/stderr.txt` remain rejected.

## Findings and Disposition

- Fixed: stdout redirection records now carry a bounded volatile path/route
  field so logs can report the actual `target-path=/tmp/<basename>` and
  `target-route=volatile-vfs:/tmp/<basename>`.
- Fixed: stdout volatile scratch storage now records the active accepted path
  and descriptor-backed `cat` reads require the matching path.
- Fixed: the parser accepts the four stdout/fd1 truncate and append spellings
  for conservative `/tmp` basenames.
- Fixed: QEMU/substitute scenario and wrapper cover all accepted forms plus
  deterministic negative path/fd cases.
- Deferred: stderr arbitrary paths, input arbitrary paths, persistent storage,
  broad writable filesystem mutation, descriptor moves, recursive directories,
  process accounting/concurrency, Pi 5 proof, networking, SSH, and phase
  transition.

## Evidence

Primary evidence: `tasks/evidence/2026-06-04-phase10-stdout-arbitrary-tmp-output-redirection-core/qemu-local-shell-stdout-arbitrary-tmp-output-redirection-smoke.log`.

Evidence level: QEMU/substitute.

The retained transcript records:

- truncate/create: `exec stdout >/tmp/alpha.log` with
  `target-route=volatile-vfs:/tmp/alpha.log`, userspace TalosWrite
  provenance, descriptor-backed readback, waitpid, and laststatus;
- append/create: `exec stdout >>/tmp/beta.out` with
  `target-route=volatile-vfs:/tmp/beta.out` and descriptor-backed readback;
- explicit fd1 truncate: `exec stdout 1>/tmp/gamma.log` with matching route
  and readback;
- explicit fd1 append: `exec stdout 1>>/tmp/delta.out` with matching route
  and readback;
- shell fd1 restoration through a following normal `exec stdout` routed to
  `runtime-console0/stdout`;
- negatives for `/var/out.txt`, `/tmp/nested/out.txt`, `/tmp/`,
  `3>/tmp/alpha.log`, `/tmp/../bad.txt`, and `/tmp/stderr.txt`;
- final `errors=0`, classification
  `qemu-local-shell-stdout-arbitrary-tmp-output-redirection-complete`, and
  PASS.

Retained-control static inspection: exact stdout/stderr file redirection,
append/create, explicit fd1, `/dev/null`, read-only stdin redirection,
descriptor/pipeline controls, VFS exec/open/read/write, lifecycle/status,
waitpid, laststatus, and descriptor-backed cat evidence remain available from
the prior accepted task records and evidence logs.

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute: `scripts/qemu-local-shell-stdout-arbitrary-tmp-output-redirection-smoke.sh --quiet` passed and retained the primary evidence log.
- retained-control static inspection: prior accepted PASS logs listed above
  were reviewed for unchanged control coverage.
- diff check: `git diff --check` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff check: `git diff --cached --check` passed before commit.

hardwareTestLock remained unlocked/restored and unused.
