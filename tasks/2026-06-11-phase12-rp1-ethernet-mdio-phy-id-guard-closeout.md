# Phase 12 RP1 Ethernet MDIO PHY-ID Guard Closeout

Task id: phase12-rp1-ethernet-mdio-phy-id-guard-closeout-20260611

Status: accepted

Classification: rp1-ethernet-mdio-phy-id-guard-static-frontier-closed

Evidence level: static inspection of accepted source contract, guard core task
record, guard core classification/evidence JSON, focused tests, touched source,
project docs, and git history. No Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, runtime MDIO transaction, NCR.MPE write,
GPIO32/PHY reset write, Ethernet driver behavior, interrupt handling,
DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or phase
transition was performed.

## Goal

Close out the local/static MDIO PHY-ID guard frontier and decide whether the
serialized Pi 5 proof is mechanically objective.

## Scope Performed

- Reconciled the accepted source contract
  `phase12-rp1-ethernet-mdio-phy-id-source-contract-v1` with the accepted
  guard report contract
  `phase12-rp1-ethernet-mdio-phy-id-guard-report-contract-v1`.
- Confirmed the candidate guard preserves the exact Clause 22 phy1 PHY-ID
  discriminator, PHY address/registers, observed MACB/NCR/NSR/MAN targets,
  MAN frames, no-write MPE precondition, operation order, timeout
  classifications, rejected claims, retained risks, and source evidence.
- Confirmed the paired no-MDIO/no-Ethernet control uses the same report path
  while withholding candidate-only MDIO/PHY/Ethernet target facts.
- Closed same-shaped local/static guard retries for this candidate/control
  pair.
- Selected only the queued serialized Pi 5 MDIO PHY-ID proof as the next
  mechanically objective task.

## Findings

- fixed: accepted guard evidence carries exact target and identity facts for
  the future proof: phy1 / ethernet-phy@1 address 1, MII_PHYSID1/2 registers
  0x02 and 0x03, MACB_MID context target 0x1c001000fc, NCR target
  0x1c00000000, NSR target 0x1c00000008, MAN target 0x1c00000034, and
  translated comparator target 0x1f001000fc.
- fixed: accepted guard evidence preserves Clause 22 MAN frame construction:
  PHYSID1 frame 0x600a0000 and PHYSID2 frame 0x600e0000, NSR.IDLE bit 2
  polling, MAN.DATA[15:0] extraction, and bounded timeout classifications.
- fixed: the first hardware proof remains constrained by the source-backed
  no-write precondition: NCR.MPE bit 4 must already be set; no NCR.MPE write
  is accepted, and MAN has no restore write because MDIO reads are
  transactions.
- fixed: paired control withholds MDIO target addresses, PHY identity,
  register identity, MAN frames, operation order, and source evidence while
  retaining the same report path and explicit no-MDIO/no-Ethernet
  classification.
- fixed: validators and focused tests reject missing contract identity,
  ambiguous target input, control target leakage, wrong PHY/register/MAN frame
  facts, runtime MDIO claims, MDIO/PHY ownership, NCR.MPE write permission,
  GPIO32/PHY reset ownership, Ethernet readiness, broad MMIO readiness,
  interrupt completion, DMA/descriptors, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase-transition claims.
- deferred: serialized Pi 5 MDIO PHY-ID proof, actual MDIO transaction
  evidence, any future NCR.MPE write authority, PHY reset ownership, Ethernet
  runtime behavior, packet I/O, networking, sockets, SSH, Phase 12.2, and
  phase transition remain future explicitly queued or supervisor-owned work.
- not-an-issue: hardwareTestLock was not acquired because this closeout is
  static-only and performs no hardware action.
- removed: same-shaped local/static MDIO PHY-ID guard retries for this
  candidate/control pair are closed; no source or evidence files were removed.

## Accepted Checkpoint

Accepted source contract:
phase12-rp1-ethernet-mdio-phy-id-source-contract-v1.

Accepted guard report contract:
phase12-rp1-ethernet-mdio-phy-id-guard-report-contract-v1.

Accepted candidate classification:
rp1-ethernet-mdio-phy-id-guard-candidate-local-static.

Accepted control classification:
no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-control.

Accepted closeout classification:
rp1-ethernet-mdio-phy-id-guard-static-frontier-closed.

~~~text
selected discriminator: rp1-ethernet-mdio-clause22-phy1-physid1-physid2
controller: rp1_eth / raspberrypi,rp1-gem / cdns,macb
phy: phy1 / ethernet-phy@1 / address 1
phy ID registers: MII_PHYSID1 0x02, MII_PHYSID2 0x03
MACB_MID context target: 0x1c001000fc
translated comparator target: 0x1f001000fc
NCR target: 0x1c00000000, MPE bit 4
NSR target: 0x1c00000008, IDLE bit 2
MAN target: 0x1c00000034, DATA bits 15:0
PHYSID1 MAN frame: 0x600a0000
PHYSID2 MAN frame: 0x600e0000
first-proof precondition: NCR.MPE must already be set; no NCR.MPE write
restore policy: no MAN restore write; lab boot restore evidence required
~~~

This checkpoint authorizes only the already queued serialized proof task to
attempt the guarded MDIO PHY-ID discriminator under hardwareTestLock. It does
not accept broad MDIO ownership, PHY reset ownership, NCR.MPE write authority,
Ethernet driver behavior, interrupt completion, DMA/descriptors, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

## Selected Proof Gates

The serialized Pi 5 proof is mechanically authorized because the accepted
contract and guard core define the exact target identity, operation order,
no-write precondition, paired control, capture requirements, no-MAN-restore
policy, and allowed classifications needed for a bounded candidate/control
hardware run.

The proof must:

- acquire hardwareTestLock before archive publication, staging, power action,
  or runtime MDIO interaction;
- retain candidate/control identity, archive digest, fresh serial cursor and
  transcript, TFTP delta, final pre-restore identity, lab boot restore
  evidence, classification JSON, and evidence map;
- classify only as mdio-phy1-physid-visible, mdio-phy1-physid-timeout,
  mdio-phy1-physid-source-contract-violated-blocker,
  precise-staging-capture-blocker, or
  no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-control;
- perform no NCR.MPE write and no GPIO32/PHY reset write;
- infer no Ethernet runtime readiness, interrupts, DMA/descriptors, packet
  I/O, networking, sockets, SSH, Phase 12.2, or phase transition.

## Evidence

- Accepted source contract:
  `tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract.md`.
- Accepted guard core:
  `tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-guard-core.md`.
- Guard core classification:
  `tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-guard-core/classification.json`.
- Guard core evidence map:
  `tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-guard-core/evidence-map.json`.
- Implementation:
  `src/rp1_ethernet.rs`.
- Project docs:
  `docs/src/project/phase12-networking-ssh.md`.
- Closeout classification:
  `tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-guard-closeout/classification.json`.
- Closeout evidence map:
  `tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-guard-closeout/evidence-map.json`.

## Validation

- static inspection: accepted source contract, guard core task record, guard
  core classification/evidence JSON, focused tests, touched source, project
  docs, and git history reviewed.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- documentation build: not required; no docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles local/static contract/core evidence without accepting
  runtime MDIO ownership or Ethernet behavior: satisfied.
- Same-shaped local/static guard retries are closed for this candidate/control
  pair: satisfied.
- NextAction selects the serialized Pi 5 proof with exact capture and safety
  gates: satisfied.
- Accepted closeout is committed before any hardware proof starts: satisfied
  by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-phy-id-pi5-proof-20260611 on the next worker wake if
dependencies remain satisfied, hardwareTestLock remains unlocked, and
supervisorIntervention.active remains false. That task must serialize under
hardwareTestLock and preserve candidate/control identity, boot archive digest,
fresh serial cursor/output, TFTP delta, final pre-restore identity, lab boot
restore evidence, task-owned classification JSON, and evidence map before
accepting any MDIO PHY-ID proof or precise blocker.
