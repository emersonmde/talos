# Phase 7 QEMU Descriptor-Write Smoke Core

Task: phase7-qemu-descriptor-write-smoke-core-20260529
Status: accepted
Date: 2026-05-29

## Scope

This implementation task added only the QEMU/substitute descriptor-write smoke
for the accepted talos_write fd 1/fd 2 runtime-console0 slice. It wires the
lower-AArch64 svc #0 scenario through the saved-frame syscall path, inherited
stdio descriptors, copy_from_user(), and runtime-console0 capture.

It did not add Pi 5 hardware proof, hardwareTestLock acquisition, boot archive
publication, stdin/read, close, dup, process loading, VFS/filesystem behavior,
path copying, shell behavior, networking, SSH, RP1/PCIe work, UART interrupt
ownership, DMA/cache-driver policy, live process-owned descriptor tables, or a
phase transition.

## Changed Files

- build.rs: registered TALOS_BOOT_SCENARIO=qemu_descriptor_write_smoke and
  reused the recoverable lower-EL syscall assembly path.
- src/main.rs: routed the new QEMU boot scenario and excluded it from the
  default QEMU timer smoke fallback.
- src/arch/aarch64/exceptions.rs: allowed the descriptor-write smoke to use
  the recoverable QEMU syscall exception handler shape.
- src/target/qemu_virt.rs: added the descriptor-write payload, UserData backing
  storage, runtime-console0 capture, fd success/error observations,
  talos_nop/unknown regressions, copy-probe quarantine, and final
  classification/PASS output.
- scripts/qemu-descriptor-write-smoke.sh: added the focused QEMU gate and
  retained log copy.
- docs/src/roadmap.md and docs/src/decisions/README.md: recorded the accepted
  QEMU/substitute descriptor-write evidence and deferred surfaces.

## Accepted Evidence

- static inspection: git status --short before edits showed a pre-existing
  docs/src/roadmap.md working-tree edit that was preserved.
- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed.
- QEMU/substitute smoke: scripts/qemu-descriptor-write-smoke.sh passed.
- retained QEMU evidence:
  tasks/evidence/2026-05-29-qemu-descriptor-write-smoke-core/qemu-descriptor-write-smoke.log.
- retained classification lines:
  qemu-descriptor-write-smoke: final participants=8 expected=8 errors=0
  classification=qemu-descriptor-write-smoke-complete; qemu-descriptor-write-smoke:
  PASS.
- QEMU/substitute regression: scripts/qemu-syscall-smoke.sh passed.
- QEMU/substitute regression: scripts/qemu-pointer-copy-smoke.sh passed.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Accepted Capability

The accepted capability is only QEMU/substitute evidence that lower-EL stable
svc #0 can route talos_write x8 = 1 through the descriptor-write syscall core.
The retained log proves fd 1 and fd 2 write 18-byte UserData buffers through
DescriptorTable::with_inherited_stdio(), copy_from_user(), and
runtime-console0, while fd 0 and fd 99 return -EBADF, the guard range returns
-EFAULT, nonzero reserved x3 returns -EINVAL, talos_nop and unknown-syscall
behavior remains intact, x8 = 0x7001 remains -ENOSYS outside proof scenarios,
and diagnostic marker 0x7a10 remains outside stable syscall dispatch.

## Deferred Work

Pi 5 descriptor-write hardware proof, stdin/read, close, dup, process loading,
process-owned descriptor tables, process-owned address spaces, VFS/filesystem
behavior, path copying, argv/envp loading, shell behavior, networking, SSH,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, blocking/readiness,
signals, restart semantics, and full POSIX descriptor claims remain blocked
until later explicit tasks.

## Next Action

phase7-descriptor-write-closeout-checkpoint-20260529 is mechanically unblocked
for the next worker wake if durable state and the working tree remain
compatible. That task should reconcile the accepted descriptor syscall
contract, smoke plan, core implementation, retained QEMU evidence, regression
gates, and deferred surfaces before any Pi 5 descriptor-write proof planning.
