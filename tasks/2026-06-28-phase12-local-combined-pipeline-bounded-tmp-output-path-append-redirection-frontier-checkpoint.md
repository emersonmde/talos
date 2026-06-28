# Phase 12 Local Combined Pipeline Bounded Tmp Output-Path Append Redirection Frontier Checkpoint

Task:
phase12-local-combined-pipeline-bounded-tmp-output-path-append-redirection-frontier-checkpoint-20260628

Status: accepted and committed; durable supervisor state records the final commit SHA.

## Summary

Checkpointed the accepted local-only combined pipeline bounded volatile /tmp
output-path append redirection frontier after the direct path-form and
fixed-/bin bare-name cores. This checkpoint adds no runtime behavior.

The accepted direct path-form witnesses remain exactly:

- '/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/talos-pipeline-output-alpha.txt'
- '/bin/stdin </etc/banner.txt | /bin/stdin >>/tmp/talos-pipeline-output-alpha.txt'
- '/bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/talos-pipeline-error-beta.log'
- '/bin/stdin </etc/banner.txt | /bin/stderr 2>>/tmp/talos-pipeline-error-beta.log'

The accepted fixed-/bin bare-name witnesses remain exactly:

- 'stdin </etc/banner.txt | stdin >/tmp/talos-pipeline-output-alpha.txt'
- 'stdin </etc/banner.txt | stdin >>/tmp/talos-pipeline-output-alpha.txt'
- 'stdin </etc/banner.txt | stderr 2>/tmp/talos-pipeline-error-beta.log'
- 'stdin </etc/banner.txt | stderr 2>>/tmp/talos-pipeline-error-beta.log'

Both forms keep descriptor-backed VFS/userspace execution, producer fd0 from
initramfs:/etc/banner.txt, producer fd1 as the serialized pipe endpoint,
final-stage fd0 from that pipe endpoint, child-only final-stage fd1 or fd2
rebinding to a safe volatile-vfs /tmp leaf file, append-at-EOF semantics for
the second write, descriptor-backed cat readback, lifecycle/status/process
observations, and later descriptor restoration controls. Bare-name stages
resolve only through the fixed bounded /bin lookup before using the same
accepted execution path.

## Findings

- fixed: Corrected the predecessor bare-name task record status from
  accepted-pending-commit wording to accepted-and-committed wording so the
  task record matches durable supervisor state and git history.
- fixed: Added this checkpoint record and task-owned classification/evidence
  map so the accepted combined pipeline bounded /tmp output-path append
  frontier is durable independently of transient supervisor state.
- not-an-issue: Direct and bare-name predecessor records, classification JSON,
  evidence maps, and retained transcripts agree on witness strings, volatile
  /tmp targets, 0xc4 stdout readback bytes, 0x3e stderr readback bytes,
  descriptor restoration controls, and no hardware claim.
- deferred: Persistent writable filesystem behavior, nested/traversal paths,
  paths outside volatile /tmp, separated redirection-token grammar, explicit
  alternate fd syntax, mixed direct/bare broadening, PATH/current-directory
  lookup, command lookup beyond bounded /bin, arbitrary shell grammar, live
  networking/SSH, Pi 5 hardware proof, generated-root retry, boot publication,
  and phase transition remain outside this checkpoint.
- deferred: No later queued same-lane local POSIX/VFS task is mechanically
  objective after this checkpoint; supervisor planning is required before the
  worker promotes another task.

## Evidence

- static inspection: accepted predecessor task records and JSON evidence were
  reconciled:
  tasks/2026-06-28-phase12-local-direct-combined-pipeline-bounded-tmp-output-path-append-redirection-core.md,
  tasks/2026-06-28-phase12-local-bare-name-combined-pipeline-bounded-tmp-output-path-append-redirection-core.md,
  and their task-owned evidence directories.
- direct predecessor commit: 413ed733d9a9fe7e5dc3db46e8f4df461055130e.
- bare-name predecessor commit: b431d46141aa1c47da73a7b69c24931570e7bf1b.
- retained direct QEMU/substitute transcript:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-bounded-tmp-output-path-append-redirection-core/qemu-substitute-local-command-loop.log.
- retained bare-name QEMU/substitute transcript:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-bounded-tmp-output-path-append-redirection-core/qemu-substitute-local-command-loop.log.
- task-owned checkpoint JSON:
  tasks/evidence/2026-06-28-phase12-local-combined-pipeline-bounded-tmp-output-path-append-redirection-frontier-checkpoint/classification.json
  and evidence-map.json.

## Accepted Boundary

The accepted frontier is limited to final-stage stdout/stderr truncate then
append redirection for two-stage combined pipelines targeting safe volatile
/tmp leaf files:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/talos-pipeline-output-alpha.txt
/bin/stdin </etc/banner.txt | /bin/stdin >>/tmp/talos-pipeline-output-alpha.txt
/bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/talos-pipeline-error-beta.log
/bin/stdin </etc/banner.txt | /bin/stderr 2>>/tmp/talos-pipeline-error-beta.log
stdin </etc/banner.txt | stdin >/tmp/talos-pipeline-output-alpha.txt
stdin </etc/banner.txt | stdin >>/tmp/talos-pipeline-output-alpha.txt
stdin </etc/banner.txt | stderr 2>/tmp/talos-pipeline-error-beta.log
stdin </etc/banner.txt | stderr 2>>/tmp/talos-pipeline-error-beta.log
~~~

The path policy remains absolute /tmp leaf only: non-empty ASCII basename, no
nested slash, no dot/dotdot basename, no writes outside volatile /tmp, and no
cross-stream reserved basename alias. Negative controls from the predecessor
tasks prove unsupported commands, unsupported lookup beyond bounded /bin,
mixed direct/bare forms, malformed append grammar, unsupported output paths,
nested/traversal paths, and reserved aliases fail before file creation/write or
new successful process records.

selected_next_task=null.
planningNeeded=true because no later queued same-lane local POSIX/VFS task is
mechanically objective after this checkpoint.

Live network/SSH remains paused. No Pi 5 hardware claim is made.

## Validation

- static inspection of task records, retained evidence paths, docs, and git
  diff: passed.
- jq empty on task-owned classification/evidence JSON: passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check: passed before commit.
