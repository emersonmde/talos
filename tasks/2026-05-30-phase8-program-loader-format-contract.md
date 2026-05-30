# Phase 8 Program Loader Format Contract Task

Task: phase8-program-loader-format-contract-20260530

Status: accepted

## Scope

Documentation-only loader format contract after the accepted program-loader
source inventory. The task defines the first executable image subset,
deterministic rejection matrix, segment permission and zero-fill policy,
entry-point validation, process-install boundary, deferred stack/descriptor
surfaces, and next bounded smoke-plan recommendation.

Non-goals: no Rust, assembly, QEMU execution, Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, ELF parser implementation, process
creation, exec/spawn/wait, argv/envp stack implementation, shell, writable
filesystem, persistent storage, networking, SSH, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver policy.

## Evidence

- static inspection: git status --short before edits was clean.
- static documentation/source review: used the accepted program-loader source
  inventory, read-only initramfs/VFS closeout, Phase 7 EL0/address-space
  contract, copy-in/copy-out contract, POSIX baseline loader vocabulary,
  src/initramfs.rs, src/posix.rs, src/syscall.rs, roadmap, and ADR index.
- documentation diff: added
  docs/src/project/phase8-program-loader-format-contract.md, linked it from
  docs/src/SUMMARY.md, updated docs/src/roadmap.md and
  docs/src/decisions/README.md, and added this task record.
- recommendation: next bounded task should be
  phase8-qemu-program-loader-smoke-plan-20260530.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Result

Accepted as the first Milestone 8.3 loader format contract. The accepted
format policy is a narrow static ELF64/AArch64 executable subset consumed from
the read-only initramfs/VFS regular-file boundary. Loader implementation,
process address-space installation, argv/envp stack construction, process
creation, exec/spawn/wait, shell behavior, descriptor-backed filesystem
syscalls, Pi 5 hardware proof, writable filesystems, persistent storage,
networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
policy remain blocked.
