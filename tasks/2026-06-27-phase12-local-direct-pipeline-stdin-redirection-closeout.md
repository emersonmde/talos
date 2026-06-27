# Phase 12 Local Direct Pipeline Stdin Redirection Closeout

Task id: phase12-local-direct-pipeline-stdin-redirection-closeout-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted direct path-form two-stage pipeline producer stdin
redirection frontier after the core task accepted:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin
~~~

This closeout is static reconciliation work. It does not change source
behavior, run Pi 5 hardware, resume live networking/SSH, retry generated-root
command input, accept bare-name pipeline-stage stdin redirection, accept
consumer-stage redirection, accept broader shell grammar, or accept a phase
transition.

## Findings

- fixed: The accepted direct path-form pipeline-stage stdin redirection
  frontier is reconciled against the retained task record, classification,
  evidence map, QEMU/substitute transcript, docs, and regression evidence.
- fixed: The accepted evidence records producer argv0=/bin/stdin, fd0
  source-route=initramfs:/etc/banner.txt, fd1 as the pipe endpoint, inherited
  fd2, a closed loader temporary descriptor, a successful userspace stdin
  read, and a bounded process-table entry.
- fixed: The accepted evidence records consumer argv0=/bin/stdin, fd0 as the
  pipe endpoint from the producer, inherited fd1/fd2, a closed loader
  temporary descriptor, successful launch/status, successful pipe read to EOF,
  and a bounded process-table entry.
- fixed: Shell fd0 restoration, pipeline lifecycle/status, explicit waitpid
  for both participants, laststatus, /proc/talos/processes, zero-argument ps,
  and pipestatus-compatible observations remain cited as retained controls.
- fixed: Existing direct/bare-name stdin redirection, direct/bare pipeline
  argv, command argv, process-status VFS, zero-argument ps, pipestatus, and
  cat-banner regression surfaces remain cited as retained controls.
- not-an-issue: No implementation change is required for this closeout; the
  direct pipeline stdin redirection core task already accepted the source
  behavior and regression evidence.
- deferred: Bare-name pipeline-stage stdin redirection, consumer-stage
  redirection, multistage pipeline redirection, output redirection,
  append/truncate, writable filesystem behavior, environment-backed PATH,
  current-directory search, command lookup beyond bounded /bin, arbitrary
  shell grammar, unbounded pipelines, pipeline concurrency, scheduler
  concurrency, fork/signals, process groups/sessions, persistent storage,
  live networking/SSH, Pi 5 hardware proof, generated-root retry, and phase
  transition.

## Evidence Map

- Direct path-form pipeline stdin redirection core:
  tasks/2026-06-27-phase12-local-direct-pipeline-stdin-redirection-core.md.
- Direct path-form pipeline stdin redirection classification and evidence:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stdin-redirection-core/classification.json
  and
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stdin-redirection-core/evidence-map.json.
- Direct path-form pipeline stdin redirection QEMU/substitute transcript:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stdin-redirection-core/qemu-local-shell-direct-pipeline-stdin-redirection-smoke.log.
- Retained regression records:
  tasks/2026-06-26-phase12-local-direct-stdin-redirection-core.md,
  tasks/2026-06-26-phase12-local-bare-name-stdin-redirection-core.md,
  tasks/2026-06-26-phase12-local-direct-pipeline-stage-argv-core.md,
  tasks/2026-06-26-phase12-local-bare-name-pipeline-stage-argv-core.md,
  tasks/2026-06-26-phase12-local-command-argv-frontier-checkpoint.md,
  tasks/2026-06-26-phase12-local-process-status-vfs-core.md,
  tasks/2026-06-26-phase12-local-ps-command-vfs-backed-core.md,
  tasks/2026-06-26-phase12-local-pipefail-status-core.md, and
  tasks/2026-06-02-qemu-local-cat-banner-core.md.
- Closeout classification and evidence map:
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stdin-redirection-closeout/classification.json
  and
  tasks/evidence/2026-06-27-phase12-local-direct-pipeline-stdin-redirection-closeout/evidence-map.json.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted direct path-form pipeline stdin redirection frontier is
local-only and static/unit/QEMU-substitute backed. A direct path-form
two-stage pipeline can redirect only the producer fd0 from one read-only
initramfs file while preserving producer fd1 as the pipe endpoint:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin
~~~

Both stages execute through descriptor-backed VFS open/read, the accepted
loader, userspace launch/status, and bounded process-table observation. The
producer records fd0 source-route=initramfs:/etc/banner.txt, fd1 as the pipe
endpoint, inherited fd2, loader-temp-open=false, and a regular-file EOF after
read. The consumer records fd0 as the pipe endpoint, inherited fd1/fd2,
loader-temp-open=false, and pipe EOF after writer close. Shell fd0
restoration, explicit waitpid for both participants, laststatus,
/proc/talos/processes, zero-argument ps, and pipestatus remain coherent.

The accepted direct path-form surface now gives a mechanically objective
follow-up for bounded bare-name pipeline-stage stdin redirection through the
already accepted fixed /bin lookup policy.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain bare-name pipeline-stage stdin redirection until the
selected follow-up accepts it, consumer-stage redirection, multistage pipeline
redirection, output regular-file redirection, append/truncate, writable
filesystem behavior, combined redirections beyond accepted exact forms,
environment-backed PATH, current-directory search, command lookup beyond
bounded /bin, quoting, escaping, globbing, variables, shell functions,
arbitrary shell grammar, unbounded pipelines, pipeline concurrency, scheduler
concurrency, fork/signals, process groups/sessions, broad procfs/Linux ps,
PID policy expansion, waitpid options, persistent storage, live networking/SSH,
Pi 5 hardware proof, generated-root command-input retry, and phase transition.

## Validation

- Static inspection of task records, retained evidence paths, docs, and git
  diff: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

## Result

selected_next_task: phase12-local-bare-name-pipeline-stdin-redirection-core-20260627.

The bare-name pipeline-stage stdin redirection core task is mechanically
unblocked after this accepted closeout is committed, provided the hardware
lock remains restored/unlocked and supervisor intervention remains inactive.
