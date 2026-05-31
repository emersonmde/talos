# Phase 10 Local Line-Cancel Closeout Checkpoint

Status: accepted

Task: phase10-local-line-cancel-closeout-checkpoint-20260531

## Scope

This checkpoint closes out the bounded Ctrl-C local line-cancel feature as
documentation-only work. It reconciles the accepted QEMU/substitute
line-cancel core, serialized Raspberry Pi 5 proof, retained evidence,
descriptor-backed stdin and stdout frontier, prompt-local cancellation
semantics, deferred terminal/shell/userspace/filesystem surfaces, and next
feature-led planning recommendation.

No Rust or assembly behavior changed. No QEMU scenario was rerun. No Pi 5
hardware action, boot archive publication, or hardwareTestLock acquisition was
performed.

## Reviewed Evidence

- local line-cancel core implementation, QEMU/substitute evidence, and task
  record commit: 359ffbdf1342979053ede0452ce597bc3cbfc52c.
- Pi 5 local line-cancel proof harness implementation commits:
  cb2a362712f9e8b803d4cd4e0aa98593bde2b49e and
  15dbf99226a231d1ef80663e90f233be4aea7cff.
- Pi 5 local line-cancel proof acceptance commit:
  18d4567a5d08ebe6ae76c0c997e2ac5894755161.
- retained QEMU/substitute transcript:
  tasks/evidence/2026-05-31-qemu-local-line-cancel-core/qemu-local-line-cancel-smoke.log.
- retained Backspace/Delete QEMU/substitute regression transcript:
  tasks/evidence/2026-05-31-qemu-local-line-editing-core/qemu-local-line-editing-smoke.log.
- retained Pi 5 accepted serial transcript through PASS:
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/serial-transcript-through-pass.txt.
- retained Pi 5 accepted proof summary:
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/proof-result-local3.txt.
- retained Pi 5 accepted proof lines:
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/proof-lines-local3.txt.
- retained Pi 5 archive/image review:
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/archive-review.txt.
- retained Pi 5 fresh TFTP proof:
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/tftp-kernel-fetch-local3.txt.
- retained Pi 5 restore proof:
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local3-candidate-visible-cancel/post-snapshot-restore-status.json.
- retained inconclusive-run triage:
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local1-clean-candidate/,
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local1-known-good-control/,
  and
  tasks/evidence/2026-05-31-pi5-local-line-cancel-proof/local2-unchanged-candidate-rerun/.

The retained QEMU/substitute transcript contains partial 'bogus' input,
Ctrl-C 0x03 cancellation, visible 'talos: line-canceled', a fresh 'talos>'
prompt, following 'pwd' dispatch, visible '/' output, descriptor-backed
fd0/stdout markers, next-prompt readiness, final classification
'qemu-local-line-cancel-complete', and exact PASS line
'qemu-local-line-cancel: PASS'. The retained Backspace/Delete regression keeps
the previous accepted erase-byte correction path covered.

The retained Pi 5 proof summary contains source commit
15dbf99226a231d1ef80663e90f233be4aea7cff, accepted candidate archive digest
39791d161a9466fee248ca83a8175415db6f1089f40dbb8272ddbff41bbae854,
candidate kernel digest
8e47209f4248ea04fdfe005892a4ec2f346cb52d2535c8fdb98d3fd3345c6f75, kernel
size 99552 bytes, 'typed_cancel=bogus_ctrl_c', 'typed_command=pwd', successful
serial interaction, visible cancellation response, visible '/' output,
descriptor-backed input and output markers, prompt readiness, final
classification 'pi5-local-line-cancel-complete', result PASS, fresh
'da591740/kernel_2712.img' TFTP fetch evidence, and restore tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The first local1 Pi 5 candidate run is retained only as inconclusive evidence.
Before accepting the hardware proof, triage recorded candidate identity, fresh
serial cursor, TFTP delta, a conclusive restored-tree known-good control, and
an unchanged candidate rerun. The accepted local3 run supplied the required
visible cancellation transcript after a proof-local visibility adjustment.

## Accepted Frontier

The accepted capability is bounded Ctrl-C prompt cancellation over the
descriptor-backed serial command-loop path. Talos can print the serial prompt,
accept typed command bytes through fd0/runtime-console0 canonical-lite input,
treat Ctrl-C 0x03 as a prompt-local cancel event, discard the partially
collected editable line before Enter dispatch, print a short
'talos: line-canceled' response through descriptor-backed stdout, return to a
fresh 'talos>' prompt, accept the next typed command, dispatch the following
kernel-backed built-in, write the visible response through descriptor-backed
stdout, and return to a ready prompt.

The accepted feature is intentionally prompt-local. It does not accept POSIX
signal delivery, process interruption, terminal sessions, foreground process
groups, termios policy, screen repainting, shell history, broad
escape-sequence parsing, arrow keys, or general job control.

Existing built-ins remain deterministic: help lists 'pwd', 'echo hello'
prints 'hello', 'pwd' prints '/', 'stdio' reports fd 0/fd 1/fd 2 identity plus
runtime-console0 backing and descriptor-backed markers, empty input reports
'talos: empty-command', unknown input reports 'talos: unknown-command', and
unexpected arguments to non-argument built-ins report
'talos: unexpected-argument'.

This frontier is still a kernel-backed local command loop. It is not accepted
userspace shell execution, process spawning, external command lookup,
filesystem-backed command execution, broad POSIX read/stdio behavior, or a
general shell parser.

## Deferred Surfaces

Still blocked after this checkpoint:

- POSIX signal delivery, process interruption, terminal sessions, foreground
  process groups, termios, job control, and signal restart behavior.
- cursor addressing, screen repainting, shell history, kill/yank editing,
  broad escape-sequence parsing, arrow keys, and readline-style editing.
- userspace shell execution.
- external command execution, process spawning, exec/wait/exit, and process
  lifecycle integration.
- filesystem-backed commands, file inspection from the shell, 'cd',
  path traversal, VFS lookup, directory listing, and writable filesystem state.
- broad argv/envp as process startup ABI, token vectors, quoting/escaping,
  globbing, environment expansion, pipes, redirection, descriptor inheritance
  across exec, and terminal job control.
- broad POSIX read/stdio readiness.
- persistent local configuration.
- networking and SSH.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.
- resumed Phase 8 proof-only work unless a later feature-led task directly
  needs it for local interactivity.

## Next Planning Recommendation

The next smallest feature-led Phase 10 task should add a bounded Ctrl-U
prompt-local line-kill behavior on the same descriptor-backed serial input
path. The user-visible feature should be: start typing a command at the
'talos>' prompt, press Ctrl-U, discard the entire editable line, print a short
kernel-backed line-killed response or otherwise visibly return to a fresh
prompt, and then accept and dispatch the next typed command normally.

That follow-up should stay prompt-local. It should not claim POSIX signal
delivery, process interruption, terminal sessions, termios, job control,
history, yank/paste behavior, userspace shell execution, process spawning, or
filesystem command lookup. Old Phase 8 proof-only work remains paused unless
it directly unblocks this local interactivity feature.

## Validation

- static inspection: reconciled retained QEMU/substitute and Pi 5 line-cancel
  evidence paths from the accepted core and hardware proof.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

HardwareTestLock remained unlocked/restored and unused by this checkpoint.
