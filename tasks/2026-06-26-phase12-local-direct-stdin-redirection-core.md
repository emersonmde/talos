# Phase 12 Local Direct Stdin Redirection Core

Task id: phase12-local-direct-stdin-redirection-core-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the thinnest direct absolute-path command stdin redirection path:

~~~text
/bin/stdin </etc/banner.txt
~~~

The command remains a direct VFS executable path. `/bin/stdin` opens and reads
through descriptor-backed VFS, launches through the accepted userspace
startup/status path, and sees fd0 replaced only for the child by the read-only
initramfs regular file `/etc/banner.txt`.

This task does not accept bare-name stdin redirection, pipeline-stage
redirection, output redirection expansion, append/truncate, writable
filesystem behavior, broader shell grammar, environment-backed PATH, live
networking/SSH, Pi 5 hardware proof, generated-root retry, or a phase
transition.

## Findings

- fixed: Direct absolute-path command parsing now accepts the exact
  `/bin/stdin </etc/banner.txt` form without admitting general redirection
  grammar or treating the redirection token as argv.
- fixed: The child `/bin/stdin` descriptor table records fd0=regular-file,
  fd1=stdio-output, fd2=stdio-output, and loader-temp-open=false.
- fixed: Redirection evidence records op=source, source-path=/etc/banner.txt,
  source-route=initramfs:/etc/banner.txt, child-only=true, and
  shell-restored=true.
- fixed: Userspace stdin read evidence records read-source=initramfs:/etc/banner.txt
  and read-result=regular-file-eof-after-read.
- fixed: waitpid, laststatus, bounded process-table,
  `/proc/talos/processes`, zero-argument `ps`, and pipestatus-compatible
  observations remain coherent for the redirected direct command.
- fixed: Unsupported direct path redirection variants such as
  `/bin/stdout </etc/banner.txt`, `/bin/stdin </dev/null`, and
  `/bin/stdin < /etc/banner.txt` fail closed without additional successful
  process records.
- fixed: The QEMU/substitute smoke harness now has a dedicated
  qemu_local_shell_direct_stdin_redirection boot scenario and task-owned
  transcript.
- fixed: Existing exec-prefixed read-only stdin redirection remains a retained
  regression/control surface rather than being replaced.
- fixed: The retained cat-banner QEMU/substitute smoke expected the old
  six-entry `/bin` listing; the harness now accounts for the accepted
  `pingdiag` and `sockdiag` entries and keeps the regression green.
- not-an-issue: Existing direct command argv and pipeline argv surfaces remain
  separate accepted features; this task does not depend on broad shell grammar.
- deferred: Bare-name stdin redirection, pipeline-stage redirection, output
  redirection expansion, append/truncate, writable filesystem behavior,
  combined redirections beyond accepted exact forms, environment-backed PATH,
  current-directory search, arbitrary shell grammar, unbounded pipelines,
  pipeline concurrency, scheduler concurrency, fork/signals, process
  groups/sessions, persistent storage, live networking/SSH, Pi 5 hardware
  proof, generated-root retry, and phase transition.

## Evidence Map

- Classification and evidence JSON:
  tasks/evidence/2026-06-26-phase12-local-direct-stdin-redirection-core/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-direct-stdin-redirection-core/evidence-map.json.
- QEMU/substitute transcript:
  tasks/evidence/2026-06-26-phase12-local-direct-stdin-redirection-core/qemu-local-shell-direct-stdin-redirection-smoke.log.
- Implementation and smoke harness:
  build.rs, src/main.rs, src/local_command_loop.rs, src/target/qemu_virt.rs,
  scripts/qemu-local-serial-command-loop-smoke.sh, and
  scripts/qemu-local-shell-direct-stdin-redirection-smoke.sh.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted direct stdin redirection frontier is local-only and
static/unit/QEMU substitute backed. The direct path-form command
`/bin/stdin </etc/banner.txt` now runs through descriptor-backed VFS open/read,
the accepted loader, userspace launch/status, and child-only fd0 replacement
from `initramfs:/etc/banner.txt`.

The shell restores fd0 after the child exits. The loader temporary descriptor
is closed. The bounded process table, waitpid, laststatus,
`/proc/talos/processes`, zero-argument `ps`, and pipestatus surfaces remain
intact.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain bare-name stdin redirection, pipeline-stage
redirection, output regular-file redirection expansion, append/truncate,
writable filesystem behavior, combined redirections beyond accepted exact
forms, environment-backed PATH, current-directory search, command lookup
beyond existing bounded surfaces, quoting, escaping, globbing, variables,
arbitrary shell grammar, unbounded pipelines, pipeline concurrency, scheduler
concurrency, fork/signals, process groups/sessions, broad procfs/Linux ps, PID
policy expansion, waitpid options, persistent storage, live networking/SSH,
Pi 5 hardware proof, generated-root command-input retry, and phase transition.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet
  local_command_loop_runs_direct_absolute_path_vfs_command_with_readonly_stdin_redirection:
  passed; harness executed 846 no_std tests.
- QEMU/substitute direct stdin redirection smoke using
  scripts/qemu-local-shell-direct-stdin-redirection-smoke.sh with task-owned
  evidence paths: passed.
- Retained regression smokes passed: exec-prefixed read-only stdin
  redirection, absolute-path command, bare-name argv command, direct pipeline
  stage argv, bare-name pipeline stage argv, process-status VFS,
  zero-argument ps, pipeline status, and cat-banner. The pipeline-status smoke
  first hit a transient host QEMU port allocation failure, then passed on
  rerun with no Talos behavior failure.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: pending before commit.

## Result

selected_next_task: phase12-local-direct-stdin-redirection-closeout-20260626.

The direct stdin redirection closeout is mechanically unblocked after this
accepted core task is committed, provided the hardware lock remains
restored/unlocked and supervisor intervention remains inactive.
