# Phase 10 combined stdin/stdout redirection core

Task: `phase10-combined-stdin-stdout-redirection-core-20260605`

## Goal

Accept the smallest useful combined-redirection command: one VFS-backed
userspace program reads from an accepted read-only fd0 source while fd1 is
redirected to a volatile `/tmp` output file.

## Scope

Accepted combined form:

- `exec stdin </etc/banner.txt >/tmp/stdin-report.txt`

The accepted form is intentionally exact. It composes the already accepted
read-only `/etc/banner.txt` stdin source with the already accepted volatile
`/tmp/<basename>` stdout truncate/create sink for the VFS-backed
`/bin/stdin` fixture.

## Findings and Disposition

- Fixed: the shell request model now carries a separate stdin redirection slot
  and an output redirection slot, while preserving the single-redirection
  behavior used by earlier stdout, stderr, dup, close, `/dev/null`, and
  pipeline controls.
- Fixed: execution applies fd0 rebinding before fd1 rebinding, launches the
  userspace program with both child-only descriptor mutations visible in the
  inheritance record, then restores fd1 and fd0 in reverse order.
- Fixed: exec summaries now emit up to two `exec-redirection` records so the
  retained evidence names fd0 and fd1 independently.
- Fixed: the QEMU serial command path exposed that the exact combined command
  did not fit the old 32-byte canonical line buffer. The canonical line
  capacity is now 64 bytes, enough for the accepted command and its negative
  controls without changing the parser's conservative grammar.
- Fixed: task-owned QEMU/substitute coverage records the positive combined
  command, descriptor-backed volatile file readback, waitpid, laststatus, and
  deterministic negatives for output-first ordering, `/dev/null` combined
  input, explicit fd1 alias in the combined form, and spaced input grammar.
- Deferred: arbitrary input paths, `/dev/null` combined input, explicit fd1
  aliases in the combined form, append combined forms, stderr combined forms,
  broader descriptor grammar, descriptor moves, multi-command redirection,
  persistent storage, recursive directories, process accounting/concurrency,
  Pi 5 proof, networking, SSH, and phase transition.

## Evidence

Primary evidence:
`tasks/evidence/2026-06-05-phase10-combined-stdin-stdout-redirection-core/qemu-local-shell-combined-stdin-stdout-redirection-smoke.log`.

Evidence level: QEMU/substitute.

The retained transcript records:

- `exec stdin </etc/banner.txt >/tmp/stdin-report.txt` dispatching through the
  accepted `/bin/stdin` VFS/open/read, loader, startup ABI, lifecycle,
  waitpid, and laststatus lineage;
- descriptor inheritance with `fd0=regular-file fd1=regular-file
  fd2=stdio-output`;
- fd0 source record:
  `exec-redirection op=source source-fd=0x0 source-path=/etc/banner.txt
  source-route=initramfs:/etc/banner.txt child-only=true
  shell-restored=true`;
- fd1 sink record:
  `exec-redirection op=sink source-fd=0x1
  target-path=/tmp/stdin-report.txt
  target-route=volatile-vfs:/tmp/stdin-report.txt child-only=true
  shell-restored=true`;
- userspace stdin report written through redirected fd1 with
  `stdout-bytes=0x3d`, then read back by descriptor-backed
  `cat /tmp/stdin-report.txt`;
- consuming `waitpid` and non-consuming `laststatus` for `/bin/stdin`;
- deterministic negatives for `exec stdin >/tmp/stdin-report.txt
  </etc/banner.txt`, `exec stdin </dev/null >/tmp/stdin-report.txt`,
  `exec stdin </etc/banner.txt 1>/tmp/stdin-report.txt`, and
  `exec stdin < /etc/banner.txt >/tmp/stdin-report.txt`;
- final `errors=0`, classification
  `qemu-local-shell-combined-stdin-stdout-redirection-complete`, and PASS.

Retained-control static inspection found PASS/classification markers in:

- `tasks/evidence/2026-06-04-phase10-readonly-regular-file-stdin-redirection-core/qemu-local-shell-readonly-regular-file-stdin-redirection-smoke.log`
- `tasks/evidence/2026-06-04-phase10-dev-null-stdin-redirection-core/qemu-local-shell-dev-null-stdin-redirection-smoke.log`
- `tasks/evidence/2026-06-04-phase10-stdout-arbitrary-tmp-output-redirection-core/qemu-local-shell-stdout-arbitrary-tmp-output-redirection-smoke.log`
- `tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log`
- `tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log`
- `tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log`
- `tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log`
- `tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log`

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute: `scripts/qemu-local-shell-combined-stdin-stdout-redirection-smoke.sh` passed and retained the primary evidence log.
- retained-control static inspection: required prior PASS/classification logs
  listed above were inspected.
- diff check: `git diff --check` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff check: `git diff --cached --check` passed before commit.

hardwareTestLock remained unlocked/restored and unused.
