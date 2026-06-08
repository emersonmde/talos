# Static Reconciliation Notes

Task:
phase11-rp1-clock-sentinel-address-discriminator-closeout-20260608

Classification:
rp1-sysinfo-and-clock-window-sentinel-frontier-closed

## Inputs Inspected

- tasks/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-source-contract.md
- tasks/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-core.md
- tasks/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-control-pi5.md
- tasks/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-pi5.md
- tasks/evidence/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-source-contract/evidence-map.json
- tasks/evidence/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-core/evidence-map.json
- tasks/evidence/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-control-pi5/evidence-map.json
- tasks/evidence/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-pi5/evidence-map.json
- tasks/evidence/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-pi5/classification.json
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md

## Reconciliation

The source contract selected one read-only discriminator:
rp1-sysinfo-vs-clock-sentinel-read. Its only real-candidate RP1 operations are
32-bit loads from SYSINFO_CHIP_ID at 0x1f00000000, SYSINFO_PLATFORM at
0x1f00000004, and CLK_ADC_CTRL at 0x1f00018144. No writes or restore
operations are part of the accepted contract.

The local/static core implemented the real candidate and paired
no-MMIO/no-RP1/no-GIC control. The real archive SHA-256 is
f60e5899e994c4be98ccd3ac826b5c88f271db968056aff6afb9c1cf705fe42a, with
kernel_2712.img SHA-256
b61eb83442ee5bd332da0de8e53b42c63d4b9950a5a9b81db5f1abfc26bf1794 and size
47,776 bytes. The control archive preserves output shape while constructing no
forbidden RP1/GIC/MMIO address.

The paired no-MMIO/no-RP1/no-GIC control proof accepted the decisive rerun as
no-mmio-sysinfo-clock-sentinel-control-visible. It selected tree
499b836e2dfbd94d9301dfcb90d9625cd90e6e7507ba8070413ce8b36c5c551e, retained
two 47,288-byte TFTP fetches, retained 60 control markers, passed the v2
identity join with no rejection reasons, and restored to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The real proof accepted the decisive rerun as
rp1-sysinfo-and-clock-window-sentinel. It selected tree
22c13cf75878b9f1776d9ae00b760457df45a508b915c3032f4ac792693a74a4, retained
two 47,776-byte TFTP fetches, retained 62 result markers, passed the v2
identity join with no rejection reasons, and restored to tree
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The decisive real output reported SYSINFO_CHIP_ID=0xdeaddead,
SYSINFO_PLATFORM=0xdeaddead, and CLK_ADC_CTRL=0xdeaddead. The accepted
booleans were chip-id-matches-expected=false, chip-id-is-deaddead=true,
platform-is-deaddead=true, adc-ctrl-is-deaddead=true,
sysinfo-pair-equal=true, and sysinfo-vs-adc-same=true.

## Accepted Frontier

The chain closes as
rp1-sysinfo-and-clock-window-sentinel-frontier-closed. The accepted frontier is
limited to the read-only SYSINFO/address-decode sentinel boundary: Talos'
contracted SYSINFO identity reads returned the same sentinel value as the
retained CLK_ADC_CTRL comparator on Pi 5.

The closeout does not accept live RP1 SYSINFO identity, broad RP1 clock/reset
ownership, clock/reset writes, GPIO ownership, event generation, interrupt
delivery, GIC acknowledgement, ISR/handler ownership, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe behavior, Milestone 11.3, or a
phase transition.

## Next Action

No worker-owned follow-up is created by this closeout. Supervisor planning is
required for any different address/decode discriminator, GPIO ownership retry,
interrupt-delivery slice, broader clock/reset step, or phase transition.
