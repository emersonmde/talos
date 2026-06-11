# Phase 12 RP1 Ethernet MDIO Register Vector Source Contract

Task id: phase12-rp1-ethernet-mdio-register-vector-source-contract-20260611

Status: accepted

Classification: rp1-ethernet-mdio-register-vector-source-contract-accepted

Evidence level: static inspection of the accepted corrected-target after-MPE
PHY-ID proof closeout, proof capture summary/classification/evidence-map JSON,
Phase 12 docs, roadmap, retained MACB/GEM Clause 22 sources, and task-owned
JSON. No Pi 5 hardware run, boot archive publication, hardwareTestLock
acquisition, NCR write, runtime MAN write, GPIO32/PHY reset action, Ethernet
driver behavior, DMA/descriptors, interrupts, packet I/O, networking, sockets,
SSH, Phase 12.2, or phase transition was performed.

## Goal

Define the smallest source-backed corrected-target MDIO register-vector
discriminator after the accepted after-MPE PHY-ID proof returned 0xffff/0xffff
for its selected MAN.DATA reads.

## Scope Performed

- Consumed the accepted after-MPE PHY-ID proof closeout at commit
  9a1bd0c526507eb819b6e1f43b902fce1af9e9b5.
- Preserved the accepted corrected observed-window MACB/GEM target set:
  MACB_MID context 0x1c001000fc, NCR 0x1c00100000, NSR 0x1c00100008, and
  MAN 0x1c00100034.
- Defined a no-NCR-write, no-reset Clause 22 register-vector discriminator for
  rp1_eth phy1 / ethernet-phy@1 / PHY address 1.
- Selected the minimum six-register vector required by the supervisor task:
  BMCR 0x00, BMSR 0x01, PHYSID1 0x02, PHYSID2 0x03, ANAR 0x04, and ANLPAR
  0x05.
- Reconciled source-backed Clause 22 MAN frame construction, including PHYA
  bits for PHY address 1, instead of broadening the accepted 0xffff/0xffff
  proof beyond the selected frames it recorded.
- Preserved the accepted MPE precondition: if corrected NCR.MPE bit 4 is
  clear, a future proof performs no NCR write and no MAN write and classifies
  a precise precondition blocker.
- Defined paired no-MDIO/no-Ethernet control evidence using the same reporting
  path while constructing no MDIO target and no MAN frame.
- Recorded findings with disposition.

## Non-Goals

No code implementation beyond source/docs/evidence contract work, no Pi 5
hardware run, no boot archive publication, no hardwareTestLock acquisition, no
NCR write, no runtime MAN write, no GPIO32/PHY reset assertion/deassertion, no
Ethernet driver implementation, no DMA/descriptors, no interrupts, no packet
I/O, no networking, no sockets, no SSH, no Phase 12.2 work, and no phase
transition.

This task does not claim that an all-ones register vector proves PHY absence,
PHY reset ownership, link state, or usable Ethernet.

## Reconciled Inputs

- tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof.md.
- tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-proof-closeout.md.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof/classification.json.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof/evidence-map.json.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof/capture-summary.json.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-proof-closeout/classification.json.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-macb.h.
- tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-uapi-linux-mii.h.
- tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c.
- docs/src/project/phase12-networking-ssh.md.
- docs/src/roadmap.md.
- git history through 9a1bd0c526507eb819b6e1f43b902fce1af9e9b5.

## Source Facts

- rp1.dtsi defines rp1_eth as ethernet@100000, compatible with
  raspberrypi,rp1-gem and cdns,macb.
- bcm2712-rpi-5-b.dts assigns phy-handle phy1 and declares phy1 as
  ethernet-phy@1 with reg 0x1.
- The accepted observed-window MACB/GEM controller base is 0x1c00100000, with
  MACB_MID context at 0x1c001000fc returning 0x70109 in the accepted proof.
- macb.h defines NCR offset 0x0000, NSR offset 0x0008, MAN offset 0x0034,
  NCR.MPE bit offset 4, NSR.IDLE bit offset 2, MAN.DATA bits 15:0, MAN.REGA
  bits 22:18, MAN.PHYA bits 27:23, MAN.RW bits 29:28, and MAN.SOF bits 31:30.
- macb.h defines Clause 22 SOF=1, READ=2, and CODE=2. Linux macb_mdio_read_c22
  constructs read frames with SOF, RW, PHYA, REGA, and CODE.
- uapi linux/mii.h defines MII_BMCR 0x00, MII_BMSR 0x01, MII_PHYSID1 0x02,
  MII_PHYSID2 0x03, MII_ADVERTISE 0x04, and MII_LPA 0x05.

## Selected Contract

Accepted contract id:
phase12-rp1-ethernet-mdio-register-vector-source-contract-v1.

~~~text
selected discriminator: rp1-ethernet-mdio-after-mpe-clause22-phy1-register-vector
purpose: distinguish global all-ones/no-response behavior from PHY-ID-only evidence
controller: rp1_eth / raspberrypi,rp1-gem / cdns,macb
PHY source identity: phy1 / ethernet-phy@1 / reg 0x1
PHY address: 1
observed-window base: 0x1c00100000
MACB_MID context: 0x1c001000fc, expected raw 0x70109
NCR target: 0x1c00100000
NSR target: 0x1c00100008
MAN target: 0x1c00100034
MPE precondition: corrected NCR bit 4 already set
NCR write permission: forbidden
GPIO32/PHY reset permission: forbidden
NSR.IDLE polling: bit 2 before and after each MAN write
MAN.DATA extraction: bits 15:0 after post-read idle
paired control: same reporting path, no MDIO target, no MAN frame, no volatile load/store
~~~

## Register Vector

The selected read-only vector is:

| register | source name | purpose | Clause 22 read MAN frame |
| --- | --- | --- | --- |
| 0x00 | MII_BMCR | control baseline | 0x60820000 |
| 0x01 | MII_BMSR | status/link capability baseline | 0x60860000 |
| 0x02 | MII_PHYSID1 | PHY-ID high word | 0x608a0000 |
| 0x03 | MII_PHYSID2 | PHY-ID low word | 0x608e0000 |
| 0x04 | MII_ADVERTISE / ANAR | local advertisement | 0x60920000 |
| 0x05 | MII_LPA / ANLPAR | link-partner advertisement | 0x60960000 |

Frame construction is:

~~~text
MAN = (SOF 1 << 30)
    | (READ 2 << 28)
    | (PHYA 1 << 23)
    | (REGA register << 18)
    | (CODE 2 << 16)
~~~

This source-backed frame list includes the PHYA field for phy1 address 1. The
accepted after-MPE proof remains accepted only for its recorded selected
PHYSID1/PHYSID2 MAN.DATA return values. This contract does not reinterpret
that proof as broad PHY1 responsiveness or as a complete register-vector read.

## Operation Order For A Future Candidate

1. Print a candidate start marker and accepted input frontier.
2. Read observed-window MACB_MID context at 0x1c001000fc as context only.
3. Read corrected NCR at 0x1c00100000.
4. If corrected NCR.MPE bit 4 is clear, perform no NCR write, no MAN write,
   no GPIO32/PHY reset action, and classify a precondition blocker.
5. For each selected register, poll corrected NSR at 0x1c00100008 until
   IDLE bit 2 is set or a bounded timeout expires.
6. Write corrected MAN at 0x1c00100034 with the exact register-specific
   Clause 22 read frame from the table.
7. Poll corrected NSR.IDLE again; read corrected MAN and extract DATA[15:0].
8. Record the ordered six-entry vector with raw MAN readback, DATA value,
   valid flag, and timeout/precondition status for each entry.
9. Classify only the selected register-vector discriminator outcome. Do not
   infer PHY reset ownership, link state, broad MDIO/PHY ownership, Ethernet
   driver behavior, packet I/O, networking, sockets, SSH, Phase 12.2, or a
   phase transition.

## Allowed Future Proof Classifications

- mdio-phy1-register-vector-visible: all selected reads complete within the
  bounded timeout and at least one DATA value is not 0xffff.
- mdio-phy1-register-vector-global-all-ones-visible: all selected reads
  complete within the bounded timeout and all six DATA values are 0xffff.
- mdio-phy1-register-vector-physid-only-all-ones-mixed-visible: PHYSID1 and
  PHYSID2 return 0xffff while at least one non-PHY-ID selected register returns
  a value other than 0xffff.
- mdio-phy1-register-vector-timeout: initial or post-MAN corrected NSR.IDLE
  polling times out for one or more selected registers.
- mdio-phy1-register-vector-precondition-blocker: corrected NCR.MPE bit 4 is
  clear or corrected NCR identity/pre-read is not decisive; no MAN write
  occurs.
- precise-staging-capture-blocker: candidate/control identity, serial, TFTP,
  final identity, restore evidence, or task-owned JSON is not precise enough
  to classify.
- no-mdio-no-ethernet-rp1-ethernet-mdio-register-vector-control: paired
  control output from the same reporting path with no MDIO target
  construction, no MAN frame construction, and no volatile MDIO load/store.

An all-ones vector is a visible MAN.DATA vector for the selected register set,
not proof of PHY absence, reset state, link state, or usable Ethernet.

## Paired Control Boundary

The paired no-MDIO/no-Ethernet control must preserve the same capture/reporting
path while:

- constructing no MDIO target address and no MAN frame;
- performing no NCR, NSR, MAN, GPIO32, PHY reset, Ethernet MMIO, DMA,
  interrupt, packet, or network volatile load/store;
- withholding candidate-only PHY address, register-vector, MAN frame, raw MAN,
  DATA, result-valid, and timeout fields;
- retaining contract id, rejected-claim labels, retained-risk labels, and
  control classification;
- classifying only as
  no-mdio-no-ethernet-rp1-ethernet-mdio-register-vector-control.

## Qualitative Difference From Same-Shaped PHY-ID Retry

Same-shaped after-MPE PHY-ID hardware retries remain closed for the accepted
candidate/control pair. The register-vector discriminator is different because
it requires a six-register source-backed Clause 22 vector and classifies
whether all selected reads return 0xffff, whether only PHY-ID remains all
ones while other registers differ, or whether the result is a timeout or
precondition blocker.

This distinction can help decide whether the accepted 0xffff/0xffff evidence
looks like global all-ones/no-response behavior or PHY-ID-only evidence, but
it still does not prove PHY reset ownership, link state, broad MDIO/PHY
ownership, Ethernet driver readiness, or packet I/O.

## Findings

- fixed: selected a source-backed six-register Clause 22 vector after the
  accepted after-MPE PHY-ID proof returned 0xffff/0xffff for its selected
  MAN.DATA reads.
- fixed: preserved corrected observed-window MACB/GEM NCR, NSR, MAN, and
  MACB_MID targets from the accepted after-MPE proof closeout.
- fixed: required no NCR write, no GPIO32/PHY reset action, and no MAN write
  if corrected NCR.MPE bit 4 is not already set.
- fixed: reconciled exact Clause 22 MAN frame construction for phy1 address 1
  using source-backed PHYA bits, and kept the prior accepted proof bounded to
  its recorded selected frames rather than broad PHY1 ownership.
- fixed: named all-ones, mixed-vector, timeout, precondition-blocker,
  capture-blocker, and paired-control classifications.
- fixed: defined paired no-MDIO/no-Ethernet control evidence through the same
  reporting path with no MDIO target, no MAN frame, and no volatile access.
- deferred: local/static guard implementation, serialized Pi 5 proof, actual
  register-vector evidence, PHY reset/GPIO32 ownership, broad MDIO/PHY
  ownership, Ethernet driver behavior, interrupts, DMA/descriptors, packet
  I/O, networking, sockets, SSH, Phase 12.2, and phase transition remain
  future explicit tasks.
- not-an-issue: no hardwareTestLock was acquired because this task is
  source/docs/evidence only.
- removed: no obsolete implementation, docs, or evidence was removed.

## Rejected Claims And Retained Risks

Rejected claims:

- runtime register-vector evidence by this task;
- NCR write permission;
- GPIO32/PHY reset action;
- PHY absence from an all-ones vector;
- PHY reset ownership;
- link state;
- broad MDIO/PHY ownership;
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

- The future proof may classify a precondition blocker if corrected NCR.MPE is
  clear in the selected boot state.
- A global all-ones vector may indicate no response, reset-held PHY state,
  missing prerequisite sequencing, or another MDIO-path issue; this contract
  intentionally does not choose among those causes.
- A mixed vector would still prove only selected MAN.DATA read visibility, not
  link, MAC configuration, DMA, interrupts, packet I/O, sockets, SSH, or
  Phase 12.2 readiness.
- GPIO32/ETH_RST_N, PHY reset state, link state, DMA/descriptors, interrupts,
  packet I/O, sockets, SSH, and Phase 12.2 readiness remain unaccepted.

## Evidence

- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-source-contract/classification.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-source-contract/evidence-map.json.
- Accepted after-MPE proof:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof.md.
- Accepted after-MPE closeout:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-proof-closeout.md.
- Proof capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof/capture-summary.json.
- Retained MACB register source:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-macb.h.
- Retained MII register source:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract/source/linux-rpi-6.12-uapi-linux-mii.h.
- Retained Linux MACB MDIO source:
  tasks/evidence/2026-06-09-phase12-rp1-ethernet-source-inventory/source/linux-rpi-6.12-macb_main.c.
- Project docs:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap:
  docs/src/roadmap.md.

## Validation

- static inspection: accepted after-MPE proof closeout, proof
  classification/evidence-map/capture-summary JSON, Phase 12 docs, roadmap,
  retained MACB/GEM Clause 22 source references, and git history reviewed.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Contract names corrected NCR/NSR/MAN/MACB_MID targets, PHY address, selected
  register set, exact MAN frame construction, polling bounds, data extraction,
  precondition-blocker behavior, paired control, rejected claims, retained
  risks, and nextAction: satisfied.
- Contract is qualitatively different from same-shaped PHY-ID retry by
  selecting a multi-register read-only vector whose purpose is distinguishing
  global all-ones/no-response behavior from PHY-ID-only evidence: satisfied.
- No runtime hardware action or broad MDIO/PHY ownership is accepted:
  satisfied.
- Accepted source contract is committed before the guard core starts:
  satisfied by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-register-vector-guard-core-20260611 on the next
worker wake if dependencies remain satisfied. Keep that task local/static
only; do not run hardware, acquire hardwareTestLock, write NCR, write MAN on
hardware, touch GPIO32/PHY reset, infer PHY absence from an all-ones vector,
infer broad MDIO/PHY ownership, start Ethernet behavior, networking, sockets,
SSH, Phase 12.2, or a phase transition from this source contract.
