# Phase 10 Local Echo Command Closeout Checkpoint

Status: accepted

Task: phase10-local-echo-command-closeout-checkpoint-20260531

## Scope

This checkpoint closes out the bounded `echo hello` local command feature as
documentation-only work. It reconciles the accepted QEMU/substitute echo core,
serialized Raspberry Pi 5 proof, retained evidence, descriptor-backed stdin and
stdout frontier, parser limitations, deferred shell/userspace/filesystem
surfaces, and next feature-led planning recommendation.

No Rust or assembly behavior changed. No QEMU scenario was rerun. No Pi 5
hardware action, boot archive publication, or hardwareTestLock acquisition was
performed.

## Reviewed Evidence

- local echo command core implementation and QEMU/substitute evidence commit:
  33fcf235bf3528c75085c568404f3e7a3bd1d1ea.
- local echo command core task-record commit:
  924d971666b8c879feb2065522b3b15dc2765104.
- Pi 5 local echo command proof implementation/evidence commit:
  83588278519d155bc5e714ff7df086ebc5cb04af.
- Pi 5 local echo command proof task-record commit:
  d3f31503be34d5e8ddc54c61158a7efe7b12e4cc.
- retained QEMU/substitute transcript:
  tasks/evidence/2026-05-31-qemu-local-echo-command-core/qemu-local-echo-command-smoke.log.
- retained Pi 5 accepted serial transcript:
  tasks/evidence/2026-05-31-pi5-local-echo-command-proof/local2-clean-candidate/serial-transcript.txt.
- retained Pi 5 accepted proof summary:
  tasks/evidence/2026-05-31-pi5-local-echo-command-proof/local2-clean-candidate/proof-result-local2.txt.
- retained Pi 5 archive/image review:
  tasks/evidence/2026-05-31-pi5-local-echo-command-proof/local2-clean-candidate/archive-review.txt.
- retained Pi 5 fresh TFTP proof:
  tasks/evidence/2026-05-31-pi5-local-echo-command-proof/local2-clean-candidate/tftp-kernel-fetch-local2.txt.
- retained Pi 5 restore proof:
  tasks/evidence/2026-05-31-pi5-local-echo-command-proof/local2-clean-candidate/post-snapshot-restore-status.json.

The retained QEMU/substitute transcript contains `talos> echo hello`, visible
output `hello`, descriptor-backed-input and descriptor-backed-output markers,
next-prompt readiness, final classification
`qemu-local-echo-command-complete`, and exact PASS line
`qemu-local-echo-command: PASS`. The same transcript keeps help, status,
`stdio`, empty input, and unknown-command behavior visible and deterministic.

The retained Pi 5 proof summary contains the accepted candidate archive digest
1ec5389c84e3a779ef1d98c5b664b3771947c8415fd02ee731f2cbfbafa646d4,
candidate kernel digest
cc80d0bb12d2f98a889ad5ec8de21119d2ba16031b4015c3b81bfcef958d5d4e, kernel
size 98664 bytes, fresh serial cursor 3549504, two fresh
`da591740/kernel_2712.img` TFTP fetches at 98664 bytes,
`typed_command=echo hello`, visible output `hello`, descriptor-backed input
and output markers, `rpi5-local-echo-command-proof: ready-for-next
prompt=true`, final classification `pi5-local-echo-command-complete`, exact
PASS line `rpi5-local-echo-command-proof: PASS`, and restore tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The first local1 Pi 5 attempt is retained only as inconclusive capture
evidence. It reached the PASS path but did not retain enough visible serial
bytes around the command response. The accepted local2 run records the fresh
candidate identity, serial cursor, TFTP evidence, visible response, and restore
proof.

## Accepted Frontier

The accepted capability is an argument-bearing kernel-backed local command over
the descriptor-backed command-loop path. Talos can print the serial prompt,
accept typed `echo hello` through fd0/runtime-console0 canonical-lite input,
parse a command word plus simple trailing argument text, dispatch the
kernel-backed `echo` built-in, write visible `hello` output through
descriptor-backed stdout, and return to a ready prompt.

Existing built-ins remain deterministic: help and status reject unexpected
arguments, `stdio` reports fd 0/fd 1/fd 2 identity plus runtime-console0
backing and descriptor-backed markers, empty input reports `talos:
empty-command`, and unknown input reports `talos: unknown-command`.

This frontier is still a kernel-backed local command loop. It is not accepted
userspace shell execution, process spawning, external command lookup,
filesystem-backed command execution, broad POSIX read/stdio behavior, or a
general shell parser.

## Parser And Argument Limitations

The accepted parser recognizes one command word made from lowercase letters,
digits, hyphen, or underscore, then treats the trimmed remaining text as a
single argument string for `echo`. It does not accept quoting, escaping,
globbing, environment expansion, pipes, redirection, command substitution,
argv/envp process startup ABI, or general token vectors for non-`echo`
commands. The canonical-lite line capacity remains intentionally small and the
proof accepts only the bounded `echo hello` behavior.

## Deferred Surfaces

Still blocked after this checkpoint:

- userspace shell execution.
- external command execution, process spawning, exec/wait/exit, and process
  lifecycle integration.
- filesystem-backed commands, file inspection from the shell, and current
  working directory semantics.
- broad argv/envp as process startup ABI, token vectors, quoting/escaping,
  globbing, environment expansion, pipes, redirection, descriptor inheritance
  across exec, and terminal job control.
- broad POSIX read/stdio readiness, termios, terminal sessions, and general
  TTY ownership.
- writable filesystem state and persistent local configuration.
- networking and SSH.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.
- resumed Phase 8 proof-only work unless a later feature-led task directly
  needs it for local interactivity.

## Next Planning Recommendation

The next smallest feature-led Phase 10 task should add a kernel-backed `pwd`
command that prints `/` through the accepted descriptor-backed stdout path and
records the current directory as a narrow process-local placeholder below the
future filesystem/process model. The user-visible feature should be: type
`pwd` at the serial prompt, read the completed line through fd0/runtime-console0
descriptor plumbing, dispatch the kernel-backed built-in, print `/`, and return
to a ready prompt.

That follow-up should keep current-directory state deliberately minimal:
single process owner, root-only value, no path traversal, no VFS lookup, no
directory listing, no chdir, no userspace shell execution, and no filesystem
program loading. Old Phase 8 proof-only work remains paused unless it directly
unblocks this local interactivity feature.

## Validation

- static inspection: reconciled retained QEMU/substitute and Pi 5 echo evidence
  paths from the accepted echo core and hardware proof.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

HardwareTestLock remained unlocked/restored and unused by this checkpoint.
