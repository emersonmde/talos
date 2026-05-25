# Pi 5 Remote Wake-Request Proof Evidence

Task: phase6-pi5-remote-wakeup-request-proof-20260525

Current classification: pi5-remote-wakeup-request-complete

## Attempts

1. remote1: archive SHA256
   798e745264b690e7557809d15287c48b19bac26961f672a15a34bdfaf24efd81, kernel
   SHA256 83acdae42737e19f337e5d8f9364a38216ab0540cd3db3dfb92b3e23c8061fee,
   size 103,040 bytes. TFTP served the candidate, but cursor-valid serial only
   contained NUL/newline bytes and retained serial did not include the
   remote-wakeup marker. Classification:
   pi5-remote-wakeup-request-candidate-fetched-current-serial-not-proven.

2. remote2: same archive and kernel. Cursor-valid retained serial showed
   rpi5-remote-wakeup-request: start, CPU 0 publication for logical CPUs 1,
   2, and 3, duplicate target-1 coalescing, SGI INTID 1 sends, per-target
   observe/EOI/consume reports, drained queues, rejected cross-owner mutation,
   deferred secondary production dispatch, and
   classification=pi5-remote-wakeup-request-complete with PASS.

## Hardware Artifacts

- remote1 and remote2 captured health, status, serial cursor, TFTP cursor,
  pre-run snapshot, publish result, post-publish status/files, power cycle,
  TFTP delta before restore, serial observation/peek, restore result, and
  post-restore status.
- Accepted archive review passed with file_count=19, kernel_size=103040,
  header_image_size=103040, text_offset=0, flags=12, and
  loader_diagnostic=false.

## Validation Summary

- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec test: passed with 113 no_std tests after setting
  PATH for QEMU 9.2.0.
- scripts/qemu-remote-wakeup-request-smoke.sh: passed with
  classification=qemu-remote-wakeup-request-complete.
- scripts/qemu-cross-core-ipi-delivery-smoke.sh: passed with
  classification=qemu-cross-core-ipi-delivery-complete.
- scripts/qemu-smoke.sh: passed.
- scripts/rpi5-remote-wakeup-request-boot-tree.sh: passed.
- scripts/rpi5-archive-review.sh
  target/talos-rpi5-remote-wakeup-request-boot.tar.gz: passed.
- Serialized Pi 5 hardware run under hardwareTestLock: accepted on remote2.
- Restore proof: remote1/restore-exit.txt and remote2/restore-exit.txt are 0.

## Acceptance

Accepted as Pi 5 hardware evidence for the bounded scheduler-facing remote
wake-request model. Local runnable transitions from remote requests remain
deferred to a later target-owned wake-consumption task.
