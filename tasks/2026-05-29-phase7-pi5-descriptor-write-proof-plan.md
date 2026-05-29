# Phase 7 Pi 5 Descriptor-Write Proof Plan

Task: phase7-pi5-descriptor-write-proof-plan-20260529
Status: accepted

## Scope

This documentation-only task defined the serialized Raspberry Pi 5
descriptor-write syscall proof plan after the accepted QEMU descriptor-write
closeout. It did not change Rust, assembly, boot scenarios, QEMU scripts, Pi 5
hardware state, stdin/read, close, dup, process loading, VFS/filesystem
behavior, shell behavior, networking, SSH, RP1/PCIe, UART interrupt ownership,
or DMA/cache-driver policy.

## Plan Summary

- The next implementation should add a focused rpi5_descriptor_write_proof
  boot scenario.
- The lower-EL payload must execute stable svc #0 with talos_write x8 = 1 and
  prove fd 1 and fd 2 writes through copy_from_user(), inherited stdio
  descriptors, and runtime-console0.
- The proof must retain runtime-console0 observations for the exact stdout and
  stderr 18-byte buffers named by the plan.
- The payload must retain fd 0 and fd 99 -EBADF cases, guard-range -EFAULT,
  reserved-register -EINVAL, talos_nop x8 = 0, and unknown-syscall x8 = 17.
- Proof-only talos_copy_probe x8 = 0x7001 must remain quarantined as -ENOSYS
  in the descriptor-write proof scenario.
- Diagnostic SVC marker 0x7a10 may appear only as proof-owned completion
  vocabulary after all production return observations and must not dispatch as
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
rpi5-descriptor-write-proof: final participants=8 expected=8 errors=0 classification=pi5-descriptor-write-proof-complete
rpi5-descriptor-write-proof: PASS
~~~

It must also include the required rpi5-descriptor-write-proof validation,
write_stdout, write_stderr, runtime-console, errno, talos_nop,
unknown-syscall, copy-probe quarantine, user-observed, and diagnostic-marker
quarantine lines named in
docs/src/project/phase7-pi5-descriptor-write-proof-plan.md.

## Evidence

- static inspection: git status --short before edits showed a pre-existing
  docs/src/roadmap.md working-tree edit that was preserved.
- static documentation diff summary: added
  docs/src/project/phase7-pi5-descriptor-write-proof-plan.md; updated
  docs/src/SUMMARY.md, docs/src/roadmap.md, and docs/src/decisions/README.md;
  added this task record.
- referenced accepted descriptor-write closeout commit:
  d00b1939ed49266b107d5d130a64e6851a5f628a.
- referenced retained QEMU/substitute evidence:
  tasks/evidence/2026-05-29-qemu-descriptor-write-smoke-core/qemu-descriptor-write-smoke.log.
- plan summary: defined rpi5_descriptor_write_proof invariant, fd 1/fd 2
  runtime-console0 writes, fd/error cases, scalar and unknown-syscall
  regressions, exact PASS/classification lines, retained physical evidence
  requirements, hardwareTestLock ownership, candidate identity, fresh
  serial/TFTP requirements, restoration proof, inconclusive-run triage,
  diagnostic-marker quarantine, and blocked descriptor/filesystem/shell/network
  surfaces.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Next Action

The next mechanically unblocked task is
phase7-pi5-descriptor-write-proof-20260529 after this plan is accepted and
committed, provided hardwareTestLock is unlocked. It should implement and run
only the serialized physical proof defined by this plan.
