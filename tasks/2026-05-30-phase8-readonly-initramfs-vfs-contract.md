# Phase 8 Read-Only Initramfs/VFS Contract Task

Task: phase8-readonly-initramfs-vfs-contract-20260530

Status: accepted

## Scope

Documentation-only contract after the accepted Phase 8 filesystem/program
loading source inventory. The task defines the read-only initial filesystem
content model, VFS vocabulary, lookup/read/open-file-description semantics,
path-copy interaction, errno precedence, deterministic fixture expectations,
deferred surfaces, and the next bounded Phase 8.1 tasks.

Non-goals: no Rust, assembly, QEMU execution, Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, VFS implementation, initramfs
parser, descriptor-backed filesystem read, ELF/program loader, argv/envp setup,
process creation, shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
or DMA/cache-driver policy.

## Evidence

- static inspection: git status --short before edits was clean.
- static documentation review: reviewed the accepted Phase 8 source inventory,
  roadmap, ADR index, accepted POSIX path/error vocabulary, read/stdin
  contract, process descriptor table contract, and `src/posix.rs` ownership
  markers for errors, path normalization, and descriptor object kinds.
- documentation diff: added
  docs/src/project/phase8-readonly-initramfs-vfs-contract.md, linked it from
  docs/src/SUMMARY.md, updated docs/src/roadmap.md and
  docs/src/decisions/README.md, and added this task record.
- recommendation: next bounded task should be
  phase8-readonly-initramfs-vfs-smoke-plan-20260530; the dependency-gated core
  task should be phase8-readonly-initramfs-vfs-core-20260530 after the contract
  and smoke plan are accepted.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Result

Accepted and committed as the documentation-only read-only initramfs/VFS
contract. No Phase 8 runtime behavior is accepted. The smoke-plan task is
recommended next; target-independent core implementation, QEMU/substitute
runtime evidence, Pi 5 hardware proof, ELF/program loading, argv/envp setup,
process creation, shell, networking, SSH, RP1/PCIe, UART interrupt ownership,
and DMA/cache-driver policy remain blocked until later explicit tasks accept
their contracts and gates.
