# Phase 7 Descriptor Syscall Contract

Task: phase7-descriptor-syscall-contract-20260529
Status: accepted
Date: 2026-05-29

## Scope

This documentation-only task accepted the first descriptor syscall contract
after the descriptor syscall source inventory. It defined a bounded
stdout/stderr write syscall boundary backed by runtime-console0.

It did not change Rust or assembly behavior, run QEMU, run Pi 5 hardware,
publish boot archives, acquire hardwareTestLock, implement descriptor I/O, add
process loading, add VFS/filesystem behavior, add shell behavior, or change
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

## Contract Summary

- Stable syscall: talos_write, selected by svc #0 and x8 = 1.
- Arguments: x0 = fd, x1 = user pointer, x2 = length, x3 through x5 reserved
  zero.
- Accepted descriptors: fd 1 and fd 2, both write-capable
  DescriptorObjectKind::StdioOutput objects from inherited stdio.
- User-copy rule: validate and copy the complete user byte range with
  copy_from_user() before any runtime-console0 side effect.
- Success return: exact requested byte count, or 0 for zero-length writes after
  descriptor/reserved-register validation.
- Error returns: -EBADF, -ENOTSUP, -EFAULT, -EINVAL, future -EIO for backend
  failure once encoded, and unchanged -ENOSYS for unknown syscalls.
- Quarantine: proof-only talos_copy_probe remains separate and must not become
  a descriptor write selector or stable ABI precedent.

## Files Reviewed

- docs/src/project/phase7-descriptor-syscall-source-inventory.md
- docs/src/project/phase7-syscall-abi-contract.md
- docs/src/project/phase7-syscall-trap-routing-contract.md
- docs/src/project/phase7-pointer-taking-syscall-contract.md
- src/syscall.rs
- src/posix.rs
- src/runtime_console.rs

## Changed Files

- docs/src/project/phase7-descriptor-syscall-contract.md
- docs/src/SUMMARY.md
- docs/src/roadmap.md
- docs/src/decisions/README.md
- tasks/2026-05-29-phase7-descriptor-syscall-contract.md

## Recommendation

The next bounded task should be a QEMU descriptor-write smoke plan or
target-independent descriptor-write implementation core, depending on
supervisor decomposition. The evidence should cover fd 1/fd 2 success,
invalid-descriptor and unsupported-object failures without console side
effects, guard-range -EFAULT, reserved-register -EINVAL, and unchanged
talos_nop, unknown-syscall, and proof-only talos_copy_probe behavior.

## Validation

- static inspection: reviewed the accepted source inventory, syscall ABI and
  trap-routing contracts, pointer-copy contract, and relevant syscall, POSIX,
  and runtime-console source files.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- Rust fmt/tests, QEMU runs, and Pi 5 hardware runs were not required because
  this task changes only Markdown documentation and durable worker state.
