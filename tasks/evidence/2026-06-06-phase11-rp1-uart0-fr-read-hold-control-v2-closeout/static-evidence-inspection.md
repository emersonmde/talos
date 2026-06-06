# Static Evidence Inspection

Task id: phase11-rp1-uart0-fr-read-hold-control-v2-closeout-20260606

## Inspected Inputs

- tasks/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5.md
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5/classification.json
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5/evidence-map.json
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5/candidate-rerun-identity-join-check.json
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5/known-good-control-identity-join-check.json
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5/candidate-rerun-summary.json
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5/known-good-control-summary.json
- tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-hold-control-v2-proof-pi5/triage-and-rerun-analysis.json

## Findings

- fixed: the accepted proof classification is
  candidate-fetch-without-control-marker, not mapped/read-value,
  bus-fault/trap, pre-read-control-visible-without-read-result, or
  capture-staging-blocked.
- fixed: candidate rerun identity join passed pi5-capture-transaction-v2 with
  selected tree ae324bd791d7df59a0a8eabc74c936b1bd68ba6c2c9b645dcdc19f561e4e80c0,
  effective kernel_2712.img, expected 46,320-byte
  da591740/kernel_2712.img fetches, empty pre-power serial drain, stable TFTP,
  final pre-restore selected-tree identity, and restore to
  a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10.
- fixed: known-good control identity join passed the same v2 contract with
  restored tree a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10,
  expected 104,136-byte fetches, empty pre-power serial drain, stable TFTP,
  final pre-restore identity, restore proof, and PASS serial output.
- fixed: the candidate rerun serial window retained 1,064,457 bytes and
  27,177 TALOS: fr-hold-control-post-read-loop occurrences.
- removed: the post-read-loop tail alone is not promoted to RP1 UART0 FR
  mapped/read-value, bus-fault/trap, or pre-read-control-visible-without-read-result
  because the contracted read-value/classification line, pre-read control
  marker, post-read terminal marker, and trap/panic text were absent.
- not-an-issue: the first candidate run remains rejected because v2 required
  an empty pre-power /serial/read drain before trusting the saturated
  direct-read serial window.
- deferred: the next discriminator must explain the selected-candidate
  post-read-loop tail without repeating the same FR-read hardware run shape.

## Conclusion

The closeout accepts candidate-fetch-without-control-marker only. The v2 proof
contract is functional, and the selected RP1 candidate reached enough serial
tail output to require a new discriminator, but current evidence does not prove
RP1 UART0 FR register semantics.
