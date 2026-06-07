# Static Evidence Inspection

Task id: phase11-rp1-uart0-fr-tail-stable-control-closeout-20260606

## Inspected Inputs

- tasks/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core.md
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core/evidence-map.json
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-result-core/static-inspection.md
- tasks/2026-06-06-phase11-rp1-uart0-fr-tail-stable-no-mmio-control-pi5.md
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-no-mmio-control-pi5/classification.json
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-no-mmio-control-pi5/evidence-map.json
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-no-mmio-control-pi5/control-rerun-after-kg/identity-join-check.json
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-tail-stable-no-mmio-control-pi5/control-rerun-after-kg/capture-invariant-summary.json

## Findings

- fixed: the accepted no-MMIO control classification is
  tail-stable-control-visible, not
  candidate-fetch-without-tail-stable-control, capture-staging-blocked, or
  restore-blocked.
- fixed: the accepted source/static core keeps the RP1 result proof and the
  no-MMIO control paired by output shape while preserving the one-load/no-load
  boundary.
- fixed: the decisive control rerun identity join passed
  pi5-capture-transaction-v2 with selected tree
  b4b780193281538a643aec3c17898ae59204c335f32452b90cf08b0cb8e10161,
  effective kernel_2712.img, expected 45,728-byte
  da591740/kernel_2712.img fetches, empty pre-power serial drain, stable TFTP,
  final pre-restore selected-tree identity, and restore to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: the accepted serial window retained 1,771 occurrences of
  TALOS: fr-tail-stable-control from the selected no-MMIO control candidate.
- removed: the visible simulated/control marker is not promoted to RP1 UART0
  FR mapped/read-value, bus-fault/trap, or firmware-state evidence because the
  candidate intentionally performed no RP1 MMIO.
- not-an-issue: the earlier inconclusive control attempts remain retained as
  triage evidence, but they do not weaken the later accepted v2 identity-joined
  control rerun.
- deferred: the queued RP1 tail-stable result proof must independently prove
  mapped-read-value-tail-stable, at-or-after-rp1-load-no-tail-result,
  bus-fault-or-trap-visible, candidate-fetch-without-rp1-tail-result,
  capture-staging-blocked, or restore-blocked.

## Conclusion

The closeout accepts tail-stable-control-visible only. The no-MMIO
result-output shape is capturable on Pi 5, so the queued RP1 tail-stable result
proof is mechanically unblocked under hardware lock and supervisor intervention
rules. RP1 UART0 FR register semantics remain unaccepted.
