# Phase 12 RP1 Ethernet MDIO Register Vector Guard Core

Task id: phase12-rp1-ethernet-mdio-register-vector-guard-core-20260611

Status: accepted

Classification:
rp1-ethernet-mdio-register-vector-guard-core-accepted

Evidence level: local/static implementation, focused unit tests, JSON
validation, diff hygiene, and git history. No Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, runtime MMIO load/store, NCR write,
MAN write on hardware, GPIO32/PHY reset action, Ethernet driver behavior,
DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or phase
transition was performed.

## Goal

Implement the local/static corrected-target MDIO register-vector guard report
and paired no-MDIO/no-Ethernet control selected by the accepted source
contract.

## Scope Performed

- Added a deterministic register-vector guard surface in src/rp1_ethernet.rs.
- Candidate evidence preserves the accepted source contract id, selected
  discriminator, purpose, MACB_MID context target 0x1c001000fc, corrected
  NCR/NSR/MAN targets 0x1c00100000/0x1c00100008/0x1c00100034, PHY address 1,
  the six selected Clause 22 registers, exact PHYA=1 MAN frames, MPE
  precondition, bounded NSR.IDLE polling policy, MAN.DATA extraction, rejected
  claims, retained risks, and hardware-proof boundary.
- Paired control evidence uses the same report path while constructing no MDIO
  targets, no MAN frames, and no runtime volatile load/store intent.
- Validators reject missing source contracts, control target leakage, source
  contract identity/target/field drift, missing source evidence, runtime
  volatile load/store claims, NCR writes, missing MPE gating, wrong
  0x1c00000000-era MDIO targets, unbounded polling, MAN writes without an MPE
  precondition, PHY-absence claims from all-ones vectors, broad MDIO/PHY
  ownership, GPIO32/PHY reset actions, Ethernet readiness, interrupts,
  DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, and
  phase-transition claims.
- Focused rp1_ethernet tests cover accepted candidate construction, accepted
  paired control construction, and deterministic rejection cases.

## Findings

- fixed: implemented the accepted six-register Clause 22 register-vector guard
  surface with corrected observed-window MACB/GEM NCR/NSR/MAN targets.
- fixed: preserved exact MAN frames with PHYA bits for phy1 address 1:
  0x60820000, 0x60860000, 0x608a0000, 0x608e0000, 0x60920000, and 0x60960000.
- fixed: kept the MPE precondition explicit while rejecting NCR writes and MAN
  writes that lack the corrected NCR.MPE gate.
- fixed: implemented a paired no-MDIO/no-Ethernet control that withholds
  candidate-only target, register-vector, MAN-frame, and result fields.
- fixed: validators reject all-ones-to-PHY-absence overclaims and broader
  Ethernet/networking/phase-transition claims.
- deferred: serialized Pi 5 register-vector proof, actual MAN.DATA vector
  evidence, PHY reset/GPIO32 ownership, broad MDIO/PHY ownership, Ethernet
  driver behavior, interrupts, DMA/descriptors, packet I/O, networking,
  sockets, SSH, Phase 12.2, and phase transition remain future explicit tasks.
- not-an-issue: hardwareTestLock was not acquired because this task is
  local/static only and performs no hardware action.
- removed: no obsolete code or evidence was removed.

## Accepted Guard Surface

Candidate classification:
rp1-ethernet-mdio-register-vector-guard-candidate-local-static.

Control classification:
no-mdio-no-ethernet-rp1-ethernet-mdio-register-vector-control.

~~~text
source contract: phase12-rp1-ethernet-mdio-register-vector-source-contract-v1
report contract: phase12-rp1-ethernet-mdio-register-vector-guard-report-contract-v1
selected discriminator: rp1-ethernet-mdio-after-mpe-clause22-phy1-register-vector
purpose: distinguish global all-ones/no-response behavior from PHY-ID-only evidence
MACB_MID context: 0x1c001000fc raw 0x00070109
NCR target: 0x1c00100000
NSR target: 0x1c00100008
MAN target: 0x1c00100034
PHY address: 1
register vector:
  MII_BMCR 0x00 -> 0x60820000
  MII_BMSR 0x01 -> 0x60860000
  MII_PHYSID1 0x02 -> 0x608a0000
  MII_PHYSID2 0x03 -> 0x608e0000
  MII_ADVERTISE / ANAR 0x04 -> 0x60920000
  MII_LPA / ANLPAR 0x05 -> 0x60960000
MPE precondition: corrected NCR.MPE bit 4 must already be set before any MAN write
polling policy: bounded NSR.IDLE bit 2 poll before and after each MAN write
result extraction: MAN.DATA bits 15:0
~~~

The guard is a local/static report surface only. It does not perform or accept
runtime volatile load/store, NCR writes, MAN writes on hardware, PHY reset
actions, or Ethernet behavior.

## Evidence

- Implementation:
  src/rp1_ethernet.rs.
- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-guard-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-guard-core/evidence-map.json.
- Accepted register-vector source contract:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-register-vector-source-contract.md.
- Accepted after-MPE PHY-ID proof closeout:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-proof-closeout.md.

## Validation

- static inspection: accepted register-vector source contract, accepted
  after-MPE proof closeout, existing MDIO guard surfaces, and touched
  src/rp1_ethernet.rs.
- fmt: cargo fmt --all -- --check.
- focused tests: cargo -Zjson-target-spec test --quiet rp1_ethernet.
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- documentation build: /home/node/.cargo/bin/mdbook build.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Implementation exposes deterministic candidate and paired no-MDIO/no-Ethernet
  control report surfaces for the selected register-vector discriminator:
  satisfied.
- Candidate report includes required corrected-target, register-vector, MPE
  gate, polling, extraction, rejected-claim, retained-risk, and hardware-proof
  boundary fields: satisfied.
- Control report uses the same reporting path while withholding candidate-only
  MDIO targets, MAN frames, and result fields: satisfied.
- Focused tests cover accepted candidate report construction, accepted control
  report construction, and deterministic rejection of wrong-target, ungated-MAN,
  all-ones-to-PHY-absence, GPIO32/PHY-reset, Ethernet-readiness,
  networking/SSH, Phase 12.2, and phase-transition overclaims: satisfied.
- Accepted implementation/evidence is committed before closeout starts:
  satisfied by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-register-vector-guard-closeout-20260611 on the next
worker wake if dependencies remain satisfied. Do not run hardware, acquire
hardwareTestLock, write NCR, write MAN on hardware, touch GPIO32/PHY reset,
infer PHY absence from all-ones vectors, infer broad MDIO/PHY ownership, start
Ethernet behavior, Phase 12.2, or a phase transition from this local/static
guard.
