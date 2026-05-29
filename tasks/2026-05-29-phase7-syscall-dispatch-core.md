# Phase 7 Syscall Dispatch Core

Task: phase7-syscall-dispatch-core-20260529
Status: accepted

## Scope

This implementation task added only the target-independent Phase 7.3 syscall
dispatch core required by the accepted syscall ABI contract. It did not route
exception vectors, change assembly, add a boot scenario, enter EL0, copy user
memory, mutate descriptor tables through syscall entry, load programs, expose
VFS/filesystem behavior, create shell behavior, touch networking or SSH, run
QEMU, publish a Pi 5 archive, acquire hardwareTestLock, or change RP1/PCIe,
UART interrupt ownership, or DMA/cache-driver policy.

## Accepted Core

- Stable syscall SVC immediate remains svc #0.
- Diagnostic SVC marker 0x7a10 is represented only as proof vocabulary and is
  rejected as a stable syscall immediate.
- x8 syscall number vocabulary contains talos_nop = 0 only.
- x0 through x5 are represented as a scalar argument view for the pure
  dispatch layer.
- talos_nop returns x0 = 0.
- Unknown syscall numbers return x0 = -ENOSYS encoded as two's-complement
  u64.
- The accepted errno subset is encoded for EINVAL=22, EBADF=9, EFAULT=14,
  ENOSYS=38, and ENOTSUP=95.

## Evidence

- static inspection: git status --short before edits was clean.
- unit tests: cargo -Zjson-target-spec test passed.
- formatting: cargo fmt --all -- --check passed.
- static inspection: git diff --check passed.
- documentation: mdbook build passed.

## Deferred Work

Production exception-handler integration, QEMU syscall smoke, Pi 5 hardware
proof, pointer-taking syscalls, byte copy-in/copy-out, descriptor I/O, process
loading, VFS/filesystem, shell behavior, networking, SSH, RP1/PCIe, UART
interrupt ownership, and DMA/cache-driver policy remain blocked until later
explicit tasks.
