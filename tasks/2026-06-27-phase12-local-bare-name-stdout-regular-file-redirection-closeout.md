# Phase 12 Local Bare-Name Stdout Regular-File Redirection Closeout

Task id: phase12-local-bare-name-stdout-regular-file-redirection-closeout-20260627

Status: accepted; commit hash recorded in durable supervisor state after
commit.

## Scope

Reconcile the accepted fixed-/bin bare-name stdout regular-file redirection
frontier after the core task accepted:

~~~text
stdout >/tmp/stdout.txt
~~~

This closeout is static reconciliation work. It does not change source
behavior, run Pi 5 hardware, resume live networking/SSH, retry generated-root
command input, accept stderr output redirection, accept append/truncate
expansion, accept arbitrary output paths, accept pipeline output redirection,
accept persistent writable filesystem behavior, or accept a phase transition.

## Findings

- fixed: The accepted bare-name stdout regular-file redirection frontier is
  reconciled against the retained core task record, classification JSON,
  evidence map, QEMU/substitute transcript, docs, and retained direct
  path-form stdout redirection evidence.
- fixed: The exact accepted witness remains 'stdout >/tmp/stdout.txt';
  command lookup is only the accepted fixed bounded /bin lookup to
  '/bin/stdout'. No environment-backed PATH lookup, current-directory search,
  stderr redirection, append, arbitrary output path, pipeline output,
  combined input/output redirection, or kernel-backed command redirection is
  accepted by this closeout.
- fixed: The accepted evidence records child-only fd1 rebinding to
  volatile-vfs:/tmp/stdout.txt, userspace TalosWrite provenance, descriptor
  readback through 'cat /tmp/stdout.txt', normal 'stdout' shell fd1
  restoration, closed loader temporary descriptor state, and coherent
  waitpid/laststatus/process-table observations.
- fixed: The selected next task is
  phase12-local-stdout-regular-file-redirection-frontier-checkpoint-20260627
  because its dependencies are objective: the bare-name stdout core is
  accepted and committed, this closeout reconciles the accepted evidence,
  supervisor intervention is inactive, and the hardware lock is
  restored/unlocked.
- not-an-issue: No implementation change is required for this closeout; the
  core task already recorded static inspection, fmt/lint/typecheck, unit test,
  QEMU/substitute, JSON, diff, docs, and staged-diff evidence.
- deferred: stderr redirection, append/truncate, arbitrary output paths,
  pipeline-output redirection, combined input/output redirection, persistent
  writable filesystem behavior, environment-backed PATH, current-directory
  search, command lookup beyond bounded /bin, quoting, escaping, globbing,
  variables, shell functions, arbitrary shell grammar, unbounded pipelines,
  pipeline concurrency, scheduler concurrency, fork/signals, process
  groups/sessions, live networking/SSH, Pi 5 hardware proof, generated-root
  retry, and phase transition.

## Evidence Map

- Bare-name stdout regular-file redirection core:
  tasks/2026-06-27-phase12-local-bare-name-stdout-regular-file-redirection-core.md.
- Bare-name stdout regular-file redirection classification and evidence:
  tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-regular-file-redirection-core/classification.json
  and
  tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-regular-file-redirection-core/evidence-map.json.
- Bare-name stdout regular-file redirection QEMU/substitute transcript:
  tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-regular-file-redirection-core/qemu-local-shell-bare-name-stdout-regular-file-redirection-smoke.log.
- Direct stdout regular-file redirection closeout retained evidence:
  tasks/2026-06-27-phase12-local-direct-stdout-regular-file-redirection-closeout.md.
- Closeout classification and evidence map:
  tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-regular-file-redirection-closeout/classification.json
  and
  tasks/evidence/2026-06-27-phase12-local-bare-name-stdout-regular-file-redirection-closeout/evidence-map.json.
- Project notes:
  docs/src/roadmap.md, docs/src/project/phase12-networking-ssh.md, and
  docs/src/project/early-posix-shape.md.

## Accepted Frontier

The accepted bare-name stdout regular-file output redirection frontier is
local-only and static/unit/QEMU-substitute backed. The accepted witness is
exactly:

~~~text
stdout >/tmp/stdout.txt
~~~

The command resolves only through the bounded fixed /bin lookup to
'/bin/stdout'. The launched child records fd1=regular-file, op=sink,
target-path=/tmp/stdout.txt, target-stream=regular-file,
target-route=volatile-vfs:/tmp/stdout.txt, child-only=true, and
shell-restored=true. The userspace stdout fixture writes 0x1f bytes through
fd1 with source=userspace-talos-write; a later descriptor-backed
'cat /tmp/stdout.txt' reads the same fixture bytes with
source=volatile-vfs-descriptor-read. A subsequent normal 'stdout' records
fd1=stdio-output and route=runtime-console0/stdout, proving shell fd1
restoration. The direct path-form witness '/bin/stdout >/tmp/stdout.txt'
remains accepted.

Live network/SSH reachability remains paused. No Pi 5 hardware claim is made.

## Deferred Frontier

Deferred surfaces remain stderr redirection, append/truncate, arbitrary output
paths, pipeline-output redirection, combined input/output redirection,
persistent writable filesystem behavior, environment-backed PATH,
current-directory search, command lookup beyond bounded /bin, quoting,
escaping, globbing, variables, shell functions, arbitrary shell grammar,
unbounded pipelines, pipeline concurrency, scheduler concurrency,
fork/signals, process groups/sessions, broad procfs/Linux ps, PID policy
expansion, waitpid options, live networking/SSH, Pi 5 hardware proof,
generated-root command-input retry, and phase transition.

## Validation

- Static inspection of task records, retained evidence paths, docs, and git
  diff: passed.
- jq empty on task-owned JSON evidence: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed.

## Result

selected_next_task: phase12-local-stdout-regular-file-redirection-frontier-checkpoint-20260627.

The stdout regular-file redirection frontier checkpoint is mechanically
unblocked after this accepted closeout is committed, provided the hardware
lock remains restored/unlocked, supervisor intervention remains inactive, and
the repo has no conflicting uncommitted changes.
