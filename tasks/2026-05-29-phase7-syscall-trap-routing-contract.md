# Phase 7 Syscall Trap-Routing Contract

Task: phase7-syscall-trap-routing-contract-20260529
Status: accepted

## Scope

This documentation-only task defined the production syscall trap-routing
contract needed before implementation. It did not change Rust, assembly, boot
scenarios, QEMU scripts, Pi 5 hardware state, descriptor I/O,
copy-in/copy-out, process loading, VFS, filesystem, shell behavior,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

## Contract Summary

- Production syscall routing accepts only lower-AArch64 synchronous exceptions
  with ESR EC 0x15 and stable svc #0.
- x8 supplies the syscall number, x0 through x5 supply scalar arguments, and
  x0 receives the dispatch return value.
- talos_nop returns 0. Unknown syscall numbers return -ENOSYS.
- ELR and SPSR are preserved; the implementation must not blindly advance ELR
  because accepted lower-EL proof evidence already reports post-SVC ELR.
- Non-syscall traps remain on the existing fatal exception path.
- Diagnostic SVC marker 0x7a10 remains proof-only and must not become stable
  ABI behavior.
- QEMU syscall smoke is mandatory before claiming production syscall routing.

## Evidence

- static inspection: git status --short before edits was clean.
- static documentation diff summary: added
  docs/src/project/phase7-syscall-trap-routing-contract.md; updated
  docs/src/SUMMARY.md, docs/src/roadmap.md, and docs/src/decisions/README.md;
  added this task record.
- static contract summary: defined production routing inputs, outputs,
  mutation rules, failure classes, diagnostic proof quarantine, and QEMU smoke
  requirements while keeping descriptor I/O, copy-in/copy-out, process
  loading, filesystem, shell, networking, SSH, and Pi 5 syscall proof blocked.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Next Action

The next mechanically unblocked task is
phase7-qemu-syscall-smoke-plan-20260529. It should define the exact QEMU
payload, expected svc #0 talos_nop and unknown-syscall return evidence,
classification/PASS lines, retained log path, and diagnostic-proof
preservation or quarantine requirements before runtime routing implementation.
