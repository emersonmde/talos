# Phase 12 Local Bare-Name Combined Pipeline Bounded Tmp Output-Path Append Redirection Core

Task:
phase12-local-bare-name-combined-pipeline-bounded-tmp-output-path-append-redirection-core-20260628

Status: accepted and committed; durable supervisor state records the final commit SHA.

## Summary

Accepted the fixed-/bin bare-name combined pipeline bounded volatile /tmp
output-path append slice. The exact bare-name witnesses now succeed after both
stages resolve through bounded /bin lookup:

- 'stdin </etc/banner.txt | stdin >/tmp/talos-pipeline-output-alpha.txt'
- 'stdin </etc/banner.txt | stdin >>/tmp/talos-pipeline-output-alpha.txt'
- 'stdin </etc/banner.txt | stderr 2>/tmp/talos-pipeline-error-beta.log'
- 'stdin </etc/banner.txt | stderr 2>>/tmp/talos-pipeline-error-beta.log'

Each witness uses descriptor-backed VFS/userspace execution, producer fd0 from
initramfs:/etc/banner.txt, producer fd1 as the serialized pipe endpoint,
final-stage fd0 from the pipe, child-only final-stage fd1 or fd2 rebinding to a
volatile-vfs file, append-at-EOF semantics for the second write, closed loader
temporaries, process/status observations, and later descriptor restoration
controls.

## Findings

- fixed: Bare-name pipeline consumer parsing now accepts safe volatile /tmp leaf
  stdout/stderr output paths for the final stage instead of only the older fixed
  combined pipeline filenames.
- fixed: Added QEMU/substitute local_command_loop coverage for both accepted
  bare-name stdout and stderr truncate-then-append witnesses, descriptor-backed
  cat readback, lifecycle/status/process observations, descriptor restoration,
  and deterministic negative controls.
- removed: Dead exact-pipeline volatile path constructor helpers that became
  unused after the safe /tmp leaf parser path replaced them.
- not-an-issue: Direct path-form combined pipeline bounded /tmp output-path
  append, fixed-path combined pipeline append, bounded /tmp command output-path
  append, stdin redirection, process-status VFS, ps, waitpid, laststatus, and
  pipestatus regressions remain passing in the local_command_loop gate.
- deferred: Persistent writable filesystem behavior, nested/traversal paths,
  paths outside volatile /tmp, separated redirection-token grammar, explicit
  alternate fd syntax, mixed direct/bare forms, broad shell grammar, PATH/current
  directory lookup, command lookup beyond bounded /bin, live networking/SSH, Pi 5
  hardware proof, generated-root retry, boot publication, and phase transition
  remain outside this slice.
- deferred: The combined pipeline bounded /tmp output-path append frontier
  checkpoint is the selected next task.

## Evidence

- static inspection: src/local_command_loop.rs changes are limited to bare-name
  pipeline consumer parsing, dead-code removal, and task-owned tests.
- QEMU/substitute local shell smoke: 'cargo -Zjson-target-spec test --quiet
  local_command_loop' passed with 880 talos no_std tests; transcript retained at
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-bounded-tmp-output-path-append-redirection-core/qemu-substitute-local-command-loop.log.
- fmt: 'cargo fmt --all -- --check' passed.
- evidence JSON:
  tasks/evidence/2026-06-28-phase12-local-bare-name-combined-pipeline-bounded-tmp-output-path-append-redirection-core/classification.json
  and evidence-map.json.

## Accepted Boundary

The accepted fixed-/bin bare-name combined pipeline sequence is exactly:

~~~text
stdin </etc/banner.txt | stdin >/tmp/talos-pipeline-output-alpha.txt
stdin </etc/banner.txt | stdin >>/tmp/talos-pipeline-output-alpha.txt
stdin </etc/banner.txt | stderr 2>/tmp/talos-pipeline-error-beta.log
stdin </etc/banner.txt | stderr 2>>/tmp/talos-pipeline-error-beta.log
~~~

Descriptor-backed cat readback reports 0xc4 bytes for the stdout target and
0x3e bytes for the stderr target, proving truncate-then-append ordering. The
negative controls cover unsupported command names, mixed direct/bare forms,
command lookup beyond bounded /bin, paths outside /tmp, nested/traversal or
empty/dot basenames, malformed append grammar, and cross-stream reserved
basename aliases; they fail before file creation/write or new successful process
records.

selected_next_task=phase12-local-combined-pipeline-bounded-tmp-output-path-append-redirection-frontier-checkpoint-20260628.

Live network/SSH remains paused. No Pi 5 hardware claim is made.
