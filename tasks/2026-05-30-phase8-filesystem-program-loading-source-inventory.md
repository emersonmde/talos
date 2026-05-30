# Phase 8 Filesystem/Program-Loading Source Inventory Task

Task: phase8-filesystem-program-loading-source-inventory-20260530

Status: accepted

## Scope

Documentation-only source inventory after the accepted Phase 7 final closeout
recommendation flag. The task maps existing owners and missing contracts for
VFS/filesystem objects, path copying, executable/program loading,
address-space setup, descriptor inheritance, argv/envp setup, process identity,
and boot/test scenarios.

Non-goals: no Rust, assembly, QEMU execution, Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, filesystem/VFS implementation,
program loading, shell, networking, SSH, RP1/PCIe, UART interrupt ownership, or
DMA/cache-driver policy.

## Evidence

- static inspection: git status --short before edits was clean.
- static source review: inspected src/posix.rs, src/syscall.rs,
  src/scheduler.rs, src/runtime_console.rs, src/tty.rs,
  src/arch/aarch64/exceptions.rs, src/target/qemu_virt.rs,
  src/target/rpi5.rs, and accepted Phase 7 closeout/source-inventory docs.
- documentation diff: added
  docs/src/project/phase8-filesystem-program-loading-source-inventory.md,
  linked it from docs/src/SUMMARY.md, updated docs/src/roadmap.md and
  docs/src/decisions/README.md, and added this task record.
- recommendation: next bounded task should be
  phase8-readonly-initramfs-vfs-contract-20260530.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Result

Accepted and committed as the first Phase 8.1 documentation-only inventory.
No Phase 8 runtime behavior is accepted. Read-only initramfs/VFS contract work
is recommended next; ELF/program loading, argv/envp setup, process creation,
shell, networking, SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver
policy, and Pi 5 hardware proof remain blocked.
