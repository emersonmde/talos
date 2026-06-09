# Phase 11 RP1 Clock/Reset Dependency Source Register Evidence

Task: phase11-rp1-clock-reset-dependency-source-contract-20260609

Evidence level: static source/doc inspection.

## Retained Inputs

- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md
- tasks/2026-06-07-phase11-rp1-clock-reset-status-source-contract.md
- tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-source-contract/source-reference-notes.md
- tasks/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-closeout.md
- tasks/2026-06-08-phase11-rp1-clock-sentinel-address-discriminator-pi5.md
- tasks/2026-06-08-phase11-rp1-observed-gpio-ownership-route-closeout.md
- tasks/2026-06-09-phase11-rp1-observed-gpio16-ownership-event-closeout.md
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/rp1.dtsi
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi-5-b.dts
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/clk-rp1.c
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/mfd-rp1.c
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/rp1-mfd.h
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/rp1-clock.h
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/pinctrl-rp1.c

## Accepted Frontier Inputs

- The accepted observed-aperture GPIO14 ownership/route preflight classified
  GPIO14 as FUNCSEL=4 / uart0 and did not authorize GPIO14 function changes.
- The accepted observed-aperture GPIO16 ownership/event preflight classified
  GPIO16 as FUNCSEL=31 / unknown, with GPIO16 source-enable/source-status
  clear, RIO GPIO16 bits false, pad input/output disabled, and INTID160
  not enabled/pending/active.
- The accepted read-only SYSINFO-vs-clock sentinel discriminator on the
  source-expected 0x1f aperture returned 0xdeaddead for SYSINFO_CHIP_ID,
  SYSINFO_PLATFORM, and CLK_ADC_CTRL. That is retained as a sentinel
  comparator, not as live RP1 identity.
- The accepted clock-manager status frontier previously selected source
  clock-manager offsets and proved only a read-only status snapshot. It did
  not accept clock/reset writes or ownership.

## Source Facts

- rp1-mfd.h defines:
  - RP1_SYSINFO_BASE = 0x000000
  - RP1_RESETS_BASE = 0x014000
  - RP1_CLOCKS_BANK_DEFAULT_BASE = 0x018000
- mfd-rp1.c defines SYSINFO_CHIP_ID_OFFSET = 0x0 and
  SYSINFO_PLATFORM_OFFSET = 0x4. Linux reads both during RP1 probe through
  rp1_reg_read and rejects unexpected RP1 chip identity.
- mfd-rp1.c also obtains an optional reset control and calls
  reset_control_reset during probe. This is reset-operation context, not a
  safe read-only reset-status boundary.
- clk-rp1.c defines:
  - PLL_SYS_CS = 0x08000
  - CLK_SYS_CTRL = 0x00014
  - CLK_SYS_DIV_INT = 0x00018
  - CLK_SYS_SEL = 0x00020
  - CLK_SLOW_SYS_CTRL = 0x00024
  - CLK_UART_CTRL = 0x00054
  - CLK_UART_DIV_INT = 0x00058
  - CLK_UART_SEL = 0x00060
- clk-rp1.c defines CLK_CTRL_ENABLE = BIT(11) and PLL_CS_LOCK = BIT(31).
  It marks clk_sys and clk_slow_sys as critical and comments that they are
  always enabled in hardware.

## Observed-Aperture Address Facts

The accepted observed GPIO14/GPIO16 preflights used:

~~~text
observed CPU physical = 0x1c00000000 + RP1 source offset
~~~

Selected observed CPU physical reads:

- SYSINFO_CHIP_ID: 0x1c00000000
- SYSINFO_PLATFORM: 0x1c00000004
- PLL_SYS_CS: 0x1c00020000
- CLK_SYS_CTRL: 0x1c00018014
- CLK_SYS_DIV_INT: 0x1c00018018
- CLK_SYS_SEL: 0x1c00018020
- CLK_SLOW_SYS_CTRL: 0x1c00018024
- CLK_UART_CTRL: 0x1c00018054
- CLK_UART_DIV_INT: 0x1c00018058
- CLK_UART_SEL: 0x1c00018060

No reset-controller register is selected because the retained reset facts are
write-oriented and do not provide a bounded source-backed read-only reset
status register for this task.

## Selected Contract

~~~text
contract: phase11-rp1-clock-reset-dependency-source-contract-v1
target: rp1-observed-clock-reset-dependency-preflight-read
operation: read-only observed-aperture identity and clock-manager dependency snapshot
allowed writes: none
~~~

The preflight may classify only identity/decode visibility, selected
clock-manager status visibility, selected system/UART clock disabled state,
no-return/trap, capture inconclusive, no-MMIO control visibility, or
staging/build blockers.

## Review Findings

- fixed: selected a source-backed observed-aperture dependency preflight
  instead of rerunning same-shaped GPIO14/GPIO16 preflights.
- fixed: retained exact source offsets and observed-aperture CPU physical
  addresses for SYSINFO and clock-manager status loads.
- fixed: excluded reset-controller operations because retained Linux source
  exposes reset_control_reset, not a safe read-only reset-status register.
- deferred: clock/reset writes, GPIO function changes, event generation,
  interrupt delivery, driver setup, and restore-after-write semantics remain
  future supervisor-planned work.
- not-an-issue: prior 0x1f SYSINFO/clock sentinel evidence is comparator
  context only and does not authorize broad RP1 ownership.

No findings were removed.
