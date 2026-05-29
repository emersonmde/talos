# Phase 7 Pi 5 Pointer-Copy Proof Plan

Task: phase7-pi5-pointer-copy-proof-plan-20260529
Status: accepted

## Scope

This documentation-only task defined the serialized Raspberry Pi 5
pointer-copy syscall proof plan after the accepted QEMU pointer-copy closeout.
It did not change Rust, assembly, boot scenarios, QEMU scripts, Pi 5 hardware
state, descriptor I/O, runtime console or TTY integration, process loading,
VFS/filesystem behavior, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, or DMA/cache-driver policy.

## Plan Summary

- The next implementation should add a focused rpi5_pointer_copy_proof boot
  scenario.
- The lower-EL payload must execute stable svc #0 with x8 = 0x7001, pass the
  accepted talos_copy_probe pointer/length/byte arguments, and observe
  x0 = 16 after the success copy.
- The proof must show the kernel-observed UserData backing storage changed
  from 16 bytes of 0x2a to 16 bytes of 0xa5.
- The payload must then execute the guard-range talos_copy_probe case and
  observe x0 = 0xfffffffffffffff2, the two's-complement encoding of -EFAULT.
- The payload must retain the unknown-syscall regression with x8 = 17 and
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
rpi5-pointer-copy-proof: final participants=3 expected=3 errors=0 classification=pi5-pointer-copy-proof-complete
rpi5-pointer-copy-proof: PASS
~~~

It must also include the required rpi5-pointer-copy-proof validation,
copy_probe_success, copy_probe_efault, unknown-syscall, user-observed, and
diagnostic-marker quarantine lines named in
docs/src/project/phase7-pi5-pointer-copy-proof-plan.md.

## Evidence

- static inspection: git status --short before edits was clean.
- static documentation diff summary: added
  docs/src/project/phase7-pi5-pointer-copy-proof-plan.md; updated
  docs/src/SUMMARY.md, docs/src/roadmap.md, and docs/src/decisions/README.md;
  added this task record.
- referenced accepted closeout commit:
  a30883bc5b4458850fe369b4558c27dc97736258.
- referenced retained QEMU/substitute evidence:
  tasks/evidence/2026-05-29-qemu-pointer-copy-smoke-core/qemu-pointer-copy-smoke.log.
- plan summary: defined rpi5_pointer_copy_proof invariant, success copy,
  guard-range EFAULT, unknown-syscall behavior, exact PASS/classification
  lines, retained physical evidence requirements, hardwareTestLock ownership,
  candidate identity, fresh serial/TFTP requirements, restoration proof,
  inconclusive-run triage, diagnostic-marker quarantine, and blocked
  descriptor/filesystem/shell/network surfaces.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Next Action

The next mechanically unblocked task is phase7-pi5-pointer-copy-proof-20260529
after this plan is accepted and committed, provided hardwareTestLock is
unlocked. It should implement and run only the serialized physical proof
defined by this plan.
