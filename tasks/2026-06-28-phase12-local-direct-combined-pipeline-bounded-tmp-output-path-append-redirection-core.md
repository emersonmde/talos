# Phase 12 Local Direct Combined Pipeline Bounded Tmp Output-Path Append Redirection Core

Task:
phase12-local-direct-combined-pipeline-bounded-tmp-output-path-append-redirection-core-20260628

Status: accepted and committed; durable supervisor state records the final commit SHA.

## Summary

Accepted the direct path-form combined pipeline bounded volatile /tmp output-path append slice.
The exact direct witnesses now succeed:

- '/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/talos-pipeline-output-alpha.txt'
- '/bin/stdin </etc/banner.txt | /bin/stdin >>/tmp/talos-pipeline-output-alpha.txt'
- '/bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/talos-pipeline-error-beta.log'
- '/bin/stdin </etc/banner.txt | /bin/stderr 2>>/tmp/talos-pipeline-error-beta.log'

Each witness uses descriptor-backed VFS/userspace execution, producer fd0 from
initramfs:/etc/banner.txt, producer fd1 as the serialized pipe endpoint,
final-stage fd0 from the pipe, child-only final-stage fd1 or fd2 rebinding to a
volatile-vfs file, append-at-EOF semantics for the second write, and later
descriptor restoration controls.

## Findings

- fixed: Direct absolute pipeline consumer parsing now accepts the task-owned
  safe volatile /tmp leaf stdout/stderr output paths for the final stage.
- fixed: The direct combined-pipeline acceptance guards now include the
  task-owned stdout and stderr bounded /tmp leaf append targets without enabling
  the fixed-/bin bare-name successor.
- fixed: Added QEMU/substitute local_command_loop coverage for both stdout and
  stderr truncate-then-append witnesses, descriptor-backed cat readback,
  lifecycle/status/process observations, descriptor restoration controls, and
  deterministic negative controls.
- not-an-issue: Existing fixed-path combined pipeline append, bounded /tmp
  command output-path append, stdin redirection, process-status VFS, ps,
  waitpid, laststatus, and pipestatus regressions remain passing in the
  local_command_loop gate.
- deferred: Fixed-/bin bare-name combined pipeline bounded /tmp output-path
  append redirection is the selected next task.
- deferred: Persistent writable filesystem behavior, nested or traversal paths,
  paths outside volatile /tmp, broad shell grammar, live networking/SSH, Pi 5
  hardware proof, generated-root retry, boot publication, and phase transition
  remain outside this slice.

## Evidence

- static inspection: src/local_command_loop.rs changes are limited to direct
  absolute path-form pipeline consumer parsing, direct combined-pipeline
  acceptance guards, and task-owned tests.
- QEMU/substitute local shell smoke: 'cargo -Zjson-target-spec test --quiet
  local_command_loop' passed with 879 talos no_std tests; transcript retained at
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-bounded-tmp-output-path-append-redirection-core/qemu-substitute-local-command-loop.log.
- fmt: 'cargo fmt --all -- --check' passed.
- evidence JSON:
  tasks/evidence/2026-06-28-phase12-local-direct-combined-pipeline-bounded-tmp-output-path-append-redirection-core/classification.json
  and evidence-map.json.

## Accepted Boundary

The accepted direct path-form combined pipeline sequence is exactly:

~~~text
/bin/stdin </etc/banner.txt | /bin/stdin >/tmp/talos-pipeline-output-alpha.txt
/bin/stdin </etc/banner.txt | /bin/stdin >>/tmp/talos-pipeline-output-alpha.txt
/bin/stdin </etc/banner.txt | /bin/stderr 2>/tmp/talos-pipeline-error-beta.log
/bin/stdin </etc/banner.txt | /bin/stderr 2>>/tmp/talos-pipeline-error-beta.log
~~~

Descriptor-backed cat readback reports 0xc4 bytes for the stdout target and
0x3e bytes for the stderr target, proving truncate-then-append ordering. The
negative controls cover mixed direct/bare forms, unsupported commands, paths
outside /tmp, nested/traversal or empty/dot basenames, malformed append
grammar, and cross-stream reserved basename aliases; they fail before file
creation/write or new successful process records.

selected_next_task=phase12-local-bare-name-combined-pipeline-bounded-tmp-output-path-append-redirection-core-20260628.

Live network/SSH remains paused. No Pi 5 hardware claim is made.
