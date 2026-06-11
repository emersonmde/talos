# Phase 12 RP1 Ethernet GPIO32 Event-Clear Pi 5 Proof

Task id: phase12-rp1-ethernet-gpio32-event-clear-pi5-proof-20260611

Classification: event-clear-persistent-or-firmware-owned-blocker

Evidence level: serial hardware boot/output plus lab-controller API identity,
TFTP, archive, and restore evidence.

## Goal

Run one serialized Pi 5 proof of the accepted GPIO32 event-clear guard,
limited to clearing source-backed event bits and preserving no-reset/no-output
invariants.

## Scope Completed

- Added the thinnest hardware archive path for the accepted GPIO32 event-clear
  guard: candidate and paired control boot scenarios, image/boot-tree archive
  scripts, and archive review scripts.
- Acquired hardwareTestLock before archive publication and retained pre-run
  status, boot files, snapshots, and a named restore snapshot.
- Published and ran a candidate archive that pre-read GPIO32 STATUS/CTRL,
  RIO1 OUT/OE/IN, and pad; wrote only GPIO32 CTRL SET IRQRESET value
  0x10000000; then re-read the same fields.
- Published and ran a paired control archive with a fresh nonce after rejecting
  an earlier reused-nonce control capture. The accepted control used the same
  capture-chain-v4 path while constructing no GPIO32/RIO/pad/MMIO targets and
  performing no clear.
- Restored the pre-run boot snapshot before hardware lock release.

## Findings

- fixed: candidate/control boot scenarios and archive/review scripts now cover
  the event-clear hardware proof path.
- fixed: candidate capture-chain-v4 passed with selected-tree hash
  8d4e5dacebe7ebbc467e52236d351f13511b74d44eb4be5a825e3b487bf2ed19,
  expected fetch da591740/kernel_2712.img at 51128 bytes, run-unique nonce
  eventclear-candidate-20260611T0500Z, stable TFTP delta, final identity, and
  restore evidence.
- fixed: candidate pre-state matched the source contract: STATUS 0x0abe3300,
  CTRL 0x85, RIO1 OUT/OE 0x10, RIO1 IN 0x12, pad 0x56, event bits
  0x0ab00000, FUNCSEL 5, and override bits 0.
- fixed: candidate performed only GPIO32_CTRL_SET.IRQRESET with value
  0x10000000. Post-state preserved CTRL 0x85, RIO1 OUT/OE 0x10, RIO1 IN
  0x12, and pad 0x56, but event bits persisted as 0x08800000, so the proof
  classifies as event-clear-persistent-or-firmware-owned-blocker.
- fixed: accepted paired control-rerun2 passed capture-chain-v4 with selected
  tree hash 7bd3b8ac723d9559128f8c07fbd7436bf56481b961465ec3177a745527577b26,
  expected fetch da591740/kernel_2712.img at 49616 bytes, run-unique nonce
  eventclear-control-rerun2-20260611T0528Z, stable TFTP delta, final identity,
  and restore evidence.
- removed: no obsolete source was removed.
- deferred: GPIO32 ownership, PHY reset ownership, GPIO32 write/restore retry,
  MDIO/PHY, Ethernet driver behavior, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition remain unaccepted.
- not-an-issue: an earlier control attempt reused the previous nonce and could
  not prove serial freshness. It was superseded before acceptance by
  control-rerun2 and is not the accepted paired control.

## Evidence

- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-pi5-proof/capture-summary.json.
- Candidate bundle:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-pi5-proof/candidate-run/.
- Accepted control bundle:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-pi5-proof/control-rerun2/.
- Archive reviews:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-clear-pi5-proof/archive-review/.

## Validation

- Static inspection: accepted source contract, guard core, guard closeout, and
  touched source/scripts.
- cargo fmt --all -- --check: passed.
- cargo -Zjson-target-spec check --quiet --target
  targets/aarch64-talos-rpi5-bcm2712.json: passed.
- Archive review scripts: passed for candidate and accepted control-rerun2.
- Pi 5 lab-controller API evidence: retained archive publication, power cycle,
  stable TFTP, final identity, and restore records.
- Fresh serial capture: candidate and control-rerun2 passed
  rpi5-proof-identity-join-v4-check.sh as capture-chain-v4-ready.
- jq empty on task-owned classification/evidence-map/capture-summary JSON:
  passed.

## Boundary

Accepted only that the source-backed IRQRESET event-clear attempt leaves
STATUS event bits persistent or firmware-owned while preserving no-reset and
no-output invariants. This proof does not accept GPIO32 ownership, PHY reset
assertion/deassertion, GPIO32 write/restore retry or success, MDIO/PHY
ownership, Ethernet driver behavior, interrupts, DMA/descriptors, packet I/O,
networking, sockets, SSH, Phase 12.2, or a phase transition.

## Next Action

Promote phase12-rp1-ethernet-gpio32-event-clear-proof-closeout-20260611 if it
is mechanically unblocked. Do not interpret the event-clear blocker as GPIO32
or PHY reset ownership.
