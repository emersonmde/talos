# Phase 12 Local Explicit-Fd Separated Redirection Token Frontier Checkpoint

Task:
phase12-local-explicit-fd-separated-redirection-token-frontier-checkpoint-20260628

Status: accepted and committed; durable supervisor state records the final
commit SHA.

## Summary

Checkpointed the accepted local-only explicit-fd separated redirection-token
grammar frontier after the direct, fixed-/bin bare-name, direct pipeline, and
fixed-/bin bare-name pipeline core tasks. This checkpoint adds no runtime
behavior.

The accepted direct path-form witnesses remain exactly:

- '/bin/stdout 1 > /tmp/talos-output-alpha.txt'
- '/bin/stdout 1 >> /tmp/talos-output-alpha.txt'
- '/bin/stderr 2 > /tmp/talos-error-beta.log'
- '/bin/stderr 2 >> /tmp/talos-error-beta.log'

The accepted fixed-/bin bare-name witnesses remain exactly:

- 'stdout 1 > /tmp/talos-output-alpha.txt'
- 'stdout 1 >> /tmp/talos-output-alpha.txt'
- 'stderr 2 > /tmp/talos-error-beta.log'
- 'stderr 2 >> /tmp/talos-error-beta.log'

The accepted direct path-form two-stage combined pipeline witnesses remain
exactly:

- '/bin/stdin < /etc/banner.txt | /bin/stdin 1 > /tmp/talos-pipeline-output-alpha.txt'
- '/bin/stdin < /etc/banner.txt | /bin/stdin 1 >> /tmp/talos-pipeline-output-alpha.txt'
- '/bin/stdin < /etc/banner.txt | /bin/stderr 2 > /tmp/talos-pipeline-error-beta.log'
- '/bin/stdin < /etc/banner.txt | /bin/stderr 2 >> /tmp/talos-pipeline-error-beta.log'

The accepted fixed-/bin bare-name two-stage combined pipeline witnesses remain
exactly:

- 'stdin < /etc/banner.txt | stdin 1 > /tmp/talos-pipeline-output-alpha.txt'
- 'stdin < /etc/banner.txt | stdin 1 >> /tmp/talos-pipeline-output-alpha.txt'
- 'stdin < /etc/banner.txt | stderr 2 > /tmp/talos-pipeline-error-beta.log'
- 'stdin < /etc/banner.txt | stderr 2 >> /tmp/talos-pipeline-error-beta.log'

All successful behavior remains descriptor-backed through VFS executable
open/read, userspace launch/status, child-only descriptor rebinding, safe
volatile-vfs /tmp leaf output files, append-at-EOF readback, pipe handoff for
the accepted combined pipeline witnesses, lifecycle/status observations, and
descriptor restoration controls. Bare-name forms resolve only through the fixed
bounded /bin lookup before using the same accepted execution path.

## Findings

- fixed: Added this checkpoint record and task-owned classification/evidence
  map so the accepted explicit-fd separated redirection-token frontier is
  durable independently of transient supervisor state.
- not-an-issue: The four predecessor task records, task-owned JSON, retained
  QEMU/substitute transcripts, and docs agree that accepted explicit-fd
  separated-token behavior is limited to an fd token followed by an operator
  token and path token for the exact listed direct, bare-name, direct pipeline,
  and bare-name pipeline witnesses.
- not-an-issue: Successful command-visible behavior remains backed by
  descriptor/VFS/userspace execution; this checkpoint found no fake or
  kernel-expanded command behavior in the accepted surface.
- deferred: Persistent writable filesystem behavior, nested/traversal paths,
  paths outside volatile /tmp, explicit fd input redirection, fd
  duplication/close syntax, PATH/current-directory lookup, command lookup
  beyond bounded /bin, arbitrary shell grammar, live networking/SSH, Pi 5
  hardware proof, generated-root retry, boot publication, and phase transition
  remain outside this checkpoint.
- deferred: No later queued same-lane local POSIX/VFS task is mechanically
  objective after this checkpoint; supervisor planning is required before the
  worker promotes another task.

## Evidence

- static inspection: accepted predecessor task records and JSON evidence were
  reconciled:
  tasks/2026-06-28-phase12-local-direct-explicit-fd-separated-redirection-token-core.md,
  tasks/2026-06-28-phase12-local-bare-name-explicit-fd-separated-redirection-token-core.md,
  tasks/2026-06-28-phase12-local-direct-pipeline-explicit-fd-separated-redirection-token-core.md,
  tasks/2026-06-28-phase12-local-bare-name-pipeline-explicit-fd-separated-redirection-token-core.md,
  and their task-owned evidence directories.
- direct predecessor commit: 012232121a5e0422ba1cf953f2358603bbc17796.
- bare-name predecessor commit: 10f4d7192a722b5f7dc912b127fe5f3909125232.
- direct pipeline predecessor commit:
  dded392b2ff3e9e57f93851d2df20c4acc8e1250.
- bare-name pipeline predecessor commit:
  4fb4cdd4625a2633d059bee5c7be73ed60901008.
- retained QEMU/substitute transcripts:
  tasks/evidence/2026-06-28-phase12-local-direct-explicit-fd-separated-redirection-token-core/qemu-substitute-local-command-loop.log,
  tasks/evidence/2026-06-28-phase12-local-bare-name-explicit-fd-separated-redirection-token-core/qemu-substitute-local-command-loop.log,
  tasks/evidence/2026-06-28-phase12-local-direct-pipeline-explicit-fd-separated-redirection-token-core/qemu-substitute-local-command-loop.log,
  and
  tasks/evidence/2026-06-28-phase12-local-bare-name-pipeline-explicit-fd-separated-redirection-token-core/qemu-substitute-local-command-loop.log.
- retained checkpoint QEMU/substitute transcript:
  tasks/evidence/2026-06-28-phase12-local-explicit-fd-separated-redirection-token-frontier-checkpoint/qemu-substitute-local-command-loop.log.
- task-owned checkpoint JSON:
  tasks/evidence/2026-06-28-phase12-local-explicit-fd-separated-redirection-token-frontier-checkpoint/classification.json
  and evidence-map.json.

## Accepted Boundary

The accepted explicit-fd separated redirection-token frontier is limited to fd
token followed by operator token followed by path token for these local-only
surfaces:

~~~text
/bin/stdout 1 > /tmp/talos-output-alpha.txt
/bin/stdout 1 >> /tmp/talos-output-alpha.txt
/bin/stderr 2 > /tmp/talos-error-beta.log
/bin/stderr 2 >> /tmp/talos-error-beta.log
stdout 1 > /tmp/talos-output-alpha.txt
stdout 1 >> /tmp/talos-output-alpha.txt
stderr 2 > /tmp/talos-error-beta.log
stderr 2 >> /tmp/talos-error-beta.log
/bin/stdin < /etc/banner.txt | /bin/stdin 1 > /tmp/talos-pipeline-output-alpha.txt
/bin/stdin < /etc/banner.txt | /bin/stdin 1 >> /tmp/talos-pipeline-output-alpha.txt
/bin/stdin < /etc/banner.txt | /bin/stderr 2 > /tmp/talos-pipeline-error-beta.log
/bin/stdin < /etc/banner.txt | /bin/stderr 2 >> /tmp/talos-pipeline-error-beta.log
stdin < /etc/banner.txt | stdin 1 > /tmp/talos-pipeline-output-alpha.txt
stdin < /etc/banner.txt | stdin 1 >> /tmp/talos-pipeline-output-alpha.txt
stdin < /etc/banner.txt | stderr 2 > /tmp/talos-pipeline-error-beta.log
stdin < /etc/banner.txt | stderr 2 >> /tmp/talos-pipeline-error-beta.log
~~~

The output path policy remains absolute safe volatile /tmp leaf files only:
non-empty basename, no nested slash, no dot/dotdot basename, no writes outside
volatile /tmp, and no cross-stream reserved basename alias. The input path for
the accepted pipeline producer forms remains initramfs:/etc/banner.txt.
Negative controls from the predecessor tasks prove missing operands,
unsupported paths, unsupported operators, unsupported command/stage names,
wrong fd/command pairings, unsupported fd tokens, producer/output misuse, mixed
direct/bare stages, explicit fd input redirection, and arbitrary grammar fail
before file creation/write or new successful process records.

selected_next_task=null.
planningNeeded=true because no later queued same-lane local POSIX/VFS task is
mechanically objective after this checkpoint.

Live network/SSH remains paused. No Pi 5 hardware/lab action, boot publication,
generated-root retry, persistence, arbitrary paths, PATH/current-directory
lookup, or phase transition was performed or claimed.

## Validation

- static inspection of task records, retained evidence paths, docs, and git
  diff: passed.
- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet local_command_loop: passed; 887 Talos
  no_std tests.
- jq empty on task-owned classification/evidence JSON: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed; search index size warning
  retained.
- git diff --cached --check: passed before commit.

Commit: recorded in talos-supervisor-state.json acceptance evidence.
