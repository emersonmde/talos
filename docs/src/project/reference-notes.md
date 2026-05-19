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

Linux and U-Boot source areas to index before implementation:

- Linux Cadence MACB/GEM Ethernet driver.
- Linux GICv2 and ARM generic timer drivers.
- Linux Raspberry Pi pinctrl/GPIO and RP1 support.
- Linux DMA mapping and cache-maintenance paths relevant to arm64 noncoherent devices.
- U-Boot Raspberry Pi 5 board code and boot flow.
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
- The arm64 boot ABI passes the physical DTB address in x0, with interrupts masked and MMU off. Non-secure EL2 is preferred, EL1 is allowed.
- Linux Pi 5 DTS declares compatible values for raspberrypi,5-model-b and brcm,bcm2712.
- Pi 5 uses RP1 as a major I/O controller behind PCIe. RP1 owns practical peripherals including GPIO, UARTs, SPI, I2C, Ethernet, USB, SDIO, PWM, and DMA.
- UART has two important paths: firmware console serial10 maps to the Pi 5 debug UART path; the 40-pin header UART is RP1 UART0. The lab USB serial cable observes the 40-pin login prompt, so first-light Talos output should use RP1 UART0 through the firmware-preserved RP1 mapping at physical 0x1c00030000 with initialization preserved by enable_rp1_uart=1 and pciex4_reset=0. Linux's RP1 DTS describes UART0 as RP1 bus offset 0xc0_40030000; Raspberry Pi firmware reports the preserved RP1 UART MMIO address as 0x0000001c00030000 when enable_rp1_uart=1.
- BCM2712 exposes GIC-400 / GICv2, and the architectural timer PPIs are the first timer-interrupt path to bring up.
- Raspberry Pi Linux device tree advertises PSCI 1.0 with SMC and cpu_on 0xc4000003. PSCI should be the primary SMP bring-up path.
- RP1 Ethernet appears as rp1_eth, compatible with raspberrypi,rp1-gem and cdns,macb.
- SD card runtime access is BCM2712 SDHCI; NVMe requires PCIe root complex plus NVMe driver and should not be the first persistent-storage path.
- The current lab TFTP boot sequence successfully requests Pi 5 files including config.txt, bcm2712-rpi-5-b.dtb, kernel_2712.img, initramfs_2712, overlays, and cmdline.txt.
- A public BCM2712 peripherals PDF does not appear to exist at the expected datasheets.raspberrypi.com path; Raspberry Pi Linux DTS files are currently the most practical register-map source for BCM2712.

## Current QEMU Findings

- QEMU upstream Raspberry Pi docs list raspi0, raspi1ap, raspi2b, raspi3ap, raspi3b, and raspi4b.
- No raspi5, bcm2712, or RP1 machine model is available in the checked local QEMU versions.
- QEMU raspi4b is incomplete for Pi 4 networking and PCIe and should not be used as Pi 5 validation.
- QEMU virt with -cpu cortex-a76 is the useful emulator path for generic AArch64 work.

## Open Questions

- Exact firmware handoff state on the lab Pi 5: exception level, cache/MMU state, DTB pointer convention, and secondary core state.
- Exact UART clock and initialization requirements for the Pi 5 debug UART when firmware has already enabled it.
- Whether Talos should parse the firmware-provided DTB early or compile a static Pi 5 board map first and add fuller DTB parsing later. The boot ABI means the DTB pointer must at least be preserved from x0.
- Best first networking path: direct RP1/Cadence GEM driver, staged non-Ethernet transport, or another narrow bring-up path.
- Persistent filesystem path: SD, USB mass storage, NFS root, generated image root, or staged initramfs-first approach.
- SSH implementation path: no_std crate feasibility, entropy source, key storage, authentication policy, and time requirements.
