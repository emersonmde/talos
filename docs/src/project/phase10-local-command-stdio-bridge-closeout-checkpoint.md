# Phase 10 Local Command Stdio Bridge Closeout Checkpoint

Status: accepted

Task: phase10-local-command-stdio-bridge-closeout-checkpoint-20260531

## Scope

This checkpoint closes out the descriptor-backed stdio bridge slice for the
local command loop as documentation-only work. It reconciles the accepted
QEMU/substitute core, serialized Raspberry Pi 5 proof, retained evidence,
accepted descriptor-backed output frontier, deferred shell surfaces, and next
feature-led planning recommendation.

No Rust or assembly behavior changed. No QEMU scenario was rerun. No Pi 5
hardware action, boot archive publication, or hardwareTestLock acquisition was
performed.

## Reviewed Evidence

- local command stdio bridge core:
  af8025fd7c8f7e405575a4c6cf24b3377cedf337.
- Pi 5 local command stdio bridge proof implementation/evidence:
  115a347.
- Pi 5 local command stdio bridge proof task-record commit:
  c2d096b86d44.
- retained QEMU/substitute transcript:
  tasks/evidence/2026-05-31-qemu-local-command-stdio-bridge-core/qemu-local-command-stdio-bridge-smoke.log.
- retained Pi 5 accepted serial transcript:
  tasks/evidence/2026-05-31-pi5-local-command-stdio-bridge-proof/local11-fresh-tftp-visible-response-candidate/serial-transcript.txt.
- retained Pi 5 accepted proof summary:
  tasks/evidence/2026-05-31-pi5-local-command-stdio-bridge-proof/local11-fresh-tftp-visible-response-candidate/proof-result-stdio.txt.
- retained same-candidate fresh TFTP proof:
  tasks/evidence/2026-05-31-pi5-local-command-stdio-bridge-proof/local10-fresh-tftp-visible-response-candidate/tftp-kernel-fetch-local10.txt.
- retained Pi 5 candidate archive/image review:
  tasks/evidence/2026-05-31-pi5-local-command-stdio-bridge-proof/local11-fresh-tftp-visible-response-candidate/archive-review.txt.
- retained restore proof:
  tasks/evidence/2026-05-31-pi5-local-command-stdio-bridge-proof/local11-fresh-tftp-visible-response-candidate/post-snapshot-restore-status.json.

The retained QEMU transcript contains the prompt, typed `stdio`, visible
`talos: ok stdio`, fd 0/fd 1/fd 2 inherited stdio identities,
`talos: runtime-console runtime-console0`,
`talos: descriptor-backed-output=true`, next-prompt readiness, final
classification `qemu-local-serial-command-loop-complete`, and exact PASS line
`qemu-local-serial-command-loop: PASS`. The same transcript keeps help,
status, empty input, and unknown-command behavior visible and deterministic.

The retained Pi 5 proof summary contains the accepted candidate archive digest
0885f021f34ab1398f91fa8206d587a40295663570ce8c505daa3b21ac8c2f02,
candidate kernel digest
45934e74174388e3346cf76f63af3568abf260526fd749a707b2a67568191899, kernel
size 97472 bytes, `talos: ok stdio`, fd 0/fd 1/fd 2 stdio identity lines,
`talos: runtime-console runtime-console0`,
`talos: descriptor-backed-output=true`,
`rpi5-local-command-stdio-bridge-proof: ready-for-next prompt=true`, final
classification `pi5-local-command-stdio-bridge-complete`, exact PASS line
`rpi5-local-command-stdio-bridge-proof: PASS`, and restore tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The same-candidate local10 TFTP evidence records a 97472-byte candidate kernel
fetch for the accepted local11 image identity. It is retained as fresh TFTP
evidence, not as the accepted serial transcript.

## Accepted Frontier

The accepted capability is a bounded descriptor-backed output bridge for the
kernel-backed local command loop. Talos can print the serial prompt, accept the
typed `stdio` command over the existing canonical runtime-console0 input path,
dispatch the command, write the visible response through an inherited stdout
descriptor backed by runtime-console0, report fd 0/fd 1/fd 2 stdio identities,
and reach a next-prompt ready state.

The accepted descriptor-backed output path uses a ProcessOwnerId-backed
ProcessDescriptorStore with inherited stdio descriptors in the QEMU/substitute
path. fd 1 and fd 2 resolve to stdio-output descriptors; the local command
client writes visible prompt/response bytes through the stdout bridge in the
accepted smoke path. runtime-console0 remains the backing console below that
descriptor abstraction.

The Pi 5 proof accepts the same user-visible `stdio` command behavior on
physical serial output, including fd identity lines, runtime-console0 backing,
descriptor-backed marker, ready prompt, classification, PASS, candidate
identity, fresh TFTP evidence, and restore proof. It does not accept any broader
stdio, shell, filesystem, process, or networking behavior.

## Deferred Surfaces

Still blocked after this checkpoint:

- fd0/runtime-console-backed stdin as a general descriptor read source.
- userspace shell execution.
- external command execution, process spawning, wait/exit, and process
  lifecycle integration.
- filesystem-backed commands and file inspection from the shell.
- argv/envp, pipes, descriptor inheritance across exec, process control, and
  terminal job control.
- POSIX-complete stdio, termios, terminal sessions, and general TTY ownership.
- writable filesystem state and persistent local configuration.
- networking and SSH.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.
- resumed Phase 8 proof-only work unless a later feature-led task directly
  needs it for local interactivity.

## Next Planning Recommendation

The next smallest feature-led Phase 10 task should move the accepted command
loop input side through the descriptor model: make fd 0 backed by the local
runtime-console/canonical input source for this command-loop path, prove that a
typed line is read through the stdin descriptor, dispatch the same kernel-backed
command, and print the visible descriptor-backed stdout response. The user
visible behavior should remain the same `talos> stdio` interaction, with the
new evidence showing that both input and output cross the descriptor boundary.

That follow-up should stay below userspace shell execution and process spawning
unless the supervisor explicitly decomposes those larger surfaces. Old Phase 8
proof-only work remains paused unless it directly unblocks this local
interactivity feature.
