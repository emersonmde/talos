# Phase 10 Local Command Stdin Descriptor Closeout Checkpoint

Task: phase10-local-command-stdin-descriptor-closeout-checkpoint-20260531

## Scope

This checkpoint closes out the descriptor-backed stdin/stdout local command
loop slice as documentation-only work. It reconciles the accepted
QEMU/substitute stdin-descriptor core, serialized Raspberry Pi 5 proof,
retained evidence, accepted descriptor-backed input/output frontier, deferred
shell surfaces, and next feature-led planning recommendation.

No Rust or assembly behavior changed. No QEMU scenario was rerun. No Pi 5
hardware action, boot archive publication, or hardwareTestLock acquisition was
performed.

## Reviewed Evidence

- local command stdin descriptor core:
  4259bd2ed78d77ddc16a0bfdea8ae42501522703.
- Pi 5 local command stdin descriptor proof implementation/evidence:
  1b2ab4e.
- Pi 5 local command stdin descriptor proof task-record commit:
  3ccd592a0c86.
- retained QEMU/substitute transcript:
  tasks/evidence/2026-05-31-qemu-local-command-stdin-descriptor-core/qemu-local-command-stdin-descriptor-smoke.log.
- retained Pi 5 accepted serial transcript:
  tasks/evidence/2026-05-31-pi5-local-command-stdin-descriptor-proof/local8-fresh-tftp-candidate-rerun/serial-transcript.txt.
- retained Pi 5 accepted proof summary:
  tasks/evidence/2026-05-31-pi5-local-command-stdin-descriptor-proof/local8-fresh-tftp-candidate-rerun/proof-result-stdin-descriptor.txt.
- retained Pi 5 candidate archive/image review:
  tasks/evidence/2026-05-31-pi5-local-command-stdin-descriptor-proof/local8-fresh-tftp-candidate-rerun/archive-review.txt.
- retained pre-restore TFTP proof:
  tasks/evidence/2026-05-31-pi5-local-command-stdin-descriptor-proof/local8-fresh-tftp-candidate-rerun/tftp-kernel-fetch-local8.txt.
- retained restore proof:
  tasks/evidence/2026-05-31-pi5-local-command-stdin-descriptor-proof/local8-fresh-tftp-candidate-rerun/post-snapshot-restore-status.json.

The retained QEMU transcript contains the prompt, typed `stdio`, fd0
descriptor-backed input marker, visible `talos: ok stdio`, fd 0/fd 1/fd 2
stdio identity lines, `talos: runtime-console runtime-console0`,
`talos: descriptor-backed-output=true`, next-prompt readiness, final
classification `qemu-local-command-stdin-descriptor-complete`, and exact PASS
line `qemu-local-command-stdin-descriptor: PASS`. The same transcript keeps
help, status, empty input, and unknown-command behavior visible and
deterministic.

The retained Pi 5 proof summary contains the accepted candidate archive digest
acabea1f0a779abca51c1d0b880b43929c2f09e39eed6304bdc7aaf7685cd65f,
candidate kernel digest
9466fb78b30029be15107ab8141fa9d0f033072e92c9fee2299d6c79ccda5d92, kernel
size 97936 bytes, fresh serial cursor 3537395, two fresh
`da591740/kernel_2712.img` TFTP fetches at 97936 bytes, `talos: ok stdio`,
fd 0/fd 1/fd 2 stdio identity lines,
`talos: runtime-console runtime-console0`,
`talos: descriptor-backed-input=true`,
`talos: descriptor-backed-output=true`,
`rpi5-local-command-stdio-bridge-proof: ready-for-next prompt=true`, final
classification `pi5-local-command-stdio-bridge-complete`, exact PASS line
`rpi5-local-command-stdio-bridge-proof: PASS`, and restore tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Accepted Frontier

The accepted capability is a bounded descriptor-backed stdin/stdout bridge for
the kernel-backed local command loop. Talos can print the serial prompt, accept
the typed `stdio` command over runtime-console0, read the completed line
through fd0/runtime-console0 descriptor plumbing, dispatch the kernel-backed
command, write the visible response through descriptor-backed stdout, report fd
0/fd 1/fd 2 stdio identities, record runtime-console0 backing, and reach a
next-prompt ready state.

The accepted descriptor-backed input path uses a ProcessOwnerId-backed
ProcessDescriptorStore with inherited stdio descriptors in the QEMU/substitute
path. fd 0 resolves to a runtime-console-backed stdin source for the local
command loop. fd 1 and fd 2 resolve to stdio-output descriptors; the local
command client writes prompt/response bytes through the stdout bridge in the
accepted smoke path. runtime-console0 remains the backing console below those
descriptor abstractions.

The Pi 5 proof accepts the same user-visible `stdio` command behavior on
physical serial hardware, including descriptor-backed-input and
descriptor-backed-output markers, fd identity lines, runtime-console0 backing,
ready prompt, classification, PASS, candidate identity, fresh TFTP evidence,
and restore proof. It does not accept any broader userspace shell, filesystem,
process, POSIX read, or networking behavior.

## Deferred Surfaces

Still blocked after this checkpoint:

- userspace shell execution.
- external command execution, process spawning, wait/exit, and process
  lifecycle integration.
- filesystem-backed commands and file inspection from the shell.
- argv/envp as process startup ABI, pipes, descriptor inheritance across exec,
  process control, and terminal job control.
- broad POSIX read/stdio readiness, termios, terminal sessions, and general
  TTY ownership.
- writable filesystem state and persistent local configuration.
- networking and SSH.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.
- resumed Phase 8 proof-only work unless a later feature-led task directly
  needs it for local interactivity.

## Next Planning Recommendation

The next smallest feature-led Phase 10 task should add an argument-bearing
kernel-backed `echo` command over the accepted descriptor-backed stdin/stdout
path. The user-visible feature should be: type `echo hello` at the serial
prompt, read the completed line through fd0/runtime-console0 descriptor
plumbing, parse the command plus argument text, dispatch the kernel-backed
built-in, print `hello` through descriptor-backed stdout, and return to a
ready prompt.

That follow-up should stay below userspace shell execution, process spawning,
filesystem-backed command lookup, and POSIX-complete argv/envp unless the
supervisor explicitly decomposes those larger surfaces. Old Phase 8 proof-only
work remains paused unless it directly unblocks this local interactivity
feature.

## Validation

- static inspection: reconciled retained QEMU/substitute and Pi 5 evidence
  paths from the accepted stdin-descriptor core and hardware proof.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

HardwareTestLock remained unlocked/restored and unused by this checkpoint.
