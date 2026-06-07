# Phase 11 RP1 Clock/Reset Status Source Reference Notes

Task: phase11-rp1-clock-reset-status-source-contract-20260607

Evidence level: static source/doc inspection.

## Retained Sources

- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/project/phase11-rp1-irq-clock-gpio-contract.md
- docs/src/project/reference-notes.md
- docs/src/roadmap.md
- tasks/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-closeout.md
- tasks/evidence/2026-06-07-phase11-rp1-gpio-owned-event-discriminator-pi5/evidence-map.json
- tasks/evidence/2026-06-07-phase11-rp1-gpio-ownership-restore-pi5/evidence-map.json
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/rp1.dtsi
- tasks/evidence/2026-06-05-phase11-rp1-pcie-map-source-contract/bcm2712-rpi-5-b.dts
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/clk-rp1.c
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/mfd-rp1.c
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/pinctrl-rp1.c
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/rp1-clock.h
- tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/rp1-mfd.h

## Accepted Frontier Inputs

- GPIO14 ownership/route preflight reached Pi 5 and reported GPIO14 fsel 13 /
  unknown function, so GPIO14 event generation remains blocked.
- GPIO16 owned event-discriminator preflight reached Pi 5 and reported GPIO16
  fsel 13 / unknown function before action writes, so same-shaped GPIO16 event
  discriminator reruns are blocked.
- The accepted RP1 mapping translates RP1 bus addresses with
  `cpu_phys = 0x1f00000000 + (rp1_bus - 0xc040000000)`.
- The accepted GPIO and interrupt frontiers do not accept clock/reset writes,
  GPIO ownership, interrupt delivery, or handler ownership.

## Clock/Reset Source Facts

- `rp1.dtsi` declares `rp1_clocks: clocks@18000` with
  `compatible = "raspberrypi,rp1-clocks"`, register range
  `0xc0_4001_8000` size `0x10038`, one XOSC input clock, and assigned clock
  rates for `RP1_PLL_SYS_CORE`, `RP1_CLK_SYS`, `RP1_CLK_SLOW_SYS`,
  `RP1_CLK_UART`, and other clocks.
- With the accepted translation, the RP1 clock manager base is CPU physical
  `0x1f00018000`.
- `clk-rp1.c` defines the relevant clock status/control offsets:
  `PLL_SYS_CS = 0x08000`, `CLK_SYS_CTRL = 0x00014`,
  `CLK_SYS_DIV_INT = 0x00018`, `CLK_SYS_SEL = 0x00020`,
  `CLK_SLOW_SYS_CTRL = 0x00024`, `CLK_UART_CTRL = 0x00054`,
  `CLK_UART_DIV_INT = 0x00058`, and `CLK_UART_SEL = 0x00060`.
- `clk-rp1.c` defines `CLK_CTRL_ENABLE = BIT(11)`, `PLL_CS_LOCK = BIT(31)`,
  system clock parents `xosc`, reserved, and `pll_sys`, and UART clock
  parents including `pll_sys_pri_ph`, `pll_video`, `xosc`, and
  GPCLK-derived sources.
- `clk-rp1.c` marks `clk_sys` and `clk_slow_sys` critical and comments
  that they are always enabled in hardware.
- `rp1-mfd.h` maps `RP1_RESETS_BASE = 0x014000` and
  `RP1_CLOCKS_BANK_DEFAULT_BASE = 0x018000`. This task records the reset base
  only to forbid reset writes.
- `mfd-rp1.c` obtains an optional reset control and calls
  `reset_control_reset` during Linux RP1 probe, then rejects a short BAR1 as
  firmware-not-initialized evidence and reads SYSINFO chip/platform registers.
  Talos must not copy the reset path in this read-only source contract.

## Function-State Context

- `pinctrl-rp1.c` defines GPIO14 functions with uart0 at fsel 4, gpio at
  fsel 5, and proc_rio at fsel 6.
- `pinctrl-rp1.c` defines GPIO16 functions with uart0 at fsel 4, gpio at
  fsel 5, and proc_rio at fsel 6.
- The accepted hardware blockers observed fsel 13 for both GPIO14 and GPIO16,
  which is not a source-named GPIO function state in the retained fsel table.
- This task does not select another GPIO CTRL read or GPIO write. The next
  discriminator clarifies read-only RP1 clock manager visibility and state
  before any future supervisor-planned ownership/function write contract.

## Selected Diagnostic

```text
contract: phase11-rp1-clock-reset-status-source-contract-v1
target: rp1-clock-manager-status-read
operation: read-only clock manager status snapshot
base: 0x1f00018000
```

Allowed 32-bit reads:

- PLL_SYS_CS at `0x1f00020000`
- CLK_SYS_CTRL at `0x1f00018014`
- CLK_SYS_DIV_INT at `0x1f00018018`
- CLK_SYS_SEL at `0x1f00018020`
- CLK_SLOW_SYS_CTRL at `0x1f00018024`
- CLK_UART_CTRL at `0x1f00018054`
- CLK_UART_DIV_INT at `0x1f00018058`
- CLK_UART_SEL at `0x1f00018060`

No frequency-counter reads are selected because the Linux measurement path
writes FC0 registers before reading results. No reset-status read is selected
because the retained reset source context exposes a reset operation, not a
safe read-only status boundary for this task.

## Report Fields

- contract id and target name.
- raw values for each allowed register.
- decoded `PLL_SYS_CS.lock`.
- decoded `CLK_CTRL_ENABLE` for `clk_sys`, `clk_slow_sys`, and `clk_uart`.
- decoded system and UART divider/source fields.
- retained GPIO14/GPIO16 fsel 13 blocker context.
- terminal classification.

## Classifications

- `rp1-clock-manager-status-visible`
- `rp1-clock-manager-status-blocked-missing-clock-manager`
- `rp1-clock-manager-status-blocked-incoherent-sys-clock`
- `rp1-clock-manager-status-blocked-uart-clock-disabled`
- `rp1-clock-manager-status-inconclusive-capture`
- `staging/build-blocker`

## Paired Control Requirement

The paired control must preserve the real diagnostic's serial/output shape and
classification vocabulary while constructing no RP1 clock/reset, GPIO/RIO/pads,
MSI-X/PCIe/MIP, or GIC MMIO address and performing no volatile load/store to
those paths. The control may emit simulated raw values and a control
classification only.

## Review Findings

- fixed: selected one read-only clock-manager status target with exact
  allowed reads and report fields.
- fixed: retained Linux reset behavior as a forbidden write path, not a Talos
  implementation plan.
- fixed: tied the selected target to the accepted GPIO14/GPIO16 fsel 13
  blockers without selecting another same-shaped GPIO rerun.
- deferred: clock/reset writes, GPIO function/ownership retry, event
  generation, interrupt delivery, and handler ownership.
- not-an-issue: source-contract acceptance does not imply hardware behavior or
  RP1 clock/reset ownership.

No findings were removed.
