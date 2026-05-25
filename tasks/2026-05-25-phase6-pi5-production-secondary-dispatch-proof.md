# Phase 6 Pi 5 Production Secondary Dispatch Proof

Task ID: phase6-pi5-production-secondary-dispatch-proof-20260525
Status: accepted

## Goal

Prove the accepted QEMU production secondary dispatch slice on serialized Pi 5
hardware.

## Scope

- Added the focused Pi 5 diagnostic flag
  TALOS_RPI5_PRODUCTION_SECONDARY_DISPATCH_PROOF.
- Added focused Pi 5 image and boot-tree scripts for the hardware proof.
- Proved logical CPUs 1, 2, and 3 entering
  SecondaryProductionDiagnostic, dispatching CPU-local diagnostic kernel
  threads, publishing stable local scheduler snapshots, and rejecting
  cross-owner local queue and dispatch attempts.

## Evidence

Evidence directory:
tasks/evidence/2026-05-25-pi5-production-secondary-dispatch-proof/.

- Static/code: Pi 5 secondary entry now has a focused production-secondary
  proof path using the same PerCoreScheduler production diagnostic role and
  dispatch invariants accepted by the QEMU smoke. The path stays CPU-local and
  does not add shared run queues, migration, load balancing, or preemption.
- Unit tests: cargo -Zjson-target-spec test passed 119 no_std tests.
- QEMU/substitute retained gates: scripts/qemu-production-secondary-dispatch-smoke.sh
  and scripts/qemu-smoke.sh passed.
- Image/archive inspection: scripts/rpi5-production-secondary-dispatch-boot-tree.sh
  and scripts/rpi5-archive-review.sh
  target/talos-rpi5-production-secondary-dispatch-boot.tar.gz passed. Archive
  SHA256: 70a601fcaf1580540a4055fef794ec1182327fac0e059b0b19075eae82476f50.
  Kernel SHA256:
  bf36772c529b16d1dbf81aa1575661942ab137a189fdedaae0e5394f4c8e924d.
  Kernel size: 98,664 bytes.
- Hardware local1: cursor-valid serial showed Talos entry, the
  rpi5-production-secondary-dispatch start line, secondary cacheable-MMU
  handoff plan, PSCI CPU_ON for logical CPUs 1/2/3, per-core reports with
  role=secondary-production-diagnostic, production=true, progress 3,
  transition count 6, production dispatch count 3, context switch count 3,
  empty local queue, cross-owner rejection true, cross-owner dispatch rejection
  true, and ok=true.
- Hardware local1 final line reported participants=3 expected=3 errors=0
  lock-available=true and
  classification=pi5-production-secondary-dispatch-complete; PASS followed.
- TFTP proof: local1 TFTP delta records da591740/kernel_2712.img fetches.
  The post-publish boot-file listing records candidate kernel_2712.img and
  serial-prefixed mirror size 98,664 bytes. The TFTP endpoint's bytes field
  in the saved delta reflects the restored tree size, so the size claim is
  grounded in post-publish files and archive review rather than that field.
- Restore proof: local1 restore-exit.txt is 0 and post-restore status was
  captured after restoring the pre-run snapshot.

## Current Classification

Accepted. The Pi 5 evidence proves the first production secondary dispatch
slice on physical hardware for CPU-local diagnostic kernel threads.

## Acceptance

Accepted as serialized Pi 5 hardware evidence for production secondary
dispatch. This does not introduce shared run queues, global task lookup, remote
enqueue queues, task migration, load balancing, work stealing, multi-core
preemption, Phase 7, filesystem, networking, SSH, shell, RP1/PCIe, UART
interrupt ownership, or DMA behavior.
