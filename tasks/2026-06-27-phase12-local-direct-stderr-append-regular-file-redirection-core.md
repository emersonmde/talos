# Phase 12 Local Direct Stderr Append Regular-File Redirection Core

Task: phase12-local-direct-stderr-append-regular-file-redirection-core-20260627

Status: accepted; commit recorded in supervisor state.

## Scope

- Accepted exactly the direct path-form sequence
  '/bin/stderr 2>/tmp/stderr.txt' followed by
  '/bin/stderr 2>>/tmp/stderr.txt'.
- Kept both child launches on descriptor-backed VFS open/read, loader,
  userspace startup/status, descriptor inheritance, lifecycle, waitpid,
  laststatus, process-table, and shell descriptor restoration paths.
- Rebound only child fd2 to volatile-vfs:/tmp/stderr.txt for each redirected
  process. The first command uses the existing sink/truncate regular-file
  operation; the second opens/writes at EOF with append semantics.
- Proved descriptor-backed readback with 'cat /tmp/stderr.txt' returning both
  stderr fixture writes in order, then proved a later normal '/bin/stderr'
  routes fd2 back to runtime-console0/stderr.
- Retained accepted stdout append/truncate, stdout/stderr regular-file
  redirection, stdin and pipeline stdin redirection, command argv,
  process-status VFS/ps, pipestatus, and cat-banner regression surfaces.
- Did not start bare-name stderr append, arbitrary output path policy,
  pipeline-output append, combined input/output redirection, persistent
  writable filesystem behavior, live networking/SSH, Pi 5 hardware action,
  generated-root retry, or phase transition work.

## Findings

- fixed: direct absolute command parsing now accepts the exact token
  '2>>/tmp/stderr.txt' only for '/bin/stderr', preserving the earlier exact
  '/bin/stderr 2>/tmp/stderr.txt' sink/truncate witness.
- fixed: unit coverage now proves redirected fd2 truncate plus append writes,
  'cat /tmp/stderr.txt' readback of both stderr fixtures in order, normal direct
  stderr restoration, waitpid/laststatus coherence, and fail-closed unsupported
  direct/bare/pipeline/combined forms without extra successful process records.
- fixed: added a focused QEMU/substitute scenario, wrapper script, boot
  scenario, label, classification, command count, expected dispatch table, and
  task-owned evidence path for the direct stderr append witness.
- fixed: updated retained stdout/stderr redirection regression controls so they
  treat direct stderr append as accepted while preserving their stdout/stderr
  truncate and fail-closed path controls.
- fixed: retained local POSIX regression gates passed for stdout append,
  stdout/stderr truncate redirection, direct stdin redirection, direct pipeline
  stdin redirection, process-status VFS, zero-argument ps, pipestatus, and
  cat-banner surfaces.
- not-an-issue: the existing volatile regular-file descriptor store already had
  EOF append behavior for fd2; the missing core behavior was admitting the exact
  direct path-form append witness and proving it through shell-visible VFS
  readback.
- deferred: bare-name stderr append, arbitrary output paths, pipeline-output
  append, combined input/output redirection, kernel-backed command redirection,
  persistent writable filesystem behavior, PATH/current-directory search,
  command lookup beyond bounded /bin, arbitrary shell grammar, live
  networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
  transition remain outside this core.

## Evidence

- static inspection: src/local_command_loop.rs, src/target/qemu_virt.rs,
  src/main.rs, build.rs, scripts/qemu-local-serial-command-loop-smoke.sh,
  scripts/qemu-local-shell-direct-stderr-regular-file-append-redirection-smoke.sh,
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.
- fmt/lint/typecheck: 'cargo fmt --all -- --check' passed.
- unit tests: 'cargo -Zjson-target-spec test --quiet local_command_loop'
  passed.
- QEMU/substitute smoke:
  tasks/evidence/2026-06-27-phase12-local-direct-stderr-append-regular-file-redirection-core/qemu-local-shell-direct-stderr-regular-file-append-redirection-smoke.log
  ends PASS and records '/bin/stderr 2>/tmp/stderr.txt',
  '/bin/stderr 2>>/tmp/stderr.txt', op=append,
  'cat /tmp/stderr.txt' reading two stderr fixtures with bytes=0x3e, later
  normal '/bin/stderr' restoration, fail-closed unsupported direct append path,
  fail-closed bare-name stderr append, and fail-closed pipeline/combined forms.
- QEMU/substitute regressions: the retained direct stdout append, bare-name
  stdout append, stdout regular-file, stderr regular-file, direct stdin, direct
  pipeline stdin, direct pipeline consumer stdin, process-status VFS,
  zero-argument ps, pipestatus, and cat-banner smoke scripts passed.
- task-owned JSON:
  tasks/evidence/2026-06-27-phase12-local-direct-stderr-append-regular-file-redirection-core/classification.json
  and evidence-map.json.

## Validation

- 'cargo fmt --all -- --check' passed.
- 'cargo -Zjson-target-spec test --quiet local_command_loop' passed.
- 'scripts/qemu-local-shell-direct-stderr-regular-file-append-redirection-smoke.sh --quiet' passed.
- retained local POSIX QEMU/substitute regression smoke batch passed.
- 'bash -n scripts/qemu-local-serial-command-loop-smoke.sh' passed.
- 'bash -n scripts/qemu-local-shell-direct-stderr-regular-file-append-redirection-smoke.sh' passed.
- 'jq empty tasks/evidence/2026-06-27-phase12-local-direct-stderr-append-regular-file-redirection-core/classification.json tasks/evidence/2026-06-27-phase12-local-direct-stderr-append-regular-file-redirection-core/evidence-map.json' passed.
- 'git diff --check' passed.
- '/home/node/.cargo/bin/mdbook build' passed.
- 'git diff --cached --check' passed.

## Result

The accepted direct stderr append redirection surface writes the userspace
stderr fixture through child fd2 into volatile-vfs:/tmp/stderr.txt once with
truncate/sink semantics and then again with append-at-EOF semantics. A
descriptor-backed 'cat /tmp/stderr.txt' reads both fixture lines in order, and a
later normal '/bin/stderr' proves shell fd2 restoration.

selected_next_task:
phase12-local-direct-stderr-append-regular-file-redirection-closeout-20260627.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.
