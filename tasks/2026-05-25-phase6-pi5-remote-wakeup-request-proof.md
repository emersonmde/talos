# Phase 6 Pi 5 Remote Wake-Request Proof

Task ID: phase6-pi5-remote-wakeup-request-proof-20260525
Status: accepted

## Goal

Carry the accepted bounded remote wake-request model from QEMU substitute
evidence to serialized Raspberry Pi 5 hardware evidence.

## Scope

- Use the existing bounded RemoteWakeQueue model and SGI INTID 1 path.
- Prove CPU 0 request publication for logical CPUs 1, 2, and 3, duplicate
  coalescing, SGI signaling, target-side IPI observation/EOI, target-owned
  request consumption, drained queue state, and cross-owner runnable-queue
  mutation rejection.
- Preserve CPU 0 as the only production scheduler owner; secondary CPUs are
  diagnostic request consumers only.

## Evidence

Evidence directory:
tasks/evidence/2026-05-25-pi5-remote-wakeup-request-proof/.

- Static/code: added Pi 5 remote wake-request diagnostic wiring behind
  TALOS_RPI5_REMOTE_WAKEUP_REQUEST_PROOF, focused image/boot-tree scripts,
  Pi 5 IRQ dispatch handling for the remote-wakeup proof, and SMP cache
  invalidation cfg coverage for the new proof.
- Unit tests: cargo -Zjson-target-spec test passed 113 no_std tests with
  QEMU 9.2.0 on PATH.
- QEMU/substitute retained gates: scripts/qemu-remote-wakeup-request-smoke.sh,
  scripts/qemu-cross-core-ipi-delivery-smoke.sh, and scripts/qemu-smoke.sh
  passed.
- Image/archive inspection: scripts/rpi5-remote-wakeup-request-boot-tree.sh
  and scripts/rpi5-archive-review.sh
  target/talos-rpi5-remote-wakeup-request-boot.tar.gz passed.
- Hardware run remote1: archive SHA256
  798e745264b690e7557809d15287c48b19bac26961f672a15a34bdfaf24efd81, kernel
  SHA256 83acdae42737e19f337e5d8f9364a38216ab0540cd3db3dfb92b3e23c8061fee,
  size 103,040 bytes. TFTP fetched the candidate, but the cursor-valid serial
  observe only returned NUL/newline bytes. Classified
  pi5-remote-wakeup-request-candidate-fetched-current-serial-not-proven and
  restored the pre-run boot tree before another attempt.
- Hardware run remote2: the same archive and kernel were served. Cursor-valid
  retained serial showed Talos entry, remote wake start, request publication,
  duplicate coalescing, SGI sends to target-list bits 0x02, 0x04, and 0x08,
  receivers 1/2/3 each at receive-count=1 eoi-count=1 pending-count=1,
  consumed tasks 201/202/203, queue length 0, cross-owner rejection true,
  production deferred true, final participants=3 expected=3 errors=0
  ready-mask=0xe complete-mask=0xe
  classification=pi5-remote-wakeup-request-complete, and PASS.
- Restore proof: both hardware iterations restored their pre-run snapshots;
  each restore-exit.txt is 0.

## Current Classification

Accepted. The Pi 5 evidence proves candidate fetch, Talos entry,
secondary-core readiness, cacheable-MMU handoff, request publication, duplicate
coalescing, SGI INTID 1 delivery/EOI, target-owned consumption, drained queue
state, rejected cross-owner local scheduler mutation, deferred secondary
production dispatch, PASS classification, and restore.

## Acceptance

Accepted as the serialized Pi 5 scheduler-facing remote wake-request proof.
This does not introduce local runnable transitions from remote requests, shared
run queues, task migration, production secondary scheduler dispatch, multi-core
preemption, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, or DMA
behavior.
