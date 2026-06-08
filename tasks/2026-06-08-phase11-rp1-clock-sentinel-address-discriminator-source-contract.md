# Phase 11 RP1 Clock Sentinel Address Discriminator Source Contract

Task id: phase11-rp1-clock-sentinel-address-discriminator-source-contract-20260608

Status: accepted

Classification: accepted-source-contract

## Goal

Define the smallest source-backed read-only discriminator that explains whether
the accepted repeated 0xdeaddead RP1 clock-manager window is a clock-block
sentinel or a broader RP1 address/decode boundary before any further clock
writes, GPIO ownership retry, or interrupt-delivery work.

## Scope

- Reviewed the accepted UART0 FR, GPIO status/source-status, IO_BANK0
  MSI-X/GIC-visible, clock-manager status, idempotent CLK_ADC_CTRL
  write/restore, enable-toggle mismatch, and ADC clock-window sentinel
  frontiers.
- Reviewed retained Raspberry Pi Linux RP1 MFD, clock/reset, device-tree, and
  Talos RP1 diagnostic evidence.
- Selected one read-only discriminator:
  rp1-sysinfo-vs-clock-sentinel-read.
- Named exact allowed addresses, widths, ordering, expected invariants, report
  fields, classifications, paired no-MMIO/no-RP1/no-GIC control requirements,
  and forbidden operations.
- Updated the roadmap and Phase 11 project contract for the accepted
  source-contract frontier.

## Non-Goals

No runtime implementation, hardware run, boot archive publication,
hardwareTestLock acquisition, RP1 clock/reset writes, reset-controller writes,
GPIO/RIO/pad writes, event generation, interrupt enablement or delivery, GIC
IAR/EOIR acknowledgement, ISR installation, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe enumeration, Milestone 11.3, or
phase transition.

## Findings

- fixed: selected a SYSINFO identity/address-decode read as a qualitatively
  different discriminator from same-shaped CLK_ADC_CTRL enable-toggle or ADC
  clock-window reruns.
- fixed: used retained MFD source facts that Linux reads SYSINFO_CHIP_ID and
  SYSINFO_PLATFORM during RP1 probe and rejects unexpected chip identity.
- fixed: retained the prior clock-window 0xdeaddead result as comparator
  context only, not as successful clock/reset ownership.
- fixed: specified exact allowed operations, report fields, classification
  names, paired control requirements, and forbidden operations.
- deferred: GPIO ownership retry, interrupt-delivery work, broader clock/reset
  ownership, and any write-capable clock/reset proof require later supervisor
  planning and acceptance gates.
- not-an-issue: no restore operation is needed because the discriminator is
  read-only.

No findings were removed.

## Accepted Source Contract

Contract id:
phase11-rp1-clock-sentinel-address-discriminator-source-contract-v1

~~~text
target: rp1-sysinfo-vs-clock-sentinel-read
operation: read-only SYSINFO identity vs retained clock-window sentinel comparison
rp1 base: 0x1f00000000
clock manager base: 0x1f00018000
width: 32
~~~

Allowed 32-bit volatile little-endian loads, in order:

| Field | Source offset | CPU physical address | Purpose |
| --- | ---: | ---: | --- |
| SYSINFO_CHIP_ID | 0x000000 | 0x1f00000000 | Expected RP1 C0 identity comparator |
| SYSINFO_PLATFORM | 0x000004 | 0x1f00000004 | Platform/raw decode context |
| CLK_ADC_CTRL | 0x018144 | 0x1f00018144 | Retained clock-window sentinel comparator |

No writes, restore operations, barriers beyond normal local read ordering,
frequency-counter operations, reset-controller operations, GPIO/RIO/pad
operations, MSI-X/PCIe/MIP operations, GIC operations, interrupt operations,
or DMA/cache operations are selected.

## Source Reconciliation

- The accepted RP1 mapping translates RP1 bus address 0xc0_4000_0000 to CPU
  physical 0x1f_0000_0000.
- rp1-mfd.h defines RP1_SYSINFO_BASE as 0x000000 and
  RP1_CLOCKS_BANK_DEFAULT_BASE as 0x018000.
- mfd-rp1.c defines SYSINFO_CHIP_ID_OFFSET as 0x0 and
  SYSINFO_PLATFORM_OFFSET as 0x4. Linux RP1 probe reads both through
  rp1_reg_read, stores them as g_chip_id/g_platform, logs chip_id, and rejects
  unexpected C0 identity.
- Retained Pi 5 firmware serial logs report RP1_BOOT chip ID 0x20001927 and
  RP1_CHIP_INFO 20001927.
- clk-rp1.c defines CLK_ADC_CTRL at clock-manager offset 0x00144. The accepted
  ADC clock-window proof reported CLK_ADC_CTRL and adjacent clock-manager
  registers as 0xdeaddead.

## Expected Invariants

- chip_id_matches_expected is true when SYSINFO_CHIP_ID is 0x20001927.
- chip_id_is_deaddead is true when SYSINFO_CHIP_ID is 0xdeaddead.
- platform_is_deaddead and adc_ctrl_is_deaddead report the retained sentinel
  value for SYSINFO_PLATFORM and CLK_ADC_CTRL.
- sysinfo_pair_equal reports whether SYSINFO_CHIP_ID and SYSINFO_PLATFORM are
  identical.
- sysinfo_vs_adc_same reports whether SYSINFO_CHIP_ID and CLK_ADC_CTRL are
  identical.
- A live SYSINFO chip id with a retained CLK_ADC_CTRL 0xdeaddead value
  classifies as rp1-sysinfo-live-clock-window-sentinel.
- A SYSINFO chip id that also returns 0xdeaddead classifies as
  rp1-sysinfo-and-clock-window-sentinel or address-decode-blocked, depending
  on capture freshness and output completeness.

## Report Fields

- contract id and target name.
- SYSINFO base, clock-manager base, register names, source offsets, CPU
  physical addresses, and width.
- raw sysinfo_chip_id, sysinfo_platform, and clk_adc_ctrl.
- expected_chip_id=0x20001927.
- chip_id_matches_expected, chip_id_is_deaddead, platform_is_deaddead,
  adc_ctrl_is_deaddead, sysinfo_pair_equal, and sysinfo_vs_adc_same.
- retained previous ADC clock-window classification and raw context.
- terminal classification.

Accepted classifications:

- rp1-sysinfo-live-clock-window-sentinel
- rp1-sysinfo-and-clock-window-sentinel
- rp1-sysinfo-live-clock-window-non-sentinel
- rp1-sysinfo-unexpected-chip-id
- rp1-sysinfo-address-decode-blocked
- rp1-sysinfo-clock-sentinel-inconclusive-capture
- no-mmio-sysinfo-clock-sentinel-control-visible
- staging/build-blocker

The paired no-MMIO/no-RP1/no-GIC control must preserve the same output shape
and classification vocabulary while constructing no RP1 SYSINFO, clock/reset,
GPIO/RIO/pads, MSI-X/PCIe/MIP, or GIC MMIO address and performing no volatile
load/store to those paths.

## Forbidden Operations

- RP1 clock/reset writes, including CLK_ADC_CTRL, divider, source, PLL,
  frequency-counter, GPCLK output-enable, or reset-controller writes.
- Same-shaped CLK_ADC_CTRL enable-toggle, idempotent write, or ADC-window
  coherence reruns as the accepted proof target.
- GPIO/RIO/pad writes, IO_BANK0 event/IRQ writes, MSI-X, PCIe config, MIP, or
  GIC writes.
- Event generation, interrupt enablement or delivery, GIC acknowledgement, ISR
  installation, DMA/cache, storage, generated-root, networking, SSH, broader
  PCIe enumeration, Milestone 11.3, or phase transition.

## Accepted Claims

This task accepts only the source contract for a read-only SYSINFO identity
vs retained clock-window sentinel discriminator and the paired
no-MMIO/no-RP1/no-GIC control requirement. It does not accept runtime behavior,
hardware behavior, broad RP1 clock/reset ownership, clock/reset writes, GPIO
ownership, event generation, interrupt delivery, handler ownership, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe enumeration, Milestone
11.3, or phase transition.

## Evidence

- Source notes:
  tasks/evidence/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-source-contract/source-reference-notes.md.
- Evidence map:
  tasks/evidence/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-source-contract/evidence-map.json.
- Updated contract docs:
  docs/src/project/phase11-rp1-pcie-map-contract.md.
- Updated roadmap:
  docs/src/roadmap.md.

## Validation

- Static inspection: accepted Phase 11 closeout evidence, retained
  Raspberry Pi Linux RP1 MFD/clock/device-tree sources, current Talos RP1
  diagnostic helpers, roadmap, and project contract inspected.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check before commit: pass.

## Result

Accepted as accepted-source-contract.

Next mechanically unblocked task:
phase11-rp1-clock-sentinel-address-discriminator-core-20260608. Implement only
the accepted read-only SYSINFO-vs-clock-sentinel discriminator and paired
no-MMIO/no-RP1/no-GIC control; do not acquire hardwareTestLock for the core
task.
