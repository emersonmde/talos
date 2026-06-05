# Phase 11 RP1/PCIe Map Contract

This contract defines the first Talos RP1/PCIe mapping slice. It is a source-backed contract for one narrow read-only diagnostic; it does not claim a driver, GPIO ownership, interrupt routing, DMA/cache policy, Ethernet, networking, SSH, or generated-root progress.

## Inputs

- Raspberry Pi Linux `rpi-6.12.y` device-tree sources retained under `tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/`: `bcm2712.dtsi`, `bcm2712-rpi-5-b.dts`, `bcm2712-rpi.dtsi`, and `rp1.dtsi`.
- Existing Talos reference notes for Pi 5, RP1 UART0, and RP1 pin-control bring-up evidence.
- Lab-controller status from `GET /status` showing the current boot tree is restored and the boot config includes `enable_rp1_uart=1`.
- Talos' existing PL011 model, where the flag register is at offset `0x18`.

## Address Contract

Linux `bcm2712.dtsi` defines the Pi 5 `pcie2` non-prefetchable PCIe window:

~~~text
PCIe 00_00000000 -> CPU physical 0x1f_0000_0000
size             -> 0xffff_fffc bytes in the source range
~~~

Linux `bcm2712-rpi-5-b.dts` supplies the RP1 child range under `pcie2`:

~~~text
RP1 bus 0xc0_40000000 -> PCIe 00_00000000
size                  -> 0x0041_0000 bytes
~~~

For this first slice Talos treats the initial RP1 peripheral CPU mapping as:

~~~text
cpu_phys = 0x1f_0000_0000 + (rp1_bus - 0xc0_4000_0000)
~~~

Relevant translated addresses:

| RP1 object | RP1 bus address | CPU physical address | Width | Use |
| --- | ---: | ---: | ---: | --- |
| RP1 UART0 PL011 base | `0xc0_4003_0000` | `0x1f_0003_0000` | 32-bit MMIO | Parent block only |
| RP1 UART0 PL011 flag register | `0xc0_4003_0018` | `0x1f_0003_0018` | 32-bit read | First diagnostic target |
| RP1 GPIO bank0 control base | `0xc0_400d_0000` | `0x1f_000d_0000` | 32-bit MMIO | Documented only |
| RP1 pads bank0 base | `0xc0_400f_0000` | `0x1f_000f_0000` | 32-bit MMIO | Documented only |

## Firmware-State Assumptions

The first diagnostic depends on firmware/boot configuration leaving RP1 and UART0 mapped and clocked enough for a non-destructive flag-register read. The current lab boot config reports `enable_rp1_uart=1`, and historical Talos first-light work already used RP1 UART0 through the `pcie2` address path.

The next task must not configure GPIO14/GPIO15, change UART baud/programming, touch RP1 clocks or resets, enable interrupts, or allocate DMA. If the read faults or returns an implausible value, classify that as evidence about this firmware-state assumption rather than expanding the diagnostic in place.

## Diagnostic Contract

The next diagnostic-core task may implement only this first read:

~~~text
name: rp1-uart0-fr-read
address: 0x1f_0003_0018
width: 32-bit volatile little-endian load
source target: RP1 UART0 PL011 flag register
expected success class: mapped/read-value
~~~

The diagnostic should report:

- the source contract id `phase11-rp1-pcie-map-contract-v1`;
- the target name, address, width, and raw value;
- a classification: `mapped/read-value`, `bus-fault/trap`, `firmware-state-dependency`, or `staging/build-blocker`;
- enough serial text to tie the output to the candidate artifact in the later Pi 5 proof task.

The accepted local core may add build/static/archive-review evidence only. The serialized Pi 5 proof remains a separate task that must acquire `hardwareTestLock`, capture candidate identity, serial cursor, TFTP delta, and restore evidence.

## Pi 5 Proof Status

`phase11-rp1-register-read-pi5-proof-20260605` completed with a hardware
blocker, not a mapping acceptance. The lab published candidate tree
`a96f0d8dc17a4872cb52e94c37c85d5adc5312255d26f988dbd8b71e1b6118c9`; TFTP
served the selected 87,392-byte `da591740/kernel_2712.img` before restore in
two candidate runs. Neither run reached `rpi5-rp1-uart0-fr-read`,
`mapped/read-value`, or `PASS` serial output from a fresh cursor. A
known-good control on the restored accepted tree
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` retained
`TALOS: kernel_main` and accepted command-loop output, so the proof boundary
is `blocked-pre-entry-or-handoff-after-candidate-fetch`.

Do not treat this as evidence that the RP1 UART0 flag register is mapped or
unmapped. The next source-level investigation or revised diagnostic shape must
be supervisor-planned before changing RP1 constants or broadening into GPIO,
interrupt, DMA/cache, networking, SSH, or storage work.

`phase11-rp1-diagnostic-entry-pi5-proof-20260605` reran the revised
pre-MMIO-marker candidate after the source-level handoff task. The candidate
archive SHA-256 was
`2640ab9ceabee343ee1426b7137e1597687517f56d3b61f58a7ac0e7ab4b6608`, and the
published boot tree was
`0b25c8e08b7cdbac0447ee80a962ed7ee0fa9d219eafc3f060cfcd902c035511`. After
mandatory inconclusive-run triage, the known-good control restored
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`, fetched
the 104,136-byte control kernel, reached `TALOS: kernel_main`, and retained
PASS output. The candidate rerun fetched the selected 87,480-byte
`da591740/kernel_2712.img` twice before restore, but serial output still did
not reach `TALOS: kernel_main`, `rpi5-rp1-uart0-fr-read: start`,
`rpi5-rp1-uart0-fr-read: pre-mmio-read`, `mapped/read-value`, or `PASS`.
The classification remains `blocked-pre-entry-or-handoff-after-candidate-fetch`;
the proof does not accept RP1 mapped or unmapped behavior.

## Diagnostic Core Implementation

The local diagnostic core is compiled only when `TALOS_BOOT_SCENARIO=rpi5_rp1_uart0_fr_read` is selected. That path first reports `rpi5-rp1-uart0-fr-read: start` and `rpi5-rp1-uart0-fr-read: pre-mmio-read`, flushes UART10, then reads exactly `RP1_UART0_FR` (`0x1f_0003_0018`) with one 32-bit volatile load. A returned read reports the contract id, target name, address, width, raw value, `mapped/read-value` success classification, and PASS before returning to the existing final halt path. The pre-MMIO marker is a discriminator for the next serialized proof: if hardware reaches that marker but not the read-value line, the result is entry/handoff reachability plus an RP1 read trap/hang boundary, not a mapping acceptance.

The diagnostic does not add raw assembly early-entry UART markers. Prior Phase 10 evidence quarantined that path from prompt-capable Pi 5 controls after it made accepted controls fail, so this slice keeps the marker inside the existing Rust/serial path. It does not add GPIO, pin-control, clock, reset, interrupt, DMA/cache, Ethernet, networking, SSH, storage, generated-root, or shell behavior.

Artifact helpers:

- `scripts/rpi5-rp1-uart0-fr-read-image.sh` builds the candidate image.
- `scripts/rpi5-rp1-uart0-fr-read-boot-tree.sh` stages the candidate image into a Pi 5 boot tree for the later serialized hardware proof.

## Deferred Work

- PCIe enumeration and BAR discovery for general external devices.
- Talos-owned RP1 reset, clock, GPIO, pinctrl, UART initialization, and interrupt routing.
- DMA addressability, cache maintenance, IOMMU policy, Ethernet, networking, SSH, storage drivers, and writable persistent filesystems.
- Any fix for the Phase 10 Pi 5 generated-root firmware-initramfs overlap blocker.
