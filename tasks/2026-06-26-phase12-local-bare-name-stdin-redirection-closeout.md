# Phase 12 Local Bare-Name Stdin Redirection Closeout

Task id: phase12-local-bare-name-stdin-redirection-closeout-20260626

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted bare-name stdin redirection boundary without adding
runtime behavior:

~~~text
stdin </etc/banner.txt
~~~

The accepted command remains bounded to the fixed /bin lookup. stdin resolves
to /bin/stdin, opens and reads through descriptor-backed VFS, launches through
the accepted userspace startup/status path, and sees fd0 replaced only for the
child by the read-only initramfs regular file /etc/banner.txt.

This closeout does not implement pipeline-stage redirection, output
redirection expansion, append/truncate, writable filesystem behavior, broader
shell grammar, live networking/SSH, Pi 5 hardware proof, generated-root retry,
or a phase transition.

## Findings

- fixed: The bare-name stdin redirection core task record, classification
  JSON, evidence map, QEMU/substitute transcript, docs, and durable state
  consistently identify the accepted command as stdin </etc/banner.txt.
- fixed: The accepted boundary is tied to fixed bounded /bin lookup,
  descriptor-backed VFS open/read, the accepted loader, userspace
  launch/status, child-only fd0 replacement from initramfs:/etc/banner.txt,
  inherited fd1/fd2, shell fd0 restoration, and a closed loader temporary
  descriptor.
- fixed: Retained observations cover canonical argv0=/bin/stdin, waitpid,
  laststatus, bounded process-table state, /proc/talos/processes,
  zero-argument ps, and pipestatus-compatible state for the redirected
  bare-name command.
- fixed: Retained regression evidence keeps direct path-form stdin
  redirection, exec-prefixed read-only stdin redirection, descriptor
  restoration, direct and bare command argv, direct and bare pipeline argv,
  process-status VFS, zero-argument ps, pipestatus, and cat /etc/banner.txt
  green.
- not-an-issue: The accepted direct path-form and bare-name stdin redirection
  surfaces now complete the currently queued bounded read-only stdin
  redirection pair.
- deferred: Pipeline-stage redirection, output redirection expansion,
  append/truncate, writable filesystem behavior, combined redirections beyond
  accepted exact forms, environment-backed PATH, current-directory search,
  command lookup beyond bounded /bin, arbitrary shell grammar, unbounded
  pipelines, pipeline concurrency, scheduler concurrency, fork/signals,
  process groups/sessions, persistent storage, live networking/SSH, Pi 5
  hardware proof, generated-root retry, and phase transition.

## Evidence Map

- Closeout classification and evidence JSON:
  tasks/evidence/2026-06-26-phase12-local-bare-name-stdin-redirection-closeout/classification.json
  and
  tasks/evidence/2026-06-26-phase12-local-bare-name-stdin-redirection-closeout/evidence-map.json.
- Accepted bare-name stdin redirection core:
  tasks/2026-06-26-phase12-local-bare-name-stdin-redirection-core.md,
  tasks/evidence/2026-06-26-phase12-local-bare-name-stdin-redirection-core/classification.json,
  tasks/evidence/2026-06-26-phase12-local-bare-name-stdin-redirection-core/evidence-map.json,
  and
  tasks/evidence/2026-06-26-phase12-local-bare-name-stdin-redirection-core/qemu-local-shell-bare-name-stdin-redirection-smoke.log.
- Retained controls:
  tasks/evidence/2026-06-26-phase12-local-direct-stdin-redirection-core/qemu-local-shell-direct-stdin-redirection-smoke.log,
  tasks/evidence/2026-06-04-phase10-readonly-regular-file-stdin-redirection-core/qemu-local-shell-readonly-regular-file-stdin-redirection-smoke.log,
  tasks/evidence/2026-06-26-phase12-local-absolute-path-vfs-command-core/qemu-local-shell-absolute-path-vfs-command-smoke.log,
  tasks/evidence/2026-06-26-phase12-local-bare-name-command-argv-core/qemu-local-shell-bare-name-command-argv-smoke.log,
  tasks/evidence/2026-06-26-phase12-local-direct-pipeline-stage-argv-core/qemu-local-shell-direct-pipeline-stage-argv-smoke.log,
  tasks/evidence/2026-06-26-phase12-local-bare-name-pipeline-stage-argv-core/qemu-local-shell-bare-name-pipeline-stage-argv-smoke.log,
  tasks/evidence/2026-06-26-phase12-local-process-status-vfs-core/qemu-local-shell-process-status-vfs-smoke.log,
  tasks/evidence/2026-06-26-phase12-local-ps-command-vfs-core/qemu-local-shell-ps-command-vfs-smoke.log,
  tasks/evidence/2026-06-26-phase12-local-pipefail-status-core/qemu-local-shell-pipeline-status-smoke.log,
  and tasks/evidence/2026-06-02-qemu-local-cat-banner-core/qemu-local-cat-banner-smoke.log.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted bare-name stdin redirection frontier is local-only and
static/unit/QEMU substitute backed. The only accepted bare-name input
redirection command is:

~~~text
stdin </etc/banner.txt
~~~

The executable resolves through fixed bounded /bin lookup to /bin/stdin, then
comes through descriptor-backed VFS open/read and the accepted loader/userspace
launch/status path. The redirection source is initramfs:/etc/banner.txt,
installed only as child fd0, with shell fd0 restored after child exit. The
loader temporary descriptor is closed, fd1/fd2 are inherited, and waitpid,
laststatus, /proc/talos/processes, zero-argument ps, and
pipestatus-compatible observations remain coherent.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain pipeline-stage redirection, output regular-file
redirection expansion, append/truncate, writable filesystem behavior, combined
redirections beyond accepted exact forms, environment-backed PATH,
current-directory search, command lookup beyond bounded /bin, quoting,
escaping, globbing, variables, arbitrary shell grammar, unbounded pipelines,
pipeline concurrency, scheduler concurrency, fork/signals, process
groups/sessions, broad procfs/Linux ps, PID policy expansion, waitpid options,
persistent storage, live networking/SSH, Pi 5 hardware proof, generated-root
command-input retry, and phase transition.

## Validation

- Static inspection of task records, retained evidence paths, docs, and git
  diff: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.

## Result

selected_next_task: phase12-local-stdin-redirection-frontier-checkpoint-20260626.

The stdin redirection frontier checkpoint is mechanically unblocked after this
closeout is committed, provided the hardware lock remains restored/unlocked
and supervisor intervention remains inactive.
