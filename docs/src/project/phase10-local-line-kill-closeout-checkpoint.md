# Phase 10 Local Line-Kill Closeout Checkpoint

Status: accepted

Task: phase10-local-line-kill-closeout-checkpoint-20260531

## Scope

This checkpoint closes out the bounded Ctrl-U local line-kill feature as
documentation-only work. It reconciles the accepted QEMU/substitute line-kill
core, serialized Raspberry Pi 5 proof, retained evidence, descriptor-backed
stdin and stdout frontier, prompt-local whole-line discard semantics,
regressions for Backspace/Delete and Ctrl-C, deferred
terminal/shell/userspace/filesystem surfaces, and next feature-led planning
recommendation.

No Rust or assembly behavior changed. No QEMU scenario was rerun. No Pi 5
hardware action, boot archive publication, or hardwareTestLock acquisition was
performed.

## Reviewed Evidence

- local line-kill core implementation, QEMU/substitute evidence, and task
  record commit: d1ca91409ad541736958dd8408c4097b412d1c73.
- Pi 5 local line-kill proof harness implementation commits:
  e5f7a9f11510acbf19804220ea1c22e92efd2ed0,
  52476290c3927c9dde7838c807b8ff4c45a44819, and
  d3e4b59c39cf35eac204a2f36245a2d1d8d64ce3.
- Pi 5 local line-kill proof acceptance commit:
  472f80241196f6ce3a89a713af17b3cd7071c02d.
- retained QEMU/substitute transcript:
  tasks/evidence/2026-05-31-qemu-local-line-kill-core/qemu-local-line-kill-smoke.log.
- retained Ctrl-C line-cancel regression transcript:
  tasks/evidence/2026-05-31-qemu-local-line-cancel-core/qemu-local-line-cancel-smoke.log.
- retained Backspace/Delete line-editing regression transcript:
  tasks/evidence/2026-05-31-qemu-local-line-editing-core/qemu-local-line-editing-smoke.log.
- retained Pi 5 accepted serial transcript through PASS:
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/serial-transcript-through-pass.txt.
- retained Pi 5 accepted proof summary:
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/proof-result-local6.txt.
- retained Pi 5 accepted proof lines:
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/proof-lines-local6.txt.
- retained Pi 5 archive/image review:
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/archive-review.txt.
- retained Pi 5 fresh TFTP proof:
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/tftp-kernel-fetch-local6.txt.
- retained Pi 5 restore proof:
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local6-candidate-summary/post-snapshot-restore-status.json.
- retained inconclusive-run triage:
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local1-known-good-control/,
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local2-unchanged-candidate-rerun/,
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local3-known-good-control/,
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local3-candidate-flushed-metadata/,
  and
  tasks/evidence/2026-05-31-pi5-local-line-kill-proof/local5-unchanged-flushed-candidate-rerun/.

The retained QEMU/substitute transcript contains partial 'bogus' input,
Ctrl-U 0x15 line kill, visible 'talos: line-killed', following 'pwd'
dispatch, visible '/' output, descriptor-backed fd0/stdout markers, one
clear-line control telemetry event, next-prompt readiness, final
classification 'qemu-local-line-kill-complete', and exact PASS line
'qemu-local-line-kill: PASS'. The retained Ctrl-C and Backspace/Delete
regressions keep the previous accepted prompt-local cancellation and erase-byte
correction paths covered.

The retained Pi 5 proof records source commit
d3e4b59c39cf35eac204a2f36245a2d1d8d64ce3, accepted candidate archive digest
89049ac8adc6871ed728587e9445c91955f4d7a1d7f128abc4721724bde741a8,
candidate kernel digest
f4e5d43981e049b008b750c5dd5fc37b458628f5a0cf7d8f038ca91e1d964765, kernel
size 100272 bytes, visible 'talos: line-killed', following 'pwd' dispatch,
visible '/' output, descriptor-backed input and output markers, proof summary
for partial bogus, Ctrl-U, final line pwd, raw-bytes=10, controls=1,
responses=2, prompt readiness, final classification
'pi5-local-line-kill-complete', exact PASS line
'rpi5-local-line-kill-proof: PASS', fresh 'da591740/kernel_2712.img' TFTP
fetch evidence, and restore tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

Earlier Pi 5 candidate attempts reached parts of the internal proof path but
did not retain enough visible serial metadata to satisfy the hardware proof
acceptance gate. Before accepting proof-local visibility changes, triage
recorded candidate identity, fresh serial cursor, TFTP delta, restored-tree
known-good controls, and unchanged candidate reruns. The final local6 run
supplied the accepted physical transcript.

## Accepted Frontier

The accepted capability is bounded Ctrl-U prompt-local line kill over the
descriptor-backed serial command-loop path. Talos can print the serial prompt,
accept typed command bytes through fd0/runtime-console0 canonical-lite input,
treat Ctrl-U 0x15 as a prompt-local line-kill event, discard the entire
collected editable line before Enter dispatch, print a short
'talos: line-killed' response through descriptor-backed stdout, accept the
next typed command, dispatch the following kernel-backed built-in, write the
visible response through descriptor-backed stdout, and return to a ready
'talos>' prompt.

The accepted feature is intentionally prompt-local. It does not accept POSIX
signal delivery, process interruption, terminal sessions, foreground process
groups, termios policy, job control, shell history, yank/paste behavior,
readline editing, cursor addressing, screen repainting, broad escape-sequence
parsing, or arrow keys.

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
- cursor addressing, screen repainting, shell history, kill/yank beyond
  bounded Ctrl-U prompt-local line kill, broad escape-sequence parsing, arrow
  keys, and readline-style editing.
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

The next smallest feature-led Phase 10 task should add a bounded literal echo
tail path on the same descriptor-backed serial command loop. The user-visible
feature should be: type 'echo local serial works' at the 'talos>' prompt,
press Enter, dispatch the command through fd0/runtime-console0 input, print
'local serial works' through descriptor-backed stdout, and return to a ready
prompt.

That follow-up should stay kernel-backed and deliberately narrow. It should
not claim broad shell tokenization, quoting, escaping, globbing, environment
expansion, argv/envp process ABI, userspace shell execution, process spawning,
filesystem command lookup, POSIX read completeness, or terminal/session
semantics. Old Phase 8 proof-only work remains paused unless it directly
unblocks this local interactivity feature.

## Validation

- static inspection: reconciled retained QEMU/substitute and Pi 5 line-kill
  evidence paths from the accepted core and hardware proof.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

HardwareTestLock remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
