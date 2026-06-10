# Phase 12 Pi 5 Known-Good Bounded Runtime Readiness V2 Closeout

Task id: phase12-pi5-known-good-bounded-runtime-readiness-v2-closeout-20260610

## Goal

Close out the v2 known-good runtime-readiness proof and decide whether GPIO32
write/restore v2 is mechanically unlocked.

## Scope

- Consumed the accepted v2 runtime-readiness proof evidence from
  phase12-pi5-known-good-bounded-runtime-readiness-v2-pi5-proof-20260610.
- Reconciled restored-tree identity, immutable primary helper artifact, derived
  primary summary, TFTP delta, final status/files, hardware lock release, and
  retained risks.
- Did not run hardware, publish a boot archive, acquire hardwareTestLock, change
  code, or authorize GPIO32 write/restore v2.

## Evidence Summary

- Source proof commit: `30648dcb73b2b98de00ebc6b0395da1855617dff`.
- Run label: `known-good-runtime-readiness-v2-20260610T2332Z`.
- The proof retained the primary helper JSON at
  `tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-v2-pi5-proof/known-good-runtime-readiness-v2-20260610T2332Z-runtime-readiness-primary.json`.
- The derived summary is marked
  `derived_from_retained_primary_artifact=true` and `overwrite_policy=refuse-existing-primary-summary-or-status`.
- Pre-power and final boot identity matched known-good tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`
  with `kernel_2712.img`.
- Stable TFTP evidence had 13 events and included two expected
  `da591740/kernel_2712.img` fetches at 104136 bytes.
- The retained primary helper used
  `deadline-loop-direct-read-after-saturated-cursor`, read 6847 bytes, and
  observed `rpi5-production-timer-preemption: PASS`.
- The retained primary helper did not observe `TALOS: kernel_main`; therefore
  `valid_known_good_talos_readiness=false` and the helper classification is
  `known-good-fetch-observed-without-talos-readiness`.

## Findings

- fixed: the v2 proof used the repaired primary-artifact retention path; the
  primary helper JSON was retained under an immutable run-label-qualified name.
- fixed: derived summary and classification evidence were derived from the
  retained primary artifact rather than a mutable/latest helper path.
- fixed: stable known-good boot identity and TFTP fetch evidence were retained.
- blocked: valid known-good Talos readiness remains unaccepted because the
  repaired helper did not classify the retained primary artifact as ready.
- deferred: GPIO32 write/restore v2 remains blocked; its valid-known-good
  readiness dependency is still unsatisfied.
- not-an-issue: no additional hardware run or docs/src source change was
  required by this static closeout beyond recording the accepted frontier.

## Classification

`known-good-fetch-pass-marker-observed-helper-readiness-unaccepted-closeout`

This closeout accepts the repaired primary-artifact retention contract and the
v2 proof's stable known-good identity/TFTP evidence. It does not accept
`valid-known-good-talos-readiness`, because the retained primary helper
artifact did not satisfy the helper readiness contract.

GPIO32 write/restore v2 is not mechanically unlocked. Same-shaped GPIO32 or
known-good runtime-readiness hardware retries remain held until the supervisor
plans a changed discriminator or accepts a different readiness contract.

Rejected claims:

- valid known-good Talos readiness
- GPIO32 write/restore v2 authorization
- PHY reset assertion/deassertion
- MDIO/PHY ownership
- Ethernet driver behavior
- DMA/descriptors
- interrupts
- packet I/O
- networking, sockets, SSH
- Phase 12.2 or phase transition

## Validation

- static inspection of v2 proof record, classification/evidence JSON, helper
  output, TFTP delta, final status/files, hardware lock release, and git
  history: completed
- `jq empty` on task-owned evidence JSON: passed
- `git diff --check`: passed
- `/home/node/.cargo/bin/mdbook build`: passed
- `git diff --cached --check`: passed

## Evidence

- `tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-v2-closeout/classification.json`
- `tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-v2-closeout/evidence-map.json`
- Source proof record:
  `tasks/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-v2-pi5-proof.md`
- Source proof classification:
  `tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-v2-pi5-proof/classification.json`
- Retained primary helper summary:
  `tasks/evidence/2026-06-10-phase12-pi5-known-good-bounded-runtime-readiness-v2-pi5-proof/known-good-runtime-readiness-v2-20260610T2332Z-runtime-readiness-primary-summary.json`

Next action: supervisor planning required. Do not promote
phase12-rp1-ethernet-gpio32-phy-reset-write-restore-v2-pi5-proof-20260610,
and do not rerun same-shaped GPIO32 or known-good readiness hardware proofs
without a changed discriminator or changed readiness contract.
