# Static Evidence Inspection

Task id: phase11-rp1-uart0-fr-tail-stable-result-closeout-20260606

## Inspected Inputs

- tasks/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core.md
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core/evidence-map.json
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core/static-inspection.md
- tasks/2026-06-06-phase11-rp1-uart0-fr-tail-stable-control-closeout.md
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-control-closeout/evidence-map.json
- tasks/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-pi5.md
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-pi5/classification.json
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-pi5/evidence-map.json
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-pi5/result-rerun-after-kg/identity-join-check.json
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-pi5/result-rerun-after-kg/capture-invariant-summary.json

## Findings

- fixed: the accepted RP1 Pi 5 proof classification is
  mapped-read-value-tail-stable, not at-or-after-rp1-load-no-tail-result,
  bus-fault-or-trap-visible, candidate-fetch-without-rp1-tail-result,
  capture-staging-blocked, or restore-blocked.
- fixed: the accepted source/static core proves the RP1 tail-stable candidate
  has exactly one contracted volatile 32-bit load from 0x1f00030018 and the
  no-MMIO control has zero RP1 MMIO loads.
- fixed: the accepted no-MMIO control closeout proves the tail-stable
  result-output shape is retained on Pi 5 before the RP1 MMIO proof.
- fixed: the decisive RP1 candidate rerun passed pi5-capture-transaction-v2
  with selected tree
  0e187f9f73118c237337b25d85e57c51dbf18a18bf87ab0d3850c63291b153eb,
  effective kernel_2712.img, expected 45,800-byte
  da591740/kernel_2712.img fetches, empty pre-power serial drain, stable TFTP,
  final pre-restore selected-tree identity, and restore to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: the accepted serial window retained 1,498 occurrences of
  TALOS: fr-tail-stable-result with contract
  phase11-rp1-pcie-map-contract-v1, target rp1-uart0-fr-read, address
  0x1f00030018, width 32, raw 0xdeaddead, and classification
  mapped/read-value.
- removed: the first inconclusive candidate run is retained only as triage
  evidence because it failed the v2 pre-power/final-identity gates.
- deferred: GPIO/pin-control ownership, RP1 clocks/resets, interrupts,
  DMA/cache, storage, generated-root, networking, SSH, broader PCIe, Milestone
  11.2, and phase transition remain outside this closeout.
- not-an-issue: accepting the raw value from the contracted FR read-result path
  does not require accepting any unrelated RP1 subsystem behavior.

## Conclusion

The closeout accepts mapped-read-value-tail-stable for the narrow read-only
RP1 UART0 FR diagnostic boundary. No explicit queued task remains after this
closeout; supervisor planning is required before any next Phase 11 task.
