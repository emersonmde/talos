# Phase 8 Program Loader Source Inventory Task

Task: phase8-program-loader-source-inventory-20260530

Status: accepted

## Scope

Documentation-only source inventory after the accepted read-only initramfs/VFS
closeout. The task maps source owners and missing contracts for loading an
executable image from the accepted read-only fixture boundary.

Non-goals: no Rust, assembly, QEMU execution, Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, ELF parser, loader implementation,
user page mapping, process creation, exec/spawn/wait, argv/envp setup, shell,
networking, SSH, RP1/PCIe, UART interrupt ownership, or DMA/cache-driver
policy.

## Evidence

- static inspection: git status --short before edits was clean.
- static source/documentation review: inspected the accepted read-only
  initramfs/VFS closeout, Phase 8 source inventory and contract, Phase 7
  EL0/address-space and process-descriptor contracts, src/initramfs.rs,
  src/posix.rs, src/syscall.rs, src/scheduler.rs, src/memory_map/layout.rs,
  src/memory_map/translation.rs, src/arch/aarch64/exceptions.rs,
  src/target/qemu_virt.rs, src/target/rpi5.rs, roadmap, and ADR index.
- documentation diff: added
  docs/src/project/phase8-program-loader-source-inventory.md, linked it from
  docs/src/SUMMARY.md, updated docs/src/roadmap.md and
  docs/src/decisions/README.md, and added this task record.
- recommendation: next bounded task should be
  phase8-program-loader-format-contract-20260530.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Result

Accepted and committed as the first Milestone 8.3 documentation-only program
loader inventory. No loader runtime behavior is accepted. The loader format
contract is recommended next; ELF parsing, user page mapping, process creation,
exec/spawn/wait, argv/envp setup, shell, networking, SSH, RP1/PCIe, UART
interrupt ownership, DMA/cache-driver policy, Pi 5 hardware proof, writable
filesystems, and persistent storage remain blocked.
