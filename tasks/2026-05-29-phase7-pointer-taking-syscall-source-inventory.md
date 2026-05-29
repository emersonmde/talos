# Phase 7 Pointer-Taking Syscall Source Inventory

Task: phase7-pointer-taking-syscall-source-inventory-20260529
Status: accepted

## Scope

This documentation-only task inventoried the source owners and gaps for the
first lower-EL syscall that passes a user pointer into the accepted
copy-in/copy-out helper boundary. It did not change Rust, assembly, boot
scenarios, QEMU scripts, Pi 5 hardware state, descriptor I/O, process loading,
VFS, filesystem, shell behavior, networking, SSH, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver policy.

## Source Review Summary

- src/arch/aarch64/exceptions.rs owns lower-AArch64 svc #0 routing, saved
  x0-through-x5 argument capture, x8 syscall-number extraction, x0 return
  mutation, and RoutedSyscall proof logging.
- src/syscall.rs owns the stable svc #0 vocabulary, diagnostic 0x7a10
  quarantine, the current talos_nop-only syscall namespace, SyscallReturn
  negative errno encoding, and -EFAULT/-EINVAL/-ENOSYS mappings.
- src/posix.rs owns UserMapping, UserAccessKind, validate_user_memory_access(),
  copy_from_user(), copy_to_user(), DEFAULT_USER_COPY_LIMIT, and
  all-or-nothing helper tests.
- src/target/qemu_virt.rs and scripts/qemu-syscall-smoke.sh own the current
  QEMU/substitute syscall smoke scenario, payload, output vocabulary, and
  script gate pattern for any later pointer-copy smoke.
- src/target/rpi5.rs owns the physical scalar syscall proof path, but no Pi 5
  pointer-copy proof is unblocked by this inventory.

## Evidence

- static inspection: git status --short before edits was clean.
- static source-owner/gap summary: mapped frame argument extraction, syscall
  number/dispatch ownership, user-memory mapping provenance, copy helper call
  ownership, return/error encoding, QEMU smoke ownership, and proof-only
  diagnostic-surface quarantine.
- static documentation diff summary: added
  docs/src/project/phase7-pointer-taking-syscall-source-inventory.md; updated
  docs/src/roadmap.md, docs/src/decisions/README.md, docs/src/SUMMARY.md; added
  this task record.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Next Action

Supervisor planning should create
phase7-pointer-taking-syscall-contract-20260529 before any implementation or
QEMU pointer-copy smoke plan is promoted. The contract should decide
proof-only versus stable syscall status, assign the syscall number and
x0-through-x5 argument roles, define user mapping/backing-storage provenance,
define success and -EFAULT observations, and keep diagnostic output
quarantined.

phase7-qemu-pointer-copy-smoke-plan-20260529 remains dependency-blocked until
that contract is accepted. Descriptor I/O, process loading, VFS/filesystem,
shell, networking, SSH, and Pi 5 pointer-copy hardware proof remain blocked.
