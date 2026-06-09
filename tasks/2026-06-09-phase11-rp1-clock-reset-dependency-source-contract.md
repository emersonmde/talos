# Phase 11 RP1 Clock/Reset Dependency Source Contract

Task id: phase11-rp1-clock-reset-dependency-source-contract-20260609

Status: accepted

Classification: accepted-observed-clock-reset-dependency-source-contract

## Goal

Define the smallest source-backed read-only RP1 clock/reset dependency
preflight needed before any Talos-owned RP1 GPIO, event, or driver setup work.

## Scope

- Reviewed retained Raspberry Pi Linux RP1 MFD, clock, reset, pinctrl/GPIO,
  PCIe, device-tree sources, and accepted Talos Phase 11 evidence.
- Used the accepted GPIO14/GPIO16 non-GPIO-function blockers and observed
  0x1c RP1 aperture evidence as constraints, not as authorization for writes.
- Selected one read-only observed-aperture identity and clock-manager
  dependency preflight.
- Named exact addresses, widths, ordering, report fields, paired no-MMIO
  control shape, and classifications.
- Updated project and roadmap docs for the accepted source-contract frontier.

## Non-Goals

No runtime source change, Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, clock/reset writes, GPIO function changes,
GPIO/RIO/pad/INTE/CTRL writes, IRQRESET, interrupt unmasking, IAR/EOIR
acknowledgement, ISR/handler install, event generation, interrupt delivery,
endpoint config retry, bridge setup write, DMA/cache, storage, generated-root,
networking, SSH, Milestone 11.3, or phase transition. This task does not reopen
same-shaped GPIO14 or GPIO16 ownership/event preflight reruns.

## Findings And Disposition

- fixed: selected the observed 0x1c RP1 aperture for the next read-only
  clock/reset dependency preflight because the accepted observed GPIO14/GPIO16
  preflights used that aperture, while the retained 0x1f SYSINFO-vs-clock
  discriminator closed on a broader 0xdeaddead sentinel boundary.
- fixed: retained source-backed SYSINFO and clock-manager offsets from Linux
  MFD/clock sources and translated them into exact observed-aperture CPU
  physical addresses.
- fixed: selected only read-only identity and clock-manager status loads that
  can classify whether the clock/reset dependency boundary is visible before
  any GPIO function or event work.
- fixed: recorded that retained Linux reset-controller behavior is a forbidden
  write path. No reset-status register is selected because the retained source
  facts expose reset operations, not a narrow safe read-only reset-status
  boundary.
- deferred: any clock/reset write, reset ownership, GPIO function change,
  event generation, interrupt delivery, driver setup, and restore-after-write
  semantics require future supervisor planning.
- not-an-issue: the contract can use prior 0x1f clock/SYSINFO evidence as
  comparator context without rereading that same sentinel path or accepting
  broad RP1 ownership.

No findings were removed.

## Accepted Source Contract

Contract id:
phase11-rp1-clock-reset-dependency-source-contract-v1

~~~text
target: rp1-observed-clock-reset-dependency-preflight-read
operation: read-only observed-aperture identity and clock-manager dependency snapshot
observed RP1 base: 0x1c00000000
width: 32-bit volatile little-endian loads
allowed writes: none
~~~

Allowed loads, in order:

| Field | Source offset | CPU physical address | Purpose |
| --- | ---: | ---: | --- |
| SYSINFO_CHIP_ID | 0x000000 | 0x1c00000000 | observed-aperture identity/decode comparator |
| SYSINFO_PLATFORM | 0x000004 | 0x1c00000004 | observed-aperture platform/decode context |
| PLL_SYS_CS | 0x020000 | 0x1c00020000 | PLL_SYS lock/source status context |
| CLK_SYS_CTRL | 0x018014 | 0x1c00018014 | critical system clock enable/source context |
| CLK_SYS_DIV_INT | 0x018018 | 0x1c00018018 | system clock divider context |
| CLK_SYS_SEL | 0x018020 | 0x1c00018020 | system clock selected source context |
| CLK_SLOW_SYS_CTRL | 0x018024 | 0x1c00018024 | slow system clock enable/source context |
| CLK_UART_CTRL | 0x018054 | 0x1c00018054 | UART clock enable/source context for RP1 UART paths |
| CLK_UART_DIV_INT | 0x018058 | 0x1c00018058 | UART clock divider context |
| CLK_UART_SEL | 0x018060 | 0x1c00018060 | UART clock selected source context |

No reset-controller read is selected. Retained Linux source obtains an
optional reset control and calls reset_control_reset during RP1 probe; that is
write-like source context and remains forbidden for this read-only preflight.

## Source Reconciliation

- The accepted observed GPIO14 and GPIO16 preflights established useful
  read-only visibility under the observed RP1 aperture:
  observed CPU physical address = 0x1c00000000 + source offset.
- The retained source-expected 0x1f SYSINFO-vs-clock discriminator returned
  0xdeaddead for SYSINFO_CHIP_ID, SYSINFO_PLATFORM, and CLK_ADC_CTRL. That is
  retained as a sentinel comparator, not as a live RP1 identity claim.
- rp1-mfd.h names RP1_SYSINFO_BASE as 0x000000, RP1_RESETS_BASE as 0x014000,
  and RP1_CLOCKS_BANK_DEFAULT_BASE as 0x018000.
- mfd-rp1.c defines SYSINFO_CHIP_ID_OFFSET as 0x0 and
  SYSINFO_PLATFORM_OFFSET as 0x4. Linux reads both during RP1 probe and
  rejects unexpected chip identity.
- clk-rp1.c defines PLL_SYS_CS at clock-manager offset 0x08000,
  CLK_SYS_CTRL at 0x00014, CLK_SYS_DIV_INT at 0x00018, CLK_SYS_SEL at 0x00020,
  CLK_SLOW_SYS_CTRL at 0x00024, CLK_UART_CTRL at 0x00054,
  CLK_UART_DIV_INT at 0x00058, and CLK_UART_SEL at 0x00060.
- clk-rp1.c defines CLK_CTRL_ENABLE as bit 11 and PLL_CS_LOCK as bit 31.
  It marks clk_sys and clk_slow_sys as critical and comments that they are
  always enabled in hardware.

## Report Fields

- contract id and target name.
- observed base, register names, source offsets, CPU physical addresses, and
  width.
- raw sysinfo_chip_id, sysinfo_platform, pll_sys_cs, clk_sys_ctrl,
  clk_sys_div_int, clk_sys_sel, clk_slow_sys_ctrl, clk_uart_ctrl,
  clk_uart_div_int, and clk_uart_sel.
- expected_chip_id=0x20001927.
- decoded booleans: chip_id_matches_expected, chip_id_is_deaddead,
  platform_is_deaddead, pll_sys_locked, clk_sys_enabled,
  clk_slow_sys_enabled, clk_uart_enabled, any_selected_clock_deaddead, and
  all_selected_clock_deaddead.
- retained context strings for the accepted GPIO14 UART0 blocker, accepted
  GPIO16 FUNCSEL=31 blocker, and retained 0x1f SYSINFO/clock sentinel
  closeout.
- reset_status_source=none-selected-read-only.
- terminal classification.

Accepted classifications:

- observed-clock-reset-dependency-visible
- observed-clock-reset-dependency-blocked-sysinfo-sentinel
- observed-clock-reset-dependency-blocked-clock-manager-sentinel
- observed-clock-reset-dependency-blocked-system-clock-disabled
- observed-clock-reset-dependency-blocked-uart-clock-disabled
- observed-clock-reset-dependency-no-return-or-trap
- observed-clock-reset-dependency-inconclusive-capture
- no-mmio-clock-reset-dependency-control-visible
- staging/build-blocker

The paired no-MMIO/no-RP1/no-GIC/no-PCIe control must preserve the same output
shape while constructing no RP1 SYSINFO, clock/reset, GPIO/RIO/pads, MSI-X,
PCIe config, MIP, GIC, DMA, or other MMIO address and performing no volatile
load/store to those paths.

## Forbidden Operations

- Clock/reset writes, including clock enable/disable, divider, source, PLL,
  frequency-counter, GPCLK output-enable, or reset-controller writes.
- GPIO function changes, GPIO/RIO/pad/INTE/CTRL writes, IRQRESET
  acknowledgement, event generation, interrupt pending generation beyond
  read-only observation, interrupt unmasking, interrupt delivery, IAR/EOIR
  acknowledgement, or ISR/handler installation.
- Endpoint config retry, bridge setup writes, DMA/cache, storage,
  generated-root, networking, SSH, Milestone 11.3, or phase transition.

## Accepted Claims

This task accepts only the source contract for a read-only observed-aperture
RP1 identity and clock-manager dependency preflight and its paired no-MMIO
control requirement. It does not accept runtime behavior, hardware behavior,
live RP1 identity, clock/reset ownership, clock/reset writes, GPIO ownership,
event generation, interrupt delivery, handler ownership, DMA/cache,
networking, SSH, Milestone 11.3, or a phase transition.

## Evidence

- Source/register evidence:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-source-contract/source-register-evidence.md.
- Evidence map:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-source-contract/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-09-phase11-rp1-clock-reset-dependency-source-contract/classification.json.
- Updated project contract docs:
  docs/src/project/phase11-rp1-pcie-map-contract.md.
- Updated roadmap:
  docs/src/roadmap.md.

## Validation

- Static inspection: retained Linux RP1 MFD/clock/reset/pinctrl/device-tree
  sources, accepted Phase 11 evidence maps, roadmap, and project contract
  inspected.
- jq evidence-map/classification checks: passed.
- git diff --check: passed.
- docs build: mdbook build passed because docs/src files were updated.
- git diff --cached --check: passed before commit.

## Next Action

phase11-rp1-clock-reset-dependency-core-20260609 is mechanically unblocked on
the next worker wake if this committed source contract remains present. The
next task may implement only this read-only real candidate and paired no-MMIO
control; it must not acquire hardwareTestLock.
