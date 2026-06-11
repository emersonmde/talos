# Phase 12 RP1 Ethernet GPIO32 PHY-Reset Write/Restore v2 Proof Closeout

Task id: phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-proof-closeout-20260610
Status: accepted
Owner: worker
Classification:
rp1-ethernet-gpio32-phy-reset-write-restore-v2-blocked-no-write-frontier-closed
Evidence level: static inspection of accepted v2 proof task record,
classification/evidence JSON, capture summary, project docs, and git history.
No additional hardware run was performed.

## Goal

Close out the GPIO32 / ETH_RST_N write/restore v2 proof after the committed
blocked/no-write result and decide whether any same-shaped retry or next
Phase 12.1 ownership step is mechanically objective.

## Findings

- fixed: reconciled accepted v3 known-good readiness as the precondition that
  unlocked the v2 proof, with closeout commit
  9fb316e220ad3cdc22cd35ac7398df319853f294.
- fixed: reconciled the accepted v2 proof classification
  rp1-ethernet-gpio32-phy-reset-blocked-unexpected-event-state from commit
  4300e6bc54883c1ec8c4020abd068ee8277ed637.
- fixed: confirmed candidate and paired no-GPIO/no-MMIO control archive
  reviews, selected-tree identities, serial markers, stable TFTP deltas, and
  final restore proof were retained by the v2 proof.
- blocked: the candidate did not perform the GPIO32 write/restore sequence.
  It classified as blocked-unexpected-event-state with writes-performed=false
  after observing baseline-status=0xabe3300, baseline-ctrl=0x85,
  baseline-out=0x10, baseline-oe=0x10, baseline-in=0x12, and
  event-bits=0xab00000.
- fixed: confirmed the paired control used the same report/capture path,
  withheld GPIO write/MMIO target facts, observed its run-unique marker, and
  classified as no-gpio-write-no-ethernet-rp1-ethernet-gpio32-phy-reset-control.
- fixed: confirmed final lab boot tree was restored to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10 with
  effective kernel kernel_2712.img before hardwareTestLock release.
- deferred: GPIO32 write/restore success, GPIO32 ownership, PHY reset
  assertion/deassertion proof, MDIO/PHY ownership, Ethernet driver readiness,
  interrupt completion, DMA/descriptors, packet I/O, networking, sockets, SSH,
  Phase 12.2, and phase transition remain unaccepted.
- not-an-issue: this closeout did not acquire hardwareTestLock or run
  hardware; it is a static reconciliation checkpoint over already committed
  proof evidence.

No findings were removed.

## Accepted Boundary

The accepted checkpoint is a precise blocked/no-write hardware result. The
candidate and paired control both reached selected-tree identity, run-unique
serial output, and stable TFTP deltas under the repaired readiness/capture
chain. That closes the earlier lab no-fetch blocker for this v2 attempt.

The feature attempt itself remains blocked because the candidate observed an
unexpected GPIO32 event state and reported writes-performed=false before any
task-owned GPIO/RIO/pad write. This closeout does not accept GPIO32 ownership,
PHY reset assertion/deassertion, MDIO/PHY ownership, Ethernet driver behavior,
DMA, descriptors, interrupts, packet I/O, networking, sockets, SSH, Phase
12.2, or a phase transition.

## Same-Shaped Retry Policy

Same-shaped GPIO32 PHY-reset write/restore hardware retries are closed for
this candidate/control pair. A future GPIO32 follow-up requires supervisor
planning for a qualitatively different discriminator, such as an explicit
event-state/source-clearance or ownership precondition task with its own
scope, acceptance gates, and evidence requirements. This closeout does not
choose that follow-up and does not authorize MDIO/PHY, interrupt,
DMA/descriptor, packet I/O, networking, socket, SSH, Phase 12.2, or
phase-transition work.

## Evidence

- Source v3 readiness closeout:
  tasks/2026-06-11-phase12-pi5-known-good-bounded-runtime-readiness-v3-closeout.md.
- V2 proof task record:
  tasks/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof.md.
- V2 proof classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof/classification.json.
- V2 proof evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof/evidence-map.json.
- V2 proof capture summary:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof/capture-summary.json.
- Closeout classification:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-proof-closeout/classification.json.
- Closeout evidence map:
  tasks/evidence/2026-06-10-phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-proof-closeout/evidence-map.json.

## Validation

- static inspection: v3 readiness closeout, v2 proof task record, v2 proof
  classification/evidence map, capture summary, project docs, and git history
  reviewed.
- JSON validation: jq empty on closeout classification/evidence-map JSON
  passed.
- diff check: git diff --check passed.
- docs build: /home/node/.cargo/bin/mdbook build passed.
- staged diff check: git diff --cached --check passed.

## Next Action

Supervisor planning is required before the next explicit Phase 12.1 slice. Do
not rerun the same GPIO32 write/restore proof without a qualitatively
different discriminator for the unexpected GPIO32 event state and exact
pre-write ownership conditions. No mechanically objective MDIO/PHY,
interrupt, DMA/descriptor, packet I/O, networking, socket, SSH, Phase 12.2, or
phase-transition follow-up is selected by this closeout.
