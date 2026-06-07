# Phase 11 RP1 UART0 FR Tail-Stable Result Closeout

Task id: phase11-rp1-uart0-fr-tail-stable-result-closeout-20260606

Status: accepted

## Goal

Close out the tail-stable RP1 UART0 FR-read discriminator and define the next
Phase 11 boundary without broadening scope.

## Scope

- Inspected the accepted tail-stable source/static core evidence.
- Inspected the accepted no-MMIO tail-stable Pi 5 control evidence and
  closeout.
- Inspected the accepted RP1 tail-stable Pi 5 proof task record and evidence.
- Reconciled accepted and unaccepted claims into the Phase 11 RP1/PCIe map
  contract and roadmap.

## Non-Goals

No hardware run, boot archive publication, hardwareTestLock acquisition, RP1
source change, RP1 constants change, GPIO, pin-control, clocks/resets,
interrupts, DMA/cache, storage, generated-root, networking, SSH, broader PCIe,
Milestone 11.2, or phase transition. This closeout does not authorize broader
RP1 ownership from the narrow read-only UART0 FR diagnostic.

## Classification

mapped-read-value-tail-stable.

The accepted source/static core committed at
c3700b21166100468e2131d35c221d88d1f1612e created a paired discriminator: the
RP1 candidate performs exactly one contracted 32-bit volatile load from
0x1f00030018 and, only if the load returns, repeatedly emits the compact
tail-stable read-result/classification marker; the no-MMIO control performs no
RP1 address construction or volatile load and repeats the same output shape as
simulated/control.

The no-MMIO control proof and closeout committed at
b5878af296576c6b930426b2b6db208eaeec515c and
9b2d46059c0ebe99327a189da1894b2f11d0e9e9 accepted the output shape as
tail-stable-control-visible on Pi 5. That proved the marker-retention and v2
capture path before the RP1 MMIO proof.

The RP1 tail-stable Pi 5 proof committed at
427acda2f1b6cf95a0e56a01196596e5be9cd97d accepted
mapped-read-value-tail-stable. The decisive candidate rerun selected tree
0e187f9f73118c237337b25d85e57c51dbf18a18bf87ab0d3850c63291b153eb with
effective kernel_2712.img and a 45,800-byte da591740/kernel_2712.img. The v2
identity join passed with an empty pre-power /serial/read drain, stable
same-cursor TFTP retained two selected-candidate fetches, final pre-restore
identity still matched the selected tree, and restore returned the lab to
a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.

The decisive serial window retained 1,498 occurrences of:

    TALOS: fr-tail-stable-result contract=phase11-rp1-pcie-map-contract-v1 target=rp1-uart0-fr-read address=0x1f00030018 width=32 raw=0xdeaddead classification=mapped/read-value

Accepted claims are limited to the first read-only RP1 UART0 FR
mapped/read-value diagnostic boundary for the contracted single load at
0x1f00030018, plus the proof-chain evidence needed to tie that output to the
selected candidate. GPIO/pin-control ownership, RP1 clocks/resets, interrupts,
DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
11.2, and phase transition remain unaccepted.

No explicit queued task remains after this closeout. Supervisor planning is
required for the next bounded Phase 11 slice.

## Findings And Disposition

- fixed: reconciled the accepted RP1 tail-stable proof into the Phase 11
  contract and roadmap boundary.
- fixed: confirmed the source/static core preserved the one-load RP1 candidate
  and zero-load no-MMIO control separation before hardware proof.
- fixed: confirmed the no-MMIO control passed first, so the accepted
  tail-stable result output is not explained by marker-retention or capture
  loss.
- fixed: accepted only the decisive candidate rerun with v2 identity join,
  selected-candidate TFTP/final identity, repeated tail-stable result markers,
  and restore proof.
- removed: earlier inconclusive candidate evidence is not promoted to an RP1
  behavior claim because it failed the v2 pre-power/final-identity gates.
- deferred: GPIO/pin-control ownership, RP1 clocks/resets, interrupts,
  DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
  11.2, and any phase transition require supervisor-planned follow-up tasks.
- not-an-issue: the raw 0xdeaddead value is accepted only as the retained raw
  value from the contracted RP1 UART0 FR read-result path, not as evidence for
  unrelated RP1 subsystems.

## Evidence

- Static evidence inspection:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-closeout/static-evidence-inspection.md.
- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-closeout/evidence-map.json.
- Source/static core evidence:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core/evidence-map.json.
- No-MMIO control closeout evidence:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-control-closeout/evidence-map.json.
- RP1 tail-stable Pi 5 proof evidence:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-pi5/evidence-map.json.

## Validation

- static inspection of RP1 proof task record/evidence: passed.
- git diff --check: passed.
- mdbook build: passed.
- git diff --cached --check before commit: passed.

## Result

Accepted as mapped-read-value-tail-stable. No mechanically unblocked queued
task remains; supervisor planning is required before any next Phase 11 task.
