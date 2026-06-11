# Phase 12 RP1 Ethernet MDIO PHY ID After-MPE Guard Closeout

Task id: phase12-rp1-ethernet-mdio-phy-id-after-mpe-guard-closeout-20260611

Status: accepted

Classification:
rp1-ethernet-mdio-phy-id-after-mpe-guard-static-frontier-closed

Evidence level: static inspection of the accepted after-MPE source contract,
accepted guard core task record, guard classification/evidence JSON, touched
rp1_ethernet source, Phase 12 docs, roadmap, and git history. No Pi 5 hardware
run, boot archive publication, hardwareTestLock acquisition, runtime volatile
load/store, NCR write, MAN write, PHY-ID read, PHY reset/GPIO32 action,
Ethernet driver behavior, DMA/descriptors, packet I/O, networking, sockets,
SSH, Phase 12.2, or phase transition was performed by this closeout.

## Goal

Close out the corrected-target after-MPE MDIO PHY-ID guard core without
expanding beyond the local/static report boundary.

## Scope Performed

- Consumed the accepted after-MPE source contract at commit
  dd1fff6a9f427bd96c61691b5ba57e975c86a4b5 and the accepted guard core at
  commit ca76edd78945d38a7fea9eb1f34ac38fa3f500c9.
- Reconciled the guard core against the accepted NCR.MPE ownership frontier,
  source contract, rp1_ethernet implementation, Phase 12 docs, roadmap, and
  git history.
- Recorded the accepted local/static boundary: candidate construction for
  corrected observed-window NCR/NSR/MAN targets, MPE precondition, bounded
  NSR.IDLE polling, PHYSID1/PHYSID2 MAN frames, MAN.DATA extraction, rejected
  claims, retained risks, and hardware-proof boundary fields.
- Preserved the paired no-MDIO/no-Ethernet control as the same reporting path
  while constructing no MDIO targets, no MAN frames, and no runtime volatile
  load/store intent.
- Closed same-shaped local/static guard retries for this candidate/control
  pair because the accepted guard already fixes the report shape and does not
  need repetition before hardware proof.
- Selected only the serialized Pi 5 corrected-target after-MPE PHY-ID proof as
  the next bounded hardware task, preserving the no-NCR-write MPE gate.

## Findings

- fixed: reconciled the accepted guard core as the current corrected-target
  after-MPE MDIO PHY-ID report frontier.
- fixed: recorded that future candidate proof may use only corrected
  observed-window NCR 0x1c00100000, NSR 0x1c00100008, MAN 0x1c00100034, and
  MACB_MID context 0x1c001000fc.
- fixed: preserved the accepted no-NCR-write MPE precondition: corrected
  NCR.MPE bit 4 must already be set before any MAN write; otherwise no NCR or
  MAN write may occur.
- fixed: preserved the paired control shape with no MDIO target construction,
  no MAN frames, no candidate-only result fields, and no volatile access
  intent.
- fixed: closed same-shaped local/static guard retries for this
  candidate/control pair.
- deferred: serialized Pi 5 MAN/PHY-ID proof, visible PHY-ID reads, PHY
  reset/GPIO32 ownership, broad MDIO/PHY ownership, Ethernet driver behavior,
  interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition remain future explicit tasks.
- not-an-issue: no hardwareTestLock was acquired because this closeout is a
  static checkpoint over already accepted local/static evidence.
- removed: no obsolete implementation, docs, or task evidence was removed.

## Accepted Boundary

The accepted frontier is exactly the corrected-target after-MPE local/static
guard report surface:

- candidate report contract
  phase12-rp1-ethernet-mdio-phy-id-after-mpe-guard-report-contract-v1;
- source contract
  phase12-rp1-ethernet-mdio-phy-id-after-mpe-source-contract-v1;
- accepted MPE frontier
  rp1-ethernet-mdio-mpe-enable-ownership-frontier-closed;
- selected discriminator
  rp1-ethernet-mdio-after-mpe-clause22-phy1-physid1-physid2;
- corrected targets: NCR 0x1c00100000, NSR 0x1c00100008, MAN 0x1c00100034,
  and MACB_MID context 0x1c001000fc;
- PHY address 1 with MII_PHYSID1 register 0x02 / MAN frame 0x600a0000 and
  MII_PHYSID2 register 0x03 / MAN frame 0x600e0000;
- no-NCR-write MPE gate: corrected NCR.MPE bit 4 must already be set before
  any MAN write;
- bounded NSR.IDLE bit 2 polling before and after each MAN write;
- MAN.DATA bits 15:0 extraction only after post-MAN idle;
- paired control classification
  no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-after-mpe-control with no MDIO
  targets, no MAN frames, no result fields, and no volatile access intent.

This accepts only local/static report construction and validation. It does not
accept hardware evidence, runtime RP1 MMIO access, NCR writes, MAN writes,
visible PHY-ID reads, broad MDIO/PHY ownership, PHY reset/GPIO32 ownership,
Ethernet driver behavior, interrupts, DMA/descriptors, packet I/O, networking,
sockets, SSH, Phase 12.2, or a phase transition.

## Same-Shaped Retry Policy

Same-shaped local/static guard retries are closed for this candidate/control
pair. Repeating the accepted report construction would not prove MAN
transaction safety, PHY-ID visibility, PHY reset ownership, broad MDIO/PHY
ownership, Ethernet driver behavior, or packet I/O.

The only selected follow-up is the queued serialized Pi 5 corrected-target
after-MPE PHY-ID proof. That proof must acquire hardwareTestLock and preserve
candidate/control selected-tree identity, archive/static review output, fresh
serial cursor/output, run-unique serial markers, TFTP delta, final
pre-restore identity, lab restore evidence, capture summary, classification
JSON, and evidence map. It may perform no NCR write; MAN writes are allowed
only if corrected NCR.MPE bit 4 is already set.

## Rejected Claims And Retained Risks

Rejected claims:

- hardware proof by this closeout;
- runtime RP1 MMIO load/store evidence;
- NCR write permission or execution;
- MAN write without corrected NCR.MPE precondition;
- visible PHY-ID read evidence;
- runtime MDIO transaction success;
- broad MDIO/PHY ownership;
- PHY reset/GPIO32 ownership;
- Ethernet driver readiness;
- interrupt completion;
- DMA/descriptors;
- packet I/O;
- networking;
- sockets;
- SSH;
- Phase 12.2;
- phase transition.

Retained risks:

- Corrected NCR.MPE may be clear in the selected boot state and must block all
  MAN writes.
- Visible PHY-ID reads remain unproven until a serialized hardware proof.
- GPIO32 / ETH_RST_N ownership remains unproven.
- Link, MAC, DMA, interrupt, packet, socket, SSH, and Phase 12.2 readiness
  remain unaccepted.

## Evidence

- Accepted source contract:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-source-contract.md.
- Accepted guard core:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-guard-core.md.
- Guard core classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-guard-core/classification.json.
- Guard core evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-guard-core/evidence-map.json.
- Implementation:
  src/rp1_ethernet.rs.
- Closeout classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-guard-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-guard-closeout/evidence-map.json.
- Project docs:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap:
  docs/src/roadmap.md.

## Validation

- static inspection: accepted source contract, accepted guard core record,
  guard classification/evidence JSON, touched rp1_ethernet source, Phase 12
  docs, roadmap, and git history reviewed.
- JSON checks: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles the source contract and guard core without expanding
  beyond corrected-target after-MPE MAN/PHY-ID discriminator boundaries:
  satisfied.
- Checkpoint explicitly selects only
  phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof-20260611 as the next
  hardware task and preserves no-NCR-write gating: satisfied.
- If blocked, checkpoint records a precise blocker and leaves
  planningNeeded=true with a concrete reason: not applicable; accepted.
- Accepted checkpoint is committed before any hardware proof starts:
  satisfied by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-phy-id-after-mpe-pi5-proof-20260611 on the next
worker wake if hardwareTestLock remains unlocked and supervisorIntervention is
inactive. Do not run hardware in this closeout, write NCR, bypass the MPE
precondition, touch GPIO32/PHY reset, infer broad MDIO/PHY ownership, start
Ethernet behavior, Phase 12.2, or a phase transition.
