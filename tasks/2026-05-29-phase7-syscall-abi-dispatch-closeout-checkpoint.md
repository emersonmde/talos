# Phase 7 Syscall ABI and Dispatch Closeout Checkpoint

Task: phase7-syscall-abi-dispatch-closeout-checkpoint-20260529
Status: accepted
Date: 2026-05-29

## Scope

This documentation-only checkpoint reconciles the accepted Phase 7.3 syscall
ABI, lower-AArch64 trap routing, QEMU and Pi 5 scalar syscall proof, copy
helpers, proof-only pointer-copy evidence, descriptor-write evidence, retained
gates, blocked surfaces, and Milestone 7.4 source-inventory recommendation.

It did not add Rust or assembly behavior, rerun QEMU, rerun Pi 5 hardware,
publish a boot archive, acquire hardwareTestLock, implement Milestone 7.4,
add process loading, VFS/filesystem behavior, shell behavior, networking, SSH,
RP1/PCIe work, UART interrupt ownership, DMA/cache-driver policy, or perform a
phase transition.

## Accepted Evidence

- static inspection: git status --short before edits was clean.
- scalar QEMU syscall evidence:
  tasks/evidence/2026-05-29-qemu-syscall-smoke-core/qemu-syscall-smoke.log.
- scalar Pi 5 syscall proof:
  tasks/evidence/2026-05-29-pi5-syscall-proof/local3-candidate-rerun/rerun-proof-lines.txt.
- QEMU pointer-copy evidence:
  tasks/evidence/2026-05-29-qemu-pointer-copy-smoke-core/qemu-pointer-copy-smoke.log.
- Pi 5 pointer-copy proof:
  tasks/evidence/2026-05-29-pi5-pointer-copy-proof/local3-candidate-rerun/rerun-proof-lines.txt.
- QEMU descriptor-write evidence:
  tasks/evidence/2026-05-29-qemu-descriptor-write-smoke-core/qemu-descriptor-write-smoke.log.
- Pi 5 descriptor-write proof:
  tasks/evidence/2026-05-29-pi5-descriptor-write-proof/local3-candidate-rerun/rerun-proof-lines.txt.
- accepted closeout document:
  docs/src/project/phase7-syscall-abi-dispatch-closeout-checkpoint.md.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Accepted Capability

Milestone 7.3 accepts stable lower-AArch64 svc #0 syscall routing, the x8
syscall-number and x0-through-x5 scalar argument convention, x0 return/-errno
encoding, talos_nop and unknown-syscall dispatch, diagnostic marker
quarantine, target-independent copy_from_user/copy_to_user helpers,
proof-only talos_copy_probe evidence, and talos_write fd 1/fd 2 writes through
proof-owned inherited runtime-console0 stdio descriptors.

The accepted physical evidence covers scalar syscall routing, proof-only
pointer-copy helper plumbing, and descriptor-backed stdout/stderr writes on
Pi 5. It does not accept stdin/read, close, dup, process-owned descriptors,
loaded user programs, filesystems, shell behavior, networking, SSH, or full
POSIX descriptor readiness.

## Deferred Work

stdin/read, close, dup, pipes, sockets, poll/select, descriptor lifetime,
process-owned descriptor tables, process-owned address spaces, program
loading, argv/envp, exit/wait, VFS/filesystem behavior, path copying, shell,
networking, SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
restart semantics, signals, lower-EL data-abort recovery, and full POSIX
descriptor claims remain blocked until later explicit tasks.

## Next Action

phase7-file-descriptor-table-source-inventory-20260529 is mechanically
unblocked for the next worker wake if durable state, the working tree, and
hardwareTestLock remain compatible. That task must stay documentation-only and
inventory Milestone 7.4 source owners and contract gaps before any descriptor
lifetime, close/dup/read, VFS/filesystem, shell, networking, SSH, or hardware
work.
