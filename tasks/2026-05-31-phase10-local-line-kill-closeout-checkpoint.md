# Phase 10 Local Line-Kill Closeout Checkpoint Task

Task: phase10-local-line-kill-closeout-checkpoint-20260531

Status: accepted

## Goal

Checkpoint the accepted Ctrl-U local line-kill feature and record the next
smallest feature-led local interactivity recommendation.

## Scope

This was documentation-only closeout work. It reconciled the accepted
QEMU/substitute line-kill core, serialized Raspberry Pi 5 proof, retained
evidence, descriptor-backed stdin/stdout frontier, prompt-local Ctrl-U
whole-line discard semantics, regressions for Backspace/Delete and Ctrl-C,
deferred terminal/shell/userspace/filesystem surfaces, and next feature-led
recommendation.

No implementation changed. No QEMU scenario was rerun. No Pi 5 hardware action,
boot archive publication, or hardwareTestLock acquisition was performed.

## Evidence Map

- Closeout doc:
  docs/src/project/phase10-local-line-kill-closeout-checkpoint.md.
- QEMU/substitute line-kill evidence:
  tasks/evidence/2026-05-31-qemu-local-line-kill-core/qemu-local-line-kill-smoke.log.
- QEMU/substitute Ctrl-C line-cancel regression evidence:
  tasks/evidence/2026-05-31-qemu-local-line-cancel-core/qemu-local-line-cancel-smoke.log.
- QEMU/substitute Backspace/Delete regression evidence:
  tasks/evidence/2026-05-31-qemu-local-line-editing-core/qemu-local-line-editing-smoke.log.
- Pi 5 accepted line-kill serial transcript through PASS:
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/serial-transcript-through-pass.txt.
- Pi 5 accepted line-kill proof summary:
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/proof-result-local6.txt.
- Pi 5 accepted line-kill proof lines:
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/proof-lines-local6.txt.
- Pi 5 archive/image review:
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/archive-review.txt.
- Pi 5 fresh TFTP proof:
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/tftp-kernel-fetch-local6.txt.
- Pi 5 restore proof:
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/post-snapshot-restore-status.json.
- Inconclusive-run triage evidence:
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local1-known-good-control/,
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local2-unchanged-candidate-rerun/,
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local3-known-good-control/,
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local3-candidate-flushed-metadata/,
  and
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local5-unchanged-flushed-candidate-rerun/.

## Accepted Frontier

The accepted frontier is descriptor-backed serial prompt input with bounded
Ctrl-U whole-line discard before dispatch. The command loop can accept partial
typed input, discard that editable line on Ctrl-U 0x15, print a short
line-killed response, accept a following command, dispatch the kernel-backed
command after Enter, print the visible response through descriptor-backed
stdout, and return to a ready prompt.

The checkpoint keeps POSIX signals, process interruption, terminal sessions,
foreground process groups, termios, job control, cursor addressing, screen
repainting, shell history, kill/yank beyond bounded Ctrl-U line kill, broad
escape parsing, arrow keys, userspace shell execution, process spawning,
filesystem-backed commands, 'cd', path traversal, VFS lookup, directory
listing, broad POSIX read/stdio, quoting/escaping, globbing, argv/envp process
ABI, pipes/redirection, networking, SSH, RP1/PCIe, UART interrupts, DMA/cache
policy, and paused Phase 8 proof-only work deferred.

## Next Recommendation

Recommend a bounded literal echo tail feature as the next Phase 10 local
interactivity slice: type 'echo local serial works' at the prompt, press
Enter, dispatch through fd0/runtime-console0, print 'local serial works'
through descriptor-backed stdout, and return to a ready prompt. The follow-up
should stay narrow and should not claim broad shell tokenization,
quoting/escaping/globbing, environment expansion, argv/envp process ABI,
userspace shell execution, process spawning, filesystem command lookup, or
terminal/session semantics.

## Validation

- static inspection: retained line-kill evidence paths reviewed.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware lock: hardwareTestLock remained unlocked/restored and unused.

Acceptance commit: recorded in durable supervisor state after commit creation.
