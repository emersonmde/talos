# Pi 5 Shared Scheduler Metadata Proof Evidence

Task: phase6-pi5-shared-scheduler-metadata-proof-20260525

Current classification: pi5-shared-scheduler-metadata-complete

## Attempts

1. local1: archive SHA256
   7ec358f5809aee223364948fa20ba9b4e73f8fd76a1ac0238081926568f74bf0,
   kernel SHA256
   232cab18a49eb75ddc1969438d45ab1874359492028dfea81522f22507d24382,
   size 99,136 bytes. Cursor-valid serial showed Talos entry and completed
   shared scheduler metadata reports for logical CPUs 0, 1, 2, and 3.
   Classification: pi5-shared-scheduler-metadata-complete.

## Hardware Artifacts

- local1 captured health, status, serial cursor, TFTP cursor, pre-run snapshot,
  publish result, post-publish status/files, power cycle, serial observations,
  TFTP delta, restore result, post-restore status, and post-restore serial
  peek.
- local1 serial-key-lines.txt contains the accepted cursor-valid lines; the
  full logical 0 report is present in serial-combined.txt with interleaved
  secondary UART text before the line prefix.
- local1 TFTP delta records repeated da591740/kernel_2712.img fetches with
  bytes=99136 from 10.42.1.4.
- Accepted archive review passed with file_count=19, kernel_size=99136,
  header_image_size=99136, text_offset=0, flags=12, and
  loader_diagnostic=false.

## Validation Summary

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test: passed with 125 no_std tests.
- scripts/qemu-smoke.sh: passed.
- scripts/qemu-shared-scheduler-metadata-smoke.sh: passed with
  classification=qemu-shared-scheduler-metadata-complete.
- scripts/qemu-production-secondary-dispatch-smoke.sh: passed with
  classification=qemu-production-secondary-dispatch-complete.
- scripts/qemu-remote-wake-to-local-runnable-smoke.sh: passed with
  classification=qemu-remote-wake-to-local-runnable-complete.
- scripts/rpi5-shared-scheduler-metadata-image.sh: passed.
- scripts/rpi5-shared-scheduler-metadata-boot-tree.sh: passed.
- scripts/rpi5-archive-review.sh
  target/talos-rpi5-shared-scheduler-metadata-boot.tar.gz: passed.
- Serialized Pi 5 hardware run under hardwareTestLock: accepted on local1.
- Restore proof: local1 restore-exit.txt is 0.
- git diff --check: passed.
- mdbook build: passed.

## Acceptance

Accepted as Pi 5 hardware evidence for shared scheduler metadata. Shared run
queues, remote enqueue queues, task migration, multi-core preemption,
userspace, filesystem, networking, SSH, shell, RP1/PCIe, UART interrupt
ownership, and DMA behavior remain deferred.
