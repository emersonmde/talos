# Source/Script/Image Comparison

Task: `phase11-rp1-diagnostic-preentry-handoff-source-core-20260605`

## Compared Inputs

- Accepted hardware-control boot source: `target/rpi5-local-cat-banner-boot-tree-local1`.
- Previous blocked diagnostic candidate: `target/rpi5-rp1-uart0-fr-read-proof-boot-tree-local1`.
- Diagnostic source routing: `build.rs`, `src/boot/rpi5.rs`, `src/target/rpi5.rs`, and `src/arch/aarch64/boot.S`.
- Diagnostic helpers: `scripts/rpi5-rp1-uart0-fr-read-image.sh` and `scripts/rpi5-rp1-uart0-fr-read-boot-tree.sh`.
- Standard Pi 5 helpers: `scripts/rpi5-image.sh` and `scripts/rpi5-boot-tree.sh`.

## Dispositions

- fixed: `run_rp1_uart0_fr_read_diagnostic` previously read `RP1_UART0_FR` before printing a diagnostic-specific marker. The revised code prints `start` and `pre-mmio-read`, flushes UART10, then performs the same single volatile 32-bit load.
- fixed: the revised static candidate can support the later hardware classifications `reached-Talos/pre-MMIO`, `mapped/read-value`, or a remaining pre-entry/handoff blocker without changing RP1 constants.
- removed: `TALOS_RPI5_EARLY_ENTRY_PROVENANCE_SCENARIO` is not assigned to the RP1 diagnostic. Phase 10 retained evidence shows raw assembly UART markers made accepted prompt-capable Pi 5 controls fail, so reintroducing that path would weaken the proof.
- not-an-issue: `build.rs` still registers only the explicit `rpi5_rp1_uart0_fr_read` scenario with no implied shell behavior and no assembly marker define.
- not-an-issue: `src/boot/rpi5.rs` calls the diagnostic only under `talos_boot_scenario = "rpi5_rp1_uart0_fr_read"`, after the normal Pi 5 Rust entry, target initialization, boot identity, memory planning, and retained early-phase lines.
- not-an-issue: `scripts/rpi5-rp1-uart0-fr-read-boot-tree.sh` follows the accepted boot-tree shape: it starts from `scripts/rpi5-boot-tree.sh`, then replaces root and `da591740/` prefixed `kernel_2712.img` and `kernel8.img` with the focused candidate image.
- not-an-issue: `scripts/rpi5-boot-tree.sh` preserves the accepted Pi 5 config/cmdline cleanup: `earlycon` points at `0x1f00030000`, `dtoverlay=uart0-pi5` is removed, and `kernel_address=` is removed.
- deferred: only a serialized Pi 5 run can decide whether the revised candidate reaches the pre-MMIO marker, traps/hangs on the RP1 read, returns a raw value, or still fails before visible Talos output.

## Expected Revised Candidate Markers

- `rpi5-rp1-uart0-fr-read: start`
- `rpi5-rp1-uart0-fr-read: pre-mmio-read`
- `phase11-rp1-pcie-map-contract-v1`
- `classification=mapped/read-value`
- `rpi5-rp1-uart0-fr-read: PASS`

Evidence level: static/source comparison only. No hardware lock, archive publication, TFTP observation, serial hardware run, or power cycle was performed.

## Revised Candidate Identity

- archive: `target/talos-rpi5-rp1-uart0-fr-read-preentry-handoff-source-core.tar.gz`
- archive SHA-256: `2640ab9ceabee343ee1426b7137e1597687517f56d3b61f58a7ac0e7ab4b6608`
- root/prefixed kernel SHA-256: `4500b99a4405f91176d39dc8178fcd396611e97577eb98c357927df05de6f792`
- kernel size: `87480`
- arm64 Image header: `text_offset=0`, `header_image_size=87480`, `flags=12`, `magic=ARMd`
- ELF entry: `0x200000`; `_start` and `__kernel_start` remain at `0x200000`.
