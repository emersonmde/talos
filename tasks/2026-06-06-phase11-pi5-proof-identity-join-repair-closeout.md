# Phase 11 Pi 5 Proof Identity Join Repair Closeout

Task id: phase11-pi5-proof-identity-join-repair-closeout-20260606

Status: accepted

## Goal

Reconcile the identity-join repair and known-good control into an accepted
proof-chain boundary before deciding whether the RP1 UART0 FR-read
hold-control candidate can be rerun.

## Scope

- Reviewed the accepted repair-core task and evidence from commit
  `4c3e0cfe836a679443983ffbc0f2dbab35f0bc5a`.
- Reviewed the accepted known-good control task and evidence from commit
  `5edb8fa141f18d3e44cd3c2e1dd07396eeb4b298`.
- Reconciled the repaired `pi5-proof-identity-join-v1` contract with the
  known-good hardware control's serial, TFTP, lab-status, restore, and checker
  output.
- Updated the Phase 11 RP1/PCIe map contract and roadmap with the accepted
  proof-chain boundary.
- Did not run hardware, publish a boot archive, acquire hardwareTestLock,
  change kernel/RP1 source, execute an RP1 UART0 FR read, or advance Milestone
  11.2.

## Classification

proof-chain-ready-for-candidate-rerun.

The accepted repair core introduced `pi5-proof-identity-join-v1`, requiring one
run label to tie selected tree hash, effective kernel, expected fetch path and
byte count, serial cursor/window identity, stable TFTP cursor/delta identity,
final pre-restore identity, and restore identity. It also replayed the old
hold-control candidate run and kept that run classified as
`capture-staging-blocked` because the TFTP and final pre-restore identities
matched the restored known-good tree, not the selected 46,320-byte candidate.

The accepted known-good control then proved the repaired contract on hardware.
It selected tree
`a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`, effective
`kernel_2712.img`, and expected `da591740/kernel_2712.img` byte count 104,136.
The stable pre-restore TFTP delta retained 13 events, including two expected
104,136-byte fetches. The fresh serial window used
`deadline-loop-direct-read-after-saturated-cursor`, retained 7,070 bytes, and
contained `rpi5-production-timer-preemption: PASS`. The identity-join checker
reported `decisive_rp1_hardware_classification_allowed=true`,
`classification=proof-chain-ready-for-candidate-rerun`, and no rejection
reasons. The run restored the pre-run tree before hardware-lock release.

Accepted claims are limited to proof-chain readiness for a later candidate
rerun that passes the same `pi5-proof-identity-join-v1` gate. This closeout
does not accept RP1 UART0 FR mapped/read-value, bus-fault/trap, unmapped/trap,
firmware-state behavior, candidate behavior, GPIO, interrupts, DMA/cache,
storage, generated-root, networking, SSH, broader PCIe, Milestone 11.2, or a
phase transition.

## Findings And Disposition

- fixed: the proof-chain boundary is now explicit: a later RP1 candidate run
  must satisfy `pi5-proof-identity-join-v1` before any decisive RP1 hardware
  classification is accepted.
- fixed: repair-core evidence is reconciled with known-good control evidence;
  the old hold-control candidate run remains `capture-staging-blocked`, while
  the known-good run is accepted as proof-chain-ready.
- fixed: known-good hardware evidence ties lab status, selected tree,
  expected fetch, serial window, stable pre-restore TFTP, final pre-restore
  identity, restore, and post-restore identity into one retained proof bundle.
- fixed: docs/src/project/phase11-rp1-pcie-map-contract.md records that the
  next RP1 UART0 FR-read hold-control candidate proof is unblocked only under
  the repaired contract.
- fixed: docs/src/roadmap.md records the accepted Phase 11 proof-chain
  frontier without accepting RP1 mapped/read-value or trap behavior.
- removed: known-good proof-chain readiness is not treated as RP1 candidate
  behavior.
- not-an-issue: the known-good serial window omitted `TALOS: kernel_main`, but
  retained `rpi5-production-timer-preemption: PASS`; lab-controller proof
  rules accept that downstream marker for this restored production-timer
  control.
- deferred: the actual RP1 UART0 FR-read candidate classification remains for
  the separately queued serialized Pi 5 task, gated by this closeout and the
  hardware lock.

## Evidence

- Static evidence inspection:
  `tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-repair-closeout/static-evidence-inspection.md`.
- Evidence map:
  `tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-repair-closeout/evidence-map.json`.
- Repair-core task:
  `tasks/2026-06-06-phase11-pi5-proof-identity-join-repair-core.md`.
- Repair-core evidence:
  `tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-repair-core/evidence-map.json`.
- Known-good control task:
  `tasks/2026-06-06-phase11-pi5-proof-identity-join-known-good-control.md`.
- Known-good control evidence:
  `tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-known-good-control/evidence-map.json`.
- Known-good identity-join checker:
  `tasks/evidence/2026-06-06-phase11-pi5-proof-identity-join-known-good-control/known-good-run-identity-join-check.json`.

## Validation

- static evidence inspection of repair-core and known-good control records:
  passed.
- git diff --check: passed.
- /home/node/.cargo/bin/mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as proof-chain-ready-for-candidate-rerun.

The next queued RP1 UART0 FR-read hold-control candidate proof may be promoted
only if hardwareTestLock remains unlocked/restored and it retains a full
candidate proof bundle that passes `pi5-proof-identity-join-v1`. Same-shaped
runs that fail the repaired join contract must classify as
`capture-staging-blocked`, not as mapped/read-value or trap/unmapped behavior.
