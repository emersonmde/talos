# Phase 12 RP1 Ethernet MDIO PHY ID After-MPE Guard Core

Task id: phase12-rp1-ethernet-mdio-phy-id-after-mpe-guard-core-20260611

Status: accepted

Classification:
rp1-ethernet-mdio-phy-id-after-mpe-guard-core-accepted

Evidence level: local/static implementation, focused unit tests, JSON
validation, diff hygiene, and git history. No Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, runtime MMIO load/store, NCR write,
MAN write, PHY-ID read, PHY reset or GPIO32 action, Ethernet driver behavior,
DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or phase
transition was performed.

## Goal

Implement the local/static corrected-target MDIO PHY-ID guard report and paired
no-MDIO/no-Ethernet control selected by the accepted after-MPE source contract.

## Scope Performed

- Added a separate after-MPE guard surface in src/rp1_ethernet.rs instead of
  rewriting the earlier wrong-target MDIO PHY-ID blocker surface.
- Candidate evidence preserves the accepted after-MPE source contract id,
  accepted NCR.MPE ownership frontier, MACB_MID context target 0x1c001000fc,
  corrected NCR/NSR/MAN targets 0x1c00100000/0x1c00100008/0x1c00100034, PHY
  address 1, PHYSID1/PHYSID2 register ids, MAN frame values 0x600a0000 and
  0x600e0000, MPE precondition, bounded NSR.IDLE polling policy, MAN.DATA
  extraction, rejected claims, retained risks, and hardware-proof boundary.
- Paired control evidence uses the same report path while constructing no MDIO
  targets, no MAN frames, and no runtime volatile load/store intent.
- Validators reject missing source contracts, control target leakage, source
  contract identity/target/field drift, missing source evidence, runtime
  volatile load/store claims, NCR writes, missing MPE gating, wrong
  0x1c00000000-era MDIO targets, unbounded polling, MAN writes without an MPE
  precondition, broad MDIO/PHY ownership, GPIO32/PHY reset actions, Ethernet
  readiness, interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase-transition claims.
- Focused rp1_ethernet tests cover accepted candidate construction, accepted
  paired control construction, and deterministic rejection cases.

## Findings

- fixed: implemented a corrected-target after-MPE candidate report surface with
  NCR/NSR/MAN targets rooted at observed-window MACB/GEM base 0x1c00100000.
- fixed: preserved the accepted NCR.MPE ownership frontier as explicit input
  context for the next proof without granting NCR write permission in this
  guard.
- fixed: preserved the prior wrong-target MDIO PHY-ID surface as closed blocker
  history while adding a distinct after-MPE contract/report identity.
- fixed: implemented a paired control report that withholds MDIO targets, MAN
  frames, candidate result fields, and volatile access intent.
- fixed: validators reject NCR writes, missing MPE gating, wrong-target inputs,
  unbounded polling, MAN writes without MPE precondition, GPIO32/PHY-reset
  actions, Ethernet readiness, networking/SSH, Phase 12.2, and phase-transition
  overclaims.
- deferred: serialized Pi 5 corrected-target PHY-ID proof, actual MAN
  transaction evidence, visible PHY-ID reads, PHY reset/GPIO32 ownership,
  Ethernet driver behavior, interrupts, DMA/descriptors, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition remain future
  explicit tasks.
- not-an-issue: hardwareTestLock was not acquired because this task is
  local/static only and performs no hardware action.
- removed: no obsolete code or evidence was removed.

## Accepted Guard Surface

Candidate classification:
rp1-ethernet-mdio-phy-id-after-mpe-guard-candidate-local-static.

Control classification:
no-mdio-no-ethernet-rp1-ethernet-mdio-phy-id-after-mpe-control.

~~~text
source contract: phase12-rp1-ethernet-mdio-phy-id-after-mpe-source-contract-v1
report contract: phase12-rp1-ethernet-mdio-phy-id-after-mpe-guard-report-contract-v1
accepted MPE frontier: rp1-ethernet-mdio-mpe-enable-ownership-frontier-closed
selected discriminator: rp1-ethernet-mdio-after-mpe-clause22-phy1-physid1-physid2
MACB_MID context: 0x1c001000fc raw 0x00070109
NCR target: 0x1c00100000
NSR target: 0x1c00100008
MAN target: 0x1c00100034
PHY address: 1
PHY ID registers: MII_PHYSID1 0x02, MII_PHYSID2 0x03
MAN frames: PHYSID1 0x600a0000, PHYSID2 0x600e0000
MPE precondition: corrected NCR.MPE bit 4 must already be set before any MAN write
polling policy: bounded NSR.IDLE bit 2 poll before and after each MAN write
result extraction: MAN.DATA bits 15:0
~~~

The guard is a local/static report surface only. It does not perform or accept
runtime volatile load/store, NCR writes, MAN writes, PHY-ID reads, or PHY reset
actions.

## Evidence

- Implementation:
  src/rp1_ethernet.rs.
- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-guard-core/classification.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-guard-core/evidence-map.json.
- Accepted after-MPE source contract:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-phy-id-after-mpe-source-contract.md.
- Accepted NCR.MPE proof closeout:
  tasks/2026-06-11-phase12-rp1-ethernet-mdio-mpe-enable-proof-closeout.md.

## Validation

- static inspection: accepted after-MPE source contract, existing MDIO PHY-ID
  and MPE guard surfaces, and touched src/rp1_ethernet.rs.
- fmt: cargo fmt --all -- --check.
- focused tests: cargo -Zjson-target-spec test --quiet rp1_ethernet (504
  talos no_std tests passed, including three new after-MPE guard tests).
- JSON validation: jq empty on task-owned classification/evidence-map JSON.
- diff check: git diff --check.
- documentation build: not required; no docs/src files were touched.
- staged diff check: git diff --cached --check before commit.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Implementation exposes deterministic candidate and paired no-MDIO/no-Ethernet
  control report surfaces for the corrected-target after-MPE discriminator:
  satisfied.
- Candidate report includes all required contract identity, corrected target
  addresses, MPE gate, NSR/MAN operation fields, rejected claims, retained
  risks, and hardware-proof boundary fields: satisfied.
- Control report uses the same reporting path while withholding candidate-only
  MDIO targets, MAN frames, and result fields: satisfied.
- Focused tests cover accepted candidate report construction, accepted control
  report construction, and deterministic rejection of wrong-target, ungated-MAN,
  NCR-write, GPIO32/PHY-reset, Ethernet-readiness, networking/SSH, Phase 12.2,
  and phase-transition overclaims: satisfied.
- Accepted implementation/evidence is committed before guard closeout starts:
  satisfied by the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-mdio-phy-id-after-mpe-guard-closeout-20260611 on the next
worker wake if dependencies remain satisfied. Do not run hardware, acquire
hardwareTestLock, write NCR, write MAN, retry PHY-ID on hardware, touch
GPIO32/PHY reset, infer broad MDIO/PHY ownership, start Ethernet behavior,
Phase 12.2, or a phase transition from this local/static guard.
