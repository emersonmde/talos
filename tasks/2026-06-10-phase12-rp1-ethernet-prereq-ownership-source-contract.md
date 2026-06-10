# Phase 12 RP1 Ethernet Prerequisite Ownership Source Contract

Task: phase12-rp1-ethernet-prereq-ownership-source-contract-20260610

Status: accepted

Classification: rp1-ethernet-prereq-ownership-source-contract-accepted

Evidence level: static inspection of accepted observed-window MACB_MID proof
closeout, Phase 12 source inventory/path ADR, retained Raspberry Pi Linux
source excerpts, Phase 11 clock/GPIO/interrupt/DMA closeouts, project docs, and
task-owned JSON. No Pi 5 hardware run or runtime implementation was performed.

## Goal

Define the next source-backed RP1 Ethernet prerequisite ownership contract after
the accepted observed-window MACB_MID identity proof.

## Scope

- Consumed the accepted observed-window MACB_MID proof closeout at commit
  6d1dd2647e03b467261b3518d6350f4c4ffd9661.
- Preserved the accepted identity context: SYSINFO_CHIP_ID at 0x1c00000000
  returned 0x20001927 and observed-window MACB_MID at 0x1c001000fc returned
  raw 0x70109, idnum 0x7, rev 0x109.
- Reconciled retained Linux source facts for rp1_eth clocks, clock names,
  RP1_INT_ETH, PHY reset GPIO32, PHY/MDIO relationship, and
  DMA/descriptor/interrupt dependencies.
- Compared those source facts with accepted Phase 11 frontiers for
  clock/reset, GPIO ownership, interrupt delivery, and DMA/cache.
- Selected the next mechanically objective follow-up as local/static
  prerequisite ownership report construction, not hardware or driver work.
- Recorded findings with disposition.

## Non-Goals

No implementation changes outside contract/docs/evidence, no Pi 5 hardware
run, no boot archive publication, no hardwareTestLock acquisition, no Ethernet
driver implementation, no RP1 MMIO writes, no clock/reset writes, no GPIO
writes, no PHY reset assertion/deassertion, no MDIO transactions, no DMA, no
descriptor rings, no interrupts, no packet I/O, no networking, no sockets, no
SSH, no Phase 12.2 work, and no phase transition.

## Reconciled Inputs

- tasks/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-proof-closeout.md.
- tasks/2026-06-09-phase12-rp1-ethernet-source-inventory.md.
- tasks/2026-06-09-phase12-rp1-ethernet-path-adr.md.
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi.
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-bcm2712-rpi-5-b.dts.
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-cdns-macb.yaml.
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c.
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1-mfd.h.
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1-clock.h.
- tasks/2026-06-09-phase11-rp1-irq-clock-gpio-milestone-closeout.md.
- tasks/2026-06-09-phase11-rp1-dma-cache-milestone-11-3-closeout.md.
- tasks/2026-06-09-phase11-rp1-hardware-substrate-closeout.md.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/project/phase11-rp1-pcie-map-contract.md.
- docs/src/roadmap.md.

## Selected Prerequisite

The selected next prerequisite is the RP1 Ethernet prerequisite ownership
report contract:

| Field | Value |
| --- | --- |
| contract id | phase12-rp1-ethernet-prereq-ownership-contract-v1 |
| selected prerequisite | rp1-ethernet-clock-reset-phy-mdio-dma-ownership-report |
| controller | rp1_eth |
| compatible | raspberrypi,rp1-gem / cdns,macb |
| accepted identity context | observed-window MACB_MID 0x1c001000fc raw 0x70109 / idnum 0x7 / rev 0x109 |
| identity role | context-only, not broad Ethernet MMIO readiness |
| source register window | RP1 bus 0xc0_40100000 size 0x4000 |
| observed-window identity target | 0x1c001000fc |
| translated-window comparator | 0x1f001000fc, retained sentinel-only |
| interrupt source fact | RP1_INT_ETH = 6 |
| clock source facts | pclk=RP1_CLK_SYS, hclk=RP1_CLK_SYS, tsu_clk=RP1_CLK_ETH_TSU, tx_clk=RP1_CLK_ETH |
| clock ids | RP1_CLK_SYS=12, RP1_CLK_ETH=16, RP1_CLK_ETH_TSU=29 |
| PHY mode | rgmii-id |
| PHY handle | phy1 / ethernet-phy@1 / reg 0x1 |
| PHY reset route | RP1 GPIO32, active low, source reset duration 5 ms |
| Cadence/RP1 config | gigabit, hardware clock change, jumbo, PTP, dma_burst_length 16 |
| report access | local/static report construction only |
| hardware access selected by this contract | none |

The next report is deliberately an ownership/preflight report instead of a new
MMIO discriminator. The accepted observed-window MACB_MID proof establishes a
read-only identity boundary, but Linux's RP1 GEM path still requires clock
enabling, PHY reset GPIO ownership, MDIO/PHY handling, interrupts, DMA
descriptors, and packet buffers before packet behavior. The safe next step is
to make those prerequisites explicit and machine-checkable before any worker
implements writes or driver behavior.

No new read-only hardware field is selected in this source contract. The
retained source-backed prerequisites that matter next are write-backed or
ownership-backed: Linux enables pclk/hclk/tx_clk/tsu_clk, may toggle the
PHY-reset GPIO through the MDIO bus reset hook, performs MDIO transactions,
programs descriptor ring base registers, and handles interrupts/completions.
Selecting any of those as a hardware proof in this task would either require
writes or imply ownership that Talos has not accepted. The later serialized
proof, if selected by the report closeout, may only prove that the local/static
candidate/control report is visible through the capture path.

## Source Reconciliation

- rp1.dtsi defines rp1_eth at RP1 bus 0xc0_40100000, compatible
  raspberrypi,rp1-gem / cdns,macb, RP1_INT_ETH, clocks pclk/hclk/tsu_clk/tx_clk,
  phy-mode rgmii-id, and status disabled in the shared source node.
- bcm2712-rpi-5-b.dts enables rp1_eth, assigns phy1, configures
  phy-reset-gpios as RP1 GPIO32 active low, sets phy-reset-duration to 5 ms,
  and defines ethernet-phy@1 with Broadcom powerdown/EEE quirks.
- rp1-mfd.h defines RP1_INT_ETH as 6. rp1-clock.h defines RP1_CLK_SYS as 12,
  RP1_CLK_ETH as 16, and RP1_CLK_ETH_TSU as 29.
- cdns,macb.yaml requires compatible, reg, interrupts, clocks, clock-names,
  and phy-mode, and permits phy-handle plus MDIO/ethernet-phy children.
- macb_main.c maps raspberrypi,rp1-gem to a Cadence GEM config with gigabit,
  hardware clock change, jumbo, PTP, dma_burst_length 16, macb_clk_init, and
  macb_init.
- macb_clk_init obtains pclk/hclk and optional tx_clk/rx_clk/tsu_clk and
  prepares/enables them. For RP1, the DT clock names provide pclk, hclk,
  tsu_clk, and tx_clk.
- macb_main.c obtains optional phy-reset GPIO and phy-reset-duration; its MDIO
  reset path asserts then deasserts the reset GPIO. MDIO read/write paths are
  transactions and remain forbidden by this contract.
- macb_main.c's packet path allocates DMA rings/buffers, programs RBQP/TBQP
  ring bases, maps TX buffers, uses interrupts/NAPI/completion handling, and
  adjusts packet/link state. Those are dependencies, not accepted Talos
  behavior.

## Phase 11 Frontier Reconciliation

- Accepted Phase 11 clock/reset evidence is a narrow read-only/status and
  dependency frontier. It explicitly does not accept clock/reset ownership,
  reset-controller ownership, or clock/reset writes.
- Accepted Phase 11 GPIO evidence keeps GPIO ownership, function changes,
  RIO/pad/INTE/CTRL writes, and event generation unaccepted. GPIO32 PHY reset
  therefore cannot be asserted or deasserted from this contract.
- Accepted Phase 11 interrupt evidence documents routing/status frontiers, but
  not interrupt delivery, IAR/EOIR acknowledgement, ISR/handler ownership, or
  Ethernet interrupt completion.
- Accepted Phase 11 DMA/cache evidence documents local/static ownership and
  maintenance rules plus diagnostic visibility, but not live DMA, channel
  ownership, descriptor rings, transfer completion, interrupt completion, or
  Ethernet DMA consumption.

## Required Future Candidate Fields

A later local/static implementation selected by this contract must emit a
candidate prerequisite ownership report that preserves:

- contract id phase12-rp1-ethernet-prereq-ownership-contract-v1;
- source contract id phase12-rp1-ethernet-prereq-ownership-source-contract-20260610;
- accepted observed-window MACB_MID identity context and a flag that it is
  context-only;
- controller, compatible strings, rp1_eth source register window, observed
  identity target, and translated comparator/sentinel;
- RP1_INT_ETH number 6 and rejected interrupt ownership claims;
- clock names pclk, hclk, tsu_clk, tx_clk and clock ids 12, 12, 29, 16;
- clock policy classification no-clock-reset-ownership;
- PHY mode rgmii-id, phy1/reg 0x1, RP1 GPIO32 active-low reset, and
  phy-reset-duration 5 ms;
- PHY/MDIO policy classification no-phy-reset-or-mdio-ownership;
- DMA/descriptor dependency classification no-live-dma-or-descriptor-ownership;
- rejected runtime/hardware claims and retained risks listed below;
- report classification
  rp1-ethernet-prereq-ownership-candidate-local-static.

The candidate report must construct no new MMIO targets beyond inert metadata,
must perform no loads or stores, and must not claim ownership of clocks, reset,
GPIO32, PHY, MDIO, interrupts, DMA, descriptors, packet I/O, networking,
sockets, SSH, Phase 12.2, or a phase transition.

## Paired Control Boundary

The paired control must preserve the same reporting path while withholding the
accepted rp1_eth prerequisite facts and carrying an explicit classification:
no-ownership-no-ethernet-rp1-ethernet-prereq-control.

The control may include the contract id, rejected-claim list, and retained risk
labels as inert report metadata. It must not include Ethernet clock IDs,
GPIO32, PHY/MDIO, interrupt, DMA, descriptor, or MACB_MID identity fields as
accepted candidate facts.

## Findings

- fixed: selected prerequisite ownership reporting as the next smallest
  source-backed boundary after MACB_MID identity visibility.
- fixed: reconciled rp1_eth clocks and clock names with Linux macb clock
  enable behavior while keeping Talos clock/reset ownership unaccepted.
- fixed: reconciled PHY reset GPIO32 and MDIO/PHY source facts while keeping
  Talos GPIO32, PHY reset, and MDIO ownership unaccepted.
- fixed: reconciled RP1_INT_ETH and Linux interrupt/completion dependency
  without accepting Ethernet interrupt delivery or completion.
- fixed: reconciled Cadence/RP1 DMA descriptor and packet-buffer dependencies
  against the accepted Phase 11 DMA/cache frontier.
- fixed: specified exact local/static candidate and paired-control report
  fields for the next worker-owned report-core task.
- deferred: report-core implementation, closeout, serialized report visibility
  proof, any actual clock/reset/GPIO/PHY/MDIO/DMA/interrupt ownership, Ethernet
  driver readiness, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition remain future work.
- not-an-issue: no hardwareTestLock was acquired because this task is
  source/contract scoped.

No findings were removed.

## Rejected Claims And Retained Risks

Rejected claims:

- Ethernet driver readiness;
- broad Ethernet MMIO readiness;
- RP1 MMIO writes;
- clock/reset ownership or writes;
- GPIO32 ownership or PHY reset assertion/deassertion;
- MDIO transactions or PHY ownership;
- interrupt delivery, handler ownership, or completion;
- DMA, descriptor rings, channel ownership, or transfer completion;
- packet I/O, networking, sockets, SSH;
- Phase 12.2 work or phase transition.

Retained risks:

- observed-window MACB_MID identity does not prove clocks, PHY, MDIO, DMA,
  interrupts, or packet behavior;
- source facts identify required prerequisites but not Talos ownership;
- the next report-core task is local/static only and must not be treated as a
  hardware proof;
- any later hardware proof selected by closeout is limited to report visibility
  unless a future supervisor task provides different explicit scope.

## Evidence

- Task record:
  tasks/2026-06-10-phase12-rp1-ethernet-prereq-ownership-source-contract.md.
- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-prereq-ownership-source-contract/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-prereq-ownership-source-contract/evidence-map.json.
- Accepted observed-window proof closeout:
  tasks/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-proof-closeout.md.
- Phase 12 source inventory and path ADR:
  tasks/2026-06-09-phase12-rp1-ethernet-source-inventory.md and
  tasks/2026-06-09-phase12-rp1-ethernet-path-adr.md.
- Retained Raspberry Pi Linux sources under
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/.
- Phase 11 closeouts:
  tasks/2026-06-09-phase11-rp1-irq-clock-gpio-milestone-closeout.md,
  tasks/2026-06-09-phase11-rp1-dma-cache-milestone-11-3-closeout.md, and
  tasks/2026-06-09-phase11-rp1-hardware-substrate-closeout.md.
- Project docs:
  docs/src/project/phase12-networking-ssh.md and
  docs/src/project/phase11-rp1-pcie-map-contract.md.
- Roadmap:
  docs/src/roadmap.md.

## Validation

- static inspection: reviewed accepted observed-window proof closeout, Phase
  12 source inventory/path ADR, retained Linux source excerpts, accepted Phase
  11 clock/GPIO/interrupt/DMA closeouts, project docs, and task-owned JSON.
- JSON checks: jq empty on task-owned evidence-map/classification JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Contract explicitly names the selected next prerequisite without implying
  Ethernet driver readiness or broad MMIO readiness from the observed-window
  MACB_MID proof: satisfied.
- Contract reconciles rp1_eth clocks, clock names, PHY reset GPIO32, PHY/MDIO,
  interrupts, and DMA/descriptor dependencies against Phase 11 frontiers:
  satisfied.
- Follow-up implementation fields are exact enough for a worker to implement
  mechanically, with forbidden writes and ownership claims enumerated:
  satisfied.
- If no follow-up is mechanically objective, planningNeeded is set true:
  not applicable; the local/static report-core follow-up is mechanically
  objective.
- Accepted contract/evidence is committed before any follow-up starts:
  satisfied by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-prereq-ownership-report-core-20260610 on the next worker
wake. That task may implement only local/static candidate and paired-control
prerequisite ownership report construction for this contract. It must not run
hardware, publish a boot archive, acquire hardwareTestLock, program RP1 MMIO,
write clocks/resets/GPIO, assert or deassert PHY reset, perform MDIO
transactions, create DMA descriptors or rings, claim interrupts/completions,
perform packet I/O, add networking/sockets/SSH, start Phase 12.2, or infer a
phase transition.
