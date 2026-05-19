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

## 2026-05-19 - Try Linux-Derived RP1 UART0 CPU Address First

- Status: accepted
- Context: Matthew clarified that no-UART-output first-light failures should be treated as hardware-contract evidence and that correct Pi 5 offsets should make first UART output simple. A fresh Raspberry Pi Linux reference checkout shows `rp1.dtsi` declaring RP1 UART0 at RP1 bus address `0xc0_40030000`, while `bcm2712.dtsi` maps the pcie2 32-bit non-prefetchable window to CPU physical `0x1f00000000`. That implies the CPU-visible RP1 UART0 address is `0x1f00030000`. Talos had centered first-light diagnostics on `0x1c00030000`, based on earlier firmware-preserved UART evidence, and repeated hardware runs produced no marker.
- Decision: Treat `0x1f00030000` as the Linux-derived RP1 UART0 CPU address for new Pi 5 diagnostics and keep `0x1c00030000` as a fallback firmware-preserved probe. The raw loader now writes `N0`/`N1` through the Linux-derived address before trying the older `L0`/`L1` probes.
- Required validation: Local validation must show the raw diagnostic embeds both RP1 UART0 addresses, standard Talos formatting/tests/build/smoke gates pass, and one controlled Pi 5 hardware run under `hardwareTestLock` records whether `N0`, `N1`, `L0`, `L1`, or other side effects appear.
- Risks: The pcie2 non-prefetchable address may only become valid after a state transition the raw loader has not reached, or the firmware-preserved address may still be the only inherited early mapping. Trying both addresses keeps the experiment bounded.
- Alternatives considered: keep using only `0x1c00030000`, switch to a non-UART side effect immediately, or wait for vclog/EEPROM diagnostics. The source-backed address correction is the smallest productive offset experiment.
- Hardware result: The first controlled run with this diagnostic published archive `b5cb364106dae20de1a61a25fed66ef1df9f36023362ec1c443f34b44205dc90`. TFTP served the updated 4096-byte raw loader, but serial again stopped at the firmware/RP1 boundary with no `N0`/`N1`/`L0`/`L1`/`U1`/`W0` marker and no reset side effect. This rules out the Linux-derived RP1 UART0 address as sufficient by itself; the workflow remains unblocked for the next hardware-contract or handoff diagnostic.

## 2026-05-19 - Try PSCI Reset Before MMIO Watchdog

- Status: accepted
- Context: The watchdog raw loader diagnostic did not produce UART output or a second firmware boot. That leaves two different possibilities: the firmware never transfers CPU execution to the raw loader, or the loader runs in a state where both lab-visible UART and BCM2712 watchdog MMIO are ineffective.
- Decision: Add a PSCI `SYSTEM_RESET` SMC call before the MMIO watchdog reset in the diagnostic loader. This is still a diagnostic-only path and leaves the normal Talos image unchanged.
- Required validation: Build and disassemble the raw loader to confirm the SMC instruction is present, pass archive review and standard Talos local gates, run exactly one hardware cycle under `hardwareTestLock`, observe serial long enough for a possible monitor-mediated reboot, then roll back the archive.
- Risks: PSCI may be unavailable in the firmware handoff state, or the SMC may return without side effects. If no reset occurs, the result is evidence against this specific non-MMIO side channel, not proof that Talos can never execute.
- Alternatives considered: switch immediately to a Linux/UEFI-loaded payload, require EEPROM/vclog support, or keep adding UART-only markers. PSCI reset is a small, reversible diagnostic and tests a different side-effect class than RP1 UART or PM watchdog MMIO.

## 2026-05-19 - Add UEFI Intermediate-Loader Diagnostic

- Status: accepted
- Context: Direct Raspberry Pi firmware handoff has loaded Talos files but has not produced markers through UART, custom armstub, raw loader, watchdog reset, or PSCI reset. A known-running intermediate loader can separate Talos execution mechanics from the Pi firmware handoff boundary.
- Decision: Add a minimal AArch64 UEFI application diagnostic that prints `Talos EFI first-light PASS` through UEFI text output, plus a FAT-image staging script and QEMU/AAVMF smoke test. This creates a locally validated substitute payload for understanding loader behavior without changing the normal Talos kernel image.
- Required validation: The EFI file must be PE32+ AArch64 with EFI application subsystem, the FAT image must contain `EFI/BOOT/BOOTAA64.EFI` and `startup.nsh`, QEMU/AAVMF must print the PASS marker, and standard Talos gates must still pass.
- Risks: QEMU/AAVMF validation proves the payload and UEFI call path, not Pi 5 hardware execution. A physical test should not depend on an external bootloader; if this payload remains useful, Talos should reach it through a Talos-owned loader path.
- Alternatives considered: continue direct firmware diagnostics, require EEPROM/vclog support, or build a Linux/kexec path first. UEFI is useful substitute validation, but it is not the Talos hardware boot path.

## 2026-05-19 - Stage U-Boot as the UEFI Hardware Bridge

- Status: superseded
- Context: The UEFI diagnostic runs under QEMU/AAVMF, but the lab Pi 5 still boots directly from Raspberry Pi firmware into `kernel_2712.img`. A physical UEFI test needs an intermediate loader that can run on Pi 5 and launch `EFI/BOOT/BOOTAA64.EFI`.
- Decision: Superseded by project direction. Talos should develop its own kernel and bootloader from scratch; U-Boot must not be used as an implementation dependency, boot target, compatibility layer, or shortcut.
- Required validation: Remove U-Boot-specific staging from the active tool path. Continue with Talos-owned loader diagnostics, local gates, and one controlled hardware test at a time.
- Risks: External bootloader staging would hide Talos bootloader bugs and move the project away from the from-scratch kernel goal.
- Alternatives considered: use U-Boot as a bridge. Rejected because it does not match the project direction.

## 2026-05-19 - Test Raw Loader Under Circle-Style Minimal Config

- Status: accepted
- Context: Matthew clarified that Talos must own its bootloader path. The existing raw loader is Talos-owned, but previous hardware tests used the Talos first-light config with `enable_rp1_uart=1`, `pciex4_reset=0`, UART debug settings, and extra diagnostic options. Circle's Pi 5 bare-metal config is much smaller.
- Decision: Add a separate raw-loader staging path that keeps the Talos-owned loader binary but uses a Circle-style minimal Pi 5 config: `arm_64bit=1`, `kernel_address=0x80000`, `initial_turbo=0`, `[pi5]`, and `kernel=kernel_2712.img`. This tests the config-shape hypothesis without introducing an external bootloader.
- Required validation: Archive review must identify the diagnostic as `raw-pi5-circle-config`, allow the intentionally omitted RP1-preservation settings only for that diagnostic, and standard local gates must pass before exactly one hardware run under `hardwareTestLock`.
- Risks: Omitting RP1 UART preservation may make loader UART output less likely, so the meaningful hardware side effects are still firmware re-entry/reset evidence and any serial/TFTP movement. No marker still does not prove CPU execution is impossible.
- Alternatives considered: continue with U-Boot staging, repeat the prior first-light config, or wait for EEPROM/vclog evidence. The minimal-config raw loader is the smallest Talos-owned experiment that directly follows from the reference comparison.

## 2026-05-19 - Recombine Raw Loader With Debug Firmware Settings

- Status: accepted
- Context: The Circle-style minimal-config raw loader and the Linux-derived RP1 UART0 probe each stopped at the same firmware/RP1 boundary. The minimal config intentionally omitted the normal Talos first-light debug knobs, while Raspberry Pi firmware documentation says `os_check=0` is appropriate for bare-metal development, `enable_rp1_uart=1` initializes RP1 UART0, `pciex4_reset=0` preserves RP1 state, and `uart_2ndstage=1` plus `sha256=1` can increase firmware logging.
- Decision: Run one controlled hardware test that keeps the latest Talos-owned raw loader, including `0x1f00030000` and `0x1c00030000` UART probes, but stages it through the normal first-light debug config instead of the Circle-style minimal config. This tests whether the missing debug/preservation settings explain the no-marker state without introducing an external bootloader.
- Required validation: Local validation must pass formatting, tests, Pi 5 build, raw marker inspection, archive review, QEMU smoke, mdBook, and diff check before acquiring `hardwareTestLock` for exactly one Pi 5 run.
- Hardware result: Archive `4831f8acdfab1b9303c78f062190e2f149b85363f11b902b28097140bc845ff4` published and power-cycled successfully. Serial advanced `40997->41705` through Raspberry Pi firmware/RP1 logs but still showed no `N0`, `N1`, `L0`, `L1`, `U1`, `W0`, heartbeat, Talos output, or reset side effect during the 85-second observe. The TFTP cursor did not expose a fresh delta for this run, so this result is publish/power-cycle/serial evidence rather than fresh file-load proof. Rollback restored the previous archive.
- Risks: Since no marker appeared even with the debug settings restored, the likely issue is deeper than optional firmware logging or RP1 UART preservation. Repeating config-only variants is now low signal unless new reference evidence identifies a specific setting.
- Alternatives considered: repeat the Circle-style minimal config, switch immediately to a different side effect, or wait for vclog/EEPROM diagnostics. The recombined test was the smallest remaining config-shape hypothesis.

## 2026-05-19 - Cover EL3 Exceptions in Raw Loader Diagnostic

- Status: accepted
- Context: Repeated raw-loader diagnostics reached the same no-marker boundary even after UART address, config, PSCI reset, and watchdog reset variants. The raw loader installed same-EL exception vectors only for EL1 and EL2. If Raspberry Pi firmware enters a diagnostic at EL3, an early MMIO abort could bypass the skip handler before any UART or reset side effect becomes visible.
- Decision: Extend the Talos-owned raw loader diagnostic to install `VBAR_EL3` when `CurrentEL` reports EL3, and teach the skip handler to advance `ELR_EL3` before `eret`. This keeps the diagnostic from depending on an unverified firmware entry exception level.
- Required validation: Build the raw loader, inspect disassembly for `VBAR_EL3` and `ELR_EL3`, run formatting, tests, Pi 5 target build, raw marker inspection, archive review, QEMU smoke, mdBook, and diff check before one controlled hardware run.
- Hardware result: Archive `a1485244630ceef8f74f06d68a23717eab1415d4ba0475010f7b920b520f88d0` was published and power-cycled successfully. Serial advanced from cursor `41705` to `42355` through Raspberry Pi firmware logs with no `N0`, `N1`, `L0`, `L1`, `U1`, `W0`, heartbeat, Talos output, or reset side effect. Recent TFTP logs showed the usual config, DTB, overlay, cmdline, and `kernel_2712.img` requests at the boot time; rollback restored the previous archive.
- Risks: This does not prove the real entry EL. It only removes a plausible diagnostic blind spot. Since no reset side effect appeared, further direct-firmware variants should focus on entry contract or lab-visible side effects rather than more exception-level-only changes.
- Alternatives considered: assume EL1/EL2 per Linux boot protocol, repeat the same raw loader, or wait for EEPROM/vclog diagnostics. The EL3 vector change was small, local, and directly tied to the observed lack of any post-entry side effect.

## 2026-05-19 - Simplify to an Assembly-Only UART Proof

- Status: accepted
- Context: Matthew clarified that first-light should stop accumulating loader complexity until UART text is proven. The next proof should follow Daedalus' Pi 4 boot/UART shape and minimal-OS principles: firmware entry, preserve `x0` if useful, park secondary cores, and write fixed bytes through the simplest plausible UART path.
- Decision: Add a separate `asm-uart-proof` diagnostic image that is only 144 bytes of AArch64 assembly linked at `0x80000`. It preserves `x0` in `x19`, parks non-primary cores using `MPIDR_EL1`, initializes one PL011 path at Linux/Circle's RP1 UART0 physical address `0x1f00030000`, and repeatedly writes `TA\r\n`. It deliberately avoids Rust, stack setup, BSS clearing, exception vectors, PSCI, watchdog, and multi-UART fallback logic.
- Required validation: Local validation must inspect the disassembly for the small entry shape and UART polling loop, confirm the marker bytes exist, pass archive review, formatting, tests, Pi 5 build, QEMU smoke, mdBook, and diff check, then run exactly one hardware test under `hardwareTestLock`.
- Hardware result: Archive `3e405afa92020ca74c02d9e64c2b8f79711be31122b88bd21c1f2d9819f4c17b` published and power-cycled successfully. Serial advanced from cursor `42355` to `43005` through Raspberry Pi firmware logs but showed no repeated `TA` marker. The TFTP cursor did not expose a fresh delta for this run, so evidence is publish/power-cycle/serial only. A repeated rollback check toggled the one-archive rollback back to the 144-byte proof tree; the previous EL3 diagnostic archive `a1485244630ceef8f74f06d68a23717eab1415d4ba0475010f7b920b520f88d0` was republished without power-cycling so the lab boot tree was not left on the tiny proof archive.
- Risks: This is intentionally too small to recover from bad UART assumptions. A no-marker result does not prove firmware never enters the image, but it does show that the simplest direct RP1 UART0 proof still does not reach the lab-visible serial stream.
- Alternatives considered: continue extending the raw loader with exception/reset side effects, add another UART fallback, or use Rust first-light. The point of this decision is to remove those moving parts until a fixed-byte assembly marker works.

## 2026-05-19 - Mirror the Assembly UART Proof Under Serial Prefix

- Status: accepted
- Context: The first assembly-only UART proof used a root-only boot tree and did not expose a fresh TFTP delta, while many prior proven file-load runs used the Pi serial-number-prefixed `da591740/` mirror. Before changing UART code again, the smallest remaining premise was whether the simplified proof needed the same mirrored archive shape in this lab network-boot path.
- Decision: Add a prefixed staging script for the same 144-byte `asm-uart-proof` image. Keep the assembly unchanged and mirror only `config.txt`, `cmdline.txt`, `bcm2712-rpi-5-b.dtb`, `kernel_2712.img`, `kernel8.img`, and overlays under `da591740/`.
- Required validation: Local validation must pass shell syntax, proof image build, disassembly inspection, marker-byte inspection, prefixed archive review, formatting, tests, Pi 5 build, QEMU smoke, mdBook, and diff check before one controlled hardware run.
- Hardware result: Archive `ea6bf1f7c94d7a175ae018872980c0d0d6d2bf7300a99b2dceffa274f4e923e2` published and power-cycled successfully with the mirrored tree. Serial advanced from cursor `43005` to `43655` through Raspberry Pi firmware logs but still showed no repeated `TA` marker. The TFTP cursor again did not expose a fresh delta for this run, so evidence is publish/power-cycle/serial only. A repeated rollback check again toggled the one-archive rollback back to the 144-byte proof tree; the previous EL3 diagnostic archive `a1485244630ceef8f74f06d68a23717eab1415d4ba0475010f7b920b520f88d0` was republished without power-cycling to leave the lab boot tree on the 4120-byte previous diagnostic archive.
- Risks: The no-marker result now covers the simplified assembly proof with and without the serial-prefixed mirror. Since the code remained single-UART by design, the next simplification-compatible experiment should change only the UART base/preservation assumption, not add loader machinery.
- Alternatives considered: switch immediately to the firmware-preserved `0x1c00030000` UART mapping, reintroduce multi-UART fallback, or add reset side effects. Mirroring the archive shape was the smaller single-premise test after the root-only run lacked fresh TFTP evidence.

## 2026-05-19 - Test Assembly UART Proof at Firmware-Preserved RP1 Mapping

- Status: accepted
- Context: The simplified assembly proof did not produce `TA` at Linux/Circle's RP1 UART0 CPU physical address `0x1f00030000`, either root-only or mirrored under `da591740/`. Earlier firmware documentation and boot logs identified a firmware-preserved RP1 UART mapping at `0x1c00030000` when `enable_rp1_uart=1` and `pciex4_reset=0` are used.
- Decision: Keep the assembly-only proof and prefixed archive shape unchanged except for the single UART base literal, switching from `0x1f00030000` to `0x1c00030000`. This preserves Matthew's simplification constraint while testing the strongest remaining reference-backed UART address premise.
- Required validation: Local validation must pass shell syntax, proof image build, disassembly inspection showing the `0x1c` high word literal, marker-byte inspection, prefixed archive review, formatting, tests, Pi 5 build, QEMU smoke, mdBook, and diff check before one controlled hardware run.
- Hardware result: Archive `cf3ec38488a46752a52a27893a73bafecb1daf83590f0e20f17e0c606e131f80` published and power-cycled successfully. Serial advanced from cursor `43655` to `44305` through Raspberry Pi firmware logs but still showed no repeated `TA` marker. The TFTP cursor again did not expose a fresh delta for this run, so evidence is publish/power-cycle/serial only. The previous EL3 diagnostic archive `a1485244630ceef8f74f06d68a23717eab1415d4ba0475010f7b920b520f88d0` was republished without power-cycling afterward, and status confirmed the 4120-byte previous diagnostic tree.
- Risks: This result covers the two main RP1 UART0 base assumptions within the minimal assembly proof. More UART-base churn is low signal unless new reference evidence identifies a different single UART path or missing GPIO/clock step.
- Alternatives considered: add multi-UART fallback back into the proof, add exception/reset side effects, or change archive layout again. The single literal change was the smallest bounded test still aligned with the simplification policy.

## 2026-05-19 - Test Assembly UART Proof on BCM2712 UART10

- Status: accepted
- Context: The 144-byte assembly-only proof did not emit `TA` through RP1 UART0 at either `0x1f00030000` or `0x1c00030000`. Reference notes and the Talos Pi 5 target map identify Raspberry Pi 5 firmware console `serial10` / debug UART as BCM2712 UART10 at `0x107d001000`.
- Decision: Keep the assembly proof, prefixed archive shape, marker, parking loop, and no-stack/no-BSS/no-exception constraint unchanged except for the single UART base literal, switching it to `0x107d001000`. This tests the simplest Talos-owned serial10 hypothesis without adding loader complexity.
- Required validation: Local validation must pass shell syntax, proof image build, disassembly inspection showing the `0x107d001000` literal, marker-byte inspection, prefixed archive review, formatting, tests, Pi 5 build, QEMU smoke, mdBook, and diff check before one controlled hardware run.
- Hardware result: Archive `4099e52e8bc660727dd0081ddd332453e4e0b438b118065ec5c05a575f8c9f82` published and power-cycled successfully. Serial advanced from cursor `44305` to `44955` through Raspberry Pi firmware logs but still showed no repeated `TA` marker. The TFTP follow-up did not show a current 144-byte serial10 image fetch; it showed repeated earlier 4120-byte diagnostic fetches, so the useful evidence for this run is publish/power-cycle/serial rather than fresh file-load proof. The previous EL3 diagnostic archive `a1485244630ceef8f74f06d68a23717eab1415d4ba0475010f7b920b520f88d0` was republished without power-cycling afterward, and status confirmed the 4120-byte previous diagnostic tree.
- Risks: This result argues against the three obvious single-address UART paths being sufficient in the current handoff state. It does not prove the image never executes; it may still indicate a missing UART clock/reset/GPIO mux prerequisite, a firmware handoff mismatch, or lab serial being attached only to RP1 UART0 while serial10 is elsewhere.
- Alternatives considered: reintroduce multi-UART fallback, add reset/exception side effects, or pivot to another loader shape. The serial10 literal change was the last small single-UART experiment before returning to reference review of GPIO, clock, reset, and handoff premises.

## 2026-05-19 - Test Assembly UART Proof With Explicit RP1 GPIO Mux

- Status: accepted
- Context: The single-address assembly proofs covered RP1 UART0 at `0x1f00030000` and `0x1c00030000`, plus BCM2712 UART10 at `0x107d001000`, without a marker. Linux RP1 references show the 40-pin header UART path is RP1 UART0 on GPIO14/GPIO15, with RP1 GPIO control at bus `0xc0_400d0000`, pads at `0xc0_400f0000`, and GPIO14/GPIO15 selecting `uart0` at function select value 4. The corresponding pcie2 CPU physical addresses are `0x1f000d0000`, `0x1f000f0000`, and `0x1f00030000`.
- Decision: Keep the proof assembly-only and single-UART, but add the minimum Linux-derived RP1 pin setup before PL011 initialization: set GPIO14 pad input-enable/no-pull, GPIO15 pad input-enable/pull-up, set both GPIO control registers to function select 4, issue a barrier, then write `TA\r\n` through RP1 UART0 at `0x1f00030000`.
- Required validation: Local validation must inspect the disassembly for the pad/control register literals and PL011 write loop, confirm marker bytes exist, pass prefixed archive review, formatting, tests, Pi 5 build, QEMU smoke, mdBook, and diff check before one controlled hardware run.
- Hardware result: Archive `323e972a2da9bb56552d0c0a6d3abbd742e2f543d2bdbfe5a26cfbd3f29ef479` published and power-cycled successfully. Serial advanced from cursor `44955` to `45667` through Raspberry Pi firmware logs and reached a network-wait line, but no repeated `TA` marker appeared. The TFTP follow-up did not capture a current 224-byte proof-image fetch before rollback; later TFTP evidence showed the restored 4120-byte diagnostic image, so this result is publish/power-cycle/serial evidence rather than fresh file-load proof. The previous EL3 diagnostic archive `a1485244630ceef8f74f06d68a23717eab1415d4ba0475010f7b920b520f88d0` was republished without power-cycling afterward, and status confirmed the 4120-byte previous diagnostic tree.
- Risks: Writing RP1 pad/control registers assumes the pcie2 RP1 window is usable at firmware handoff. If it is not, the proof can fail before the UART write without an exception handler by design. This still tests the smallest GPIO-mux premise while preserving the simplification constraint.
- Alternatives considered: add exception recovery around GPIO writes, add multi-UART fallback, or use a non-UART side effect. Those would reintroduce loader complexity before the GPIO-mux premise had been tested in the minimal proof.

## 2026-05-19 - Capture Full TFTP Cursor Before Hardware Tests

- Status: accepted
- Context: Several recent Pi 5 UART-proof hardware runs published and power-cycled correctly, but their TFTP follow-up deltas showed stale 4120-byte diagnostic fetches instead of the current tiny proof image. Review found the lab endpoint defaults to a 64 KiB `max_bytes` window; `/tftp/logs?cursor=0&limit=1` returned `cursor_end=65536` even though the real EOF cursor was already beyond `215000`.
- Decision: Add `scripts/rpi5-tftp-cursor.sh` and require it, or an equivalent `/tftp/logs?cursor=0&max_bytes=1048576&limit=1` call, to capture the pre-run TFTP EOF cursor before controlled hardware tests. Also use `scripts/rpi5-wait-tftp-delta.sh <cursor>` after power-cycle because `/serial/observe` can return after the first serial burst before the Pi reaches the network/TFTP phase. Treat prior stale or empty deltas as an evidence-collection flaw, not as proof the hardware tests did or did not fetch the current archive.
- Required validation: The helpers must pass shell syntax validation. The cursor helper must return the same current EOF cursor as the expanded TFTP log query before it is used as a hardware-test gate, and the wait helper must be used before rollback in the next hardware run.
- Hardware follow-up: A wait-for-TFTP rerun of the 224-byte RP1 GPIO-mux assembly UART proof captured fresh TFTP events for `da591740/kernel_2712.img` at 224 bytes before rollback. The serial output still stopped at the same firmware/DDR boundary with no `TA` marker, so the current archive is now proven fetched even though the UART proof is still not visible.
- Risks: If the TFTP log grows past 1 MiB, this helper can become truncated again. If that happens, use the endpoint's large-cursor clamp behavior or add a lab API cursor endpoint rather than relying on the default window.
- Alternatives considered: ignore TFTP deltas and use only serial evidence, or continue manually tuning cursor requests per run. A small helper gives repeatable evidence without changing the Talos boot image.

## 2026-05-19 - Add Minimal Image Header to Assembly UART Proof

- Status: accepted
- Context: The wait-for-TFTP hardware run proved the current 224-byte assembly UART proof is fetched as `da591740/kernel_2712.img`, but the lab UART still shows no `TA` marker. Circle's Pi 5 examples use raw binaries, but the Linux arm64 boot ABI and Talos' normal kernel image use an arm64 Image header with magic `ARMd`, size, text offset, and flags.
- Decision: Keep the first-light proof assembly-only and direct-entry, but prepend the minimal arm64 Image header before branching to the existing UART proof code. The header advertises `text_offset=0`, exact image size, flags `0xc`, and magic `ARMd`. This tests a firmware image-contract hypothesis without adding Rust, stack, BSS, exception handling, reset side effects, or loader machinery.
- Required validation: Local validation must inspect the generated header, confirm the marker bytes still exist, pass archive review, mdBook, and diff check before any future hardware run.
- Hardware result: Archive `6fb98f25ed3d43aaf501cd156e8fa523d00ff4f76ee515bcbb44d1a6666079b6` was published and power-cycled successfully. Corrected TFTP evidence captured cursor `351353->352704` with 13 fresh events, including served `da591740/kernel_2712.img` at 288 bytes twice, the prefixed config, DTB, overlays, and cmdline. Serial observed only a trailing NUL/newline from cursor `46965->46967` and no repeated `TA` marker. Rollback restored the previous boot tree with hash `02a1311a6419ca764a2b19b2a34e4ad1b71e74972c6484e7b5620fc8018ec7d9`.
- Risks: If Raspberry Pi firmware happily boots raw binaries, this will not change behavior. If firmware uses the header for placement or validation in this TFTP path, the header may be the missing contract needed before entry.
- Alternatives considered: keep repeating raw 224-byte proof runs, add more UART-side setup, or reintroduce loader diagnostics. The header is a smaller single-premise change now that fresh TFTP evidence proves the tiny image is fetched.
