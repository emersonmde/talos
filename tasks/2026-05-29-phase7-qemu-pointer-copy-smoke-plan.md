# Phase 7 QEMU Pointer-Copy Smoke Plan

Task: phase7-qemu-pointer-copy-smoke-plan-20260529
Status: accepted

## Scope

This documentation-only task planned the first QEMU/substitute smoke for the
proof-only talos_copy_probe pointer-taking syscall. It did not change Rust,
assembly, boot scenarios, QEMU scripts, Pi 5 hardware state, descriptor I/O,
process loading, VFS, filesystem, shell behavior, networking, SSH, RP1/PCIe,
UART interrupt ownership, or DMA/cache-driver policy.

## Plan Summary

- The later boot scenario is qemu_pointer_copy_smoke with retained evidence
  under tasks/evidence/2026-05-29-qemu-pointer-copy-smoke-core/.
- The success case uses stable svc #0, x8 = 0x7001, UserData at
  0x0000_0000_0011_0000, length 16, expected byte 0x2a, replacement byte
  0xa5, and reserved x4/x5 zeros. It must return x0 = 16 and print the
  replacement data pattern.
- The EFAULT case uses the same syscall and byte arguments against the guard
  range at 0x0000_0000_001e_0000 and must return x0 = 0xfffffffffffffff2.
- The smoke must also retain the scalar unknown-syscall -ENOSYS observation
  and keep diagnostic marker 0x7a10 proof-only.
- The plan names phase7-qemu-pointer-copy-smoke-core-20260529 as the next
  bounded implementation task.

## Evidence

- static inspection: git status --short before edits was clean.
- static plan review: inspected the accepted pointer-taking syscall contract,
  source inventory, existing QEMU syscall smoke plan, roadmap, decision log,
  and task record style.
- static documentation diff summary: added
  docs/src/project/phase7-qemu-pointer-copy-smoke-plan.md; updated
  docs/src/roadmap.md, docs/src/decisions/README.md, docs/src/SUMMARY.md; added
  this task record.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Next Action

phase7-qemu-pointer-copy-smoke-core-20260529 is now mechanically unblocked for
the next worker wake if durable state and the working tree remain compatible.
That task must implement only the QEMU/substitute pointer-copy smoke for the
proof-only talos_copy_probe boundary and must keep descriptor I/O, process
loading, VFS/filesystem, shell, networking, SSH, and Pi 5 pointer-copy hardware
proof blocked.
