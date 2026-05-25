# Pi 5 Production Secondary Dispatch Proof Evidence

Task: phase6-pi5-production-secondary-dispatch-proof-20260525

Current classification: pi5-production-secondary-dispatch-complete

## Attempts

1. local1: archive SHA256
   70a601fcaf1580540a4055fef794ec1182327fac0e059b0b19075eae82476f50,
   kernel SHA256
   bf36772c529b16d1dbf81aa1575661942ab137a189fdedaae0e5394f4c8e924d,
   size 98,664 bytes. Cursor-valid serial showed Talos entry and completed
   production secondary dispatch reports for logical CPUs 1, 2, and 3.
   Classification: pi5-production-secondary-dispatch-complete.

## Hardware Artifacts

- local1 captured health, status, serial cursor, TFTP cursor, pre-run snapshot,
  publish result, post-publish status/files, power cycle, serial observation,
  TFTP delta, restore result, and post-restore status.
- local1 serial-key-lines.txt contains the accepted cursor-valid lines.
- local1 TFTP delta records da591740/kernel_2712.img fetches. The saved
  TFTP event bytes field reflects the restored tree size, so candidate size
  evidence comes from archive review and post-publish files.
- Accepted archive review passed with file_count=19, kernel_size=98664,
  header_image_size=98664, text_offset=0, flags=12, and
  loader_diagnostic=false.

## Validation Summary

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test: passed with 119 no_std tests.
- scripts/qemu-production-secondary-dispatch-smoke.sh: passed with
  classification=qemu-production-secondary-dispatch-complete.
- scripts/qemu-smoke.sh: passed.
- scripts/rpi5-image.sh: passed.
- scripts/rpi5-production-secondary-dispatch-boot-tree.sh: passed.
- scripts/rpi5-archive-review.sh
  target/talos-rpi5-production-secondary-dispatch-boot.tar.gz: passed.
- Serialized Pi 5 hardware run under hardwareTestLock: accepted on local1.
- Restore proof: local1 restore-exit.txt is 0.
- git diff --check: passed.

## Acceptance

Accepted as Pi 5 hardware evidence for CPU-local production secondary
dispatch. Shared run queues, global task lookup, remote enqueue queues, task
migration, multi-core preemption, userspace, filesystem, networking, SSH,
shell, RP1/PCIe, UART interrupt ownership, and DMA behavior remain deferred.
