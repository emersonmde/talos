# Phase 12 Local Direct Stdout Append Regular-File Redirection Core

Task: phase12-local-direct-stdout-append-regular-file-redirection-core-20260627

Status: accepted; commit recorded in supervisor state.

## Scope

- Accepted exactly the direct path-form sequence
  '/bin/stdout >/tmp/stdout.txt' followed by
  '/bin/stdout >>/tmp/stdout.txt'.
- Kept execution on descriptor-backed VFS open/read, loader, userspace
  launch/status, descriptor inheritance, lifecycle, waitpid, laststatus,
  process-table, and shell descriptor restoration paths.
- Rebound only child fd1 to volatile-vfs:/tmp/stdout.txt for each redirected
  process. The first command uses the existing sink/truncate regular-file
  operation; the second opens/writes at EOF with append semantics.
- Proved descriptor-backed readback with 'cat /tmp/stdout.txt' returning both
  stdout fixture writes in order, then proved a later normal '/bin/stdout'
  routes fd1 back to runtime-console0/stdout.
- Retained accepted direct and fixed-/bin bare-name stdout/stderr regular-file
  redirection, stdin redirection, pipeline stdin redirection, command argv,
  process-status VFS/ps, pipestatus, and cat-banner regression surfaces.
- Did not start bare-name stdout append, stderr append, arbitrary output path
  policy, pipeline-output append, combined input/output redirection, writable
  persistent filesystem behavior, live networking/SSH, Pi 5 hardware action,
  generated-root retry, or phase transition work.

## Findings

- fixed: direct absolute command parsing now accepts the exact token
  '>>/tmp/stdout.txt' only for '/bin/stdout', preserving the earlier exact
  '/bin/stdout >/tmp/stdout.txt' sink/truncate witness.
- fixed: unit coverage now proves redirected fd1 truncate plus append writes,
  'cat /tmp/stdout.txt' readback of both stdout fixtures in order, normal direct
  stdout restoration, waitpid/laststatus coherence, and fail-closed unsupported
  direct forms without successful process records.
- fixed: added a focused QEMU/substitute scenario, wrapper script, boot
  scenario, label, classification, command count, expected dispatch table, and
  task-owned evidence path for the direct stdout append witness.
- fixed: retained local POSIX regression gates passed for stdout/stderr
  regular-file redirection, direct stdin redirection, direct pipeline stdin
  redirection, process-status VFS, zero-argument ps, pipestatus, and cat-banner
  surfaces.
- not-an-issue: the existing volatile regular-file descriptor path already had
  EOF append behavior; the missing core behavior was admitting the exact direct
  append witness and proving it through shell-visible VFS readback.
- deferred: bare-name stdout append, stderr append, arbitrary output paths,
  pipeline-output append, combined input/output redirection, kernel-backed
  command redirection, persistent writable filesystem behavior,
  PATH/current-directory search, command lookup beyond bounded /bin, arbitrary
  shell grammar, live networking/SSH, Pi 5 hardware proof, generated-root retry,
  and phase transition remain outside this core.

## Evidence

- static inspection: src/local_command_loop.rs, src/target/qemu_virt.rs,
  src/main.rs, build.rs, scripts/qemu-local-serial-command-loop-smoke.sh,
  scripts/qemu-local-shell-direct-stdout-regular-file-append-redirection-smoke.sh,
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.
- fmt/lint/typecheck: 'cargo fmt --all -- --check' passed.
- unit tests: 'cargo -Zjson-target-spec test --quiet local_command_loop'
  passed.
- QEMU/substitute smoke:
  tasks/evidence/2026-06-27-phase12-local-direct-stdout-append-regular-file-redirection-core/qemu-local-shell-direct-stdout-regular-file-append-redirection-smoke.log
  ends PASS and records '/bin/stdout >/tmp/stdout.txt',
  '/bin/stdout >>/tmp/stdout.txt', op=append,
  'cat /tmp/stdout.txt' reading two stdout fixtures with bytes=0x3e, later
  normal '/bin/stdout' restoration, and fail-closed unsupported append forms.
- QEMU/substitute regressions: the retained stdout regular-file, stderr
  regular-file, direct stdin, direct pipeline stdin, process-status VFS,
  zero-argument ps, pipestatus, and cat-banner smoke scripts passed.
- task-owned JSON:
  tasks/evidence/2026-06-27-phase12-local-direct-stdout-append-regular-file-redirection-core/classification.json
  and evidence-map.json.

## Validation

- 'cargo fmt --all -- --check' passed.
- 'cargo -Zjson-target-spec test --quiet local_command_loop' passed.
- 'scripts/qemu-local-shell-direct-stdout-regular-file-append-redirection-smoke.sh --quiet' passed.
- retained local POSIX QEMU/substitute regression smoke batch passed.
- 'jq empty tasks/evidence/2026-06-27-phase12-local-direct-stdout-append-regular-file-redirection-core/classification.json tasks/evidence/2026-06-27-phase12-local-direct-stdout-append-regular-file-redirection-core/evidence-map.json' passed.
- 'git diff --check' passed.
- '/home/node/.cargo/bin/mdbook build' passed.
- 'git diff --cached --check' passed.

## Result

The accepted direct stdout append redirection surface writes the userspace
stdout fixture through child fd1 into volatile-vfs:/tmp/stdout.txt once with
truncate/sink semantics and then again with append-at-EOF semantics. A
descriptor-backed 'cat /tmp/stdout.txt' reads both fixture lines in order, and a
later normal '/bin/stdout' proves shell fd1 restoration.

selected_next_task:
phase12-local-direct-stdout-append-regular-file-redirection-closeout-20260627.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.
