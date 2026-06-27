# Phase 12 Local Direct Stdout Regular-File Redirection Closeout

Task id: phase12-local-direct-stdout-regular-file-redirection-closeout-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted direct path-form stdout regular-file redirection
frontier after the core task accepted:

~~~text
/bin/stdout >/tmp/stdout.txt
~~~

This closeout is static reconciliation work. It does not change source
behavior, run Pi 5 hardware, resume live networking/SSH, retry generated-root
command input, accept bare-name output redirection, accept stderr output
redirection, accept append/truncate expansion, accept pipeline output
redirection, accept persistent writable filesystem behavior, or accept a phase
transition.

## Findings

- fixed: The accepted direct stdout regular-file redirection frontier is
  reconciled against the retained core task record, classification JSON,
  evidence map, QEMU/substitute transcript, docs, and regression evidence.
- fixed: The exact accepted witness remains
  '/bin/stdout >/tmp/stdout.txt'; no bare-name command lookup, stderr
  redirection, append, arbitrary path, pipeline output, combined
  input/output redirection, or kernel-backed command redirection is accepted by
  this closeout.
- fixed: The accepted evidence records child-only fd1 rebinding to
  volatile-vfs:/tmp/stdout.txt, userspace TalosWrite provenance, descriptor
  readback through 'cat /tmp/stdout.txt', normal '/bin/stdout' shell fd1
  restoration, closed loader temporary descriptor state, and coherent
  waitpid/laststatus/process-table observations.
- fixed: The selected next task is
  phase12-local-bare-name-stdout-regular-file-redirection-core-20260627
  because its dependencies are objective: the direct stdout core is accepted
  and committed, this closeout reconciles the accepted evidence, supervisor
  intervention is inactive, and the hardware lock is restored/unlocked.
- not-an-issue: No implementation change is required for this closeout; the
  core task already recorded static inspection, fmt/lint/typecheck, unit test,
  QEMU/substitute, JSON, diff, docs, and staged-diff evidence.
- deferred: Bare-name stdout output redirection, stderr redirection, append,
  arbitrary output paths, pipeline-output redirection, combined input/output
  redirection, persistent writable filesystem behavior, environment-backed
  PATH, current-directory search, command lookup beyond bounded /bin, quoting,
  escaping, globbing, variables, shell functions, arbitrary shell grammar,
  unbounded pipelines, pipeline concurrency, scheduler concurrency,
  fork/signals, process groups/sessions, live networking/SSH, Pi 5 hardware
  proof, generated-root retry, and phase transition.

## Evidence Map

- Direct stdout regular-file redirection core:
  tasks/2026-06-27-phase12-local-direct-stdout-regular-file-redirection-core.md.
- Direct stdout regular-file redirection classification and evidence:
  tasks/evidence/2026-06-27-phase12-local-direct-stdout-regular-file-redirection-core/classification.json
  and
  tasks/evidence/2026-06-27-phase12-local-direct-stdout-regular-file-redirection-core/evidence-map.json.
- Direct stdout regular-file redirection QEMU/substitute transcript:
  tasks/evidence/2026-06-27-phase12-local-direct-stdout-regular-file-redirection-core/qemu-local-shell-stdout-regular-file-redirection-smoke.log.
- Closeout classification and evidence map:
  tasks/evidence/2026-06-27-phase12-local-direct-stdout-regular-file-redirection-closeout/classification.json
  and
  tasks/evidence/2026-06-27-phase12-local-direct-stdout-regular-file-redirection-closeout/evidence-map.json.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted direct stdout regular-file output redirection frontier is
local-only and static/unit/QEMU-substitute backed. A direct absolute path
program can bind only child fd1 to the volatile scratch file:

~~~text
/bin/stdout >/tmp/stdout.txt
~~~

The child resolves directly through descriptor-backed VFS open/read, the
accepted loader, userspace launch/status, and bounded process-table
observation. The launched process records fd1=regular-file, op=sink,
target-path=/tmp/stdout.txt, target-stream=regular-file,
target-route=volatile-vfs:/tmp/stdout.txt, child-only=true, and
shell-restored=true. The userspace stdout fixture writes 0x1f bytes through
fd1 with source=userspace-talos-write; a later descriptor-backed
'cat /tmp/stdout.txt' reads the same fixture bytes with
source=volatile-vfs-descriptor-read. A subsequent normal '/bin/stdout' records
fd1=stdio-output and route=runtime-console0/stdout, proving shell fd1
restoration.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain bare-name stdout output redirection, stderr
redirection, append/truncate, arbitrary output paths, pipeline-output
redirection, combined input/output redirection, persistent writable filesystem
behavior, environment-backed PATH, current-directory search, command lookup
beyond bounded /bin, quoting, escaping, globbing, variables, shell functions,
arbitrary shell grammar, unbounded pipelines, pipeline concurrency, scheduler
concurrency, fork/signals, process groups/sessions, broad procfs/Linux ps, PID
policy expansion, waitpid options, live networking/SSH, Pi 5 hardware proof,
generated-root command-input retry, and phase transition.

## Validation

- Static inspection of task records, retained evidence paths, docs, and git
  diff: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

## Result

selected_next_task: phase12-local-bare-name-stdout-regular-file-redirection-core-20260627.

The bare-name stdout regular-file redirection core is mechanically unblocked
after this accepted closeout is committed, provided the hardware lock remains
restored/unlocked, supervisor intervention remains inactive, and the repo has
no conflicting uncommitted changes.
