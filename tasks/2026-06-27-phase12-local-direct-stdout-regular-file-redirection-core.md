# Phase 12 Local Direct Stdout Regular-File Redirection Core

Task: phase12-local-direct-stdout-regular-file-redirection-core-20260627

Status: accepted pending commit.

## Scope

- Accepted exactly '/bin/stdout >/tmp/stdout.txt' as the first local-only direct path-form stdout regular-file output redirection witness.
- Kept the child program on the descriptor-backed VFS open/read, loader, userspace launch/status, descriptor inheritance, lifecycle, waitpid, laststatus, process-table, and shell descriptor restoration path.
- Kept readback on descriptor-backed VFS with 'cat /tmp/stdout.txt'.
- Did not start bare-name output redirection, stderr file redirection, append/truncate, unsupported output path policy, pipeline output redirection, combined input/output redirection, writable persistent filesystem behavior, live networking/SSH, Pi 5 hardware action, generated-root retry, or phase transition work.

## Findings

- fixed: absolute-path command parsing now accepts the exact stdout output token '>/tmp/stdout.txt' only when the command path is '/bin/stdout'. This leaves bare-name output redirection, stderr file redirection, append, unsupported paths, pipeline output redirection, and combined input/output redirection fail-closed for this new direct surface.
- fixed: the task-owned QEMU/substitute stdout regular-file smoke now sends '/bin/stdout >/tmp/stdout.txt' for the accepted witness and '/bin/stdout' for the restoration witness instead of the older exec-prefixed command form.
- fixed: qemu_virt dispatch expectations for the stdout regular-file smoke now require the direct absolute-path witness.
- fixed: unit coverage now proves redirected write, 'cat /tmp/stdout.txt' readback, normal stdout restoration, waitpid/laststatus coherence, and fail-closed unsupported forms without successful process records.
- not-an-issue: older exec-prefixed stdout file-redirection tests remain as regression history in the broader local command loop; this task accepts only the direct absolute-path witness and records later bare-name/output expansion separately.
- deferred: bare-name stdout output redirection, stderr regular-file redirection, append/truncate, arbitrary output paths, pipeline output redirection, combined input/output redirection, kernel-backed command redirection, persistent writable filesystem behavior, PATH/current-directory search, arbitrary shell grammar, live networking/SSH, Pi 5 hardware proof, generated-root retry, and phase transition remain outside this core.

## Evidence

- static inspection: src/local_command_loop.rs, src/target/qemu_virt.rs, scripts/qemu-local-serial-command-loop-smoke.sh, scripts/qemu-local-shell-stdout-regular-file-redirection-smoke.sh, docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and docs/src/project/early-posix-shape.md.
- fmt/lint/typecheck: 'cargo fmt --all -- --check' passed.
- unit tests: 'cargo -Zjson-target-spec test --quiet local_command_loop' passed with 853 no_std tests after rerunning with the Talos QEMU tool path on PATH.
- QEMU/substitute smoke: tasks/evidence/2026-06-27-phase12-local-direct-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log ends PASS and records '/bin/stdout >/tmp/stdout.txt', 'cat /tmp/stdout.txt' readback, '/bin/stdout' restoration, and fail-closed negative controls.
- task-owned JSON: tasks/evidence/2026-06-27-phase12-local-direct-stdout-regular-file-redirection-core/classification.json and evidence-map.json.

## Validation

- 'cargo fmt --all -- --check' passed.
- 'cargo -Zjson-target-spec test --quiet local_command_loop' passed.
- 'scripts/qemu-local-shell-stdout-regular-file-redirection-smoke.sh' passed.
- 'jq empty tasks/evidence/2026-06-27-phase12-local-direct-stdout-regular-file-redirection-core/classification.json tasks/evidence/2026-06-27-phase12-local-direct-stdout-regular-file-redirection-core/evidence-map.json' passed.
- 'git diff --check' passed.
- '/home/node/.cargo/bin/mdbook build' passed.
- 'git diff --cached --check' passed.

## Result

The accepted direct stdout redirection surface writes the userspace stdout fixture through child fd1 into volatile-vfs:/tmp/stdout.txt, reads the same bytes back through descriptor-backed VFS cat, restores shell fd1 for the next normal stdout command, and preserves waitpid/laststatus/process-table observability.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.
