# Phase 12 RP1 Ethernet GEM MID Pi 5 Proof

Task: phase12-rp1-ethernet-gem-mid-pi5-proof-20260609

## Scope

Run the serialized Pi 5 visibility/control proof selected by
phase12-rp1-ethernet-gem-mid-diagnostic-closeout-20260609. The proof is
limited to a candidate read-only volatile load of `MACB_MID` at CPU physical
`0x1f001000fc` and a paired no-Ethernet/no-MMIO control report. It does not
implement an Ethernet driver, packet I/O, DMA, descriptor rings, interrupts,
clock/reset writes, PHY reset, networking, sockets, SSH, Phase 12.2 work, or a
phase transition.

## Findings

- Fixed: added bounded boot scenarios and archive/review scripts for
  `rpi5_rp1_ethernet_gem_mid_visibility_candidate` and
  `rpi5_rp1_ethernet_gem_mid_visibility_no_mmio_control`.
- Fixed: initial candidate/control captures with identity-join mismatches were
  rejected, then triaged with the required order: candidate identity, fresh
  serial cursor, TFTP delta, restore/control, and reruns with a fresh
  discriminator and drained serial path.
- Fixed: the control rerun proved the reporting path without constructing an
  Ethernet MMIO target. It used tree
  `dcac966fee3fd21eba83cc449bea85ef1490bc249c825e79d867c6b27095d93f`, fetched
  `da591740/kernel_2712.img` twice at 47808 bytes, emitted 80 control markers,
  and restored snapshot `pre-gem-mid-proof-20260610T0038Z`.
- Deferred: the decisive candidate rerun reached the candidate report path but
  read `raw=0xdeaddead` from MACB_MID at `0x1f001000fc`. This is classified as
  `rp1-ethernet-gem-mid-blocked-address-decode-sentinel`, not live GEM
  visibility or broad Ethernet MMIO readiness.
- Not-an-issue: the proof keeps Ethernet driver readiness, broad MMIO
  readiness, RP1 MMIO/DMA programming, descriptor rings, transfer completion,
  interrupt completion, clock/reset ownership, PHY reset ownership, packet I/O,
  networking, sockets, SSH, Phase 12.2, and phase transition claims rejected.

## Evidence

- Archive review:
  `tasks/evidence/2026-06-09-phase12-rp1-ethernet-gem-mid-pi5-proof/archive-review/`.
- Candidate rerun2:
  `tasks/evidence/2026-06-09-phase12-rp1-ethernet-gem-mid-pi5-proof/candidate-rerun2/`.
- Control rerun:
  `tasks/evidence/2026-06-09-phase12-rp1-ethernet-gem-mid-pi5-proof/control-rerun/`.
- Final restore status:
  `tasks/evidence/2026-06-09-phase12-rp1-ethernet-gem-mid-pi5-proof/final-lab-status-before-lock-release.json`.
- Classification:
  `tasks/evidence/2026-06-09-phase12-rp1-ethernet-gem-mid-pi5-proof/classification.json`.
- Evidence map:
  `tasks/evidence/2026-06-09-phase12-rp1-ethernet-gem-mid-pi5-proof/evidence-map.json`.

## Validation

- Pre-hardware static inspection of accepted closeout and candidate/control
  archive review: passed.
- Serialized hardwareTestLock: acquired before publication and released in
  state after evidence capture.
- Lab API identity: candidate and control reruns both recorded matching staged
  tree hashes, `kernel_2712.img`, expected fetch paths, TFTP deltas, serial
  windows, and restore proof.
- `jq empty` on task-owned evidence-map/classification JSON: passed.
- `git diff --check`: passed.
- `/home/node/.cargo/bin/mdbook build`: passed because docs were touched.
- `git diff --cached --check`: passed before commit.

## Classification

Accepted as a precise blocker:
`rp1-ethernet-gem-mid-blocked-address-decode-sentinel`.

The no-Ethernet/no-MMIO control is visible and proves the reporting path. The
candidate reaches the bounded read path and reports `raw=0xdeaddead`, so the
next step requires supervisor planning for the bridge/translation/enablement
dependency before any Ethernet implementation or Phase 12.2 work.
