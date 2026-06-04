# Phase 10 Stdout Regular-File Append Redirection Core

Task: phase10-stdout-regular-file-append-redirection-core-20260604
Status: accepted

## Scope

Accept exactly one append-output redirection sequence for stdout: first 'exec stdout >/tmp/stdout.txt' creates/truncates the accepted volatile scratch file, then 'exec stdout >>/tmp/stdout.txt' appends a second VFS-backed '/bin/stdout' fixture write to the existing file. 'cat /tmp/stdout.txt' must read both fixture payloads in order through the volatile VFS descriptor-read path.

This is not missing-file append-create, stderr append, arbitrary paths, persistence, partial overwrite, broad writable filesystem mutation, descriptor moves, here-docs, multi-stage or concurrent pipelines, jobs, signals, fork, async execution, networking, SSH, Pi 5 proof, or a phase transition.

## Findings

- fixed: Added 'StdoutAppendTmpStdout' as the exact '>>/tmp/stdout.txt' fd1 append form. Parsing accepts only the no-space token and execution restricts it to the VFS-backed '/bin/stdout' fixture.
- fixed: Append redirection requires an existing '/tmp/stdout.txt'; missing-file append remains a deterministic 'exec-invalid-path' negative rather than silently creating a file.
- fixed: Reused the volatile scratch regular-file descriptor and userspace 'TalosWrite' path without truncating the existing file for append, so the existing in-memory file length is preserved and the second payload is written after the first.
- fixed: The append child descriptor table reports 'fd1=regular-file'; the redirection record reports 'op=append', 'target-path=/tmp/stdout.txt', 'target-stream=regular-file', 'target-route=volatile-vfs:/tmp/stdout.txt', and 'source=shell-redirection-stdout-tmp-stdout-append'.
- fixed: The descriptor-backed 'cat /tmp/stdout.txt' readback reports 'bytes=0x3e' and 'source=volatile-vfs-descriptor-read', proving two 0x1f userspace stdout payloads were retained in order.
- fixed: Added no_std unit coverage, a dedicated QEMU/substitute wrapper and boot scenario, expected dispatch rows, harness grep assertions, and task-owned retained evidence for the append sequence and deterministic negatives.
- deferred: stderr append, append to missing files, arbitrary append paths, persistent storage, partial overwrite, wider writable filesystem behavior, metadata, directory mutation, broader descriptor grammar, Pi 5 proof, networking, SSH, and phase transition.

## Evidence

- QEMU/substitute task smoke: tasks/evidence/2026-06-04-phase10-stdout-regular-file-append-redirection-core/qemu-local-shell-stdout-regular-file-append-redirection-smoke.log records 'exec stdout >/tmp/stdout.txt', 'exec-redirection op=sink', 'target-path=/tmp/stdout.txt', 'target-route=volatile-vfs:/tmp/stdout.txt', then 'exec stdout >>/tmp/stdout.txt', 'exec-redirection op=append', 'target-path=/tmp/stdout.txt', 'target-route=volatile-vfs:/tmp/stdout.txt', fd1 regular-file inheritance, userspace 'TalosWrite' provenance, waitpid, laststatus, descriptor-backed 'cat /tmp/stdout.txt' with 'bytes=0x3e source=volatile-vfs-descriptor-read', normal 'exec stdout' restoration through 'runtime-console0/stdout', unsupported append variants, descriptor-backed 'cat /etc/banner.txt', final 'qemu-local-shell-stdout-regular-file-append-redirection-complete', and PASS.
- Retained stdout truncate/create control: tasks/evidence/2026-06-04-phase10-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log.
- Retained stderr regular-file and distinct-stream controls: tasks/evidence/2026-06-04-phase10-stderr-regular-file-redirection-core/qemu-local-shell-stderr-regular-file-redirection-smoke.log and tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log.
- Retained read-only input redirection controls: tasks/evidence/2026-06-04-phase10-dev-null-stdin-redirection-core/qemu-local-shell-dev-null-stdin-redirection-smoke.log and tasks/evidence/2026-06-04-phase10-readonly-regular-file-stdin-redirection-core/qemu-local-shell-readonly-regular-file-stdin-redirection-smoke.log.
- Retained /dev/null and descriptor redirection/pipeline controls: tasks/evidence/2026-06-04-phase10-dev-null-stdout-redirection-contract-core/qemu-local-shell-dev-null-stdout-redirection-smoke.log, tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-dev-null-stderr-redirection-smoke.log, tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log, tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log, tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log, and tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log.
- Retained lifecycle/status, waitpid, laststatus, and descriptor-backed cat controls: tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log, tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log, and tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet local_command_loop passed after adding the append unit test.
- full unit tests: cargo -Zjson-target-spec test --quiet passed 408 no_std tests.
- QEMU/substitute: scripts/qemu-local-shell-stdout-regular-file-append-redirection-smoke.sh --quiet passed with retained PASS log.
- docs: /home/node/.cargo/bin/mdbook build passed after the task record and roadmap update.
- diff check: git diff --check passed.
- staged diff check: git diff --cached --check passed.

## Commit

Commit: accepted implementation and evidence pending final validation/commit; final SHA will be recorded in durable supervisor state.
