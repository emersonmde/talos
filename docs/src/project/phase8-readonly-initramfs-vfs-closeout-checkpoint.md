# Phase 8 Read-Only Initramfs/VFS Closeout Checkpoint

Status: accepted closeout checkpoint for
phase8-readonly-initramfs-vfs-closeout-checkpoint-20260530.

## Scope

This documentation-only checkpoint reconciles the accepted Milestone 8.1
read-only initramfs/VFS contract, smoke plan, target-independent core, and
QEMU/substitute smoke evidence before any program-loading or shell work.

It adds no Rust or assembly behavior, reruns no QEMU scenario, performs no
Raspberry Pi 5 hardware action, publishes no boot archive, and acquires no
hardwareTestLock. It does not accept descriptor-backed production filesystem
syscalls, an open syscall ABI, directory iteration, seek syscalls, object final
release, firmware/TFTP initramfs delivery, ELF/program loading, argv/envp
setup, process creation, exec/spawn/wait, shell behavior, writable
filesystems, persistent storage, networking, SSH, RP1/PCIe, UART interrupt
ownership, or DMA/cache-driver policy.

## Accepted Capability

The accepted slice is a bounded read-only initramfs/VFS proof surface:

- the documentation contract defines the immutable initial filesystem content
  model, root/directory/regular-file vocabulary, lookup and read semantics,
  deterministic errno precedence, fixture expectations, and deferred surfaces;
- the smoke plan defines the exact qemu_readonly_initramfs_vfs_smoke scenario,
  retained evidence path, PASS/classification lines, success cases, negative
  cases, and scaled regression policy;
- the target-independent core adds src/initramfs.rs with the stable
  phase8-readonly-initramfs-vfs-v1 fixture, normalized lookup, immutable
  metadata, regular-file open-file descriptions, offset/EOF handling, and
  all-or-nothing copy_to_user-backed reads; and
- the retained QEMU/substitute smoke proves the planned fixture identity,
  lookup, successful reads, offset/EOF behavior, and deterministic errno cases
  through serial/log output.

The stable fixture contains /etc/banner.txt, /bin/init as regular-file data
only, /empty, and /dir/nested.txt. /bin/init is not executable in this slice.

## Evidence Reconciliation

| Task | Commit | Evidence level | Result |
| --- | --- | --- | --- |
| phase8-readonly-initramfs-vfs-contract-20260530 | b9c724cbdbb9dfb6b960668a13047a9bd1b6b602 | static documentation/source-owner inspection | accepted contract and deferred-surface boundary |
| phase8-readonly-initramfs-vfs-smoke-plan-20260530 | 978fb2d492c835aabaad5a1fff0f9f5d23330f0e | static documentation inspection | accepted QEMU/substitute smoke plan, exact output, and evidence path |
| phase8-readonly-initramfs-vfs-core-20260530 | f0dc488f78cb1dacbe9db3a9f6102fd61e66b053 | target-independent source/tests | accepted immutable fixture core and 261 no_std tests |
| phase8-qemu-readonly-initramfs-vfs-smoke-core-20260530 | 1146b51900b6e9da8b307fd0ce7d6779fe3b25a1 | QEMU/substitute serial/log evidence | accepted retained smoke PASS/classification evidence |

Retained QEMU/substitute evidence:

- tasks/evidence/2026-05-30-qemu-readonly-initramfs-vfs-smoke-core/qemu-readonly-initramfs-vfs-smoke.log
- fixture identity: phase8-readonly-initramfs-vfs-v1,
  stable-manifest digest 0x2a2a56c54aecce72
- final lines:
  qemu-readonly-initramfs-vfs-smoke: final participants=8 expected=8 errors=0
  classification=qemu-readonly-initramfs-vfs-smoke-complete
  qemu-readonly-initramfs-vfs-smoke: PASS

The retained log includes /, /etc/banner.txt, /empty, and /dir/nested.txt
success observations and the planned ENOENT, ENOTDIR, EISDIR, ENAMETOOLONG,
EBADF, EFAULT, EINVAL, and ENOTSUP negative cases.

## Deferred Surfaces

The following remain explicitly blocked until later tasks define and accept
their contracts and validation gates:

- descriptor-backed production filesystem syscalls and an open syscall ABI;
- directory iteration, seek syscalls, final object release, close-on-exec
  inheritance, and descriptor integration for filesystem-backed objects;
- firmware/TFTP initramfs envelopes, CPIO compatibility, compression, boot
  archive publication, and Pi 5 physical proof;
- ELF/program loading, executable /bin/init behavior, segment validation,
  argv/envp setup, process creation, exec/spawn/wait, and shell behavior;
- writable filesystems, persistent storage, block devices, networking, SSH,
  RP1/PCIe, UART interrupt ownership, and DMA/cache-driver policy.

## Residual Risks

- The accepted fixture is compiled into the kernel for unit and QEMU/substitute
  proof. Later work still needs an initramfs transport envelope and physical
  archive identity before making Pi 5 filesystem claims.
- File-backed descriptor integration is not accepted. The read helper is
  target-independent and fixture-owned, not reachable through production
  talos_open or read syscalls for filesystem objects.
- /bin/init is data only. The next loader work must define executable format
  rules, address-space ownership, user stack layout, and loader error mapping
  before any process image can run.

## Recommended Next Task

The next bounded Phase 8 task should be
phase8-program-loader-source-inventory-20260530, documentation-only under
Milestone 8.3.

That inventory should map source owners and missing contracts for executable
image format selection, ELF header/program-header validation, segment
permissions, zero-fill, entry-point validation, user address-space ownership,
argv/envp stack layout, loader error mapping, and how a loaded image becomes a
process program. It should use the accepted read-only initramfs/VFS regular
file as an input source, but must keep process creation, exec/spawn/wait,
shell behavior, Pi 5 hardware proof, networking, SSH, RP1/PCIe, UART interrupt
ownership, and DMA/cache-driver policy blocked until later explicit tasks.

## Validation

- static inspection: git status --short before edits was clean.
- static evidence review: inspected the accepted Phase 8 source inventory,
  read-only initramfs/VFS contract, smoke plan, core task record, QEMU smoke
  task record, roadmap, ADR index, and retained QEMU/substitute evidence path.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- hardware: no Pi 5 archive publication, hardware-lock acquisition, power
  cycle, or serial observation was performed by this closeout.
