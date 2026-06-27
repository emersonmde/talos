# Phase 12 Local Bare-Name Stdout Append Regular-File Redirection Core

Task: phase12-local-bare-name-stdout-append-regular-file-redirection-core-20260627

Status: accepted; commit recorded in supervisor state.

## Scope

- Accepted exactly the fixed-/bin bare-name sequence
  'stdout >/tmp/stdout.txt' followed by 'stdout >>/tmp/stdout.txt'.
- Kept command resolution limited to the existing bounded /bin lookup, resolving
  the accepted bare name to '/bin/stdout' without PATH, current-directory
  search, or lookup beyond bounded /bin.
- Reused the accepted descriptor-backed VFS open/read, loader, userspace
  launch/status, child-only fd1 rebinding, waitpid, laststatus, process-table,
  and volatile VFS regular-file readback paths.
- Retained the accepted direct path-form stdout append surface and local
  POSIX/VFS/shell regression surfaces.
- Did not start stderr append, arbitrary output path expansion, pipeline-output
  append, combined input/output redirection, persistent writable filesystem
  behavior, live networking/SSH, Pi 5 hardware action, generated-root retry, or
  phase transition work.

## Findings

- fixed: bare-name command parsing now accepts the exact token
  '>>/tmp/stdout.txt' only for 'stdout', producing
  LocalCommandExecRedirection::StdoutAppendTmpStdout through the fixed /bin
  lookup path.
- fixed: unit coverage proves the exact bare-name truncate-plus-append witness,
  descriptor-backed readback of both stdout fixtures in order, normal stdout
  restoration, waitpid/laststatus coherence, and retained direct path-form
  behavior.
- fixed: added the qemu_local_shell_bare_name_stdout_regular_file_append_redirection
  boot scenario, label, classification, command count, expected dispatch table,
  smoke wrapper, and task-owned evidence path.
- fixed: project docs now record the accepted bare-name stdout append frontier
  and select the already queued bare-name stdout append closeout.
- not-an-issue: the existing volatile regular-file descriptor implementation
  already appends at EOF; the missing feature was admitting the exact bare-name
  append witness and proving it through the bounded /bin command path.
- deferred: stderr append, arbitrary output paths, pipeline-output append,
  combined input/output redirection, kernel-backed command redirection,
  persistent writable filesystem behavior, PATH/current-directory search,
  command lookup beyond bounded /bin, arbitrary shell grammar, live
  networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
  transition remain outside this core.

## Evidence

- static inspection: src/local_command_loop.rs, src/target/qemu_virt.rs,
  src/main.rs, build.rs, scripts/qemu-local-serial-command-loop-smoke.sh,
  scripts/qemu-local-shell-bare-name-stdout-regular-file-append-redirection-smoke.sh,
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.
- fmt/lint/typecheck: 'cargo fmt --all -- --check' passed.
- unit tests: 'cargo -Zjson-target-spec test --quiet local_command_loop'
  passed with 858 no_std tests.
- QEMU/substitute smoke:
  tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-append-regular-file-redirection-core/qemu-local-shell-bare-name-stdout-regular-file-append-redirection-smoke.log
  ends PASS and records 'stdout >/tmp/stdout.txt',
  'stdout >>/tmp/stdout.txt', op=append,
  'cat /tmp/stdout.txt' reading two stdout fixtures with bytes=0x3e, later
  normal 'stdout' restoration, and fail-closed unsupported bare-name append
  forms.
- QEMU/substitute regressions: retained direct stdout append, bare-name stdout
  regular-file redirection, stderr regular-file redirection, direct stdin,
  direct pipeline stdin, process-status VFS, zero-argument ps, pipestatus, and
  cat-banner smoke scripts passed.
- task-owned JSON:
  tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-append-regular-file-redirection-core/classification.json
  and evidence-map.json.

## Validation

- 'cargo fmt --all -- --check' passed.
- 'cargo -Zjson-target-spec test --quiet local_command_loop' passed.
- 'scripts/qemu-local-shell-bare-name-stdout-regular-file-append-redirection-smoke.sh --quiet' passed.
- retained local POSIX QEMU/substitute regression smoke batch passed.
- 'jq empty tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-append-regular-file-redirection-core/classification.json tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-append-regular-file-redirection-core/evidence-map.json' passed.
- 'git diff --check' passed.
- '/home/node/.cargo/bin/mdbook build' passed.
- 'git diff --cached --check' passed.

## Result

The accepted bare-name stdout append redirection surface resolves 'stdout' only
through the fixed bounded /bin lookup to '/bin/stdout', writes the userspace
stdout fixture through child fd1 into volatile-vfs:/tmp/stdout.txt once with
truncate/sink semantics and then again with append-at-EOF semantics. A
descriptor-backed 'cat /tmp/stdout.txt' reads both fixture lines in order, and
a later normal 'stdout' proves shell fd1 restoration.

selected_next_task:
phase12-local-bare-name-stdout-append-regular-file-redirection-closeout-20260627.

Live networking/SSH remains paused. No Pi 5 hardware claim is made.
