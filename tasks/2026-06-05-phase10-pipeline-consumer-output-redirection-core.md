# Phase 10 pipeline consumer-output redirection core

Task: `phase10-pipeline-consumer-output-redirection-core-20260605`

Status: accepted

## Goal

Accept the smallest pipeline-plus-file-redirection composition where the
pipeline consumer reads from the accepted pipe-backed fd0 and writes its
stdout report to a child-only volatile `/tmp` file descriptor.

## Scope

Accepted form:

- `exec stdout | exec stdin >/tmp/pipe-consumer.txt`

The producer remains the accepted VFS-backed `/bin/stdout` fixture. The
consumer remains the accepted VFS-backed `/bin/stdin` fixture. Only the
consumer fd1 is redirected to an accepted volatile VFS stdout sink; the pipe
still connects producer fd1 to consumer fd0.

## Findings and Disposition

- Fixed: pipeline execution now allows a consumer stdout truncate/create sink
  only for the exact `/bin/stdout | /bin/stdin >/tmp/<basename>` composition.
  The policy is not enabled for standalone `exec stdin >/tmp/...` or for
  stderr/append/producer-redirection combinations.
- Fixed: the task-owned transcript records a new pipeline source marker,
  `shell-pipe-consumer-stdout-redirection`, so retained evidence distinguishes
  this composition from the plain stdout-to-stdin pipeline.
- Fixed: QEMU boot-scenario registration, target-side expected dispatch, and
  the serial smoke wrapper now cover the exact composition and deterministic
  negatives.
- Fixed: the first smoke attempt used an overlong negative command and hit the
  64-byte line boundary as `input-error` rather than parser rejection. The
  retained smoke uses a shorter producer-file plus consumer-file negative so
  command 10 proves `unexpected-argument` without line truncation.
- Deferred: append consumer output, stderr-producing pipelines with consumer
  output redirection, producer file redirection combined with consumer output
  redirection, multi-stage/concurrent pipelines, pipefail, jobs, fork/signals,
  arbitrary descriptor syntax, persistence, recursive directories, process
  accounting/concurrency, Pi 5 proof, networking, SSH, and phase transition.

## Evidence

Primary evidence:
`tasks/evidence/2026-06-05-phase10-pipeline-consumer-output-redirection-core/qemu-local-shell-pipeline-consumer-output-redirection-smoke.log`.

Evidence level: QEMU/substitute.

The retained transcript records:

- `exec stdout | exec stdin >/tmp/pipe-consumer.txt` dispatching through the
  accepted `/bin/stdout` and `/bin/stdin` VFS/open/read, loader, startup ABI,
  lifecycle, waitpid, and laststatus lineage;
- producer fd1 as `pipe-endpoint`, `exec-stdout` writing 0x1f bytes to
  `stream=pipe-writer route=pipe:stdout-to-stdin`, and the pipeline record
  reporting `bytes-written=0x1f bytes-read=0x1f writer-closed=true
  reader-eof=true shell-restored=true source=shell-pipe-consumer-stdout-redirection`;
- consumer fd0 as `pipe-endpoint`, fd1 as `regular-file`, and a child-only
  stdout sink record for `/tmp/pipe-consumer.txt`;
- the consumer `/bin/stdin` report written to the volatile file and read back
  with descriptor-backed `cat /tmp/pipe-consumer.txt` at 0x44 bytes;
- consuming `waitpid` and non-consuming `laststatus` for the consumer
  `/bin/stdin` lifecycle record;
- retained plain pipeline control `exec stdout | exec stdin` with
  `source=shell-pipe-stdout-to-stdin`;
- deterministic negatives for consumer append redirection, stderr producer
  with consumer output redirection, and producer file redirection combined
  with consumer output redirection;
- final `errors=0`, classification
  `qemu-local-shell-pipeline-consumer-output-redirection-complete`, and PASS.

Retained-control static inspection found PASS/classification markers in:

- `tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log`
- `tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log`
- `tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log`
- `tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log`
- `tasks/evidence/2026-06-04-phase10-stdout-arbitrary-tmp-output-redirection-core/qemu-local-shell-stdout-arbitrary-tmp-output-redirection-smoke.log`
- `tasks/evidence/2026-06-04-phase10-stderr-arbitrary-tmp-output-redirection-core/qemu-local-shell-stderr-arbitrary-tmp-output-redirection-smoke.log`
- `tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log`
- `tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log`
- `tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log`

## Validation

- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute: `scripts/qemu-local-shell-pipeline-consumer-output-redirection-smoke.sh` passed and retained the primary evidence log.
- retained-control static inspection: required prior PASS/classification logs
  listed above were inspected.
- diff check: `git diff --check` passed.
- docs: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff check: `git diff --cached --check` passed before commit.

hardwareTestLock remained unlocked/restored and unused.
