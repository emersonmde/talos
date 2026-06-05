# Source/Script/Image Comparison

Task: phase11-rp1-diagnostic-entry-control-source-core-20260605

## Compared Inputs

- Accepted prompt-capable Pi 5 control boot tree: target/rpi5-local-cat-banner-boot-tree-local1.
- Prior blocked RP1 register-read candidate: target/rpi5-rp1-uart0-fr-read-proof-boot-tree-local1.
- Revised pre-MMIO RP1 read candidate: target/rpi5-rp1-uart0-fr-read-preentry-handoff-source-core-boot-tree.
- New entry-control candidate: target/rpi5-rp1-entry-control-source-core-boot-tree.
- Source routing: build.rs, src/main.rs, src/target/rpi5.rs, src/arch/aarch64/boot.S, and src/boot/rpi5.rs.
- Helper scripts: scripts/rpi5-rp1-entry-control-image.sh, scripts/rpi5-rp1-entry-control-boot-tree.sh, scripts/rpi5-rp1-uart0-fr-read-image.sh, and scripts/rpi5-rp1-uart0-fr-read-boot-tree.sh.

## Dispositions

- fixed: rpi5_rp1_entry_control now branches from rust_entry after EarlyPhaseLine::RustEntry and before BootInfo::from_aarch64_x0, target::init, boot reports, memory planning, allocator setup, or the RP1 UART0 FR read path.
- fixed: the new candidate is qualitatively different from the prior pre-MMIO marker because it stops before normal Pi 5 target init; the prior candidate reached the marker only after target initialization and normal boot reporting code.
- fixed: task-owned helper scripts produce kernel_2712-rp1-entry-control.img, root kernel_2712.img/kernel8.img, and a byte-identical da591740/ mirror from the accepted Pi 5 boot-tree shape.
- fixed: marker evidence confirms the candidate contains rpi5-rp1-entry-control: rust-entry-control, rpi5-rp1-entry-control: no-rp1-mmio, classification=entry-control-reached, and PASS.
- removed: raw TALOS_RPI5_EARLY_ENTRY_PROVENANCE_SCENARIO assembly marker routing remains absent.
- not-an-issue: the existing RP1 UART0 FR read candidate still owns the only RP1 read-value classification strings and keeps the one-read diagnostic contract unchanged.
- not-an-issue: the entry-control kernel retains the accepted arm64 Image contract: _start and __kernel_start at 0x200000, text_offset=0, header_image_size=51808, flags=12, and magic=ARMd.
- deferred: hardware proof, candidate publication, TFTP evidence, serial evidence, power cycle, restore, and hardwareTestLock acquisition are explicitly deferred to the queued Pi 5 proof task.

## Candidate Identity

- archive: target/talos-rpi5-rp1-entry-control-source-core.tar.gz
- archive SHA-256 at final review: 52a8ef95e6154c18e41227ea48685e8cff01a52f518068c8acbb65c072cbe9e1
- root/prefixed kernel SHA-256: b3e62b950cf007a0ee8d1d7f420fd8c26c28573c5b6925a7f0d93d0b77a367ea
- kernel size: 51808
- arm64 Image header: text_offset=0, header_image_size=51808, flags=12, magic=ARMd

Evidence level: source/static image/archive inspection only. No hardware lock, archive publication, TFTP observation, serial hardware run, or power cycle was performed.
