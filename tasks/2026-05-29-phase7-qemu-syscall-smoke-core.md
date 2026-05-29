# Phase 7 QEMU Syscall Smoke Core

Task: phase7-qemu-syscall-smoke-core-20260529
Status: accepted

## Scope

This implementation task added only the QEMU/substitute Phase 7.3 production
syscall smoke required by the accepted trap-routing contract and QEMU syscall
smoke plan. It did not run Pi 5 hardware, publish archives, acquire
hardwareTestLock, add descriptor I/O, byte copy-in/copy-out, pointer-taking
syscalls, process loading, VFS/filesystem behavior, shell behavior, networking,
SSH, RP1/PCIe work, UART interrupt ownership, DMA/cache-driver policy, or
unrelated scheduler changes.

## Accepted Core

- Added qemu_syscall_smoke as a QEMU-only boot scenario.
- Added a recoverable lower-AArch64 synchronous svc #0 route that calls the
  accepted target-independent syscall dispatch core.
- Mutated only saved x0 for syscall returns and preserved the saved ELR/SPSR
  contract.
- The built-in user payload observes talos_nop returning x0 = 0, then observes
  unknown syscall number 17 returning x0 = 0xffffffffffffffda.
- Diagnostic SVC marker 0x7a10 remains proof-owned completion vocabulary and
  is not dispatched as a stable syscall.
- The accepted qemu_el0_trap_smoke diagnostic marker proof remains passing.

## Evidence

- static inspection: git status --short before edits was clean.
- unit tests: cargo -Zjson-target-spec test passed with 198 no_std tests.
- formatting: cargo fmt --all -- --check passed.
- QEMU/substitute diagnostic preservation:
  scripts/qemu-el0-trap-smoke.sh passed; retained log:
  tasks/evidence/2026-05-29-qemu-syscall-smoke-core/qemu-el0-trap-smoke.log.
- QEMU/substitute production syscall smoke:
  scripts/qemu-syscall-smoke.sh passed; retained log:
  tasks/evidence/2026-05-29-qemu-syscall-smoke-core/qemu-syscall-smoke.log.
- QEMU syscall smoke final evidence:

      qemu-syscall-smoke: final participants=2 expected=2 errors=0 classification=qemu-syscall-smoke-complete
      qemu-syscall-smoke: PASS

- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Deferred Work

Pi 5 production syscall proof, descriptor I/O, byte copy-in/copy-out,
pointer-taking syscalls, process loading, VFS/filesystem behavior, shell
behavior, networking, SSH, RP1/PCIe, UART interrupt ownership, and
DMA/cache-driver policy remain blocked until later explicit tasks.
