# Phase 7 Descriptor-Write Closeout Checkpoint

Task: phase7-descriptor-write-closeout-checkpoint-20260529
Status: accepted
Date: 2026-05-29

## Scope

This documentation-only checkpoint reconciles the accepted descriptor source
inventory, descriptor syscall contract, QEMU descriptor-write smoke plan,
descriptor-write core implementation, retained QEMU/substitute evidence,
regression gates, residual risks, and deferred surfaces.

It did not add Rust or assembly behavior, rerun QEMU, publish a Pi 5 boot
archive, acquire hardwareTestLock, observe physical serial output, add
stdin/read, close, dup, process loading, VFS/filesystem behavior, path copying,
shell behavior, networking, SSH, RP1/PCIe work, UART interrupt ownership,
DMA/cache-driver policy, live process-owned descriptor tables, or a phase
transition.

## Accepted Evidence

- static inspection: git status --short before edits showed a pre-existing
  docs/src/roadmap.md working-tree edit that was preserved.
- static inspection: reviewed retained QEMU/substitute evidence at
  tasks/evidence/2026-05-29-qemu-descriptor-write-smoke-core/qemu-descriptor-write-smoke.log.
- accepted implementation/evidence commit:
  26c36ffaada05e4ba598144c44f49210534b233a.
- retained classification lines:
  qemu-descriptor-write-smoke: final participants=8 expected=8 errors=0
  classification=qemu-descriptor-write-smoke-complete; qemu-descriptor-write-smoke:
  PASS.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Accepted Capability

The accepted capability is only QEMU/substitute evidence that lower-EL stable
svc #0 can route talos_write x8 = 1 through the descriptor-write syscall core.
The retained log proves fd 1 and fd 2 write 18-byte UserData buffers through
DescriptorTable::with_inherited_stdio(), copy_from_user(), and
runtime-console0, while fd 0 and fd 99 return -EBADF, the guard range returns
-EFAULT, nonzero reserved x3 returns -EINVAL, talos_nop and unknown-syscall
behavior remains intact, x8 = 0x7001 remains -ENOSYS outside proof scenarios,
and diagnostic marker 0x7a10 remains outside stable syscall dispatch.

## Deferred Work

Pi 5 descriptor-write hardware proof, stdin/read, close, dup, process loading,
process-owned descriptor tables, process-owned address spaces, VFS/filesystem
behavior, path copying, argv/envp loading, shell behavior, networking, SSH,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, blocking/readiness,
signals, restart semantics, lower-EL fault-table recovery, and full POSIX
descriptor claims remain blocked until later explicit tasks.

## Next Action

phase7-pi5-descriptor-write-proof-plan-20260529 is mechanically unblocked for
the next worker wake if durable state, the working tree, and hardwareTestLock
remain compatible. That task should stay documentation-only and plan the
serialized Pi 5 descriptor-write proof before any hardware action.
