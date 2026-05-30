# Phase 7 Pi 5 Read And Stdin Proof

Task: phase7-pi5-read-stdin-proof-20260530
Status: in progress

## Scope

This task carries the accepted fixed-stdin talos_read invariant to serialized
Raspberry Pi 5 hardware evidence. The implementation adds only the focused
rpi5_read_stdin_proof boot scenario, image and boot-tree helpers, fixed
ProcessDescriptorStore-backed stdin proof state, and retained lab evidence for
the physical proof.

It does not add runtime-console0/TTY/hardware stdin, pipes, sockets, regular
files, filesystem reads, process loading, shell behavior, networking, SSH,
object finalization, RP1/PCIe work, UART interrupt ownership,
DMA/cache-driver policy, or full POSIX descriptor readiness.

## Implementation Evidence

- fmt/lint: cargo fmt --all -- --check passed before hardware.
- unit tests: cargo -Zjson-target-spec test passed before hardware.
- QEMU/substitute: scripts/qemu-read-stdin-smoke.sh passed before hardware.
- QEMU/substitute regressions: scripts/qemu-syscall-smoke.sh,
  scripts/qemu-descriptor-write-smoke.sh, scripts/qemu-close-syscall-smoke.sh,
  and scripts/qemu-dup-syscall-smoke.sh passed before hardware.

## Hardware Evidence

Evidence directory:
tasks/evidence/2026-05-30-pi5-read-stdin-proof/.

Pending serialized Pi 5 run under hardwareTestLock.

## Result

In progress.
