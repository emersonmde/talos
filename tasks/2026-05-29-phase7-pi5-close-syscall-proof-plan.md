# Phase 7 Pi 5 Close Syscall Proof Plan

Task: phase7-pi5-close-syscall-proof-plan-20260529
Status: accepted

## Scope

This documentation-only task defined the serialized Raspberry Pi 5 close
syscall proof plan after the accepted close syscall closeout. It did not
change Rust, assembly, boot scenarios, QEMU scripts, Pi 5 hardware state,
dup/read syscall behavior, process loading, VFS, filesystem, shell behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, object finalization, or
DMA/cache-driver policy.

## Plan Summary

- The next implementation should add a focused rpi5_close_syscall_proof boot
  scenario.
- The lower-EL payload must execute stable svc #0 with x8 = 2 to close fd 1
  and fd 2 through the current ProcessOwnerId-backed descriptor table.
- The payload must prove writes after close return -EBADF without adding
  runtime-console0 bytes.
- A reserved-argument close on fd 2 must return -EINVAL and leave fd 2 open,
  proven by a later successful write through fd 2.
- Repeated close and invalid close must return -EBADF with no table mutation.
- talos_nop, unknown-syscall -ENOSYS, proof-only talos_copy_probe quarantine,
  and diagnostic marker 0x7a10 quarantine must remain intact.
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
rpi5-close-syscall-proof: final participants=11 expected=11 errors=0 classification=pi5-close-syscall-proof-complete
rpi5-close-syscall-proof: PASS
~~~

It must also include the required rpi5-close-syscall-proof close, write,
runtime-console, errno, regression, copy-probe quarantine, and
diagnostic-marker quarantine lines named in
docs/src/project/phase7-pi5-close-syscall-proof-plan.md.

## Evidence

- static inspection: git status --short before edits was clean.
- static documentation diff summary: added
  docs/src/project/phase7-pi5-close-syscall-proof-plan.md; updated
  docs/src/SUMMARY.md, docs/src/roadmap.md, and docs/src/decisions/README.md;
  added this task record.
- plan summary: defined rpi5_close_syscall_proof invariant, stable svc #0
  talos_close behavior, close(fd 1), close(fd 2), write-after-close -EBADF,
  reserved-argument -EINVAL no-mutation, repeated/invalid close -EBADF,
  talos_nop and unknown-syscall regressions, exact PASS/classification lines,
  retained physical evidence requirements, hardwareTestLock ownership,
  candidate identity, fresh serial/TFTP requirements, restoration proof,
  inconclusive-run triage, diagnostic/proof-only quarantine, and blocked
  dup/read/filesystem/shell/network/object-finalization surfaces.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Next Action

The next mechanically unblocked task is
phase7-pi5-close-syscall-proof-20260529 after this plan is accepted and
committed, provided hardwareTestLock is unlocked. It should implement and run
only the serialized physical proof defined by this plan.
