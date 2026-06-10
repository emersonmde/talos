# Phase 12 RP1 Ethernet GPIO32 PHY-Reset Source Contract

Task id: phase12-rp1-ethernet-gpio32-phy-reset-source-contract-20260610
Status: accepted
Owner: worker
Classification: rp1-ethernet-gpio32-phy-reset-source-contract-accepted
Evidence level: static inspection of accepted Phase 12 clock/prerequisite
proof closeouts, accepted Phase 11 GPIO frontiers, retained Raspberry Pi Linux
source excerpts, Phase 12 docs, roadmap, and task-owned JSON. No Pi 5 hardware
run, GPIO/RIO/pad/MMIO write, PHY reset assertion/deassertion, MDIO
transaction, boot archive publication, or hardwareTestLock acquisition was
performed.

## Goal

Define the next bounded GPIO32 / ETH_RST_N source contract after the accepted
observed-window MACB_MID identity, prerequisite report visibility, and
Ethernet-private clock write/restore proofs.

## Scope

- Consumed the accepted prerequisite ownership proof closeout and accepted
  CLK_ETH_TSU_CTRL / CLK_ETH_CTRL write/restore proof closeouts.
- Reconciled retained Linux device-tree and MACB source facts for Pi 5
  `phy-reset-gpios = <&rp1_gpio 32 GPIO_ACTIVE_LOW>`,
  `phy-reset-duration = <5>`, and Linux's MDIO bus reset hook.
- Reconciled those source facts against accepted Phase 11 GPIO constraints:
  Talos has read-only/status and blocker evidence, but no GPIO ownership,
  pinmux/function-change, RIO, pad, INTE/CTRL, event-generation, or
  write/restore authority.
- Defined only a read-only/local-static preflight report core as the next
  follow-up, with paired no-GPIO/no-Ethernet control, accepted classifications,
  validation gates, and future write/restore safety invariants.
- Recorded findings with disposition.

## Non-Goals

No runtime implementation, no Pi 5 hardware run, no boot archive publication,
no hardwareTestLock acquisition, no GPIO/RIO/pad/MMIO write, no pinmux or pad
configuration change, no PHY reset assertion/deassertion, no MDIO/PHY
ownership, no packet I/O, no networking, no sockets, no SSH, no Phase 12.2,
and no phase transition.

## Accepted Input Frontier

The accepted input frontier is deliberately narrow:

- observed-window MACB_MID identity context only: `SYSINFO_CHIP_ID` at
  `0x1c00000000` returned `0x20001927`, and observed-window `MACB_MID` at
  `0x1c001000fc` returned raw `0x70109`, idnum `0x7`, rev `0x109`;
- prerequisite ownership report visibility/control output only, including
  source-backed metadata for RGMII-ID `phy1`, GPIO32 PHY reset, PHY/MDIO
  policy, clocks, interrupts, DMA, and descriptors;
- accepted CLK_ETH_TSU_CTRL and CLK_ETH_CTRL idempotent write/readback/restore
  proofs, each closed to one Ethernet-private clock-manager register and
  paired no-clock-write control;
- accepted Phase 11 GPIO frontiers, which document source/status and blocker
  evidence while keeping GPIO ownership, function changes, RIO/pad/INTE/CTRL
  writes, and event generation unaccepted.

This contract does not reinterpret report visibility or clock write/restore
proofs as GPIO ownership, MDIO/PHY ownership, Ethernet driver readiness,
packet I/O, networking, sockets, SSH, Phase 12.2, or a phase transition.

## Source Facts

Contract id: `phase12-rp1-ethernet-gpio32-phy-reset-source-contract-v1`

| Field | Value |
| --- | --- |
| selected prerequisite | `rp1-ethernet-gpio32-phy-reset-readonly-preflight` |
| controller | `rp1_eth` |
| compatible | `raspberrypi,rp1-gem` / `cdns,macb` |
| accepted identity context | observed-window `MACB_MID` `0x1c001000fc`, raw `0x70109`, idnum `0x7`, rev `0x109` |
| PHY mode | `rgmii-id` |
| PHY handle/node/reg | `phy1` / `ethernet-phy@1` / `0x1` |
| reset route | RP1 GPIO32 / `ETH_RST_N` |
| polarity | active-low |
| logical assertion | Linux GPIO logical value `1` asserts reset; because the line is active-low, physical ETH_RST_N is low |
| logical deassertion | Linux GPIO logical value `0` deasserts reset; because the line is active-low, physical ETH_RST_N is high |
| source reset duration | 5 ms |
| Linux hook relationship | `macb_mdio_reset` is installed as the MDIO bus reset hook and asserts then deasserts `phy_reset_gpio` |
| selected Talos access | local/static/read-only preflight report construction only |
| hardware access selected by this task | none |

Retained Raspberry Pi Linux source facts:

- `linux-rpi-6.12-rp1.dtsi` lines 999-1015 define `rp1_eth` at
  `ethernet@100000`, compatible `raspberrypi,rp1-gem` / `cdns,macb`, with
  `phy-mode = "rgmii-id"` and the Ethernet clock/interrupt prerequisites.
- `linux-rpi-6.12-bcm2712-rpi-5-b.dts` lines 169-180 enable `rp1_eth`, set
  `phy-handle = <&phy1>`, define `phy-reset-gpios = <&rp1_gpio 32
  GPIO_ACTIVE_LOW>`, set `phy-reset-duration = <5>`, and define
  `ethernet-phy@1` at reg `0x1`.
- `linux-rpi-6.12-cdns-macb.yaml` lines 94-97 require/allow `phy-mode` and
  `phy-handle`, and lines 156-163 allow `reset-gpios` on Ethernet PHY child
  nodes.
- `linux-rpi-6.12-macb_main.c` lines 5407-5420 acquire the optional
  `"phy-reset"` GPIO as `GPIOD_OUT_LOW`, read `phy-reset-duration`, default to
  10 ms, and clamp the duration to at most 1000 ms.
- `linux-rpi-6.12-macb_main.c` lines 500-508 implement `macb_mdio_reset` by
  setting the PHY reset GPIO logical value to `1`, sleeping
  `phy_reset_ms`, then setting it to `0`.
- `linux-rpi-6.12-macb_main.c` lines 1074-1085 allocate the MDIO bus and
  install `macb_mdio_reset` as `bp->mii_bus->reset`.

## Read-Only Preflight Follow-Up

The mechanically selected follow-up is
`phase12-rp1-ethernet-gpio32-phy-reset-preflight-core-20260610`. It may only
implement local/static report construction for a later read-only hardware
visibility proof. It must not assert or deassert reset.

Candidate report fields must include:

- source contract id and source task id;
- accepted input frontier: observed-window MACB_MID identity, prerequisite
  report visibility, CLK_ETH_TSU_CTRL proof closeout, and CLK_ETH_CTRL proof
  closeout;
- `rp1_eth` controller identity, compatible strings, PHY mode, PHY
  handle/node/reg, GPIO controller `rp1_gpio`, GPIO line 32, ETH_RST_N route,
  active-low polarity, logical assertion/deassertion mapping, reset duration
  5 ms, and MDIO reset hook relationship;
- Phase 11 GPIO constraints: GPIO ownership, function changes, RIO OUT/OE/IN
  writes, pad writes, INTE/CTRL writes, event generation, interrupt delivery,
  and GPIO write/restore authority remain unaccepted;
- rejected claims, retained risks, source evidence, and future hardware-proof
  boundary classification.

The paired control must use the same report path while withholding
candidate-only GPIO32/ETH_RST_N/PHY-reset facts and carrying classification
`no-gpio-no-ethernet-rp1-ethernet-gpio32-phy-reset-control`.

Accepted local/static classifications for the follow-up:

- `rp1-ethernet-gpio32-phy-reset-preflight-candidate-local-static`
- `no-gpio-no-ethernet-rp1-ethernet-gpio32-phy-reset-control`
- `contract-rejected-input`

The later read-only Pi 5 proof, if selected by the preflight closeout, may
classify only preflight report visibility/control output or a precise staging,
capture, or source-contract blocker. It must not accept GPIO ownership, PHY
reset assertion/deassertion, MDIO/PHY ownership, packet I/O, networking,
sockets, SSH, Phase 12.2, or a phase transition.

## Future Write/Restore Safety Invariants

This source contract does not select a write-backed PHY reset task. A later
supervisor-planned write/restore task must first provide separate acceptance
criteria for all of the following:

- accepted GPIO32 ownership or a precise pre-state/restore contract that does
  not rely on ownership by implication;
- source-backed and hardware-visible GPIO function, RIO OUT/OE/IN, pad state,
  and any required output-enable state before assertion;
- logical-to-physical polarity handling for active-low ETH_RST_N;
- pre-state capture, bounded assertion duration, deassertion, restore, and
  post-restore readback evidence;
- paired no-GPIO/no-Ethernet control on the same report/capture path;
- failure classifications for assertion mismatch, deassertion mismatch,
  restore failure, staging/capture blocker, and source-contract blocker.

No later task may infer PHY/MDIO ownership, packet path readiness, interrupts,
DMA/descriptors, networking, sockets, SSH, Phase 12.2, or a phase transition
from this source contract alone.

## Findings

- fixed: identified the exact source GPIO route as RP1 GPIO32 / ETH_RST_N from
  retained Pi 5 device-tree source.
- fixed: recorded the active-low logical/physical reset semantics: Linux
  logical assertion value `1` drives the active-low reset line physically low,
  and logical deassertion value `0` drives it physically high.
- fixed: recorded source reset duration as 5 ms from the Pi 5 device tree.
- fixed: tied the reset sequence to the Linux MACB MDIO bus reset hook rather
  than to broad Ethernet driver readiness.
- fixed: reconciled accepted Phase 11 GPIO frontiers as blockers for direct
  GPIO/RIO/pad/function writes in this task.
- fixed: selected only the local/static read-only preflight report core as the
  next mechanically objective task.
- deferred: read-only Pi 5 preflight visibility, write-backed GPIO32 reset
  ownership, MDIO/PHY ownership, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition remain future tasks.
- not-an-issue: no hardwareTestLock was acquired because this task is
  source/static and performs no hardware action.

No findings were removed.

## Rejected Claims And Retained Risks

Rejected claims:

- GPIO ownership;
- PHY reset assertion or deassertion;
- MDIO transactions or PHY ownership;
- runtime Ethernet driver readiness;
- broad Ethernet MMIO readiness;
- RP1 MMIO, GPIO, RIO, pad, INTE, CTRL, or clock writes;
- packet I/O, networking, sockets, SSH;
- Phase 12.2 work or phase transition.

Retained risks:

- GPIO32/ETH_RST_N source facts do not prove Talos can safely drive or restore
  the line.
- Phase 11 GPIO frontiers have not accepted ownership or write/restore
  authority for GPIO32.
- Linux ties the reset sequence to MDIO bus reset; Talos still lacks accepted
  MDIO/PHY ownership.
- A later hardware proof must still use candidate/control identity, serial
  freshness, TFTP delta, final identity, restore evidence, and task-owned JSON
  if hardware publication is selected.

## Evidence

- Task record:
  tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-source-contract.md.
- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-source-contract/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-source-contract/evidence-map.json.
- Accepted observed-window proof closeout:
  tasks/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-proof-closeout.md.
- Accepted prerequisite proof closeout:
  tasks/2026-06-10-phase12-rp1-ethernet-prereq-ownership-proof-closeout.md.
- Accepted CLK_ETH_TSU_CTRL proof closeout:
  tasks/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-proof-closeout.md.
- Accepted CLK_ETH_CTRL proof closeout:
  tasks/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-proof-closeout.md.
- Accepted Phase 11 GPIO frontier:
  tasks/2026-06-09-phase11-rp1-irq-clock-gpio-milestone-closeout.md.
- Retained Linux Ethernet source:
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi.
- Retained Pi 5 board source:
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-bcm2712-rpi-5-b.dts.
- Retained Linux MACB source:
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c.
- Retained Linux MACB binding:
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-cdns-macb.yaml.
- Phase 11 GPIO source notes:
  tasks/evidence/2026-06-07-phase11-rp1-irq-clock-gpio-source-contract/source-reference-notes.md.
- Project docs:
  docs/src/project/phase12-networking-ssh.md and
  docs/src/project/phase11-rp1-irq-clock-gpio-contract.md.
- Roadmap:
  docs/src/roadmap.md.

## Validation

- static inspection: accepted Phase 12 proof closeouts, accepted Phase 11 GPIO
  frontiers, retained Linux source excerpts, Phase 12 docs, roadmap, and
  touched evidence reviewed.
- JSON validation: jq empty on classification/evidence-map JSON passed.
- diff check: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed.

## Next Action

Mechanically promote
`phase12-rp1-ethernet-gpio32-phy-reset-preflight-core-20260610` on the next
worker wake. Keep that task local/static: do not run hardware, acquire
hardwareTestLock, write GPIO/RIO/pad/MMIO, assert/deassert PHY reset, perform
MDIO, implement packet I/O/networking/sockets/SSH, start Phase 12.2, or infer
a phase transition.
