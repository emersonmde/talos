# Pi 5 Remote Wake To Local Runnable Proof Evidence

Task: phase6-pi5-remote-wake-to-local-runnable-proof-20260525

Current classification: pi5-remote-wake-to-local-runnable-complete

## Attempts

1. local1: archive SHA256
   acf72b3b52416ac8e41178c7bf328d4f075981c5800f937cb016c9cecb8226b2,
   kernel SHA256
   01e04b23addf8876d58d0d6f332d9b8d923a9f814bcf2a72c68cfb5f421ffae6,
   size 103,040 bytes. The current serial observe captured only early
   firmware output before the pre-run snapshot was restored. Classification:
   pi5-remote-wake-to-local-runnable-early-firmware-only.

2. local2: same archive and kernel. Cursor-valid serial showed Talos entry,
   remote wake start, CPU 0 publication for logical CPUs 1, 2, and 3,
   duplicate target-1 coalescing, SGI INTID 1 sends, per-target
   observe/EOI/consume reports, drained queues, rejected cross-owner
   mutation, deferred secondary production dispatch, local Blocked -> Runnable
   transitions for tasks 201, 202, and 203, duplicate local enqueue rejection,
   and classification=pi5-remote-wake-to-local-runnable-complete with PASS.

## Hardware Artifacts

- local1 and local2 captured health, status, serial cursor, TFTP cursor,
  pre-run snapshot, publish result, post-publish status/files, power cycle,
  serial observation, restore result, and post-restore status.
- local2 serial-key-lines.txt contains the accepted cursor-valid lines.
- local2 TFTP delta shows the Pi fetching da591740/kernel_2712.img and
  firmware support files from 10.42.1.3.
- Accepted archive review passed with file_count=19, kernel_size=103040,
  header_image_size=103040, text_offset=0, flags=12, and
  loader_diagnostic=false.

## Validation Summary

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test: passed with 116 no_std tests.
- scripts/qemu-remote-wake-to-local-runnable-smoke.sh: passed with
  classification=qemu-remote-wake-to-local-runnable-complete.
- scripts/qemu-remote-wakeup-request-smoke.sh: passed with
  classification=qemu-remote-wakeup-request-complete.
- scripts/qemu-smoke.sh: passed.
- scripts/rpi5-image.sh: passed.
- scripts/rpi5-remote-wake-to-local-runnable-boot-tree.sh: passed.
- scripts/rpi5-archive-review.sh
  target/talos-rpi5-remote-wake-to-local-runnable-boot.tar.gz: passed.
- Serialized Pi 5 hardware run under hardwareTestLock: accepted on local2.
- Restore proof: local1/restore-exit.txt and local2/restore-exit.txt are 0.
- git diff --check: passed.

## Acceptance

Accepted as Pi 5 hardware evidence for target-owned remote wake consumption
into a target-local runnable queue. Shared run queues, global task lookup,
remote enqueue queues, task migration, production secondary scheduler
dispatch, multi-core preemption, userspace, filesystem, networking, SSH,
shell, RP1/PCIe, UART interrupt ownership, and DMA behavior remain deferred.
