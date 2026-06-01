# Phase 10 Local Ls Root Closeout Checkpoint

Status: accepted

Task: phase10-local-ls-root-closeout-checkpoint-20260601

## Scope

This checkpoint closes out the bounded local `ls /` root-listing feature as
documentation-only work. It reconciles the accepted QEMU/substitute core,
serialized Raspberry Pi 5 proof, retained evidence, descriptor-backed stdin and
stdout frontier, command-loop regressions, deferred parser/userspace/filesystem
surfaces, and the handoff back to supervisor planning for the next feature-led
local interactivity task.

No Rust or assembly behavior changed. No QEMU scenario was rerun. No Pi 5
hardware action, boot archive publication, or hardwareTestLock acquisition was
performed.

## Reviewed Evidence

- local `ls /` core implementation, QEMU/substitute evidence, and task
  record commit: ca654f56c59a0255bf3e996c8224cde6c474ca5a.
- Pi 5 local `ls /` proof acceptance commit:
  1c42bfee16dd1997df18e4c9d900bf2e9287252f.
- retained QEMU/substitute `ls /` transcript:
  tasks/evidence/2026-06-01-qemu-local-ls-root-core/qemu-local-ls-root-smoke.log.
- retained QEMU/substitute help regression transcript:
  tasks/evidence/2026-05-31-qemu-local-help-command-core/qemu-local-help-command-smoke.log.
- retained QEMU/substitute literal echo regression transcript:
  tasks/evidence/2026-05-31-qemu-local-literal-echo-core/qemu-local-literal-echo-smoke.log.
- retained QEMU/substitute pwd regression transcript:
  tasks/evidence/2026-05-31-qemu-local-pwd-command-core/qemu-local-pwd-command-smoke.log.
- retained Backspace/Delete line-editing regression transcript:
  tasks/evidence/2026-05-31-qemu-local-line-editing-core/qemu-local-line-editing-smoke.log.
- retained Ctrl-C line-cancel regression transcript:
  tasks/evidence/2026-05-31-qemu-local-line-cancel-core/qemu-local-line-cancel-smoke.log.
- retained Ctrl-U line-kill regression transcript:
  tasks/evidence/2026-05-31-qemu-local-line-kill-core/qemu-local-line-kill-smoke.log.
- retained Pi 5 accepted serial transcript:
  tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local5-capture-window-proof/serial-full-window-after-write.txt.
- retained Pi 5 accepted proof summary:
  tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local5-capture-window-proof/proof-result.txt.
- retained Pi 5 archive/image review:
  tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local5-capture-window-proof/archive-review.txt.
- retained Pi 5 fresh TFTP proof:
  tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local5-capture-window-proof/tftp-delta-before-restore.json.
- retained Pi 5 restore proof:
  tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local5-capture-window-proof/post-restore-status.json.
- retained local4 capture-gap audit:
  tasks/evidence/2026-06-01-pi5-local-ls-root-proof/local4-response-capture-audit/audit-summary.txt.

The retained QEMU/substitute transcript contains input `ls /`, visible root
entries `bin`, `dir`, `empty`, and `etc`, descriptor-backed
fd0/runtime-console0 input and descriptor-backed stdout markers, next-prompt
readiness, final classification `qemu-local-ls-root-complete`, and exact PASS
line `qemu-local-ls-root: PASS`. Rerun local command-loop regressions keep
help, literal echo, pwd, Backspace/Delete, Ctrl-C, and Ctrl-U behavior covered.

The retained Pi 5 proof records unchanged candidate archive digest
16f5a053e05459239645b96eace01ee7f46139fa558b264af8a336a02d2a112c, candidate
kernel digest 904eeb9348ff1c0d1ade43c8e441b68f3bf9cef01055b2af2c3ad7d23a82eb24,
descriptor-backed input and output markers, input summary for `ls /`, visible
root entries `bin`, `dir`, `empty`, and `etc`, complete
`ready-for-next prompt=true`, final classification
`pi5-local-ls-root-complete`, exact PASS line
`rpi5-local-ls-root-proof: PASS`, fresh candidate TFTP service, and restore to
tree hash a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The physical proof was accepted only after the local5 capture-window rerun
retained the visible root entries and complete ready-for-next line. The earlier
local4 run reached final PASS but missed those retained response lines, so the
local4 response-capture audit is preserved as capture-gap evidence rather than
feature acceptance evidence.

## Accepted Frontier

The accepted capability is a bounded kernel-backed `ls /` command on the
descriptor-backed serial command-loop path. Talos can print the serial prompt,
accept `ls /` through fd0/runtime-console0 canonical-lite input, dispatch over
the accepted read-only initramfs root, print `bin`, `dir`, `empty`, and
`etc` through descriptor-backed stdout, and return to a ready `talos>`
prompt.

The accepted listing is deliberately root-only. It validates the fixed
read-only initramfs root fixture paths and does not accept recursive listing,
general path traversal, writable filesystem state, descriptor-backed filesystem
syscalls, or filesystem-backed external command execution.

Existing built-ins remain deterministic: `help` reports the accepted command
frontier, `status` reports the kernel-backed built-in frontier, `stdio`
reports fd identity and runtime-console0 backing, `pwd` prints `/`,
`echo hello` prints `hello`, bounded literal echo prints the accepted
literal tail, empty input reports `talos: empty-command`, unknown input
reports `talos: unknown-command`, and unexpected arguments to non-argument
built-ins report `talos: unexpected-argument`.

This frontier is still a kernel-backed local command loop. It is not accepted
userspace shell execution, process spawning, external command lookup, a broad
shell parser, or broad POSIX read/stdio behavior.

## Deferred Surfaces

Still deferred after this checkpoint:

- broad shell parser/tokenization, quoting, escaping, globbing, environment
  expansion, command substitution, multiline input, and shell variables.
- recursive listing, general path traversal, VFS lookup beyond the accepted
  read-only root fixture, descriptor-backed filesystem syscalls, writable
  filesystem state, and filesystem-backed command execution.
- argv/envp process ABI, userspace shell execution, process spawning,
  exec/wait/exit, process lifecycle integration, and descriptor inheritance
  across exec.
- broad POSIX read/stdio readiness, terminal sessions, termios, foreground
  process groups, job control, POSIX signal delivery, and signal restart
  behavior.
- cursor addressing, screen repainting, shell history, readline-style editing,
  arrow keys, broad escape-sequence parsing, pipes, and redirection.
- networking and SSH.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.
- resumed Phase 8 proof-only work unless a later feature-led task directly
  needs it for local interactivity.

## Next Planning Handoff

No explicit queued task remains after this checkpoint. The worker should set
planningNeeded=true and hand selection of the next smallest feature-led local
interactivity task back to the supervisor, instead of inventing a new direction
or promoting paused Phase 8 proof-only work.

The next task should stay in the local serial interactivity milestone unless
the supervisor records a deliberate phase or milestone transition. It should
continue to prefer the smallest user-visible behavior over diagnostic-only or
smoke-only work.

## Validation

- static inspection: reconciled retained QEMU/substitute and Pi 5 `ls /`
  evidence paths from the accepted core and hardware proof.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

HardwareTestLock remained unlocked/restored and unused by this checkpoint.

Acceptance commit: recorded in durable supervisor state after commit creation.
