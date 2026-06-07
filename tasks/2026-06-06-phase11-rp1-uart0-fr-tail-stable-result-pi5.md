# Phase 11 RP1 UART0 FR Tail-Stable Result Pi 5

Task id: phase11-rp1-uart0-fr-tail-stable-result-pi5-20260606

Status: accepted

## Goal

Run the tail-stable RP1 UART0 FR-read discriminator on Pi 5 and classify
returned-read evidence without relying on one-shot markers.

## Scope

- Acquired hardwareTestLock for the serialized RP1 tail-stable result proof.
- Published only the accepted RP1 tail-stable result archive:
  target/talos-rpi5-rp1-uart0-fr-tail-stable-result-core.tar.gz.
- Retained static archive identity, publication identity, empty pre-power
  serial drain, v2 capture-transaction identity join, stable pre-restore TFTP
  evidence, final pre-restore identity, and restore proof.
- Retained the first inconclusive candidate run and the required known-good
  control before the accepted candidate rerun.

## Classification

mapped-read-value-tail-stable.

The decisive candidate rerun selected boot tree
`0e187f9f73118c237337b25d85e57c51dbf18a18bf87ab0d3850c63291b153eb` with
effective `kernel_2712.img` and a 45,800-byte
`da591740/kernel_2712.img`. The v2 identity join passed with no rejection
reasons: the pre-power `/serial/read` drain was empty, stable pre-restore
TFTP retained two served 45,800-byte selected-candidate fetches, final
pre-restore identity still matched the selected tree, and restore returned the
lab to `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.

The saturated direct-read serial window retained 1,498 occurrences of:

```text
TALOS: fr-tail-stable-result contract=phase11-rp1-pcie-map-contract-v1 target=rp1-uart0-fr-read address=0x1f00030018 width=32 raw=0xdeaddead classification=mapped/read-value
```

This accepts the narrow RP1 UART0 FR read-result boundary for the contracted
single 32-bit volatile load at `0x1f00030018`. It does not accept GPIO or
pin-control ownership, RP1 clocks/resets, interrupts, DMA/cache behavior,
storage, generated-root, networking, SSH, broader PCIe enumeration, Milestone
11.2, or a phase transition.

## Findings And Disposition

- fixed: acquired and released hardwareTestLock around the serialized RP1
  tail-stable hardware proof.
- fixed: retained static archive identity for the accepted RP1 result archive,
  including archive SHA-256, selected kernel size, and review output.
- fixed: retained the first candidate run as capture-staging-blocked triage
  evidence because its pre-power serial drain was non-empty and final
  pre-restore identity no longer matched the selected tree.
- fixed: ran the required known-good control after the inconclusive first
  capture; it passed the v2 identity join and retained the
  `rpi5-production-timer-preemption: PASS` marker.
- fixed: reran the RP1 candidate after the known-good control; the rerun passed
  v2 identity join and retained the repeated tail-stable read-result marker.
- deferred: broader Phase 11 RP1 ownership remains outside this first
  read-only diagnostic.
- not-an-issue: the accepted raw value is a hardware proof of the contracted
  mapped/read-value boundary only, not approval to expand into GPIO,
  interrupts, DMA/cache, storage, networking, SSH, broader PCIe, Milestone
  11.2, or a phase transition.

## Evidence

- Evidence map:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-pi5/evidence-map.json.
- Classification:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-pi5/classification.json.
- Decisive candidate rerun:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-pi5/result-rerun-after-kg/.
- Known-good control:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-pi5/known-good-control-run/.
- Inconclusive first candidate run retained as triage evidence:
  tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-pi5/result-run/.

## Validation

- static archive identity check: passed against the accepted core evidence.
- lab-controller serialized Pi 5 hardware run: passed on the decisive
  candidate rerun.
- pi5-capture-transaction-v2 identity join: passed on the decisive candidate
  rerun with no rejection reasons.
- stable same-cursor TFTP evidence before restore: passed; two 45,800-byte
  selected-candidate kernel fetches were retained.
- serial hardware boot/output: passed; 1,498 occurrences of
  `TALOS: fr-tail-stable-result` were retained with raw `0xdeaddead` and
  `classification=mapped/read-value`.
- known-good control and candidate rerun after inconclusive evidence: run and
  retained.
- restore proof before hardware-lock release: passed.
- git diff --check: passed.
- mdbook build: passed because docs/src files changed.
- git diff --cached --check before commit: passed.

## Result

Accepted as mapped-read-value-tail-stable. The queued
phase11-rp1-uart0-fr-tail-stable-result-closeout-20260606 closeout is
mechanically unblocked on a future worker wake if hardwareTestLock remains
unlocked/restored.
