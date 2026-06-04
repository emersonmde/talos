# Phase 10 Stdout Regular-File Redirection Core

Task: phase10-stdout-regular-file-redirection-core-20260604
Status: accepted

## Scope

Accept exactly one regular-file output redirection form: exec stdout >/tmp/stdout.txt. The launched VFS-backed /bin/stdout child gets fd1 rebound to a minimal volatile regular-file descriptor, writes the userspace stdout fixture bytes through the descriptor path, closes/restores fd1 after exit, and exposes the captured bytes through cat /tmp/stdout.txt.

This is not persistent storage, stderr file redirection, append, arbitrary paths, directory mutation, unlink/rename/mkdir, permissions, timestamps, fsync, descriptor moves, broader descriptor syntax, here-docs, pipe expansion, jobs, signals, networking, SSH, Pi 5 proof, or a phase transition.

## Findings

- fixed: Added StdoutToTmpStdout as the exact >/tmp/stdout.txt fd1 sink form. Parsing accepts only the no-space token, and execution restricts it to the VFS-backed /bin/stdout fixture.
- fixed: Added a tiny volatile scratch-file state for /tmp/stdout.txt with create/truncate semantics, a regular-file descriptor object, descriptor write from userspace memory, close, reopen for readback, and descriptor readback through cat /tmp/stdout.txt.
- fixed: The child descriptor table reports fd1=regular-file; the redirection record reports op=sink, target-path=/tmp/stdout.txt, target-stream=regular-file, and target-route=volatile-vfs:/tmp/stdout.txt.
- fixed: The userspace stdout fixture writes exactly 0x1f bytes through fd1 and records stream=regular-file, route=volatile-vfs:/tmp/stdout.txt, and source=userspace-talos-write.
- fixed: A following cat /tmp/stdout.txt observes the captured Talos userspace stdout fixture bytes and records path, byte count, and provenance with source=volatile-vfs-descriptor-read. A later normal exec stdout writes to runtime-console0/stdout, proving shell fd1 restoration.
- fixed: Added deterministic negatives for append-like exec stdout >>/tmp/stdout.txt, arbitrary target exec stdout >/tmp/other.txt, and stderr regular-file output exec stderr 2>/tmp/stdout.txt.
- fixed: Added no_std unit coverage, a dedicated QEMU/substitute wrapper, kernel boot scenario label/classification, expected dispatch rows, and task-owned retained evidence.
- deferred: stderr regular-file redirection, append/truncate variants beyond this exact create/truncate form, arbitrary paths, persistent/wider writable filesystem behavior, metadata, directory mutation, broader descriptor syntax, Pi 5 proof, networking, SSH, and phase transition.

## Evidence

- QEMU/substitute task smoke: tasks/evidence/2026-06-04-phase10-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log records command 3 exec stdout >/tmp/stdout.txt, fd1=regular-file, exec-redirection op=sink target-path=/tmp/stdout.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stdout.txt, exec-stdout bytes=0x1f return=0x1f stream=regular-file route=volatile-vfs:/tmp/stdout.txt, waitpid, laststatus, descriptor-backed cat /tmp/stdout.txt readback with Talos userspace stdout fixture plus cat path=/tmp/stdout.txt bytes=0x1f source=volatile-vfs-descriptor-read, normal exec stdout restoration through runtime-console0/stdout, append/arbitrary-path/stderr-file negatives, descriptor-backed cat /etc/banner.txt, final qemu-local-shell-stdout-regular-file-redirection-complete, and PASS.
- Retained /dev/null output sink controls: tasks/evidence/2026-06-04-phase10-dev-null-stdout-redirection-contract-core/qemu-local-shell-dev-null-stdout-redirection-smoke.log and tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-dev-null-stderr-redirection-smoke.log.
- Retained read-only input redirection controls: tasks/evidence/2026-06-04-phase10-dev-null-stdin-redirection-core/qemu-local-shell-dev-null-stdin-redirection-smoke.log and tasks/evidence/2026-06-04-phase10-readonly-regular-file-stdin-redirection-core/qemu-local-shell-readonly-regular-file-stdin-redirection-smoke.log.
- Retained descriptor redirection, pipeline, VFS exec/open/read, waitpid/laststatus, and cat controls: tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log, tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log, tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log, tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log, tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log, tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log, and tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test --quiet local_command_loop passed 406 no_std tests, including the new volatile stdout regular-file redirection unit test.
- QEMU/substitute: scripts/qemu-local-shell-stdout-regular-file-redirection-smoke.sh --quiet passed with retained PASS log.
- full unit tests: cargo -Zjson-target-spec test --quiet passed 406 no_std tests.
- docs: /home/node/.cargo/bin/mdbook build passed after the roadmap update.
- diff check: git diff --check passed.
- staged diff check: git diff --cached --check passed.

## Commit

Commit: accepted implementation and evidence committed; final SHA recorded in durable supervisor state.
