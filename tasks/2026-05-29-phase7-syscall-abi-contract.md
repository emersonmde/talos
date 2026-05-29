# Phase 7 Syscall ABI Contract

Task: phase7-syscall-abi-contract-20260529
Status: accepted

## Scope

This documentation-only task defined the first bounded Phase 7.3 syscall ABI
contract after the accepted source inventory. It did not change Rust,
assembly, boot scenarios, QEMU scripts, Pi 5 hardware state, process loading,
descriptor I/O, VFS, filesystem, shell behavior, networking, SSH, RP1/PCIe,
UART interrupt ownership, or DMA/cache-driver policy.

## Accepted Contract

- Stable syscall trap: lower-AArch64 synchronous SVC with SVC immediate 0.
- Diagnostic marker 0x7a10 remains proof-only and is not a syscall number.
- Register convention: x8 is syscall number, x0 through x5 are scalar
  arguments, x0 is the sole return register, and negative x0 values encode
  -errno.
- First syscall namespace: talos_nop = 0 returns 0; all other syscall numbers
  return -ENOSYS when the trap frame is valid.
- First errno subset: EINVAL=22, EBADF=9, EFAULT=14, ENOSYS=38, ENOTSUP=95.
- First proof slice: target-independent dispatch/error conversion only, with
  implementation, QEMU, and Pi 5 hardware proof deferred to explicit later
  tasks.

## Evidence

- static documentation diff summary: added
  docs/src/project/phase7-syscall-abi-contract.md; updated
  docs/src/roadmap.md, docs/src/decisions/README.md, and
  docs/src/SUMMARY.md; added this task record.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Next Action

The next mechanically unblocked task is phase7-syscall-dispatch-core-20260529,
bounded to target-independent syscall dispatch vocabulary, negative errno
encoding, and unit tests. Production exception routing, QEMU syscall smoke, and
Pi 5 hardware proof remain blocked.
