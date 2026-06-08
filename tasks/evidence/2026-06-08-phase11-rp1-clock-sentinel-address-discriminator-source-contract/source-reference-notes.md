# Phase 11 RP1 Clock Sentinel Address Discriminator Source Reference Notes

Task: phase11-rp1-clock-sentinel-address-discriminator-source-contract-20260608

Evidence level: static source/doc inspection.

## Retained Sources

- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md
- tasks/2026-06-07-phase11-rp1-clock-write-effect-discriminator-closeout.md
- tasks/evidence/2026-06-07-phase11-rp1-clock-write-effect-discriminator-closeout/evidence-map.json
- tasks/evidence/2026-06-07-phase11-rp1-clock-write-effect-discriminator-pi5/classification.json
- tasks/evidence/2026-06-07-phase11-rp1-clock-write-effect-discriminator-source-contract/source-reference-notes.md
- tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-pi5/classification.json
- tasks/evidence/2026-06-07-phase11-rp1-clock-reset-write-restore-pi5/classification.json
- tasks/evidence/2026-06-07-phase11-rp1-clock-adc-enable-toggle-pi5/classification.json
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/rp1.dtsi
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/clk-rp1.c
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/mfd-rp1.c
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/rp1-mfd.h
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/rp1-clock.h

## Retained Sentinel Evidence

- phase11-rp1-clock-write-effect-discriminator-closeout-20260607 accepted
  rp1-clock-adc-window-readback-sentinel-frontier-closed.
- The real Pi 5 proof read CLK_SYS_CTRL, CLK_UART_CTRL, two ordered
  CLK_ADC_CTRL values, CLK_ADC_DIV_INT, and CLK_ADC_SEL. All returned
  0xdeaddead.
- The accepted result was classified as a read-only sentinel boundary, not as
  successful non-idempotent clock ownership or broad RP1 clock/reset ownership.
- Same-shaped CLK_ADC_CTRL enable-toggle, idempotent write, or ADC-window
  hardware reruns remain blocked without a different discriminator and
  explicit acceptance criteria.

## Source Facts

- The accepted RP1 mapping translates RP1 bus address 0xc0_4000_0000 to CPU
  physical 0x1f_0000_0000.
- rp1-mfd.h names RP1_SYSINFO_BASE as 0x000000 and
  RP1_CLOCKS_BANK_DEFAULT_BASE as 0x018000.
- mfd-rp1.c defines SYSINFO_CHIP_ID_OFFSET as 0x0 and
  SYSINFO_PLATFORM_OFFSET as 0x4. During Linux RP1 probe, it reads both through
  rp1_reg_read, stores them as g_chip_id/g_platform, logs chip_id, and rejects
  the device if g_chip_id does not match RP1_C0_CHIP_ID.
- Retained Pi 5 firmware serial logs report RP1_BOOT chip ID 0x20001927 and
  RP1_CHIP_INFO 20001927, so a source-backed SYSINFO identity read has an
  expected non-sentinel identity value even though Talos has not accepted RP1
  driver ownership.
- clk-rp1.c defines CLK_ADC_CTRL at clock-manager offset 0x00144,
  CLK_ADC_DIV_INT at 0x00148, and CLK_ADC_SEL at 0x00150, all under the RP1
  clock-manager base 0x018000. The prior Pi 5 ADC-window proof returned
  0xdeaddead for those selected clock-manager locations.

## Selected Discriminator

~~~text
contract: phase11-rp1-clock-sentinel-address-discriminator-source-contract-v1
target: rp1-sysinfo-vs-clock-sentinel-read
operation: read-only SYSINFO identity vs retained clock-window sentinel comparison
width: 32
~~~

Allowed 32-bit volatile little-endian reads, in order:

1. SYSINFO_CHIP_ID at CPU physical 0x1f00000000.
2. SYSINFO_PLATFORM at CPU physical 0x1f00000004.
3. CLK_ADC_CTRL at CPU physical 0x1f00018144.

No writes, restore operations, clock toggles, divider/source/PLL/frequency
counter operations, reset-controller operations, GPIO/RIO/pad operations,
MSI-X/PCIe/MIP operations, GIC operations, interrupt operations, or DMA/cache
operations are selected.

## Expected Invariants

- A live RP1 SYSINFO decode should return a non-sentinel chip id. The expected
  C0 identity from retained Pi 5 firmware logs is 0x20001927.
- SYSINFO_PLATFORM may vary by platform flags, so it is reported raw and only
  checked for the retained sentinel value and equality with chip id.
- CLK_ADC_CTRL is retained as the clock-window sentinel comparator. It is not
  written, restored, or interpreted as successful clock ownership.
- If SYSINFO_CHIP_ID is 0x20001927 while CLK_ADC_CTRL remains 0xdeaddead, the
  result distinguishes live RP1 identity/address decode from the clock-window
  sentinel boundary.
- If SYSINFO_CHIP_ID also returns 0xdeaddead, the result localizes the blocker
  to the broader RP1 SYSINFO/address-decode path rather than to the ADC clock
  register window alone.

## Report Fields

- contract id and target name.
- SYSINFO and clock-manager base addresses.
- register names, source offsets, CPU physical addresses, and width.
- raw sysinfo_chip_id, sysinfo_platform, and clk_adc_ctrl values.
- expected_chip_id=0x20001927.
- chip_id_matches_expected, chip_id_is_deaddead, platform_is_deaddead,
  adc_ctrl_is_deaddead, sysinfo_pair_equal, and sysinfo_vs_adc_same booleans.
- retained prior clock-window classification and ADC-window raw context.
- terminal classification.

## Classifications

- rp1-sysinfo-live-clock-window-sentinel
- rp1-sysinfo-and-clock-window-sentinel
- rp1-sysinfo-live-clock-window-non-sentinel
- rp1-sysinfo-unexpected-chip-id
- rp1-sysinfo-address-decode-blocked
- rp1-sysinfo-clock-sentinel-inconclusive-capture
- no-mmio-sysinfo-clock-sentinel-control-visible
- staging/build-blocker

## Paired Control Requirement

The paired control must preserve the real diagnostic's serial/output shape and
classification vocabulary while constructing no RP1 SYSINFO, clock/reset,
GPIO/RIO/pads, MSI-X/PCIe/MIP, or GIC MMIO address and performing no volatile
load/store to those paths. The control may emit simulated raw values and the
control-only classification.

## Forbidden Operations

- Any RP1 clock/reset write, including CLK_ADC_CTRL, divider, source, PLL,
  frequency-counter, GPCLK output-enable, or reset-controller writes.
- Any same-shaped CLK_ADC_CTRL enable-toggle, idempotent write, or ADC-window
  coherence rerun as the accepted proof target.
- GPIO/RIO/pad writes, IO_BANK0 event/IRQ writes, MSI-X, PCIe config, MIP, or
  GIC writes.
- Event generation, interrupt enablement or delivery, GIC acknowledgement, ISR
  installation, DMA/cache, storage, generated-root, networking, SSH, broader
  PCIe enumeration, Milestone 11.3, or phase transition.

## Review Findings

- fixed: selected a source-backed SYSINFO identity/address-decode read to
  distinguish a live RP1 identity path from the retained clock-window sentinel.
- fixed: retained the ADC clock-window 0xdeaddead value only as comparator
  context, not as a clock ownership claim.
- fixed: defined one read-only discriminator with exact addresses, ordering,
  report fields, classifications, paired control requirements, and forbidden
  operations.
- deferred: any GPIO ownership retry, interrupt-delivery slice, broader
  clock/reset step, or write-capable clock/reset proof requires later
  supervisor planning.
- not-an-issue: no restore semantics are needed because the selected
  discriminator is read-only.

No findings were removed.
