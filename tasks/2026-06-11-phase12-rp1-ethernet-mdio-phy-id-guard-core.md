# Phase 12 RP1 Ethernet MDIO PHY-ID Guard Core

Task id: phase12-rp1-ethernet-mdio-phy-id-guard-core-20260611

Status: accepted

Classification: rp1-ethernet-mdio-phy-id-guard-core-local-static-accepted

Evidence level: local/static implementation, focused unit tests, JSON
validation, diff hygiene, and git history. No Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, runtime MDIO transaction, NCR.MPE
write, GPIO32/PHY reset write, Ethernet driver behavior, packet I/O,
networking, sockets, SSH, Phase 12.2, or phase transition was performed.

## Goal

Implement the local/static candidate-control report and guard surface selected
by the accepted MDIO/PHY-ID source contract.

## Scope Performed

- Added deterministic MDIO PHY-ID guard contract/report types in
  src/rp1_ethernet.rs.
- Candidate evidence preserves the accepted source contract id, selected
  Clause 22 phy1 PHY-ID discriminator, rp1_eth identity, phy1 / ethernet-phy@1
  address 1, MII_PHYSID1/2 registers 0x02 and 0x03, observed-window MACB_MID
  identity context, translated comparator target, NCR/NSR/MAN register offsets
  and observed-window targets, NCR.MPE and NSR.IDLE bits, MAN.DATA extraction,
  Clause 22 constants, exact MAN frame values 0x600a0000 and 0x600e0000,
  preconditions, operation order, allowed future classifications, rejected
  claims, retained risks, and source evidence.
- Paired control evidence uses the same report path while withholding
  candidate-only MDIO/PHY/Ethernet target facts and classifies as
  no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-control.
- Validators reject missing source contract, control target leakage, wrong
  identity, wrong target, wrong PHY ID / MAN frame fields, missing source
  evidence, runtime MDIO transaction claims, MDIO/PHY ownership, NCR.MPE write
  permission, GPIO32/PHY reset ownership, Ethernet readiness, broad MMIO
  readiness, interrupt completion, DMA/descriptors, packet I/O, networking,
  sockets, SSH, Phase 12.2, and phase transition claims.
- Focused rp1_ethernet tests cover accepted candidate construction, accepted
  paired control construction, and deterministic rejection cases.

## Findings

- fixed: implemented source-backed local/static candidate guard evidence for
  the accepted Clause 22 phy1 PHY-ID discriminator.
- fixed: implemented paired no-MDIO/no-Ethernet control evidence that withholds
  MDIO target addresses, PHY address/registers, MAN frames, and operation
  details while preserving the same report path.
- fixed: validators reject target, identity, frame, source-evidence, runtime
  MDIO, ownership, downstream runtime, and phase-transition overclaims before a
  future proof can consume the guard.
- fixed: focused tests cover candidate, control, and deterministic rejection
  cases under the rp1_ethernet filter.
- deferred: serialized Pi 5 MDIO PHY-ID proof, actual MDIO transaction evidence,
  any future NCR.MPE write authority, PHY reset ownership, Ethernet runtime
  behavior, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition remain future queued or supervisor-owned work.
- not-an-issue: hardwareTestLock was not acquired because this task is
  local/static only and performs no hardware action.
- removed: no obsolete code or evidence was removed.

## Accepted Guard Surface

Candidate classification:
rp1-ethernet-mdio-phy-id-guard-candidate-local-static.

Control classification:
no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-control.

~~~text
source contract: phase12-rp1-ethernet-mdio-phy-id-source-contract-v1
report contract: phase12-rp1-ethernet-mdio-phy-id-guard-report-contract-v1
selected discriminator: rp1-ethernet-mdio-clause22-phy1-physid1-physid2
controller: rp1_eth / raspberrypi,rp1-gem / cdns,macb
phy: phy1 / ethernet-phy@1 / address 1
phy ID registers: MII_PHYSID1 0x02, MII_PHYSID2 0x03
observed MACB_MID context: 0x1c001000fc
translated comparator: 0x1f001000fc
NCR observed target: 0x1c00000000, MPE bit 4
NSR observed target: 0x1c00000008, IDLE bit 2
MAN observed target: 0x1c00000034, DATA bits 15:0
PHYSID1 MAN frame: 0x600a0000
PHYSID2 MAN frame: 0x600e0000
precondition: NCR.MPE must already be set; no NCR.MPE write is accepted
~~~

The guard is a local/static report surface only. It does not perform or accept
runtime MDIO transactions.

## Evidence

- Implementation:
  src/rp1_ethernet.rs.
- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-guard-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-guard-core/evidence-map.json.
- Accepted source contract:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-source-contract.md.

## Validation

- static inspection: accepted MDIO/PHY-ID source contract and touched
  src/rp1_ethernet.rs.
- fmt: cargo fmt --all -- --check.
- focused tests: cargo -Zjson-target-spec test --quiet rp1_ethernet.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- documentation build: not required; no docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Candidate report construction preserves the accepted MDIO/PHY-ID contract
  fields and rejected-claim boundaries: satisfied.
- Paired control uses the same report path while withholding candidate-only
  MDIO/PHY/Ethernet target facts: satisfied.
- Validator coverage rejects overclaims and unsafe/ambiguous guard input:
  satisfied.
- Focused tests cover accepted candidate report, accepted control report, and
  deterministic rejection cases: satisfied.
- Accepted implementation/evidence is committed before closeout starts:
  satisfied by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-phy-id-guard-closeout-20260611 on the next worker
wake if dependencies remain satisfied. Do not run hardware or perform runtime
MDIO transactions from this task.
