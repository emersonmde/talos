# Phase 7 Pi 5 Syscall Proof Plan

Task: phase7-pi5-syscall-proof-plan-20260529
Status: accepted

## Scope

This documentation-only task defined the serialized Raspberry Pi 5 production
syscall proof plan after the accepted QEMU syscall routing closeout. It did not
change Rust, assembly, boot scenarios, QEMU scripts, Pi 5 hardware state,
descriptor I/O, byte copy-in/copy-out, pointer-taking syscalls, process
loading, VFS, filesystem, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

## Plan Summary

- The next implementation should add a focused rpi5_syscall_proof boot
  scenario.
- The lower-EL payload must execute stable svc #0 with x8 = 0 and observe
  x0 = 0 after return.
- The payload must then execute stable svc #0 with x8 = 17 and observe
  x0 = 0xffffffffffffffda, the two's-complement encoding of -ENOSYS.
- Diagnostic SVC marker 0x7a10 may appear only as proof-owned completion
  vocabulary after the production return observations and must not dispatch as
  a syscall.
- The future hardware task must acquire hardwareTestLock, tie the candidate to
  source commit/archive/kernel/TFTP identity, use fresh serial and TFTP
  cursors, restore the prior accepted boot tree, and retain physical serial
  evidence.
- If a Pi 5 run is inconclusive, no code changes are allowed until candidate
  identity, fresh serial cursor, TFTP delta, known-good control, and unchanged
  candidate rerun evidence are recorded.

## Required Lines

The implementation task must retain a Pi 5 serial log with these final lines:

~~~text
rpi5-syscall-proof: final participants=2 expected=2 errors=0 classification=pi5-syscall-proof-complete
rpi5-syscall-proof: PASS
~~~

It must also include the required rpi5-syscall-proof syscall case,
user-observed, and diagnostic-marker quarantine lines named in
docs/src/project/phase7-pi5-syscall-proof-plan.md.

## Evidence

- static inspection: git status --short before edits was clean.
- static documentation diff summary: added
  docs/src/project/phase7-pi5-syscall-proof-plan.md; updated
  docs/src/SUMMARY.md, docs/src/roadmap.md, and docs/src/decisions/README.md;
  added this task record.
- plan summary: defined rpi5_syscall_proof invariant, stable svc #0 talos_nop
  and unknown-syscall behavior, exact PASS/classification lines, retained
  physical evidence requirements, hardwareTestLock ownership, candidate
  identity, fresh serial/TFTP requirements, restoration proof,
  inconclusive-run triage, diagnostic-marker quarantine, and blocked
  descriptor/copy/filesystem/shell/network surfaces.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Next Action

The next mechanically unblocked task is phase7-pi5-syscall-proof-20260529
after this plan is accepted and committed, provided hardwareTestLock is
unlocked. It should implement and run only the serialized physical proof
defined by this plan.
