# Phase 7 QEMU Close Syscall Smoke Core

Task: phase7-qemu-close-syscall-smoke-core-20260529
Status: accepted

## Scope

This implementation task added the QEMU/substitute smoke proving lower-AArch64
talos_close behavior through the current ProcessOwnerId-backed
ProcessDescriptorStore. It did not acquire hardwareTestLock, run Pi 5 hardware,
publish a boot archive, implement dup/read, process loading, VFS/filesystem
behavior, shell behavior, networking, SSH, RP1/PCIe, UART interrupt ownership,
DMA/cache-driver policy, or a full POSIX descriptor claim.

## Changed Files

- build.rs: registers qemu_close_syscall_smoke and reuses the accepted
  recoverable lower-EL syscall vector path.
- src/main.rs: routes qemu_close_syscall_smoke before the generic
  descriptor-write scenario.
- src/arch/aarch64/exceptions.rs: routes the close smoke to its handler before
  the earlier process-descriptor and descriptor-write handlers.
- src/target/qemu_virt.rs: adds the close payload, current-owner
  ProcessDescriptorStore harness, close/write observations, regression
  observations, and final classification/PASS output.
- scripts/qemu-close-syscall-smoke.sh: builds/runs the focused QEMU scenario,
  greps the accepted lines, and retains the serial log.
- docs/src/roadmap.md and docs/src/decisions/README.md: record the accepted
  QEMU/substitute evidence and deferred surfaces.

## Accepted Evidence

- Retained QEMU/substitute smoke log:
  tasks/evidence/2026-05-29-qemu-close-syscall-smoke-core/qemu-close-syscall-smoke.log.
- Focused QEMU/substitute evidence: scripts/qemu-close-syscall-smoke.sh passed
  and retained classification=qemu-close-syscall-smoke-complete plus
  qemu-close-syscall-smoke: PASS.
- Unit tests: cargo -Zjson-target-spec test passed with 231 no_std tests.
- Formatting: cargo fmt --all -- --check passed.
- QEMU/substitute regressions: scripts/qemu-descriptor-write-smoke.sh and
  scripts/qemu-syscall-smoke.sh passed.
- Static inspection: git diff --check passed.
- Documentation: mdbook build passed.

## Accepted Capability

The accepted capability is QEMU/substitute evidence only. The retained log
proves talos_close x8 = 2 closes fd 1 and fd 2 through
dispatch_process_descriptor() and
ProcessDescriptorStore::close_current_descriptor(); later talos_write on closed
descriptors returns -EBADF without runtime-console0 side effects; fd 2 remains
usable after fd 1 closes and after a failed reserved-register close; repeated
close and badfd close return -EBADF; talos_nop and unknown syscall behavior
remain intact; x8 = 0x7001 remains -ENOSYS; and diagnostic marker 0x7a10
remains outside stable syscall dispatch.

## Deferred Work

Pi 5 physical close proof, dup/read syscalls, process loading, VFS/filesystem,
stdin/read object model, shell, networking, SSH, object finalization,
blocking/readiness, signals, restart semantics, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, and full POSIX descriptor readiness remain
blocked.

## Next Task

phase7-close-syscall-closeout-checkpoint-20260529 is mechanically unblocked for
the next worker wake if durable state and the working tree remain compatible.
