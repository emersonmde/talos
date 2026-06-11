# Phase 12 RP1 Ethernet MDIO PHY ID Source Contract

Task id: phase12-rp1-ethernet-mdio-phy-id-source-contract-20260611

Status: accepted

Classification: rp1-ethernet-mdio-phy-id-source-contract-accepted

Evidence level: static inspection of retained Raspberry Pi Linux device-tree
sources, retained Cadence MACB/GEM Linux source, fetched matching
Raspberry Pi Linux macb.h and uapi mii.h excerpts, accepted Phase 12 task
records, Phase 12 docs, and task-owned JSON. No code implementation, Pi 5
hardware run, boot archive publication, hardwareTestLock acquisition,
runtime MDIO transaction, GPIO32/PHY reset write, Ethernet driver behavior,
DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or phase
transition was performed.

## Goal

Define a source-backed contract for the smallest useful MDIO/PHY-ID
discriminator after the accepted observed-window GEM identity and
Ethernet-private clock write/restore proofs.

## Scope Performed

- Inspected retained Raspberry Pi Linux device-tree evidence for rp1_eth,
  phy1, and ethernet-phy@1.
- Inspected retained Linux Cadence MACB/GEM MDIO source for management port
  enable, Clause 22 read construction, idle polling, and result extraction.
- Fetched and retained the matching Raspberry Pi Linux rpi-6.12.y macb.h and
  uapi linux/mii.h excerpts required to bind register offsets and PHY ID
  register numbers to source.
- Preserved the accepted frontier: observed-window MACB_MID identity,
  prerequisite ownership report visibility, CLK_ETH_TSU_CTRL and CLK_ETH_CTRL
  idempotent write/restore proofs, and the GPIO32 event-clear
  persistent/firmware-owned blocker.
- Defined the paired no-MDIO/no-Ethernet control requirements.
- Recorded findings with disposition.

## Non-Goals

No runtime implementation, no Pi 5 hardware run, no boot archive publication,
no hardwareTestLock acquisition, no actual MDIO transaction, no GPIO32/PHY
reset write, no PHY reset assertion/deassertion, no Ethernet driver behavior,
no interrupt handling, no DMA/descriptors, no packet I/O, no networking, no
sockets, no SSH, no Phase 12.2 work, and no phase transition.

## Reconciled Inputs

- tasks/2026-06-11-phase12-rp1-ethernet-clock-reset-prereq-closeout.md.
- tasks/2026-06-10-phase12-rp1-ethernet-observed-window-discriminator-proof-closeout.md.
- tasks/2026-06-10-phase12-rp1-ethernet-prereq-ownership-proof-closeout.md.
- tasks/2026-06-10-phase12-rp1-ethernet-clock-reset-write-restore-proof-closeout.md.
- tasks/2026-06-10-phase12-rp1-ethernet-clk-eth-ctrl-write-restore-proof-closeout.md.
- tasks/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-proof-closeout.md.
- tasks/2026-06-09-phase12-rp1-ethernet-source-inventory.md.
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi.
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-bcm2712-rpi-5-b.dts.
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-cdns-macb.yaml.
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-macb.h.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-uapi-linux-mii.h.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.

## Source Facts

- rp1.dtsi defines rp1_eth as ethernet@100000, compatible with
  raspberrypi,rp1-gem and cdns,macb, with source RP1 bus register window
  0xc0_40100000 size 0x4000.
- bcm2712-rpi-5-b.dts enables rp1_eth, assigns phy-handle phy1, and declares
  phy1 as ethernet-phy@1 with reg 0x1. The same node also records GPIO32
  active-low reset metadata, but this contract does not require reset success
  or a GPIO32 write.
- The accepted observed-window identity proof maps the selected live
  observed-window base to 0x1c00100000, with MACB_MID at 0x1c001000fc
  returning raw 0x70109, idnum 0x7, rev 0x109. The 0x1f00100000 translated
  comparator window remains sentinel/comparator-only for this contract.
- macb_main.c enables the management port by writing MACB_BIT(MPE) to NCR
  before registering the MII bus.
- macb_main.c implements Clause 22 reads by polling NSR.IDLE, writing MAN with
  SOF=MACB_MAN_C22_SOF, RW=MACB_MAN_C22_READ, PHYA=mii_id, REGA=regnum, and
  CODE=MACB_MAN_C22_CODE, polling NSR.IDLE again, then extracting MAN.DATA.
- macb.h defines NCR offset 0x0000, NSR offset 0x0008, MAN offset 0x0034,
  NCR.MPE bit offset 4, NSR.IDLE bit offset 2, and MAN fields DATA[15:0],
  CODE[17:16], REGA[22:18], PHYA[27:23], RW[29:28], SOF[31:30].
- macb.h defines Clause 22 SOF=1, READ=2, WRITE=1, and CODE=2.
- uapi linux/mii.h defines MII_PHYSID1 register 0x02 and MII_PHYSID2 register
  0x03.

## Selected Contract

The smallest useful future discriminator is a paired Clause 22 PHY-ID read
candidate/control report:

| Field | Value |
| --- | --- |
| contract id | phase12-rp1-ethernet-mdio-phy-id-source-contract-v1 |
| selected discriminator | rp1-ethernet-mdio-clause22-phy1-physid1-physid2 |
| controller | rp1_eth / raspberrypi,rp1-gem / cdns,macb |
| PHY source identity | phy1 / ethernet-phy@1 / reg 0x1 |
| PHY address | 1 |
| PHY ID registers | MII_PHYSID1 0x02 and MII_PHYSID2 0x03 |
| source RP1 bus base | 0xc0_40100000 |
| observed-window base | 0x1c00100000 |
| comparator window | 0x1f00100000, sentinel/comparator-only |
| NCR target | offset 0x0000, observed 0x1c00100000 |
| NSR target | offset 0x0008, observed 0x1c00100008 |
| MAN target | offset 0x0034, observed 0x1c00100034 |
| required precondition | read NCR and require MPE already set before any candidate read, or classify source-contract-violated-blocker without writing NCR |
| idle polling | poll NSR.IDLE bit 2 before and after each MAN write |
| timeout policy | bounded polling, source-derived Linux timeout 1000000 usec equivalent or tighter Talos-owned bounded poll |
| result extraction | MAN.DATA bits 15:0 after post-read idle |
| no-restore expectation | no restore write for MAN because MDIO reads are transactions; no NCR MPE write is allowed in the first proof |

The future candidate may construct two Clause 22 read frames, one for register
0x02 and one for register 0x03, only after the no-write MPE precondition and
initial idle poll pass. The exact MAN values are:

- PHYSID1: (SOF 1 << 30) | (RW read 2 << 28) | (PHYA 1 << 23) |
  (REGA 2 << 18) | (CODE 2 << 16) = 0x600a0000.
- PHYSID2: (SOF 1 << 30) | (RW read 2 << 28) | (PHYA 1 << 23) |
  (REGA 3 << 18) | (CODE 2 << 16) = 0x600e0000.

The accepted source contract does not itself execute those writes. A later
guard/core task must make the report shape and validators explicit before any
serialized Pi 5 proof is considered.

## Operation Order For A Future Candidate

1. Print a candidate start marker and accepted input frontier.
2. Read observed-window MACB_MID context at 0x1c001000fc as context only.
3. Read NCR at 0x1c00100000 and require MPE bit 4 already set.
4. If MPE is clear, perform no write and classify source-contract-violated-blocker.
5. Poll NSR at 0x1c00100008 until IDLE bit 2 is set or the bounded timeout
   expires.
6. Write MAN at 0x1c00100034 with 0x600a0000 for Clause 22 PHY address 1,
   register 0x02.
7. Poll NSR.IDLE again; read MAN and extract DATA[15:0] as physid1.
8. Repeat the same poll/write/poll/read sequence with MAN value 0x600e0000 for
   register 0x03 and extract physid2.
9. Classify only the selected discriminator outcome. Do not infer full MDIO
   ownership, PHY reset ownership, link readiness, or Ethernet driver
   readiness.

## Allowed Future Proof Classifications

- mdio-phy1-physid-visible: both Clause 22 PHY ID reads complete within the
  bounded timeout and return non-timeout 16-bit data.
- mdio-phy1-physid-timeout: initial or post-MAN NSR.IDLE polling times out.
- mdio-phy1-physid-source-contract-violated-blocker: required source
  precondition fails, including NCR.MPE clear when the proof is not allowed to
  write it.
- precise-staging-capture-blocker: candidate/control identity, serial, TFTP,
  or restore evidence is not precise enough to classify.
- no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-control: paired control output
  from the same reporting path with no MDIO target construction and no volatile
  MDIO store.

## GPIO32 Event-Clear Blocker Handling

The accepted GPIO32 event-clear proof classifies GPIO32 event state as
persistent or firmware-owned and closes same-shaped GPIO32 retries. This MDIO
source contract deliberately does not require PHY reset success, GPIO32
ownership, GPIO32 event clearing, or GPIO32 writes. The future discriminator
may only test the selected MDIO/PHY-ID transaction path when the guard allows
it; it must keep GPIO32 reset state as retained risk, not accepted ownership.

## Paired Control Boundary

The paired control must preserve the same capture/reporting path while:

- constructing no MDIO target address and no MAN frame;
- performing no NCR, NSR, or MAN volatile load/store for Ethernet MDIO;
- withholding candidate-only PHY address, PHY ID register, MAN frame, and
  result fields;
- retaining contract id, rejected-claim labels, and control classification;
- classifying as no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-control.

## Findings

- fixed: selected the smallest source-backed non-GPIO32 discriminator as
  Clause 22 PHY-ID reads for phy1 address 1, registers 0x02 and 0x03.
- fixed: tied the selected discriminator to retained Raspberry Pi device-tree
  facts for rp1_eth and ethernet-phy@1/reg 0x1.
- fixed: tied the management transaction path to Linux MACB source for MPE,
  NSR.IDLE polling, MAN frame construction, and MAN.DATA extraction.
- fixed: fetched and retained matching source headers for exact MACB register
  offsets, bit fields, Clause 22 constants, and MII PHY ID register numbers.
- fixed: required a no-write MPE precondition for the first candidate so this
  source contract does not silently expand into NCR ownership.
- fixed: defined paired no-MDIO/no-Ethernet control requirements.
- deferred: local/static guard implementation, serialized Pi 5 proof, actual
  MDIO transaction evidence, and any decision to allow an NCR.MPE write remain
  future supervisor-owned tasks.
- deferred: GPIO32/PHY reset ownership remains blocked by accepted event-state
  and event-clear evidence.
- not-an-issue: no hardwareTestLock was acquired because this task is
  source/docs/evidence scoped.
- removed: no obsolete implementation, docs, or task evidence was removed.

## Rejected Claims And Retained Risks

Rejected claims:

- Ethernet driver readiness;
- broad Ethernet MMIO readiness beyond the selected observed-window targets;
- MDIO ownership beyond the selected PHY-ID discriminator;
- NCR.MPE ownership or write permission;
- PHY reset ownership;
- GPIO32 ownership, event clearing, write/restore retry, or reset success;
- interrupt delivery, handler ownership, or completion;
- DMA/descriptors, channel ownership, or transfer completion;
- packet I/O;
- networking;
- sockets;
- SSH;
- Phase 12.2;
- phase transition.

Retained risks:

- NCR.MPE may be clear; the first candidate must classify that as a
  source-contract blocker unless a later explicit task permits the MPE write.
- GPIO32 / ETH_RST_N remains unowned and may leave the PHY in reset or another
  firmware-owned state.
- A visible PHY ID read would prove only the selected management transaction,
  not full MDIO, PHY, link, MAC, DMA, interrupt, packet, or network readiness.
- Timeout policy still needs a Talos-owned local/static guard and test surface
  before hardware.

## Evidence

- Source task record:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract.md.
- Fetched source excerpts and checksums:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/.
- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/classification.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/evidence-map.json.
- Project docs:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap:
  docs/src/roadmap.md.

## Validation

- static inspection: retained Linux/Raspberry Pi source evidence, fetched
  matching headers, accepted Phase 12 task records, project docs, and roadmap.
- JSON checks: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Contract identifies exact source-backed MDIO/PHY-ID target, PHY address,
  registers, GEM management/status registers, observed-window addresses,
  operation order, timeout/precondition policy, and allowed future proof
  classifications: satisfied.
- Contract explicitly handles GPIO32 event-clear blocker by not requiring PHY
  reset success or GPIO32 writes: satisfied.
- Paired control requirements with no MDIO/no Ethernet target construction are
  explicit: satisfied.
- Rejected claims include Ethernet driver readiness, MDIO ownership beyond the
  selected discriminator, PHY reset ownership, interrupts, DMA/descriptors,
  packet I/O, networking, sockets, SSH, Phase 12.2, and phase transition:
  satisfied.
- Accepted contract is committed before the guard core starts: satisfied by
  the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-phy-id-guard-core-20260611 on the next worker wake
if dependencies remain satisfied, hardwareTestLock is unlocked, and
supervisorIntervention.active remains false. Keep that task limited to the
already queued local/static guard/report surface; do not run hardware or
perform runtime MDIO transactions unless a later explicit proof task owns that
scope.
