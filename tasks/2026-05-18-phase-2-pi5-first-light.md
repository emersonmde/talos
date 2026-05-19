# Phase 2 Pi 5 First Light

## Task

- Title: Pi 5 firmware handoff and firmware-preserved serial first light
- Owner: Seldon / supervisor loop
- Date: 2026-05-18
- Milestone: Phase 2.1
- Scope: Prepare, review, publish, and validate the first Talos boot archive for the physical Raspberry Pi 5 lab.

## Goal

Boot Talos through the normal Raspberry Pi 5 firmware path and capture enough serial evidence to prove that Talos code reached the CPU entry point.

## Acceptance Criteria

- A local boot tree can be staged with `config.txt`, `cmdline.txt`, `bcm2712-rpi-5-b.dtb`, and Talos `kernel_2712.img`.
- Pre-hardware review confirms the archive contents match the lab-controller contract and does not require direct UniFi access or credential handling.
- The lab publishes the archive through `PUT /boot/archive`.
- A single controlled power cycle reaches Talos code on the Pi 5.
- Serial output includes Talos version, exception level, DTB physical address, target name, service summary, and panic/halt behavior if boot fails later.
- A failed boot can be rolled back, and the evidence record captures the archive digest, publish result, power-cycle time, serial tail, classification, suspected cause, and next action.

Required validation level:

- Local preparation: formatting/lint/typecheck plus Pi 5 target build.
- Pre-hardware readiness: static inspection of staged archive contents.
- Acceptance: lab-controller API path plus serial hardware boot/output.

## Context

- Lab API: `http://talos-lab-api:8080`.
- Serial is configured and reachable through the lab API.
- The lab API currently lists a valid Pi 5 boot tree, including `bcm2712-rpi-5-b.dtb`, `kernel_2712.img`, `initramfs_2712`, and overlays.
- The supervisor must not publish archives or power-cycle the Pi without the hardware-test lock, acceptance criteria, and pre-hardware review.
- Talos has local staging helpers: `scripts/rpi5-image.sh` and `scripts/rpi5-boot-tree.sh`.

## Work Performed

- Added local Pi 5 image generation and boot-tree staging scripts.
- Documented the staging flow in the lab-controller notes.
- Marked the Phase 2.1 archive-publish criterion as having local staging support, with publish still intentionally not run.
- Split QEMU and Pi 5 console selection so the Pi 5 build no longer writes
  diagnostics to QEMU's PL011 address.
- Added an early Pi 5 firmware-preserved PL011 console path at physical
  `0x10_7d00_1000`, derived from Raspberry Pi Linux `bcm2712.dtsi`
  `soc@107c000000` ranges plus `uart10: serial@7d001000`.
- Re-routed the first-light Pi 5 console to RP1 UART0 because the lab cable is
  on the 40-pin header path.
- Corrected the RP1 UART0 physical address from the PCIe non-prefetchable CPU
  window `0x1f_0003_0000` to the firmware-preserved RP1 UART mapping
  `0x1c_0003_0000`. Raspberry Pi firmware reports `RP1_UART
  0000001c00030000` with `enable_rp1_uart=1`, and Linux describes RP1 UART0 as
  RP1 bus offset `0xc0_40030000`.

## Evidence

Completed local checks:

~~~bash
sh -n scripts/rpi5-image.sh scripts/rpi5-boot-tree.sh
./scripts/rpi5-image.sh
./scripts/rpi5-boot-tree.sh target/test-pi-boot-source target/test-rpi5-boot-tree
tar -C target/test-rpi5-boot-tree -czf target/test-talos-rpi5-boot.tar.gz .
./scripts/rpi5-archive-review.sh target/test-talos-rpi5-boot.tar.gz
cargo fmt --check
cargo -Zjson-target-spec test
cargo -Zjson-target-spec build --target targets/aarch64-talos-rpi5-bcm2712.json
mdbook build
~~~

Results:

- Shell syntax check passed.
- Pi 5 image generation produced `target/aarch64-talos-rpi5-bcm2712/debug/kernel_2712.img`.
- Synthetic boot-tree staging produced the required archive-root files: `config.txt`, `cmdline.txt`, `bcm2712-rpi-5-b.dtb`, and `kernel_2712.img`.
- Synthetic tar archive inspection showed only relative safe paths.
- QEMU no_std tests passed: 3 tests.
- Pi 5 target build passed.
- Pi 5 image generation passed after routing the Pi 5 console to UART10.
- mdBook build passed.
- 2026-05-19 supervisor recheck: Pi 5 target build passed.
- 2026-05-19 supervisor recheck: QEMU smoke passed and printed the expected Talos boot banner.
- 2026-05-19 lab API check: health ok, serial.configured=true, boot tree lists required Pi 5 files, and rollback is available.
- 2026-05-19 lab API check: serial peek returned the retained Linux login prompt, proving the serial read path is live.
- 2026-05-19 lab API check: UniFi guard status reported `GET /proxy/network/api/s/default/stat/sta/88:a2:9e:ae:c8:7f: HTTP Error 400`, so no power-cycle was attempted.
- 2026-05-19 Matthew clarified the Pi OS Lite TFTP tree is disposable and authorized replacing it with Talos boot files.
- 2026-05-19 official Raspberry Pi boot docs recheck: Pi 5 firmware defaults to `kernel_2712.img`, requires `config.txt` for a bootable partition, supports `enable_rp1_uart=1` to preserve 40-pin header RP1 UART0 at 115200, and `os_check=0` is appropriate for bare-metal development.
- 2026-05-19 local validation after RP1 UART0 routing passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, `cargo -Zjson-target-spec build --target targets/aarch64-talos-rpi5-bcm2712.json`, and `./scripts/rpi5-image.sh`.
- 2026-05-19 staged archive `target/talos-rpi5-boot.tar.gz` with digest `9db4ed077db8a5ae7f5985dfab750ff194c0c6eade2e78454753520b720cd644`; contents were exactly `config.txt`, `cmdline.txt`, `bcm2712-rpi-5-b.dtb`, `kernel_2712.img`, `overlays/overlay_map.dtb`, `overlays/bcm2712d0.dtbo`, and `overlays/uart0-pi5.dtbo`.
- 2026-05-19 lab publish succeeded through `PUT /boot/archive`; the API reported `ok=true`, `file_count=7`, `extracted_bytes=169919`, and rollback archive `/state/boot-previous.tar.gz`.
- 2026-05-19 the controlled power-cycle did not run. Five bounded status retries all reported the UniFi guard error `GET /proxy/network/api/s/default/stat/sta/88:a2:9e:ae:c8:7f: HTTP Error 400`, so the lab API failed closed before power action.
- 2026-05-19 serial cursor remained at 57 with the retained `talos-pi5 login:` prompt after the blocked power attempt, confirming no new boot output was observed.
- 2026-05-19 follow-up lab API check after the fixed-port API update: status returned `ok=true`, `serial.configured=true`, guard `fixed-port`, Weathertop port 8 `poe_state=UP`, and the active boot tree still contained the published Talos files.
- 2026-05-19 controlled power cycle succeeded through `POST /power/cycle`; response `ok=true`, guard `mode=fixed-port`, `switch_name=Weathertop`, `port_idx=8`, `poe_state=UP`.
- 2026-05-19 serial observe from cursor 57 captured RP1 firmware output only: `1.81 RP1 FW: load 0` and `1.82 RP1_BOOT chip ID: 0x20001927`, followed by a NUL/newline on the longer observe window. No Talos banner, boot-info line, or panic/halt output was observed.
- 2026-05-19 rollback succeeded through `POST /boot/rollback`; the restored Pi OS Lite boot tree contains 414 files including `config.txt`, `cmdline.txt`, `bcm2712-rpi-5-b.dtb`, `initramfs_2712`, `kernel8.img`, and `kernel_2712.img`.
- 2026-05-19 recovery power cycle succeeded after rollback. Serial confirmed Linux boot resumed: `Linux 6.12.75+rpt-rpi-2712`, RP1 PCIe enumeration, SD root mount, systemd start, hostname `talos`, and root filesystem remounted read/write.
- 2026-05-19 post-hardware local review found the likely load-address bug: the shared linker placed the Pi 5 image at `0x40200000`, which is QEMU virt's `0x40000000` RAM base plus the arm64 Image `0x200000` text offset. The Pi 5 firmware path should use the same text offset from the Pi RAM base, so the Pi 5 target now uses `linker-rpi5.ld` and links `_start` at `0x00200000`.
- 2026-05-19 validation after the Pi 5 linker split passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, `cargo -Zjson-target-spec build --target targets/aarch64-talos-rpi5-bcm2712.json`, `./scripts/rpi5-image.sh`, and `./scripts/qemu-smoke.sh`. Static symbol inspection confirmed Pi 5 `_start` and `__kernel_start` at `0x00200000`; QEMU smoke still printed the Talos boot banner and PASS line.
- 2026-05-19 follow-up hardware-loop note: a second publish/power-cycle attempt returned `ok=true`, but the serial cursor did not advance, so it did not produce valid Talos boot evidence. The boot tree was rolled back to Pi OS Lite and status again showed 414 boot files with `kernel8.img`, `kernel_2712.img`, `initramfs_2712`, `config.txt`, `cmdline.txt`, and `bcm2712-rpi-5-b.dtb`; retained Linux logs still show root remounted read/write.
- 2026-05-19 added a Pi-only assembly marker that writes `T0\r\n` to RP1 UART0 immediately after arm64 Image entry branches to the real start label, before preserving x0, clearing BSS, setting the stack, or entering Rust. This is intended to distinguish "firmware did not reach Talos entry" from "Rust or early memory setup failed before normal console output."
- 2026-05-19 Matthew pushed back on treating the missing serial cursor movement as a hard blocker. Follow-up review found a concrete marker bug: Talos used `0x1f00030000`, while firmware and Pi 5 bare-metal references point at `0x1c00030000` for the preserved RP1 UART0 mapping. Talos now uses `0x1c_0003_0000` in both the Pi 5 target map and the assembly marker, and staged `cmdline.txt` normalizes `earlycon=pl011,mmio32,0x1c00030000`.
- 2026-05-19 local validation after the RP1 UART address correction passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, Pi 5 boot-tree generation, `./scripts/qemu-smoke.sh`, and `mdbook build`. QEMU smoke still printed the Talos boot banner and PASS line.
- 2026-05-19 hardware attempt with the corrected `0x1c00030000` marker published archive digest `a1063ed6fe9b32decd729389bc40c132826a35b16cd6b671e744f8cdfb479ed4`; `POST /power/cycle` returned `ok=true`; serial advanced from cursor `26367` to `27015` and emitted Pi firmware/DDR logs through `DDR 4267 1 0 64 152 BL:1`, but no `T0` marker or Talos banner appeared.
- 2026-05-19 changed the Pi assembly marker to initialize PL011 control/line-control using the existing firmware clock/divisor state, clear interrupts, bound the TX-full wait, and emit `T1\r\n` so an unexpected flag state cannot hang before Rust. Local validation passed again.
- 2026-05-19 hardware attempt with the bounded `T1` marker published archive digest `49ceeba809c0e0a88497c7265e2459e7e39d2a07fc74e6d27bb9dc794954d146`; `POST /power/cycle` returned `ok=true`; serial advanced from cursor `27017` to `27665` and again stopped after the same firmware/DDR log boundary with no `T1` marker.
- 2026-05-19 added a duplicate `kernel8.img` alongside `kernel_2712.img` because the lab API status helper reported `active_name=kernel8.img` even when the Talos config selected `kernel_2712.img`. Local validation passed. Hardware attempt published archive digest `5b2bea918f23756c64cbc98529b55d95e246c342cc364010ee5581281ff8fbe5`; `POST /power/cycle` returned `ok=true`; serial advanced from cursor `27665` to `28313` and still stopped at the firmware/DDR boundary without a Talos marker.
- 2026-05-19 non-hardware image-format review found the arm64 Image header advertised `image_size=0x200000` even though the generated binary was 82616 bytes. Talos now emits `__kernel_image_end - _start` in the Image header and `scripts/rpi5-image.sh` fails if the header size differs from the generated file size.
- 2026-05-19 local validation after the arm64 Image header correction passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, `cargo -Zjson-target-spec build --target targets/aarch64-talos-rpi5-bcm2712.json`, `./scripts/rpi5-image.sh`, `./scripts/qemu-smoke.sh`, and `mdbook build`. Header inspection showed `file=82616`, `header_size=82616`, and `text_offset=0x00200000`.
- 2026-05-19 lab API probe found no exposed TFTP log endpoint: `/boot/logs`, `/tftp/logs`, and `/logs/tftp` all returned 404. The existing exposed read-only evidence is `/status`, `/boot/files`, `/serial/peek`, and `/serial/tail`.
- 2026-05-19 added `scripts/rpi5-archive-review.sh` as a local pre-hardware gate. It checks required archive files, unsafe paths, `config.txt` first-light settings, `kernel_2712.img`/`kernel8.img` equality, arm64 Image magic, text offset, and header image size.
- 2026-05-19 corrected-image archive pre-hardware review passed for `target/talos-rpi5-boot.tar.gz`: sha256 `7c3994c313c5491414927d998c535522c8b1f920a2e608debafb4676fb7aadff`, required files present, `kernel_2712.img` and `kernel8.img` identical, `kernel_size=82616`, `header_image_size=82616`, and `text_offset=2097152`.
- 2026-05-19 corrected-image hardware attempt published archive digest `b437659350d6c110f056a664d17fb48a3dfb24117440e04013454f44b1918ab9`; `PUT /boot/archive` returned `ok=true`, `file_count=8`, and `extracted_bytes=252535`; `POST /power/cycle` returned `ok=true`; serial advanced from cursor `28313` to `29021` and emitted Pi firmware logs through `DDR 4267 1 0 64 152 BL:1`, then `RP1 FW: load 0` and `RP1_BOOT chip ID: 0x20001927`. No `T1` marker or Talos banner appeared.
- 2026-05-19 post-hardware review classified the corrected-image run as still failing before Talos entry. The next bounded hypothesis is that the inherited Linux `dtoverlay=uart0-pi5` is unnecessary for bare-metal first-light and may add avoidable firmware/device-tree work before entry. The staging script now strips that line and the archive review gate rejects it.

## Review

- Pre-hardware review findings: passed for one controlled attempt. The archive uses only explicit relative paths, includes the lab-controller required Pi 5 files, selects `kernel_2712.img`, enables RP1 UART0 preservation, and has rollback available.
- Hardware test evidence: archive publish and one controlled power-cycle succeeded. Serial confirms the Pi rebooted far enough to emit RP1 firmware messages on the lab cable, but Talos entry output was not observed. Rollback and recovery power-cycle also succeeded, returning the board to the Pi OS Lite boot tree.
- Post-hardware review findings: failed acceptance. The first hardware run proved reboot, RP1 UART visibility, rollback, and recovery, but did not prove Talos entry. The Pi 5 physical link base has been corrected and an assembly entry marker has been added for the next valid hardware run.

## Result

The first Talos boot archive was published to the lab TFTP root and controlled reboots were executed successfully through the fixed-port lab API. The observed serial output proves the lab cable and RP1 firmware path are active during boot, but it does not prove Talos reached entry. Local review found and corrected the Pi 5 link-address mismatch and the RP1 UART0 mapping. Subsequent hardware attempts with a direct assembly UART marker still stop after the firmware DDR log boundary, before any Talos marker appears. The next investigation should determine whether Pi firmware is reading the Talos TFTP boot tree after DDR init, whether a `boot.img`/full boot-tree shape is required for this network path, or whether the raw `kernel_2712.img` image format/header is rejected before entry.

## Follow-Up

- Add lab API visibility for TFTP requests or bootloader file-load logs; current serial evidence stops before confirming that `config.txt`, `kernel_2712.img`, or `kernel8.img` are requested after DDR init.
- Re-check whether this network boot path needs a `boot.img` ramdisk or fuller Pi OS boot tree shape even though Pi 5 embeds the old `start.elf` role in EEPROM firmware.
- Inspect the remaining staged `config.txt`, `cmdline.txt`, and `kernel_2712.img` contract against Raspberry Pi 5 firmware expectations, especially whether the raw ELF-stripped binary is acceptable as `kernel_2712.img` in this network boot path.
- Restore or recreate a known-good Pi OS Lite TFTP boot tree source before further repeated publish cycles; the lab API keeps only one rollback archive, so repeated Talos publishes can displace the known-good rollback.
