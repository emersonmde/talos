# Reference Notes

This page tracks sources that should guide Talos planning and implementation. Keep it current as sources are confirmed, corrected, or replaced.

## Primary Sources

Raspberry Pi 5 product documentation:

- Raspberry Pi 5 datasheet: https://www.raspberrypi.com/documentation/hardware/raspberrypi/5/datasheet.pdf
- BCM2712 processor notes: https://raw.githubusercontent.com/raspberrypi/documentation/master/documentation/asciidoc/computers/processors/bcm2712.adoc

Raspberry Pi firmware, boot, and configuration documentation:

- Raspberry Pi boot order documentation: https://www.raspberrypi.com/documentation/configuration/boot-order.md
- Raspberry Pi configuration documentation: https://www.raspberrypi.com/documentation/computers/config_txt.html
- EEPROM boot flow: https://raw.githubusercontent.com/raspberrypi/documentation/master/documentation/asciidoc/computers/raspberry-pi/bootflow-eeprom.adoc
- EEPROM bootloader notes: https://raw.githubusercontent.com/raspberrypi/documentation/master/documentation/asciidoc/computers/raspberry-pi/boot-eeprom.adoc
- Boot configuration reference: https://raw.githubusercontent.com/raspberrypi/documentation/master/documentation/asciidoc/computers/config_txt/boot.adoc
- PCIe documentation: https://raw.githubusercontent.com/raspberrypi/documentation/master/documentation/asciidoc/computers/raspberry-pi/pcie.adoc

QEMU Raspberry Pi board documentation:

- QEMU Raspberry Pi boards: https://www.qemu.org/docs/master/system/arm/raspi.html

Raspberry Pi Linux device trees:

- Pi 5 board DTS: https://github.com/raspberrypi/linux/blob/rpi-6.12.y/arch/arm64/boot/dts/broadcom/bcm2712-rpi-5-b.dts
- BCM2712 DTSI: https://github.com/raspberrypi/linux/blob/rpi-6.12.y/arch/arm64/boot/dts/broadcom/bcm2712.dtsi
- Pi 5 common DTSI: https://github.com/raspberrypi/linux/blob/rpi-6.12.y/arch/arm64/boot/dts/broadcom/bcm2712-rpi.dtsi
- BCM2712 downstream DTSI: https://github.com/raspberrypi/linux/blob/rpi-6.12.y/arch/arm64/boot/dts/broadcom/bcm2712-ds.dtsi
- RP1 DTSI: https://github.com/raspberrypi/linux/blob/rpi-6.12.y/arch/arm64/boot/dts/broadcom/rp1.dtsi
- RP1 peripherals datasheet: https://pip.raspberrypi.com/documents/RP-008370-DS-rp1-peripherals.pdf

ARM architecture:

- ARM Architecture Reference Manual for A-profile architecture: https://developer.arm.com/documentation/ddi0487/latest
- AArch64 instruction set reference: https://developer.arm.com/documentation/ddi0602/latest
- Linux arm64 boot ABI: https://raw.githubusercontent.com/torvalds/linux/master/Documentation/arch/arm64/booting.rst
- Cortex-A76 technical reference material should be used for CPU-specific behavior when needed.
- ARM GIC architecture and generic timer references still need exact primary links before implementation.

Rust kernel development:

- Writing an OS in Rust: https://os.phil-opp.com/
- Rust embedded book: https://docs.rust-embedded.org/book/
- Rustonomicon, for unsafe Rust constraints: https://doc.rust-lang.org/nomicon/

These are advisory design references, not project requirements. Talos should use
their proven patterns where appropriate, especially around no_std, custom test
harnesses, panic paths, allocators, paging, and interrupt setup, while adapting
them to AArch64 and Raspberry Pi 5 hardware instead of copying x86 assumptions.

Linux, Raspberry Pi firmware, and bare-metal source areas to index before implementation:

- Linux Cadence MACB/GEM Ethernet driver.
- Linux GICv2 and ARM generic timer drivers.
- Linux Raspberry Pi pinctrl/GPIO and RP1 support.
- Linux DMA mapping and cache-maintenance paths relevant to arm64 noncoherent devices.
- Any available Cadence GEM/macb public programming reference.

Local Daedalus references:

- Daedalus roadmap: /opt/strider/openclaw/current/workspace/projects/daedalus/docs/src/roadmap.md
- Daedalus testing strategy: /opt/strider/openclaw/current/workspace/projects/daedalus/docs/src/architecture/testing.md
- Daedalus Raspberry Pi references: /opt/strider/openclaw/current/workspace/projects/daedalus/docs/src/references/raspberry-pi.md
- Daedalus target/toolchain files: .cargo/config.toml, aarch64-daedalus.json, build.rs, linker.ld, and scripts/qemu-runner.sh

## Current Hardware Findings

- Raspberry Pi 5 uses BCM2712 and Cortex-A76-class CPUs.
- BCM2712 is documented as quad-core Arm Cortex-A76, Armv8-A, up to 2.4 GHz, with 64 KiB I/D L1, 512 KiB L2 per core, and 2 MiB shared L3.
- Pi 5 uses EEPROM boot. The firmware loads the kernel directly; start.elf is not the Pi 5 kernel-loading path.
- Pi 5 firmware defaults to kernel_2712.img and falls back to kernel8.img. Pi 5 is 64-bit only for kernel boot.
- The `armstub` config option names a boot-partition file containing a small Arm stub that runs before the kernel. The Pi 5 firmware has an embedded default stub, but a configured custom stub is still a useful diagnostic for whether config parsing and pre-kernel handoff are reached.
- `os_prefix` applies to operating-system files such as kernels, initramfs, cmdline, DTBs, and overlays; firmware first tests a prefix for viability using key files and ignores it if the expected kernel and DTB are not found. This makes a serial-prefixed mirror useful only if it includes the config-selected kernel and DTB.
- The arm64 boot ABI passes the physical DTB address in x0, with interrupts masked and MMU off. Non-secure EL2 is preferred, EL1 is allowed.
- TF-A's Raspberry Pi 5 BL31 port describes the Pi 5 handoff as a minimal BL31 implementation for 64-bit EL2 payloads. Its direct-Linux path reads the firmware-filled kernel entry and DTB fields from the armstub header, then passes the DTB address in x0 and zeros x1-x3 before entering BL33.
- Linux Pi 5 DTS declares compatible values for raspberrypi,5-model-b and brcm,bcm2712.
- Pi 5 uses RP1 as a major I/O controller behind PCIe. RP1 owns practical peripherals including GPIO, UARTs, SPI, I2C, Ethernet, USB, SDIO, PWM, and DMA.
- UART has two important paths: firmware console serial10 maps to the Pi 5 debug UART path; the 40-pin header UART is RP1 UART0. The lab USB serial cable observes the 40-pin login prompt. Linux's Pi 5 DTS maps RP1 UART0 from RP1 bus address `0xc0_40030000` through pcie2 non-prefetchable space to CPU physical `0x1f00030000`; the raw loader now tries that address before falling back to the previously assumed firmware-preserved `0x1c00030000` probe. Raspberry Pi firmware documentation still says `enable_rp1_uart=1` initializes RP1 UART0 at 115200 and `pciex4_reset=0` prevents resetting RP1 before OS entry.
- Linux's RP1 pinctrl facts for the header UART are: GPIO14/GPIO15 use `function = "uart0"`; `pinctrl-rp1.c` maps that to function-select value 4 on both pins; bank0 GPIO control registers are at RP1 bus `0xc0_400d0000` plus `pin * 8 + 4`; pad registers are at `0xc0_400f0000` plus `4 + pin * 4`. Through the pcie2 CPU window, the minimal assembly GPIO-mux proof writes pad/control registers at `0x1f000f003c`, `0x1f000f0040`, `0x1f000d0074`, and `0x1f000d007c` before writing RP1 UART0.
- BCM2712 exposes GIC-400 / GICv2, and the architectural timer PPIs are the first timer-interrupt path to bring up.
- Raspberry Pi Linux device tree advertises PSCI 1.0 with SMC and cpu_on 0xc4000003. PSCI should be the primary SMP bring-up path.
- External bootloaders are not Talos implementation dependencies or hardware boot targets. They may be inspected only as reference material for public hardware contracts and boot behavior.
- RP1 Ethernet appears as rp1_eth, compatible with raspberrypi,rp1-gem and cdns,macb.
- SD card runtime access is BCM2712 SDHCI; NVMe requires PCIe root complex plus NVMe driver and should not be the first persistent-storage path.
- The current lab TFTP boot sequence successfully requests Pi 5 files including config.txt, bcm2712-rpi-5-b.dtb, kernel_2712.img, initramfs_2712, overlays, and cmdline.txt.
- Lab TFTP evidence must capture the pre-run cursor with the full log window. Calling `/tftp/logs?cursor=0&limit=1` uses the endpoint default `max_bytes=65536`, so once logs exceed 64 KiB the returned `cursor_end` is a truncated-window boundary rather than the current EOF cursor. Use `scripts/rpi5-tftp-cursor.sh`, or explicitly request `max_bytes=1048576`, before each hardware run. After power-cycle, do not rollback immediately after the first serial burst; `/serial/observe` can return before the network/TFTP phase. Poll `scripts/rpi5-wait-tftp-delta.sh <cursor>` until fresh TFTP events appear or the bounded timeout expires. Several recent no-marker UART-proof follow-up reads are therefore reclassified as stale or premature TFTP-delta collection, while their publish, power-cycle, and serial evidence remains valid.
- Direct-root, `boot_ramdisk=1`, root-only armstub, serial-prefixed mirror, combined prefix-plus-armstub, and preserved-UART marker Talos archives all stop at the same firmware/RP1 serial boundary. TFTP logs now prove the Pi is served the selected config, kernel, DTB, overlays, cmdline, and custom armstub files.
- Circle's Raspberry Pi 5 bare-metal configuration keeps `kernel_2712.img` but sets `kernel_address=0x80000`. Talos now uses the same firmware load base while keeping arm64 Image `text_offset=0`, so post-marker absolute symbols match a reference bare-metal handoff convention.
- The current assembly-only first-light proof deliberately has no Rust, stack setup, BSS clearing, exception vectors, MPIDR filtering, or broad fallback logic. It starts with a minimal arm64 Image header, writes a marker to BCM2712 `uart10`, configures RP1 GPIO14/GPIO15 for UART0, then repeatedly writes fixed bytes to RP1 UART0.
- The current non-UART entry discriminator is `asm-entry-reset-proof`: a 96-byte assembly-only Image that preserves x0, performs no MMIO, and calls PSCI `SYSTEM_RESET`. It is intended to prove or disprove BL33 entry independently of RP1 UART.
- The `asm-entry-reset-firmware-address` variant uses the same 96-byte Image but removes `kernel_address=0x80000` to let Pi 5 firmware choose placement while keeping Image `text_offset=0`, matching the official Pi 5 Image convention more closely than the Circle-style forced-address path. Its hardware run served the 96-byte image and then produced a second TFTP boot sequence before restore, so the next UART marker proof should use the firmware-selected Image placement rather than the forced Circle-style address.
- The `asm-uart-proof-firmware-address` variant uses the same no-`kernel_address` Image placement as the successful reset discriminator. The direct firmware-preserved RP1 UART0 form served the 128-byte image but produced no `TA` marker. The follow-up RP1 GPIO14/GPIO15 mux form served the 208-byte image and reached BL31 notices, but still produced no `TA`. The successful 272-byte variant added readback flushing after RP1 pin-control writes and after each UART data write, matching Linux's RP1 posted-write pattern; the lab then printed repeated `TA` on the 40-pin serial header after BL31 handoff.
- A Rust-sized padded variant of the successful UART proof was built as an 83,304-byte arm64 Image, matching the current normal Rust kernel image size while keeping the same assembly entry and RP1 readback-flush UART writes. Hardware served `da591740/kernel_2712.img` at 83,304 bytes twice and printed repeated `TP` on serial after BL31. This rules out raw Image size/TFTP transfer length as the immediate reason the normal Rust image does not currently reach its boot markers; the next boundary is the Rust image layout/linker/startup contract.
- A follow-up normal Rust image hardware run mirrored the successful proof's initial BCM2712 `uart10` write before the RP1 pinmux/UART marker path and used the same 83,304-byte Image size and serial-prefixed boot tree. Hardware served `da591740/kernel_2712.img` at 83,304 bytes twice but serial showed only NUL/newline and no `B0`, `B1`, `B2`, or Talos banner before restore. Because the padded assembly image of the same size enters and prints, the remaining difference is in the normal Rust-linked image contents or boot-tree metadata rather than size alone.
- A linker-layout assembly proof then used Talos' normal Pi 5 linker script shape: `.text.boot` at `0x80000`, vectors at `0x80800`, `.text` at `0x81000`, `.rodata` at `0x91000`, and the same 83,304-byte Image size as the Rust kernel. Hardware served `da591740/kernel_2712.img` at 83,304 bytes twice and printed repeated `TL` after BL31. This rules out normal linker section gaps, image padding, and rodata placement as the Rust marker failure cause; focus next on the normal Rust boot marker code path and early startup instructions.
- Replacing the normal Rust image's entry marker with a closer copy of the successful assembly UART sequence still did not print: hardware served the 83,304-byte Rust image twice and serial showed only NUL/newline, with no `BR` marker or Talos banner. Since the standalone linker-layout assembly image with the same addresses and size prints `TL`, the remaining difference is likely Cargo/Rust link composition or early executable contents around the boot object rather than the boot tree, Image size, or section addresses alone.
- A public BCM2712 peripherals PDF does not appear to exist at the expected datasheets.raspberrypi.com path; Raspberry Pi Linux DTS files are currently the most practical register-map source for BCM2712.

## Current QEMU Findings

- QEMU upstream Raspberry Pi docs list raspi0, raspi1ap, raspi2b, raspi3ap, raspi3b, and raspi4b.
- No raspi5, bcm2712, or RP1 machine model is available in the checked local QEMU versions.
- QEMU raspi4b is incomplete for Pi 4 networking and PCIe and should not be used as Pi 5 validation.
- QEMU virt with -cpu cortex-a76 is the useful emulator path for generic AArch64 work.
- QEMU virt with AAVMF/QEMU_EFI can run Talos' minimal AArch64 UEFI diagnostic from a FAT image and prints `Talos EFI first-light PASS`. This is substitute validation for loader-call mechanics, not physical Pi 5 validation or a hardware boot plan.

## Open Questions

- Exact firmware handoff state on the lab Pi 5: exception level, cache/MMU state, DTB pointer convention, and secondary core state.
- Exact UART clock and initialization requirements for the Pi 5 debug UART when firmware has already enabled it.
- Whether Talos should parse the firmware-provided DTB early or compile a static Pi 5 board map first and add fuller DTB parsing later. The boot ABI means the DTB pointer must at least be preserved from x0.
- Best first networking path: direct RP1/Cadence GEM driver, staged non-Ethernet transport, or another narrow bring-up path.
- Persistent filesystem path: SD, USB mass storage, NFS root, generated image root, or staged initramfs-first approach.
- SSH implementation path: no_std crate feasibility, entropy source, key storage, authentication policy, and time requirements.
