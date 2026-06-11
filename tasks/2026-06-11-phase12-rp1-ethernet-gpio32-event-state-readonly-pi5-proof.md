# Phase 12 RP1 Ethernet GPIO32 Event-State Read-Only Pi 5 Proof

Task id: phase12-rp1-ethernet-gpio32-event-state-readonly-pi5-proof-20260611

Classification: rp1-ethernet-gpio32-event-state-readonly-pi5-proof-accepted

Evidence level: serial hardware boot/output plus lab-controller API identity,
TFTP, archive, and restore evidence.

## Goal

Run the serialized read-only Pi 5 GPIO32 event-state discriminator proof with a
paired no-GPIO/no-Ethernet control and no GPIO/RIO/pad/MMIO writes or event
clearing.

## Scope Completed

- Promoted the queued task only after the accepted static closeout selected this
  proof and hardwareTestLock was unlocked.
- Added the thinnest hardware archive path for the accepted event-state
  discriminator: candidate and paired control boot scenarios, image/boot-tree
  archive scripts, and archive review scripts.
- Built candidate/control archives with run-unique nonces and statically
  inspected the images before publication.
- Acquired hardwareTestLock before boot archive publication and retained
  pre-run status, boot files, snapshots, and a named restore snapshot.
- Published and ran fresh candidate/control rerun archives through the same
  capture-chain-v4 path.
- Retained selected-tree identity, archive digest, run-unique serial evidence,
  stable TFTP delta, final pre-restore identity, restore evidence, v4 checker
  output, classification JSON, and evidence map.

## Findings

- fixed: added rpi5_rp1_ethernet_gpio32_event_state_candidate and
  rpi5_rp1_ethernet_gpio32_event_state_no_mmio_control boot scenarios and
  registered them in build.rs so archive builds are first-class checked cfg
  values.
- fixed: candidate serial report read only GPIO32 STATUS/CTRL, RIO1 OUT/OE/IN,
  and pad; it reported STATUS 0x0abe3300, CTRL 0x85, RIO1 OUT/OE 0x10,
  RIO1 IN 0x12, pad 0x56, event bits 0x0ab00000, source decoding
  source-backed-bits-20-27, writes-performed=false, and
  event-clear-performed=false.
- fixed: paired control used the same capture path with no GPIO32/RIO/pad/MMIO
  target facts and classified as
  no-gpio-no-ethernet-rp1-ethernet-gpio32-event-state-control.
- fixed: both rerun proof bundles passed rpi5-proof-identity-join-v4-check.sh
  with capture-chain-v4-ready, empty rejection reasons, selected tree/TFTP
  final identity, run-unique serial marker evidence, and restore evidence.
- fixed: an initial malformed shell interpolation launched an overlapping,
  aborted helper path; I killed the stray helpers, manually restored the
  pre-run snapshot, preserved recovery evidence, moved aborted bulky artifacts
  under target/talos-aborted-overlap-20260611T0254Z, and reran with fresh
  nonces before accepting any hardware evidence.
- deferred: rp1-ethernet-gpio32-event-state-blocked-event-state remains a
  planning input only. It does not authorize event clearing, GPIO32 ownership,
  PHY reset, write/restore retry, MDIO/PHY ownership, Ethernet readiness,
  networking, SSH, or a phase transition.
- not-an-issue: candidate/control archive sizes changed between nonce builds
  because the run-unique nonce text is embedded in the image; each accepted
  run records its own archive review and expected fetch byte count.

## Evidence

- Classification:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-readonly-pi5-proof/classification.json.
- Evidence map:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-readonly-pi5-proof/evidence-map.json.
- Capture summary:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-readonly-pi5-proof/capture-summary.json.
- Candidate rerun bundle:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-readonly-pi5-proof/candidate-rerun/.
- Control rerun bundle:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-readonly-pi5-proof/control-rerun/.
- Archive reviews:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-readonly-pi5-proof/archive-review/.
- Recovery evidence:
  tasks/evidence/2026-06-11-phase12-rp1-ethernet-gpio32-event-state-readonly-pi5-proof/manual-restore-after-overlap-status.json.

## Validation

- Static archive/image inspection before publication: passed for candidate and
  control rerun archives.
- cargo fmt --all -- --check: passed after formatting.
- Required compile/archive generation: passed for candidate and control rerun
  scenarios.
- Lab-controller API evidence: passed; retained status, boot files, snapshots,
  archive publication, power cycle, TFTP delta, final pre-restore identity, and
  restore records.
- Fresh serial capture: passed through run-unique capture-chain-v4 checks for
  both candidate and control reruns.
- jq empty on task-owned classification/evidence-map/capture-summary JSON:
  passed.

## Boundary

Accepted only the read-only event-state hardware discriminator. The proof does
not accept GPIO32 ownership, event clearing, PHY reset assertion/deassertion,
GPIO32 write/restore retry or success, MDIO/PHY ownership, Ethernet driver
behavior, interrupts, DMA/descriptors, packet I/O, networking, sockets, SSH,
Phase 12.2, or a phase transition.

## Next Action

Promote
phase12-rp1-ethernet-gpio32-event-state-proof-closeout-20260611 on the next
worker wake if supervisorIntervention remains inactive and hardwareTestLock is
unlocked. The closeout must reconcile the accepted blocked-event-state hardware
proof and must not authorize event clearing or GPIO/RIO/pad/MMIO writes.
