# Phase 6 Pi 5 Remote Wake To Local Runnable Proof

Task ID: phase6-pi5-remote-wake-to-local-runnable-proof-20260525
Status: accepted

## Goal

Carry the accepted QEMU target-owned wake consumption and local runnable
transition proof to serialized Pi 5 hardware.

## Scope

- Added the focused Pi 5 diagnostic flag
  TALOS_RPI5_REMOTE_WAKE_TO_LOCAL_RUNNABLE_PROOF on top of the accepted
  remote wake-request proof path.
- Added focused Pi 5 image and boot-tree scripts for the hardware proof.
- Proved request publication, duplicate coalescing, SGI INTID 1 signaling,
  target-side observation/EOI, target-owned request consumption outside IPI
  context, local blocked-to-runnable transition, duplicate local enqueue
  rejection, queue drain, cross-owner rejection, and deferred production
  secondary dispatch.

## Evidence

Evidence directory:
tasks/evidence/2026-05-25-pi5-remote-wake-to-local-runnable-proof/.

- Static/code: Pi 5 remote wake-request diagnostic now has optional
  target-owned local runnable reporting gated by
  TALOS_RPI5_REMOTE_WAKE_TO_LOCAL_RUNNABLE_PROOF. The IPI hot path remains
  bounded to observation/accounting/EOI; local runnable mutation happens after
  the target drains its owned request queue.
- Unit tests: cargo -Zjson-target-spec test passed 116 no_std tests.
- QEMU/substitute retained gates: scripts/qemu-remote-wake-to-local-runnable-smoke.sh,
  scripts/qemu-remote-wakeup-request-smoke.sh, and scripts/qemu-smoke.sh passed.
- Image/archive inspection: scripts/rpi5-remote-wake-to-local-runnable-boot-tree.sh
  and scripts/rpi5-archive-review.sh target/talos-rpi5-remote-wake-to-local-runnable-boot.tar.gz
  passed. Archive SHA256:
  acf72b3b52416ac8e41178c7bf328d4f075981c5800f937cb016c9cecb8226b2.
  Kernel SHA256:
  01e04b23addf8876d58d0d6f332d9b8d923a9f814bcf2a72c68cfb5f421ffae6.
  Kernel size: 103,040 bytes.
- Hardware local1: captured firmware-only current serial after publish/power
  and restored the pre-run snapshot. This was classified as early serial
  capture only, not an accepted proof.
- Hardware local2: cursor-valid serial showed Talos entry, request
  publication for targets 1/2/3, duplicate request coalescing for target 1,
  SGI sends, target-side reports with receive-count=1, eoi-count=1,
  pending-count=1, consumed tasks 201/202/203, queue length 0, cross-owner
  rejection true, production deferred true, plus local Blocked -> Runnable
  transitions for each target with duplicate-local-rejected=true and ok=true.
  The final line reported participants=3 expected=3 errors=0
  ready-mask=0xe complete-mask=0xe
  classification=pi5-remote-wake-to-local-runnable-complete and PASS.
- Restore proof: local1 and local2 restore-exit.txt are 0.

## Current Classification

Accepted. The Pi 5 evidence proves target-owned remote wake consumption into a
target-local runnable queue for diagnostic tasks while preserving CPU-local
scheduler ownership and deferring production secondary dispatch.

## Acceptance

Accepted as the serialized Pi 5 target-owned remote wake to local runnable
proof. This does not introduce shared run queues, global task lookup, remote
enqueue queues, task migration, load balancing, work stealing, production
secondary scheduler dispatch, multi-core preemption, Phase 7, filesystem,
networking, SSH, shell, RP1/PCIe, UART interrupt, or DMA behavior.
