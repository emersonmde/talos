# Phase 10 Local Serial Command Loop Closeout Checkpoint

Status: accepted

Task: phase10-local-serial-command-loop-closeout-checkpoint-20260531

## Scope

This checkpoint closes out the first feature-led Phase 10 local serial
interactivity slice as documentation-only work. It reconciles the accepted
QEMU/substitute command-loop core, serialized Raspberry Pi 5 proof, retained
evidence, temporary kernel-backed built-ins, deferred shell surfaces, and next
planning state.

No Rust or assembly behavior changed. No QEMU scenario was rerun. No Pi 5
hardware action, boot archive publication, or hardwareTestLock acquisition was
performed.

## Reviewed Evidence

- local serial command-loop core:
  32be75b9471772aa7b2f53e7c2e8c5932015b17c.
- serialized Pi 5 local serial command-loop proof:
  edb9b21.
- retained QEMU/substitute transcript:
  tasks/evidence/2026-05-31-qemu-local-serial-command-loop-core/qemu-local-serial-command-loop-smoke.log.
- retained Pi 5 hardware transcript:
  tasks/evidence/2026-05-31-pi5-local-serial-command-loop-proof/local6-clean-candidate/serial-transcript.txt.
- retained Pi 5 selected-command proof summary:
  tasks/evidence/2026-05-31-pi5-local-serial-command-loop-proof/local6-clean-candidate/proof-result-selected-bogus.txt.
- Pi 5 candidate archive digest:
  b27764bce3d8a47562e16679119f24e3b841ed7f7aa6249070cb360cc6e3a134.
- Pi 5 candidate kernel digest:
  09a02e4dd9cbeac61ef20f4cd4cef6a1e62d1364abdb87aff9724a6101e1fb34.

The retained QEMU transcript contains the prompt, the typed "help" line with
"talos: ok help" and "talos: commands help status", an empty Enter dispatch
with "talos: empty-command", the typed "bogus" line with
"talos: unknown-command", "ready-for-next prompt=true", final classification
"qemu-local-serial-command-loop-complete", and exact PASS line
"qemu-local-serial-command-loop: PASS".

The retained Pi 5 selected-command proof contains "selected-command=bogus",
"PASS talos> bogus", "PASS talos: unknown-command", dispatch status
"unknown-command", "ready-for-next prompt=true", classification
"pi5-local-serial-command-loop-complete", exact PASS line
"rpi5-local-serial-command-loop-proof: PASS", and restore tree hash
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

## Accepted Frontier

The accepted capability is a bounded local command-loop boundary above
runtime-console0 and the accepted canonical-lite TTY receive path. Talos can
print a "talos> " prompt, read a completed serial line, dispatch that line to
kernel-backed command handling, print a visible response, and reach a next
prompt/ready state.

QEMU/substitute evidence accepts the help, empty input, and unknown-command
paths. Serialized Pi 5 hardware evidence accepts the physical unknown-command
path over the BCM2712 UART10 runtime-console0 route.

The accepted built-ins are intentionally temporary and kernel-backed. The help
command lists "help status", and status reports the command-loop version,
runtime-console0 identity, and "builtins kernel-backed". These built-ins are
useful for local interactivity but are not a replacement for userspace shell
execution.

## Deferred Surfaces

Still blocked after this checkpoint:

- userspace shell execution.
- external command execution and process spawning.
- descriptor-backed stdin/stdout/stderr for the local command loop.
- filesystem-backed commands and file inspection from the shell.
- argv/envp, wait/exit, pipes, process control, and terminal job control.
- writable filesystem state and persistent local configuration.
- networking and SSH.
- RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.
- resumed Phase 8 proof-only work unless a later feature-led task needs it.

## Next Planning State

No explicit queued follow-up task remains after this checkpoint. The worker
sets planningNeeded=true because choosing the next Phase 10 feature step
requires supervisor decomposition, not another worker-invented task.

The concrete planning blocker is the missing supervisor-owned task definition
with scope, non-goals, dependencies, acceptance criteria, gates, docs, and
evidence. The likely next feature-led planning target is to move the accepted
command-loop interaction toward descriptor-backed stdin/stdout while preserving
the same user-visible serial behavior. Supervisor planning may instead choose a
smaller command-coverage step or a process-launch prerequisite, but it should
remain feature-led and keep old Phase 8 smoke/closeout work paused unless it
directly unblocks local interactivity.
