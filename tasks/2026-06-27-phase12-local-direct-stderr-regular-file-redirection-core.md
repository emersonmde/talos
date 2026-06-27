# Phase 12 Local Direct Stderr Regular-File Redirection Core

Task: phase12-local-direct-stderr-regular-file-redirection-core-20260627

Status: accepted; commit recorded in supervisor state.

## Scope

- Accepted exactly '/bin/stderr 2>/tmp/stderr.txt' as the first local-only direct path-form stderr regular-file output redirection witness.
- Kept the child program on the descriptor-backed VFS open/read, loader, userspace launch/status, descriptor inheritance, lifecycle, waitpid, laststatus, process-table, and shell descriptor restoration path.
- Rebound only child fd2 to a volatile regular file for '/tmp/stderr.txt'; fd0 and fd1 remain inherited from the shell.
- Proved descriptor-backed readback with 'cat /tmp/stderr.txt' and proved a later normal '/bin/stderr' routes fd2 back to runtime-console0/stderr.
- Retained accepted stdout regular-file redirection, stdin redirection, pipeline stdin redirection, command argv, pipeline argv, process-table/status, /proc/talos/processes, zero-argument ps, pipestatus, and cat-banner regression surfaces.
- Did not start bare-name stderr redirection, append/truncate, arbitrary output path policy, pipeline-output redirection, combined input/output redirection, writable persistent filesystem behavior, live networking/SSH, Pi 5 hardware action, generated-root retry, or phase transition work.

## Findings

- fixed: absolute-path command parsing now accepts the exact stderr output token '2>/tmp/stderr.txt' only when the command path is '/bin/stderr'. This leaves bare-name stderr output redirection, separated '2>' syntax, append/truncate, unsupported paths, pipeline output redirection, and combined input/output redirection fail-closed for this new direct surface.
- fixed: unit coverage now proves redirected fd2 write, 'cat /tmp/stderr.txt' readback, normal stderr restoration, fd0/fd1 inheritance, waitpid/laststatus coherence, and fail-closed unsupported direct forms without successful process records.
- fixed: the task-owned QEMU/substitute stderr regular-file smoke now sends '/bin/stderr 2>/tmp/stderr.txt' for the accepted witness and '/bin/stderr' for the restoration witness instead of the older exec-prefixed command form.
- fixed: qemu_virt dispatch expectations for the stderr regular-file smoke now require the direct absolute-path witness and direct-form negative controls.
- not-an-issue: older exec-prefixed stderr file-redirection and append tests remain as historical regression surfaces; this task accepts only the direct absolute-path witness and records later bare-name/output expansion separately.
- deferred: bare-name stderr output redirection, append/truncate, arbitrary output paths, pipeline output redirection, combined input/output redirection, kernel-backed command redirection, persistent writable filesystem behavior, PATH/current-directory search, arbitrary shell grammar, live networking/SSH, Pi 5 hardware proof, generated-root retry, and phase transition remain outside this core.

## Evidence

- static inspection: src/local_command_loop.rs, src/target/qemu_virt.rs, scripts/qemu-local-serial-command-loop-smoke.sh, scripts/qemu-local-shell-stderr-regular-file-redirection-smoke.sh, docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and docs/src/project/early-posix-shape.md.
- fmt/lint/typecheck: 'cargo fmt --all -- --check' passed.
- unit tests: 'cargo -Zjson-target-spec test --quiet local_command_loop' passed.
- QEMU/substitute smoke: tasks/evidence/2026-06-27-phase12-local-direct-stderr-regular-file-redirection-core/qemu-local-shell-stderr-regular-file-redirection-smoke.log ends PASS and records '/bin/stderr 2>/tmp/stderr.txt', 'cat /tmp/stderr.txt' readback, '/bin/stderr' restoration, and fail-closed negative controls.
- QEMU/substitute regressions: tasks/evidence/2026-06-27-phase12-local-direct-stderr-regular-file-redirection-core/regressions/ contains PASS transcripts for retained stdout regular-file redirection, stdin redirection, pipeline stdin redirection, command argv, pipeline argv, process-status VFS, zero-argument ps, pipestatus, and cat-banner surfaces.
- task-owned JSON: tasks/evidence/2026-06-27-phase12-local-direct-stderr-regular-file-redirection-core/classification.json and evidence-map.json.

## Validation

- 'cargo fmt --all -- --check' passed.
- 'cargo -Zjson-target-spec test --quiet local_command_loop' passed.
- 'scripts/qemu-local-shell-stderr-regular-file-redirection-smoke.sh' passed.
- task-owned QEMU/substitute regression smoke batch passed and retained transcripts under tasks/evidence/2026-06-27-phase12-local-direct-stderr-regular-file-redirection-core/regressions/.
- 'jq empty tasks/evidence/2026-06-27-phase12-local-direct-stderr-regular-file-redirection-core/classification.json tasks/evidence/2026-06-27-phase12-local-direct-stderr-regular-file-redirection-core/evidence-map.json' passed.
- 'git diff --check' passed.
- '/home/node/.cargo/bin/mdbook build' passed with the existing large search-index warning only.
- 'git diff --cached --check' passed.

## Result

The accepted direct stderr redirection surface writes the userspace stderr fixture through child fd2 into volatile-vfs:/tmp/stderr.txt, reads the same bytes back through descriptor-backed VFS cat, restores shell fd2 for the next normal stderr command, and preserves waitpid/laststatus/process-table observability.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.
