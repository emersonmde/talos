# Phase 12 RP1 Ethernet MDIO PHY ID After-MPE Source Contract

Task id: phase12-rp1-ethernet-mdio-phy-id-after-mpe-source-contract-20260611

Status: accepted

Classification: rp1-ethernet-mdio-phy-id-after-mpe-source-contract-accepted

Evidence level: static inspection of the accepted MDIO PHY-ID no-write
blocker, accepted NCR.MPE proof and closeout, retained Raspberry Pi Linux
MACB/RP1 source evidence, rp1_ethernet source, Phase 12 docs, roadmap, and
task-owned JSON. No runtime implementation, Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, runtime MMIO load/store, NCR write,
MAN write, PHY-ID read, PHY reset/GPIO32 action, Ethernet driver behavior,
DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or phase
transition was performed.

## Goal

Define the smallest source-backed corrected-target MDIO PHY-ID discriminator
after the accepted NCR.MPE set/readback/restore ownership proof.

## Scope Performed

- Consumed the accepted MDIO PHY-ID no-write blocker and the accepted NCR.MPE
  proof closeout at commit 42ffd800b6658edbbfebfe8ee57c57e0f31e63de.
- Reconciled the prior PHY-ID blocker's wrong MDIO target set against the
  accepted observed-window MACB/GEM base.
- Selected exact corrected observed-window MDIO targets: NCR 0x1c00100000,
  NSR 0x1c00100008, and MAN 0x1c00100034.
- Defined a future no-NCR-write Clause 22 PHY-ID candidate gated on corrected
  NCR.MPE already being set.
- Defined a paired no-MDIO/no-Ethernet control using the same reporting path
  while constructing no MDIO targets or MAN frames.
- Recorded findings with disposition.

## Non-Goals

No code runtime implementation, no Pi 5 hardware run, no boot archive
publication, no hardwareTestLock acquisition, no runtime MMIO load/store, no
NCR write, no MAN write, no PHY-ID read, no PHY reset action, no GPIO32 action,
no Ethernet driver implementation, no DMA/descriptors, no interrupt ownership,
no packet I/O, no networking, no sockets, no SSH, no Phase 12.2 work, and no
phase transition.

## Reconciled Inputs

- tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract.md.
- tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-pi5-proof.md.
- tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-proof-closeout.md.
- tasks/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-source-contract.md.
- tasks/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-pi5-proof.md.
- tasks/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-proof-closeout.md.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-pi5-proof/classification.json.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-pi5-proof/classification.json.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-proof-closeout/classification.json.
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-rp1.dtsi.
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-bcm2712-rpi-5-b.dts.
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-macb.h.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-uapi-linux-mii.h.
- src/rp1_ethernet.rs.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- git history through 42ffd800b6658edbbfebfe8ee57c57e0f31e63de.

## Source Facts

- rp1.dtsi defines rp1_eth as ethernet@100000, compatible with
  raspberrypi,rp1-gem and cdns,macb.
- bcm2712-rpi-5-b.dts assigns phy-handle phy1 and declares phy1 as
  ethernet-phy@1 with reg 0x1.
- The accepted observed-window identity proof maps the selected live
  observed-window MACB/GEM controller base to 0x1c00100000, with MACB_MID at
  0x1c001000fc returning raw 0x70109.
- macb.h defines NCR offset 0x0000, NSR offset 0x0008, MAN offset 0x0034,
  NCR.MPE bit offset 4, NSR.IDLE bit offset 2, and MAN.DATA bits 15:0.
- macb.h defines Clause 22 SOF=1, READ=2, WRITE=1, and CODE=2.
- uapi linux/mii.h defines MII_PHYSID1 register 0x02 and MII_PHYSID2 register
  0x03.
- The accepted NCR.MPE proof candidate used the corrected NCR target
  0x1c00100000, observed pre_raw 0x10, wrote 0x10, read back 0x10, restored
  0x10, and performed no MAN writes or PHY-ID reads.

## Prior Wrong-Target Blocker Reconciliation

The accepted MDIO PHY-ID no-write blocker used an earlier target set of NCR
0x1c00000000, NSR 0x1c00000008, and MAN 0x1c00000034. That run remains
valuable only as a closed blocker and capture-chain/control proof: it performed
no NCR write, no MAN write, no PHY-ID read, and accepted no runtime MDIO
transaction. The raw 0x20001927 value at 0x1c00000000 is the observed RP1
SYSINFO_CHIP_ID positive-control value, not the corrected observed-window
MACB/GEM NCR target.

The corrected after-MPE contract must therefore discard the earlier MDIO target
addresses for future candidate construction while retaining the evidence as a
closed wrong-target/no-write blocker.

## Selected Contract

Accepted contract id:
phase12-rp1-ethernet-mdio-phy-id-after-mpe-source-contract-v1.

~~~text
selected discriminator: rp1-ethernet-mdio-after-mpe-clause22-phy1-physid1-physid2
controller: rp1_eth / raspberrypi,rp1-gem / cdns,macb
PHY source identity: phy1 / ethernet-phy@1 / reg 0x1
PHY address: 1
observed-window base: 0x1c00100000
MACB_MID context: 0x1c001000fc, raw 0x70109
NCR target: 0x1c00100000
NSR target: 0x1c00100008
MAN target: 0x1c00100034
MPE precondition: corrected NCR bit 4 already set
NCR write permission: forbidden for this first corrected-target PHY-ID proof
NSR.IDLE polling: bit 2 before and after each MAN write
PHY ID registers: MII_PHYSID1 0x02 and MII_PHYSID2 0x03
MAN frames: PHYSID1 0x600a0000, PHYSID2 0x600e0000
MAN.DATA extraction: bits 15:0 after post-read idle
paired control: same reporting path, no MDIO target, no MAN frame, no volatile load/store
~~~

The future candidate may construct and write MAN frames only if the corrected
NCR pre-read at 0x1c00100000 has MPE bit 4 already set. If corrected NCR.MPE
is clear, the future proof must perform no NCR write, no MAN write, and no
PHY-ID read, then classify a precise source-contract/precondition blocker.

## Operation Order For A Future Candidate

1. Print a candidate start marker and accepted input frontier.
2. Read observed-window MACB_MID context at 0x1c001000fc as context only.
3. Read corrected NCR at 0x1c00100000.
4. If corrected NCR.MPE bit 4 is clear, perform no write and classify a
   source-contract/precondition blocker.
5. Poll corrected NSR at 0x1c00100008 until IDLE bit 2 is set or a bounded
   timeout expires.
6. Write corrected MAN at 0x1c00100034 with 0x600a0000 for Clause 22 PHY
   address 1, register 0x02.
7. Poll corrected NSR.IDLE again; read corrected MAN and extract DATA[15:0] as
   physid1.
8. Repeat the same poll/write/poll/read sequence with MAN value 0x600e0000 for
   register 0x03 and extract physid2.
9. Classify only the selected corrected-target after-MPE discriminator
   outcome. Do not infer broad MDIO/PHY ownership, PHY reset ownership, link
   readiness, or Ethernet driver readiness.

## Allowed Future Proof Classifications

- mdio-phy1-physid-after-mpe-visible: both corrected-target Clause 22 PHY ID
  reads complete within the bounded timeout and return 16-bit data.
- mdio-phy1-physid-after-mpe-timeout: initial or post-MAN corrected NSR.IDLE
  polling times out.
- mdio-phy1-physid-after-mpe-precondition-blocker: corrected NCR.MPE bit 4 is
  clear or corrected NCR identity/pre-read is not decisive; no MAN write
  occurs.
- precise-staging-capture-blocker: candidate/control identity, serial, TFTP,
  or restore evidence is not precise enough to classify.
- no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-after-mpe-control: paired
  control output from the same reporting path with no MDIO target construction
  and no volatile MDIO load/store.

## Paired Control Boundary

The paired no-MDIO/no-Ethernet control must preserve the same capture/reporting
path while:

- constructing no MDIO target address and no MAN frame;
- performing no NCR, NSR, or MAN volatile load/store for Ethernet MDIO;
- withholding candidate-only PHY address, PHY ID register, MAN frame, raw
  result, and result-valid fields;
- retaining contract id, rejected-claim labels, retained-risk labels, and
  control classification;
- classifying only as
  no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-after-mpe-control.

## Same-Shaped Retry Policy

Same-shaped wrong-target MDIO PHY-ID retries remain closed. The next useful
PHY-ID proof must use corrected observed-window targets and the no-NCR-write
MPE gate above. Same-shaped NCR.MPE set/readback/restore retries also remain
closed for the accepted candidate/control pair because that proof cannot by
itself establish MAN transaction safety or PHY-ID visibility.

## Findings

- fixed: corrected the future PHY-ID target set from the earlier wrong
  0x1c00000000/0x08/0x34 addresses to observed-window MACB/GEM targets
  0x1c00100000/0x08/0x34.
- fixed: preserved the prior PHY-ID hardware proof as a closed wrong-target
  no-write blocker and control-path proof, not as an accepted runtime MDIO
  transaction.
- fixed: consumed the accepted NCR.MPE proof as the source-backed reason to
  retry PHY-ID only through the corrected after-MPE discriminator.
- fixed: required a no-NCR-write MPE gate for the first corrected-target
  PHY-ID proof; if corrected MPE is clear, future hardware must classify a
  precise blocker before any MAN write.
- fixed: named exact NCR, NSR, MAN, MACB_MID context, PHY address, PHY-ID
  register numbers, MAN frame values, NSR.IDLE polling, MAN.DATA extraction,
  paired control shape, allowed classifications, and same-shaped retry policy.
- deferred: local/static guard implementation, serialized Pi 5 proof, actual
  MAN transaction evidence, visible PHY-ID reads, PHY reset/GPIO32 ownership,
  Ethernet driver behavior, interrupts, DMA/descriptors, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition remain future
  explicit tasks.
- not-an-issue: no hardwareTestLock was acquired because this task is
  source/docs/evidence only.
- removed: no obsolete implementation, docs, or evidence was removed.

## Rejected Claims And Retained Risks

Rejected claims:

- runtime MDIO transaction evidence by this task;
- NCR write permission in the first corrected-target PHY-ID proof;
- visible PHY-ID read evidence;
- broad MDIO/PHY ownership;
- PHY reset or GPIO32 ownership;
- Ethernet driver readiness;
- interrupt delivery/completion;
- DMA/descriptors;
- packet I/O;
- networking;
- sockets;
- SSH;
- Phase 12.2;
- phase transition.

Retained risks:

- The corrected after-MPE proof may still classify a precondition blocker if
  corrected NCR.MPE is clear in the selected boot state.
- Even visible PHY-ID reads would prove only the selected management
  transaction boundary, not link, MAC, DMA, interrupts, packet I/O, sockets,
  SSH, or Phase 12.2 readiness.
- GPIO32/ETH_RST_N, PHY reset state, link state, DMA/descriptors, interrupts,
  packet I/O, sockets, SSH, and Phase 12.2 readiness remain unaccepted.

## Evidence

- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-source-contract/classification.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-source-contract/evidence-map.json.
- Accepted MDIO PHY-ID no-write blocker:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-pi5-proof.md.
- Accepted NCR.MPE proof:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-pi5-proof.md.
- Accepted NCR.MPE closeout:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-proof-closeout.md.
- Retained MACB register source:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-macb.h.
- Project docs:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap:
  docs/src/roadmap.md.

## Validation

- static inspection: accepted MDIO PHY-ID no-write blocker, accepted NCR.MPE
  proof/closeout, retained Linux MACB/RP1 source, rp1_ethernet source, Phase
  12 docs, roadmap, and git history reviewed.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Contract explicitly corrects the PHY-ID target set to accepted
  observed-window MACB/GEM base 0x1c00100000 and preserves prior wrong-target
  evidence as a closed blocker: satisfied.
- Contract names exact NCR, NSR, MAN, MACB_MID context, PHY address, PHY-ID
  register numbers, MAN frame values, MPE precondition, NSR.IDLE polling,
  MAN.DATA extraction, paired control shape, allowed classifications, and
  same-shaped retry policy: satisfied.
- Contract permits no NCR write in the first corrected-target PHY-ID proof and
  requires a precise precondition blocker if corrected NCR.MPE is clear:
  satisfied.
- Contract rejects broad MDIO/PHY ownership, PHY reset/GPIO32 ownership,
  Ethernet driver readiness, interrupts, DMA/descriptors, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition: satisfied.
- Accepted source contract is committed before the guard core starts: satisfied
  by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-phy-id-after-mpe-guard-core-20260611 on the next
worker wake if dependencies remain satisfied. Do not run hardware, acquire
hardwareTestLock, write NCR, write MAN, retry PHY-ID on hardware, touch
GPIO32/PHY reset, infer broad MDIO/PHY ownership, start Ethernet behavior, or
advance Phase 12.2 from this source contract.
