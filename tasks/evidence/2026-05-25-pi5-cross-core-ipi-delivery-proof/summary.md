# Pi 5 Cross-Core IPI Delivery Proof Evidence

Task: phase6-pi5-cross-core-ipi-delivery-proof-20260525

Current classification: pi5-cross-core-ipi-delivery-complete

## Attempts

1. Directed target-list SGIR: archive af89b6b2952a11833b30213b96d0615a1060305c52f234dfef6656e08012ec7c, kernel 1dd1cead59bbc123d6874be175ca9c3ac73bdf3cda224ac650dca8be1cf7ce4a, size 95,536 bytes. Result: all secondaries reached ready-mask=0xe; SGI receive/EOI counts stayed zero.

2. Directed target-list SGIR with explicit SGI Group 1/PPI enable setup: archive 740efe16750dc9b1056d453120871c9456306907b90ada934139e564d2843f16, kernel 38188d75e89f437a525622c79f267a551b1e4abd9a1e1f221173f245b7cfd923, size 96,032 bytes. Result: same ready/no-receive invariant failure.

3. SGIR target-filter all-except-self: archive 263fdf38c2b138e13ab2ebcca50d401524551181a3156a078bf5b545fda57053, kernel 563489cc5bf06c2e271d00e26d0ba2d6d05845d2927a02bc33f854df4d254a98, size 95,872 bytes, SGIR 0x01000001. Result: all secondaries reached ready-mask=0xe; complete-mask=0x0; receive/EOI counts stayed zero.

4. Secondary CPU-interface discriminator with HPPIR/IAR/DAIF/HCR report fields: archive 6bc6b07d10c29c408e9c198d28d324cdfe098af1d2d8bd53f1a6e1ead7d39d0f, kernel cac5c86c137ffc2dada30ea0ab6486d14205fa503e1acea945cc00cf0a0b7f59, size 96,632 bytes. Result: the candidate fetched, but the first discriminator shape still used WFE before polling. Report fields showed poll-count=0 and zero HPPIR/IAR/DAIF/HCR samples, so this run did not classify pending-at-interface versus not-pending.

5. Active-spin CPU-interface discriminator: archive 668c81cf5d38e50a9bb46722568a24c1bd385bc61bad5d38dbec05d7dd30aae6, kernel 2a0bded765427afb0c678729d89931cc289c27ab2bab94598ad8549072bed0ef, size 96,696 bytes. Result: TFTP repeatedly served da591740/kernel_2712.img at 96,696 bytes, but cursor-valid serial observe was empty and retained serial peek did not include the unique cpuif-poll=active-spin marker. Classify this run as candidate-fetched/current-serial-not-proven, not as SGI-delivery evidence.

6. Pi 5 IRQ-dispatch correction proof: archive a6c5cb6999784e8f8c61a07765d39e9549c19c0ae37a54267c738b116a521a79, kernel 44792c6681d0e67df08abeaebd18f2408680940ead47e2cf1e0b44f5b3956837, size 97,016 bytes. Result: after the Pi 5 exception dispatcher included the cross-core IPI proof flag, cursor-valid serial showed `cpuif-poll=active-spin`, SGIR 0x01000001, receivers 1, 2, and 3 each reached `receive-count=1 eoi-count=1 intid=1`, final `participants=3 expected=3 errors=0 ready-mask=0xe complete-mask=0xe classification=pi5-cross-core-ipi-delivery-complete`, and `PASS`.

## Hardware Artifacts

Each attempt captured health, pre-status, pre-serial cursor, TFTP cursor, snapshot, publish, post-publish status/files, power cycle, TFTP delta, serial observe or retained serial peek, pre-restore status, restore, post-restore status, and serial peek evidence. Later discriminator attempts are under cpuif/, cpuif2/, cpuif-spin/, and cpuif-spin2/.

The accepted archive review passed with file_count=19, kernel_size=97016, header_image_size=97016, text_offset=0, flags=12, and loader_diagnostic=false.

## Validation Summary

- cargo fmt --all -- --check: passed after the Pi 5 IRQ-dispatch correction.
- cargo -Zjson-target-spec test: passed with 110 no_std tests after the Pi 5 IRQ-dispatch correction.
- scripts/qemu-smoke.sh: passed after the Pi 5 IRQ-dispatch correction.
- scripts/qemu-cross-core-ipi-delivery-smoke.sh: passed after the Pi 5 IRQ-dispatch correction with classification=qemu-cross-core-ipi-delivery-complete.
- scripts/rpi5-cross-core-ipi-delivery-boot-tree.sh: passed
- scripts/rpi5-archive-review.sh target/talos-rpi5-cross-core-ipi-delivery-boot.tar.gz: passed
- Serialized Pi 5 hardware run under hardwareTestLock: accepted. The irqdispatch1 run produced cursor-valid serial and classification=pi5-cross-core-ipi-delivery-complete.
- Restore proof: all restore-exit.txt files captured so far are 0
- git diff --check: passed after documentation updates.
- mdbook build: not run because mdbook is unavailable in the container.

## Acceptance

Accepted as raw Pi 5 SGI delivery evidence for the future scheduler wakeup prerequisite. This does not introduce production remote wakeups, remote enqueue ownership, shared run queues, task migration, or broader scheduler migration.
