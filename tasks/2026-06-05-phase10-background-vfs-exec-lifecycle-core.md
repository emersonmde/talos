# Phase 10 background VFS exec lifecycle core

Task: `phase10-background-vfs-exec-lifecycle-core-20260605`

## Goal

Accept the thinnest real background execution/accounting slice: exact
shell-visible `exec /bin/status42 &` launches through the accepted VFS-backed
exec path, reports a background job record, leaves the shell responsive, and
observes/reaps completion on the next command without corrupting foreground
`waitpid` or `laststatus`.

Accepted shell-visible form:

- `exec /bin/status42 &`

## Scope

- Route the background launch through the accepted fixed `/bin` lookup,
  VFS/open/read, loader, launch, descriptor inheritance, startup ABI, and
  userspace status lineage.
- Add one minimal shell-owned background job record for the accepted exact form.
- Report a running job at launch and a completed/reaped job on the next command.
- Keep the foreground lifecycle record and waitable child state unchanged by the
  background launch.
- Retain pipeline, file redirection, `waitpid`, `laststatus`, `stdio`, and
  descriptor-backed `cat` controls.

## Non-goals

- Do not add POSIX job control, process groups, signals, true concurrent
  scheduler execution, multiple background jobs, `jobs`/`fg`/`bg`, PATH
  lookup, arbitrary async syntax, background pipelines/redirections,
  persistence, Pi 5 proof, networking, SSH, or a phase transition.

## Findings and Disposition

- Fixed: `exec /bin/status42 &` now reports
  `source=vfs-open-read mode=background` while preserving the accepted VFS
  loader/launch/descriptor/startup evidence for `/bin/status42`.
- Fixed: the shell records exactly one background job with a stable job id, pid,
  command path, pending/running state, and `shell-responsive=true`; the next
  command observes completion, status `0x2a`, and `reaped=true`.
- Fixed: background completion does not overwrite the foreground
  `laststatus` or `waitpid` state. The task-owned transcript records
  `waitpid no-child` and `last-process none` immediately after the
  background completion, followed by normal foreground `/bin/zero`
  `waitpid`/`laststatus` controls.
- Fixed: the command-loop boundary advertises
  `background-vfs-exec-lifecycle`, and the QEMU serial smoke has a dedicated
  label, classification, command table, wrapper script, and content assertions.
- Fixed: stale local tests still encoded the previous 32-byte canonical line
  capacity. They now build truncation cases from `CANONICAL_LINE_CAPACITY`
  rather than hard-coded old lengths, and one parser-negative pipeline test uses
  shorter `/tmp` names so it reaches parser rejection instead of the line
  limit.
- Deferred: general async execution, multiple jobs, background redirection,
  background pipelines, POSIX job-control commands, process groups, signals,
  fork/scheduler concurrency, broad process table policy, Pi 5 proof,
  networking, SSH, and phase transition.
- Not-an-issue: this slice intentionally observes completion on the next shell
  command rather than accepting true scheduler-concurrent user processes; the
  output labels the behavior as background accounting and keeps broader process
  control deferred.

## Evidence

Primary QEMU/substitute evidence:

- `tasks/evidence/2026-06-05-phase10-background-vfs-exec-lifecycle-core/qemu-local-shell-background-vfs-exec-lifecycle-smoke.log`

The retained log records:

- `exec /bin/status42 &`;
- `talos: exec path=/bin/status42 source=vfs-open-read mode=background`;
- accepted `exec-source`, `exec-loader`, `exec-launch`,
  `exec-descriptors`, and `exec-startup-abi` lines for `/bin/status42`;
- `background-job ... state=running reaped=false status=pending
  shell-responsive=true source=background-vfs-exec-accounting`;
- a following `cat /etc/banner.txt` command that first reports
  `background-job ... state=completed status=0x2a observed-status=0x2a
  reaped=true shell-responsive=observed`, then prints the descriptor-backed
  initramfs banner;
- foreground `waitpid no-child` and `last-process none` after the background
  completion;
- a subsequent foreground `exec /bin/zero` with normal `waitpid` and
  `laststatus`;
- retained `exec stdout | exec stdin` pipeline transfer and descriptor-backed
  `cat /etc/banner.txt` controls;
- deterministic negative controls for `exec /bin/status42&` and
  `exec stdout &`;
- `errors=0`,
  `classification=qemu-local-shell-background-vfs-exec-lifecycle-complete`,
  and `PASS`.

Retained control evidence inspected:

- `tasks/evidence/2026-06-05-phase10-background-vfs-exec-lifecycle-core/retained-control-inspection.txt`
- `tasks/evidence/2026-06-05-phase10-combined-stdin-stdout-redirection-core/qemu-local-shell-combined-stdin-stdout-redirection-smoke.log`
- `tasks/evidence/2026-06-05-phase10-pipeline-consumer-output-redirection-core/qemu-local-shell-pipeline-consumer-output-redirection-smoke.log`
- `tasks/evidence/2026-06-05-phase10-pipeline-producer-file-redirection-away-core/qemu-local-shell-pipeline-producer-file-redirection-away-smoke.log`
- `tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log`
- `tasks/evidence/2026-06-04-phase10-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log`

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute:
  `scripts/qemu-local-shell-background-vfs-exec-lifecycle-smoke.sh --quiet`
  passed and retained the primary evidence log.
- static inspection: retained-control grep check passed for primary and
  historical control logs.
- docs validation: `/home/node/.cargo/bin/mdbook build` passed.
- diff hygiene: `git diff --check` passed.
- staged diff hygiene: `git diff --cached --check` passed.

Implementation/evidence commit: recorded in durable supervisor state after
commit.
