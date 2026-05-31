# Phase 10 Local Pwd Command Closeout Checkpoint

Status: accepted

Task: phase10-local-pwd-command-closeout-checkpoint-20260531

## Scope

This checkpoint closes out the bounded root-only `pwd` local command feature as
documentation-only work. It reconciles the accepted QEMU/substitute `pwd` core,
serialized Raspberry Pi 5 proof, retained evidence, descriptor-backed stdin and
stdout frontier, current-directory placeholder limits, deferred
shell/userspace/filesystem surfaces, and next feature-led planning
recommendation.

No Rust or assembly behavior changed. No QEMU scenario was rerun. No Pi 5
hardware action, boot archive publication, or hardwareTestLock acquisition was
performed.

## Reviewed Evidence

- local `pwd` command core implementation, QEMU/substitute evidence, and task
  record commit: 33fcf235bf3528c75085c568404f3e7a3bd1d1ea.
- Pi 5 local `pwd` command proof harness implementation commit:
  215bf0bca780a8c50f01977e778cdadd34d20238.
- Pi 5 local `pwd` command proof evidence and task-record commit:
  c2c0a5a862b3d5b78e25138b731393645f00f6ef.
- retained QEMU/substitute transcript:
  tasks/evidence/2026-05-31-qemu-local-pwd-command-core/qemu-local-pwd-command-smoke.log.
- retained Pi 5 accepted serial transcript:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/serial-transcript.txt.
- retained Pi 5 accepted normalized serial transcript:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/serial-transcript-normalized.txt.
- retained Pi 5 accepted proof summary:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/proof-result-local2.txt.
- retained Pi 5 archive/image review:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/archive-review.txt.
- retained Pi 5 fresh TFTP proof:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/tftp-kernel-fetch-local2.txt.
- retained Pi 5 restore proof:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local2-clean-candidate/post-snapshot-restore-status.json.
- retained inconclusive-run triage:
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local1-clean-candidate/
  and
  tasks/evidence/2026-05-31-pi5-local-pwd-command-proof/local1-known-good-control/.

The retained QEMU/substitute transcript contains `talos> pwd`, visible output
`/`, fd0/runtime-console0 descriptor-backed input, descriptor-backed stdout
output, next-prompt readiness, final classification
`qemu-local-pwd-command-complete`, and exact PASS line
`qemu-local-pwd-command: PASS`. It also keeps accepted `echo hello`,
`stdio`, help, status, empty input, and unknown-command behavior deterministic.

The retained Pi 5 proof summary contains source commit
215bf0bca780a8c50f01977e778cdadd34d20238, accepted candidate archive digest
6754773f7511b5c06b2fab5d1bb954212921ced5df1876f9c9c9d257dd2db5ae,
candidate kernel digest
1a31c94a569aa52ceb339b035ab35478f37adb5ef9ec2057b2cff8ab03327c4d, kernel
size 98816 bytes, `typed_command=pwd`, successful serial write, visible `/`
output, descriptor-backed input and output markers, prompt readiness, final
classification `pi5-local-pwd-command-complete`, exact PASS line
`rpi5-local-pwd-command-proof: PASS`, two fresh
`da591740/kernel_2712.img` TFTP fetches at 98816 bytes, and restore tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The first local1 Pi 5 candidate run is retained only as inconclusive evidence.
Before any code changes, triage recorded candidate identity, fresh serial
cursor, TFTP delta, a known-good control boot of the restored prior accepted
tree, and then reran the unchanged candidate as local2. The local2 rerun is the
accepted proof.

## Accepted Frontier

The accepted capability is a root-only kernel-backed `pwd` local command over
the descriptor-backed command-loop path. Talos can print the serial prompt,
accept typed `pwd` through fd0/runtime-console0 canonical-lite input, dispatch
the kernel-backed `pwd` built-in, write visible `/` output through
descriptor-backed stdout, and return to a ready prompt.

The current-directory model is intentionally only a placeholder. It has a
single root value, `/`, shaped for future process-local command context, but it
does not perform path traversal, normalization, VFS lookup, directory listing,
or `cd` mutation.

Existing built-ins remain deterministic: help lists `pwd`, `echo hello`
prints `hello`, `stdio` reports fd 0/fd 1/fd 2 identity plus
runtime-console0 backing and descriptor-backed markers, empty input reports
`talos: empty-command`, and unknown input reports `talos: unknown-command`.

This frontier is still a kernel-backed local command loop. It is not accepted
userspace shell execution, process spawning, external command lookup,
filesystem-backed command execution, broad POSIX read/stdio behavior, or a
general shell parser.

## Deferred Surfaces

Still blocked after this checkpoint:

- `cd`, current-directory mutation, path traversal, path normalization, VFS
  lookup, and directory listing.
- userspace shell execution.
- external command execution, process spawning, exec/wait/exit, and process
  lifecycle integration.
- filesystem-backed commands, file inspection from the shell, and writable
  filesystem state.
- broad argv/envp as process startup ABI, token vectors, quoting/escaping,
  globbing, environment expansion, pipes, redirection, descriptor inheritance
  across exec, and terminal job control.
- broad POSIX read/stdio readiness, termios, terminal sessions, and general
  TTY ownership.
- persistent local configuration.
- networking and SSH.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.
- resumed Phase 8 proof-only work unless a later feature-led task directly
  needs it for local interactivity.

## Next Planning Recommendation

The next smallest feature-led Phase 10 task should improve the same serial
interaction loop by accepting Backspace/Delete while editing a command line. The
user-visible feature should be: type a wrong character at the `talos>` prompt,
press Backspace or Delete, type the corrected character, press Enter, dispatch
the corrected command through the existing descriptor-backed path, print the
expected visible response, and return to a ready prompt.

That follow-up should stay below full terminal and shell semantics: no termios,
no cursor addressing, no history, no kill/yank editing, no escape-sequence
parser beyond the bounded key bytes needed for Backspace/Delete, no userspace
shell execution, no process spawning, and no filesystem command lookup. Old
Phase 8 proof-only work remains paused unless it directly unblocks this local
interactivity feature.

## Validation

- static inspection: reconciled retained QEMU/substitute and Pi 5 `pwd`
  evidence paths from the accepted `pwd` core and hardware proof.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

HardwareTestLock remained unlocked/restored and unused by this checkpoint.
