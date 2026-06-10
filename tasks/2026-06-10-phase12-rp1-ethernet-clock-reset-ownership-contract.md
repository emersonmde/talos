# Phase 12 RP1 Ethernet Clock/Reset Ownership Contract

Task: phase12-rp1-ethernet-clock-reset-ownership-contract-20260610

Status: accepted

Classification: rp1-ethernet-clock-reset-ownership-contract-accepted

Evidence level: static inspection of accepted prerequisite ownership proof
closeout, Phase 12 docs, Phase 11 clock/reset closeouts, retained Raspberry Pi
Linux source excerpts, and task-owned JSON. No Pi 5 hardware run, RP1 MMIO
write, clock/reset write, or runtime ownership implementation was performed.

## Goal

Define the smallest safe RP1 Ethernet clock/reset ownership slice after the
accepted MACB_MID identity proof and prerequisite ownership report visibility.

## Scope

- Consumed accepted prerequisite ownership proof closeout commit
  a6757c56f5ec405c7cd8d650ad284eb77128b6eb.
- Preserved the accepted observed-window identity context: SYSINFO_CHIP_ID at
  0x1c00000000 returned 0x20001927 and observed-window MACB_MID at
  0x1c001000fc returned raw 0x70109, idnum 0x7, rev 0x109.
- Reconciled Linux rp1_eth clock/reset source facts with accepted Phase 11
  clock/reset frontiers.
- Defined the exact source/API ownership surfaces, invariants, read-only
  baseline requirements, and retained risks required before any future
  write-backed Ethernet clock/reset ownership task.
- Selected the local/static clock-reset guard core as the next mechanically
  objective follow-up.
- Recorded findings with disposition.

## Non-Goals

No runtime implementation outside contract/docs/evidence, no Pi 5 hardware run,
no boot archive publication, no hardwareTestLock acquisition, no RP1 MMIO
writes, no clock/reset writes or ownership, no GPIO32 or PHY reset ownership,
no MDIO transactions, no DMA, no descriptor rings, no interrupts/completions,
no packet I/O, no networking, no sockets, no SSH, no Phase 12.2 work, and no
phase transition.

## Accepted Input Frontier

The accepted input frontier is:

- observed-window MACB_MID identity only: 0x1c001000fc returned raw 0x70109,
  idnum 0x7, rev 0x109 under capture-chain-v4;
- prerequisite report visibility/control output only:
  phase12-rp1-ethernet-prereq-ownership-pi5-proof-20260610 printed the
  candidate prerequisite metadata and paired no-ownership/no-Ethernet control
  through the same report path;
- Phase 11 clock/reset evidence remains narrow: read-only/status snapshots,
  one selected CLK_ADC_CTRL idempotent write/restore boundary, a blocked
  non-idempotent enable-toggle attempt, and observed-aperture dependency
  evidence, not broad clock/reset ownership.

This contract does not reinterpret report visibility as runtime ownership and
does not accept Ethernet driver readiness, broad Ethernet MMIO readiness,
RP1 MMIO writes, GPIO32/PHY reset ownership, MDIO/PHY ownership, interrupts,
DMA, descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase
transition.

## Source And Ownership Surfaces

Retained Raspberry Pi Linux source facts identify these exact ownership inputs:

| Surface | Accepted source fact | Contract role |
| --- | --- | --- |
| rp1_eth device node | raspberrypi,rp1-gem / cdns,macb at RP1 bus 0xc0_40100000 | Ethernet identity context only |
| clock API names | pclk, hclk, tsu_clk, tx_clk | Future Talos clock owner API inputs |
| clock IDs | pclk=RP1_CLK_SYS 12, hclk=RP1_CLK_SYS 12, tsu_clk=RP1_CLK_ETH_TSU 29, tx_clk=RP1_CLK_ETH 16 | Source-owned IDs for guard reporting |
| Linux enable path | macb_clk_init obtains clocks and calls clk_prepare_enable on pclk, hclk, tx_clk, optional rx_clk, and tsu_clk | Source evidence for future ownership invariants |
| rp1_eth reset controller | no rp1_eth resets/reset-names are present in the retained Pi 5 rp1_eth node; cdns,macb.yaml permits a reset controller for other platforms | No Talos reset-controller write target accepted |
| PHY reset route | phy-reset-gpios = RP1 GPIO32 active low, duration 5 ms, toggled by Linux MDIO reset path | Separate GPIO/PHY/MDIO ownership, not clock/reset ownership |

The exact accepted API surface for the next implementation is a local/static
guard/report, not a hardware register operation. A future write-backed task
must first supply a source contract that maps any target clock ID to the exact
Talos clock-manager register, bit fields, pre-read/readback/restore sequence,
and shared-clock safety policy. This contract intentionally accepts no raw
Ethernet clock register write target.

## Read-Only Baseline Requirements

Before any future write-backed Ethernet clock/reset task can be accepted, a
read-only baseline must record:

- the same candidate/control reporting path selected here;
- accepted observed-window MACB_MID identity context as context only;
- pclk/hclk/tsu_clk/tx_clk names and IDs exactly as source-backed metadata;
- that pclk and hclk share RP1_CLK_SYS and are therefore shared-clock guarded;
- that tx_clk and tsu_clk are Ethernet-specific source IDs but still have no
  accepted Talos register write target in this contract;
- absence of an accepted rp1_eth reset-controller target in the retained Pi 5
  device-tree node;
- a paired no-clock-reset/no-Ethernet control that withholds candidate-only
  Ethernet clock/reset facts.

That baseline may prove report visibility only unless a later task supplies
different explicit acceptance criteria.

## Future Write-Backed Ownership Invariants

A future write-backed ownership task must satisfy all of these invariants
before it can be accepted:

- do not disable, gate, or transition RP1_CLK_SYS through pclk/hclk because it
  is shared by system users;
- do not touch any reset-controller path unless a source contract proves an
  rp1_eth reset target exists on the Pi 5 path and defines restore semantics;
- do not fold PHY reset into clock/reset ownership; GPIO32 and MDIO remain
  separate ownership tasks;
- use pre-read, post-read, restore-write, and restore-read evidence for every
  selected writable clock register;
- preserve non-target fields such as source/divider/auxsource bits unless a
  future contract explicitly selects them;
- include a paired no-clock-reset/no-Ethernet control before any hardware proof;
- reject any inference from clock/reset ownership to PHY/MDIO, interrupts,
  DMA, descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or phase
  transition.

## Selected Next Follow-Up

The next mechanically objective follow-up is
phase12-rp1-ethernet-clock-reset-guard-core-20260610.

That task may implement only a local/static candidate and paired-control guard
surface derived from this contract. The candidate must preserve the accepted
input frontier, exact clock source facts, API ownership targets, read-only
baseline requirements, future write-backed invariants, rejected claims, and
retained risks. The control must use the same report path while withholding
candidate-only Ethernet clock/reset facts and carrying a no-clock-reset/
no-Ethernet control classification.

## Findings

- fixed: separated prerequisite report visibility from runtime clock/reset
  ownership.
- fixed: identified the exact source/API clock ownership inputs:
  pclk/hclk/tsu_clk/tx_clk, RP1_CLK_SYS/RP1_CLK_ETH_TSU/RP1_CLK_ETH IDs, and
  Linux macb_clk_init enable behavior.
- fixed: recorded that pclk and hclk share RP1_CLK_SYS and must not be
  disabled or transitioned by an Ethernet ownership task.
- fixed: recorded that the retained Pi 5 rp1_eth node supplies no accepted
  reset-controller target, while PHY reset is GPIO32/MDIO-owned and separate.
- fixed: required read-only baseline evidence before any write-backed
  ownership proof.
- fixed: selected the local/static guard core as the next bounded follow-up.
- deferred: local/static guard implementation, any Pi 5 baseline proof,
  write-backed clock ownership, GPIO32/PHY reset ownership, MDIO/PHY,
  interrupts, DMA, descriptors, packet I/O, networking, sockets, SSH, Phase
  12.2, and phase transition remain future work.
- not-an-issue: no hardwareTestLock was acquired because this task is
  contract/docs/evidence only.

No findings were removed.

## Rejected Claims And Retained Risks

Rejected claims:

- Ethernet driver readiness;
- broad Ethernet MMIO readiness;
- RP1 MMIO writes;
- clock/reset writes or ownership;
- RP1_CLK_SYS transition through pclk or hclk;
- reset-controller ownership;
- GPIO32 ownership or PHY reset assertion/deassertion;
- MDIO transactions or PHY ownership;
- interrupt delivery, handler ownership, or completion;
- DMA, descriptor rings, channel ownership, or transfer completion;
- packet I/O, networking, sockets, SSH;
- Phase 12.2 work or phase transition.

Retained risks:

- Clock names and IDs are source facts, not Talos ownership.
- pclk/hclk are shared-clock inputs and require stricter safety than
  Ethernet-private clocks.
- tx_clk and tsu_clk still require a future source contract that identifies
  exact Talos register targets and restore semantics before hardware writes.
- The retained Pi 5 rp1_eth source node does not provide an accepted
  reset-controller target.
- PHY reset remains GPIO32/MDIO-owned and must not be hidden under
  clock/reset work.

## Evidence

- Task record:
  tasks/2026-06-10-phase12-rp1-ethernet-clock-reset-ownership-contract.md.
- Classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-ownership-contract/classification.json.
- Evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-clock-reset-ownership-contract/evidence-map.json.
- Accepted prerequisite proof closeout:
  tasks/2026-06-10-phase12-rp1-ethernet-prereq-ownership-proof-closeout.md.
- Accepted prerequisite Pi 5 proof:
  tasks/2026-06-10-phase12-rp1-ethernet-prereq-ownership-pi5-proof.md.
- Accepted source contract:
  tasks/2026-06-10-phase12-rp1-ethernet-prereq-ownership-source-contract.md.
- Retained Raspberry Pi Linux sources under
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/.
- Phase 11 clock/reset docs:
  docs/src/project/phase11-rp1-pcie-map-contract.md and
  tasks/2026-06-09-phase11-rp1-irq-clock-gpio-milestone-closeout.md.
- Project docs:
  docs/src/project/phase12-networking-ssh.md and docs/src/roadmap.md.

## Validation

- static inspection: reviewed accepted prerequisite proof closeout, Phase 12
  docs, Phase 11 clock/reset closeouts, retained Linux source excerpts, and
  touched docs/evidence.
- JSON checks: jq empty on task-owned evidence-map/classification JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Contract states the accepted input frontier and rejected claims from the
  prerequisite ownership proof closeout: satisfied.
- Contract identifies exact clock/reset ownership sources, required
  invariants, readback/baseline requirements, and risks before any future
  write-backed ownership task: satisfied.
- Contract separates read-only baseline evidence from write-backed
  clock/reset ownership and rejects broad Ethernet driver readiness by
  implication: satisfied.
- NextAction selects one bounded follow-up only if mechanically objective:
  satisfied; the local/static guard core is selected.
- Accepted contract is committed before any follow-up starts: satisfied by the
  commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-clock-reset-guard-core-20260610 on the next worker wake.
That task may implement only local/static candidate and paired-control
clock/reset guard/report construction from this contract. It must not run
hardware, publish a boot archive, acquire hardwareTestLock, program RP1 MMIO,
write clocks/resets, assert or deassert PHY reset, perform MDIO transactions,
create DMA descriptors or rings, claim interrupts/completions, perform packet
I/O, add networking/sockets/SSH, start Phase 12.2, or infer a phase
transition.
