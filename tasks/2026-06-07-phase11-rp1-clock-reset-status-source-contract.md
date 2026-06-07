# Phase 11 RP1 Clock/Reset Status Source Contract

Task id: phase11-rp1-clock-reset-status-source-contract-20260607

Status: accepted

Classification: accepted-source-contract

## Goal

Define the smallest source-backed read-only RP1 clock/reset or function-state
status contract after the GPIO14 and GPIO16 fsel 13 blockers, without
accepting RP1 clock/reset writes, GPIO ownership, interrupt delivery, or a
phase transition.

## Scope

- Reviewed accepted Phase 11 RP1 mapping, interrupt-routing, GIC-visible
  route, GPIO bank source-status, GPIO14 ownership preflight, and GPIO16
  event-discriminator evidence.
- Reviewed retained Raspberry Pi Linux RP1 clock/reset/MFD/pinctrl/GPIO/device
  tree sources and Talos RP1 diagnostic helpers.
- Selected one read-only/no-write diagnostic target:
  `rp1-clock-manager-status-read`.
- Named exact allowed register reads, report fields, classification names,
  paired no-MMIO/no-RP1/no-GIC control requirements, and forbidden operations.
- Updated project contract and roadmap docs for the accepted source-contract
  frontier.

## Non-Goals

No runtime implementation, hardware run, boot archive publication,
hardwareTestLock acquisition, RP1 clock/reset writes, GPIO CTRL/INTE/RIO/pad
writes, GPIO event generation, interrupt enablement or delivery, GIC
IAR/EOIR acknowledgement, ISR installation, broad GPIO driver ownership,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe enumeration,
Milestone 11.3, or phase transition.

## Findings

- fixed: selected the RP1 clock manager as the next read-only status boundary,
  rather than retrying the same-shaped GPIO14/GPIO16 fsel 13 event paths.
- fixed: retained exact clock-manager source addresses for PLL_SYS lock state,
  critical system clocks, slow system clock, and UART clock control state.
- fixed: treated Linux's RP1 reset path as source context only. The MFD driver
  obtains an optional reset controller and calls reset_control_reset during
  probe, but this contract forbids reset writes.
- fixed: retained function-state context from the accepted GPIO14/GPIO16
  blockers and pinctrl source tables, while keeping the new diagnostic focused
  on read-only clock-manager status.
- deferred: any clock/reset write contract, GPIO ownership retry, event
  generation, interrupt delivery, and handler ownership require later
  supervisor planning.
- not-an-issue: a read-only clock status value can clarify firmware/RP1 clock
  manager visibility, but it is not proof that Talos owns clocks, resets, GPIO
  functions, or interrupt delivery.

No findings were removed.

## Accepted Source Contract

Contract id: phase11-rp1-clock-reset-status-source-contract-v1

```text
target: rp1-clock-manager-status-read
operation: read-only clock manager status snapshot
source block: RP1 clocks@18000, compatible raspberrypi,rp1-clocks
translated base: 0x1f00018000
```

Allowed 32-bit volatile little-endian loads:

| Field | Source offset | CPU physical address | Decoding |
| --- | ---: | ---: | --- |
| PLL_SYS_CS | `0x08000` | `0x1f00020000` | lock bit 31, refdiv low bits |
| CLK_SYS_CTRL | `0x00014` | `0x1f00018014` | enable bit 11, aux source, source |
| CLK_SYS_DIV_INT | `0x00018` | `0x1f00018018` | integer divider |
| CLK_SYS_SEL | `0x00020` | `0x1f00018020` | selected source status |
| CLK_SLOW_SYS_CTRL | `0x00024` | `0x1f00018024` | enable bit 11, source |
| CLK_UART_CTRL | `0x00054` | `0x1f00018054` | enable bit 11, aux source, source |
| CLK_UART_DIV_INT | `0x00058` | `0x1f00018058` | integer divider |
| CLK_UART_SEL | `0x00060` | `0x1f00018060` | selected source status |

The selected target is read-only and non-destructive. It may report only the
contract id, target name, addresses, raw values, decoded `CLK_CTRL_ENABLE`
bits, decoded `PLL_CS_LOCK`, source/divider fields, the retained GPIO14/GPIO16
fsel 13 blocker context, and one terminal classification.

Accepted classifications:

- `rp1-clock-manager-status-visible`
- `rp1-clock-manager-status-blocked-missing-clock-manager`
- `rp1-clock-manager-status-blocked-incoherent-sys-clock`
- `rp1-clock-manager-status-blocked-uart-clock-disabled`
- `rp1-clock-manager-status-inconclusive-capture`
- `staging/build-blocker`

The paired no-MMIO/no-RP1/no-GIC control must preserve the same serial/output
shape and classification vocabulary while constructing no RP1 clock/reset,
GPIO/RIO/pads, MSI-X/PCIe/MIP, or GIC MMIO address and performing no volatile
load/store to those paths.

## Forbidden Operations

- RP1 clock/reset writes, including clock enable/disable, divider, source,
  PLL, frequency-counter setup, or reset-controller writes.
- GPIO CTRL/INTE/RIO/pad writes, GPIO event generation, IRQRESET, or GPIO14 /
  GPIO16 ownership retry.
- GIC, MSI-X, PCIe config, or MIP writes; GIC IAR/EOIR reads or writes;
  interrupt unmasking, delivery acceptance, or ISR installation.
- Any broad RP1 driver ownership, DMA/cache, storage, generated-root,
  networking, SSH, broader PCIe enumeration, Milestone 11.3, or phase
  transition claim.

## Accepted Claims

This task accepts only the source contract for a read-only RP1 clock manager
status snapshot and the paired no-MMIO/no-RP1/no-GIC control requirement. It
does not accept runtime behavior, hardware behavior, clock/reset ownership,
GPIO ownership, GPIO event generation, interrupt delivery, GIC acknowledgement,
handler ownership, DMA/cache, storage, generated-root, networking, SSH,
broader PCIe enumeration, Milestone 11.3, or a phase transition.

## Evidence

- tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-source-contract/source-reference-notes.md
- tasks/evidence/2026-06-07-phase11-rp1-clock-reset-status-source-contract/evidence-map.json
- docs/src/project/phase11-rp1-pcie-map-contract.md
- docs/src/roadmap.md

## Validation

- Static inspection of accepted Phase 11 docs/evidence and retained RP1
  clock/reset/MFD/pinctrl/GPIO/device-tree sources: pass.
- git diff --check: pass.
- /home/node/.cargo/bin/mdbook build: pass.
- git diff --cached --check: pass.

## Next Action

Promote phase11-rp1-clock-reset-status-core-20260607 on the next worker wake
if dependencies remain satisfied. Do not acquire hardwareTestLock for the
local/static core.
