# Phase 11 RP1/PCIe Mapping Source Inspection Notes

Task: `phase11-rp1-pcie-map-source-contract-20260605`

Evidence level: static source/docs/reference inspection plus lab-controller API status read.

## Source Files Retained

- `bcm2712.dtsi`: Raspberry Pi Linux `rpi-6.12.y` BCM2712 device tree.
- `bcm2712-rpi-5-b.dts`: Raspberry Pi Linux `rpi-6.12.y` Pi 5 board device tree.
- `bcm2712-rpi.dtsi`: Raspberry Pi Linux `rpi-6.12.y` Pi common device tree include.
- `rp1.dtsi`: Raspberry Pi Linux `rpi-6.12.y` RP1 device tree include.

## Findings

- Fixed: selected a first diagnostic target that is read-only and already source-backed: RP1 UART0 PL011 flag register. Linux `rp1.dtsi` declares RP1 UART0 as `compatible = "arm,pl011-axi"` with `reg = <0xc0 0x40030000 0x0 0x100>` and PrimeCell periphid `0x00341011`.
- Fixed: reconciled the CPU physical address translation. Linux `bcm2712.dtsi` maps PCIe2 non-prefetchable PCI address `00_00000000` to CPU physical `0x1f_0000_0000`. Linux `bcm2712-rpi-5-b.dts` maps RP1 peripheral bus `0xc0_40000000..0xc0_4040ffff` to that same PCIe non-prefetchable window. Therefore RP1 UART0 `0xc0_40030000` is CPU physical `0x1f_0003_0000`, and its PL011 flag register at offset `0x18` is CPU physical `0x1f_0003_0018`.
- Fixed: recorded the firmware-preserved-state assumption. The current lab boot config reports `enable_rp1_uart=1` and `kernel=kernel_2712.img` via `GET /status`. That supports a first read-only proof against RP1 UART0 without Talos taking GPIO, pinmux, clock, reset, interrupt, or DMA ownership.
- Deferred: exact RP1 clock/reset ownership, interrupt routing, GPIO ownership, DMA/cache policy, and Ethernet/networking remain outside this contract.
- Not-an-issue: the diagnostic may report a variable flag-register value. Acceptance should classify a successful run as a mapped/read-value result, not require one exact bit pattern. A bus fault/trap is also decisive evidence against the current mapping or firmware-state assumption.

## Commands

~~~bash
curl -fsSL https://raw.githubusercontent.com/raspberrypi/linux/rpi-6.12.y/arch/arm64/boot/dts/broadcom/bcm2712.dtsi \
  -o tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712.dtsi
curl -fsSL https://raw.githubusercontent.com/raspberrypi/linux/rpi-6.12.y/arch/arm64/boot/dts/broadcom/bcm2712-rpi-5-b.dts \
  -o tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi-5-b.dts
curl -fsSL https://raw.githubusercontent.com/raspberrypi/linux/rpi-6.12.y/arch/arm64/boot/dts/broadcom/bcm2712-rpi.dtsi \
  -o tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi.dtsi
curl -fsSL https://raw.githubusercontent.com/raspberrypi/linux/rpi-6.12.y/arch/arm64/boot/dts/broadcom/rp1.dtsi \
  -o tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/rp1.dtsi
curl -fsS http://talos-lab-api:8080/status
~~~

## Lab Status Snapshot

- `hardwareTestLock`: unlocked/restored in supervisor state.
- `GET /status` boot config: `kernel=kernel_2712.img`, `enable_rp1_uart=1`, `os_check=0`.
- `effective_kernel`: `kernel_2712.img`.
- `tree_hash`: `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.
