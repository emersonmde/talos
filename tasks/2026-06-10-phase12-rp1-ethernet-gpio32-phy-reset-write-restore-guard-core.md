# Phase 12 RP1 Ethernet GPIO32 PHY-Reset Write/Restore Guard Core

Task id: phase12-rp1-ethernet-gpio32-phy-reset-write-restore-guard-core-20260610
Status: accepted
Owner: worker
Classification: rp1-ethernet-gpio32-phy-reset-write-restore-guard-core-local-static-accepted
Evidence level: static inspection, focused QEMU-backed unit tests, JSON
validation, diff checks, and docs build. No Pi 5 hardware run, boot archive
publication, hardwareTestLock acquisition, runtime GPIO/RIO/pad/MMIO write,
PHY reset assertion/deassertion, MDIO transaction, Ethernet driver behavior,
packet I/O, networking, sockets, SSH, Phase 12.2, or phase transition was
performed.

## Goal

Implement the local/static report surface for the accepted GPIO32 / ETH_RST_N
write/restore guard contract, so the later serialized Pi 5 proof has a typed
candidate/control shape and deterministic validators before any hardware
write is attempted.

## Scope

- Consumed accepted write/restore source contract commit
  e44d5d1f80104f8058a446fd7bfae8a988255eb3.
- Added Rp1EthernetGpio32PhyResetWriteRestoreGuardContractEvidence, report
  input/output, report evidence, builder, rejected-evidence helper, and
  validators in src/rp1_ethernet.rs.
- Preserved exact GPIO32 target identity: rp1_gpio line 32 / ETH_RST_N,
  bank1 bit 4, GPIO32 STATUS/CTRL observed targets
  0x1c000d4020/0x1c000d4024, RIO1 OUT/OE/IN observed targets
  0x1c000e4000/0x1c000e4004/0x1c000e4008, and GPIO32 pad observed target
  0x1c000f4014.
- Added preconditions, restore-baseline fields, operation sequence,
  blocked/no-write classification set, future proof classification set,
  rejected claim list, retained risk list, and source evidence linkage.
- Added paired no-GPIO-write/no-Ethernet control construction that withholds
  writable GPIO32/RIO/pad/MMIO target facts.
- Added validators for missing restore baseline, non-GPIO32 writes, MDIO/PHY
  overclaims, interrupt/DMA/descriptor/packet/network/socket/SSH claims,
  Phase 12.2 claims, phase-transition claims, invalid guard identity/targets,
  invalid source evidence, and invalid blocked/no-write classifications.
- Added focused tests for accepted candidate, paired control, blocked/no-write
  report construction, shape bypasses, missing restore baseline, non-GPIO32
  writes, MDIO/PHY overclaims, interrupt/DMA overclaims, Phase 12.2 claims,
  and rejected-evidence naming.
- Updated the Phase 12 project doc with the accepted local/static guard API
  boundary.

## Non-Goals

No hardware run, no boot archive publication, no hardwareTestLock acquisition,
no runtime GPIO/RIO/pad/MMIO write, no PHY reset assertion/deassertion, no
MDIO/PHY register ownership, no interrupt/event ownership, no DMA/descriptors,
no packet I/O, no networking, no sockets, no SSH, no Phase 12.2, no broad GPIO
framework, no generic pinctrl ownership API, and no phase transition.

## Accepted Interface

The accepted report contract id is
phase12-rp1-ethernet-gpio32-phy-reset-write-restore-report-contract-v1. The
guard contract id is
phase12-rp1-ethernet-gpio32-phy-reset-write-restore-source-contract-v1.

The candidate report carries source-backed target identity, no-write
preconditions, restore-baseline fields, operation ordering, blocked/no-write
classifications, future proof classifications, rejected claims, retained
risks, and source evidence. The candidate classification is
rp1-ethernet-gpio32-phy-reset-write-restore-guard-candidate-local-static.

The blocked/no-write report uses the same target-bearing report shape but
requires the classification to be one of the accepted blocked set:

- rp1-ethernet-gpio32-phy-reset-blocked-sentinel-read
- rp1-ethernet-gpio32-phy-reset-blocked-unsafe-function
- rp1-ethernet-gpio32-phy-reset-blocked-unexpected-event-state
- rp1-ethernet-gpio32-phy-reset-blocked-missing-restore-baseline
- rp1-ethernet-gpio32-phy-reset-inconclusive-capture

The paired control preserves the same report path while carrying no writable
GPIO32/RIO/pad/MMIO target facts and classifies as
no-gpio-write-no-ethernet-rp1-ethernet-gpio32-phy-reset-control.

## Findings

- fixed: added a typed local/static guard contract and report surface for
  GPIO32 / ETH_RST_N write/restore evidence.
- fixed: retained exact source-backed and observed GPIO32 STATUS/CTRL,
  RIO1 OUT/OE/IN, and GPIO32 pad targets in candidate evidence.
- fixed: made the complete restore-baseline field set mandatory and rejected
  missing baseline evidence deterministically.
- fixed: added a blocked/no-write report kind so future proofs can report
  no-write precondition failures without claiming GPIO or Ethernet readiness.
- fixed: added paired no-GPIO-write/no-Ethernet control construction that
  withholds target facts.
- fixed: added validators rejecting non-GPIO32 writes, MDIO/PHY ownership,
  interrupt/DMA/descriptor/packet/network/socket/SSH claims, Phase 12.2, and
  phase transition.
- not-an-issue: no hardware lock was needed because this task is local/static
  and performs no runtime write.
- deferred: serialized Pi 5 candidate/control proof, restore readback, and
  hardware blocked/no-write evidence remain for a later explicitly authorized
  task.

## Validation

- static inspection: accepted write/restore source contract, touched
  src/rp1_ethernet.rs, and touched docs/evidence.
- fmt: cargo fmt --all completed.
- unit tests / QEMU substitute: cargo -Zjson-target-spec test --quiet
  rp1_ethernet passed with 489 no_std tests, including the new GPIO32
  write/restore guard candidate/control/blocked/rejection cases.
- JSON validation: jq empty passed for task-owned classification and
  evidence-map JSON.
- diff whitespace: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed.
- staged diff whitespace: git diff --cached --check passed before commit.

## Next Action

Promote
phase12-rp1-ethernet-gpio32-phy-reset-write-restore-guard-closeout-20260610
on the next worker wake if dependencies remain satisfied. Keep that closeout
static only; do not run hardware or perform GPIO/RIO/pad/MMIO writes in the
closeout.
