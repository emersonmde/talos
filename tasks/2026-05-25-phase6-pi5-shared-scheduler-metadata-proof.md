# Phase 6 Pi 5 Shared Scheduler Metadata Proof

Task ID: phase6-pi5-shared-scheduler-metadata-proof-20260525
Status: accepted

## Goal

Carry the accepted shared scheduler metadata invariant to serialized Pi 5
hardware evidence.

## Scope

- Added the focused Pi 5 diagnostic flag
  TALOS_RPI5_SHARED_SCHEDULER_METADATA_PROOF.
- Added focused Pi 5 image and boot-tree scripts for the hardware proof.
- Proved logical CPUs 0 through 3 publishing/querying the shared metadata
  boundary without cross-owner local scheduler mutation or migration.

## Evidence

Evidence directory:
tasks/evidence/2026-05-25-pi5-shared-scheduler-metadata-proof/.

- Static/code: Pi 5 secondary entry now has a focused shared scheduler
  metadata proof path using the accepted cacheable-MMU secondary handoff before
  shared metadata access. The path stays CPU-local and does not add shared run
  queues, migration, load balancing, or preemption.
- Unit tests: cargo -Zjson-target-spec test passed 125 no_std tests.
- Retained QEMU gates: scripts/qemu-smoke.sh,
  scripts/qemu-shared-scheduler-metadata-smoke.sh,
  scripts/qemu-production-secondary-dispatch-smoke.sh, and
  scripts/qemu-remote-wake-to-local-runnable-smoke.sh passed.
- Image/archive inspection:
  scripts/rpi5-shared-scheduler-metadata-boot-tree.sh and
  scripts/rpi5-archive-review.sh
  target/talos-rpi5-shared-scheduler-metadata-boot.tar.gz passed. Archive
  SHA256: 7ec358f5809aee223364948fa20ba9b4e73f8fd76a1ac0238081926568f74bf0.
  Kernel SHA256:
  232cab18a49eb75ddc1969438d45ab1874359492028dfea81522f22507d24382.
  Kernel size: 99,136 bytes.
- Hardware local1: cursor-valid serial showed Talos entry, the
  rpi5-shared-scheduler-metadata start line, secondary cacheable-MMU handoff
  plan, PSCI CPU_ON for logical CPUs 1/2/3, reports for logical CPUs 0/1/2/3,
  task IDs 101/201/301/401, lookup success, boot-task lookup success,
  rejected cross-owner scheduler mutation, rejected cross-owner metadata
  publication, preserved local runnable queues, errors=0, and ok=true.
- Hardware local1 final line reported participants=4 expected=4 errors=0
  state-lock-available=true metadata-lock-available=true final-metadata-len=4
  final-metadata-generation=4 and
  classification=pi5-shared-scheduler-metadata-complete; PASS followed.
- TFTP proof: local1 TFTP delta records da591740/kernel_2712.img fetches with
  bytes=99,136 from 10.42.1.4.
- Restore proof: local1 restore-exit.txt is 0 and post-restore status/serial
  were captured after restoring the pre-run snapshot.

## Current Classification

Accepted. The Pi 5 evidence proves the first shared scheduler metadata
invariant on physical hardware.

## Acceptance

Accepted as serialized Pi 5 hardware evidence for shared scheduler metadata.
This does not introduce shared run queues, remote enqueue queues, task
migration, load balancing, work stealing, multi-core preemption, Phase 7,
filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt ownership, or DMA
behavior.
