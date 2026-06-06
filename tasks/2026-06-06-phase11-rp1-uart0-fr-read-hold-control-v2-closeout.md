# Phase 11 RP1 UART0 FR Read Hold-Control V2 Closeout

Task id: phase11-rp1-uart0-fr-read-hold-control-v2-closeout-20260606

Status: accepted

## Goal

Reconcile the RP1 UART0 FR-read hold-control v2 proof result and record the
exact accepted hardware boundary before any further Phase 11 work.

## Scope

- Inspected the accepted v2 proof task record and evidence.
- Reconciled the v2 identity-joined candidate rerun, known-good control, TFTP,
  serial, final pre-restore identity, and restore evidence.
- Updated the Phase 11 RP1/PCIe contract and roadmap boundary.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition, source
runtime changes, new RP1 constants, GPIO, interrupts, DMA/cache, storage,
generated-root, networking, SSH, broader PCIe, Milestone 11.2, or phase
transition.

## Classification

candidate-fetch-without-control-marker.

The accepted v2 proof committed at
2622424972e8fbf7c3a0749ddb4c7458d3ec68c5 staged candidate tree
ae324bd791d7df59a0a8eabc74c936b1bd68ba6c2c9b645dcdc19f561e4e80c0 with
effective kernel_2712.img and expected 46,320-byte
da591740/kernel_2712.img fetches. The decisive candidate rerun passed the
pi5-capture-transaction-v2 identity join with an empty pre-power /serial/read
drain, stable same-cursor TFTP, final selected-tree identity, restore proof,
and 27,177 occurrences of TALOS: fr-hold-control-post-read-loop in the serial
window.

That evidence does not retain the contracted
rpi5-rp1-uart0-fr-read read-value/classification line, the pre-read control
marker, the post-read terminal marker, or trap/panic text. The post-read-loop
tail is therefore accepted only as candidate-tied serial reachability after
publication/fetch, not as RP1 UART0 FR mapped/read-value, bus-fault/trap, or
pre-read-control-visible-without-read-result evidence.

Accepted claims are limited to selected candidate publication/fetch, v2
candidate identity join, visible post-read-loop-tail output without contracted
control/read-result markers, known-good proof-chain control, restore hygiene,
and the candidate-fetch-without-control-marker classification. RP1 UART0 FR
mapped/read-value behavior, bus-fault/trap behavior,
pre-read-control-visible-without-read-result, firmware-state behavior, GPIO,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, and phase transition remain unaccepted.

Because the v2 contract is no longer the blocker, another same-shaped RP1
FR-read hardware rerun is not useful by itself. Supervisor planning is required
for a qualitatively different discriminator that explains why the selected
candidate produces the post-read-loop tail without retaining the contracted
control/read-result/trap markers.

## Findings And Disposition

- fixed: reconciled the accepted v2 proof into the Phase 11 contract and
  roadmap boundary.
- fixed: kept the first candidate run as rejected evidence because its
  non-empty pre-power drain prevents decisive v2 classification.
- fixed: retained the known-good control as proof that the v2 capture contract
  itself was healthy before the candidate rerun.
- fixed: accepted only the candidate rerun evidence that passed v2 identity
  join with selected candidate TFTP/final identity and restore proof.
- deferred: mapped/read-value or bus-fault/trap requires a new discriminator
  that retains the contracted read-value/classification line or trap evidence.
- removed: no RP1 mapped/read-value, bus-fault/trap, or pre-read-control claim
  is inferred from the post-read-loop tail alone.
- not-an-issue: the 27,177 post-read-loop occurrences remain useful risk
  evidence, but they are not sufficient acceptance evidence for RP1 register
  semantics.

## Evidence

- Static evidence inspection:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-v2-closeout/static-evidence-inspection.md.
- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-v2-closeout/evidence-map.json.
- Source proof task:
  tasks/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5.md.
- Source proof evidence:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5/evidence-map.json.

## Validation

- static inspection of v2 proof task record/evidence: passed.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as candidate-fetch-without-control-marker. No queued task remains
mechanically unblocked after this closeout; supervisor planning is required for
the next non-repetitive discriminator.
