# Phase 7 QEMU Syscall Smoke Plan

Task: phase7-qemu-syscall-smoke-plan-20260529
Status: accepted

## Scope

This documentation-only task defined the first QEMU-only production syscall
smoke plan before runtime routing implementation. It did not change Rust,
assembly, boot scenarios, QEMU scripts, Pi 5 hardware state, descriptor I/O,
copy-in/copy-out, process loading, VFS, filesystem, shell behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

## Plan Summary

- The next implementation should add a qemu_syscall_smoke boot scenario.
- The lower-EL payload must execute stable svc #0 with x8 = 0 and observe
  x0 = 0 after return.
- The payload must then execute stable svc #0 with x8 = 17 and observe
  x0 = 0xffffffffffffffda, the two's-complement encoding of -ENOSYS.
- The smoke may use diagnostic SVC marker 0x7a10 only as proof-owned
  completion vocabulary after the production return observations.
- The retained QEMU serial log must include production syscall case lines,
  user-observed return lines, diagnostic-marker quarantine, final
  classification, and PASS.

## Required Lines

The implementation task must retain a QEMU serial log with these final lines:

    qemu-syscall-smoke: final participants=2 expected=2 errors=0 classification=qemu-syscall-smoke-complete
    qemu-syscall-smoke: PASS

It must also include the required qemu-syscall-smoke syscall case,
user-observed, and diagnostic-marker quarantine lines named in
docs/src/project/phase7-qemu-syscall-smoke-plan.md.

## Evidence

- static inspection: git status --short before edits was clean.
- static documentation diff summary: added
  docs/src/project/phase7-qemu-syscall-smoke-plan.md; updated
  docs/src/SUMMARY.md, docs/src/roadmap.md, and docs/src/decisions/README.md;
  added this task record.
- plan summary: defined qemu_syscall_smoke invariant, stable svc #0 talos_nop
  and unknown-syscall behavior, exact PASS/classification lines, retained log
  requirements, diagnostic-marker quarantine, local validation gates, QEMU-only
  evidence level, and blocked Pi 5/descriptor/copy/filesystem/shell/network
  surfaces.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Next Action

The next mechanically unblocked task is phase7-qemu-syscall-smoke-core-20260529
after this plan is accepted and committed. It should implement only the
QEMU-only qemu_syscall_smoke routing, payload, script gate, retained evidence,
and diagnostic proof preservation defined by this plan.
