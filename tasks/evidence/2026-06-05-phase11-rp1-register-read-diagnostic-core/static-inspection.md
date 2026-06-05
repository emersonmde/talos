# Static Inspection: RP1 UART0 FR Read Diagnostic Core

Task: `phase11-rp1-register-read-diagnostic-core-20260605`

## Scope Reviewed

- `build.rs`: registers only the explicit `rpi5_rp1_uart0_fr_read` boot scenario.
- `src/boot/rpi5.rs`: calls the diagnostic only under that boot scenario.
- `src/target/rpi5.rs`: defines `RP1_UART0_FR = 0x1f_0003_0018`, performs a single 32-bit volatile read, reports the contract id, target, address, width, raw value, success classification, and PASS.
- `scripts/rpi5-rp1-uart0-fr-read-image.sh`: builds the focused candidate image without publishing.
- `scripts/rpi5-rp1-uart0-fr-read-boot-tree.sh`: stages the focused image into the standard Pi 5 boot-tree shape for the later serialized hardware proof.

## Disposition

- fixed: the implementation is gated by `TALOS_BOOT_SCENARIO=rpi5_rp1_uart0_fr_read` and does not change the normal shell/VFS scenarios.
- fixed: the diagnostic adds no new RP1 writes; the new operation is one 32-bit volatile read from `0x1f_0003_0018`.
- fixed: the serial output includes `phase11-rp1-pcie-map-contract-v1`, `rp1-uart0-fr-read`, address, width, raw value, `classification=mapped/read-value`, and PASS.
- deferred: bus-fault/trap and firmware-state classifications require the serialized Pi 5 proof task because this local core does not run hardware or install a recoverable data-abort probe.
- not-an-issue: existing Pi 5 target initialization still preserves the accepted first-light UART path; this task adds no new pin-control, clock, reset, interrupt, DMA/cache, networking, SSH, storage, generated-root, or shell behavior.

## Evidence Level

Static inspection and local build/image review only. No hardware lock, archive publication, TFTP observation, serial hardware run, or power cycle was performed.
