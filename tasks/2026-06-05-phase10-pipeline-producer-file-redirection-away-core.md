# Phase 10 pipeline producer file-redirection-away core

Task: `phase10-pipeline-producer-file-redirection-away-core-20260605`

## Goal

Accept the smallest inverse pipeline/file-redirection composition where the
pipeline producer redirects fd1 to a volatile VFS file instead of the pipe, so
the downstream consumer observes deterministic pipe EOF/no-data.

Accepted shell-visible form:

- `exec stdout >/tmp/pipe-source.txt | exec stdin`

## Scope

- Route the producer and consumer through the accepted fixed `/bin` lookup,
  VFS/open/read, loader, launch, lifecycle, waitpid, and laststatus lineage.
- Prove producer fd1 regular-file redirection overrides the installed pipe
  writer for the producer child only.
- Read the redirected producer payload back through descriptor-backed
  `cat /tmp/pipe-source.txt`.
- Retain the accepted positive pipeline transfer, descriptor-mixing redirect
  away, arbitrary `/tmp` output redirection, waitpid, laststatus, and cat
  controls.

## Non-goals

- Do not add general pipeline redirection grammar, multi-stage pipelines,
  process concurrency, pipefail, jobs, fork/signals, arbitrary descriptor
  syntax, persistence, broader output path policy, Pi 5 proof, networking, SSH,
  or a phase transition.

## Findings and Disposition

- Fixed: `exec stdout >/tmp/pipe-source.txt | exec stdin` is now accepted only
  for `/bin/stdout` producer fd1 truncate/create redirection with an unredirected
  `/bin/stdin` consumer.
- Fixed: the pipeline record now distinguishes this exact behavior with
  `source=shell-pipe-producer-file-redirection-away`; bytes written/read are
  both zero because producer fd1 targets `volatile-vfs:/tmp/pipe-source.txt`
  instead of the pipe.
- Fixed: the task-owned QEMU scenario, wrapper script, command-count table,
  label/classification wiring, and expected dispatch table now cover the
  accepted command and deterministic negatives.
- Fixed: a first QEMU attempt exposed incomplete kernel-side label and
  classification wiring for the new scenario; the feature transcript was
  present, but the run was classified as the generic serial loop. The harness
  wiring was completed and the task-owned smoke then passed.
- Deferred: producer append redirection in a pipeline, stderr producer file
  redirection, producer and consumer file redirection in the same pipeline,
  multi-stage/concurrent pipelines, process accounting/concurrency, arbitrary
  descriptor syntax, persistence, Pi 5 proof, networking, SSH, and phase
  transition.
- Not-an-issue: the accepted behavior composes existing child-only stdout file
  redirection and accepted pipe setup ordering; no new ordering or path-policy
  ADR is required.

## Evidence

Primary QEMU/substitute evidence:

- `tasks/evidence/2026-06-05-phase10-pipeline-producer-file-redirection-away-core/qemu-local-shell-pipeline-producer-file-redirection-away-smoke.log`

The retained log records:

- `exec stdout >/tmp/pipe-source.txt | exec stdin`;
- producer fd1 as `regular-file`, with
  `target-route=volatile-vfs:/tmp/pipe-source.txt`;
- `exec-stdout ... stream=regular-file route=volatile-vfs:/tmp/pipe-source.txt`;
- the pipeline record with `bytes-written=0`, `bytes-read=0`,
  `reader-eof=true`, `shell-restored=true`, and
  `source=shell-pipe-producer-file-redirection-away`;
- consumer stdin read result `pipe-eof/no-data`;
- descriptor-backed `cat /tmp/pipe-source.txt` reading 0x1f bytes;
- retained positive `exec stdout | exec stdin` pipeline transfer control;
- deterministic negatives for producer append, stderr producer file
  redirection, and producer+consumer file redirection;
- `errors=0`,
  `classification=qemu-local-shell-pipeline-producer-file-redirection-away-complete`,
  and `PASS`.

Retained control evidence inspected:

- `tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log`
- `tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log`
- `tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log`
- `tasks/evidence/2026-06-04-phase10-stdout-arbitrary-tmp-output-redirection-core/qemu-local-shell-stdout-arbitrary-tmp-output-redirection-smoke.log`
- `tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log`
- `tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log`

## Validation

- fmt/lint: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet` passed.
- QEMU/substitute:
  `scripts/qemu-local-shell-pipeline-producer-file-redirection-away-smoke.sh`
  passed and retained the primary evidence log.
- static inspection: retained control evidence logs listed above include PASS
  markers and expected pipeline/output-redirection/waitpid/laststatus/cat
  markers.
- static inspection: retained-control grep check passed for primary and
  historical control logs.
- diff hygiene: `git diff --check` passed.
- docs validation: `/home/node/.cargo/bin/mdbook build` passed.
- staged diff hygiene: `git diff --cached --check` passed.

Implementation/evidence commit:
`ccdb07d8a7297b90ca2770c7d8fa9478839c2274`.
