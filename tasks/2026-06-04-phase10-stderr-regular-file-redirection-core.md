# Phase 10 Stderr Regular-File Redirection Core

Task: phase10-stderr-regular-file-redirection-core-20260604
Status: accepted

## Scope

Accept exactly one regular-file stderr redirection form: exec stderr 2>/tmp/stderr.txt. The launched VFS-backed /bin/stderr child gets fd2 rebound to a minimal volatile regular-file descriptor, writes the userspace stderr fixture bytes through the descriptor path, closes/restores fd2 after exit, and exposes the captured bytes through cat /tmp/stderr.txt while stdout remains visible and distinct.

This is not append, arbitrary paths, stdout file redirection to the stderr path, persistent storage, directory mutation, unlink/rename/mkdir, permissions, timestamps, fsync, descriptor moves, broader descriptor syntax, here-docs, pipe expansion, jobs, signals, networking, SSH, Pi 5 proof, or a phase transition.

## Findings

- fixed: Added StderrToTmpStderr as the exact 2>/tmp/stderr.txt fd2 sink form. Parsing accepts only the no-space token, and execution restricts it to the VFS-backed /bin/stderr fixture.
- fixed: Added a separate volatile scratch-file state for /tmp/stderr.txt with create/truncate semantics, a regular-file descriptor object, descriptor write from userspace memory, close, reopen for readback, and descriptor readback through cat /tmp/stderr.txt.
- fixed: The child descriptor table reports fd2=regular-file; the redirection record reports op=sink, target-path=/tmp/stderr.txt, target-stream=regular-file, and target-route=volatile-vfs:/tmp/stderr.txt.
- fixed: The userspace stderr fixture writes exactly 0x1f bytes through fd2 and records stream=regular-file, route=volatile-vfs:/tmp/stderr.txt, and source=userspace-talos-write.
- fixed: Descriptor inheritance now permits a regular-file fd2 for the child table. Without this, the shell would reject the redirected stderr child before recording the intended userspace write.
- fixed: A following cat /tmp/stderr.txt observes the captured Talos userspace stderr fixture bytes and records path, byte count, and provenance with source=volatile-vfs-descriptor-read. Later normal exec stderr and exec stdout prove shell fd2 restoration and distinct stdout visibility.
- fixed: Added deterministic negatives for append-like exec stderr 2>>/tmp/stderr.txt, arbitrary target exec stderr 2>/tmp/other.txt, and stdout regular-file output to /tmp/stderr.txt.
- fixed: Added no_std unit coverage, a dedicated QEMU/substitute wrapper, kernel boot scenario label/classification, expected dispatch rows, and task-owned retained evidence.
- deferred: append/truncate variants beyond this exact create/truncate form, arbitrary paths, persistent/wider writable filesystem behavior, metadata, directory mutation, broader descriptor syntax, Pi 5 proof, networking, SSH, and phase transition.

## Evidence

- QEMU/substitute task smoke: tasks/evidence/2026-06-04-phase10-stderr-regular-file-redirection-core/qemu-local-shell-stderr-regular-file-redirection-smoke.log records command 3 exec stderr 2>/tmp/stderr.txt, fd2=regular-file, exec-redirection op=sink target-path=/tmp/stderr.txt target-stream=regular-file target-route=volatile-vfs:/tmp/stderr.txt, exec-stderr bytes=0x1f return=0x1f stream=regular-file route=volatile-vfs:/tmp/stderr.txt, waitpid, laststatus, descriptor-backed cat /tmp/stderr.txt readback with Talos userspace stderr fixture plus cat path=/tmp/stderr.txt bytes=0x1f source=volatile-vfs-descriptor-read, normal exec stderr restoration through runtime-console0/stderr, normal exec stdout visibility through runtime-console0/stdout, append/arbitrary-path/stdout-file negatives, descriptor-backed cat /etc/banner.txt, final qemu-local-shell-stderr-regular-file-redirection-complete, and PASS.
- Retained stdout regular-file control: tasks/evidence/2026-06-04-phase10-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log.
- Retained /dev/null output sink controls: tasks/evidence/2026-06-04-phase10-dev-null-stdout-redirection-contract-core/qemu-local-shell-dev-null-stdout-redirection-smoke.log and tasks/evidence/2026-06-04-phase10-dev-null-stderr-redirection-core/qemu-local-shell-dev-null-stderr-redirection-smoke.log.
- Retained normal stdio controls: tasks/evidence/2026-06-03-phase10-userspace-stdout-inherited-fd-core/qemu-local-shell-userspace-stdout-smoke.log and tasks/evidence/2026-06-03-phase10-userspace-stderr-inherited-fd-core/qemu-local-shell-userspace-stderr-smoke.log.
- Retained descriptor redirection, pipeline, VFS exec/open/read, waitpid/laststatus, and cat controls: tasks/evidence/2026-06-04-phase10-stdout-to-stderr-fd-dup-redirection-core/qemu-local-shell-stdout-to-stderr-redirection-smoke.log, tasks/evidence/2026-06-04-phase10-stderr-to-stdout-fd-dup-redirection-core/qemu-local-shell-stderr-to-stdout-redirection-smoke.log, tasks/evidence/2026-06-04-phase10-pipeline-stderr-dup-to-stdout-core/qemu-local-shell-pipeline-stderr-dup-to-stdout-smoke.log, tasks/evidence/2026-06-04-phase10-pipeline-stdout-redirect-away-core/qemu-local-shell-pipeline-stdout-redirect-away-smoke.log, tasks/evidence/2026-06-03-phase10-absolute-vfs-exec-dispatch-core/qemu-local-shell-absolute-vfs-exec-dispatch-smoke.log, tasks/evidence/2026-06-03-phase10-minimal-waitpid-lifecycle-observation-core/qemu-local-shell-waitpid-lifecycle-smoke.log, tasks/evidence/2026-06-03-phase10-shell-last-process-status-observation/qemu-local-shell-last-process-status-smoke.log, and tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log.

## Validation

- fmt/lint/typecheck: cargo fmt --all -- --check passed.
- focused unit tests: cargo -Zjson-target-spec test --quiet local_command_loop_redirects_child_stderr_to_volatile_regular_file passed 407 no_std tests, including the new volatile stderr regular-file redirection unit test.
- QEMU/substitute: scripts/qemu-local-shell-stderr-regular-file-redirection-smoke.sh --quiet passed with retained PASS log.
- full unit tests: cargo -Zjson-target-spec test --quiet passed 407 no_std tests.
- retained controls: static inspection confirmed the retained QEMU/substitute control evidence logs listed above all contain PASS.
- docs: /home/node/.cargo/bin/mdbook build passed after the roadmap update.
- diff check: git diff --check passed.
- staged diff check: git diff --cached --check passed.

## Commit

Commit: accepted implementation and evidence committed; final SHA recorded in durable supervisor state.
