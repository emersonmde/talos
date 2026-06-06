# Phase 11 Pi 5 Proof Identity Join Known-Good Control

Task id: phase11-pi5-proof-identity-join-known-good-control-20260606

Status: accepted

## Goal

Validate the repaired proof identity-join contract on the restored known-good
Pi 5 boot tree before using it for another RP1 candidate proof.

## Scope

- Acquired hardwareTestLock for one serialized known-good Pi 5 run.
- Ran the restored known-good boot tree without publishing an RP1 candidate
  archive.
- Captured selected boot identity, fresh serial cursor/window identity, stable
  same-cursor TFTP delta, final pre-restore identity, restore evidence, and
  post-restore identity through the repaired proof bundle.
- Replayed the proof bundle through the `pi5-proof-identity-join-v1` checker.
- Restored the pre-run boot tree before hardware-lock release.

## Classification

proof-chain-known-good-ready.

The known-good control selected tree
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10` with
effective `kernel_2712.img` and expected
`da591740/kernel_2712.img` byte count 104,136. The stable pre-restore TFTP
delta retained 13 events, including two served 104,136-byte fetches for the
expected known-good kernel. The fresh serial window used the repaired
`deadline-loop-direct-read-after-saturated-cursor` path from saturated cursor
4,194,304 and retained 7,070 bytes, including
`rpi5-production-timer-preemption: PASS`.

The identity-join checker accepted the bundle with
`decisive_rp1_hardware_classification_allowed=true`,
`classification=proof-chain-ready-for-candidate-rerun`, and no rejection
reasons. This task accepts only known-good proof-chain readiness for a later
candidate rerun. It does not accept RP1 UART0 FR mapped/read-value,
bus-fault/trap, firmware-state behavior, candidate behavior, GPIO, interrupts,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
11.2, or a phase transition.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around the serialized control
  run.
- fixed: retained preflight lab status and boot-file identity for the selected
  restored known-good tree.
- fixed: retained a fresh serial proof window using the repaired direct-read
  path after saturated cursor detection.
- fixed: retained stable same-cursor TFTP evidence before restore, including
  two expected 104,136-byte known-good kernel fetches.
- fixed: restored the pre-run boot tree with snapshot
  `phase11-pi5-proof-identity-join-known-good-pre-20260606T210203Z` before
  hardware-lock release.
- fixed: replayed the retained proof bundle through
  `scripts/rpi5-proof-identity-join-check.sh` and accepted no rejection
  reasons.
- not-an-issue: `TALOS: kernel_main` was absent from the retained serial
  window, but the accepted downstream
  `rpi5-production-timer-preemption: PASS` marker is sufficient for this
  restored production-timer known-good control per the lab-controller proof
  contract.
- removed: no RP1 candidate behavior or FR-read result is inferred from this
  known-good control.

## Evidence

- Classification:
  `tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-known-good-control/classification.json`.
- Evidence map:
  `tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-known-good-control/evidence-map.json`.
- Full repaired proof bundle:
  `tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-known-good-control/known-good-run/`.
- Identity-join checker output:
  `tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-known-good-control/known-good-run-identity-join-check.json`.
- Identity-join checker exit:
  `tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-known-good-control/known-good-run-identity-join-check.exit`.

## Validation

- serialized Pi 5 hardware run through lab-controller endpoints under
  hardwareTestLock: passed.
- lab API status and boot-file identity preflight and post-restore records:
  passed.
- fresh serial capture through repaired proof identity-join contract: passed.
- stable same-cursor TFTP evidence before restore: passed.
- restore proof before hardware-lock release: passed.
- identity-join checker replay: passed.
- git diff --check: passed.
- mdbook build: not run because no `docs/src` files were touched.
- git diff --cached --check before commit: passed.

## Result

Accepted as proof-chain-known-good-ready. The next queued closeout can reconcile
this proof-chain readiness before any RP1 candidate hardware proof is promoted.
