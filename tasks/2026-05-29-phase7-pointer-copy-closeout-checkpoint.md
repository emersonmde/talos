# Phase 7 Pointer-Copy Closeout Checkpoint

Task: phase7-pointer-copy-closeout-checkpoint-20260529
Status: accepted

## Scope

This documentation-only checkpoint reconciled the accepted pointer-taking
syscall contract, QEMU pointer-copy smoke plan, QEMU pointer-copy smoke core,
retained QEMU/substitute evidence, regression gates, and deferred surfaces.
It added no Rust or assembly behavior, QEMU rerun, Pi 5 hardware run,
hardwareTestLock acquisition, boot archive publication, descriptor I/O,
runtime console or TTY integration, process loading, VFS/filesystem behavior,
path copying, shell behavior, networking, SSH, RP1/PCIe work, UART interrupt
ownership, DMA/cache-driver policy, demand paging, copy-on-write,
signal/restart semantics, or lower-EL fault-table recovery.

## Changed Files

- docs/src/project/phase7-pointer-copy-closeout-checkpoint.md: added the
  closeout record for accepted QEMU/substitute pointer-copy evidence.
- docs/src/SUMMARY.md: linked the closeout document.
- docs/src/roadmap.md: moved the current Phase 7.3 frontier from QEMU
  pointer-copy smoke core to the closeout checkpoint and recorded deferred
  surfaces.
- docs/src/decisions/README.md: added the closeout ADR.
- tasks/2026-05-29-phase7-pointer-copy-closeout-checkpoint.md: recorded this
  task.

## Accepted Evidence

- static inspection: git status --short before edits was clean.
- retained QEMU/substitute evidence:
  tasks/evidence/2026-05-29-qemu-pointer-copy-smoke-core/qemu-pointer-copy-smoke.log.
- referenced retained classification lines:
  qemu-pointer-copy-smoke: final participants=3 expected=3 errors=0
  classification=qemu-pointer-copy-smoke-complete; qemu-pointer-copy-smoke:
  PASS.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Accepted Capability

The accepted capability remains QEMU/substitute evidence only: lower-EL stable
svc #0 can route proof-only x8 = 0x7001 talos_copy_probe through the saved
frame syscall path in qemu_pointer_copy_smoke, invoke the accepted copy helpers
against explicit UserData mapping/backing storage, return 16 for the 16-byte
success case, return -EFAULT for the guard case, preserve unknown-syscall
-ENOSYS behavior, and keep diagnostic marker 0x7a10 outside syscall dispatch.

## Deferred Work

Pi 5 pointer-copy hardware proof, descriptor read/write/close/dup syscalls,
runtime console or TTY-backed stdio, process loading, process-owned address
spaces, VFS/filesystem behavior, path copying, argv/envp loading, shell
behavior, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, demand paging, copy-on-write, signal/restart
semantics, and lower-EL fault-table recovery remain blocked until later
explicit tasks.

## Next Action

Supervisor planning is required before the next task because no explicit
queued follow-up exists after this closeout. The recommended bounded direction
is a documentation-only Pi 5 pointer-copy proof plan that preserves
hardwareTestLock serialization, candidate identity, fresh TFTP/serial evidence,
inconclusive-run triage, restoration proof, and blocked descriptor/process/
filesystem/networking surfaces.
