# Phase 8 QEMU Read-Only Initramfs/VFS Smoke Core

Status: accepted as the QEMU/substitute Milestone 8.1 read-only
initramfs/VFS smoke after the accepted
[Phase 8 Read-Only Initramfs/VFS Smoke Plan](../docs/src/project/phase8-readonly-initramfs-vfs-smoke-plan.md)
and
[Phase 8 Read-Only Initramfs/VFS Core](2026-05-30-phase8-readonly-initramfs-vfs-core.md).

## Scope

- Added the qemu_readonly_initramfs_vfs_smoke boot scenario to build.rs and
  src/main.rs.
- Added src/target/qemu_virt.rs smoke orchestration that exercises only the
  accepted immutable fixture, lookup, regular-file read, EOF/offset, and
  deterministic errno boundary.
- Added scripts/qemu-readonly-initramfs-vfs-smoke.sh to build the scenario,
  run QEMU, grep the accepted output lines, and retain the normalized log.
- Did not acquire hardwareTestLock, publish a Pi 5 boot archive, run Pi 5
  hardware, wire production filesystem syscalls, parse initramfs archives, add
  program loading, or broaden descriptor I/O.

## Retained Evidence

- QEMU/substitute smoke log:
  tasks/evidence/2026-05-30-qemu-readonly-initramfs-vfs-smoke-core/qemu-readonly-initramfs-vfs-smoke.log
- Fixture identity:
  phase8-readonly-initramfs-vfs-v1, stable-manifest digest
  0x2a2a56c54aecce72.
- PASS/classification:
  qemu-readonly-initramfs-vfs-smoke: final participants=8 expected=8 errors=0
  classification=qemu-readonly-initramfs-vfs-smoke-complete
  qemu-readonly-initramfs-vfs-smoke: PASS

The retained log includes the planned success cases for /, /etc/banner.txt,
/empty, and /dir/nested.txt, plus the planned ENOENT, ENOTDIR, EISDIR,
ENAMETOOLONG, EBADF, EFAULT, EINVAL, and ENOTSUP negative cases. The EBADF,
EFAULT, and EINVAL cases report offset-unchanged=true.

## Regression Scope

The smoke touches boot-scenario routing, QEMU scenario orchestration, and the
new script only. It does not touch shared syscall dispatch, descriptor-table
syscall behavior, read/stdin, user-copy helpers, lower-EL routing, or existing
diagnostic output owners, so the scalar/descriptor/read regression smoke
scripts from the plan were not rerun.

## Deferred Surfaces

Pi 5 hardware proof, boot archive publication, firmware/TFTP initramfs
delivery, descriptor-backed production filesystem syscalls, open syscall ABI,
directory iteration, seek syscalls, object final release, ELF/program loading,
executable /bin/init behavior, argv/envp setup, process creation,
exec/spawn/wait, shell behavior, writable filesystems, persistent storage,
networking, SSH, RP1/PCIe, UART interrupt ownership, and DMA/cache-driver
policy remain blocked.

## Evidence

- static inspection: git status --short before edits was clean.
- source/script diff: build.rs registered the new scenario; src/main.rs routes
  it; src/target/qemu_virt.rs emits the planned fixture/read/error lines;
  scripts/qemu-readonly-initramfs-vfs-smoke.sh builds, runs, greps, and
  retains the log.
- formatting: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed with 261 no_std tests.
- QEMU/substitute smoke: scripts/qemu-readonly-initramfs-vfs-smoke.sh passed.
- regression smoke justification: existing scalar, descriptor-write, close,
  dup, and read/stdin smoke scripts were not required because no shared
  syscall dispatch, descriptor table, read, user-copy, lower-EL routing,
  boot-scenario routing shared by those smokes, or diagnostic output owner was
  touched.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.

## Next Action

The next mechanically unblocked task is
phase8-readonly-initramfs-vfs-closeout-checkpoint-20260530, which should
reconcile the accepted contract, smoke plan, target-independent core, retained
QEMU/substitute evidence, deferred surfaces, and residual risks before any
loader or shell work.
