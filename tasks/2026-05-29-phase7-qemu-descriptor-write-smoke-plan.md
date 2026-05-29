# Phase 7 QEMU Descriptor-Write Smoke Plan

Task: phase7-qemu-descriptor-write-smoke-plan-20260529
Status: accepted
Date: 2026-05-29

## Scope

This documentation-only task accepted the first QEMU/substitute descriptor-write
smoke plan after the descriptor syscall contract. It defines the bounded
qemu_descriptor_write_smoke evidence that must later prove talos_write through
lower-AArch64 svc #0, fd 1/fd 2 inherited stdio descriptors, copy_from_user(),
and runtime-console0.

It did not change Rust or assembly behavior, run QEMU, run Pi 5 hardware,
publish boot archives, acquire hardwareTestLock, implement descriptor I/O, add
stdin/read, close, dup, process loading, VFS/filesystem behavior, shell
behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

## Plan Summary

- Scenario: qemu_descriptor_write_smoke.
- Stable syscall under proof: talos_write with svc #0, x8 = 1, fd/user-pointer/
  length in x0/x1/x2, and x3 through x5 reserved zero.
- Success cases: fd 1 and fd 2 write 18 bytes from UserData through
  DescriptorTable::with_inherited_stdio(), copy_from_user(), and
  runtime-console0, returning x0 = 18.
- Error cases: fd 0 and fd 99 return -EBADF, guard-range user pointer returns
  -EFAULT, and nonzero reserved register x3 returns -EINVAL.
- Regression cases: talos_nop returns 0, unknown syscall 17 returns -ENOSYS,
  x8 = 0x7001 remains -ENOSYS in this smoke, and diagnostic marker 0x7a10
  remains proof-only.
- Evidence: retained QEMU/substitute serial log with
  classification=qemu-descriptor-write-smoke-complete and
  qemu-descriptor-write-smoke: PASS.

## Files Reviewed

- docs/src/project/phase7-descriptor-syscall-contract.md
- docs/src/project/phase7-qemu-syscall-smoke-plan.md
- docs/src/project/phase7-qemu-pointer-copy-smoke-plan.md
- src/syscall.rs
- src/posix.rs
- src/runtime_console.rs

## Changed Files

- docs/src/project/phase7-qemu-descriptor-write-smoke-plan.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-29-phase7-qemu-descriptor-write-smoke-plan.md

## Recommendation

The next bounded implementation task should be
phase7-descriptor-write-core-20260529. It should implement only the
target-independent talos_write descriptor-write core for fd 1/fd 2
runtime-console0, with focused unit tests for success, zero length, invalid
descriptors, invalid user ranges, reserved registers, length limits, errno
encoding, and regression of talos_nop/unknown syscall/proof-only copy-probe
behavior.

## Validation

- static inspection: reviewed the accepted descriptor syscall contract,
  syscall smoke plan, pointer-copy smoke plan, and relevant syscall, POSIX, and
  runtime-console source files.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
