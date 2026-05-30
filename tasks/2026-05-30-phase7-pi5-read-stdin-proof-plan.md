# Phase 7 Pi 5 Read And Stdin Proof Plan

Task: phase7-pi5-read-stdin-proof-plan-20260530
Status: accepted

## Scope

This documentation-only task defined the serialized Raspberry Pi 5 proof plan
for the accepted fixed-stdin talos_read invariant. It did not change Rust or
assembly behavior, run QEMU, run Pi 5 hardware, publish a boot archive,
power-cycle hardware, observe serial output, acquire hardwareTestLock, attach
fd 0 to runtime-console0/TTY/hardware stdin, add process loading,
VFS/filesystem behavior, shell behavior, networking, SSH, object finalization,
RP1/PCIe, UART interrupt ownership, DMA/cache-driver policy, or a full POSIX
descriptor claim.

## Evidence

- Plan document:
  docs/src/project/phase7-pi5-read-stdin-proof-plan.md.
- Source QEMU/substitute evidence:
  tasks/evidence/2026-05-29-qemu-read-stdin-smoke-core/qemu-read-stdin-smoke.log.
- Accepted QEMU classification:
  classification=qemu-read-stdin-smoke-complete; qemu-read-stdin-smoke: PASS.
- The plan requires the later Pi 5 proof to retain candidate identity,
  archive/kernel digests, fresh serial cursor, fresh TFTP delta/fetch,
  hardwareTestLock acquire/release, restore proof, exact fd 0/fd 3/error/EOF
  observations, scalar regressions, copy-probe quarantine, diagnostic-marker
  quarantine, classification, and PASS evidence.

## Validation

- static inspection: git status --short before edits was clean.
- static documentation/source inspection: reviewed accepted read/stdin docs,
  retained QEMU read/stdin evidence, and previous Pi 5 close/dup proof-plan
  structure.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.
- staged whitespace inspection: git diff --cached --check passed before
  commit.
- Rust fmt/tests, QEMU runs, Pi 5 hardware runs, archive publication, and
  hardwareTestLock acquisition were not required for this documentation-only
  plan.

## Deferred Work

Pi 5 physical read proof, runtime-console0/TTY/hardware stdin, process
loading, VFS/filesystem, shell, networking, SSH, object finalization,
dup2/fcntl, signals, wait queues, nonblocking I/O, RP1/PCIe, UART interrupt
ownership, DMA/cache-driver policy, and full POSIX descriptor readiness remain
blocked.

## Next Task

The next bounded task is phase7-pi5-read-stdin-proof-20260530, scoped to
implementing and running only the serialized physical proof described by the
accepted plan, after confirming hardwareTestLock is unlocked/restored.
