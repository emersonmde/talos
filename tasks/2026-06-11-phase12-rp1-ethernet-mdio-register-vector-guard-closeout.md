# Phase 12 RP1 Ethernet MDIO Register Vector Guard Closeout

Task id: phase12-rp1-ethernet-mdio-register-vector-guard-closeout-20260611

Status: accepted

Classification:
rp1-ethernet-mdio-register-vector-guard-static-frontier-closed

Evidence level: static inspection of the accepted register-vector source
contract, accepted guard core task record, guard classification/evidence JSON,
touched rp1_ethernet source, Phase 12 docs, roadmap, and git history. No Pi 5
hardware run, boot archive publication, hardwareTestLock acquisition, runtime
volatile load/store, NCR write, MAN write, register-vector read, PHY
reset/GPIO32 action, Ethernet driver behavior, DMA/descriptors, packet I/O,
networking, sockets, SSH, Phase 12.2, or phase transition was performed by
this closeout.

## Goal

Close out the corrected-target MDIO register-vector guard core without
expanding beyond the local/static report boundary.

## Scope Performed

- Consumed the accepted register-vector source contract at commit
  880901aca85ef98d90c6f0278ed6cdc94bbfd9d6 and the accepted guard core at
  commit 3087034c952d77943e0cb32030f9752fa9d599dd.
- Reconciled the guard core against the accepted corrected-target after-MPE
  PHY-ID frontier, source contract, rp1_ethernet implementation, Phase 12
  docs, roadmap, and git history.
- Recorded the accepted local/static boundary: candidate construction for the
  six-register Clause 22 vector, corrected observed-window NCR/NSR/MAN
  targets, MACB_MID context, PHY address 1, exact PHYA=1 MAN frames, MPE
  precondition, bounded NSR.IDLE polling, MAN.DATA extraction, rejected
  claims, retained risks, and hardware-proof boundary fields.
- Preserved the paired no-MDIO/no-Ethernet control as the same reporting path
  while constructing no MDIO targets, no MAN frames, no candidate-only
  register-vector/result fields, and no runtime volatile load/store intent.
- Closed same-shaped local/static guard retries for this candidate/control
  pair because the accepted guard already fixes the report shape and does not
  need repetition before hardware proof.
- Selected only the serialized Pi 5 corrected-target register-vector proof as
  the next bounded hardware task, preserving the no-NCR-write MPE gate and
  no-reset boundary.

## Findings

- fixed: reconciled the accepted guard core as the current corrected-target
  MDIO register-vector report frontier.
- fixed: recorded that a future candidate proof may use only corrected
  observed-window NCR 0x1c00100000, NSR 0x1c00100008, MAN 0x1c00100034, and
  MACB_MID context 0x1c001000fc.
- fixed: preserved exact six-register Clause 22 vector frames for PHY address
  1: BMCR 0x60820000, BMSR 0x60860000, PHYSID1 0x608a0000, PHYSID2
  0x608e0000, ANAR 0x60920000, and ANLPAR 0x60960000.
- fixed: preserved the accepted no-NCR-write MPE precondition: corrected
  NCR.MPE bit 4 must already be set before any MAN write; otherwise no NCR or
  MAN write may occur.
- fixed: preserved the paired control shape with no MDIO target construction,
  no MAN frames, no candidate-only result fields, and no volatile access
  intent.
- fixed: closed same-shaped local/static guard retries for this
  candidate/control pair.
- deferred: serialized Pi 5 register-vector proof, actual MAN.DATA vector
  evidence, PHY reset/GPIO32 ownership, broad MDIO/PHY ownership, Ethernet
  driver behavior, interrupts, DMA/descriptors, packet I/O, networking,
  sockets, SSH, Phase 12.2, and phase transition remain future explicit
  tasks.
- not-an-issue: no hardwareTestLock was acquired because this closeout is a
  static checkpoint over already accepted local/static evidence.
- removed: no obsolete implementation, docs, or task evidence was removed.

## Accepted Boundary

The accepted frontier is exactly the corrected-target local/static
register-vector guard report surface:

- source contract
  phase12-rp1-ethernet-mdio-register-vector-source-contract-v1;
- report contract
  phase12-rp1-ethernet-mdio-register-vector-guard-report-contract-v1;
- selected discriminator
  rp1-ethernet-mdio-after-mpe-clause22-phy1-register-vector;
- candidate classification
  rp1-ethernet-mdio-register-vector-guard-candidate-local-static;
- paired control classification
  no-mdio-no-ethernet-rp1-ethernet-mdio-register-vector-control;
- corrected targets: NCR 0x1c00100000, NSR 0x1c00100008, MAN 0x1c00100034,
  and MACB_MID context 0x1c001000fc;
- PHY address 1;
- selected read-only vector:
  BMCR 0x00 / 0x60820000, BMSR 0x01 / 0x60860000, PHYSID1 0x02 /
  0x608a0000, PHYSID2 0x03 / 0x608e0000, ANAR 0x04 / 0x60920000, and ANLPAR
  0x05 / 0x60960000;
- no-NCR-write MPE gate: corrected NCR.MPE bit 4 must already be set before
  any MAN write;
- bounded NSR.IDLE bit 2 polling before and after each MAN write;
- MAN.DATA bits 15:0 extraction only after post-MAN idle;
- paired control with no MDIO targets, no MAN frames, no register-vector
  result fields, and no volatile access intent.

This accepts only local/static report construction and validation. It does not
accept hardware evidence, runtime RP1 MMIO access, NCR writes, MAN writes,
visible register-vector reads, PHY absence from all-ones values, broad
MDIO/PHY ownership, PHY reset/GPIO32 ownership, Ethernet driver behavior,
interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH,
Phase 12.2, or a phase transition.

## Same-Shaped Retry Policy

Same-shaped local/static guard retries are closed for this candidate/control
pair. Repeating the accepted report construction would not prove MAN
transaction safety, register-vector visibility, PHY reset ownership, broad
MDIO/PHY ownership, Ethernet driver behavior, or packet I/O.

The only selected follow-up is the queued serialized Pi 5 corrected-target
register-vector proof. That proof must acquire hardwareTestLock and preserve
candidate/control selected-tree identity, fresh serial cursor/output,
run-unique serial markers, TFTP delta, final pre-restore identity, lab restore
evidence, capture summary, classification JSON, and evidence map. It may
perform no NCR write, no GPIO32/PHY reset action, and MAN writes are allowed
only if corrected NCR.MPE bit 4 is already set. If evidence is inconclusive,
the proof must perform the standard Pi 5 inconclusive-run triage before any
code changes: candidate identity, fresh serial cursor, TFTP delta, known-good
control or named snapshot restore when appropriate, then candidate rerun.

## Rejected Claims And Retained Risks

Rejected claims:

- hardware proof by this closeout;
- runtime RP1 MMIO load/store evidence;
- NCR write permission or execution;
- MAN write without corrected NCR.MPE precondition;
- visible register-vector read evidence;
- runtime MDIO transaction success;
- PHY absence from all-ones values;
- broad MDIO or PHY ownership;
- PHY reset or GPIO32 ownership;
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
- The selected register-vector MAN.DATA values remain unproven until a
  serialized hardware proof.
- An all-ones vector would be visible register-vector evidence only, not proof
  of PHY absence, reset state, link state, or usable Ethernet.
- GPIO32 / ETH_RST_N ownership remains unproven.
- Link, MAC, DMA, interrupt, packet, socket, SSH, and Phase 12.2 readiness
  remain unaccepted.

## Evidence

- Accepted source contract:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-source-contract.md.
- Accepted guard core:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-guard-core.md.
- Guard core classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-guard-core/classification.json.
- Guard core evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-guard-core/evidence-map.json.
- Implementation:
  src/rp1_ethernet.rs.
- Closeout classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-guard-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-guard-closeout/evidence-map.json.
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
- Checkpoint reconciles local/static guard evidence without expanding beyond
  candidate/control report construction and validators: satisfied.
- Same-shaped local/static retry policy is explicit: satisfied.
- Pi 5 proof requirements include serialized hardware lock,
  candidate/control identity, fresh serial cursor/output, TFTP delta, final
  pre-restore identity, restore proof, capture summary, classification JSON,
  evidence map, and inconclusive-run triage before code changes: satisfied.
- Accepted closeout is committed before hardware proof starts: satisfied by
  the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-register-vector-pi5-proof-20260611 on the next
worker wake if hardwareTestLock remains unlocked and supervisorIntervention is
inactive. Keep the hardware run to the selected register-vector
candidate/control proof: no NCR write, no GPIO32/PHY reset action, no Ethernet
driver behavior, no packet I/O, no networking, no SSH, no Phase 12.2, and no
phase transition.
