# Phase 12 Local Separated Redirection Token Frontier Checkpoint

Task:
phase12-local-separated-redirection-token-frontier-checkpoint-20260628

Status: accepted and committed; durable supervisor state records the final
commit SHA.

## Summary

Checkpointed the accepted local-only separated redirection-token grammar
frontier after the direct, fixed-/bin bare-name, direct pipeline, and
fixed-/bin bare-name pipeline core tasks. This checkpoint adds no runtime
behavior.

The accepted direct path-form witnesses remain exactly:

- '/bin/stdin < /etc/banner.txt'
- '/bin/stdout > /tmp/talos-output-alpha.txt'
- '/bin/stdout >> /tmp/talos-output-alpha.txt'
- '/bin/stderr 2> /tmp/talos-error-beta.log'
- '/bin/stderr 2>> /tmp/talos-error-beta.log'

The accepted fixed-/bin bare-name witnesses remain exactly:

- 'stdin < /etc/banner.txt'
- 'stdout > /tmp/talos-output-alpha.txt'
- 'stdout >> /tmp/talos-output-alpha.txt'
- 'stderr 2> /tmp/talos-error-beta.log'
- 'stderr 2>> /tmp/talos-error-beta.log'

The accepted direct path-form two-stage combined pipeline witnesses remain
exactly:

- '/bin/stdin < /etc/banner.txt | /bin/stdin > /tmp/talos-pipeline-output-alpha.txt'
- '/bin/stdin < /etc/banner.txt | /bin/stdin >> /tmp/talos-pipeline-output-alpha.txt'
- '/bin/stdin < /etc/banner.txt | /bin/stderr 2> /tmp/talos-pipeline-error-beta.log'
- '/bin/stdin < /etc/banner.txt | /bin/stderr 2>> /tmp/talos-pipeline-error-beta.log'

The accepted fixed-/bin bare-name two-stage combined pipeline witnesses remain
exactly:

- 'stdin < /etc/banner.txt | stdin > /tmp/talos-pipeline-output-alpha.txt'
- 'stdin < /etc/banner.txt | stdin >> /tmp/talos-pipeline-output-alpha.txt'
- 'stdin < /etc/banner.txt | stderr 2> /tmp/talos-pipeline-error-beta.log'
- 'stdin < /etc/banner.txt | stderr 2>> /tmp/talos-pipeline-error-beta.log'

All successful behavior remains descriptor-backed through VFS executable
open/read, userspace launch/status, child-only descriptor rebinding, safe
volatile-vfs /tmp leaf output files, append-at-EOF readback where applicable,
pipe handoff for the accepted combined pipeline witnesses, lifecycle/status
observations, and descriptor restoration controls. Bare-name forms resolve only
through the fixed bounded /bin lookup before using the same accepted execution
path.

## Findings

- fixed: Added this checkpoint record and task-owned classification/evidence
  map so the accepted separated redirection-token frontier is durable
  independently of transient supervisor state.
- fixed: Corrected the retained direct path-form pipeline separated-token
  regression control in the bare-name pipeline separated-token test to use a
  truncate witness after the prerequisite readback, avoiding a bounded
  volatile-vfs file capacity collision while still proving the accepted direct
  pipeline separated-token control remains passing.
- not-an-issue: The four predecessor task records, task-owned JSON, retained
  QEMU/substitute transcripts, and docs agree that accepted separated-token
  behavior is limited to an operator token followed by its path operand for the
  exact listed direct, bare-name, direct pipeline, and bare-name pipeline
  witnesses.
- not-an-issue: Successful command-visible behavior remains backed by
  descriptor/VFS/userspace execution; this checkpoint found no fake or
  kernel-expanded command behavior in the accepted surface.
- deferred: Persistent writable filesystem behavior, nested/traversal paths,
  paths outside volatile /tmp, separated explicit fd syntax, mixed direct/bare
  broadening, multistage pipelines, PATH/current-directory lookup, command
  lookup beyond bounded /bin, arbitrary shell grammar, live networking/SSH, Pi
  5 hardware proof, generated-root retry, boot publication, and phase
  transition remain outside this checkpoint.
- deferred: No later queued same-lane local POSIX/VFS task is mechanically
  objective after this checkpoint; supervisor planning is required before the
  worker promotes another task.

## Evidence

- static inspection: accepted predecessor task records and JSON evidence were
  reconciled:
  tasks/2026-06-28-phase12-local-direct-separated-redirection-token-core.md,
  tasks/2026-06-28-phase12-local-bare-name-separated-redirection-token-core.md,
  tasks/2026-06-28-phase12-local-direct-pipeline-separated-redirection-token-core.md,
  tasks/2026-06-28-phase12-local-bare-name-pipeline-separated-redirection-token-core.md,
  and their task-owned evidence directories.
- direct predecessor commit: c013c5d371bc05b8a6818dd7ce7114ca6312e8a1.
- bare-name predecessor commit: eabb53738865fae13ab61e251bbab7103cba67a8.
- direct pipeline predecessor commit:
  94f45281c365e77e7a812c198d297bf7137153ea.
- bare-name pipeline predecessor commit:
  eba7120c83ed2db23eabf5cc24ed575389ad5368.
- retained QEMU/substitute transcripts:
  tasks/evidence/2026-06-28-phase12-local-direct-separated-redirection-token-core/qemu-substitute-local-command-loop.log,
  tasks/evidence/2026-06-28-phase12-local-bare-name-separated-redirection-token-core/qemu-substitute-local-command-loop.log,
  tasks/evidence/2026-06-28-phase12-local-direct-pipeline-separated-redirection-token-core/qemu-substitute-local-command-loop.log,
  and
  tasks/evidence/2026-06-28-phase12-local-bare-name-pipeline-separated-redirection-token-core/qemu-substitute-local-command-loop.log.
- retained checkpoint QEMU/substitute transcript:
  tasks/evidence/2026-06-28-phase12-local-separated-redirection-token-frontier-checkpoint/qemu-substitute-local-command-loop.log.
- task-owned checkpoint JSON:
  tasks/evidence/2026-06-28-phase12-local-separated-redirection-token-frontier-checkpoint/classification.json
  and evidence-map.json.

## Accepted Boundary

The accepted separated redirection-token frontier is limited to operator token
followed by path token for these local-only surfaces:

~~~text
/bin/stdin < /etc/banner.txt
/bin/stdout > /tmp/talos-output-alpha.txt
/bin/stdout >> /tmp/talos-output-alpha.txt
/bin/stderr 2> /tmp/talos-error-beta.log
/bin/stderr 2>> /tmp/talos-error-beta.log
stdin < /etc/banner.txt
stdout > /tmp/talos-output-alpha.txt
stdout >> /tmp/talos-output-alpha.txt
stderr 2> /tmp/talos-error-beta.log
stderr 2>> /tmp/talos-error-beta.log
/bin/stdin < /etc/banner.txt | /bin/stdin > /tmp/talos-pipeline-output-alpha.txt
/bin/stdin < /etc/banner.txt | /bin/stdin >> /tmp/talos-pipeline-output-alpha.txt
/bin/stdin < /etc/banner.txt | /bin/stderr 2> /tmp/talos-pipeline-error-beta.log
/bin/stdin < /etc/banner.txt | /bin/stderr 2>> /tmp/talos-pipeline-error-beta.log
stdin < /etc/banner.txt | stdin > /tmp/talos-pipeline-output-alpha.txt
stdin < /etc/banner.txt | stdin >> /tmp/talos-pipeline-output-alpha.txt
stdin < /etc/banner.txt | stderr 2> /tmp/talos-pipeline-error-beta.log
stdin < /etc/banner.txt | stderr 2>> /tmp/talos-pipeline-error-beta.log
~~~

The output path policy remains absolute safe volatile /tmp leaf files only:
non-empty basename, no nested slash, no dot/dotdot basename, no writes outside
volatile /tmp, and no cross-stream reserved basename alias. The input path for
the accepted stdin forms remains initramfs:/etc/banner.txt. Negative controls
from the predecessor tasks prove missing operands, unsupported paths,
unsupported operators, unsupported command names, consumer-only separated
pipeline redirection, mixed direct/bare stages, mixed no-space/separated
pipeline stages, and separated explicit fd syntax fail before file
creation/write or new successful process records.

selected_next_task=null.
planningNeeded=true because no later queued same-lane local POSIX/VFS task is
mechanically objective after this checkpoint.

Live network/SSH remains paused. No Pi 5 hardware/lab action, boot publication,
generated-root retry, persistence, arbitrary paths, separated explicit fd
syntax, or phase transition was performed or claimed.

## Validation

- static inspection of task records, retained evidence paths, docs, and git
  diff: passed.
- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test --quiet local_command_loop: passed; 883 Talos
  no_std tests.
- jq empty on task-owned classification/evidence JSON: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed; search index size warning
  retained.
- git diff --cached --check: passed before commit.

Commit: recorded in talos-supervisor-state.json acceptance evidence.
