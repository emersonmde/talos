# Phase 12 RP1 Ethernet GPIO32 PHY-Reset Write/Restore Guard Closeout

Task id: phase12-rp1-ethernet-gpio32-phy-reset-write-restore-guard-closeout-20260610
Status: accepted
Owner: worker
Classification:
rp1-ethernet-gpio32-phy-reset-write-restore-guard-frontier-closed
Evidence level: static inspection of accepted source contract, accepted
local/static guard core evidence, focused tests, documentation, and git
history. No Pi 5 hardware run, boot archive publication, hardwareTestLock
acquisition, runtime GPIO/RIO/pad/MMIO write, PHY reset
assertion/deassertion, MDIO transaction, Ethernet driver behavior, packet I/O,
networking, sockets, SSH, Phase 12.2, or phase transition was performed.

## Goal

Close out the accepted local/static GPIO32 / ETH_RST_N write/restore guard
core and decide whether the serialized Pi 5 proof is mechanically objective.

## Findings

- fixed: reconciled the accepted write/restore source contract
  rp1-ethernet-gpio32-phy-reset-write-restore-source-contract-accepted from
  commit e44d5d1f80104f8058a446fd7bfae8a988255eb3.
- fixed: reconciled the accepted guard core classification
  rp1-ethernet-gpio32-phy-reset-write-restore-guard-core-local-static-accepted
  from commit 78716ed2caadb22ad1b172831fd64c8b22d3a3d8.
- fixed: confirmed the local/static candidate report preserves the exact
  GPIO32 / ETH_RST_N target identity: GPIO32 STATUS/CTRL at
  0x1c000d4020/0x1c000d4024, RIO1 OUT/OE/IN at
  0x1c000e4000/0x1c000e4004/0x1c000e4008, GPIO32 pad state at
  0x1c000f4014, bank1 bit 4, active-low assertion/deassertion semantics, and
  the retained 5 ms source reset duration.
- fixed: confirmed the candidate report carries preconditions, restore
  baseline fields, operation order, blocked/no-write classifications, allowed
  future proof classifications, rejected claims, retained risks, and source
  evidence without performing or implying a runtime write.
- fixed: confirmed the paired control uses the same report path while carrying
  no writable GPIO32/RIO/pad/MMIO target facts and withholding candidate-only
  GPIO32/ETH_RST_N write/restore facts.
- fixed: confirmed focused tests covered accepted candidate construction,
  accepted no-GPIO-write/no-Ethernet control construction, blocked/no-write
  construction, shape bypasses, missing restore baseline, non-GPIO32 writes,
  MDIO/PHY overclaims, interrupt/DMA overclaims, Phase 12.2 claims, and
  rejected-evidence naming.
- fixed: selected the queued serialized Pi 5 candidate/control proof as
  mechanically objective because the accepted source contract and guard core
  already define exact candidate/control report shapes, no-write preconditions,
  restore requirements, allowed classifications, rejected claims, validation
  gates, and task-owned evidence artifacts.
- not-an-issue: hardwareTestLock was not acquired because this closeout is
  static reconciliation only.
- deferred: serialized Pi 5 write/restore proof, hardware restore readback,
  MDIO/PHY ownership, Ethernet driver readiness, interrupt completion,
  DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, and phase
  transition remain future or explicitly rejected scope.

No findings were removed.

## Accepted Boundary

The accepted frontier is closed at a local/static report surface for one exact
GPIO32 / ETH_RST_N write/restore candidate/control pair. The guard core
provides a candidate report for GPIO32 bank1 bit 4 and a paired no-GPIO-write
control through the same output path. It does not prove runtime GPIO/RIO/pad
or MMIO writes, PHY reset assertion/deassertion on Pi 5, hardware restore
readback, MDIO/PHY ownership, Ethernet driver readiness, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

The next serialized proof is mechanically objective because the accepted
source contract names one GPIO32 / ETH_RST_N target, no-write preconditions,
operation sequence, restore baseline fields, paired control requirements,
rejected claims, retained risks, and classification set; the accepted guard
core implements and tests candidate/control/blocked report construction; the
proof task already defines candidate/control capture gates, restore
requirements, and task-owned evidence artifacts; and hardwareTestLock is
currently unlocked.

## Same-Shaped Retry Policy

Same-shaped local/static GPIO32 / ETH_RST_N write/restore report retries are
closed for this candidate/control pair. Future local/static work needs
materially different scope or acceptance criteria. The next selected boundary
is the serialized Pi 5 candidate/control write/restore proof.

## Proof Authorization

The next proof must acquire hardwareTestLock before archive publication,
staging, power cycling, or any runtime GPIO/RIO/pad/MMIO write. Candidate and
control must each join selected boot-tree identity, expected TFTP fetches,
fresh serial nonce/marker evidence, final pre-restore identity, and restore
proof in one retained capture-chain transaction.

The control must preserve the same report path while constructing no writable
GPIO32/RIO/pad/MMIO target and no candidate-only PHY-reset facts. The
candidate may write only the accepted GPIO32 / ETH_RST_N ownership fields
needed to assert active-low reset, wait 5 ms, deassert reset, and restore every
touched field to the captured baseline. Any sentinel/all-ones/unsafe
function/event/missing-baseline/capture-chain failure must classify as an
accepted blocked/no-write result before any store.

## Evidence

- Source contract task record:
  tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-source-contract.md.
- Source contract classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-source-contract/classification.json.
- Guard core task record:
  tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-guard-core.md.
- Guard core classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-guard-core/classification.json.
- Guard core evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-guard-core/evidence-map.json.
- Closeout classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-guard-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-guard-closeout/evidence-map.json.

## Validation

- static inspection: accepted source contract, guard core task record,
  classification/evidence JSON, focused tests, docs, and git history reviewed.
- JSON validation: jq empty on closeout classification/evidence-map JSON
  passed.
- diff check: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed.

## Next Action

Mechanically promote
phase12-rp1-ethernet-gpio32-phy-reset-write-restore-pi5-proof-20260610 on the
next worker wake if hardwareTestLock remains unlocked. The proof must acquire
hardwareTestLock before archive publication, staging, power cycling, or any
runtime GPIO/RIO/pad/MMIO write and must remain bounded to candidate/control
GPIO32 / ETH_RST_N write/restore or accepted blocked/no-write evidence. Do not
broaden to MDIO/PHY ownership, Ethernet driver readiness, interrupts,
DMA/descriptors, packet I/O, networking, sockets, SSH, Phase 12.2, or a phase
transition.
