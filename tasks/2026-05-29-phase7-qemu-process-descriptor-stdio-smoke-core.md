# Phase 7 QEMU Process Descriptor Stdio Smoke Core

Task: phase7-qemu-process-descriptor-stdio-smoke-core-20260529
Status: accepted

## Scope

This implementation task added the QEMU/substitute smoke proving that
lower-AArch64 talos_write fd 1 and fd 2 route through a ProcessOwnerId-backed
process-owned inherited stdio table. It did not acquire hardwareTestLock, run
Pi 5 hardware, publish a boot archive, implement stdin/read, close, dup,
process loading, VFS/filesystem behavior, shell behavior, networking, SSH,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, or a full POSIX
descriptor claim.

## Implementation Notes

- build.rs adds the qemu_process_descriptor_stdio_smoke boot scenario,
  implying the accepted descriptor-write lower-EL payload while selecting a
  process-owned stdio harness.
- src/main.rs routes that scenario to
  target::qemu_virt::run_process_descriptor_stdio_smoke().
- src/arch/aarch64/exceptions.rs routes the new scenario to the process-owned
  descriptor stdio exception handler before the earlier proof-owned
  descriptor-write handler.
- src/target/qemu_virt.rs creates ProcessOwnerId 1 in a
  ProcessDescriptorStore, installs inherited stdio, resolves the current owner,
  and dispatches talos_write using the resolved process-owned table.
- scripts/qemu-process-descriptor-stdio-smoke.sh builds the scenario, runs
  QEMU, greps the accepted process-owned lookup/output lines, and retains the
  serial log.

## Evidence

- Accepted plan commit:
  b314ab881f82a07da32bd1db88786a4dbf6d471e.
- Retained QEMU/substitute smoke log:
  tasks/evidence/2026-05-29-qemu-process-descriptor-stdio-smoke-core/qemu-process-descriptor-stdio-smoke.log.
- Focused QEMU/substitute evidence:
  scripts/qemu-process-descriptor-stdio-smoke.sh passed and retained
  classification=qemu-process-descriptor-stdio-smoke-complete plus
  qemu-process-descriptor-stdio-smoke: PASS.
- Regression evidence:
  scripts/qemu-descriptor-write-smoke.sh, scripts/qemu-syscall-smoke.sh, and
  scripts/qemu-pointer-copy-smoke.sh passed.
- Unit tests:
  cargo -Zjson-target-spec test passed with 222 no_std tests.
- Formatting:
  cargo fmt --all -- --check passed.
- Static inspection:
  git diff --check passed.
- Documentation:
  mdbook build passed.

## Deferred Work

Pi 5 physical proof, stdin/read behavior, close/dup/read syscalls, PID
allocation, fork/spawn/exec, process loading, VFS/filesystem, shell,
networking, SSH, RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy,
and full POSIX descriptor claims remain blocked.

## Next Task

The next bounded task is
phase7-process-descriptor-table-closeout-checkpoint-20260529, scoped to
closing out the accepted process-owned descriptor-table contract/core/QEMU
evidence before any broader descriptor-table work.
