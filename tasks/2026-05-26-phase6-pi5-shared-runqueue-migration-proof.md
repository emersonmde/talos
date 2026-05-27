# Phase 6 Pi 5 Shared Run-Queue Migration Proof

Task ID: phase6-pi5-shared-runqueue-migration-proof-20260526
Status: accepted

## Goal

Run the serialized Pi 5 physical proof for the accepted shared
run-queue/migration invariant after QEMU evidence is accepted.

## Scope

- Added the focused Pi 5 proof flag
  TALOS_RPI5_SHARED_RUNQUEUE_MIGRATION_PROOF.
- Added focused Pi 5 image and boot-tree scripts for the hardware proof.
- Proved the implemented SharedRunQueue::publish_migration and
  SharedRunQueue::consume_for_destination path on physical Pi 5 cores.
- Preserved the diagnostic boundary: no load balancing, work stealing,
  multi-core timer preemption, userspace, filesystem, networking, SSH, shell,
  RP1/PCIe, UART interrupt, or DMA/cache-driver behavior.

## Evidence

Evidence directory:
tasks/evidence/2026-05-26-pi5-shared-runqueue-migration-proof/.

- Static/code: build.rs, src/main.rs, src/boot/rpi5.rs, src/target/rpi5.rs,
  src/smp.rs, and scripts/ add a focused Pi 5 shared run-queue/migration proof
  path and staging helpers.
- Image/archive inspection:
  scripts/rpi5-shared-runqueue-migration-boot-tree.sh and
  scripts/rpi5-archive-review.sh
  target/talos-rpi5-shared-runqueue-migration-boot.tar.gz passed.
  Archive SHA256:
  4d5c8e2666d64ddcc5df7b49c8d3a541b01634800917616cbdb88404a54630d5.
  Kernel SHA256:
  98a9cb87bcb89c38b19a097a05695a136aaf6b0eb911ec03c3b0c17eeab6a394.
  Kernel size: 102,952 bytes.
- Hardware local1: cursor-valid serial reached
  classification=pi5-shared-runqueue-migration-complete and PASS.
- Hardware local1 final line reported participants=4 expected=4 errors=0
  lock-available=true, proving all four physical-core participants completed
  the named invariant.
- TFTP proof: local1 TFTP delta records da591740/kernel_2712.img fetches from
  10.42.1.4 with bytes=102952 before restore.
- Restore proof: local1 restore-exit.txt is 0 and post-restore status/serial
  were captured after restoring the pre-run snapshot.

## Validation

- git status --short before edits: clean.
- fmt/lint: cargo fmt --all -- --check passed.
- unit tests: cargo -Zjson-target-spec test passed, 142 no_std tests.
- QEMU/substitute baseline: scripts/qemu-smoke.sh passed with
  talos: qemu smoke PASS.
- focused QEMU/substitute: scripts/qemu-shared-runqueue-migration-smoke.sh
  passed with classification qemu-shared-runqueue-migration-complete.
- image/archive inspection:
  scripts/rpi5-archive-review.sh
  target/talos-rpi5-shared-runqueue-migration-boot.tar.gz passed.
- serial hardware boot/output:
  classification=pi5-shared-runqueue-migration-complete.
- whitespace inspection: git diff --check passed.
- documentation: mdbook build passed.

## Result

Accepted. The Pi 5 evidence proves the accepted shared run-queue/migration
invariant on physical cores, including candidate identity, TFTP fetch, boot
execution, scheduler behavior through the implemented shared queue core,
classification, PASS output, and restore evidence.
