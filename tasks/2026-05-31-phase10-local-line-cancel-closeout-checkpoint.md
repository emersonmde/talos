# Phase 10 Local Line-Cancel Closeout Checkpoint Task

Task: phase10-local-line-cancel-closeout-checkpoint-20260531

Status: accepted

## Goal

Checkpoint the accepted Ctrl-C local line-cancel feature and record the next
smallest feature-led local interactivity recommendation.

## Scope

This was documentation-only closeout work. It reconciled the accepted
QEMU/substitute line-cancel core, serialized Raspberry Pi 5 proof, retained
evidence, descriptor-backed stdin/stdout frontier, prompt-local Ctrl-C
cancellation semantics, deferred terminal/shell/userspace/filesystem surfaces,
and next feature-led recommendation.

No implementation changed. No QEMU scenario was rerun. No Pi 5 hardware action,
boot archive publication, or hardwareTestLock acquisition was performed.

## Evidence Map

- Closeout doc:
  docs/src/project/phase10-local-line-cancel-closeout-checkpoint.md.
- QEMU/substitute line-cancel evidence:
  tasks/evidence/2026-05-31-qemu-local-line-cancel-core/qemu-local-line-cancel-smoke.log.
- QEMU/substitute Backspace/Delete regression evidence:
  tasks/evidence/2026-05-31-qemu-local-line-editing-core/qemu-local-line-editing-smoke.log.
- Pi 5 accepted line-cancel serial transcript through PASS:
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/serial-transcript-through-pass.txt.
- Pi 5 accepted line-cancel proof summary:
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/proof-result-local3.txt.
- Pi 5 accepted line-cancel proof lines:
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/proof-lines-local3.txt.
- Pi 5 archive/image review:
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/archive-review.txt.
- Pi 5 fresh TFTP proof:
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/tftp-kernel-fetch-local3.txt.
- Pi 5 restore proof:
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/post-snapshot-restore-status.json.
- Inconclusive-run triage evidence:
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local1-clean-candidate/,
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local1-known-good-control/,
  and
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local2-unchanged-candidate-rerun/.

## Accepted Frontier

The accepted frontier is descriptor-backed serial prompt input with bounded
Ctrl-C line cancellation before dispatch. The command loop can accept partial
typed input, discard that line on Ctrl-C 0x03, print a short cancellation
response, return to a fresh prompt, accept a following command, dispatch the
kernel-backed command after Enter, print the visible response through
descriptor-backed stdout, and return to a ready prompt.

The checkpoint keeps POSIX signals, process interruption, terminal sessions,
foreground process groups, termios, job control, cursor addressing, shell
history, kill/yank editing, broad escape parsing, arrow keys, userspace shell
execution, process spawning, filesystem-backed commands, 'cd', path traversal,
VFS lookup, directory listing, broad POSIX read/stdio, quoting/escaping,
globbing, argv/envp process ABI, pipes/redirection, networking, SSH, RP1/PCIe,
UART interrupts, DMA/cache policy, and paused Phase 8 proof-only work
deferred.

## Next Recommendation

Recommend a bounded Ctrl-U prompt-local line-kill feature as the next Phase 10
local interactivity slice: start typing at the prompt, press Ctrl-U, discard
the editable line, visibly return to a fresh prompt, and accept the next
command normally. The follow-up should not claim POSIX signals, process
interruption, termios, job control, history/yank behavior, userspace shell
execution, process spawning, or filesystem command lookup.

## Validation

- static inspection: retained line-cancel evidence paths reviewed.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware lock: hardwareTestLock remained unlocked/restored and unused.
