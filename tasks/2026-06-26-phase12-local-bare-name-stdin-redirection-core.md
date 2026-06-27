# Phase 12 Local Bare-Name Stdin Redirection Core

Task id: phase12-local-bare-name-stdin-redirection-core-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Implement the thinnest bare-name command stdin redirection path:

~~~text
stdin </etc/banner.txt
~~~

The command remains bounded to the accepted fixed `/bin` lookup. `stdin`
resolves to `/bin/stdin`, opens and reads through descriptor-backed VFS,
launches through the accepted userspace startup/status path, and sees fd0
replaced only for the child by the read-only initramfs regular file
`/etc/banner.txt`.

This task does not accept environment-backed PATH, command lookup beyond
bounded `/bin`, pipeline-stage redirection, output redirection expansion,
append/truncate, writable filesystem behavior, broader shell grammar,
generated-root retry, live networking/SSH, Pi 5 hardware proof, or a phase
transition.

## Findings

- fixed: Bare fixed-/bin command parsing now accepts the exact
  `stdin </etc/banner.txt` form without admitting general redirection grammar
  or treating the redirection token as argv.
- fixed: The child `/bin/stdin` descriptor table records fd0=regular-file,
  fd1=stdio-output, fd2=stdio-output, and loader-temp-open=false.
- fixed: Redirection evidence records op=source, source-path=/etc/banner.txt,
  source-route=initramfs:/etc/banner.txt, child-only=true, and
  shell-restored=true.
- fixed: Userspace stdin read evidence records canonical argv0=/bin/stdin,
  read-source=initramfs:/etc/banner.txt, and
  read-result=regular-file-eof-after-read.
- fixed: waitpid, laststatus, bounded process-table,
  `/proc/talos/processes`, zero-argument `ps`, and pipestatus-compatible
  observations remain coherent for the redirected bare-name command.
- fixed: Unsupported bare-name redirection variants such as
  `stdout </etc/banner.txt`, `stdin </dev/null`, and
  `stdin < /etc/banner.txt` fail closed without additional successful process
  records.
- fixed: The QEMU/substitute smoke harness now has a dedicated
  qemu_local_shell_bare_name_stdin_redirection boot scenario and task-owned
  transcript.
- fixed: The shared QEMU smoke post-run assertion block now covers the new
  bare-name stdin redirection smoke instead of falling through to the default
  serial-command assertions after the scenario itself reaches PASS.
- fixed: The builtins boundary string now includes the accepted bounded
  bare-name stdin redirection surface.
- not-an-issue: Existing direct path-form stdin redirection and
  exec-prefixed read-only stdin redirection remain separate retained
  regression/control surfaces rather than being replaced.
- deferred: Pipeline-stage redirection, output redirection expansion,
  append/truncate, writable filesystem behavior, combined redirections beyond
  accepted exact forms, environment-backed PATH, current-directory search,
  command lookup beyond bounded `/bin`, quoting, escaping, globbing,
  variables, arbitrary shell grammar, unbounded pipelines, pipeline
  concurrency, scheduler concurrency, fork/signals, process groups/sessions,
  persistent storage, live networking/SSH, Pi 5 hardware proof,
  generated-root retry, and phase transition.

## Evidence Map

- Classification and evidence JSON:
  tasks/evidence/2026-06-26-phase12-local-bare-name-stdin-redirection-core/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-bare-name-stdin-redirection-core/evidence-map.json.
- QEMU/substitute transcript:
  tasks/evidence/2026-06-26-phase12-local-bare-name-stdin-redirection-core/qemu-local-shell-bare-name-stdin-redirection-smoke.log.
- Implementation and smoke harness:
  build.rs, src/main.rs, src/local_command_loop.rs, src/target/qemu_virt.rs,
  scripts/qemu-local-serial-command-loop-smoke.sh, and
  scripts/qemu-local-shell-bare-name-stdin-redirection-smoke.sh.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted bare-name stdin redirection frontier is local-only and
static/unit/QEMU substitute backed. The bare-name command
`stdin </etc/banner.txt` resolves through fixed `/bin` lookup to
`/bin/stdin`, runs through descriptor-backed VFS open/read, the accepted
loader, userspace launch/status, and child-only fd0 replacement from
`initramfs:/etc/banner.txt`.

The shell restores fd0 after the child exits. The loader temporary descriptor
is closed. The bounded process table, waitpid, laststatus,
`/proc/talos/processes`, zero-argument `ps`, and pipestatus surfaces remain
intact.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain pipeline-stage redirection, output regular-file
redirection expansion, append/truncate, writable filesystem behavior, combined
redirections beyond accepted exact forms, environment-backed PATH,
current-directory search, command lookup beyond bounded `/bin`, quoting,
escaping, globbing, variables, arbitrary shell grammar, unbounded pipelines,
pipeline concurrency, scheduler concurrency, fork/signals, process
groups/sessions, broad procfs/Linux ps, PID policy expansion, waitpid options,
persistent storage, live networking/SSH, Pi 5 hardware proof, generated-root
command-input retry, and phase transition.

## Validation

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet
  local_command_loop_runs_bare_name_vfs_command_with_readonly_stdin_redirection:
  passed; harness executed 847 no_std tests.
- QEMU/substitute bare-name stdin redirection smoke using
  scripts/qemu-local-shell-bare-name-stdin-redirection-smoke.sh with
  task-owned evidence paths: passed after moving from the default occupied
  local port to a free local port; final accepted run used port 54430.
- Retained regression smokes passed: exec-prefixed read-only stdin
  redirection, direct path-form stdin redirection, absolute-path command,
  bare-name argv command, direct pipeline stage argv, bare-name pipeline stage
  argv, process-status VFS, zero-argument ps, pipeline status, and cat-banner;
  final accepted run used unique local ports 54431 through 54440.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed; html backend written with
  existing large search-index warning.
- git diff --cached --check: pending before commit.

## Result

selected_next_task: phase12-local-bare-name-stdin-redirection-closeout-20260626.

The bare-name stdin redirection closeout is mechanically unblocked after this
accepted core task is committed, provided the hardware lock remains
restored/unlocked and supervisor intervention remains inactive.
