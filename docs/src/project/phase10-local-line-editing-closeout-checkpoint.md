# Phase 10 Local Line-Editing Closeout Checkpoint

Status: accepted

Task: phase10-local-line-editing-closeout-checkpoint-20260531

## Scope

This checkpoint closes out the bounded Backspace/Delete local line-editing
feature as documentation-only work. It reconciles the accepted QEMU/substitute
line-editing core, serialized Raspberry Pi 5 proof, retained evidence,
descriptor-backed stdin and stdout frontier, erase-byte semantics, deferred
terminal/shell/userspace/filesystem surfaces, and next feature-led planning
recommendation.

No Rust or assembly behavior changed. No QEMU scenario was rerun. No Pi 5
hardware action, boot archive publication, or hardwareTestLock acquisition was
performed.

## Reviewed Evidence

- local line-editing core implementation, QEMU/substitute evidence, and task
  record commit: 31a3281a79994ce97688548b7d0a4a9601e2e6a5.
- Pi 5 local line-editing proof harness implementation commit:
  d7b788caa1f93cf10575fcb9c650e923e660f307.
- Pi 5 local line-editing proof inconclusive-run triage/evidence commit:
  ad41fc5c4123a617a1dad9074a61f017d718fa53.
- Pi 5 local line-editing proof acceptance commit:
  b4fa92c5586cbf30c5823f8e9f8aa533b5e26e2d.
- retained QEMU/substitute transcript:
  tasks/evidence/2026-05-31-qemu-local-line-editing-core/qemu-local-line-editing-smoke.log.
- retained Pi 5 accepted serial transcript through PASS:
  tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local2-unchanged-candidate-rerun/serial-transcript-through-pass.txt.
- retained Pi 5 accepted proof summary:
  tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local2-unchanged-candidate-rerun/proof-result-local2.txt.
- retained Pi 5 accepted raw serial transcript:
  tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local2-unchanged-candidate-rerun/serial-transcript.txt.
- retained Pi 5 archive/image review:
  tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local1-clean-candidate/archive-review.txt.
- retained Pi 5 fresh TFTP proof:
  tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local2-unchanged-candidate-rerun/tftp-kernel-fetch-local2.txt.
- retained Pi 5 restore proof:
  tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local2-unchanged-candidate-rerun/post-restore-status.json.
- retained inconclusive-run triage:
  tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local1-clean-candidate/,
  tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local1-known-good-control/,
  and
  tasks/evidence/2026-05-31-pi5-local-line-editing-proof/local2-known-good-control/.

The retained QEMU/substitute transcript contains 'talos> pwx' plus Backspace
correction to 'pwd', visible '/' output, descriptor-backed fd0/stdout markers,
next-prompt readiness, final classification
'qemu-local-line-editing-complete', and exact PASS line
'qemu-local-line-editing: PASS'. It also proves the same corrected 'pwd'
dispatch after Delete 0x7f and preserves accepted help, status, 'stdio',
'echo hello', empty input, unknown-command, and unexpected-argument behavior.

The retained Pi 5 proof summary contains source commit
d7b788caa1f93cf10575fcb9c650e923e660f307, accepted candidate archive digest
1bde053105052508d32ebd1c9fa8faa959a2b9959ff1b40721dc3859e668adda,
candidate kernel digest
f2b38552e791d7fb23aa8553694d30e914abfa80e8e3bf2b4c03f0547cb97f7e, kernel
size 98944 bytes, 'typed_command=pwx_backspace_d', successful serial write,
one Backspace erase, corrected 'pwd' dispatch, visible '/' output,
descriptor-backed input and output markers, prompt readiness, final
classification 'pi5-local-line-editing-complete', exact PASS line
'rpi5-local-line-editing-proof: PASS', two fresh
'da591740/kernel_2712.img' TFTP fetches at 98944 bytes, and restore tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The first local1 Pi 5 candidate run is retained only as inconclusive evidence.
Before accepting the hardware proof, triage recorded candidate identity, fresh
serial cursor, TFTP delta, a conclusive restored-tree known-good control, and
then reran the unchanged candidate as local2. The local2 unchanged rerun is the
accepted hardware proof.

## Accepted Frontier

The accepted capability is bounded local line editing over the
descriptor-backed serial command-loop path. Talos can print the serial prompt,
accept typed command bytes through fd0/runtime-console0 canonical-lite input,
handle Backspace 0x08 and Delete 0x7f by removing the previous editable byte
before Enter dispatch, dispatch the corrected kernel-backed command, write the
visible response through descriptor-backed stdout, and return to a ready
'talos>' prompt.

The accepted feature is intentionally byte-oriented and prompt-local. It
records erase counts for proof visibility, but it does not accept terminal
cursor addressing, screen repainting, shell history, kill/yank editing, broad
escape-sequence parsing, arrow keys, or termios policy.

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

- termios, terminal sessions, cursor addressing, screen repainting, shell
  history, kill/yank editing, broad escape-sequence parsing, arrow keys, and
  readline-style editing.
- userspace shell execution.
- external command execution, process spawning, exec/wait/exit, and process
  lifecycle integration.
- filesystem-backed commands, file inspection from the shell, 'cd',
  path traversal, VFS lookup, directory listing, and writable filesystem state.
- broad argv/envp as process startup ABI, token vectors, quoting/escaping,
  globbing, environment expansion, pipes, redirection, descriptor inheritance
  across exec, and terminal job control.
- broad POSIX read/stdio readiness and POSIX signal delivery.
- persistent local configuration.
- networking and SSH.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.
- resumed Phase 8 proof-only work unless a later feature-led task directly
  needs it for local interactivity.

## Next Planning Recommendation

The next smallest feature-led Phase 10 task should add a bounded local
line-cancel behavior for Ctrl-C on the same descriptor-backed serial input
path. The user-visible feature should be: start typing a command at the
'talos>' prompt, press Ctrl-C, discard the partially collected line, print a
short kernel-backed cancellation response, return to a ready prompt, and then
accept and dispatch the next typed command normally.

That follow-up should use the existing TTY control-event vocabulary only as a
prompt-local line-cancel path. It should not claim POSIX signal delivery,
process interruption, terminal sessions, termios, job control, userspace shell
execution, process spawning, or filesystem command lookup. Old Phase 8
proof-only work remains paused unless it directly unblocks this local
interactivity feature.

## Validation

- static inspection: reconciled retained QEMU/substitute and Pi 5
  line-editing evidence paths from the accepted core and hardware proof.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

HardwareTestLock remained unlocked/restored and unused by this checkpoint.
