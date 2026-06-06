# Static Evidence Inspection

Task id: phase11-rp1-uart0-fr-read-closeout-20260606

Inspection level: static inspection of accepted task records and retained
hardware-proof evidence.

## Inputs

- `tasks/2026-06-06-phase11-rp1-uart0-fr-read-refresh-core.md`
- `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-refresh-core/evidence-map.json`
- `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-refresh-core/static-inspection.md`
- `tasks/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof.md`
- `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof/evidence-map.json`
- `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof/classification.json`
- `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof/candidate-run/capture-invariant-summary.json`
- `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof/candidate-run/serial-observe-window.json`
- `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof/known-good-control-run/run-summary.json`
- `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof/candidate-rerun/run-summary.json`
- `tasks/evidence/2026-06-06-phase11-rp1-uart0-fr-read-pi5-proof/validation-summary.txt`

## Observations

- Refresh-core task status is accepted and committed. Its retained static
  evidence shows the diagnostic is entered from `rust_entry`, uses the UART10
  early-serial helper, and performs one 32-bit volatile read from
  `RP1_UART0_FR` at `0x1f_0003_0018` when execution reaches the read.
- The first Pi 5 candidate run selected tree
  `25ff74c5c496e861d534080a6e8ec65cb36d261f16775515cd37a79938d41b71` with
  effective kernel `kernel_2712.img` and expected
  `da591740/kernel_2712.img` size 45,832 bytes.
- Stable pre-restore TFTP evidence for the first candidate run retained 13
  events, including two served 45,832-byte candidate kernel fetches.
- The first candidate run serial window started and ended at cursor `4194304`,
  returned zero bytes, and contained none of the FR-read start, pre-MMIO,
  mapped/read-value, or PASS markers.
- The known-good control retained stable TFTP evidence with two served
  104,136-byte control kernel fetches, but serial again started and ended at
  cursor `4194304` with zero bytes and no `TALOS: kernel_main`.
- The candidate rerun retained serial cursor `4194304` with zero bytes and
  stable zero-event TFTP evidence.
- The Pi 5 proof classification file records
  `serial-capture-saturated-after-candidate-fetch`, `accepted=false`, and
  `completed_with_blocker_evidence=true`.
- Restore evidence returned the lab to tree
  `a0452458391d0e398b7e17e0f068bb652235f666bf277d004e0e214626128d10`.

## Classification

`serial-capture-saturated-after-candidate-fetch`.

This accepts only source/static candidate refresh, first-run candidate
publication/fetch, restore hygiene, and serial-capture blocker evidence. It
does not accept RP1 mapped/read-value behavior, RP1 unmapped/trap behavior,
firmware-state behavior, or pre-MMIO reachability.

## Disposition

- fixed: exact candidate identity, first-run fetch evidence, zero-byte serial
  window, known-good control, candidate rerun, and restore evidence are
  reconciled in the closeout task record.
- removed: mapping, trap, firmware-state, and pre-MMIO reachability claims are
  explicitly excluded.
- deferred: serial cursor/capture completeness repair is required before any
  same-shaped RP1 FR-read rerun.
- not-an-issue: the local/static diagnostic remains valid as a candidate, but
  it is not hardware mapping evidence.
