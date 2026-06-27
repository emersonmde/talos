# Phase 12 Local Bare-Name Stdout Regular-File Redirection Core

Task: phase12-local-bare-name-stdout-regular-file-redirection-core-20260627

Status: accepted; commit pending.

## Scope

- Accepted exactly `stdout >/tmp/stdout.txt` through the fixed bounded `/bin` lookup to `/bin/stdout`.
- Kept the child program on the descriptor-backed VFS open/read, loader, userspace launch/status, descriptor inheritance, lifecycle, waitpid, laststatus, process-table, and shell descriptor restoration path.
- Kept readback on descriptor-backed VFS with `cat /tmp/stdout.txt`.
- Retained the accepted direct path-form witness `/bin/stdout >/tmp/stdout.txt`.
- Did not start PATH environment lookup, current-directory search, command lookup beyond bounded `/bin`, stderr file redirection, append/truncate, arbitrary output path expansion, pipeline output redirection, combined input/output redirection, persistent filesystem semantics, live networking/SSH, Pi 5 hardware action, generated-root retry, or phase transition work.

## Findings

- fixed: bare-name command parsing now accepts the exact stdout output token `>/tmp/stdout.txt` only when the resolved command name is `stdout`, then canonicalizes argv0 through the existing fixed `/bin` lookup to `/bin/stdout`.
- fixed: the local command loop boundary string now records the bounded bare-name stdout regular-file redirection surface.
- fixed: the task-owned QEMU/substitute smoke now drives `stdout >/tmp/stdout.txt`, descriptor-backed `cat /tmp/stdout.txt` readback, normal `stdout` restoration, and unsupported append/path/pipeline/combined-form negatives.
- fixed: qemu_virt and build.rs know the task-owned `qemu_local_shell_bare_name_stdout_regular_file_redirection` boot scenario.
- not-an-issue: descriptor installation, volatile VFS regular-file storage, shell descriptor restoration, loader temp-descriptor closure, waitpid, laststatus, and readback behavior were already implemented by the direct stdout redirection surface and were reused unchanged.
- deferred: stderr redirection, append/truncate, arbitrary paths, pipeline output redirection, combined input/output redirection beyond accepted exact forms, writable persistent filesystem behavior, environment-backed PATH, current-directory search, command lookup beyond bounded `/bin`, broad shell grammar, live networking/SSH, Pi 5 hardware proof, generated-root retry, and phase transition remain outside this core.

## Evidence

- static inspection: src/local_command_loop.rs, src/target/qemu_virt.rs, build.rs, scripts/qemu-local-serial-command-loop-smoke.sh, scripts/qemu-local-shell-bare-name-stdout-regular-file-redirection-smoke.sh, docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and docs/src/project/early-posix-shape.md.
- fmt/lint/typecheck: `cargo fmt --all -- --check` passed.
- unit tests: `cargo -Zjson-target-spec test --quiet local_command_loop` passed with 854 no_std tests; captured in `target/task-bare-stdout-test.out` with status `0`.
- QEMU/substitute smoke: `tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-regular-file-redirection-core/qemu-local-shell-bare-name-stdout-regular-file-redirection-smoke.log` ends PASS and records `stdout >/tmp/stdout.txt`, `cat /tmp/stdout.txt` readback, `stdout` restoration, and fail-closed negative controls.
- task-owned JSON: `tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-regular-file-redirection-core/classification.json` and `evidence-map.json`.

## Validation

- `cargo fmt --all -- --check` passed.
- `cargo -Zjson-target-spec test --quiet local_command_loop` passed.
- `scripts/qemu-local-shell-bare-name-stdout-regular-file-redirection-smoke.sh` passed.
- `jq empty tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-regular-file-redirection-core/classification.json tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-regular-file-redirection-core/evidence-map.json` passed.
- `git diff --check` passed.
- `/home/node/.cargo/bin/mdbook build` passed; existing large search-index warning only.
- `git diff --cached --check` passed.

## Result

The accepted bare-name stdout redirection surface resolves only through the bounded fixed `/bin` lookup to `/bin/stdout`, writes the userspace stdout fixture through child fd1 into `volatile-vfs:/tmp/stdout.txt`, reads the same bytes back through descriptor-backed VFS `cat`, restores shell fd1 for the next normal stdout command, and preserves waitpid/laststatus/process-table observability.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.
