# Phase 12 RP1 Ethernet GPIO32 PHY-Reset Preflight Closeout

Task id: phase12-rp1-ethernet-gpio32-phy-reset-preflight-closeout-20260610
Status: accepted
Owner: worker
Classification:
rp1-ethernet-gpio32-phy-reset-preflight-frontier-closed
Evidence level: static inspection of accepted source contract, accepted
local/static preflight core, task-owned evidence, project docs, roadmap, and
git history. No Pi 5 hardware run, boot archive publication,
hardwareTestLock acquisition, RP1 GPIO/RIO/pad/MMIO write, PHY reset
assertion or deassertion, MDIO transaction, packet I/O, networking, sockets,
SSH, Phase 12.2, or phase transition was performed by this closeout.

## Goal

Close out the accepted local/static GPIO32 / ETH_RST_N PHY-reset preflight
report core and decide whether the serialized read-only Pi 5 preflight proof
is mechanically objective.

## Scope

- Consumed the accepted GPIO32 PHY-reset source contract from commit
  278b754ff3a8f589429cd91aaadc3085db6e7b90.
- Consumed the accepted local/static preflight report core from commit
  af6ab83de7ce6ef7569ea632bac30b80bda06d2b.
- Reconciled the candidate/control report surface, rejected claims, retained
  risks, future write/restore safety invariants, and validation evidence.
- Closed same-shaped local/static GPIO32 PHY-reset report retries for this
  candidate/control pair.
- Selected only the already queued serialized read-only Pi 5 preflight proof
  as the next mechanically objective follow-up because the accepted core
  explicitly selected this closeout, this closeout has explicit criteria, the
  proof has explicit criteria, and hardwareTestLock is the only runtime
  serialization gate.

## Findings

- fixed: reconciled the accepted source contract boundary:
  `phase12-rp1-ethernet-gpio32-phy-reset-source-contract-v1`, `rp1_eth`,
  RGMII-ID `phy1`, `rp1_gpio` line 32 / ETH_RST_N, active-low polarity,
  logical assertion/deassertion mapping, source reset duration 5 ms, and the
  Linux MACB MDIO reset hook relationship.
- fixed: reconciled the accepted report core boundary:
  `phase12-rp1-ethernet-gpio32-phy-reset-preflight-report-contract-v1` with
  candidate classification
  `rp1-ethernet-gpio32-phy-reset-preflight-candidate-local-static` and paired
  control classification
  `no-gpio-no-ethernet-rp1-ethernet-gpio32-phy-reset-control`.
- fixed: confirmed the candidate report preserves accepted input frontier,
  source identities, GPIO32/ETH_RST_N facts, Phase 11 GPIO constraints,
  future write/restore safety invariants, rejected claims, retained risks, and
  future proof boundary fields.
- fixed: confirmed the paired control uses the same report path while
  constructing no GPIO32/ETH_RST_N/PHY-reset target and withholding
  candidate-only facts.
- fixed: confirmed focused local/static tests cover accepted candidate
  construction, accepted control construction, and deterministic rejection of
  source/shape bypasses, GPIO ownership, PHY reset assertion/deassertion,
  MDIO/PHY ownership, runtime writes, packet I/O, Phase 12.2, and phase
  transition claims.
- not-an-issue: no hardwareTestLock was acquired because this closeout is
  static-only and makes no hardware claim.
- deferred: serialized read-only Pi 5 preflight visibility, GPIO32 reset
  write/restore ownership, MDIO/PHY ownership, packet I/O, networking,
  sockets, SSH, Phase 12.2, and phase transition remain future or rejected
  scope.

No findings were removed.

## Accepted Boundary

The accepted frontier is closed at one local/static GPIO32 PHY-reset
candidate/control preflight report surface. The candidate report preserves the
source-backed GPIO32 / ETH_RST_N PHY reset facts and accepted constraints. The
paired no-GPIO/no-Ethernet control proves the same report path can be built
without candidate-only GPIO32, PHY reset, MDIO, or Ethernet target facts.

This closeout does not accept hardware visibility, GPIO ownership, PHY reset
assertion/deassertion, MDIO/PHY ownership, RP1 GPIO/RIO/pad/MMIO writes,
Ethernet driver readiness, DMA, descriptors, interrupts, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

## Same-Shaped Retry Policy

Same-shaped local/static GPIO32 PHY-reset preflight report retries are closed
for this candidate/control pair. Repeating the same source contract and report
construction would not create a new boundary.

The next accepted step is not a write-backed reset operation. It is only the
already queued serialized read-only Pi 5 preflight proof, which must preserve
candidate/control identity, selected-tree/TFTP joins, fresh serial evidence,
final identity, restore proof, and task-owned JSON. The proof must still reject
GPIO ownership, PHY reset assertion/deassertion, MDIO/PHY ownership, packet
I/O, networking, sockets, SSH, Phase 12.2, and phase transition claims.

## Evidence

- Source contract task:
  tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-source-contract.md.
- Source contract classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-source-contract/classification.json.
- Preflight core task:
  tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-preflight-core.md.
- Preflight core classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-preflight-core/classification.json.
- Preflight core evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-preflight-core/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-preflight-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-preflight-closeout/evidence-map.json.
- Project docs:
  docs/src/project/phase12-networking-ssh.md.
- Roadmap:
  docs/src/roadmap.md.

## Validation

- static inspection: accepted source contract, accepted preflight core,
  task-owned evidence, project docs, roadmap, and git history reviewed.
- JSON validation: jq empty on closeout classification/evidence-map JSON
  passed.
- diff check: git diff --check passed.
- documentation build: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed.

## Acceptance Check

- Task record lists findings with disposition: satisfied.
- Checkpoint reconciles source contract, report core, validation, rejected
  claims, retained risks, and deferred work without expanding acceptance to
  hardware visibility or GPIO/PHY/MDIO ownership: satisfied.
- Checkpoint states whether same-shaped local/static GPIO32 PHY-reset
  preflight report retries are closed: satisfied; closed for this
  candidate/control pair.
- NextAction selects the read-only Pi 5 preflight proof only if mechanically
  objective and hardwareTestLock is the only runtime serialization gate:
  satisfied; the explicit queued proof is selected.
- Accepted checkpoint is committed before any follow-up starts: satisfied by
  the commit recorded in supervisor state after this task.

## Next Action

Mechanically promote
phase12-rp1-ethernet-gpio32-phy-reset-readonly-preflight-pi5-proof-20260610 on
the next worker wake if hardwareTestLock remains unlocked. The proof must be
read-only and serialized. It must not accept GPIO ownership, PHY reset
assertion/deassertion, MDIO/PHY ownership, packet I/O, networking, sockets,
SSH, Phase 12.2, or a phase transition.
