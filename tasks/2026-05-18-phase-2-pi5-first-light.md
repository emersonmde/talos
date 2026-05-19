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
- 2026-05-19 minimal-config hardware attempt published archive digest `ad1c2f124015724f5026e9eb791405d296a546c59ba20f2712f18b827061f944`; `PUT /boot/archive` returned `ok=true`, `file_count=8`, and `extracted_bytes=252515`; `POST /power/cycle` returned `ok=true`; serial advanced from cursor `29021` to `29729` and emitted the same firmware/RP1 boundary ending in `RP1_BOOT chip ID: 0x20001927`. No `T1` marker or Talos banner appeared.
- 2026-05-19 post-hardware image-contract review compared Talos with the official Raspberry Pi `kernel_2712.img`. The official decompressed Pi 5 kernel image advertises arm64 Image `text_offset=0`, while Talos advertised `0x00200000`. The Pi 5 linker now uses `KERNEL_IMAGE_TEXT_OFFSET=0`, and the archive review gate expects `text_offset=0` for Pi 5.
- 2026-05-19 offset-zero hardware attempt published archive digest `a2b2fe61e41f0178f68437527e03a837d1cd21c9e10635b65c5ffa9aeac94116`; `PUT /boot/archive` returned `ok=true`; `POST /power/cycle` returned `ok=true`; serial advanced from cursor `29729` to `30437` and still stopped at the same firmware/RP1 boundary without `T1` or Talos output.
- 2026-05-19 follow-up image-contract review found another header mismatch: the official decompressed Pi 5 kernel image advertises arm64 Image flags `0xc`, while Talos advertised `0`. Talos now emits flags `0xc` for the Pi 5 image and the archive review gate checks that value.
- 2026-05-19 flags-matched hardware attempt published archive digest `22057bcd3614ebc871e3974a9f99bde524eb3241250105ca3343d1c07b21d462`; `PUT /boot/archive` returned `ok=true`; `POST /power/cycle` returned `ok=true`; serial advanced from cursor `30437` to `31145` and again stopped at the same firmware/RP1 boundary without `T1` or Talos output.
- 2026-05-19 post-hardware review: matching the official Pi 5 `text_offset` and flags did not change the failure boundary. The next useful direction is to determine whether the bootloader is actually reaching the TFTP/config/kernel load phase after RP1 firmware, or to try the Pi 5 `boot_ramdisk=1` path with a FAT32 `boot.img` so the network boot path receives a fuller boot filesystem shape.
- 2026-05-19 added `scripts/rpi5-boot-img.sh` and `scripts/rpi5-boot-ramdisk-tree.sh` to stage a documented Pi 5 `boot_ramdisk=1` network-boot experiment. The archive review gate now verifies `boot.img` is a readable FAT image containing the required config/kernel files when `boot_ramdisk=1` is enabled.
- 2026-05-19 boot-ramdisk hardware attempt published archive digest `4d6fc4527e6af0691167e974f4ca3e33c2afa1b3cf0ae8922e39c6e772e1bd94`; `PUT /boot/archive` returned `ok=true`, `file_count=9`, and `extracted_bytes=67361394`; `POST /power/cycle` returned `ok=true`; serial advanced from cursor `31145` to `31853` and again stopped at the same firmware/RP1 boundary without `T1`, Talos output, or visible `sha256=1` file-load hashes.
- 2026-05-19 post-hardware review: the unchanged boundary across direct-root, minimal-config, Pi5-header-matched, and `boot_ramdisk=1` archives suggests the next hypothesis is before or at the bootloader file-load/config phase rather than in Talos' Rust/BSS/stack path. Further Talos image-header tweaks have diminishing value without a way to see file-load progress or a different firmware entry path such as a custom armstub.
- 2026-05-19 added a custom armstub diagnostic path. `scripts/rpi5-armstub-diagnostic.sh` builds `armstub8-2712.bin` from a minimal AArch64 assembly stub that writes `S1\r\n` to firmware-preserved RP1 UART0 and waits. `scripts/rpi5-armstub-diagnostic-tree.sh` stages a one-off archive with `armstub=armstub8-2712.bin`, while the normal boot-tree path remains unchanged. The archive review gate now validates this optional armstub file when selected.
- 2026-05-19 armstub diagnostic hardware attempt published archive digest `c7fa87aa0d7b9989284e8bcfdd7ad09364c38619ea6eabd1531d4112a15ce8ea`; `PUT /boot/archive` returned `ok=true`, `file_count=9`, and `extracted_bytes=252645`; `POST /power/cycle` returned `ok=true`; serial advanced from cursor `31853` to `32561` and again stopped at the same firmware/RP1 boundary without `S1`, `T1`, or Talos output. This means either the network-boot firmware path did not reach the configured custom armstub, or the Pi 5 firmware ignored `armstub=armstub8-2712.bin` in this mode. The next bounded hypothesis is that the narrow Talos boot tree is missing some otherwise harmless firmware-stage files from the known-good Pi OS Lite tree, so a full-source boot tree with only the kernel/config deltas is worth testing before declaring the path blocked.
- 2026-05-19 added `scripts/rpi5-prefixed-boot-tree.sh` to mirror the required Talos boot files under `da591740/`, matching the Pi serial-prefix probe seen in the known-good TFTP request sequence. The archive review gate validates that the prefixed mirror is present and byte-identical when used.
- 2026-05-19 serial-prefix mirror hardware attempt published archive digest `aef985a35014cfffae791861c364a4776397569aaeca049c2b1cc891400f2628`; `PUT /boot/archive` returned `ok=true`, `file_count=16`, and `extracted_bytes=505030`; `POST /power/cycle` returned `ok=true`; serial advanced from cursor `32561` to `33269` and again stopped at the same firmware/RP1 boundary without `S1`, `T1`, or Talos output. This rules out the simplest root-vs-`da591740/` prefix fallback hypothesis for the current Talos archive shape.
- 2026-05-19 firmware/network-boot hypothesis review found one remaining low-cost archive-shape gap: the armstub diagnostic used only root files, while the serial-prefix mirror did not request `armstub8-2712.bin`. Added `scripts/rpi5-prefixed-armstub-diagnostic-tree.sh` to stage both `armstub=armstub8-2712.bin` and a byte-identical `da591740/` mirror, with archive-review checks for the mirrored armstub.
- 2026-05-19 combined serial-prefix plus armstub hardware attempt published archive digest `e16838e468ec5d07ff1a53a8fbfc5801446b6599e94edb25e6a752492a8279e7`; local gates passed at `cargo fmt --check`, `cargo -Zjson-target-spec test`, Pi 5 target build, `./scripts/rpi5-image.sh`, and archive review. `PUT /boot/archive` returned `ok=true`, `file_count=18`, and `extracted_bytes=505290`; `POST /power/cycle` returned `ok=true`; serial advanced from cursor `33269` to `33977` and again stopped at the same firmware/RP1 boundary without `S1`, `T1`, or Talos output.
- 2026-05-19 post-hardware review: the combined diagnostic rules out the remaining cheap root-vs-`da591740/` armstub/config archive-shape gap. The failure boundary is now before Talos entry and before the configured custom armstub can prove execution, or the Pi 5 network-boot firmware is not honoring that armstub/config path. Further kernel-image or archive-shape iterations are unlikely to create useful signal without lab-side TFTP request/file-load visibility, EEPROM boot diagnostics, or a recreated known-good Pi OS Lite boot source for direct comparison.
- 2026-05-19 lab API TFTP visibility run used the upgraded `/tftp/logs` endpoint around a controlled power cycle without republishing. Serial advanced from cursor `33977` to `34685` and still stopped at the firmware/RP1 boundary. Delayed TFTP logs proved the Pi requested and was served `da591740/config.txt`, `da591740/kernel_2712.img`, `da591740/bcm2712-rpi-5-b.dtb`, overlay files, `da591740/cmdline.txt`, and `da591740/armstub8-2712.bin`. This moves the boundary from "unknown file-load" to "files load, but neither the custom armstub marker nor kernel marker appears on the lab UART."
- 2026-05-19 review of the custom armstub found a concrete diagnostic flaw: the assembly marker disabled and re-enabled PL011 but did not program `IBRD`, `FBRD`, or `IMSC` like the Rust PL011 early init. Added those writes to both the armstub diagnostic and the Pi 5 assembly entry marker. Local gates passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, Pi 5 target build, `./scripts/rpi5-image.sh`, archive review, and QEMU smoke. Hardware attempt with archive digest `95ffda315ab53a17c2d66a4ef276b9a307b5f9a687c1a1cf07bbcb6c5f67197b` showed TFTP served the updated 128-byte `armstub8-2712.bin` and `kernel_2712.img`, but serial still stopped at the same boundary without `S1`, `T1`, or Talos output.
- 2026-05-19 added a preserved-UART marker before any PL011 reconfiguration: armstub emits `P0` first, then `S1`; kernel entry emits `P1` first, then `T1`. Local gates again passed. Hardware attempt with archive digest `62279261e7b5588cbcb340e96745ba991c28b213dd4d063b9de93f89cdc04876` showed TFTP served the updated 168-byte `armstub8-2712.bin` and `kernel_2712.img`, but serial still stopped at the same firmware/RP1 boundary without `P0`, `S1`, `P1`, `T1`, or Talos output.
- 2026-05-19 reference-implementation review found Circle's Pi 5 bare-metal config uses `kernel=kernel_2712.img` with `kernel_address=0x80000`. Talos now stages `kernel_address=0x80000` and links the Pi 5 image at `0x80000` while keeping arm64 Image `text_offset=0` and flags `0xc`. Local validation passed: `cargo fmt --check`, `cargo -Zjson-target-spec test`, Pi 5 target build, `./scripts/rpi5-image.sh`, symbol inspection showing `_start` and `__kernel_start` at `0x80000`, prefixed archive review, QEMU smoke, and `mdbook build`.
- 2026-05-19 Circle-style address plus custom armstub hardware attempt published archive digest `12f14dd0b2314fb926eb64d332faff5ff9ef7ea75a194d48a2d17e911d5ba67d`. `PUT /boot/archive` and `POST /power/cycle` returned `ok=true`; TFTP logs showed `da591740/config.txt`, `kernel_2712.img`, DTB, overlays, cmdline, and `armstub8-2712.bin` served, but serial still showed only the firmware/RP1 boundary with no `P0`, `S1`, `P1`, `T1`, or Talos output.
- 2026-05-19 Circle-style address without custom armstub hardware attempt published archive digest `d1f48d4a5e6c97a4554fda691140e6348c7ba26dd866e0b528eebe7057e2dad2`. `PUT /boot/archive` and `POST /power/cycle` returned `ok=true`; TFTP logs showed `da591740/config.txt`, `kernel_2712.img`, DTB, overlays, and cmdline served, plus a non-fatal `armstub8-2712.bin` not-found probe. Serial did not advance with Talos output. This rules out the custom armstub setting itself and the zero-vs-`0x80000` Pi 5 bare-metal load base as the immediate cause.
- 2026-05-19 Matthew corrected the workflow classification: the no-marker Pi 5 state is not blocked while bounded public-reference research, code review, loader diagnostics, or controlled hardware iterations remain.
- 2026-05-19 reference review compared Circle's Pi 5 bootloader with Talos. Circle stages a raw `kernel_2712.img` linked at `0x80000` rather than an arm64 Image-header payload. Added a separate raw loader diagnostic path: `scripts/rpi5-loader-diagnostic.sh` builds a 216-byte position-linked AArch64 `kernel_2712.img` that writes `L0` through firmware-preserved RP1 UART0, reinitializes RP1 UART0 and writes `L1`, also attempts a BCM2712 UART10 `U1` marker, then emits heartbeat dots. `scripts/rpi5-loader-diagnostic-tree.sh` and `scripts/rpi5-prefixed-loader-diagnostic-tree.sh` stage that raw image without changing the normal Talos boot path.
- 2026-05-19 local validation for the raw loader diagnostic passed: shell syntax checks, raw diagnostic build/disassembly, prefixed archive review with `loader_diagnostic=true`, `cargo fmt --check`, `cargo -Zjson-target-spec test`, Pi 5 target build, `./scripts/rpi5-image.sh`, `./scripts/qemu-smoke.sh`, and `mdbook build`.
- 2026-05-19 raw loader diagnostic hardware attempt published archive digest `b8c52c0736436fed53df9595b7b4e48c8aedb8e88deb055609bdd62c6b485e47`. `PUT /boot/archive` returned `ok=true`, `file_count=16`, and `extracted_bytes=175540`; `POST /power/cycle` returned `ok=true`. Serial advanced from cursor `36749` to `37457` through the same firmware/RP1 boundary and did not show `L0`, `L1`, `U1`, heartbeat dots, or Talos output. Full TFTP-log tail showed the Pi requested and was served the 216-byte `da591740/kernel_2712.img`, config, DTB, overlays, and cmdline at 17:35:23-17:35:24 UTC. This rules out the arm64 Image header and Rust/BSS/stack setup as the only explanation for the no-marker state; the next useful path is an execution proof that does not depend on the lab-visible RP1 UART path, or a deeper reference-backed handoff/exception-state diagnostic.

## Review

- Pre-hardware review findings: passed for one controlled attempt. The archive uses only explicit relative paths, includes the lab-controller required Pi 5 files, selects `kernel_2712.img`, enables RP1 UART0 preservation, and has rollback available.
- Hardware test evidence: archive publish and one controlled power-cycle succeeded. Serial confirms the Pi rebooted far enough to emit RP1 firmware messages on the lab cable, but Talos entry output was not observed. Rollback and recovery power-cycle also succeeded, returning the board to the Pi OS Lite boot tree.
- Post-hardware review findings: failed acceptance. The first hardware run proved reboot, RP1 UART visibility, rollback, and recovery, but did not prove Talos entry. The Pi 5 physical link base has been corrected and an assembly entry marker has been added for the next valid hardware run.

## Result

The first Talos boot archive was published to the lab TFTP root and controlled reboots were executed successfully through the fixed-port lab API. The observed serial output proves the lab cable and RP1 firmware path are active during boot, but it does not prove Talos reached entry. Local review found and corrected the Pi 5 link-address mismatch and the RP1 UART0 mapping. Subsequent hardware attempts with a direct assembly UART marker still stop after the firmware DDR log boundary, before any Talos marker appears. The next investigation should determine whether Pi firmware is reading the Talos TFTP boot tree after DDR init, whether a `boot.img`/full boot-tree shape is required for this network path, or whether the raw `kernel_2712.img` image format/header is rejected before entry.

## Follow-Up

- Add lab API visibility for TFTP requests or bootloader file-load logs; current serial evidence stops before confirming that `config.txt`, `kernel_2712.img`, or `kernel8.img` are requested after DDR init.
- Re-check whether this network boot path needs a `boot.img` ramdisk or fuller Pi OS boot tree shape even though Pi 5 embeds the old `start.elf` role in EEPROM firmware.
- Run one controlled custom-armstub hardware diagnostic. Seeing `S1` means firmware honored the armstub path before the kernel handoff; not seeing it keeps the failure boundary before that diagnostic entry.
- Test a full-source boot tree variant that preserves the known-good Pi OS Lite boot file shape while replacing the kernel images and first-light config lines.
- Run one controlled serial-prefix mirror hardware diagnostic.
- Review remaining firmware/config-entry hypotheses before another hardware run. A combined prefix-plus-armstub archive is a possible narrow experiment, but it should first be weighed against Raspberry Pi EEPROM/network-boot documentation and any lab-side way to recover TFTP request visibility.
- Combined prefix-plus-armstub diagnostic completed and stopped at the same boundary.
- TFTP request/file-load visibility now proves the Pi is being served the prefixed config, kernel, DTB, overlays, cmdline, and armstub files.
- The remaining productive path is a loader/handoff diagnostic that does not depend only on the lab-visible RP1 UART path, deeper comparison with public Pi 5 boot references, or a controlled strategy shift such as a Linux-loaded payload if direct firmware handoff remains opaque after bounded diagnostics.
- Inspect the remaining staged `config.txt`, `cmdline.txt`, and `kernel_2712.img` contract against Raspberry Pi 5 firmware expectations, especially whether the raw ELF-stripped binary is acceptable as `kernel_2712.img` in this network boot path.
- Restore or recreate a known-good Pi OS Lite TFTP boot tree source before extended repeated publish cycles; the lab API keeps only one rollback archive, so repeated Talos publishes can displace the known-good rollback.
