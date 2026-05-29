# Phase 7 Pointer-Taking Syscall Contract

Task: phase7-pointer-taking-syscall-contract-20260529
Status: accepted

## Scope

This documentation-only task contracted the first lower-EL syscall boundary
that carries a user pointer and length into the accepted copy-in/copy-out
helpers. It did not change Rust, assembly, boot scenarios, QEMU scripts,
Pi 5 hardware state, descriptor I/O, process loading, VFS, filesystem, shell
behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

## Contract Summary

- The first pointer-taking boundary is proof-only and QEMU/substitute scoped,
  named talos_copy_probe, with stable svc #0 and syscall number x8 = 0x7001.
- x0 is the user data pointer, x1 is a 0-through-32 byte length, x2 is the
  expected input byte value, x3 is the replacement output byte value, and x4
  and x5 are reserved zero fields.
- Nonzero success copies bytes from user memory into a kernel scratch buffer,
  checks the expected byte pattern, writes the replacement pattern back through
  copy_to_user(), and returns the copied length in x0.
- Zero-length success requires a valid non-guard user address and returns 0
  without byte movement.
- User-boundary failures return -EFAULT, proof-configuration failures return
  -EINVAL, and x8 = 0x7001 outside the proof scenario remains -ENOSYS.
- The QEMU substitute mapping/backing-storage model uses a fixed UserData page
  at 0x0000_0000_0011_0000..0x0000_0000_0011_1000.

## Evidence

- static inspection: git status --short before edits was clean.
- static contract review: inspected the accepted source inventory, syscall ABI
  contract, trap-routing ownership, copy-in/copy-out helper contract/core
  behavior, QEMU syscall smoke plan, syscall dispatch source, lower-AArch64
  route source, and helper source.
- static documentation diff summary: added
  docs/src/project/phase7-pointer-taking-syscall-contract.md; updated
  docs/src/roadmap.md, docs/src/decisions/README.md, docs/src/SUMMARY.md; added
  this task record.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Next Action

phase7-qemu-pointer-copy-smoke-plan-20260529 is now mechanically unblocked for
the next worker wake if durable state and the working tree remain compatible.
That task must plan only the QEMU/substitute pointer-copy smoke for the
proof-only talos_copy_probe boundary and must keep descriptor I/O, process
loading, VFS/filesystem, shell, networking, SSH, and Pi 5 pointer-copy hardware
proof blocked.
