# Phase 7 Syscall Trap-Routing Source Inventory

Task: phase7-syscall-trap-routing-source-inventory-20260529
Status: accepted

## Scope

This documentation-only task inventoried the production exception-routing
boundary needed to connect lower-AArch64 SVC traps to the accepted
target-independent syscall dispatch core. It did not change Rust, assembly,
boot scenarios, QEMU scripts, Pi 5 hardware state, descriptor I/O,
copy-in/copy-out, process loading, VFS, filesystem, shell behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

## Source Review Summary

- src/arch/aarch64/vectors.S owns vector classification, saved x0-through-x30
  frame layout, ESR/ELR/FAR/SPSR reads, and the current fatal synchronous
  exception handoff.
- src/arch/aarch64/exceptions.rs owns ExceptionVector, immutable
  ExceptionFrame::reg() access, cfg-gated diagnostic proof interception, and
  the non-syscall fatal fallback.
- src/syscall.rs owns stable svc #0 vocabulary, diagnostic 0x7a10 quarantine,
  x8 syscall-number vocabulary, x0-through-x5 scalar argument view, x0
  return/error encoding, talos_nop success, and unknown-syscall -ENOSYS.
- src/target/qemu_virt.rs and src/target/rpi5.rs own proof-only diagnostic EL0
  SVC marker 0x7a10 handling; those paths remain quarantined from the stable
  syscall ABI.
- src/scheduler.rs owns optional task/process-owner metadata, but process
  lifetime, descriptor lifetime, PID, wait/exit, and blocking I/O policy are
  not available to the first routing slice.

## Evidence

- static inspection: git status --short before edits was clean.
- static documentation diff summary: added
  docs/src/project/phase7-syscall-trap-routing-source-inventory.md; updated
  docs/src/roadmap.md, docs/src/decisions/README.md, and docs/src/SUMMARY.md;
  added this task record.
- static source-review summary: mapped exact source owners for lower-AArch64
  SVC detection, SVC immediate validation, x8 syscall-number extraction,
  x0-through-x5 argument capture, x0 return mutation, ELR handling, and
  non-syscall fallback. Diagnostic marker 0x7a10 remains proof-only.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Next Action

The next mechanically unblocked task is
phase7-syscall-trap-routing-contract-20260529. It should specify production
routing inputs, outputs, mutation rules, preserved-register and ELR/SPSR
behavior, non-syscall fallback, and QEMU syscall smoke requirements before any
implementation. Descriptor I/O, copy-in/copy-out, process loading, VFS,
filesystem, shell, networking, SSH, and Pi 5 syscall hardware proof remain
blocked.
