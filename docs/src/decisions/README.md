# ADR Index

Architectural decision records live here.

Create an ADR when a decision is expensive to reverse, affects subsystem boundaries, constrains future POSIX compatibility, or changes the hardware lab contract.

ADR template:

- Status:
- Context:
- Decision:
- Consequences:
- Alternatives considered:

## 2026-05-18 - Repository and Target Strategy

- Status: accepted
- Context: Talos needs to be a standalone project that Matthew can later push to GitHub, while still allowing fast generic kernel iteration before physical Pi 5 serial feedback is available.
- Decision: Keep Talos in its own git repository under `projects/talos`. Build the kernel as a Rust no_std project with a generic AArch64/QEMU virt target for fast validation and a separate `talos-rpi5-bcm2712` target for the first physical board.
- Consequences: Generated artifacts stay out of git. QEMU results may validate generic architecture and toolchain behavior, but physical Pi 5 claims require lab-controller and serial evidence.
- Alternatives considered: continue evolving Daedalus directly, make the project Pi 5-only without a generic target split, or delay repo setup until first hardware boot. Those options would make the design harder to review, harder to publish, or more likely to mix Pi 4 assumptions into Talos.

## 2026-05-19 - Target-Specific Physical Link Bases

- Status: superseded by 2026-05-19 - Match Raspberry Pi 5 Kernel Image Text Offset
- Context: The first Pi 5 hardware boot emitted RP1 firmware serial messages but no Talos banner. Static inspection showed the shared linker script placed Talos at `0x40200000`, matching QEMU virt RAM base `0x40000000` plus the arm64 Image `0x200000` text offset. Raspberry Pi firmware should instead load the arm64 Image at the text offset from the Pi RAM base, so early absolute symbols such as BSS and stack must resolve near `0x00200000` for the physical Pi path.
- Decision: Keep the arm64 Image text offset at `0x00200000`, keep QEMU virt linked at `0x40200000`, and give the Pi 5 target its own linker script that links `kernel_2712.img` at `0x00200000`.
- Required validation: QEMU smoke must continue passing for the generic target. Pi 5 target builds must show `_start` and `__kernel_start` at `0x00200000` before the next hardware archive is published. Physical acceptance still requires lab-controller publish, one controlled power-cycle, serial output proving Talos reached entry, and rollback/recovery if the boot fails.
- Risks: If Raspberry Pi firmware uses a nonzero physical base in the lab configuration, this needs revisiting. The current decision is based on the Linux arm64 Image contract plus the prior failure shape; serial hardware evidence remains the deciding proof.
- Alternatives considered: keep one QEMU-oriented linker layout for both targets, use a custom armstub, or add an assembly-only UART probe before addressing the load-base mismatch. The separate Pi 5 linker is the smallest correction that preserves the firmware contract and avoids changing the hardware boot path.

## 2026-05-19 - Pi 5 RP1 UART0 Preserved Mapping

- Status: accepted
- Context: The lab serial cable observes the Pi 5 40-pin header UART. The first RP1 UART0 marker used `0x1f00030000`, derived from the PCIe non-prefetchable window for pcie2. Hardware attempts did not emit the marker. Raspberry Pi firmware documentation says `enable_rp1_uart=1` initializes RP1 UART0 for bare-metal debug and does not reset RP1 when paired with `pciex4_reset=0`; a Pi 5 bare-metal reference reports firmware output `RP1_UART 0000001c00030000` for that mode. Raspberry Pi Linux describes RP1 UART0 as RP1 bus register offset `0xc0_40030000`.
- Decision: Treat `0x1c00030000` as the firmware-preserved RP1 UART0 physical mapping for first-light diagnostics. Use it in the Pi 5 target map, the assembly marker, and staged `earlycon` hints. Keep `0x1f...` documented as the pcie2 non-prefetchable CPU window, not the preserved firmware UART path.
- Required validation: Local Pi 5 builds must show the marker uses `0x1c00030000`. Physical acceptance still requires a controlled lab power-cycle and serial output proving Talos reached entry.
- Risks: The current hardware evidence still stops after firmware DDR logs, so this decision fixes a concrete address bug but does not yet prove the kernel image is reached. If later TFTP/image-format evidence shows the firmware uses a different mapped view at handoff, revisit this ADR.
- Alternatives considered: continue using the PCIe window address, use BCM2712 UART10 instead of the header UART, or wait for a full RP1/PCIe driver before serial diagnostics. The preserved RP1 UART path is the narrowest path aligned with the attached cable and Raspberry Pi firmware support.

## 2026-05-19 - Arm64 Image Header Size Must Match the Binary

- Status: accepted
- Context: Pi 5 hardware attempts reached firmware DDR logging but not Talos entry. A non-hardware image-format review found Talos' arm64 Image header advertised `image_size=0x200000`, while the generated `kernel_2712.img` was 82616 bytes. That stale constant came from the text offset, not the produced binary size.
- Decision: Emit `__kernel_image_end - _start` in the arm64 Image header and define `__kernel_image_end` before `.bss` in each linker script. Keep generated heap/stack reservations as `NOLOAD` memory owned by early kernel setup, not bytes claimed in the boot image file. Make `scripts/rpi5-image.sh` fail if the header size and file size diverge.
- Required validation: Pi 5 image generation must report matching file/header sizes. QEMU smoke must continue to boot the generic target, and physical Pi 5 acceptance still requires a controlled hardware run with serial evidence.
- Risks: If Raspberry Pi firmware wants a different interpretation of the arm64 `image_size` field for this network boot mode, revisit with TFTP/firmware evidence. The current choice aligns the header with the actual binary loaded by firmware.
- Alternatives considered: leave `image_size` at the text offset, set `image_size=0`, or include `NOLOAD` reservations in the file size. Matching the generated file size is the narrowest correction and gives the build a regression check.

## 2026-05-19 - Keep First-Light Firmware Configuration Minimal

- Status: accepted
- Context: Corrected-image hardware evidence reached Raspberry Pi firmware and RP1 firmware logging, but still did not emit the Talos entry marker. The staged boot tree inherited `dtoverlay=uart0-pi5` from the Linux boot source. Talos first-light writes RP1 UART0 directly through the firmware-preserved mapping, before parsing or relying on the device tree.
- Decision: Strip `dtoverlay=uart0-pi5` from Talos first-light `config.txt`. Keep `enable_rp1_uart=1`, `pciex4_reset=0`, and `uart_2ndstage=1`, because those are directly relevant to preserving the 40-pin header UART and observing firmware logs.
- Required validation: The archive review gate must fail if `dtoverlay=uart0-pi5` remains in the staged config. Physical acceptance still requires one controlled hardware run and serial output proving Talos reached entry.
- Risks: If later Talos relies on firmware-applied overlays or Linux-compatible DTB mutations, this should be revisited after a DTB parser exists. For first-light, removing the overlay reduces firmware work before entry and narrows the failure surface.
- Alternatives considered: keep all Linux boot-source config lines unchanged, remove all overlays from the archive, or switch to a boot ramdisk flow. Stripping only the Linux UART overlay is the smallest change tied to the current failure mode.

## 2026-05-19 - Match Raspberry Pi 5 Kernel Image Text Offset

- Status: accepted
- Context: Repeated hardware attempts reached Raspberry Pi firmware and RP1 firmware logging but never emitted the Talos entry marker. The Talos Pi 5 image advertised arm64 Image `text_offset=0x00200000`. A comparison against the official Raspberry Pi `kernel_2712.img` showed the decompressed Pi 5 kernel image advertises `text_offset=0`, `image_size=30081024`, flags `0xc`, and `ARMd` magic.
- Decision: Link the Talos Pi 5 image at physical `0x00000000` and advertise arm64 Image `text_offset=0` for `kernel_2712.img`. Keep the QEMU virt target at its QEMU-specific `0x40200000` link/load address.
- Follow-up: Match the official Pi 5 arm64 Image flags field as well: Talos now advertises flags `0xc` for the Pi 5 image while keeping the QEMU image flags unchanged.
- Required validation: Local Pi 5 image generation and archive review must show `text_offset=0`, matching the Raspberry Pi 5 kernel image convention. Physical acceptance still requires a controlled hardware run that reaches the Talos entry marker or later serial output.
- Risks: If the firmware places the image at a nonzero physical base while using a zero header offset, Talos' absolute BSS/stack symbols will still be wrong; hardware evidence decides this. If that happens, the next iteration should move the earliest marker to fully position-independent code before any absolute symbol use.
- Alternatives considered: keep the generic arm64 `0x200000` offset, set `image_size=0` legacy mode, or add a custom armstub. Matching the official Pi 5 kernel image header is the narrowest project-local correction.

## 2026-05-19 - Test Pi 5 Boot Ramdisk Path

- Status: accepted
- Context: Direct TFTP boot-tree attempts repeatedly reached the same firmware/RP1 boundary before Talos entry. Raspberry Pi documentation describes `boot_ramdisk=1` as useful for network boot, where the bootloader loads a raw FAT32 `boot.img` and reads subsequent boot files from it.
- Decision: Add a bounded first-light experiment that stages `boot_ramdisk=1` and a plain FAT32 `boot.img` containing the same Talos config, DTB, overlays, and kernel images. Keep the ordinary root files in the archive as well so the lab archive contract remains satisfied.
- Required validation: Local archive review must prove `boot.img` is readable by mtools and contains `config.txt`, `kernel_2712.img`, and `kernel8.img`. Physical validation requires one controlled Pi 5 power cycle and serial evidence.
- Risks: If the firmware stops even earlier or ignores `boot_ramdisk=1` for this network path, the evidence should push the next iteration back toward bootloader/TFTP visibility or a lower-level firmware diagnostic.
- Alternatives considered: keep iterating only on the raw `kernel_2712.img`, require a lab API TFTP-log endpoint first, or add a custom armstub. The boot ramdisk path is a documented Pi 5 network-boot shape and is small enough to test safely.

## 2026-05-19 - Add a Custom Armstub Diagnostic

- Status: accepted
- Context: Direct-root, minimal-config, Pi 5 Image-header-matched, and `boot_ramdisk=1` hardware attempts all rebooted the board and emitted Raspberry Pi firmware/RP1 serial output, but none emitted Talos' `T1` entry marker. The repeated boundary suggests the next useful evidence should come before the normal `kernel_2712.img` handoff rather than from another kernel header tweak.
- Decision: Add a bounded custom armstub diagnostic path. The normal Talos boot-tree script remains unchanged; a separate staging script appends `armstub=armstub8-2712.bin` and includes a tiny AArch64 binary that writes `S1\r\n` to firmware-preserved RP1 UART0 at `0x1c00030000`, then waits.
- Required validation: Local validation must prove the armstub binary is non-empty and the archive review gate accepts the optional armstub file. Physical validation is exactly one controlled lab power-cycle under the hardware lock. `S1` on serial proves the custom armstub path ran; no `S1` keeps the investigation at the firmware/config/file-load boundary.
- Risks: A custom armstub is diagnostic-only and does not prove the normal kernel handoff. If it runs, the next step is to decide whether to evolve it into a real handoff helper or use it only to instrument the bootloader boundary. If it does not run, the issue is still earlier than that path or the Pi 5 network boot firmware ignores this armstub setting.
- Alternatives considered: require a new lab TFTP-log endpoint, keep iterating on `kernel_2712.img`, or change rollback strategy. The armstub diagnostic is small, local, and reversible, and it creates pre-entry evidence without new privileged host access.

## 2026-05-19 - Test Serial-Prefixed Network Boot Mirror

- Status: accepted
- Context: The known-good Pi OS Lite TFTP sequence probes `da591740/config.txt` before falling back to root `config.txt`. Earlier evidence says that miss was not fatal for Linux, but repeated Talos runs stop before any kernel or armstub marker. If the Talos archive shape changes the fallback behavior, a serial-prefixed mirror is a small way to test that boundary without changing the kernel image.
- Decision: Add a separate staging script that keeps the normal root boot files and duplicates the same required files under `da591740/`. The archive review gate verifies the prefixed mirror is complete and byte-identical to the root files when present.
- Required validation: Local archive review must pass and show both root and `da591740/` files. Physical validation requires one controlled lab power-cycle and serial evidence.
- Risks: If this runs, it proves the root-only tree was not equivalent in this lab network-boot path, but it does not explain why fallback differed. If it does not run, it rules out the simplest serial-prefix hypothesis and pushes the next step back toward lab-side TFTP visibility or firmware/EEPROM diagnostics.
- Alternatives considered: require direct TFTP logs, keep adding kernel diagnostics, or restore a full Pi OS Lite source tree. The prefix mirror is reversible and can be tested with existing archive tooling.

## 2026-05-19 - Stop Archive-Shape Iterations Without File-Load Evidence

- Status: accepted
- Context: Direct-root, minimal-config, Image-header-matched, `boot_ramdisk=1`, custom armstub, serial-prefix mirror, and combined serial-prefix plus armstub archives all rebooted the Pi and emitted the same Raspberry Pi firmware/RP1 serial boundary, but none emitted the `S1` armstub marker, `T1` Talos entry marker, or Talos banner.
- Decision: Stop adding new Talos archive-shape variants until the workflow has lab-side TFTP request/file-load visibility, EEPROM boot diagnostics, or a recreated known-good Pi OS Lite boot source that can be compared directly. The current evidence is pre-entry and does not justify more Rust-side or arm64 Image-header changes.
- Required validation: The next hardware-dependent step should first prove which files the Pi requests and successfully loads, or prove the known-good boot source shape that differs from the staged Talos source. Hardware claims still require one controlled Pi 5 power-cycle under the hardware lock and serial/TFTP evidence.
- Risks: This hold delays continued trial-and-error, but it avoids consuming rollback history and power cycles on low-signal variants. If new lab visibility shows the firmware is loading Talos files correctly, revisit position-independent earliest-entry code or a different UART assumption.
- Alternatives considered: continue adding config variants, restore older header/linker choices, or evolve the diagnostic armstub into a handoff helper. Those paths now have low expected value because the configured armstub itself has not produced output.

## 2026-05-19 - Separate Firmware-Preserved UART From UART Reinit

- Status: accepted
- Context: The upgraded lab API proved the Pi is served the prefixed `config.txt`, `kernel_2712.img`, DTB, overlays, `cmdline.txt`, and `armstub8-2712.bin`. The original armstub and kernel markers reinitialized PL011 before writing but did not fully mirror the Rust PL011 baud and interrupt-mask setup.
- Decision: Make the custom armstub and Pi 5 entry marker write through the firmware-preserved RP1 UART0 before changing any PL011 registers, then run the explicit PL011 init and write the existing initialized marker. The armstub now attempts `P0` then `S1`; the kernel entry marker attempts `P1` then `T1`.
- Required validation: Local validation must pass formatting, unit tests, Pi 5 target build, image generation, archive review, and QEMU smoke. Physical validation is a controlled Pi 5 power-cycle with serial and TFTP-log evidence.
- Risks: If no preserved or initialized marker appears while TFTP logs prove the armstub and kernel files were served, the next failure boundary is no longer ordinary archive layout or PL011 initialization. It points toward firmware handoff semantics, custom armstub execution assumptions, or a mismatch between loaded files and executed code.
- Alternatives considered: keep testing more archive layouts, restore older Image header fields, or require Matthew input immediately. The preserved-UART marker is a small code diagnostic that directly tests the remaining UART-reinit hypothesis.

## 2026-05-19 - Use Circle-Style Pi 5 Bare-Metal Kernel Address

- Status: accepted
- Context: After TFTP logs proved the Pi is served Talos' config, kernel, DTB, overlays, cmdline, and custom armstub, serial still stopped before any preserved-UART or initialized-UART marker. The official Raspberry Pi documentation says `kernel` selects `kernel_2712.img` on Pi 5 and `kernel_address` can control the load address. Circle's Pi 5 bare-metal `config64.txt` keeps `kernel_2712.img` and sets `kernel_address=0x80000`.
- Decision: Keep the Pi 5 arm64 Image `text_offset=0` and flags `0xc`, but link Talos at physical `0x80000` and stage `kernel_address=0x80000` in the first-light `config.txt`. Keep QEMU virt on its separate QEMU-specific link base.
- Required validation: Pi 5 target builds must show `_start` and `__kernel_start` at `0x80000`; archive review must require `kernel_address=0x80000`; physical validation requires one controlled hardware run with serial and TFTP evidence.
- Risks: This does not explain why the custom armstub marker did not appear; if the next hardware run still stops at the same boundary, the remaining issue is likely firmware handoff semantics or lab-visible UART assumptions rather than ordinary kernel load address.
- Alternatives considered: continue with `kernel_address` omitted and link at zero, add more archive-layout variants, or switch immediately to a Linux-loaded payload. The Circle-style address is a reference-backed, bounded change that can be validated locally before one hardware iteration.

## 2026-05-19 - Add Raw Pi 5 Loader Diagnostic

- Status: accepted
- Context: Circle's Pi 5 bootloader builds `kernel_2712.img` as a raw position-linked binary at `0x80000`, while Talos' normal Pi 5 image starts with an arm64 Image header before branching to code. After Circle-style `kernel_address=0x80000` still produced no marker, Matthew clarified the workflow should keep using bounded reference-backed diagnostics rather than treat the state as blocked.
- Decision: Add a separate raw loader diagnostic path that stages a tiny `kernel_2712.img` without the arm64 Image header. The diagnostic writes markers to firmware-preserved RP1 UART0, reinitialized RP1 UART0, and BCM2712 UART10, then loops with heartbeat dots. The normal Talos kernel image and boot tree remain unchanged.
- Required validation: Local validation must include shell syntax checks, raw diagnostic binary generation, archive review in `loader_diagnostic=true` mode, standard Talos formatting/tests/Pi 5 build/QEMU smoke/mdBook, and exactly one controlled Pi 5 hardware run under the hardware lock.
- Risks: Absence of RP1 UART output from a raw executable does not by itself prove CPU execution never happened, because the lab-visible UART path may be the wrong ARM-side output path despite firmware logs using it. If the raw diagnostic still shows no marker while TFTP proves the 216-byte image was served, the next diagnostic should avoid relying solely on RP1 UART visibility.
- Alternatives considered: keep changing arm64 Image header fields, require EEPROM/vclog support before continuing, or switch immediately to a Linux-loaded payload. The raw loader diagnostic is smaller and directly tests a public Pi 5 bare-metal image shape already used by Circle.

## 2026-05-19 - Make the Raw Loader Diagnostic Exception-Tolerant

- Status: accepted
- Context: The first raw loader diagnostic attempted RP1 UART0 before the BCM2712 UART10 path. If RP1 MMIO was inaccessible after firmware handoff, a synchronous abort could stop the diagnostic before it reached alternate output paths.
- Decision: Install a current-EL exception vector in the raw loader diagnostic before touching MMIO, advance `ELR_EL1` or `ELR_EL2` by one instruction on exceptions, and try BCM2712 UART10 before RP1 UART0. This keeps the diagnostic tiny while making MMIO-abort behavior observable by continued control flow.
- Required validation: Build/disassemble the raw diagnostic, pass archive review in loader-diagnostic mode, pass standard local Talos gates, then run one controlled Pi 5 hardware iteration under `hardwareTestLock`.
- Risks: Skipping faulting MMIO instructions can only keep the diagnostic moving; it cannot make an unobservable UART path visible. If this still emits no marker while TFTP proves the image was served, the next evidence needs a different side effect or a different boot path.
- Alternatives considered: switch immediately to Linux-loaded payload work, keep trying UART-only variants, or require EEPROM/vclog support. Exception-tolerant control flow is a bounded diagnostic improvement that addresses a concrete flaw in the first raw-loader attempt.

## 2026-05-19 - Add Watchdog Reset as a Non-UART Execution Signal

- Status: accepted
- Context: The exception-tolerant raw loader still emitted no UART marker. Circle's Pi 5-capable watchdog/reset path documents the power-manager watchdog registers at `ARM_IO_BASE + 0x1200000` for Pi 5, with writes to `ARM_PM_WDOG` and `ARM_PM_RSTC` causing a reset. A watchdog-triggered second firmware boot would prove CPU execution even if UART output is unavailable.
- Decision: Add a watchdog reset attempt after the raw loader's UART attempts. The hardware test must roll back the archive after observation so a successful watchdog diagnostic does not leave the Pi in a reset loop.
- Required validation: Build/disassemble the raw loader, pass archive review and standard local Talos gates, run exactly one hardware cycle, observe serial long enough for a watchdog reset, inspect TFTP evidence, then restore the previous archive.
- Risks: If the CPU never reaches the raw loader, no watchdog reset occurs. If PM watchdog MMIO is inaccessible or the reset sequence is wrong for this boot state, the result is still no side effect. A successful reset would be useful but requires immediate cleanup.
- Alternatives considered: another UART-only variant, requiring EEPROM/vclog evidence, or switching directly to a Linux-loaded payload. Watchdog reset is a small non-UART side effect available from public Pi references and fits one controlled hardware iteration.
