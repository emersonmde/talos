# Phase 8 Read-Only Initramfs/VFS Closeout Checkpoint

Status: accepted for
phase8-readonly-initramfs-vfs-closeout-checkpoint-20260530.

## Scope

- Reconciled the accepted read-only initramfs/VFS contract, smoke plan,
  target-independent core, retained QEMU/substitute evidence, validation gates,
  deferred surfaces, and residual risks.
- Added docs/src/project/phase8-readonly-initramfs-vfs-closeout-checkpoint.md
  and linked it from docs/src/SUMMARY.md.
- Updated docs/src/roadmap.md and docs/src/decisions/README.md.
- Did not change Rust or assembly behavior, rerun QEMU, run Pi 5 hardware,
  publish a boot archive, or acquire hardwareTestLock.

## Accepted Reconciliation

- Contract commit: b9c724cbdbb9dfb6b960668a13047a9bd1b6b602.
- Smoke plan commit: 978fb2d492c835aabaad5a1fff0f9f5d23330f0e.
- Target-independent core commit: f0dc488f78cb1dacbe9db3a9f6102fd61e66b053.
- QEMU/substitute smoke commit:
  1146b51900b6e9da8b307fd0ce7d6779fe3b25a1.
- Retained smoke evidence:
  tasks/evidence/2026-05-30-qemu-readonly-initramfs-vfs-smoke-core/qemu-readonly-initramfs-vfs-smoke.log.
- PASS/classification:
  qemu-readonly-initramfs-vfs-smoke: final participants=8 expected=8 errors=0
  classification=qemu-readonly-initramfs-vfs-smoke-complete
  qemu-readonly-initramfs-vfs-smoke: PASS.

## Deferred Surfaces

Descriptor-backed production filesystem syscalls, open syscall ABI, directory
iteration, seek syscalls, final object release, firmware/TFTP initramfs
delivery, Pi 5 filesystem proof, ELF/program loading, executable /bin/init,
argv/envp setup, process creation, exec/spawn/wait, shell behavior, writable
filesystems, persistent storage, networking, SSH, RP1/PCIe, UART interrupt
ownership, and DMA/cache-driver policy remain blocked.

## Next Recommendation

Recommend phase8-program-loader-source-inventory-20260530 as the next bounded
documentation-only Phase 8 task. It should inventory executable image and ELF
loader source owners and missing contracts using the accepted read-only VFS
regular-file boundary as input, without accepting process creation, shell,
hardware, networking, or driver surfaces.

## Evidence

- static inspection: git status --short before edits was clean.
- static evidence review: inspected the accepted contract, smoke plan, core
  task record, QEMU/substitute smoke task record, roadmap, ADR index, and
  retained evidence path.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Next Action

Accepted and committed. Supervisor planning should create the next explicit
bounded task before any loader implementation begins.
