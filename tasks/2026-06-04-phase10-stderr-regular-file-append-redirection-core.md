# Phase 10 Stderr Regular-File Append Redirection Core

Task: phase10-stderr-regular-file-append-redirection-core-20260604
Status: accepted

## Scope

Accept exactly one append-output redirection sequence for stderr: first 'exec stderr 2>/tmp/stderr.txt' creates/truncates the accepted volatile scratch file, then 'exec stderr 2>>/tmp/stderr.txt' appends a second VFS-backed '/bin/stderr' fixture write to the existing file. 'cat /tmp/stderr.txt' must read both fixture payloads in order through the volatile VFS descriptor-read path while normal stderr and stdout remain distinct after restoration.

This is not missing-file append-create, stdout append expansion, arbitrary paths, persistence, partial overwrite, broad writable filesystem mutation, descriptor moves, here-docs, multi-stage or concurrent pipelines, jobs, signals, fork, async execution, networking, SSH, Pi 5 proof, or a phase transition.

## Findings

- fixed: Added 'StderrAppendTmpStderr' as the exact '2>>/tmp/stderr.txt' fd2 append form. Parsing accepts only the no-space token and execution restricts it to the VFS-backed '/bin/stderr' fixture.
- fixed: Append redirection requires an existing '/tmp/stderr.txt'; missing-file append remains a deterministic 'exec-invalid-path' negative rather than creating the file.
- fixed: Reused the volatile stderr scratch regular-file descriptor and userspace 'TalosWrite' path without truncating the existing file for append, preserving the first stderr payload before writing the second.
- fixed: The append child descriptor table reports 'fd2=regular-file'; the redirection record reports 'op=append', 'target-path=/tmp/stderr.txt', 'target-stream=regular-file', 'target-route=volatile-vfs:/tmp/stderr.txt', and 'source=shell-redirection-stderr-tmp-stderr-append'.
- fixed: The descriptor-backed 'cat /tmp/stderr.txt' readback reports 'bytes=0x3e' and 'source=volatile-vfs-descriptor-read', proving two 0x1f userspace stderr payloads were retained in order.
- fixed: The QEMU/substitute smoke now proves later normal 'exec stderr' routes through 'runtime-console0/stderr' and normal 'exec stdout' routes through 'runtime-console0/stdout' after the append child exits.
- fixed: Added no_std unit coverage, a dedicated QEMU/substitute wrapper and boot scenario, expected dispatch rows, harness grep assertions, and task-owned retained evidence for the append sequence and deterministic negatives.
- deferred: append to missing files, arbitrary append paths, stdout-to-stderr path mixups, persistent storage, partial overwrite, wider writable filesystem behavior, metadata, directory mutation, broader descriptor syntax, Pi 5 proof, networking, SSH, and phase transition.

## Evidence

- QEMU/substitute task smoke: tasks/evidence/2026-06-04-phase10-stderr-regular-file-append-redirection-core/qemu-local-shell-stderr-regular-file-append-redirection-smoke.log records 'exec stderr 2>/tmp/stderr.txt', 'exec-redirection op=sink', 'target-path=/tmp/stderr.txt', 'target-route=volatile-vfs:/tmp/stderr.txt', then 'exec stderr 2>>/tmp/stderr.txt', 'exec-redirection op=append', 'target-path=/tmp/stderr.txt', 'target-route=volatile-vfs:/tmp/stderr.txt', fd2 regular-file inheritance, userspace 'TalosWrite' provenance, waitpid, laststatus, descriptor-backed 'cat /tmp/stderr.txt' with 'bytes=0x3e source=volatile-vfs-descriptor-read', normal 'exec stderr' restoration through 'runtime-console0/stderr', normal 'exec stdout' through 'runtime-console0/stdout', unsupported append variants, descriptor-backed 'cat /etc/banner.txt', final 'qemu-local-shell-stderr-regular-file-append-redirection-complete', and PASS.
- Retained stdout append/truncate controls: tasks/evidence/2026-06-04-phase10-stdout-regular-file-append-redirection-core/qemu-local-shell-stdout-regular-file-append-redirection-smoke.log and tasks/evidence/2026-06-04-phase10-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log.
- Retained stderr truncate control: tasks/evidence/2026-06-04-phase10-stderr-regular-file-redirection-core/qemu-local-shell-stderr-regular-file-redirection-smoke.log.
- Retained read-only input redirection controls: tasks/evidence/2026-06-04-phase10-dev-null-stdin-redirection-core/qemu-local-shell-dev-null-stdin-redirection-smoke.log and tasks/evidence/2026-06-04-phase10-readonly-regular-file-stdin-redirection-core/qemu-local-shell-readonly-regular-file-stdin-redirection-smoke.log.
- Retained /dev/null and descriptor redirection/pipeline controls: tasks/evidence/2026-06-04-phase10-dev-null-stdout-redirection-contract-core/qemu-local-shell-dev-null-stdout-redirection-smoke.log, tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-dev-null-stderr-redirection-smoke.log, tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log, tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log, tasks/evidence/2026-06-04-phase10-minimal-stdout-to-stdin-pipe-core/qemu-local-shell-minimal-stdout-to-stdin-pipeline-smoke.log, tasks/evidence/2026-06-04-phase10-pipeline-stderr-not-piped-core/qemu-local-shell-pipeline-stderr-not-piped-smoke.log, tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log, and tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log.
- Retained lifecycle/status, waitpid, laststatus, and descriptor-backed cat controls: tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log, tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log, and tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet local_command_loop_appends_child_stderr_to_existing_volatile_regular_file passed 409 no_std tests, including the new append unit test and retained local-command-loop controls.
- QEMU/substitute: scripts/qemu-local-shell-stderr-regular-file-append-redirection-smoke.sh --quiet passed with retained PASS log.
- retained controls: static inspection confirmed the retained QEMU/substitute control evidence logs listed above all contain PASS.
- full unit tests: cargo -Zjson-target-spec test --quiet passed 409 no_std tests.
- docs: /home/node/.cargo/bin/mdbook build passed after the task record and roadmap update.
- diff check: git diff --check passed.
- staged diff check: git diff --cached --check passed.

## Commit

Commit: accepted implementation and evidence pending final validation/commit; final SHA will be recorded in durable supervisor state.
