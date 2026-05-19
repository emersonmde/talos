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

- Status: accepted
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
